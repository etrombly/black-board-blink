#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
extern crate stm32f4;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    let mut peripherals = stm32f407::Peripherals::take().unwrap();
    
    //let mut core = stm32f407::CorePeripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    //let rcc = &peripherals.RCC;
    //let flash = &peripherals.FLASH;

    // TODO: test performance and see what this needs to actually be set to
    //let clocks = rcc
    //    .cfgr
    //    .sysclk(64.mhz())
    //    .pclk1(32.mhz())
    //    .freeze(&mut flash.acr);


    // configure the system timer
    // TODO: test performance and see what this needs to actually be set to
    //core.SYST.set_clock_source(SystClkSource::Core);
    //core.SYST.set_reload(100_000);
    //core.SYST.enable_interrupt();
    //core.SYST.enable_counter();


    gpioa.odr.modify(|_, w| w.odr6().set_bit());

    loop {
        
    }
}
