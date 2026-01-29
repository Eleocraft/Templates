#![no_std]
#![no_main]

// Imports
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use stm32f4xx_hal::{
    gpio::{self, Edge, Input},
    pac::{self, interrupt},
    prelude::*,
};

// Create an alias for pin PC13
type ButtonPin = gpio::PC13<Input>;

// Global Variable Definitions
// Global variables are wrapped in safe abstractions.
// Peripherals are wrapped in a different manner than regular global mutable data.
// In the case of peripherals we must be sure only one refrence exists at a time.
// Refer to Chapter 6 of the Embedded Rust Book for more detail.

// Create a Global Variable for the GPIO Peripheral that I'm going to pass around.
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let mut dp = pac::Peripherals::take().unwrap();

    // Configure and obtain handle for delay abstraction
    // 1) Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // 2) Configure the system clocks
    let _clocks = rcc.cfgr.sysclk(16.MHz()).freeze();

    // Configure the button pin as input and obtain handle
    // On the Nucleo there is a button connected to pin PC13
    // 1) Promote the GPIOC PAC struct
    let gpioc = dp.GPIOC.split();
    // 2) Configure Pin and Obtain Handle
    let mut button = gpioc.pc13;

    // Configure Button Pin for Interrupts
    // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
    let mut syscfg = dp.SYSCFG.constrain();
    // 2) Make button an interrupt source
    button.make_interrupt_source(&mut syscfg);
    // 3) Make button an interrupt source
    button.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // 4) Enable gpio interrupt for button
    button.enable_interrupt(&mut dp.EXTI);

    // Enable the external interrupt in the NVIC by passing the button interrupt number
    unsafe {
        cortex_m::peripheral::NVIC::unmask(button.interrupt());
    }

    // Now that button is configured, move button into global context
    cortex_m::interrupt::free(|cs| {
        G_BUTTON.borrow(cs).replace(Some(button));
    });

    // Application Loop
    loop {}
}

#[interrupt]
fn EXTI15_10() {
    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        info!("interrupt");
        // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
        let mut button = G_BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}
