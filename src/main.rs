#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_time::fixed_point::FixedPoint;
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

    // Used to trigger alarm.
    let mut pin16 = pins.gpio16.into_push_pull_output();

    // Used for adding 5 seconds
    let pin14 = pins.gpio14.into_pull_up_input();

    // Used for adding 1 minute
    let pin15 = pins.gpio15.into_pull_up_input();

    let mut countdown_in_ms = 10_000;

    loop {
        if countdown_in_ms < 0 {
            alarm(&mut pin16, &mut delay);
            countdown_in_ms = 10_000;
        }

        blinky(&mut led_pin, &mut delay);

        countdown_in_ms -= 1_000;

        countdown_in_ms += add_time(&pin14, &pin15);
    }
}

fn blinky(
    led_pin: &mut hal::gpio::Pin<hal::gpio::bank0::Gpio25, hal::gpio::Output<PushPull>>,
    delay: &mut cortex_m::delay::Delay,
) {
    led_pin.set_high().unwrap();
    delay.delay_ms(500);
    led_pin.set_low().unwrap();
    delay.delay_ms(500);
}

fn add_time(
    pin14: &hal::gpio::Pin<hal::gpio::bank0::Gpio14, hal::gpio::Input<PullUp>>,
    pin15: &hal::gpio::Pin<hal::gpio::bank0::Gpio15, hal::gpio::Input<PullUp>>,
) -> i32 {
    match (pin14.is_high().unwrap(), pin15.is_high().unwrap()) {
        (true, true) => return 65,
        (true, false) => return 5,
        (false, true) => return 60,
        (false, false) => return 0,
    }
}

fn alarm(
    pin16: &mut hal::gpio::Pin<hal::gpio::bank0::Gpio16, hal::gpio::Output<PushPull>>,
    delay: &mut cortex_m::delay::Delay,
) {
    let mut i = 0;
    while i < 100 {
        pin16.set_high().unwrap();
        delay.delay_ms(20);
        pin16.set_low().unwrap();
        delay.delay_ms(20);
        i += 1;
    }
}
