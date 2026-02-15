//! End-to-end tests for Mmap bindings against real libc.

use bns_posix::PosixFile::Mmap;

#[test]
fn prot_constants() {
    assert_eq!(Mmap::PROT_NONE, 0);
    assert_eq!(Mmap::PROT_READ, 1);
    assert_eq!(Mmap::PROT_WRITE, 2);
    assert_eq!(Mmap::PROT_EXEC, 4);
}

#[test]
fn map_constants() {
    assert_eq!(Mmap::MAP_SHARED, 1);
    assert_eq!(Mmap::MAP_PRIVATE, 2);
    assert_eq!(Mmap::MAP_FIXED, 0x10);
    assert_eq!(Mmap::MAP_ANONYMOUS, 0x20);
}

#[test]
fn msync_constants() {
    assert_eq!(Mmap::MS_ASYNC, 1);
    assert_eq!(Mmap::MS_INVALIDATE, 2);
    assert_eq!(Mmap::MS_SYNC, 4);
}

#[test]
fn mmap_anonymous_roundtrip() {
    unsafe {
        let size: u64 = 4096;
        let ptr = Mmap::mmap(
            core::ptr::null(),
            size,
            Mmap::PROT_READ | Mmap::PROT_WRITE,
            Mmap::MAP_PRIVATE | Mmap::MAP_ANONYMOUS,
            -1,
            0,
        );
        assert_ne!(
            ptr as usize,
            usize::MAX,
            "mmap should not return MAP_FAILED"
        );

        let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, size as usize);
        slice[0] = 0xAB;
        slice[4095] = 0xCD;
        assert_eq!(slice[0], 0xAB);
        assert_eq!(slice[4095], 0xCD);

        let ret = Mmap::munmap(ptr, size);
        assert_eq!(ret, 0, "munmap should succeed");
    }
}

#[test]
fn mprotect_guard_page() {
    unsafe {
        let size: u64 = 4096;
        let ptr = Mmap::mmap(
            core::ptr::null(),
            size,
            Mmap::PROT_READ | Mmap::PROT_WRITE,
            Mmap::MAP_PRIVATE | Mmap::MAP_ANONYMOUS,
            -1,
            0,
        );
        assert_ne!(ptr as usize, usize::MAX);

        let ret = Mmap::mprotect(ptr, size, Mmap::PROT_READ);
        assert_eq!(ret, 0, "mprotect to PROT_READ should succeed");

        let ret = Mmap::mprotect(ptr, size, Mmap::PROT_READ | Mmap::PROT_WRITE);
        assert_eq!(ret, 0, "mprotect to PROT_READ|PROT_WRITE should succeed");

        Mmap::munmap(ptr, size);
    }
}
