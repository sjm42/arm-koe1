// main.rs

#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4::stm32f411;

// User led LD2 is on PA5

#[entry]
fn main() -> ! {
    let p = stm32f411::Peripherals::take().unwrap();
    let rcc = p.RCC;

    // Enable GPIOA clock
    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());

    let pa = &p.GPIOA;

    pa.otyper.write(|w| w.ot5().clear_bit());
    pa.moder.write(|w| w.moder5().output());
    pa.pupdr.write(|w| w.pupdr12().pull_up());

    loop {
        // let b = pa.odr.read().odr5().bit_is_clear();
        pa.odr.write(|w| w.odr5().bit(true));
        delay(200000);
        pa.odr.write(|w| w.odr5().bit(false));
        delay(800000);
    }
}

fn delay(d: u32) {
    for _i in 0..d {
        asm::nop();
    }
}

// EOF
