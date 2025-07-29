#![no_std]
#![no_main]
#![allow(non_snake_case)]

extern crate alloc;
extern crate r_efi;


use core::panic::PanicInfo;

use r_efi::efi::{Guid, Status, SystemTable};
use core::ptr::null_mut;
use core::ffi::c_void;

mod uart_debug;


// Match EFI_TIMER_ARCH_PROTOCOL_GUID from Edk2 <Protocol/Timer.h>
pub const EFI_TIMER_ARCH_PROTOCOL_GUID: Guid = Guid::from_fields(
    0x26baccb3, 
    0x6f42, 
    0x11d4, 
    0xbc, 
    0xe7, 
    &[0x0, 0x80, 0xc7, 0x3c, 0x88, 0x81],
);

//initialize global vars - several hrs later actually no global bc no shot I am multithreading 
static mut GBS: Option<r_efi::efi::BootServices> = None;

//SetTimerPeriodCallback
//Event, Context Status TimerPeriod, *TimerProtocol 
//locate timer protocol with EfiTimerArchProtocolGuid 
extern "efiapi" fn set_timer_period_callback(
    _event: r_efi::efi::Event, 
    context: *mut core::ffi::c_void,
) -> () { //it wouldnt let me build if I returned status success? MO TODO add it back??
    unsafe 
    {
        uart_debug::log("ENTER: SetTimerPeriodCallback");
    }
    
    //unwrap the gbs enum or I guess just read context ptr?
    let gbs = context as *mut r_efi::efi::BootServices;

    //locate protocol 
    let mut timer: *mut core::ffi::c_void = null_mut();
    let mut timer_guid = EFI_TIMER_ARCH_PROTOCOL_GUID;
    let status = unsafe
    {
        ((*gbs).locate_protocol)(
        &mut timer_guid as *mut _,
        null_mut(),
        &mut timer as *mut _,
        )
    };

    if status != Status::SUCCESS 
    {
        // if I can make using regex for dbg work,  Status = 0x{:X}", status.as_usize());
        panic!("Failed to locate Timer protocol") //switch this back to rturn if needed
    }
    
    //set sys timer period
    let _timer_period = 10000; //PcdGet64 (PcdSystemTimerPeriod)
    /*
    let status = 
    
    //truly no idea how to assert in rust - macro has gotta exist somewhere just didnt have time to find it - for now just panic!
    //use bs to close event
    close_event
*/
    //return, see line 40/~103ish bc maybe I cant/ shouldnt return? 
    // Status::SUCCESS.as_usize() as u64
}


#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: *const core::ffi::c_void,
    system_table: *const SystemTable,
) -> u64 {
    
    //let gbs_ptr = GBS.unwrap(); This didnt work, but it does handle the enum properly so i might want if I go back to that 
    unsafe 
    {
        uart_debug::log("ENTER: DellSetTimerPeriod entry point");
        GBS =(*system_table).boot_services;
        rust_boot_services_allocator_dxe::GLOBAL_ALLOCATOR.init(gbs_ptr);
    }
    
        //Efi Create Protocol Notify Event 
        //create event first w/ tpl callback, set function, 
        let event: *mut core::ffi::c_void = null_mut();
        let status = ((*gbs_ptr).create_event)(
                r_efi::efi::EVT_NOTIFY_SIGNAL,
                r_efi::efi::TPL_CALLBACK,
                Some(set_timer_period_callback as extern "efiapi" fn(r_efi::efi::Event, *mut core::ffi::c_void) -> () ),
                gbs_ptr as *mut c_void,//context 
                event as *mut *mut c_void,
            );
        if status != Status::SUCCESS 
        {
            panic!("Failed to locate Timer protocol") //switch this back to rturn if needed
        }

        //then register it for the protocol notify 
        let _regs = ((*gbs_ptr).register_protocol_notify)(
                &EFI_TIMER_ARCH_PROTOCOL_GUID as *const Guid as *mut Guid,
                
                null_mut(), //MO:TODO fix to actually be status 
                event as *mut *mut c_void,
            );
    // ok I had to wrap everything in unsafe block but that literally defeats the point - gotta ask Ansley if theres a better way to do gbs (semi globally/ pass between without going out of scope)
        // return EFI_SUCCESS;
        Status::SUCCESS.as_usize() as u64
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

