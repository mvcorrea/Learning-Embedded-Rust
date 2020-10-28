#![no_std]
#![no_main]

use cortex_m_rt::entry;
mod stm32_blink02;


#[entry]
fn main() -> ! {
	stm32_blink02::run();
}

// vim:ft=rust:ts=4:sw=4:nu:ai:
