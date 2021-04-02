#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nb::block;

#[allow(unused_imports)]
use panic_semihosting;

use adafruit_nrf52840_express::{
    hal::{
        prelude::*,
        timer::{self, Timer},
    },
    Board,
};

#[entry]
fn main() -> ! {
    let mut feather = Board::take().unwrap();

    let mut timer = Timer::new(feather.TIMER0);

    // Alternately flash the red and blue leds
    loop {
        feather.leds.d3.enable();
        delay(&mut timer, 250_000); // 250ms
        feather.leds.d3.disable();
        delay(&mut timer, 1_000_000); // 1s
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
    where
        T: timer::Instance,
{
    timer.start(cycles);
    let _ = block!(timer.wait());
}