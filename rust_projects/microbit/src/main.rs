#![no_std]
#![no_main]

use microbit as _;
use microbit::{
    board::Board,
    hal::{
        prelude::{OutputPin, PinState, _embedded_hal_blocking_delay_DelayMs},
        timer::Timer,
    },
};

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cortex_m_rt::entry]
fn start_here() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let blink_delay_ms = 300_u32;
    board.display_pins.col3.set_state(PinState::Low).unwrap();
    board.display_pins.row3.set_state(PinState::High).unwrap();
    loop {
        //board.display_pins.row3.set_state(PinState::High).unwrap();
        //timer.delay_ms(blink_delay_ms);
        //board.display_pins.row3.set_state(PinState::Low).unwrap();
        //timer.delay_ms(blink_delay_ms);
    }
}
