use bnd_linux::linux::mount;

#[test]
fn mount_constants() {
    assert_eq!(mount::MS_RDONLY, 1);
    assert_eq!(mount::MS_NOSUID, 2);
    assert_eq!(mount::MS_NODEV, 4);
    assert_eq!(mount::MS_NOEXEC, 8);
    assert_eq!(mount::MS_REMOUNT, 32);
    assert_eq!(mount::MS_BIND, 4096);
    assert_eq!(mount::MNT_FORCE, 1);
    assert_eq!(mount::MNT_DETACH, 2);
    assert_eq!(mount::UMOUNT_NOFOLLOW, 8);
}

#[test]
fn mount_attr_struct_size() {
    assert_eq!(core::mem::size_of::<mount::mount_attr>(), 32);
}

#[test]
fn mount_attr_size_ver0_matches_struct() {
    assert_eq!(
        mount::MOUNT_ATTR_SIZE_VER0 as usize,
        core::mem::size_of::<mount::mount_attr>()
    );
}

#[test]
fn fsconfig_cmd_constants() {
    assert_eq!(mount::FSCONFIG_SET_FLAG, 0);
    assert_eq!(mount::FSCONFIG_SET_STRING, 1);
    assert_eq!(mount::FSCONFIG_SET_BINARY, 2);
    assert_eq!(mount::FSCONFIG_SET_PATH, 3);
    assert_eq!(mount::FSCONFIG_SET_PATH_EMPTY, 4);
    assert_eq!(mount::FSCONFIG_SET_FD, 5);
    assert_eq!(mount::FSCONFIG_CMD_CREATE, 6);
}
