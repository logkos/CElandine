mod interrupts;
mod obj;
mod scheduler;
mod spinlock;
mod thread;
mod watchdog;

use crate::bindings::{DLIST, PHDATA};
pub const ProcExtMthds: &[()] = &[
//    not_supported as !, // 0 - user mode not exposed
//    not_supported as !, // 1
];

pub const ProcQueryMthds: &[()] = &[
  //  not_supported as !, // 0 - user mode not exposed
    //not_supported as !, // 1
];

pub const ProcIntMthds: &[()] = &[
    //not_supported as !, // 0 - user mode not exposed
    //not_supported as !, // 1
];

pub const ProcSigs: &[i64] = &[
    //not_supported as !, // 0 - user mode not exposed
    //not_supported as !, // 1
];


pub struct _PROCESS {
    DLIST       prclist: DLIST;        // 00: Doubly linked list of processes, must be 1st
    PHDATA      phdProcEvt;     // 08: the 'process event' to be signaled on process exit
    DWORD       dwId;           // 0C: process id, (handle for this process in NK's handle table 
    DLIST       thrdList;       // 10: Doubly linked list of all threads in the process 
    LPVOID      BasePtr;        // 18: Base pointer of exe load 
    PTHREAD     pDbgrThrd;      // 1C: thread debugging this process, if any 
    LPCWSTR     lpszProcName;   // 20: name of process 
    DWORD       tlsLowUsed;     // 24: TLS in use bitmask (first 32 slots) 
    DWORD       tlsHighUsed;    // 28: TLS in use bitmask (second 32 slots) 
    PPAGEDIRECTORY ppdir;       // 2C: page directory
    HANDLE      hTok;           // 30: process default token 
    PMODULE     pmodResource;   // 34: module that contains the resources 
    PHNDLTABLE  phndtbl;        // 38: handle table 
    DWORD       vaFree;         // 3C: lowest address of free VM 
    PNAME       pStdNames[3];   // 40: Pointer to names for stdio
    PVALIST     pVaList;        // 4C: list of externally VirtualAlloc'd addresses
    BYTE        bASID;          // 50: ASID for ARM.
    BYTE        fNotifyExiting; // 51: we're in the middle of notifying process exiting 
    BYTE        bChainDebug;    // 52: Did the creator want to debug child processes? 
    BYTE        bState;         // 53: process state
    LPDBGPARAM  ZonePtr;        // 54: address of Debug zone pointer 
    PLOCKPAGELIST pLockList;    // 58: locked page list
    openexe_t   oe;             // 5c: Pointer to executable file handle 
    e32_lite    e32;            // ??: structure containing exe header 
    o32_lite    *o32_ptr;       // ??: o32 array pointer for exe 
    long        nCallers;       // ??: number of callers into this process (PSL server only)
    WORD        wThrdCnt;       // ??: #of threads (MUST BE DWORD boundary, we use interlock operation on this)
    BYTE        bPrio;          // ??: highest priority of all threads of the process 
    BYTE        fFlags;         // ??: process flag
    DWORD       dwAffinity;     // ??: process affinity
    LPVOID      lpUnused;       // ??: was stub thread
    CRITICAL_SECTION csVM;      // ??: CS to control VM access
    CRITICAL_SECTION csLoader;  // ??: CS to control loading of DLL
    FAST_LOCK   hndlLock;
    DLIST       modList;        // ??: list of module loaded by the process 
    DLIST       viewList;       // ??: per-process view list 
    HANDLE      hMQDebuggeeWrite;    // ??: Message queue to write debug events
    PFNVOID     pfnTlsCleanup;  // ??: function to call on thread detach for cleanup of any TLS stored in this process
    HANDLE      hProcQuery;     // ??: handle to current process but with only query privileges
    DWORD       dwKrnTime;      // ??: total kernel time for all exited threads in the process
    DWORD       dwUsrTime;      // ??: total User time for all exited threads in the process
    FILETIME    ftCreate;       // ??: process creation time
    FILETIME    ftExit;         // ??: process exit time
    HANDLE      hTokCreator;    // ??: token of thread who created this process (could be NULL if process created by system)
    LONG        nCurSysTag;     // ??: current system virtual allocation tag value in this process
    WORD        wMemoryPriority; // ??: memory allocation priority
    WORD        fSystemProcess; // ??  if the process is a system process
    PAGEOUTOBJ  pageoutobj;     // ??: pageout object
    MEMSTAT     msCommit;       // ??; committed pages (VirtualAlloc + DLL/EXE globals + stack)
    MEMSTAT     msCodePaged;    // ??: pageable code pages
    MEMSTAT     msCodeNonPaged; // ??: non-pageable code pages
    MEMSTAT     msSharePaged;   // ??: pageable shared pages (e.g. file-backed mapfile)
    MEMSTAT     msShareNonPaged;// ??: non-pageable shared pages (e.g. ram-backed mapfile)
    LONG        lCntPageFault;  // ??: # of page fault taken
};