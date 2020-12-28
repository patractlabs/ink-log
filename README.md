# ink-log 
[Ink!](https://github.com/paritytech/ink) logger that prints all messages with a readable output format.

[WIP]

## Example

1. Use like [rust log](https://github.com/rust-lang/log) macro
```
ink_log::info!(target: "flipper-contract", "latest value is: {}", self.value);

ink_log::debug!("latest value is: {}", self.value);
```

2. Use custom `clog` macro
```
ink_log::clog!(info, target: "flipper-contract", "latest value is: {}", self.value);

ink_log::clog!(debug, "latest value is: {}", self.value);
```
