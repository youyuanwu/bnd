use bnd_linux_clang::libc::{mount, types};

#[test]
fn mount_layout_and_constants_match_linux() {
    assert_eq!(std::mem::size_of::<mount::mount_attr>(), 32);
    assert_eq!(std::mem::align_of::<mount::mount_attr>(), 8);
    assert_eq!(mount::MOUNT_ATTR_SIZE_VER0, 32);
    assert_eq!(mount::MS_BIND, libc::MS_BIND as u32);
    assert_eq!(mount::MS_RDONLY, libc::MS_RDONLY as u32);
    assert_eq!(mount::MNT_DETACH, libc::MNT_DETACH as u32);
}

#[test]
fn linux_endian_types_match_host_widths() {
    assert_eq!(std::mem::size_of::<types::__be16>(), 2);
    assert_eq!(std::mem::size_of::<types::__be32>(), 4);
    assert_eq!(std::mem::size_of::<types::__be64>(), 8);
    assert_eq!(std::mem::size_of::<types::__le16>(), 2);
    assert_eq!(std::mem::size_of::<types::__le32>(), 4);
    assert_eq!(std::mem::size_of::<types::__le64>(), 8);
}
