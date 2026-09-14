#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern crate bnd_macros as windows_link;

#[cfg(test)]
mod tests {
    use crate::bindings::*;

    #[test]
    fn generated_zlib_api_compiles_and_runs() {
        assert_eq!(Z_OK, 0);
        assert_eq!(Z_STREAM_END, 1);
        assert_eq!(Z_DEFLATED, 8);
        assert_eq!(MAX_WBITS, 15);

        let version = unsafe { zlibVersion() };
        assert!(!version.is_null());
        let version = unsafe { std::ffi::CStr::from_ptr(version) };
        assert!(version.to_bytes().starts_with(b"1."));
    }

    #[test]
    fn generated_checksums_match_known_values() {
        let data = b"hello";

        unsafe {
            assert_eq!(crc32(0, data.as_ptr(), data.len() as uInt), 0x3610_a686);
            assert_eq!(adler32(1, data.as_ptr(), data.len() as uInt), 0x062c_0215);
        }
    }

    #[test]
    fn generated_compression_roundtrips() {
        let bound = unsafe { compressBound(1000) };
        assert!(
            (1000..2000).contains(&bound),
            "compressBound(1000) returned {bound}"
        );

        let original = b"The quick brown fox jumps over the lazy dog";
        let mut compressed = vec![0; unsafe { compressBound(original.len() as uLong) } as usize];
        let mut compressed_len = compressed.len() as uLong;

        let result = unsafe {
            compress(
                compressed.as_mut_ptr(),
                &mut compressed_len,
                original.as_ptr(),
                original.len() as uLong,
            )
        };
        assert_eq!(result, Z_OK);

        let mut decompressed = vec![0; original.len()];
        let mut decompressed_len = decompressed.len() as uLong;
        let result = unsafe {
            uncompress(
                decompressed.as_mut_ptr(),
                &mut decompressed_len,
                compressed.as_ptr(),
                compressed_len,
            )
        };

        assert_eq!(result, Z_OK);
        assert_eq!(decompressed_len as usize, original.len());
        assert_eq!(decompressed, original);
    }

    #[test]
    fn generated_types_match_linux_lp64_layout() {
        assert_eq!(std::mem::size_of::<uLong>(), 8);
        assert_eq!(std::mem::size_of::<z_stream>(), 112);
        assert_eq!(std::mem::align_of::<z_stream>(), 8);
        assert_eq!(std::mem::size_of::<gz_header>(), 80);
        assert_eq!(std::mem::align_of::<gz_header>(), 8);

        let gz_file = gzFile_s::default();
        let _ = (gz_file.have, gz_file.next, gz_file.pos);
    }
}
