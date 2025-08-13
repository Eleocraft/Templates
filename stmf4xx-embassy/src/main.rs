#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput, gpio::{Level, Output, Pull, Speed}, usart::{Config, Uart}
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Launching");

    let mut uart = Uart::new_blocking(p.USART1, p.PB3, p.PB6, Config::default()).unwrap();
    uart.set_baudrate(9600).unwrap();

    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);
    let mut green = Output::new(p.PB0, Level::Low, Speed::Medium);
    let mut blue = Output::new(p.PB7, Level::Low, Speed::Medium);
    let mut red = Output::new(p.PB14, Level::Low, Speed::Medium);
    let mut counter = 0;

    loop {
        button.wait_for_rising_edge().await;
        counter = (counter + 1) % (1 << 3);
        green.set_level((counter & 1 == 1).into());
        blue.set_level(((counter >> 1) & 1 == 1).into());
        red.set_level(((counter >> 2) & 1 == 1).into());
        uart.blocking_write(&[counter]).unwrap();
    }
}
