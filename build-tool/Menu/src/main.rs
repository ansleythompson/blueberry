#![no_std]
#![no_main]
#![allow(non_snake_case)]

extern crate alloc;

use core::panic::PanicInfo;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, Ordering};
use r_efi::efi::{Guid, Status, Event};
use r_efi::efi;
use r_efi::protocols::simple_text_input::{InputKey, Protocol as SimpleTextInput};
use r_efi::system::SystemTable;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
mod uart_debug;
use r_efi::protocols::graphics_output;
use alloc::vec::Vec; // Vecor functionality


// global varibales to store USB information. Static: Entire life of a program.
static mut USB_MANUFACTURER: Option<String> = None;
static mut USB_PRODUCT: Option<String> = None;
static mut USB_SERIAL: Option<String> = None;
static USB_UPDATED: AtomicBool = AtomicBool::new(false);
static mut USB_DEV_INFO_VEC: Option<Vec<DellUsbDevInfo>> = None;  // Vecor functionality


// newly added variables or info hat we can print on blue screen (Take advise from MO & AS)
static mut USB_VENDOR_ID: u16 = 0;
static mut USB_PRODUCT_ID: u16 = 0;
static mut USB_CLASS: u8 = 0;
static mut USB_SUBCLASS: u8 = 0;
static mut USB_BUS: u32 = 0;
static mut USB_DEVICE: u8 = 0;
static mut USB_FUNCTION: u8 = 0;
static mut USB_PORT: u8 = 0;
static mut USB_INTERFACE: u8 = 0;

// Match gUsbExtProtocolGuid from C header
pub const G_USB_EXT_PROTOCOL_GUID: Guid = Guid::from_fields(
    0x3a7f1e32,
    0xd5a5,
    0x498a,
    0x8c,
    0x2c,
    &[0x19, 0x27, 0x11, 0x76, 0x18, 0x64],
);

#[derive(Clone)]
#[repr(C)]
pub struct PortInfo {
    pub interface: u8,
    pub port: u8,
    pub function: u8,
    pub device: u8,
    pub bus: u32,
}


#[derive(Clone)]
#[repr(C)]
pub struct UsbDeviceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_usb: u16,
    pub device_class: u8,
    pub device_sub_class: u8,
    pub device_protocol: u8,
    pub max_packet_size0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: u16,
    pub str_manufacturer: u8,
    pub str_product: u8,
    pub str_serial_number: u8,
    pub num_configurations: u8,
}


#[derive(Clone)]
#[repr(C)]
pub struct UsbInterfaceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub num_endpoints: u8,
    pub interface_class: u8,
    pub interface_sub_class: u8,
    pub interface_protocol: u8,
    pub interface: u8,
}


#[derive(Clone)]
#[repr(C)]
pub struct DellUsbDevInfo {
    pub signature: u32,
    pub device_descriptor: UsbDeviceDescriptor,
    pub interface_descriptor: UsbInterfaceDescriptor,
    pub manufacturer: [u16; 64],
    pub product: [u16; 64],
    pub serial_number: [u16; 64],
    pub port_info: PortInfo,
}

#[repr(C)]
pub struct UsbExtProtocol {
    pub usb_dev_info: DellUsbDevInfo,
    pub update_event: Event,
    pub event_type: u32,
}

// UEFI Event and Timer constants
const TIMER_RELATIVE: u32 = 1;
const SCAN_F4: u16 = 0xE; // it is a scan code for F4 0xE. 

// it is a pointer to Graphics Protocol
static mut gop_ptr: *mut graphics_output::Protocol = core::ptr::null_mut();
// this is a pointer to a keyboard input protoocl
static mut CON_IN: *mut SimpleTextInput = core::ptr::null_mut();
// flag whether the box is visible (currently set to false)
static mut BOX_VISIBLE: bool = false;


// Screen Setup Info - Screen Resolution, Pixel Format, and Frame Buffer (A frame buffer is a special area in memory (RAM) that stores the pixels that will be shown on your screen.)
#[repr(C)]
struct GraphicsOutputModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: u32,
    pixel_info: [u32; 4],
    pixels_per_scan_line: u32,
}

#[repr(C)]
struct GraphicsOutputMode {
    max_mode: u32,
    mode: u32,
    info: *mut GraphicsOutputModeInformation,
    size_of_info: usize,
    frame_buffer_base: *mut core::ffi::c_void,
    frame_buffer_size: usize,
}

