// SONIC: Standard library for formally-verifiable distributed contracts
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2019-2024 LNP/BP Standards Association, Switzerland.
// Copyright (C) 2024-2025 Laboratories for Ubiquitous Deterministic Computing (UBIDECO),
//                         Institute for Distributed and Cognitive Systems (InDCS), Switzerland.
// Copyright (C) 2019-2025 Dr Maxim Orlovsky.
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// TODO: Activate once StrictEncoding will be no_std
// #![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate core;
extern crate alloc;

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_types;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub use sonicapi::*;
#[allow(unused_imports)]
pub use ultrasonic::*;

mod state;
mod stock;
pub mod persistance;
mod deed;
mod ledger;
#[cfg(feature = "stl")]
pub mod stl;

pub use deed::{CallParams, DeedBuilder};
pub use ledger::{AcceptError, Ledger, LEDGER_MAGIC_NUMBER, LEDGER_VERSION};
pub use state::{AdaptedState, EffectiveState, RawState, Transition};
pub use stock::{IssueError, LoadError, Stock, StockError};
