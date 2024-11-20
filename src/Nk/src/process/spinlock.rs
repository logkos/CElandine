pub struct _SPINLOCK {
    pub owner_cpu: *mut u32,  // which CPU own this spin lock
    pub nestcount: *mut u32,  // nested count (for nested calls)
    pub next_owner: *mut u32, // can only be either 0 or MASTER_CPU, only updated by master CPU
}

pub unsafe fn InitializeSpinLock(psplock: *mut _SPINLOCK) -> () {
    (*psplock).owner_cpu = 0 as *mut u32;
    (*psplock).nestcount = 0 as *mut u32;
    (*psplock).next_owner = 0 as *mut u32;
}
pub fn AcquireSpinLock(psplock: *mut _SPINLOCK) -> ! {
    !todo!("AcquireSpinLock");
}

pub fn ReleaseSpinLock(psplock: *mut _SPINLOCK) -> ! {
    !todo!("ReleaseSpinLock");
}

pub fn EnablePremption(psplock: *mut _SPINLOCK) -> ! {
    !todo!("EnablePremption");
}
