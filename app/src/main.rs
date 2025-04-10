#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::peripherals;

use rtt_target as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.rcc = embassy_stm32::rcc::WPAN_DEFAULT;
    let p = embassy_stm32::init(config);

    let mut counter = 0;
    rtt_target::rtt_init_defmt!();
   
    loop {
        defmt::info!("Hello World! {}", shared::add(1, 2));
        
        counter += 1;

        embassy_time::Timer::after_millis(500).await;
    }
}
