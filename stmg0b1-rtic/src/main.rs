#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

// define monotic named "Mono" with tickrate 1khz
rtic_monotonics::systick_monotonic!(Mono, 1000);

#[rtic::app(
    // path to a svd2rust PAC
    device = stm32g0::stm32g0b1,
    // Replace with free interrupt vectors if software tasks are used
    // You can find the names of the interrupt vectors in the stm32g0b1::interrupt enum.
    dispatchers = [ TIM3_TIM4 ]
)]
mod app {
    use defmt::info;
    use super::Mono;
    use rtic_monotonics::rtic_time::Monotonic;
    use rtic_monotonics::fugit::Duration;

    // Shared resources go here
    #[shared]
    struct Shared {
        shared_test: i64,
    }

    // Local resources go here
    #[local]
    struct Local {
        local_test: i64,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        info!("init");

        // setup monotics (on g0b1 default clockspeed 16Mhz)
        Mono::start(cx.core.SYST, 16_000_000);

        task1::spawn().ok();
        task2::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
                shared_test: 0
            },
            Local {
                // Initialization of local resources go here
                local_test: 0
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        info!("idle");

        loop {
            continue;
        }
    }

    // Task 1
    #[task(local = [local_test], shared = [shared_test], priority = 1)]
    async fn task1(mut cx: task1::Context) {
        loop {
            info!("Task 1! local: {}", cx.local.local_test);
            *cx.local.local_test += 1;
            cx.shared.shared_test.lock(|v| *v += 5);

            // Wait for 5 seconds
            Mono::delay(Duration::<u32, _, _>::millis(5000)).await;
        }
    }

    // Task 2
    #[task(shared = [shared_test], priority = 1)]
    async fn task2(mut cx: task2::Context) {
        loop {
            info!("Task 2! shared: {}", cx.shared.shared_test.lock(|v| *v));
            cx.shared.shared_test.lock(|v| *v += 1);
            
            // Wait for 1 second
            Mono::delay(Duration::<u32, _, _>::millis(1000)).await;
        }
    }
}
