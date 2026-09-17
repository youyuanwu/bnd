use std::path::{Path, PathBuf};

/// Generate the bnd-linux crate through bnd-clang.
///
/// Clang emits flat `libc` RDL partitions, then the canonical WinMD is
/// structurally remapped into defining-header namespaces.
pub fn generate(output_dir: &Path) {
    let temp = tempfile::tempdir().expect("failed to create temporary metadata directory");
    let flat_winmd = generate_metadata(temp.path());
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create bnd-linux WinMD directory");
    let winmd = winmd_dir.join("bnd-linux.winmd");
    windows_clang::remap_by_header()
        .rdl_dir(temp.path().join("metadata"))
        .input(&flat_winmd)
        .output(&winmd)
        .scratch_dir(temp.path().join("remap"))
        .source("libc")
        .import("Windows::Win32")
        .reference_default()
        .write()
        .expect("failed to remap canonical bnd-linux metadata");
    let manifest_path = output_dir.join("Cargo.toml");
    let manifest = std::fs::read(&manifest_path).expect("failed to preserve bnd-linux Cargo.toml");

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
            "--package-feature-root",
            "libc",
        ]);
    });
    if let Err(payload) = generation {
        std::fs::write(manifest_path, manifest).expect("failed to restore bnd-linux Cargo.toml");
        std::panic::resume_unwind(payload);
    }
}

