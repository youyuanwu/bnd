windows_link::link!("c" "C" fn __asprintf(__ptr : *mut *mut i8, __fmt : *const i8, ...) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn __getdelim(__lineptr : *mut *mut i8, __n : *mut usize, __delimiter : i32, __stream : *mut super::file::FILE) -> super::types::__ssize_t);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn __overflow(param0 : *mut super::file::FILE, param1 : i32) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn __uflow(param0 : *mut super::file::FILE) -> i32);
windows_link::link!("c" "C" fn asprintf(__ptr : *mut *mut i8, __fmt : *const i8, ...) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn clearerr(__stream : *mut super::file::FILE));
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn clearerr_unlocked(__stream : *mut super::file::FILE));
windows_link::link!("c" "C" fn ctermid(__s : *mut i8) -> *mut i8);
windows_link::link!("c" "C" fn dprintf(__fd : i32, __fmt : *const i8, ...) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fclose(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fdopen(__fd : i32, __modes : *const i8) -> *mut super::file::FILE);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn feof(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn feof_unlocked(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn ferror(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn ferror_unlocked(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fflush(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fflush_unlocked(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fgetc(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fgetc_unlocked(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(
    feature = "__fpos_t",
    feature = "__mbstate_t",
    feature = "file",
    feature = "struct_file",
    feature = "types"
))]
windows_link::link!("c" "C" fn fgetpos(__stream : *mut super::file::FILE, __pos : *mut fpos_t) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fgets(__s : *mut i8, __n : i32, __stream : *mut super::file::FILE) -> *mut i8);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fileno(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fileno_unlocked(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn flockfile(__stream : *mut super::file::FILE));
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fmemopen(__s : *mut core::ffi::c_void, __len : usize, __modes : *const i8) -> *mut super::file::FILE);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fopen(__filename : *const i8, __modes : *const i8) -> *mut super::file::FILE);
#[cfg(all(
    feature = "cookie_io_functions_t",
    feature = "file",
    feature = "struct_file",
    feature = "types"
))]
windows_link::link!("c" "C" fn fopencookie(__magic_cookie : *mut core::ffi::c_void, __modes : *const i8, __io_funcs : super::cookie_io_functions_t::cookie_io_functions_t) -> *mut super::file::FILE);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fprintf(__stream : *mut super::file::FILE, __format : *const i8, ...) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fputc(__c : i32, __stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fputc_unlocked(__c : i32, __stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fputs(__s : *const i8, __stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fread(__ptr : *mut core::ffi::c_void, __size : usize, __n : usize, __stream : *mut super::file::FILE) -> u64);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fread_unlocked(__ptr : *mut core::ffi::c_void, __size : usize, __n : usize, __stream : *mut super::file::FILE) -> usize);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn freopen(__filename : *const i8, __modes : *const i8, __stream : *mut super::file::FILE) -> *mut super::file::FILE);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" "__isoc99_fscanf" fn fscanf(__stream : *mut super::file::FILE, __format : *const i8, ...) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fseek(__stream : *mut super::file::FILE, __off : i64, __whence : i32) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fseeko(__stream : *mut super::file::FILE, __off : super::types::__off_t, __whence : i32) -> i32);
#[cfg(all(
    feature = "__fpos_t",
    feature = "__mbstate_t",
    feature = "file",
    feature = "struct_file",
    feature = "types"
))]
windows_link::link!("c" "C" fn fsetpos(__stream : *mut super::file::FILE, __pos : *const fpos_t) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn ftell(__stream : *mut super::file::FILE) -> i64);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn ftello(__stream : *mut super::file::FILE) -> super::types::__off_t);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn ftrylockfile(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn funlockfile(__stream : *mut super::file::FILE));
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fwrite(__ptr : *const core::ffi::c_void, __size : usize, __n : usize, __s : *mut super::file::FILE) -> u64);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn fwrite_unlocked(__ptr : *const core::ffi::c_void, __size : usize, __n : usize, __stream : *mut super::file::FILE) -> usize);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn getc(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn getc_unlocked(__stream : *mut super::file::FILE) -> i32);
windows_link::link!("c" "C" fn getchar() -> i32);
windows_link::link!("c" "C" fn getchar_unlocked() -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn getdelim(__lineptr : *mut *mut i8, __n : *mut usize, __delimiter : i32, __stream : *mut super::file::FILE) -> super::types::__ssize_t);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn getline(__lineptr : *mut *mut i8, __n : *mut usize, __stream : *mut super::file::FILE) -> super::types::__ssize_t);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn getw(__stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn open_memstream(__bufloc : *mut *mut i8, __sizeloc : *mut usize) -> *mut super::file::FILE);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn pclose(__stream : *mut super::file::FILE) -> i32);
windows_link::link!("c" "C" fn perror(__s : *const i8));
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn popen(__command : *const i8, __modes : *const i8) -> *mut super::file::FILE);
windows_link::link!("c" "C" fn printf(__format : *const i8, ...) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn putc(__c : i32, __stream : *mut super::file::FILE) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn putc_unlocked(__c : i32, __stream : *mut super::file::FILE) -> i32);
windows_link::link!("c" "C" fn putchar(__c : i32) -> i32);
windows_link::link!("c" "C" fn putchar_unlocked(__c : i32) -> i32);
windows_link::link!("c" "C" fn puts(__s : *const i8) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn putw(__w : i32, __stream : *mut super::file::FILE) -> i32);
windows_link::link!("c" "C" fn remove(__filename : *const i8) -> i32);
windows_link::link!("c" "C" fn rename(__old : *const i8, __new : *const i8) -> i32);
windows_link::link!("c" "C" fn renameat(__oldfd : i32, __old : *const i8, __newfd : i32, __new : *const i8) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn rewind(__stream : *mut super::file::FILE));
windows_link::link!("c" "C" "__isoc99_scanf" fn scanf(__format : *const i8, ...) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn setbuf(__stream : *mut super::file::FILE, __buf : *mut i8));
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn setbuffer(__stream : *mut super::file::FILE, __buf : *mut i8, __size : usize));
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn setlinebuf(__stream : *mut super::file::FILE));
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn setvbuf(__stream : *mut super::file::FILE, __buf : *mut i8, __modes : i32, __n : usize) -> i32);
windows_link::link!("c" "C" fn snprintf(__s : *mut i8, __maxlen : usize, __format : *const i8, ...) -> i32);
windows_link::link!("c" "C" fn sprintf(__s : *mut i8, __format : *const i8, ...) -> i32);
windows_link::link!("c" "C" "__isoc99_sscanf" fn sscanf(__s : *const i8, __format : *const i8, ...) -> i32);
windows_link::link!("c" "C" fn tempnam(__dir : *const i8, __pfx : *const i8) -> *mut i8);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn tmpfile() -> *mut super::file::FILE);
windows_link::link!("c" "C" fn tmpnam(param0 : *mut i8) -> *mut i8);
windows_link::link!("c" "C" fn tmpnam_r(__s : *mut i8) -> *mut i8);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn ungetc(__c : i32, __stream : *mut super::file::FILE) -> i32);
windows_link::link!("c" "C" fn vasprintf(__ptr : *mut *mut i8, __f : *const i8, __arg : *mut core::ffi::c_void) -> i32);
windows_link::link!("c" "C" fn vdprintf(__fd : i32, __fmt : *const i8, __arg : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" fn vfprintf(__s : *mut super::file::FILE, __format : *const i8, __arg : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "file", feature = "struct_file", feature = "types"))]
windows_link::link!("c" "C" "__isoc99_vfscanf" fn vfscanf(__s : *mut super::file::FILE, __format : *const i8, __arg : *mut core::ffi::c_void) -> i32);
windows_link::link!("c" "C" fn vprintf(__format : *const i8, __arg : *mut core::ffi::c_void) -> i32);
windows_link::link!("c" "C" "__isoc99_vscanf" fn vscanf(__format : *const i8, __arg : *mut core::ffi::c_void) -> i32);
windows_link::link!("c" "C" fn vsnprintf(__s : *mut i8, __maxlen : usize, __format : *const i8, __arg : *mut core::ffi::c_void) -> i32);
windows_link::link!("c" "C" fn vsprintf(__s : *mut i8, __format : *const i8, __arg : *mut core::ffi::c_void) -> i32);
windows_link::link!("c" "C" "__isoc99_vsscanf" fn vsscanf(__s : *const i8, __format : *const i8, __arg : *mut core::ffi::c_void) -> i32);
pub const BUFSIZ: i32 = 8192;
pub const EOF: i32 = -1;
pub const FOPEN_MAX: i32 = 16;
pub const L_ctermid: i32 = 9;
pub const L_tmpnam: i32 = 20;
pub const P_tmpdir: *const u8 = [47, 116, 109, 112, 0].as_ptr();
pub const TMP_MAX: i32 = 238328;
pub const _IOFBF: i32 = 0;
pub const _IOLBF: i32 = 1;
pub const _IONBF: i32 = 2;
#[cfg(all(feature = "__fpos_t", feature = "__mbstate_t", feature = "types"))]
pub type fpos_t = super::__fpos_t::__fpos_t;
pub type va_list = [u64; 3];
