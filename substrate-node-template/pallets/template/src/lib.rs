#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	
	use super::*;

	use frame_support::{pallet_prelude::*, storage::child,
		traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
	};

	use frame_system::{pallet_prelude::*, ensure_signed};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	/// hello world
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		/// The currency in which the crowdfunds will be denominated
		type Currency: ReservableCurrency<Self::AccountId>;

		// The amount to be held on deposit by the owner of a crowdfund
		// type SubmissionDeposit: Get<BalanceOf<Self>>;

		/// The minimum amount that may be contributed into a crowdfund. Should almost certainly be at
		/// least ExistentialDeposit.
		type MinContribution: Get<BalanceOf<Self>>;
	}

	pub type FundIndex = u32;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
	type FundInfoOf<T> = FundInfo<AccountIdOf<T>, BalanceOf<T>, BlockNumberFor<T>>;

	#[derive(Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct FundInfo<AccountId, Balance, BlockNumber> {
		/// The account that will recieve the funds if the campaign is successful
		beneficiary: AccountId,
		/// The amount of deposit placed
		deposit: Balance,
		/// The total amount raised
		raised: Balance,
		/// Block number after which funding must have succeeded
		end: BlockNumber,
		/// Upper bound on `raised`
		goal: Balance,
	}

	#[pallet::storage]
	#[pallet::getter(fn funds)]
	/// Info on all of the funds.
	pub(super) type Funds<T: Config> = StorageMap
	<	_, 
		Blake2_128Concat, 
		FundIndex, 
		FundInfoOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn fund_count)]
	/// The total number of funds that have so far been allocated.
	pub(super) type FundCount<T: Config> = StorageValue<_, FundIndex, ValueQuery>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		Created(FundIndex, BlockNumberFor<T>),
		Contributed(<T as frame_system::Config>::AccountId, FundIndex, BalanceOf<T>, BlockNumberFor<T>),
		Withdrew(<T as frame_system::Config>::AccountId, FundIndex, BalanceOf<T>, BlockNumberFor<T>),
		Retiring(FundIndex, BlockNumberFor<T>),
		Dissolved(FundIndex, BlockNumberFor<T>, <T as frame_system::Config>::AccountId),
		Dispensed(FundIndex, BlockNumberFor<T>, <T as frame_system::Config>::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
				/// Crowdfund must end after it starts
				EndTooEarly,
				/// Must contribute at least the minimum amount of funds
				ContributionTooSmall,
				/// The fund index specified does not exist
				InvalidIndex,
				/// The crowdfund's contribution period has ended; no more contributions will be accepted
				ContributionPeriodOver,
				/// You may not withdraw or dispense funds while the fund is still active
				FundStillActive,
				/// You cannot withdraw funds because you have not contributed any
				NoContribution,
				/// You cannot dissolve a fund that has not yet completed its retirement period
				FundNotRetired,
				/// Cannot dispense funds from an unsuccessful fund
				UnsuccessfulFund,
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
