mod display;
mod errors;
mod led;

pub use errors::Error;
use crate::led::{RGB, WS2812RMT};
use crate::display::{Display, LcdDisplay};
use anyhow::Result;
// use ag_lcd::{Display, Blink, Cursor, LcdDisplay};
// use esp_idf_hal::gpio::IOPin;
// use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::hal::{delay::Delay, gpio::OutputPin, peripherals::Peripherals};
use std::thread;

const COLORS: [RGB; 5] = [RGB{ color: 12800 },
    RGB{ color: 16711680 },
    RGB{ color: 6579300 },
    RGB{ color: 6553700 },
    RGB{ color: 255 }];

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    log::info!("About to blink led");

    let peripherals = Peripherals::take().unwrap();
    let mut led = WS2812RMT::new(peripherals.pins.gpio8, peripherals.rmt.channel0)?;
    // led.set_pixel(RGB::new(50, 50, 0))?;
    // let mut color: u32 = 0;

    thread::spawn(move || loop {
        for c in COLORS {
            let _ = led.set_pixel(c);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    let rs = peripherals.pins.gpio10;
    let en = peripherals.pins.gpio3;
    let delay: Delay = Default::default();

    let mut lcd: LcdDisplay = LcdDisplay::new(rs.downgrade_output(), en.downgrade_output(), delay)
        .with_half_bus(
            peripherals.pins.gpio4.downgrade_output(),
            peripherals.pins.gpio5.downgrade_output(),
            peripherals.pins.gpio6.downgrade_output(),
            peripherals.pins.gpio7.downgrade_output())
        .with_display(Display::On)
        .with_blink(display::Blink::On)
        .build();
    let mut lcd = lcd.with_reliable_init(10_000);

    lcd.print("TEST!!");


    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
