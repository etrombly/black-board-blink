#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use rtfm::{
    app,
    export::wfi,
};
use stm32f4::stm32f407::Interrupt;
use stm32f4xx_hal::{
    gpio::{
        gpioa::{PA6, PA7},
        gpioe::{PE3, PE4, PE11, PE12, PE13},
        Output, PushPull, PullUp, Input,
    },
    prelude::*,
};
use max7219::MAX7219;

type LED1 =  PA6<Output<PushPull>>;
type LED2 =  PA7<Output<PushPull>>;
type BUTTON1 =  PE4<Input<PullUp>>;
type BUTTON2 =  PE3<Input<PullUp>>;
type MAX = MAX7219<PE11<Output<PushPull>>,PE13<Output<PushPull>>,PE12<Output<PushPull>>>;

#[app(device = stm32f4::stm32f407)]
const APP: () = {
    static mut LED1: LED1 = ();
    static mut LED2: LED2 = ();
    static mut BUTTON1: BUTTON1 = ();
    static mut BUTTON2: BUTTON2 = ();
    static mut MAX: MAX = ();
    static mut FRAME: bool = true;

    #[init]
    fn init() {
        let device: stm32f4::stm32f407::Peripherals = device;
        let rcc = device.RCC.constrain();
        let clocks = rcc
                .cfgr
                .sysclk(64.mhz())
                .pclk1(32.mhz())
                .freeze();
        
        let mut core: rtfm::Peripherals = core;
        core.SYST.set_clock_source(SystClkSource::Core);
        core.SYST.set_reload(100_000_000);
        core.SYST.enable_interrupt();
        core.SYST.enable_counter();

        let gpioa = device.GPIOA.split();
        let gpioe = device.GPIOE.split();
        let mut led1 = gpioa.pa6.into_push_pull_output();
        let mut led2 = gpioa.pa7.into_push_pull_output();

        let max_data = gpioe.pe11.into_push_pull_output();
        let max_clk = gpioe.pe12.into_push_pull_output();
        let max_cs = gpioe.pe13.into_push_pull_output();

        led1.set_high();
        led2.set_high();

        let number_of_devices: u8 = 1;
        let mut max7219 = MAX7219::new(number_of_devices, max_data, max_cs, max_clk);
        max7219.power_on();
        max7219.set_intensity(0, 1);
        max7219.clear_display(0);

        LED1 = led1;
        LED2 = led2;
        BUTTON1 = gpioe.pe4.into_pull_up_input();
        BUTTON2 = gpioe.pe3.into_pull_up_input();
        MAX = max7219;
    }

    #[idle(resources = [])]
    fn idle() -> ! {
        loop {
            wfi();
            rtfm::pend(Interrupt::EXTI2);
        }
    }

    #[exception(resources = [MAX, FRAME, LED1, LED2, BUTTON1, BUTTON2])]
    fn SysTick() {
        if *resources.FRAME {
            resources.MAX.write_raw(0, 0x01, 0b00011000);
            resources.MAX.write_raw(0, 0x02, 0b00111100);
            resources.MAX.write_raw(0, 0x03, 0b01100110);
            resources.MAX.write_raw(0, 0x04, 0b11000011);
            resources.MAX.write_raw(0, 0x05, 0b11000011);
            resources.MAX.write_raw(0, 0x06, 0b01100110);
            resources.MAX.write_raw(0, 0x07, 0b00111100);
            resources.MAX.write_raw(0, 0x08, 0b00011000);
            *resources.FRAME = false;
        } else {
            resources.MAX.write_raw(0, 0x01, 0b00011000);
            resources.MAX.write_raw(0, 0x02, 0b00011000);
            resources.MAX.write_raw(0, 0x03, 0b00011000);
            resources.MAX.write_raw(0, 0x04, 0b11111111);
            resources.MAX.write_raw(0, 0x05, 0b11111111);
            resources.MAX.write_raw(0, 0x06, 0b00011000);
            resources.MAX.write_raw(0, 0x07, 0b00011000);
            resources.MAX.write_raw(0, 0x08, 0b00011000);
            *resources.FRAME = true;
        }
        if resources.BUTTON1.is_low() {
            resources.LED1.set_low();
        } else {
            resources.LED1.set_high();
        }
        if resources.BUTTON2.is_low() {
            resources.LED2.set_low();
        } else {
            resources.LED2.set_high();
        }
    }

    //#[interrupt(resources=[LED1])]
    //fn EXTI2() {
        //resources.LED1.set_low();
    //}
};
