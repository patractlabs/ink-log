#[cfg(test)]
mod tests {
    #[test]
    fn test_ink_log() {
        // ink_env::test::register_chain_extension::<ink_env::DefaultEnvironment, log::LogRecord, log::LogRecord>()

        crate::off_chain::log(log::Level::Info as u32, "test", "pretty ink log");
    }
}
