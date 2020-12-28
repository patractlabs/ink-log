//! Logger Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::DispatchError;
use sp_std::{str, vec::Vec};

use frame_support::debug::{error, native};

/// The chain Extension of logger
pub struct LoggerExt;

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct LogRecord {
    pub level: u32,
    pub target: Vec<u8>,
    pub args: Vec<u8>,
}

impl ChainExtension for LoggerExt {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
        where
            <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        let mut env = env.buf_in_buf_out();

        // func_id refer to https://github.com/patractlabs/PIPs/blob/main/PIPs/pip-100.md
        match func_id {
            // 0xfeffff00-0xfeffffff reserved for pallet-contracts log and print system
            // 0xfeffff00 => ink-log
            0xfeffff00 => {
                // The memory of the vm stores buf in scale-codec
                let input: LogRecord = env.read_as()?;
                let target = str::from_utf8(input.target.as_slice()).unwrap();
                let args = str::from_utf8(input.args.as_slice()).unwrap();

                match input.level {
                    1 => {
                        native::error!(target: target, "❌ {}", args);
                    }
                    2 => {
                        native::warn!(target: target, "⚠️  {}", args);
                    }
                    3 => {
                        native::info!(target: target, "❤️  {}", args);
                    }
                    4 => {
                        native::debug!(target: target, "📋  {}", args);
                    }
                    5 => {
                        native::trace!(target: target, "🏷  {}", args);
                    }
                    _ => (),
                }
            }
            _ => {
                error!("call an unregistered `func_id`, func_id:{:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }

        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
