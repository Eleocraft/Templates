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

    let mut uart = Uart::new_blocking(p.USART1, p.PA10, p.PA9, Config::default()).unwrap();
    uart.set_baudrate(9600).unwrap();

    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);
    let mut led = Output::new(p.PA5, Level::Low, Speed::Medium);

    loop {
        button.wait_for_rising_edge().await;
        led.toggle();
        uart.blocking_write(&[led.is_set_high() as u8]).unwrap();
    }
}
