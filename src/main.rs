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

// https://www.st.com/en/microcontrollers-microprocessors/stm32f103.html
#[cfg(feature = "blue_pill")]
use stm32f1::stm32f103;

// https://www.st.com/en/microcontrollers-microprocessors/stm32f411re.html
#[cfg(feature = "nucleo_f411")]
use stm32f4::stm32f411;

// On Nucleo stm32f411 User led LD2 is on PA5

#[cfg(feature = "nucleo_f411")]
fn init_f411() -> stm32f411::Peripherals {
    let p = stm32f411::Peripherals::take().unwrap();
    let rcc = &p.RCC;
    let pa = &p.GPIOA;
    let pc = &p.GPIOC;

    // Enable GPIOA+C clocks
    rcc.ahb1enr.modify(|_r, w| {
        w.gpioaen().enabled();
        w.gpiocen().enabled();
        w
    });

    // Clock outputs are on PA8 = MCO1, PC9 = MCO2
    // so configure output pins as alternate function
    pa.otyper.modify(|_r, w| w.ot8().push_pull());
    pa.ospeedr.modify(|_r, w| w.ospeedr8().very_high_speed());
    pa.moder.modify(|_r, w| w.moder8().alternate());
    pc.otyper.modify(|_r, w| w.ot9().push_pull());
    pc.ospeedr.modify(|_r, w| w.ospeedr9().very_high_speed());
    pc.moder.modify(|_r, w| w.moder9().alternate());

    // Enable clock outputs 1+2
    rcc.cfgr.modify(|_r, w| {
        w.mco1pre().div1();
        w.mco1().hsi();
        w.mco2pre().div1();
        w.mco2().sysclk();
        w
    });

    // Enable push-pull output on PA5 (LED)
    pa.otyper.modify(|_r, w| w.ot5().push_pull());
    pa.ospeedr.modify(|_r, w| w.ospeedr5().low_speed());
    pa.moder.modify(|_r, w| w.moder5().output());

    p
}

#[cfg(feature = "nucleo_f411")]
fn set_led_f411(p: &stm32f411::Peripherals, state: bool) {
    let io_port = &p.GPIOA;

    // Using r/w output data register:
    // io_port.odr.modify(|_r, w| w.odr5().bit(state));

    // Using port bit set/reset register
    if state {
        // bs5() = bit set, pin 5
        io_port.bsrr.write(|w| w.bs5().set_bit());
    } else {
        // br5() = bit reset, pin 5
        io_port.bsrr.write(|w| w.br5().set_bit());
    }
}

// On blue pill stm32f103 user led is on PC13

#[cfg(feature = "blue_pill")]
fn init_f103() -> stm32f103::Peripherals {
    let p = stm32f103::Peripherals::take().unwrap();
    let rcc = &p.RCC;
    let pc = &p.GPIOC;

    // Enable GPIOC clock
    rcc.apb2enr.modify(|_r, w| w.iopcen().enabled());

    // Enable push-pull output on PC13
    pc.crh.modify(|_r, w| {
        w.mode13().output();
        w.cnf13().push_pull();
        w
    });
    p
}

#[cfg(feature = "blue_pill")]
fn set_led_f103(p: &stm32f103::Peripherals, state: bool) {
    let io_port = &p.GPIOC;

    // Using r/w output data register:
    // io_port.odr.modify(|_r, w| w.odr13().bit(!state));

    // Using port bit set/reset register
    if state {
        // true -> bit_reset (br13), because led is draining current
        io_port.bsrr.write(|w| w.br13().set_bit());
    } else {
        // false -> bit set (bs13)
        io_port.bsrr.write(|w| w.bs13().set_bit());
    }
}

#[entry]
fn main() -> ! {
    let p;
    #[cfg(feature = "nucleo_f411")]
    {
        p = init_f411();
    }
    #[cfg(feature = "blue_pill")]
    {
        p = init_f103();
    }

    loop {
        #[cfg(feature = "nucleo_f411")]
        set_led_f411(&p, true);
        #[cfg(feature = "blue_pill")]
        set_led_f103(&p, true);
        delay(200000);

        #[cfg(feature = "nucleo_f411")]
        set_led_f411(&p, false);
        #[cfg(feature = "blue_pill")]
        set_led_f103(&p, false);
        delay(800000);
    }
}

fn delay(d: u32) {
    for _i in 0..d {
        asm::nop();
    }
}

// EOF