#[repr(C)]
struct GraphicsOutput {
    _query_mode: usize,
    _set_mode: usize,
    _blt: usize,
    mode: *mut GraphicsOutputMode,
}

// This function converts a UTF-16 encoded string from UEFI into a normal Rust String.
fn utf16_cstr_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

// it is a callback function when the USB event is triggered
extern "efiapi" fn on_usb_update(_event: r_efi::efi::Event, context: *mut core::ffi::c_void) {
    // it converts a generic UEFI pointer to a USB protocol struct and save it in the variable 'protocol'
    let protocol = context as *mut UsbExtProtocol;
    if protocol.is_null() {
        unsafe {
            uart_debug::log("USB context is null.");
        }
        return;
    }

    unsafe {
            USB_MANUFACTURER = Some(utf16_cstr_to_string(&(*protocol).usb_dev_info.manufacturer));
            USB_PRODUCT = Some(utf16_cstr_to_string(&(*protocol).usb_dev_info.product));
            USB_SERIAL = Some(utf16_cstr_to_string(&(*protocol).usb_dev_info.serial_number));
            USB_UPDATED.store(true, Ordering::SeqCst);

            if USB_DEV_INFO_VEC.is_none() {
                USB_DEV_INFO_VEC = Some(Vec::new());
            }
            if let Some(vec) = USB_DEV_INFO_VEC.as_mut() {
                vec.push((*protocol).usb_dev_info.clone());
            }

            // newly added parameteres
            let dev_desc = &(*protocol).usb_dev_info.device_descriptor;
            let port_info = &(*protocol).usb_dev_info.port_info;

            USB_VENDOR_ID = dev_desc.id_vendor;
            USB_PRODUCT_ID = dev_desc.id_product;
            USB_CLASS = dev_desc.device_class;
            USB_SUBCLASS = dev_desc.device_sub_class;

            USB_BUS = port_info.bus;
            USB_DEVICE = port_info.device;
            // USB_FUNCTION = port_info.fn;
            USB_PORT = port_info.port;
            USB_INTERFACE = port_info.interface;

            uart_debug::log("USB updated");

        }
}


// checks every 0.5 seconds whther the F4 key is pressed and if pressed call the blue box or else clear the box and toggle the flag
extern "efiapi" fn poll_keys(_event: *mut core::ffi::c_void, _context: *mut core::ffi::c_void) {
    unsafe {
        // uart_debug::log("poll_keys callback triggered");
        if CON_IN.is_null() || gop_ptr.is_null() {
            uart_debug::log("CON_IN or gop_ptr is null. Exiting poll_keys.");
            return;
        }
        // uart_debug::log("poll keys started");
        let con_in = CON_IN;
        // store the key that was pressed (we expect it to F4)
        let mut key: InputKey = core::mem::zeroed();
        // if pressed, fill the key
        let status = ((*con_in).read_key_stroke)(con_in, &mut key as *mut _);
        if status == Status::NOT_READY {
            // uart_debug::log("No key pressed at this time.");
            return;
        } else if status != Status::SUCCESS {
            uart_debug::log("Error reading key stroke."); //  Status = 0x{:X}", status.as_usize());
            return;
        }

        // uart_debug::log("1 Key pressed"); //ScanCode = 0x{:X}", key.scan_code);
        if (key.scan_code & 0xFF) == SCAN_F4 {

            let gop = &mut *gop_ptr;
            let mode = &*gop.mode;
            let info = &*mode.info;
            let fb = mode.frame_buffer_base as *mut u32;
            let ppsl = info.pixels_per_scan_line;
            uart_debug::log("Framebuffer base"); //: {:p}, Resolution: {}x{}", fb, info.horizontal_resolution, info.vertical_resolution);
            if BOX_VISIBLE {
                clear_box(fb, ppsl);
            } else {
                draw_box(fb, ppsl);
            }

            BOX_VISIBLE = !BOX_VISIBLE;
        }
    }
}

