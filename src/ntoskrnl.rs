#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::base::*;

include!(concat!(env!("OUT_DIR"), "/ntoskrnl.rs"));

#[link(name = "wrapper_ntoskrnl")]
extern "C" {
    pub fn _IoGetCurrentIrpStackLocation(irp: PIRP) -> PIO_STACK_LOCATION;
    
    pub fn _KeQuerySystemTime(current_time: PLARGE_INTEGER);
}

pub use self::_IoGetCurrentIrpStackLocation as IoGetCurrentIrpStackLocation;

pub use self::_KeQuerySystemTime as KeQuerySystemTime;


#[macro_export]
macro_rules! InitializeObjectAttributes {
    ($p:expr, $n:expr, $a:expr, $r:expr, $s:expr) => {
        {
            $p.Length = std::mem::size_of::<OBJECT_ATTRIBUTES>() as ULONG;
            $p.RootDirectory = $r;
            $p.Attributes = $a;
            $p.ObjectName = $n;
            $p.SecurityDescriptor = $s;
            $p.SecurityQualityOfService = std::ptr::null_mut();
        }
    };
}

#[macro_export]
macro_rules! NT_SUCCESS {
    ($status:expr) => {
        ($status as NTSTATUS) >= 0
    };
}