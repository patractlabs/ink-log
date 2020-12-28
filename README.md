# Ink-log 
[Ink!](https://github.com/paritytech/ink) logger that prints all messages with a readable output format.

[WIP]

## Feature `ink-log-chain-extensions`
Only when the feature is available, the ink-log functions is effective.
```
ink_log = { version = "0.1", git = "https://github.com/patractlabs/ink-log", default-features = false, features = ["ink-log-chain-extensions"] }
```

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

output:
```
2020-12-28 17:44:30.274   INFO tokio-runtime-worker flipper-contract:/paritytech/ink/examples/flipper/lib.rs:42:❤️  latest value is: false
```
