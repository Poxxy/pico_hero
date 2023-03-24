use embedded_hal::blocking::i2c;
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::{
    bank0::{Gpio0, Gpio1},
    pin, Output, Pin, PushPull,
};

use core::fmt::Write;
use ssd1306::{mode::TerminalMode, prelude::*, I2CDisplayInterface, Ssd1306};

pub struct OLED<'a> {
    // Dimensions of OLED
    height: u32,
    width: u32,
    // I2C
    sda: Pin<Gpio0, Output<PushPull>>,
    scl: Pin<Gpio1, Output<PushPull>>,
    // Message
    text: &'a str,
}

impl<'a> OLED<'a> {
    pub fn new(sda: Pin<Gpio0, Output<PushPull>>, scl: Pin<Gpio1, Output<PushPull>>) -> OLED<'a> {
        OLED {
            height: 128,
            width: 32,
            sda,
            scl,
            text: "Hello World!",
        }
    }

    pub fn update_text(&mut self, message: &'a str) {
        self.text = message;
    }

    pub fn display_text(&mut self) {
        let i2c_component = i2c;
        let interface = I2CDisplayInterface::new(i2c_component);

        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_terminal_mode();
        display.init().unwrap();
        display.clear().unwrap();

        // The `write!()` macro is also supported
        write!(display, "{}", self.text);
    }
}
