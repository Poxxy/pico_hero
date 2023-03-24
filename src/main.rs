#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use embedded_time::rate::*;
use panic_probe as _;
use rp2040_hal as hal;

use core::fmt::Write;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::PushPull,
    pac,
    watchdog::Watchdog,
    Sio,
};
use ssd1306::{
    prelude::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306,
};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    // Boilerplate
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Used for blinky, always running.
    let mut led_pin = pins.gpio25.into_push_pull_output();

    // hal refers to rp_pico::hal
    let sda_pin = pins.gpio16.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio17.into_mode::<hal::gpio::FunctionI2C>();

    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        40_u32.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();
    display.clear().unwrap();

    write!(display, "{}", "Hello World!").unwrap();

    loop {
        blinky(&mut led_pin, &mut delay);
    }
}

/// Blink every second to help count timer.
fn blinky(
    led_pin: &mut hal::gpio::Pin<hal::gpio::bank0::Gpio25, hal::gpio::Output<PushPull>>,
    delay: &mut cortex_m::delay::Delay,
) {
    led_pin.set_high().unwrap();
    delay.delay_ms(500);
    led_pin.set_low().unwrap();
    delay.delay_ms(500);
}
