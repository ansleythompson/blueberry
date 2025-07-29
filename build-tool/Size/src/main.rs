#![no_std]
#![no_main]
#![allow(non_snake_case)]

extern crate alloc;

use core::panic::PanicInfo;
use r_efi::efi::{Guid, Status};
use r_efi::system::SystemTable;
//mod uart_debug;

pub const G_USB_EXT_PROTOCOL_GUID: Guid = Guid::from_fields(
    0x3a7f1e32,
    0xd5a5,
    0x498a,
    0x8c,
    0x2c,
    &[0x19, 0x27, 0x11, 0x76, 0x18, 0x64],
);

#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: *const core::ffi::c_void,
    system_table: *const SystemTable,
) -> Status {
    unsafe {
        //uart_debug::uart_init();
        //uart_debug::log("ENTER: efi_main entry point");
        let bs = (*system_table).boot_services;
        rust_boot_services_allocator_dxe::GLOBAL_ALLOCATOR.init(bs);

        // Install Protocol


        // Locate Protocol
        /* 
        let status = ((*bs).locate_protocol)(
            &G_USB_EXT_PROTOCOL_GUID as *const _,
            null_mut(),
            &mut protocol as *mut _,
        );
        if status != Status::SUCCESS {
            uart_debug::log("Failed to locate USB protocol.");
            return Status::NOT_FOUND;
        }
*/
        // Create event

    }
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

