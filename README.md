# Ink-log 
Ink-log provides pretty log printing for [Ink!](https://github.com/paritytech/ink) smart contract，it's implmented by `ChainExtension`.

[WIP]

## Usage
## 1. Contract Pallet In Substrate Runtime

### dependencies
Add this to your `Cargo.toml`:
```
runtime-log = { version = "0.1", git = "https://github.com/patractlabs/ink-log", default-features = false }

[features]
std = [
    "runtime-log/std",
]
```

### example
1. If you already have one `CustomExt`, use `runtime_log::logger_ext!` to add to your `CustomExt`.
```rust
pub struct CustomExt;

impl ChainExtension for CustomExt {
	fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		// TODO add other libs
        runtime_log::logger_ext!(func_id, env);

		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
```
2. If you don't have any `CustomExt`, use `runtime_log::LoggerExt` to set `ChainExtension`.
```rust
impl pallet_contracts::Config for Runtime {
    // ...... 
    type ChainExtension = runtime_log::LoggerExt;
}
```

## 2. Ink! contract

### dependencies
Add this to your contratc `Cargo.toml`:
```
ink_log = { version = "0.1", git = "https://github.com/patractlabs/ink-log", default-features = false, features = ["ink-log-chain-extensions"] }

[features]
std = [
    "ink_log/std",
]
```

Notes: must add feature `ink-log-chain-extensions` feature, only when the feature is available, the ink-log functions is effective.

### example

Use like [rust log](https://github.com/rust-lang/log) macro
```rust
ink_log::info!(target: "flipper-contract", "latest value is: {}", self.value);

ink_log::debug!("latest value is: {}", self.value);
```

Output:
```
2020-12-28 17:44:30.274   INFO tokio-runtime-worker flipper-contract:/paritytech/ink/examples/flipper/lib.rs:42:❤️  latest value is: false
```