fn draw_box(fb: *mut u32, ppsl: u32) {
    // Draw the blue box background
    for y in 100..250 {
        for x in 100..600 {
            let idx = (y as usize * ppsl as usize + x as usize) as usize;
            unsafe {
                // *fb.offset(idx) = 0x000000FF; // Blue color
                *fb.add(idx) = 0xFF0000FF;
            }
        }
    }

let text = unsafe {
    if let Some(vec) = USB_DEV_INFO_VEC.as_ref() {
        let mut all_info = String::from("USB Devices:\n");
        for (i, dev_info) in vec.iter().enumerate() {
            let manufacturer = utf16_cstr_to_string(&dev_info.manufacturer);
            let product = utf16_cstr_to_string(&dev_info.product);
            let serial = utf16_cstr_to_string(&dev_info.serial_number);
            let dev_desc = &dev_info.device_descriptor;
            let port_info = &dev_info.port_info;
            let info = format!(
                "Device {}:\nManufacturer: {}\nProduct: {}\nSerial: {}\nVendor ID: {:04X}, Product ID: {:04X}, Class: {:02X}, Subclass: {:02X}\nBus: {}, Device: {}, Port: {}, Interface: {}\n\n",
                i + 1,
                manufacturer,
                product,
                serial,
                dev_desc.id_vendor,
                dev_desc.id_product,
                dev_desc.device_class,
                dev_desc.device_sub_class,
                port_info.bus,
                port_info.device,
                port_info.port,
                port_info.interface
            );
            all_info.push_str(&info);
        }
        all_info
    } else {
        "Waiting for USB...".to_string()
    }
};

    // Draw the text inside the box
    draw_text(fb, ppsl, 120, 120, 0x00FFFFFF, &text); // White text
    unsafe {
        uart_debug::log("Blue box drawn with dynamic USB data.");
    }
}

fn clear_box(fb: *mut u32, ppsl: u32) {
    // Draw the blue box background
    let mode_info = unsafe { &*(*(*gop_ptr).mode).info };

    // let width = mode_info.horizontal_resolution as usize;
    // let height = mode_info.vertical_resolution as usize;
    // let box_width = 800; // 100..250, 100..600
    // let box_height = 400;
    // let box_x = (width - box_width) / 2;
    // let box_y = (height - box_height) / 2;

    // for y in box_y..(box_y + box_height) {
    //     for x in box_x..(box_x + box_width) {
    for y in 100..250 {
        for x in 100..600 {
            let idx = (y as usize * ppsl as usize + x as usize) as isize;
            unsafe {
                *fb.offset(idx) = 0xFF000000;
            }
        }
    }
    unsafe {
        uart_debug::log("Blue box cleared.");
    }
}


