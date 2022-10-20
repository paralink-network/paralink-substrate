#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_system::Config as SystemConfig;
use sp_std::prelude::*;
use xcm::latest::prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		/// The overarching call type; we assume sibling chains use the same type.
		type Call: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;
	}

	#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct FeedId {
		pub id: u32,
	}

	#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct FeedValue {
		pub value: u128,
	}

	/// Parachains interested in price feeds
	#[pallet::storage]
	pub(super) type RegisteredParachains<T: Config> =
		StorageValue<_, Vec<(ParaId, FeedId)>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn feeds)]
	pub(super) type Feeds<T: Config> = StorageMap<_, Twox64Concat, FeedId, FeedValue, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FeedValueUpdated(FeedId, FeedValue, T::AccountId),
		CurrentFeedValue(FeedId, FeedValue, T::AccountId),
		//SendRequestForLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		//SendRequestForLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		ReceiveRequestForLatestPriceValue(ParaId, FeedId),
		SendLatestPriceValue(ParaId, FeedId, FeedValue),
		ReceiveLatestPriceValue(ParaId, FeedId, FeedValue),
		//ErrorSendingRequest(SendError, ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		ErrorSendingLatestPriceValue(SendError, ParaId, FeedId, FeedValue),
	}

	#[pallet::error]
	pub enum Error<T> {
		FeedMissing,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: T::BlockNumber) {
			for (parachain_id, feed_id) in RegisteredParachains::<T>::get().into_iter() {
				log::info!("***** Paralink XCM on_finalize {:?}, {:?}", parachain_id, feed_id.id);
				let feed_value = Feeds::<T>::get(feed_id.clone());
				Self::send_latest_data_through_xcm(parachain_id, feed_id.clone(), feed_value);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn store_latest_data(
			origin: OriginFor<T>,
			feed_id: FeedId,
			value: FeedValue,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Feeds::<T>::insert(feed_id.clone(), value.clone());

			Self::deposit_event(Event::FeedValueUpdated(feed_id, value, who));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn get_latest_data(origin: OriginFor<T>, feed_id: FeedId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let feed = Feeds::<T>::get(feed_id.clone());
			match feed {
				Some(x) => Self::deposit_event(Event::CurrentFeedValue(feed_id, x, who)),
				None => Self::deposit_event(Event::CurrentFeedValue(
					feed_id,
					FeedValue { value: 66666 },
					who,
				)),
			}

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn receive_latest_data(
			origin: OriginFor<T>,
			feed_id: FeedId,
			latest_feed_value: FeedValue,
		) -> DispatchResult {
			log::info!("***** Paralink XCM store_latest_data called");
			let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			log::info!(
				"***** Paralink XCM Received latest_value = {:?}",
				latest_feed_value.clone().value
			);

			Feeds::<T>::insert(feed_id.clone(), latest_feed_value.clone());
			Self::deposit_event(Event::ReceiveLatestPriceValue(para, feed_id, latest_feed_value));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn register_feed_to_paralink(
			_origin: OriginFor<T>,
			para_id: ParaId,
			feed_id: FeedId,
		) -> DispatchResult {
			log::info!("***** Paralink XCM get_latest_data called");
			//let parachain_id = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			Self::deposit_event(Event::ReceiveRequestForLatestPriceValue(para_id, feed_id.clone()));
			RegisteredParachains::<T>::mutate(|t| {
				if t.iter().position(|(p, f)| p == &para_id && f.id == feed_id.id) == None {
					t.push((para_id, feed_id.clone()));
				}
			});

			let feed_value = Feeds::<T>::get(feed_id.clone());
			Self::send_latest_data_through_xcm(para_id, feed_id.clone(), feed_value.clone());

			Ok(())
		}
	}

	//  ---------------------- non-callable
	impl<T: Config> Pallet<T> {
		pub fn latest_data(feed_id: FeedId) -> FeedValue {
			Feeds::<T>::get(feed_id.clone()).unwrap_or_else(|| {
				log::info!("***** Paralink Parachain Oracle no round for {:?}", feed_id);
				FeedValue { value: 666 }
			})
		}

		fn send_latest_data_through_xcm(
			parachain_id: ParaId,
			feed_id: FeedId,
			latest_feed_value: Option<FeedValue>,
		) {
			log::info!(
				"***** Paralink XCM send_latest_data_through_xcm called para_id: {:?},  feed: {:?}, value:  {:?}",
				parachain_id,
				feed_id,
				latest_feed_value
			);
			if let Some(latest_feed_val) = latest_feed_value {
				match T::XcmSender::send_xcm(
					(1, Junction::Parachain(parachain_id.into())),
					Xcm(vec![Transact {
						origin_type: OriginKind::Native,
						require_weight_at_most: 1_000,
						call: <T as Config>::Call::from(Call::<T>::receive_latest_data {
							feed_id: feed_id.clone(),
							latest_feed_value: latest_feed_val.clone(),
						})
						.encode()
						.into(),
					}]),
				) {
					Ok(()) => {
						log::info!("***** Paralink XCM get_latest_data called store_latest_data");
						Self::deposit_event(Event::SendLatestPriceValue(
							parachain_id,
							feed_id,
							latest_feed_val,
						))
					},
					Err(e) => {
						log::error!(
							"***** Paralink XCM get_latest_data cannot called store_latest_data"
						);
						Self::deposit_event(Event::ErrorSendingLatestPriceValue(
							e,
							parachain_id,
							feed_id,
							latest_feed_val,
						))
					},
				}
			}

			log::info!("***** Paralink XCM send_latest_data_through_xcm exited");
		}
	}
}
