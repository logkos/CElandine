use core::ffi::{c_ulong, c_long, c_void, c_uchar, c_uint, c_int};

// Basic Microsoft types
pub type BYTE = c_uchar;
pub type BOOL = c_int;
pub type BOOLEAN = BYTE;
pub type PVOID = *mut c_void;
pub type HANDLE = *mut c_void;
pub type PHANDLE = *mut HANDLE;
pub type HMENU = *mut c_void;
pub type HWND = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HMODULE = HINSTANCE;
pub type UINT = c_uint;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type ULONG = c_ulong;
pub type LONG = c_long;
pub type DWORD = c_ulong;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type WCHAR = u16;
pub type LONGLONG = i64;
pub type LPMSG = *mut MSG;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type ULONG_PTR = usize;
pub type PDWORD_PTR = *mut ULONG_PTR;
pub type SIZE_T = ULONG_PTR;
pub type LPWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type ULONGLONG = u64;


pub type HEAP_FLAGS = u32;
pub type HKEY = *mut c_void;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FILETIME {
    pub dwLowDateTime: DWORD,
    pub dwHighDateTime: DWORD,
}

pub type LPWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
#[repr(C)]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: DWORD,
    pub ftCreationTime: FILETIME,
    pub ftLastAccessTime: FILETIME,
    pub ftLastWriteTime: FILETIME,
    pub nFileSizeHigh: DWORD,
    pub nFileSizeLow: DWORD,
    pub dwReserved0: DWORD,
    pub dwReserved1: DWORD,
    pub cFileName: [WCHAR; 260],
    pub cAlternateFileName: [WCHAR; 14],
}

pub type PMEMORY_BASIC_INFORMATION = *mut MEMORY_BASIC_INFORMATION;
#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: PVOID,
    pub AllocationBase: PVOID,
    pub AllocationProtect: DWORD,
    pub RegionSize: SIZE_T,
    pub State: DWORD,
    pub Protect: DWORD,
    pub Type: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LARGE_INTEGER_s {
    pub LowPart: ULONG,
    pub HighPart: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LARGE_INTEGER_u {
    pub LowPart: ULONG,
    pub HighPart: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union LARGE_INTEGER {
    repr: i64,
    pub s: LARGE_INTEGER_s,
    pub u: LARGE_INTEGER_u,
    pub QuadPart: LONGLONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCWSTR,
    pub lpszClass: LPCWSTR,
    pub dwExStyle: DWORD,
}