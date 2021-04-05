#![no_main]
#![no_std]

use cortex_m_rt::entry;

#[allow(unused_imports)]
use panic_semihosting;

use adafruit_nrf52840_express::{
    hal::{
        prelude::*,
        Delay,
    },
    Board,
};

#[entry]
fn main() -> ! {
    let mut feather = Board::take().unwrap();

    let mut delay = Delay::new(feather.SYST);

    // Alternately flash the red and blue leds
    loop {
        match feather.leds.d3.is_on() {
            true => {
                feather.leds.d3.disable();
                feather.leds.conn.enable();
            }
            false => {
                feather.leds.d3.enable();
                feather.leds.conn.disable();
            }
        }

        delay.delay_ms(1_000u32);
    }
}