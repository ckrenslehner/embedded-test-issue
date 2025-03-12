#![no_std]
#![no_main]


fn setup_log() {
    #[cfg(feature = "log")]
    rtt_target::rtt_init_log!();
    #[cfg(feature = "defmt")]
    rtt_target::rtt_init_defmt!();
}

#[cfg(test)]
/// Need to explicitly state the executer
#[embedded_test::tests(executor = embassy_executor::Executor::new(), setup=crate::setup_log())]
mod tests {
    use rtt_target as _;
    use embassy_stm32::Peripherals;

    // An optional init function which is called before every test
    // Asyncness is optional, so is the return value
    #[init]
    async fn init() -> Peripherals {
        let config = embassy_stm32::Config::default();
        let p = embassy_stm32::init(config);
        p
    }

    // Tests can be async (needs feature `embassy`)
    // Tests can take the state returned by the init function (optional)
    #[test]
    async fn takes_state(_state: Peripherals) {
        assert!(true)
    }

    // Tests can be ignored with the #[ignore] attribute
    #[test]
    #[ignore]
    fn it_works_ignored() {
        assert!(false)
    }

    // Tests can fail with a custom error message by returning a Result
    #[test]
    fn it_fails_with_err() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    // Tests can be annotated with #[should_panic] if they are expected to panic
    #[test]
    #[should_panic]
    fn it_passes() {
        assert!(false)
    }

    // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    #[test]
    #[timeout(10)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
}