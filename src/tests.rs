#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::log;
        // ink_env::test::register_chain_extension::<ink_env::DefaultEnvironment, log::LogRecord, log::LogRecord>()

        log::InkLogger::new().init();
        log::error!(target: "test", "hello world");

        assert_eq!(2 + 2, 4);
    }
}
