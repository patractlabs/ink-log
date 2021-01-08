// Copyright Patract Labs Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Ink! logger that prints all messages with a readable output format.
#![cfg_attr(not(feature = "std"), no_std)]

mod macros;
mod tests;

#[doc(inline)]
pub use self::macros::logger::*;
use cfg_if::cfg_if;
pub use ink_prelude::{format, vec::Vec};
pub use log::Level;
use scale::{Decode, Encode};

cfg_if! {
    if #[cfg(feature = "std")] {
        pub mod off_chain;
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct LogRecord {
    pub level: u32,
    pub target: Vec<u8>,
    pub args: Vec<u8>,
}

// func_id refer to https://github.com/patractlabs/PIPs/blob/main/PIPs/pip-100.md
// 0xfeffff00
pub const FUNC_ID_LOG: u32 = 0xfeffff00;
