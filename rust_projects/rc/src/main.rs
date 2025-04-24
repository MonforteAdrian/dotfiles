#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
    gpiote::{OutputChannel, OutputChannelPolarity},
    peripherals::PWM0,
    pwm::{
        Config, Prescaler, SequenceConfig, SequenceLoad, SequencePwm, SimplePwm,
        SingleSequenceMode, SingleSequencer,
    },
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Timer, WithTimeout};
use panic_probe as _;

const T1H: u16 = 0x8000 | 13; // Duty = 13/20 ticks (0.8us/1.25us) for a 1
const T0H: u16 = 0x8000 | 7; // Duty 7/20 ticks (0.4us/1.25us) for a 0
const RES: u16 = 0x8000;

#[derive(Clone, Copy)]
enum Button {
    A,
    B,
}

static CHANNEL: Channel<ThreadModeRawMutex, Button, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");
    let p = embassy_nrf::init(Default::default());

    spawner.must_spawn(signal_task());

    // Servo
    let mut pwm = SimplePwm::new_1ch(p.PWM1, p.P0_01);
    pwm.set_prescaler(Prescaler::Div128);
    pwm.set_max_duty(2500);
    pwm.set_duty(0, 2500 - 50);

    // Big leds
    let led_left = OutputChannel::new(
        p.GPIOTE_CH0,
        Output::new(p.P0_09, Level::High, OutputDrive::Standard),
        OutputChannelPolarity::Toggle,
    );
    led_left.out();

    spawner.must_spawn(bottom_leds(
        p.PWM0,
        p.P0_11.degrade(),
        p.P0_28.degrade(),
        p.P0_30.degrade(),
        p.P1_05.degrade(),
    ));
    let button_a = button(p.P0_14.degrade(), "A", Button::A);
    let button_b = button(p.P0_23.degrade(), "B", Button::B);
    join(button_a, button_b).await;
}

#[embassy_executor::task]
async fn bottom_leds(
    p_pwm: PWM0,
    p_front_right: AnyPin,
    p_back_right: AnyPin,
    p_back_left: AnyPin,
    p_front_left: AnyPin,
) {
    let mut config = Config::default();
    config.sequence_load = SequenceLoad::Common;
    config.prescaler = Prescaler::Div1;
    config.max_duty = 20; // 1.25us (1s / 16Mhz * 20)
    let mut pwm = SequencePwm::new_4ch(
        p_pwm,
        p_front_right,
        p_back_right,
        p_back_left,
        p_front_left,
        config,
    )
    .unwrap();

    // Declare the bits of 24 bits in a buffer we'll be
    // mutating later.
    let mut seq_words = [
        T0H, T0H, T0H, T0H, T0H, T0H, T0H, T0H, // G
        T0H, T0H, T0H, T0H, T0H, T0H, T0H, T0H, // R
        T1H, T1H, T1H, T1H, T1H, T1H, T1H, T1H, // B
        RES,
    ];

    let mut seq_config = SequenceConfig::default();
    seq_config.end_delay = 799; // 50us (20 ticks * 40) - 1 tick because we've already got one RES;

    let mut color_bit = 16;
    let mut bit_value = T0H;

    loop {
        let sequences = SingleSequencer::new(&mut pwm, &seq_words, seq_config.clone());
        sequences.start(SingleSequenceMode::Times(1)).unwrap();

        Timer::after_millis(50).await;

        if bit_value == T0H {
            if color_bit == 20 {
                bit_value = T1H;
            } else {
                color_bit += 1;
            }
        } else {
            if color_bit == 16 {
                bit_value = T0H;
            } else {
                color_bit -= 1;
            }
        }

        drop(sequences);

        seq_words[color_bit] = bit_value;
    }
}

async fn button(pin: AnyPin, id: &str, b: Button) {
    let mut button = Input::new(pin, Pull::None);
    loop {
        button.wait_for_low().await;
        info!("Button {} pressed (fut)", id);
        CHANNEL.send(b).await;
        Timer::after_millis(200).await;
        button.wait_for_high().await;
    }
}

#[embassy_executor::task]
async fn signal_task() {
    loop {
        match CHANNEL.receive().await {
            Button::A => info!("Signal recieved from button A"),
            Button::B => info!("Signal recieved from button B"),
        };
    }
}

