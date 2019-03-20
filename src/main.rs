#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f4::stm32f407;
use stm32f4xx_hal::{
    delay::Delay,
    prelude::*,
};
use max7219::{MAX7219, Command, DecodeMode};

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
    let gpioe = peripherals.GPIOE.split();
    let mut led1 = gpioa.pa6.into_push_pull_output();
    let mut led2 = gpioa.pa7.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, clocks);

    let max_data = gpioe.pe2.into_push_pull_output();
    let max_clk = gpioe.pe3.into_push_pull_output();
    let max_cs = gpioe.pe4.into_push_pull_output();

    led1.set_high();
    led2.set_high();

    let number_of_devices: u8 = 1;
    let mut max7219 = MAX7219::new(number_of_devices, max_data, max_cs, max_clk);
    max7219.power_on();
    max7219.set_intensity(0, 1);


    loop {
        //led1.set_low();
        //led2.set_high();
        max7219.write_raw(0, 0x01, 0b00011000);
        max7219.write_raw(0, 0x02, 0b00111100);
        max7219.write_raw(0, 0x03, 0b01100110);
        max7219.write_raw(0, 0x04, 0b11000011);
        max7219.write_raw(0, 0x05, 0b11000011);
        max7219.write_raw(0, 0x06, 0b01100110);
        max7219.write_raw(0, 0x07, 0b00111100);
        max7219.write_raw(0, 0x08, 0b00011000);
        delay.delay_ms(254_u8);
        //led1.set_high();
        //led2.set_low();
        max7219.write_raw(0, 0x01, 0b00011000);
        max7219.write_raw(0, 0x02, 0b00011000);
        max7219.write_raw(0, 0x03, 0b00011000);
        max7219.write_raw(0, 0x04, 0b11111111);
        max7219.write_raw(0, 0x05, 0b11111111);
        max7219.write_raw(0, 0x06, 0b00011000);
        max7219.write_raw(0, 0x07, 0b00011000);
        max7219.write_raw(0, 0x08, 0b00011000);
        delay.delay_ms(254_u8);
    }
}
