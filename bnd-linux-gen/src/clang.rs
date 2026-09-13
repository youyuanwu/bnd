use std::path::{Path, PathBuf};

/// Generate the staged bnd-linux crate through bnd-clang.
///
/// Each partition has its own RDL file and namespace. Earlier metadata is
/// supplied as a reference while parsing later partitions, then all RDL is
/// compiled into one WinMD for Rust generation.
pub fn generate(output_dir: &Path) {
    let temp = tempfile::tempdir().expect("failed to create bnd-clang metadata directory");
    let generated_winmd = generate_metadata(temp.path());
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create bnd-linux-clang WinMD directory");
    let winmd = winmd_dir.join("bnd-linux-clang.winmd");
    std::fs::copy(generated_winmd, &winmd).expect("failed to save bnd-linux-clang WinMD");
    let manifest_path = output_dir.join("Cargo.toml");
    let manifest =
        std::fs::read(&manifest_path).expect("failed to preserve bnd-linux-clang Cargo.toml");

    let generation = std::panic::catch_unwind(|| {
        windows_bindgen::bindgen([
            "--in",
            winmd.to_str().unwrap(),
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
    std::fs::create_dir_all(output_dir).expect("failed to create bnd-clang output directory");

    let eventfd_rdl = output_dir.join("eventfd.rdl");
    let eventfd_winmd = output_dir.join("eventfd.winmd");
    let epoll_rdl = output_dir.join("epoll.rdl");
    let linux_winmd = output_dir.join("bnd-linux-clang.winmd");

    generate_partition(
        "sys/eventfd.h",
        &["sys/eventfd.h", "bits/eventfd.h"],
        "libc.linux.eventfd",
        None,
        &eventfd_rdl,
    );
    compile_rdl(&[&eventfd_rdl], &eventfd_winmd);
    generate_partition(
        "sys/epoll.h",
        &["sys/epoll.h", "bits/epoll.h"],
        "libc.linux.epoll",
        Some(&eventfd_winmd),
        &epoll_rdl,
    );
    compile_rdl(&[&eventfd_rdl, &epoll_rdl], &linux_winmd);

    linux_winmd
}

fn generate_partition(
    header: &str,
    filters: &[&str],
    namespace: &str,
    reference: Option<&Path>,
    output: &Path,
) {
    let mut generator = windows_clang::clang();
    generator
        .input_text(&format!("#include <{header}>\n"))
        .args(["-x", "c", "-std=c11"])
        .filters(filters)
        .namespace(namespace)
        .library("c")
        .output(output);
    if let Some(reference) = reference {
        generator.reference(reference);
    }
    generator
        .write()
        .unwrap_or_else(|error| panic!("bnd-clang failed to generate {header} RDL: {error}"));
}

fn compile_rdl(inputs: &[&Path], output: &Path) {
    windows_rdl::reader()
        .inputs(inputs)
        .reference_default()
        .output(output)
        .write()
        .expect("windows-rdl failed to compile Linux metadata");
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
    fn generates_eventfd_and_epoll_partitions() {
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

        assert!(has("libc.linux.eventfd", "eventfd_t"));
        assert!(has("libc.linux.eventfd", "Apis"));
        assert!(has("libc.linux.epoll", "epoll_data_t"));
        assert!(has("libc.linux.epoll", "epoll_event"));
        assert!(has("libc.linux.epoll", "EPOLL_EVENTS"));
        assert!(has("libc.linux.epoll", "Apis"));

        let eventfd_t = index.expect("libc.linux.eventfd", "eventfd_t");
        let fields: Vec<_> = eventfd_t.fields().collect();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].ty(), windows_metadata::Type::U64);

        let apis = index.expect("libc.linux.eventfd", "Apis");
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

        let epoll_apis = index.expect("libc.linux.epoll", "Apis");
        let constants: Vec<_> = epoll_apis.fields().map(|field| field.name()).collect();
        let events: Vec<_> = index
            .expect("libc.linux.epoll", "EPOLL_EVENTS")
            .fields()
            .map(|field| field.name())
            .collect();
        let methods: Vec<_> = epoll_apis.methods().collect();
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
    }
}
