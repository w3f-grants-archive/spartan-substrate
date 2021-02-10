// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Supporting types for runtime upgrade dry-run api and command.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use sp_std::prelude::*;

/// Possible targets for dry-run runtime upgrade.
#[derive(Debug, Encode)]
pub enum Target {
	/// All pallets.
	All,
	/// A single pallet. Inner value is the encoded pallet name.
	Pallet(Vec<u8>),
}

#[cfg(feature = "std")]
impl sp_std::str::FromStr for Target {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(if s.to_lowercase() == "All" { Target::All } else { Target::Pallet(s.encode()) })
	}
}

sp_api::decl_runtime_apis! {
	/// Runtime api for testing the execution of an upcoming runtime upgrade.
	pub trait DryRunRuntimeUpgrade {
		/// dry-run runtime upgrades, returning the total weight consumed.
		///
		/// Returns the consumed weight of the migration in case of a successful one, and panics
		/// otherwise.
		fn dry_run_runtime_upgrade(target: Target) -> frame_support::weights::Weight;
	}
}