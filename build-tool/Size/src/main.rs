#![no_std]
#![no_main]
#![allow(non_snake_case)]


use core::panic::PanicInfo;
use r_efi::efi::Status;
use r_efi::system::SystemTable;
mod uart_debug;

#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: *const core::ffi::c_void,
    _system_table: *const SystemTable,
) -> Status {
    unsafe {
        uart_debug::uart_init();
        uart_debug::log("ENTER: efi_main entry point");
        //let bs = (*system_table).boot_services;
        //rust_boot_services_allocator_dxe::GLOBAL_ALLOCATOR.init(bs);

        // Install Protocol


        // Locate Protocol

        // Create event

    }
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

