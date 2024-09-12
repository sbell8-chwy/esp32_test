use anyhow::Result;
use core::time::Duration;
use esp_idf_svc::hal::{
    gpio::OutputPin,
    peripheral::Peripheral,
    rmt::{config::TransmitConfig, FixedLengthSignal, PinState, Pulse, RmtChannel, TxRmtDriver},
};

#[derive(Debug, Clone)]
pub struct RGB {
    pub color: u32
}

impl RGB {
    pub fn new(red: u32, green: u32, blue: u32) -> RGB {
        Self { color: (green << 16) | (red << 8) | blue }
    }
}


// This LED implementation is a mix of code from here:
// https://github.com/esp-rs/std-training/blob/main/common/lib/rgb-led/src/lib.rs
// and the protocol info compiled in this blog post.
// https://cpldcpu.wordpress.com/2014/01/14/light_ws2812-library-v2-0-part-i-understanding-the-ws2812/
pub struct WS2812RMT<'a> {
    tx_rtm_driver: TxRmtDriver<'a>,
}

impl<'d> WS2812RMT<'d> {
    // Rust ESP Board gpio2,  ESP32-C3-DevKitC-02 gpio8
    pub fn new(
        led: impl Peripheral<P = impl OutputPin> + 'd,
        channel: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> Result<Self> {
        let config = TransmitConfig::new().clock_divider(2);
        let tx = TxRmtDriver::new(channel, led, &config)?;
        Ok(Self { tx_rtm_driver: tx })
    }

    pub fn set_pixel(&mut self, rgb: RGB) -> Result<()> {
        let ticks_hz = self.tx_rtm_driver.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(950))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600))?;
        let mut signal = FixedLengthSignal::<24>::new();
        for i in (0..24).rev() {
            let p = 2_u32.pow(i);
            let bit = p & rgb.color != 0;
            let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
            signal.set(23 - i as usize, &(high_pulse, low_pulse))?;
        }
        self.tx_rtm_driver.start_blocking(&signal)?;

        Ok(())
    }
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}
