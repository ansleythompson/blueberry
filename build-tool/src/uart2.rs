//! DXE Core Sample AARCH64 Binary
//!   Basic memory mapped UART writer to send and receive bytes for demonstration purposes
//!   Code assumes UART is already fully configured and no timeouts or error handling implemented
//!
//! ## License
//!
//! Copyright (C) Microsoft Corporation. All rights reserved.
//!
//! SPDX-License-Identifier: BSD-2-Clause-Patent
//! 

// use patina_sdk::serial::SerialIO;

///
/// Constants for UART registers
/// 

const THR_REGISTER_OFFSET: usize = 0x00;
const LSR_REGISTER_OFFSET: usize = 0x05;
const LSR_TEMPT: u8 = 0x40;
const LSR_TXRDY: u8 = 0x20;
const LSR_RXRDY: u8 = 0x01;

///
/// Uart module
/// 

#[derive(Debug)]
pub struct Uart {
    base_address: usize,
}

///
/// Uart functions implementation
/// 

impl Uart {
    pub const fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    pub fn write_byte(&self, byte: u8) {
        while self.read_register(LSR_REGISTER_OFFSET) & (LSR_TEMPT | LSR_TXRDY) != (LSR_TEMPT | LSR_TXRDY) { }
        self.write_register(THR_REGISTER_OFFSET, byte);
    }

    fn read_byte(&self) -> Option<u8> {
        if self.read_register(LSR_REGISTER_OFFSET) & LSR_RXRDY == 0 {
            None
        }
        else {
            Some(self.read_register(THR_REGISTER_OFFSET))
        }
    }

    fn read_register(&self, reg: usize) -> u8 {
        let uart_mmio_ptr = self.base_address as *mut u8;
        unsafe {
            uart_mmio_ptr.add(reg).read_volatile()
        }
    }

    fn write_register(&self, reg: usize, byte: u8) {
        let uart_mmio_ptr = self.base_address as *mut u8;
        unsafe {
            uart_mmio_ptr.add(reg).write_volatile(byte)
        }
    }
}


// Implementation of patina_sdk::serial::SerialIO trait
 

// impl SerialIO for Uart {
//     fn init(&self) {}

//     fn write(&self, buffer: &[u8]) {
//         for byte in buffer {
//             self.write_byte(*byte)
//         }
//     }

//     fn read(&self) -> u8 {
//         loop {
//             if let Some(byte) = self.read_byte() {
//                 return byte;
//             }
//         }
//     }

//     fn try_read(&self) -> Option<u8> {
//         self.read_byte()
//     }
// }