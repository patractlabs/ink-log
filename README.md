# ink-log 
[Ink!](https://github.com/paritytech/ink) logger that prints all messages with a readable output format.

[WIP]

## Example

1. Use custom `log` macro
```
ink_log::log!(info, target: "flipper-contract", "latest value is: {}", self.value);

ink_log::log!(debug, target: "flipper-contract", "latest value is: {}", self.value);
```

2. Use [rust log](https://github.com/rust-lang/log) macro
```
ink_log::InkLogger::new().init();
ink_log::info!(target: "flipper-contract", "latest value is: {}", self.value);

ink_log::debug!(target: "flipper-contract", "latest value is: {}", self.value);
```
