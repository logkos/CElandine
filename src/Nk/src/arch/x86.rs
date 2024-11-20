
pub use crate::arch::common::*;
use derive_more::From;
use crate::bindings::{DWORD, ULONGLONG}

#[repr(C)]
#[derive(From)]
#[allow(non_snake_case)]

pub struct KDataStruct {
    #[from]
    pub kdata_common: KDataCommon,
    // CPU dependent part
    pub pGDT_unused: *mut ULONGLONG,            // 0x2a0 ptr to global descriptor table
    pub dwCpuCap: DWORD,                 // 0x2a4 - CPU capability bits
    pub dwOEMInitGlobalsAddr: DWORD,     // 0x2a8 ptr to OAL entry point
    pub pAddrMap: *mut PADDRMAP,         // 0x2ac - ptr to OEMAddressTable
    pub pIDT_unused: *mut PKIDTENTRY,    // 0x2b0 - ptr to interrupt descriptor table
    pub pTSS_unused: *mut KTSS,          // 0x2b4 - ptr to per-CPU TSS
    pub alPad: [i32; 18],                // 0x2bc - padding
    pub aInfo: [DWORD; 32],              // 0x300 - misc. kernel info
}

pub struct _PCB {

}