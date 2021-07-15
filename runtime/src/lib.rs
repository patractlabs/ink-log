//! Logger Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode};

pub use pallet_contracts::chain_extension::RetVal;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, SysConfig, UncheckedFrom,
};

use frame_support::{
    dispatch::DispatchError,
    log::{debug, error, info, trace, warn},
};

/// The chain Extension of logger
#[derive(Debug, PartialEq, Encode, Decode)]
pub struct LoggerExt {
    pub level: u32,
    pub target: Vec<u8>,
    pub args: Vec<u8>,
}

impl<C: pallet_contracts::Config> ChainExtension<C> for LoggerExt {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
        E: Ext<T = C>,
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        logger_ext!(func_id, env);

        Ok(RetVal::Converging(0))
    }
}

#[macro_export]
macro_rules! logger_ext {
    ($func_id:expr, $env:expr) => {
        use core::str;
        use $crate::LoggerExt;

        let mut env = $env.buf_in_buf_out();

        // func_id refer to https://github.com/patractlabs/PIPs/blob/main/PIPs/pip-100.md
        match $func_id {
            // 0xfeffff00-0xfeffffff reserved for pallet-contracts log and print system
            // 0xfeffff00 => ink-log
            0xfeffff00 => {
                fn dispatch_error(_err: str::Utf8Error) -> DispatchError {
                    DispatchError::Other("LogRecord parse failed")
                }
                // The memory of the vm stores buf in scale-codec
                // let input: LoggerExt = env.read_as_unbounded(1000)?;
                let len = env.in_len();
                let input: LoggerExt = env.read_as_unbounded(len)?;
                let target = str::from_utf8(input.target.as_slice()).map_err(dispatch_error)?;
                let args = str::from_utf8(input.args.as_slice()).map_err(dispatch_error)?;

                match input.level {
                    1 => {
                        error!(target: target, "❌ {}", args);
                    }
                    2 => {
                        warn!(target: target, "⚠️  {}", args);
                    }
                    3 => {
                        info!(target: target, "❤️  {}", args);
                    }
                    4 => {
                        debug!(target: target, "📋  {}", args);
                    }
                    5 => {
                        trace!(target: target, "🏷  {}", args);
                    }
                    _ => (),
                }
            }
            _ => {
                error!("call an unregistered `func_id`, func_id:{:}", $func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }
    };
}
