#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::base::*;

include!(concat!(env!("OUT_DIR"), "/ntoskrnl.rs"));

#[link(name = "wrapper_ntoskrnl")]
extern "C" {
    pub fn _IoGetCurrentIrpStackLocation(irp: PIRP) -> PIO_STACK_LOCATION;
    
    pub fn _KeQuerySystemTime(current_time: PLARGE_INTEGER);

    pub fn _MmCopyVirtualMemory(source_process: PEPROCESS, source_address: PVOID, target_process: PEPROCESS, target_address: PVOID, buffer_size: SIZE_T, previous_mode: KPROCESSOR_MODE, return_size: PSIZE_T) -> NTSTATUS;

    pub fn _PsGetProcessWow64Process(process: PEPROCESS) -> PVOID;
}

pub use self::_IoGetCurrentIrpStackLocation as IoGetCurrentIrpStackLocation;

pub use self::_KeQuerySystemTime as KeQuerySystemTime;

pub use self::_MmCopyVirtualMemory as MmCopyVirtualMemory;

pub use self::IoGetCurrentProcess as PsGetCurrentProcess;

pub use self::_PsGetProcessWow64Process as PsGetProcessWow64Process;


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

#[macro_export]
macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    }
}