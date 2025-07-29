#![no_std]
#![no_main]

use uefi::{Status};
mod uart2;
use core::panic::PanicInfo;

const MMIO_UART_ADDRESS: usize = 0xFE02C000;

#[no_mangle]
pub extern "efiapi" fn efi_main() -> Status {
    let hack = uart2::Uart::new(MMIO_UART_ADDRESS);
    hack.write_byte(b'\n');
    // hack.write(b"ello mate");
    hack.write_byte(b'\n');
    hack.write_byte(b't');
    hack.write_byte(b'e');
    hack.write_byte(b's');
    hack.write_byte(b't');
    hack.write_byte(b'\n');

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #![no_std]
// #![no_main]

// use r_efi::efi;
// use r_efi::protocols::graphics_output;
// use core::panic::PanicInfo;
// use core::ptr;
// mod uart_debug;

// // Driver entry point
// #[no_mangle]
// pub extern "efiapi" fn efi_main(
//     image_handle: efi::Handle,
//     system_table: *mut efi::SystemTable,
// ) -> efi::Status {
//     unsafe {
//         match graphics_driver_main(image_handle, system_table) {
//             Ok(_) => efi::Status::SUCCESS,
//             Err(status) => status,
//         }
//     }
// }

// unsafe fn graphics_driver_main(
//     _image_handle: efi::Handle,
//     system_table: *mut efi::SystemTable,
// ) -> Result<(), efi::Status> {
//     let system_table = &mut *system_table;
//     let boot_services = &mut *system_table.boot_services;
    
//     // Locate Graphics Output Protocol
//     let mut gop: *mut graphics_output::Protocol = ptr::null_mut();
//     let mut protocol_guid = graphics_output::PROTOCOL_GUID;
//     let status = (boot_services.locate_protocol)(
//         &mut protocol_guid as *mut efi::Guid,
//         ptr::null_mut(),
//         &mut gop as *mut _ as *mut *mut core::ffi::c_void,
//     );
    
//     if status != efi::Status::SUCCESS {
//         uart_debug::log("Failed to locate Graphics Output Protocol\r\n");
//         return Err(status);
//     }
    
//     let gop = &mut *gop;
    
//     // Get current mode info
//     let mode = &*gop.mode;
//     let mode_info = &*mode.info;
    
//     uart_debug::log("Graphics driver loaded successfully!\r\n");
    
//     // Draw blue box and text
//     draw_blue_box_with_text(gop, mode_info)?;
    
//     Ok(())
// }

// unsafe fn draw_blue_box_with_text(
//     gop: &mut graphics_output::Protocol,
//     mode_info: &graphics_output::ModeInformation,
// ) -> Result<(), efi::Status> {
//     let framebuffer = (*gop.mode).frame_buffer_base as *mut u32;
//     let width = mode_info.horizontal_resolution as usize;
//     let height = mode_info.vertical_resolution as usize;
    
//     // Define box dimensions (centered on screen)
//     let box_width = 400;
//     let box_height = 200;
//     let box_x = (width - box_width) / 2;
//     let box_y = (height - box_height) / 2;
    
//     // Blue color (BGRA format for most UEFI systems)
//     let blue_color = 0xFF0000FF; // Blue with full alpha
//     let white_color = 0xFFFFFFFF; // White for text background
    
//     // Draw blue box
//     for y in box_y..(box_y + box_height) {
//         for x in box_x..(box_x + box_width) {
//             let pixel_offset = y * width + x;
//             *framebuffer.add(pixel_offset) = blue_color;
//         }
//     }
    
//     // Draw white rectangle for text area
//     let text_area_x = box_x + 20;
//     let text_area_y = box_y + 20;
//     let text_area_width = box_width - 40;
//     let text_area_height = 60;
    
//     for y in text_area_y..(text_area_y + text_area_height) {
//         for x in text_area_x..(text_area_x + text_area_width) {
//             let pixel_offset = y * width + x;
//             *framebuffer.add(pixel_offset) = white_color;
//         }
//     }
    
//     // Draw simple text "HELLO UEFI" using pixel art
//     draw_simple_text(
//         framebuffer,
//         width,
//         text_area_x + 10,
//         text_area_y + 10,
//         0xFF000000, // Black text
//     );
    
//     Ok(())
// }

// unsafe fn draw_simple_text(
//     framebuffer: *mut u32,
//     screen_width: usize,
//     start_x: usize,
//     start_y: usize,
//     color: u32,
// ) {
//     // Simple 8x8 pixel font for "HELLO UEFI"
//     // Each character is represented as 8 bytes (8x8 pixels)
    
//     let text = "HELLO UEFI";
//     let char_width = 8;
//     let char_height = 8;
//     let char_spacing = 10;
    
//     // Simple font data for basic characters
//     let text = "HELLO UEFI";
//     let char_width = 8;
//     let char_height = 8;
//     let char_spacing = 10;
    
//     for (i, ch) in text.chars().enumerate() {
//         let char_x = start_x + i * char_spacing;
        
//         if let Some(char_pattern) = get_char_pattern(ch) {
//             for (row, &byte) in char_pattern.iter().enumerate() {
//                 for col in 0..8 {
//                     if (byte & (0x80 >> col)) != 0 {
//                         let pixel_x = char_x + col;
//                         let pixel_y = start_y + row;
//                         let pixel_offset = pixel_y * screen_width + pixel_x;
//                         *framebuffer.add(pixel_offset) = color;
//                     }
//                 }
//             }
//         }
//     }
// }

// fn get_char_pattern(ch: char) -> Option<[u8; 8]> {
//     match ch {
//         'H' => Some([
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b01111110,
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b00000000,
//         ]),
//         'E' => Some([
//             0b01111110,
//             0b01100000,
//             0b01100000,
//             0b01111100,
//             0b01100000,
//             0b01100000,
//             0b01111110,
//             0b00000000,
//         ]),
//         'L' => Some([
//             0b01100000,
//             0b01100000,
//             0b01100000,
//             0b01100000,
//             0b01100000,
//             0b01100000,
//             0b01111110,
//             0b00000000,
//         ]),
//         'O' => Some([
//             0b00111100,
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b00111100,
//             0b00000000,
//         ]),
//         ' ' => Some([
//             0b00000000,
//             0b00000000,
//             0b00000000,
//             0b00000000,
//             0b00000000,
//             0b00000000,
//             0b00000000,
//             0b00000000,
//         ]),
//         'U' => Some([
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b01100110,
//             0b00111100,
//             0b00000000,
//         ]),
//         'F' => Some([
//             0b01111110,
//             0b01100000,
//             0b01100000,
//             0b01111100,
//             0b01100000,
//             0b01100000,
//             0b01100000,
//             0b00000000,
//         ]),
//         'I' => Some([
//             0b00111100,
//             0b00011000,
//             0b00011000,
//             0b00011000,
//             0b00011000,
//             0b00011000,
//             0b00111100,
//             0b00000000,
//         ]),
//         _ => None,
//     }
// }

// // Driver unload function
// #[no_mangle]
// pub extern "efiapi" fn driver_unload(_image_handle: efi::Handle) -> efi::Status {
//     // In a real driver, you might want to restore the original screen content
//     efi::Status::SUCCESS
// }

// // Panic handler (required for no_std)
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }