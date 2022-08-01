// main.rs

#![no_std]
#![no_main]

// pick a panicking behavior
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

// https://www.st.com/en/microcontrollers-microprocessors/stm32f103.html
#[cfg(feature = "stm32f1")]
use stm32f1::stm32f103;

// https://www.st.com/en/microcontrollers-microprocessors/stm32f411re.html
#[cfg(feature = "stm32f4")]
use stm32f4::stm32f411;

#[cfg(feature = "nrf52840")]
use nrf52840_pac;

// On Nucleo stm32f411 User led LD2 is on PA5
// On Black pill user led is on PC13

#[cfg(feature = "nrf52840")]
fn init_nrf52() -> nrf52840_pac::Peripherals {
    let p = nrf52840_pac::Peripherals::take().unwrap();
    let p0 = &p.P0;
    let p1 = &p.P1;

    p
}

#[cfg(feature = "stm32f4")]
fn init_f4() -> stm32f411::Peripherals {
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
        w.mco1pre().div5();
        w.mco1().hsi();
        w.mco2pre().div5();
        w.mco2().sysclk();
        w
    });

    #[cfg(feature = "nucleo_f411")]
    {
        // Enable push-pull output on PA5 (LED) on Nucleo F411
        pa.otyper.modify(|_r, w| w.ot5().push_pull());
        pa.ospeedr.modify(|_r, w| w.ospeedr5().low_speed());
        pa.moder.modify(|_r, w| w.moder5().output());
    }
    #[cfg(feature = "black_pill")]
    {
        // Enable push-pull output on PC13 (LED) on Black pill
        pc.otyper.modify(|_r, w| w.ot13().push_pull());
        pc.ospeedr.modify(|_r, w| w.ospeedr13().low_speed());
        pc.moder.modify(|_r, w| w.moder13().output());
    }
    p
}

#[cfg(feature = "stm32f4")]
fn set_led_f4(p: &stm32f411::Peripherals, state: bool) {
    #[cfg(feature = "nucleo_f411")]
    let io_port = &p.GPIOA;
    #[cfg(feature = "black_pill")]
    let io_port = &p.GPIOC;

    // Using port bit set/reset register
    if state {
        // bs5() = bit set, pin 5
        #[cfg(feature = "nucleo_f411")]
        io_port.bsrr.write(|w| w.bs5().set_bit());

        // br13() = bit reset, pin 13
        #[cfg(feature = "black_pill")]
        io_port.bsrr.write(|w| w.br13().set_bit());
    } else {
        // br5() = bit reset, pin 5
        #[cfg(feature = "nucleo_f411")]
        io_port.bsrr.write(|w| w.br5().set_bit());

        // bs13() = bit set, pin 13
        #[cfg(feature = "black_pill")]
        io_port.bsrr.write(|w| w.bs13().set_bit());
    }
}

// On blue pill stm32f103 user led is on PC13

#[cfg(feature = "stm32f1")]
fn init_f1() -> stm32f103::Peripherals {
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

#[cfg(feature = "stm32f1")]
fn set_led_f1(p: &stm32f103::Peripherals, state: bool) {
    let io_port = &p.GPIOC;

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
    #[cfg(feature = "stm32f4")]
    {
        p = init_f4();
    }
    #[cfg(feature = "stm32f1")]
    {
        p = init_f1();
    }

    loop {
        #[cfg(feature = "stm32f4")]
        set_led_f4(&p, true);
        #[cfg(feature = "stm32f1")]
        set_led_f1(&p, true);
        delay(200000);

        #[cfg(feature = "stm32f4")]
        set_led_f4(&p, false);
        #[cfg(feature = "stm32f1")]
        set_led_f1(&p, false);
        delay(800000);
    }
}

fn delay(d: u32) {
    for _i in 0..d {
        asm::nop();
    }
}

// EOF
