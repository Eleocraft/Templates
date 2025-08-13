#![no_std]
#![no_main]

use defmt::*;
use cortex_m_rt::entry;
use {defmt_rtt as _, panic_probe as _};
use stm32f4xx_hal::{pac, prelude::*, time::Bps, uart::{self, Config}};

#[entry]
fn main() -> ! {
    info!("Launching");

    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(16.MHz())
        .freeze();

    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let button = gpioc.pc13.into_pull_down_input();
    let mut green = gpiob.pb0.into_push_pull_output();
    let mut blue = gpiob.pb7.into_push_pull_output();
    let mut red = gpiob.pb14.into_push_pull_output();

    let mut counter = 0;
    let mut delay = dp.TIM5.delay_us(&clocks);

    let uart_conf = Config::default().baudrate(Bps(9600));
    let tx_pin = gpiob.pb6.into_open_drain_output();
    let rx_pin = gpiob.pb3.into_open_drain_output();
    let mut uart = dp.USART1.serial::<u8>((tx_pin, rx_pin), uart_conf, &clocks).unwrap();

    loop {
        if !button.is_high() {
            delay.delay_ms(100);
            continue;
        }
        counter = (counter + 1) % (1 << 3);
        green.set_state((counter & 1 == 1).into());
        blue.set_state(((counter >> 1) & 1 == 1).into());
        red.set_state(((counter >> 2) & 1 == 1).into());
        uart.write(counter).unwrap();
        delay.delay_ms(250);
    }
}
