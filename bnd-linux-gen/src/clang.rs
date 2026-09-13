use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Generate the staged bnd-linux crate through bnd-clang.
///
/// The canonical metadata uses one flat `libc` namespace partitioned into RDL
/// files by defining header. A temporary remapped WinMD supplies the
/// namespace-based Rust package layout.
pub fn generate(output_dir: &Path) {
    let temp = tempfile::tempdir().expect("failed to create temporary metadata directory");
    let generated_winmd = generate_metadata(temp.path());
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create bnd-linux-clang WinMD directory");
    let winmd = winmd_dir.join("bnd-linux-clang.winmd");
    std::fs::copy(&generated_winmd, &winmd).expect("failed to save bnd-linux-clang WinMD");
    let remapped_winmd = temp.path().join("bnd-linux-clang.remapped.winmd");
    remap_metadata(
        &temp.path().join("metadata"),
        &generated_winmd,
        &remapped_winmd,
    );
    let manifest_path = output_dir.join("Cargo.toml");
    let manifest =
        std::fs::read(&manifest_path).expect("failed to preserve bnd-linux-clang Cargo.toml");

    let generation = std::panic::catch_unwind(|| {
        windows_bindgen::bindgen([
            "--in",
            remapped_winmd.to_str().unwrap(),
            "--out",
            output_dir.to_str().unwrap(),
            "--filter",
            "libc",
            "--sys",
            "--package",
        ]);
    });
    std::fs::write(manifest_path, manifest).expect("failed to restore bnd-linux-clang Cargo.toml");
    if let Err(payload) = generation {
        std::panic::resume_unwind(payload);
    }
}

fn generate_metadata(output_dir: &Path) -> PathBuf {
    let rdl_dir = output_dir.join("metadata");
    clear_rdl_dir(&rdl_dir);
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create bnd-clang WinMD directory");
    let linux_winmd = winmd_dir.join("bnd-linux-clang.winmd");

    const HEADERS: [&str; 6] = [
        "sys/types.h",
        "sys/eventfd.h",
        "sys/epoll.h",
        "sys/inotify.h",
        "sys/sendfile.h",
        "sys/timerfd.h",
    ];
    const PARTITION_HEADERS: [&str; 11] = [
        "sys/types.h",
        "bits/types.h",
        "sys/eventfd.h",
        "bits/eventfd.h",
        "sys/epoll.h",
        "bits/epoll.h",
        "sys/inotify.h",
        "bits/inotify.h",
        "sys/sendfile.h",
        "sys/timerfd.h",
        "bits/timerfd.h",
    ];
    let source = HEADERS
        .map(|header| format!("#include <{header}>\n"))
        .concat();

    windows_clang::clang()
        .input_text(&source)
        .args(["-x", "c", "-std=c11"])
        .namespace("libc")
        .library("c")
        .scope_headers(PARTITION_HEADERS)
        .output(&rdl_dir)
        .write_by_header()
        .expect("bnd-clang failed to generate Linux RDL partitions");

    windows_rdl::reader()
        .input(&rdl_dir)
        .reference_default()
        .output(&linux_winmd)
        .write()
        .expect("windows-rdl failed to compile Linux metadata");
    linux_winmd
}

fn remap_metadata(rdl_dir: &Path, input: &Path, output: &Path) {
    let mut rdl_files: Vec<_> = std::fs::read_dir(rdl_dir)
        .expect("failed to read bnd-clang RDL directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rdl"))
        .collect();
    rdl_files.sort();

    let mut routes = HashMap::new();
    for path in rdl_files {
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("RDL file has no UTF-8 stem");
        let stem = module_stem(stem);
        for name in windows_rdl::item_names(&path, "libc").expect("failed to read RDL item names") {
            routes.insert(name, format!("libc.{stem}"));
        }
    }

    windows_metadata::remap()
        .source("libc")
        .fallback("libc")
        .routes(routes)
        .input(input)
        .output(output)
        .remap()
        .expect("failed to remap Linux metadata");
}

fn module_stem(header_stem: &str) -> String {
    let mut stem: String = header_stem
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();
    if stem
        .as_bytes()
        .first()
        .is_some_and(|byte| byte.is_ascii_digit())
    {
        stem.insert(0, '_');
    }
    stem
}

