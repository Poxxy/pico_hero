use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::{
    bank0::{Gpio0, Gpio1},
    pin, Output, Pin, PushPull,
};

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

    pub fn display(&mut self) {
        for c in self.text.chars() {
            self.sda.set_high().unwrap();
            self.scl.set_high().unwrap();
        }
    }
}
