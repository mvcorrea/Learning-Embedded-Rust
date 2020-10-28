extern crate stm32f1;
extern crate panic_halt;
extern crate cortex_m_rt;

use stm32f1::stm32f103;

pub fn run() -> ! {
    // get handles to the hardware
    let peripherals = stm32f103::Peripherals::take().unwrap();
    let gpioc = &peripherals.GPIOC;
    let rcc = &peripherals.RCC;

    // enable the GPIO clock for IO port C
    rcc.apb2enr.write(|w| w.iopcen().set_bit());
    gpioc.crh.write(|w| unsafe{
        w.mode13().bits(0b11);
        w.cnf13().bits(0b00)
    });

    loop {
        gpioc.bsrr.write(|w| w.bs13().set_bit());
        cortex_m::asm::delay(2000000);
        gpioc.brr.write(|w| w.br13().set_bit());
        cortex_m::asm::delay(2000000);
    }
	
}

// vim:ft=rust:ts=4:sw=4:nu:ai:
