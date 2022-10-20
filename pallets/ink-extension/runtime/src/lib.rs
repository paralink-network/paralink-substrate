//! Paralink Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub use frame_support::dispatch::DispatchError;
use frame_support::dispatch::Encode;
use frame_support::log::error;
use log;

use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};

// TODO: replace with the import from ink
#[derive(Debug, Clone, PartialEq, Eq, Encode, Default, Copy)]
pub struct RoundData {
	pub started_at: u32,
	pub answer: u128,
	pub updated_at: u32,
	pub answered_in_round: u32,
}

#[derive(Debug)]
pub struct ParalinkInkExtension<T>(sp_std::marker::PhantomData<T>);

impl<T> Default for ParalinkInkExtension<T> {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<Runtime> ChainExtension<Runtime> for ParalinkInkExtension<Runtime>
where
	Runtime: pallet_contracts::Config,
	//Runtime: sublink_parachain_oracle::Config,
	Runtime: paralink_xcm::Config,
{
	fn call<E>(&mut self, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		E: Ext<T = Runtime>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		let func_id = env.func_id();
		log::info!("***** Paralink extension called {:?}", func_id);
		match func_id {
			// latest_data by id
			5 => {
				let mut env = env.buf_in_buf_out();
				//// let feed_id: <Runtime as pallet_chainlink_feed::Config>::FeedId = env.read_as_unbounded(env.in_len())?;
				//let feed_id: <<Runtime as sublink_xcm::Config>::Oracle as FeedOracle<Runtime>>::FeedId = env.read_as_unbounded(env.in_len())?;
				let feed_id: paralink_xcm::FeedId = env.read_as_unbounded(env.in_len())?;
				let feed_value: paralink_xcm::FeedValue =
					paralink_xcm::Pallet::<Runtime>::latest_data(feed_id.clone());
				//let feed = sublink_parachain_oracle::Pallet::<Runtime>::feed(feed_id.clone()).unwrap();
				//let answer = feed.latest_data();
				//
				//let feed_id = 0xB64;
				//let answer: u128 = 0x422;
				//log::info!(
				//"called latest_data extension with feed_id {:?} = {:?}",
				//feed_id,
				//answer
				//);

				let answer = RoundData {
					started_at: feed_id.id,
					answer: feed_value.value,
					updated_at: 16,
					answered_in_round: 0,
				};
				let r = answer.encode();
				env.write(&r, false, None).map_err(|_| {
					log::info!("Error when writing result");
					DispatchError::Other("ParalinkInkExtension failed to return result")
				})?;
			},

			_ => {
				error!("Called an unregistered `func_id`: {:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"));
			},
		}

		Ok(RetVal::Converging(0))
	}
}
