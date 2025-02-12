#![allow(non_camel_case_types)]
#![allow(dead_code)]

use {
    bitfield::bitfield, 
    ntapi::ntpsapi::PPS_ATTRIBUTE_LIST, 
    wdk_sys::{
        ACCESS_MASK, HANDLE, KIRQL, KPROCESSOR_MODE, NTSTATUS, PACCESS_STATE, 
        PCUNICODE_STRING, PEPROCESS, PETHREAD, PHANDLE, PKAPC, PKIRQL, POBJECT_ATTRIBUTES, 
        POBJECT_TYPE, PPEB, PRKAPC, PSIZE_T, PULONG, PUNICODE_STRING, PVOID, SIZE_T, ULONG, 
        _DRIVER_OBJECT, KPRIORITY
    }, 
    winapi::ctypes::c_void
};

bitfield! {
    pub struct PS_PROTECTION(u8);
    pub u8, type_, set_type_: 2, 0;   // 3 bits
    pub u8, audit, set_audit: 3;      // 1 bit
    pub u8, signer, set_signer: 7, 4; // 4 bits
}

pub struct PROCESS_SIGNATURE {
    pub signature_level: u8,
    pub section_seginature_level: u8,
    pub protection: PS_PROTECTION,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemModule {
    pub section: *mut c_void,
    pub mapped_base: *mut c_void,
    pub image_base: *mut c_void,
    pub size: u32,
    pub flags: u32,
    pub index: u8,
    pub name_length: u8,
    pub load_count: u8,
    pub path_length: u8,
    pub image_name: [u8; 256],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemModuleInformation {
    pub modules_count: u32,
    pub modules: [SystemModule; 256],
}

pub type DRIVER_INITIALIZE = core::option::Option<
    unsafe extern "system" fn(
        DriverObject: &mut _DRIVER_OBJECT,
        RegistryPath: PCUNICODE_STRING,
    ) -> NTSTATUS,
>;

pub type ZwCreateThreadExType = unsafe extern "system" fn (
    ThreadHandle: PHANDLE,
    DesiredAccess: ACCESS_MASK,
    ObjectAttributes: POBJECT_ATTRIBUTES,
    ProcessHandle: HANDLE,
    StartRoutine: PVOID,
    Argument: PVOID,
    CreateFlags: SIZE_T,
    ZeroBits: usize,
    StackSize: usize,
    MaximumStackSize: usize,
    AttributeList: PPS_ATTRIBUTE_LIST
) -> NTSTATUS;

pub type ZwProtectVirtualMemoryType = unsafe extern "system" fn (
    ProcessHandle: HANDLE,
    BaseAddress: *mut PVOID,
    RegionSize: PSIZE_T,
    NewProtect: ULONG,
    OldProtect: PULONG
) -> NTSTATUS;

pub type PKRUNDOWN_ROUTINE = Option<unsafe extern "system" fn(
    apc: PKAPC,
) -> NTSTATUS>;

pub type PKNORMAL_ROUTINE =  Option<unsafe extern "system" fn(
    normal_context: *mut PVOID,
    system_argument1: *mut PVOID,
    system_argument2: *mut PVOID
) -> NTSTATUS>;

pub type PKKERNEL_ROUTINE = unsafe extern "system" fn(
    apc: PKAPC,
    normal_routine: *mut PKNORMAL_ROUTINE,
    normal_context: *mut PVOID,
    system_argument1: *mut PVOID,
    system_argument2: *mut PVOID 
);

extern "system" {
    pub fn PsGetProcessPeb(ProcessId: PEPROCESS) -> PPEB;
   
    pub fn PsGetCurrentThread() -> PETHREAD;
    
    pub fn IoCreateDriver(
        driver_name: PUNICODE_STRING,
        driver_initialize: DRIVER_INITIALIZE,
    ) -> NTSTATUS;

    pub fn MmCopyVirtualMemory(
        source_process: PEPROCESS,
        source_address: PVOID,
        target_process: PEPROCESS,
        target_address: PVOID,
        buffer_size: SIZE_T,
        previous_mode: KPROCESSOR_MODE,
        return_size: PSIZE_T,
    );

    pub fn ObReferenceObjectByName(
        object_name: PUNICODE_STRING,
        attributes: u32,
        access_state: PACCESS_STATE,
        desired_access: ACCESS_MASK,
        object_type: POBJECT_TYPE,
        access_mode: KPROCESSOR_MODE,
        parse_context: PVOID,
        object: *mut PVOID,
    );

    pub fn KeRaiseIrql(new_irql: KIRQL, old_irql: PKIRQL);

    pub fn KeInitializeApc(
        apc: PRKAPC,
        thread: PETHREAD,
        environment: KAPC_ENVIROMENT,
        kernel_routine: PKKERNEL_ROUTINE,
        rundown_routine: PKRUNDOWN_ROUTINE,
        normal_routine: PKNORMAL_ROUTINE,
        apc_mode: KPROCESSOR_MODE,
        normal_context: PVOID
    );

    pub fn KeTestAlertThread(
        alert_mode: KPROCESSOR_MODE
    );

    pub fn KeInsertQueueApc(
        apc: PRKAPC,
        system_argument1: PVOID,
        system_argument2: PVOID,
        increment: KPRIORITY
    ) -> bool;
}

#[repr(C)]
pub enum KAPC_ENVIROMENT {
    OriginalApcEnvironment,
    AttachedApcEnvironment,
    CurrentApcEnvironment,
    InsertApcEnvironment
}