fn draw_char(fb: *mut u32, ppsl: u32, x: usize, y: usize, color: u32, ch: char) {
    let bitmap: Option<[u8; 8]> = match ch {
        'A' => Some([0x18, 0x24, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00]),
        'B' => Some([0x7C, 0x42, 0x42, 0x7C, 0x42, 0x42, 0x7C, 0x00]),
        'C' => Some([0x3C, 0x42, 0x40, 0x40, 0x40, 0x42, 0x3C, 0x00]),
        'D' => Some([0x78, 0x44, 0x42, 0x42, 0x42, 0x44, 0x78, 0x00]),
        'E' => Some([0x7E, 0x40, 0x40, 0x7C, 0x40, 0x40, 0x7E, 0x00]),
        'F' => Some([0x7E, 0x40, 0x40, 0x7C, 0x40, 0x40, 0x40, 0x00]),
        'G' => Some([0x3C, 0x42, 0x40, 0x4E, 0x42, 0x42, 0x3C, 0x00]),
        'H' => Some([0x42, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00]),
        'I' => Some([0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00]),
        'J' => Some([0x1E, 0x08, 0x08, 0x08, 0x08, 0x48, 0x30, 0x00]),
        'K' => Some([0x42, 0x44, 0x48, 0x70, 0x48, 0x44, 0x42, 0x00]),
        'L' => Some([0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x7E, 0x00]),
        'M' => Some([0x42, 0x66, 0x5A, 0x5A, 0x42, 0x42, 0x42, 0x00]),
        'N' => Some([0x42, 0x62, 0x52, 0x4A, 0x46, 0x42, 0x42, 0x00]),
        'O' => Some([0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00]),
        'P' => Some([0x7C, 0x42, 0x42, 0x7C, 0x40, 0x40, 0x40, 0x00]),
        'Q' => Some([0x3C, 0x42, 0x42, 0x42, 0x4A, 0x44, 0x3A, 0x00]),
        'R' => Some([0x7C, 0x42, 0x42, 0x7C, 0x48, 0x44, 0x42, 0x00]),
        'S' => Some([0x3C, 0x42, 0x40, 0x3C, 0x02, 0x42, 0x3C, 0x00]),
        'T' => Some([0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00]),
        'U' => Some([0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00]),
        'V' => Some([0x42, 0x42, 0x42, 0x42, 0x42, 0x24, 0x18, 0x00]),
        'W' => Some([0x42, 0x42, 0x42, 0x5A, 0x5A, 0x66, 0x42, 0x00]),
        'X' => Some([0x42, 0x42, 0x24, 0x18, 0x24, 0x42, 0x42, 0x00]),
        'Y' => Some([0x42, 0x42, 0x24, 0x18, 0x18, 0x18, 0x18, 0x00]),
        'Z' => Some([0x7E, 0x02, 0x04, 0x18, 0x20, 0x40, 0x7E, 0x00]),
        'a' => Some([0x00, 0x3C, 0x02, 0x3E, 0x42, 0x46, 0x3A, 0x00]),
        'b' => Some([0x40, 0x40, 0x5C, 0x62, 0x42, 0x62, 0x5C, 0x00]),
        'c' => Some([0x00, 0x3C, 0x42, 0x40, 0x40, 0x42, 0x3C, 0x00]),
        'd' => Some([0x02, 0x02, 0x3A, 0x46, 0x42, 0x46, 0x3A, 0x00]),
        'e' => Some([0x00, 0x3C, 0x42, 0x7E, 0x40, 0x3C, 0x00, 0x00]),
        'f' => Some([0x1C, 0x20, 0x7C, 0x20, 0x20, 0x20, 0x20, 0x00]),
        'g' => Some([0x00, 0x3A, 0x46, 0x42, 0x46, 0x3A, 0x02, 0x3C]),
        'h' => Some([0x40, 0x40, 0x5C, 0x62, 0x42, 0x42, 0x42, 0x00]),
        'i' => Some([0x10, 0x00, 0x30, 0x10, 0x10, 0x10, 0x38, 0x00]),
        'j' => Some([0x08, 0x00, 0x18, 0x08, 0x08, 0x48, 0x48, 0x30]),
        'k' => Some([0x40, 0x44, 0x48, 0x70, 0x48, 0x44, 0x42, 0x00]),
        'l' => Some([0x30, 0x10, 0x10, 0x10, 0x10, 0x10, 0x38, 0x00]),
        'm' => Some([0x00, 0x6C, 0x92, 0x92, 0x92, 0x92, 0x92, 0x00]),
        'n' => Some([0x00, 0x5C, 0x62, 0x42, 0x42, 0x42, 0x42, 0x00]),
        'o' => Some([0x00, 0x3C, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00]),
        'p' => Some([0x00, 0x5C, 0x62, 0x62, 0x5C, 0x40, 0x40, 0x00]),
        'q' => Some([0x00, 0x3A, 0x46, 0x46, 0x3A, 0x02, 0x02, 0x00]),
        'r' => Some([0x00, 0x5C, 0x62, 0x40, 0x40, 0x40, 0x40, 0x00]),
        's' => Some([0x00, 0x3E, 0x40, 0x3C, 0x02, 0x42, 0x3C, 0x00]),
        't' => Some([0x20, 0x20, 0x7C, 0x20, 0x20, 0x22, 0x1C, 0x00]),
        'u' => Some([0x00, 0x42, 0x42, 0x42, 0x42, 0x46, 0x3A, 0x00]),
        'v' => Some([0x00, 0x42, 0x42, 0x42, 0x24, 0x24, 0x18, 0x00]),
        'w' => Some([0x00, 0x92, 0x92, 0x92, 0x92, 0x92, 0x6C, 0x00]),
        'x' => Some([0x00, 0x42, 0x24, 0x18, 0x18, 0x24, 0x42, 0x00]),
        'y' => Some([0x00, 0x42, 0x42, 0x46, 0x3A, 0x02, 0x42, 0x3C]),
        'z' => Some([0x00, 0x7E, 0x04, 0x08, 0x10, 0x20, 0x7E, 0x00]),
        '0' => Some([0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00]),
        '1' => Some([0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00]),
        '2' => Some([0x3C, 0x66, 0x06, 0x0C, 0x30, 0x60, 0x7E, 0x00]),
        '3' => Some([0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00]),
        '4' => Some([0x0C, 0x1C, 0x2C, 0x4C, 0x7E, 0x0C, 0x0C, 0x00]),
        '5' => Some([0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00]),
        '6' => Some([0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00]),
        '7' => Some([0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00]),
        '8' => Some([0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00]),
        '9' => Some([0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00]),
        ',' => Some([0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30]),
        '.' => Some([0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00]),
        ':' => Some([0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00]),
        '\\' => Some([0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01, 0x00]),
        ' ' => Some([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        _ => None,
    };

    if let Some(bitmap) = bitmap {
        for (row, byte) in bitmap.iter().enumerate() {
            for col in 0..8 {
                if (byte >> (7 - col)) & 1 == 1 {
                    let idx = ((y + row) * ppsl as usize + (x + col)) as isize;
                    unsafe {
                        *fb.offset(idx) = color;
                    }
                }
            }
        }
    }
}

fn draw_text(fb: *mut u32, ppsl: u32, x: usize, y: usize, color: u32, text: &str) {

    let mut cursor_x = x;
    let mut cursor_y = y;
    for ch in text.chars() {
        if ch == '\n' {
            cursor_x = x;
            cursor_y += 12; // Move down for next line
        } else {
            draw_char(fb, ppsl, cursor_x, cursor_y, color, ch);
            cursor_x += 10; // Move right for next character
        }
    }
}


unsafe fn draw_blue_box_with_text(
    gop: &mut graphics_output::Protocol,
    mode_info: &graphics_output::ModeInformation,
) -> Result<(), efi::Status> {
    let framebuffer = (*gop.mode).frame_buffer_base as *mut u32;
    let width = mode_info.horizontal_resolution as usize;
    let height = mode_info.vertical_resolution as usize;
    
    // Define box dimensions (centered on screen)
    let box_width = 400;
    let box_height = 200;
    let box_x = (width - box_width) / 2;
    let box_y = (height - box_height) / 2;
    
    // Blue color (BGRA format for most UEFI systems)
    let blue_color = 0xFF0000FF; // Blue with full alpha

    // Draw blue box
    for y in box_y..(box_y + box_height) {
        for x in box_x..(box_x + box_width) {
            let pixel_offset = y * width + x;
            *framebuffer.add(pixel_offset) = blue_color;
        }
    }
    Ok(())
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: *const core::ffi::c_void,
    system_table: *const SystemTable,
) -> u64 {
    unsafe {
        let bs = (*system_table).boot_services;
        rust_boot_services_allocator_dxe::GLOBAL_ALLOCATOR.init(bs);

        uart_debug::log("Driver loaded and running in background.");

        // Locate Graphics Output Protocol (GOP)
        let mut gop_guid = graphics_output::PROTOCOL_GUID;
        let status = ((*bs).locate_protocol)(
            &mut gop_guid as *mut _,
            core::ptr::null_mut(),
            &mut gop_ptr as *mut _ as *mut *mut core::ffi::c_void,
        );

        if status != Status::SUCCESS || gop_ptr.is_null() {
            uart_debug::log("Failed to locate GOP.");
            return Status::NOT_FOUND.as_usize() as u64;
        }

        uart_debug::log("Successfully located GOP.");
        gop_ptr = &mut *gop_ptr;

        // Set up console input
        CON_IN = (*system_table).con_in;

        // Locate USB protocol and set up USB event
        let mut usb_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        let mut usb_guid = G_USB_EXT_PROTOCOL_GUID;
        let status = ((*bs).locate_protocol)(
            &mut usb_guid as *mut _,
            core::ptr::null_mut(),
            &mut usb_ptr as *mut _,
        );

        if status == Status::SUCCESS && !usb_ptr.is_null() {
            let usb_protocol = usb_ptr as *mut UsbExtProtocol;
            let mut usb_event: r_efi::efi::Event = core::ptr::null_mut();
            let status = ((*bs).create_event)(
                r_efi::efi::EVT_NOTIFY_SIGNAL,
                r_efi::efi::TPL_CALLBACK,
                Some(on_usb_update),
                usb_ptr,
                &mut usb_event,
            );

            if status == Status::SUCCESS {
                (*usb_protocol).update_event = usb_event;
                ((*bs).signal_event)(usb_event);
                uart_debug::log("USB protocol registered.");
            } else {
                uart_debug::log("Failed to create USB event.");
            }
        } else {
            uart_debug::log("USB protocol not found.");
        }

        // Wait for key input once
        let wait_event = (*CON_IN).wait_for_key;
        let mut index: usize = 0;
        let mut events: [r_efi::efi::Event; 1] = [wait_event];
        let status = ((*bs).wait_for_event)(1, events.as_mut_ptr(), &mut index);

        if status == Status::SUCCESS {
            let mut key: InputKey = core::mem::zeroed();
            let status = ((*CON_IN).read_key_stroke)(CON_IN, &mut key);
            if status == Status::SUCCESS && (key.scan_code & 0xFF) == SCAN_F4 {
                let gop = &mut *gop_ptr;
                let mode = &*gop.mode;
                let info = &*mode.info;
                let fb = mode.frame_buffer_base as *mut u32;
                let ppsl = info.pixels_per_scan_line;

                draw_box(fb, ppsl);
            }
        }

        // Exit cleanly
        return Status::SUCCESS.as_usize() as u64;
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}








// #[no_mangle]
// pub extern "efiapi" fn efi_main(
//     _image_handle: *const core::ffi::c_void,
//     system_table: *const SystemTable,
// ) -> u64 {
//     unsafe {
//         let bs = (*system_table).boot_services;
//         rust_boot_services_allocator_dxe::GLOBAL_ALLOCATOR.init(bs);

//         uart_debug::log("Driver loaded and running in background.");

//         // Locate GOP
//         let mut gop_guid = graphics_output::PROTOCOL_GUID;
//         // let mut gop_ptr: *mut core::ffi::c_void = ptr::null_mut();
//         let status = ((*bs).locate_protocol)(
//             &mut gop_guid as *mut _,
//             null_mut(),
//             // &mut gop_ptr as *mut _,
//             &mut gop_ptr as *mut _ as *mut *mut core::ffi::c_void,
//         );

//         if status != Status::SUCCESS || gop_ptr.is_null() {
//             uart_debug::log("Failed to locate GOP.");
//             return Status::NOT_FOUND.as_usize() as u64;
//         }
        
//         uart_debug::log("Successfully located GOP. ");
//         // Set global variables as poll keys can use them

//         gop_ptr = &mut *gop_ptr;
//         // Get current mode info
//         let mode = &*(*gop_ptr).mode;
//         let mode_info = &*mode.info;
//         // let _ = draw_blue_box_with_text(gop_ptr, mode_info);
//         CON_IN = (*system_table).con_in;


//         // Create timer event
//         uart_debug::log("Effort to start timer event.");
//         let mut event: *mut core::ffi::c_void = null_mut();
//         let status = ((*bs).create_event)(
//             r_efi::efi::EVT_TIMER | r_efi::efi::EVT_NOTIFY_SIGNAL,
//             r_efi::efi::TPL_CALLBACK,
//             Some(poll_keys),
//             null_mut(),
//             &mut event as *mut _,
//         );

//         if status != Status::SUCCESS {
//             uart_debug::log("Failed to create timer event.");
//             uart_debug::log("Failed to create timer event."); // Status = 0x{:X}", status.as_usize());
//             return status.as_usize() as u64;
//         }

//         uart_debug::log("Successfully set the timer event.");
//         // Set timer to fire every 0.5 seconds
//         let status = ((*bs).set_timer)(
//             event,
//             TIMER_RELATIVE,
//             500_000, // 0.5 seconds in 100ns units
//         );

//         uart_debug::log("Successfully set the timer event after every 0.5 secs");
//         if status != Status::SUCCESS {
//             uart_debug::log("Failed to set timer.");
//             return status.as_usize() as u64;
//         }

//         uart_debug::log("Polling for F4 key every 0.5s. Shell remains active.");
//         // Locate USB protocol
//         let mut usb_ptr: *mut core::ffi::c_void = null_mut();
//         let mut usb_guid = G_USB_EXT_PROTOCOL_GUID;
//         let status = ((*bs).locate_protocol)(
//             &mut usb_guid as *mut _,
//             null_mut(),
//             &mut usb_ptr as *mut _,
//         );

//         if status == Status::SUCCESS && !usb_ptr.is_null() {
//             let usb_protocol = usb_ptr as *mut UsbExtProtocol;

//             let mut usb_event: r_efi::efi::Event = null_mut();
//             let status = ((*bs).create_event)(
//                 r_efi::efi::EVT_NOTIFY_SIGNAL,
//                 r_efi::efi::TPL_CALLBACK,
//                 Some(on_usb_update),
//                 usb_ptr,
//                 &mut usb_event,
//             );

//             if status == Status::SUCCESS {
//                 (*usb_protocol).update_event = usb_event;
//                 ((*bs).signal_event)(usb_event);
//                 uart_debug::log("USB protocol registered.");
//             } else {
//                 uart_debug::log("Failed to create USB event.");
//             }
//         } else {
//             uart_debug::log("USB protocol not found.");
//         }
//                 Status::SUCCESS.as_usize() as u64
//             }
//         }
