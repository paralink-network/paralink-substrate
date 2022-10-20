#![cfg_attr(not(feature = "std"), no_std)]

#[doc(inline)]
pub use ink_prelude::{format, vec::Vec};
pub use log::Level;

use ink_env::{DefaultEnvironment, Environment};
use ink_lang as ink;
use scale::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ParalinkEnvironment {}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, Default, TypeInfo, Copy)]
pub struct RoundData {
	pub started_at: u32,
	pub answer: u128,
	pub updated_at: u32,
	pub answered_in_round: u32,
}

#[ink::chain_extension]
pub trait ParalinkExtension {
	type ErrorCode = ParalinkError;

	#[ink(extension = 5, returns_result = false)]
	fn latest_round_data(feed_id: u32) -> RoundData;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ParalinkError {
	Fail,
}

impl ink_env::chain_extension::FromStatusCode for ParalinkError {
	fn from_status_code(status_code: u32) -> Result<(), Self> {
		match status_code {
			0 => Ok(()),
			1 => Err(Self::Fail),
			_ => panic!("encountered unknown status code"),
		}
	}
}

impl Environment for ParalinkEnvironment {
	const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

	type AccountId = <DefaultEnvironment as Environment>::AccountId;
	type Balance = <DefaultEnvironment as Environment>::Balance;
	type Hash = <DefaultEnvironment as Environment>::Hash;
	type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
	type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;

	type ChainExtension = ParalinkExtension;
}

