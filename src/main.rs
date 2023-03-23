#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_time::fixed_point::FixedPoint;
mod oled;
use oled::OLED;
use panic_probe as _;
use rp2040_hal as hal;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::{AnyPin, PullUp, PushPull},
    pac,
    watchdog::Watchdog,
    Sio,
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

    // Used for OLED.
    let mut sda = pins.gpio0.into_push_pull_output();
    let mut scl = pins.gpio1.into_push_pull_output();

    let mut oled = OLED::new(sda, scl);

    loop {
        blinky(&mut led_pin, &mut delay);
        oled.display();
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