fn clear_rdl_dir(rdl_dir: &Path) {
    std::fs::create_dir_all(rdl_dir).expect("failed to create bnd-clang RDL directory");
    for entry in std::fs::read_dir(rdl_dir).expect("failed to read bnd-clang RDL directory") {
        let path = entry.expect("failed to read RDL entry").path();
        if path.extension().is_some_and(|extension| extension == "rdl") {
            std::fs::remove_file(path).expect("failed to remove stale RDL partition");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_index(bytes: Vec<u8>) -> windows_metadata::reader::Index {
        let file =
            windows_metadata::reader::File::new(bytes).expect("parse generated eventfd WinMD");
        windows_metadata::reader::Index::new(vec![file])
    }

    #[test]
    fn generates_ordered_linux_partitions() {
        let temp = tempfile::tempdir().expect("create temporary output directory");
        let winmd = generate_metadata(temp.path());
        let index = open_index(std::fs::read(&winmd).expect("read generated Linux WinMD"));

        let types: Vec<_> = index
            .types()
            .map(|ty| (ty.namespace().to_string(), ty.name().to_string()))
            .collect();
        let has = |namespace: &str, name: &str| {
            types
                .iter()
                .any(|(actual_namespace, ty)| actual_namespace == namespace && ty == name)
        };

        assert!(has("libc", "off_t"));
        assert!(has("libc", "ssize_t"));
        assert!(has("libc", "eventfd_t"));
        assert!(has("libc", "epoll_data_t"));
        assert!(has("libc", "epoll_event"));
        assert!(has("libc", "EPOLL_EVENTS"));
        assert!(has("libc", "Apis"));
        assert_eq!(
            types
                .iter()
                .map(|(namespace, _)| namespace.as_str())
                .collect::<std::collections::BTreeSet<_>>(),
            ["libc"].into_iter().collect()
        );

        let eventfd_t = index.expect("libc", "eventfd_t");
        let fields: Vec<_> = eventfd_t.fields().collect();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].ty(), windows_metadata::Type::U64);

        let apis = index.expect("libc", "Apis");
        let constants: Vec<_> = apis.fields().collect();
        let constant_names: Vec<_> = constants.iter().map(|field| field.name()).collect();
        let methods: Vec<_> = apis.methods().collect();
        let method_names: Vec<_> = methods.iter().map(|method| method.name()).collect();

        assert!(constant_names.contains(&"EFD_CLOEXEC"));
        assert!(constant_names.contains(&"EFD_NONBLOCK"));
        assert!(constant_names.contains(&"EFD_SEMAPHORE"));
        assert!(method_names.contains(&"eventfd"));
        assert!(method_names.contains(&"eventfd_read"));
        assert!(method_names.contains(&"eventfd_write"));

        let eventfd = methods
            .iter()
            .find(|method| method.name() == "eventfd")
            .expect("eventfd method");
        let import = eventfd.impl_map().expect("eventfd import");
        assert_eq!(import.import_scope().name(), "c");
        assert!(
            import
                .flags()
                .contains(windows_metadata::PInvokeAttributes::CallConvCdecl)
        );

        let signature = eventfd.signature(&[]);
        assert_eq!(signature.return_type, windows_metadata::Type::I32);
        assert_eq!(
            signature.types,
            [windows_metadata::Type::U32, windows_metadata::Type::I32]
        );

        let constants: Vec<_> = apis.fields().map(|field| field.name()).collect();
        let events: Vec<_> = index
            .expect("libc", "EPOLL_EVENTS")
            .fields()
            .map(|field| field.name())
            .collect();
        let methods: Vec<_> = apis.methods().collect();
        let method_names: Vec<_> = methods.iter().map(|method| method.name()).collect();

        assert!(constants.contains(&"EPOLL_CTL_ADD"));
        assert!(events.contains(&"EPOLLIN"));
        assert!(events.contains(&"EPOLLET"));
        assert!(methods.iter().all(|method| {
            method
                .impl_map()
                .is_some_and(|import| import.import_scope().name() == "c")
        }));
        assert!(method_names.contains(&"epoll_create1"));
        assert!(method_names.contains(&"epoll_ctl"));
        assert!(method_names.contains(&"epoll_wait"));

        let inotify_event = index.expect("libc", "inotify_event");
        let fields: Vec<_> = inotify_event.fields().collect();
        assert_eq!(
            fields.iter().map(|field| field.name()).collect::<Vec<_>>(),
            ["wd", "mask", "cookie", "len", "name"]
        );
        assert_eq!(
            fields[4].ty(),
            windows_metadata::Type::ArrayFixed(Box::new(windows_metadata::Type::I8), 0)
        );

        let constants: Vec<_> = apis.fields().map(|field| field.name()).collect();
        let methods: Vec<_> = apis.methods().collect();
        let method_names: Vec<_> = methods.iter().map(|method| method.name()).collect();

        assert!(constants.contains(&"IN_CREATE"));
        assert!(constants.contains(&"IN_CLOSE"));
        assert!(constants.contains(&"IN_MOVE"));
        assert!(constants.contains(&"IN_ALL_EVENTS"));
        assert!(constants.contains(&"IN_NONBLOCK"));
        assert!(methods.iter().all(|method| {
            method
                .impl_map()
                .is_some_and(|import| import.import_scope().name() == "c")
        }));
        assert!(method_names.contains(&"inotify_add_watch"));
        assert!(method_names.contains(&"inotify_init1"));
        assert!(method_names.contains(&"inotify_rm_watch"));

        let itimerspec = index.expect("libc", "itimerspec");
        let fields: Vec<_> = itimerspec.fields().collect();
        assert_eq!(
            fields.iter().map(|field| field.name()).collect::<Vec<_>>(),
            ["it_interval", "it_value"]
        );
        assert!(fields.iter().all(|field| {
            field.ty() == windows_metadata::Type::value_named("libc", "timespec")
        }));

        let constants: Vec<_> = apis.fields().map(|field| field.name()).collect();
        let methods: Vec<_> = apis.methods().collect();
        let method_names: Vec<_> = methods.iter().map(|method| method.name()).collect();

        assert!(constants.contains(&"TFD_CLOEXEC"));
        assert!(constants.contains(&"TFD_NONBLOCK"));
        assert!(constants.contains(&"TFD_TIMER_ABSTIME"));
        assert!(constants.contains(&"TFD_TIMER_CANCEL_ON_SET"));
        assert!(method_names.contains(&"timerfd_create"));
        assert!(method_names.contains(&"timerfd_gettime"));
        assert!(method_names.contains(&"timerfd_settime"));

        let timerfd_settime = methods
            .iter()
            .find(|method| method.name() == "timerfd_settime")
            .expect("timerfd_settime method");
        assert_eq!(
            timerfd_settime.signature(&[]).types,
            [
                windows_metadata::Type::I32,
                windows_metadata::Type::I32,
                windows_metadata::Type::PtrConst(
                    Box::new(windows_metadata::Type::value_named("libc", "itimerspec")),
                    1,
                ),
                windows_metadata::Type::PtrMut(
                    Box::new(windows_metadata::Type::value_named("libc", "itimerspec")),
                    1,
                ),
            ]
        );

        let sendfile = index
            .expect("libc", "Apis")
            .methods()
            .find(|method| method.name() == "sendfile")
            .expect("sendfile method");
        let import = sendfile.impl_map().expect("sendfile import");
        assert_eq!(import.import_scope().name(), "c");
        assert!(
            import
                .flags()
                .contains(windows_metadata::PInvokeAttributes::CallConvCdecl)
        );

        let signature = sendfile.signature(&[]);
        assert_eq!(
            signature.return_type,
            windows_metadata::Type::value_named("libc", "ssize_t")
        );
        assert_eq!(
            signature.types,
            [
                windows_metadata::Type::I32,
                windows_metadata::Type::I32,
                windows_metadata::Type::PtrMut(
                    Box::new(windows_metadata::Type::value_named("libc", "off_t")),
                    1,
                ),
                windows_metadata::Type::USize,
            ]
        );
    }
}