fn generate_metadata(output_dir: &Path) -> PathBuf {
    let rdl_dir = output_dir.join("metadata");
    clear_rdl_dir(&rdl_dir);
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create bnd-clang WinMD directory");
    let linux_winmd = winmd_dir.join("bnd-linux.winmd");

    const HEADERS: &[&str] = &[
        "sys/types.h",
        "fcntl.h",
        "unistd.h",
        "sys/stat.h",
        "sys/mman.h",
        "dirent.h",
        "sys/socket.h",
        "netinet/in.h",
        "arpa/inet.h",
        "netdb.h",
        "signal.h",
        "dlfcn.h",
        "errno.h",
        "sched.h",
        "time.h",
        "sys/time.h",
        "pthread.h",
        "stdio.h",
        "sys/epoll.h",
        "sys/eventfd.h",
        "sys/timerfd.h",
        "sys/signalfd.h",
        "sys/inotify.h",
        "sys/sendfile.h",
        "sys/xattr.h",
        "sys/mount.h",
        "linux/types.h",
    ];
    const PARTITION_HEADERS: &[&str] = &[
        "dirent.h",
        "bits/dirent.h",
        "arpa/inet.h",
        "dlfcn.h",
        "bits/dlfcn.h",
        "errno.h",
        "bits/errno.h",
        "linux/errno.h",
        "asm/errno.h",
        "asm-generic/errno.h",
        "asm-generic/errno-base.h",
        "fcntl.h",
        "bits/fcntl-linux.h",
        "linux/types.h",
        "netdb.h",
        "bits/netdb.h",
        "netinet/in.h",
        "signal.h",
        "bits/sigaction.h",
        "bits/signum-generic.h",
        "bits/signum-arch.h",
        "bits/sigcontext.h",
        "bits/types/__sigset_t.h",
        "bits/types/siginfo_t.h",
        "bits/types/__sigval_t.h",
        "bits/types/stack_t.h",
        "bits/types/struct_sigstack.h",
        "sched.h",
        "bits/sched.h",
        "bits/types/struct_sched_param.h",
        "bits/cpu-set.h",
        "pthread.h",
        "bits/pthreadtypes.h",
        "bits/thread-shared-types.h",
        "bits/pthreadtypes-arch.h",
        "bits/atomic_wide_counter.h",
        "bits/struct_mutex.h",
        "bits/struct_rwlock.h",
        "bits/types/struct___jmp_buf_tag.h",
        "bits/pthread_stack_min-dynamic.h",
        "bits/pthread_stack_min.h",
        "stdio.h",
        "bits/stdio_lim.h",
        "bits/types/__fpos_t.h",
        "bits/types/__mbstate_t.h",
        "bits/types/struct_FILE.h",
        "bits/types/cookie_io_functions_t.h",
        "sys/types.h",
        "bits/types.h",
        "sys/eventfd.h",
        "bits/eventfd.h",
        "sys/epoll.h",
        "bits/epoll.h",
        "sys/inotify.h",
        "bits/inotify.h",
        "sys/mman.h",
        "bits/mman-linux.h",
        "bits/mman-map-flags-generic.h",
        "sys/mount.h",
        "sys/sendfile.h",
        "sys/signalfd.h",
        "bits/signalfd.h",
        "sys/socket.h",
        "bits/socket.h",
        "bits/socket_type.h",
        "bits/socket-constants.h",
        "bits/types/struct_iovec.h",
        "sys/stat.h",
        "bits/struct_stat.h",
        "bits/types/struct_timespec.h",
        "sys/time.h",
        "sys/timerfd.h",
        "bits/timerfd.h",
        "sys/xattr.h",
        "time.h",
        "bits/time.h",
        "bits/types/clock_t.h",
        "bits/types/struct_tm.h",
        "bits/types/clockid_t.h",
        "bits/types/timer_t.h",
        "bits/types/struct_itimerspec.h",
        "bits/types/struct_timeval.h",
        "bits/types/locale_t.h",
        "bits/types/__locale_t.h",
        "unistd.h",
    ];
    let source = HEADERS
        .iter()
        .map(|header| format!("#include <{header}>\n"))
        .collect::<String>();

    windows_clang::clang()
        .input_text(&source)
        .args(["-x", "c", "-std=gnu11", "-D_LINUX_MOUNT_H"])
        .namespace("libc")
        .library("c")
        .libraries([
            ("crypt", "crypt"),
            ("inet_net_ntop", "resolv"),
            ("inet_net_pton", "resolv"),
            ("inet_neta", "resolv"),
        ])
        .include_macros(["_IOFBF", "_IOLBF", "_IONBF"])
        .exclude_symbol("bindresvport6")
        .scope_headers(PARTITION_HEADERS.iter().copied())
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

        assert!(constants.contains(&"EPOLL_CTL_ADD"));
        assert!(events.contains(&"EPOLLIN"));
        assert!(events.contains(&"EPOLLET"));
        for name in ["epoll_create1", "epoll_ctl", "epoll_wait"] {
            let method = methods
                .iter()
                .find(|method| method.name() == name)
                .unwrap_or_else(|| panic!("{name} missing"));
            assert_eq!(
                method
                    .impl_map()
                    .expect("native import")
                    .import_scope()
                    .name(),
                "c"
            );
        }

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

        assert!(constants.contains(&"IN_CREATE"));
        assert!(constants.contains(&"IN_CLOSE"));
        assert!(constants.contains(&"IN_MOVE"));
        assert!(constants.contains(&"IN_ALL_EVENTS"));
        assert!(constants.contains(&"IN_NONBLOCK"));
        for name in ["inotify_add_watch", "inotify_init1", "inotify_rm_watch"] {
            let method = methods
                .iter()
                .find(|method| method.name() == name)
                .unwrap_or_else(|| panic!("{name} missing"));
            assert_eq!(
                method
                    .impl_map()
                    .expect("native import")
                    .import_scope()
                    .name(),
                "c"
            );
        }

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

        let signalfd_siginfo = index.expect("libc", "signalfd_siginfo");
        let fields: Vec<_> = signalfd_siginfo.fields().collect();
        assert_eq!(fields.first().expect("ssi_signo field").name(), "ssi_signo");
        assert_eq!(fields.last().expect("__pad field").name(), "__pad");
        assert_eq!(
            fields.last().expect("__pad field").ty(),
            windows_metadata::Type::ArrayFixed(Box::new(windows_metadata::Type::U8), 28)
        );

        let constants: Vec<_> = apis.fields().map(|field| field.name()).collect();
        let methods: Vec<_> = apis.methods().collect();
        let method_names: Vec<_> = methods.iter().map(|method| method.name()).collect();

        assert!(constants.contains(&"SFD_CLOEXEC"));
        assert!(constants.contains(&"SFD_NONBLOCK"));
        assert!(method_names.contains(&"signalfd"));

        assert!(constants.contains(&"XATTR_CREATE"));
        assert!(constants.contains(&"XATTR_REPLACE"));
        for name in [
            "setxattr",
            "lsetxattr",
            "fsetxattr",
            "getxattr",
            "lgetxattr",
            "fgetxattr",
            "listxattr",
            "llistxattr",
            "flistxattr",
            "removexattr",
            "lremovexattr",
            "fremovexattr",
        ] {
            assert!(method_names.contains(&name), "{name} missing");
        }

        let getxattr = methods
            .iter()
            .find(|method| method.name() == "getxattr")
            .expect("getxattr method");
        let signature = getxattr.signature(&[]);
        assert_eq!(
            signature.return_type,
            windows_metadata::Type::value_named("libc", "ssize_t")
        );
        assert_eq!(signature.types.last(), Some(&windows_metadata::Type::USize));

        let mount_attr = index.expect("libc", "mount_attr");
        assert_eq!(
            mount_attr
                .fields()
                .map(|field| field.name())
                .collect::<Vec<_>>(),
            ["attr_set", "attr_clr", "propagation", "userns_fd"]
        );
        for name in [
            "fsconfig",
            "fsmount",
            "fsopen",
            "fspick",
            "mount",
            "mount_setattr",
            "move_mount",
            "open_tree",
            "umount",
            "umount2",
        ] {
            assert!(method_names.contains(&name), "{name} missing");
        }
        for name in ["__be16", "__be32", "__be64", "__le16", "__le32", "__le64"] {
            assert!(has("libc", name), "{name} missing");
        }
        for name in [
            "addrinfo",
            "dirent",
            "pthread_mutex_t",
            "sigaction",
            "sockaddr",
            "stat",
            "tm",
        ] {
            assert!(has("libc", name), "{name} missing");
        }
        for name in [
            "__errno_location",
            "clock_gettime",
            "chmod",
            "dlopen",
            "dup",
            "dup2",
            "fopen",
            "fsync",
            "getaddrinfo",
            "inet_pton",
            "lstat",
            "mkdir",
            "mmap",
            "opendir",
            "pthread_create",
            "sched_yield",
            "sigaction",
            "socket",
            "stat",
            "umask",
            "write",
        ] {
            assert!(method_names.contains(&name), "{name} missing");
        }
        for name in [
            "_IOFBF",
            "_IOLBF",
            "_IONBF",
            "O_RDONLY",
            "O_WRONLY",
            "O_RDWR",
            "O_CREAT",
            "O_TRUNC",
            "STDIN_FILENO",
            "STDOUT_FILENO",
            "STDERR_FILENO",
        ] {
            assert!(constants.contains(&name), "{name} missing");
        }
        assert!(!has("libc", "__pthread_unwind_buf_t"));
        for name in [
            "__pthread_register_cancel",
            "__pthread_unregister_cancel",
            "__pthread_unwind_next",
        ] {
            assert!(!method_names.contains(&name), "{name} should be omitted");
        }
        assert!(!method_names.contains(&"bindresvport6"));
        for (name, import_name) in [
            ("fscanf", "__isoc99_fscanf"),
            ("scanf", "__isoc99_scanf"),
            ("sscanf", "__isoc99_sscanf"),
            ("vfscanf", "__isoc99_vfscanf"),
            ("vscanf", "__isoc99_vscanf"),
            ("vsscanf", "__isoc99_vsscanf"),
        ] {
            let method = methods
                .iter()
                .find(|method| method.name() == name)
                .unwrap_or_else(|| panic!("{name} missing"));
            assert_eq!(
                method.impl_map().expect("native import").import_name(),
                import_name
            );
        }
        for (name, library) in [
            ("crypt", "crypt"),
            ("inet_net_ntop", "resolv"),
            ("inet_net_pton", "resolv"),
            ("inet_neta", "resolv"),
        ] {
            let method = methods
                .iter()
                .find(|method| method.name() == name)
                .unwrap_or_else(|| panic!("{name} missing"));
            assert_eq!(
                method
                    .impl_map()
                    .expect("native import")
                    .import_scope()
                    .name(),
                library
            );
        }

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
