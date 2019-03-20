#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
extern crate stm32f4;
use stm32f4::stm32f407;
use stm32f4xx_hal::{
    delay::Delay,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = stm32f407::Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc
            .cfgr
            .sysclk(64.mhz())
            .pclk1(32.mhz())
            .freeze();
    
    let core = stm32f407::CorePeripherals::take().unwrap();
    let gpioa = peripherals.GPIOA.split();
    let mut led1 = gpioa.pa6.into_push_pull_output();
    let mut led2 = gpioa.pa7.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, clocks);

    loop {
        led1.set_low();
        led2.set_high();
        delay.delay_ms(254_u8);
        led1.set_high();
        led2.set_low();
        delay.delay_ms(254_u8);
    }
}
