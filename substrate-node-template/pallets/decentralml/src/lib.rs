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
		sp_runtime::{traits::{Zero, AccountIdConversion}},
	};

	use frame_support::PalletId;

	const PALLET_ID: PalletId = PalletId(*b"decentml");

	use frame_system::{pallet_prelude::*, ensure_signed};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	/// hello world
	#[pallet::config]
	pub trait Config: frame_system::Config {

		/// The reward to be held on deposit by the owner of a task
		type TaskReward: Get<BalanceOf<Self>>;

		/// The currency in which the decentralml project will be denominated
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The minimum amount that may be contributed into a crowdfund. Should almost certainly be at
		/// least ExistentialDeposit.
		type MinContribution: Get<BalanceOf<Self>>;

		/// max length of string question
		#[pallet::constant]
		type MaxLength: Get<u32>;


		/// The amount to be held on deposit by the owner of a crowdfund
		type SubmissionDeposit: Get<BalanceOf<Self>>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		
	}

	pub type TaskIndex = u32;
	type TaskInfoOf<T> = TaskInfo<AccountIdOf<T>, BalanceOf<T>, BlockNumberFor<T>>;//, T::MaxLength>;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

	pub type FundIndex = u32;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type FundInfoOf<T> = FundInfo<AccountIdOf<T>, BalanceOf<T>, BlockNumberFor<T>>;

	#[derive(Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct TaskInfo<AccountId, Balance, BlockNumber>//, MaxLength> 
	//where MaxLength: Get<u32>,
	{

		pub creator: AccountId,
		pub beneficiary: AccountId,
		pub pays_amount: Balance,
		pub paid_amount: Balance,
		pub expiration_block: BlockNumber,  
		pub max_assignments: u32,
		pub validation_strategy: ValidationStrategy,
		pub schedule_autorefund: bool,
		pub question: Option<BoundedVec<u8,ConstU32<1024>>>,
	//	description: Option<String>,
	//	tags: Option<String>,
	pub creation_block: BlockNumber,


		// The account that will recieve the funds if the campaign is successful
		// beneficiary: AccountId,

		// /// The amount of deposit placed
		// deposit: Balance,
		// /// The total amount raised
		// raised: Balance,
		// /// Block number after which funding must have succeeded
		// end: BlockNumber,
		// /// Upper bound on `raised`
		// goal: Balance,
	}

//Default,
	#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug), feature(field_default))]
	pub enum ValidationStrategy {
		//#[default]
		AutoAccept,
		ManualAccept,
		CustomAccept,
	}

	impl Default for ValidationStrategy {
		fn default() -> Self {
			// Choose the default variant for the enum here
			ValidationStrategy::AutoAccept // Assuming AutoAccept is the default
		}
	}




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
	#[pallet::getter(fn tasks)]
	/// Info on all of the tasks.
	pub(super) type Tasks<T: Config> = StorageMap
	<	_, 
		Blake2_128Concat, 
		TaskIndex, 
		TaskInfoOf<T>,
		OptionQuery,
	>;


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

	#[pallet::storage]
	#[pallet::getter(fn task_count)]
	/// The total number of tasks that have so far been allocated.
	pub(super) type TaskCount<T: Config> = StorageValue<_, TaskIndex, ValueQuery>;

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
		TaskCreated(TaskIndex, BlockNumberFor<T>),
		TaskInprogress(TaskIndex, BlockNumberFor<T>),
		TaskCompleted(TaskIndex, BlockNumberFor<T>),
		TaskRejected(TaskIndex, BlockNumberFor<T>),

		ValidationStrategyAutoAccept(TaskIndex, BlockNumberFor<T>),
		ValidationStrategyManualAccept(TaskIndex, BlockNumberFor<T>),
		ValidationStrategyCustomAccept(TaskIndex, BlockNumberFor<T>),

		TaskWithdrawal(FundIndex, BlockNumberFor<T>),
		TaskDissolved(FundIndex, BlockNumberFor<T>),
		TaskDispensed(FundIndex, BlockNumberFor<T>),

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

		// you must deposit something for every job
		TaskInvalidPaysAmount,

		/// Task must end after it starts
		TaskEndTooEarly,


		// you have to have at least 1 assigment count of the task 
		// InvalidMaxAssignments,

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


	/// Create a new fund
	#[pallet::call_index(0)]
	#[pallet::weight(10_000)]
	pub fn create(
		origin: OriginFor<T>,
		beneficiary: AccountIdOf<T>,
		goal: BalanceOf<T>, //,
		end: BlockNumberFor<T>,
	)-> DispatchResultWithPostInfo {
		
		let creator = ensure_signed(origin)?;

		let now = <frame_system::Pallet<T>>::block_number();
			ensure!(end > now, Error::<T>::EndTooEarly);
			let deposit = T::SubmissionDeposit::get();
		let imb = T::Currency::withdraw(
			&creator,
			deposit,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::AllowDeath,
		)?;
			
		let index = <FundCount<T>>::get();
		// not protected against overflow, see safemath section
		<FundCount<T>>::put(index + 1);
		// No fees are paid here if we need to create this account; that's why we don't just
		// use the stock `transfer`.
		T::Currency::resolve_creating(&Self::fund_account_id(index), imb);

		<Funds<T>>::insert(index, FundInfo{
			beneficiary,
			deposit,
			raised: Zero::zero(),
			end,
			goal,
		});

		Self::deposit_event(Event::Created(index, now));
		Ok(().into())
	}




	/// Create a new task
	#[pallet::call_index(1)]
	#[pallet::weight(10_000)]
	pub fn create_task(
        origin: OriginFor<T>,
        question: Option<BoundedVec<u8,ConstU32<1024>>>,
		beneficiary: AccountIdOf<T>,
        pays_amount: BalanceOf<T>,
        max_assignments: u32,
        validation_strategy: ValidationStrategy,
        schedule_autorefund: bool,
        expiration_block	: BlockNumberFor<T>,

    ) -> DispatchResultWithPostInfo {
        // Ensure that the signed origin is the creator or has the right to act on behalf of the creator
        let creator = ensure_signed(origin)?;

        // Validate pays_amount and max_assignments
        ensure!(pays_amount > Zero::zero(), Error::<T>::TaskInvalidPaysAmount);
        //ensure!(max_assignments.map_or(false, |m| m > 0), Error::<T>::InvalidMaxAssignments);

        // Ensure the expiration block is in the future
        ensure!(expiration_block > frame_system::Pallet::<T>::block_number(), Error::<T>::TaskEndTooEarly);

        // Withdraw the specified amount from the creator's account
        let imb = T::Currency::withdraw(
            &creator,
            pays_amount,
            WithdrawReasons::TRANSFER,
            ExistenceRequirement::AllowDeath,
        )?;

        // Create a new task index
        let task_index = TaskCount::<T>::get();
		let creation_block = frame_system::Pallet::<T>::block_number();
        // Define the new task information
        let new_task = TaskInfo {
            creator,
			question,
            beneficiary,
            pays_amount,
            paid_amount: Zero::zero(),
            expiration_block,
            max_assignments,
            validation_strategy,
            schedule_autorefund,
            creation_block,
        };

        // Insert the new task into storage
        Tasks::<T>::insert(task_index, new_task);
        TaskCount::<T>::put(task_index + 1);

        // Deposit the withdrawn amount into the task's fund account
        T::Currency::resolve_creating(&Self::fund_account_id(task_index), imb);

        // Emit an event for task creation
        Self::deposit_event(Event::<T>::TaskCreated(task_index,creation_block));

        Ok(().into())
    }
	
	
	
	
	
	
	
	// pub fn create_task(
	// 	origin: OriginFor<T>,
	// 	// creator: AccountIdOf<T>,
	// 	// goal: BalanceOf<T>, //,
	// 	// end: BlockNumberFor<T>,

	// 	creator: AccountIdOf<T>,
	// 	beneficiary: AccountIdOf<T>,
	// 	pays_amount: BalanceOf<T>,
	// 	paid_amount: BalanceOf<T>,
	// 	expiration_block: BlockNumberFor<T>,
	// 	max_assignments: Option<u32>,
	//  	//validation_strategy: ValidationStrategy,
	// 	schedule_autorefund: bool,
	// //	question: Option<String>,
	// //	description: Option<String>,
	// //	tags: Option<String>,
	// 	creation_block: BlockNumberFor<T>,





	// )-> DispatchResultWithPostInfo {
	// 	let creator = ensure_signed(origin)?;
	// 	let now = <frame_system::Pallet<T>>::block_number();
	// 		ensure!(expiration_block > now, Error::<T>::EndTooEarly);
			


	// 	let imb = T::Currency::withdraw(
	// 		&creator,
	// 		pays_amount,
	// 		WithdrawReasons::TRANSFER,
	// 		ExistenceRequirement::AllowDeath,
	// 	)?;
			
	// 	let index = <TaskCount<T>>::get();
	// 	// not protected against overflow, see safemath section
	// 	<TaskCount<T>>::put(index + 1);
	// 	// No fees are paid here if we need to create this account; that's why we don't just
	// 	// use the stock `transfer`.
	// 	T::Currency::resolve_creating(&Self::fund_account_id(index), imb);

	// 	<Tasks<T>>::insert(index, TaskInfo{
	// 		creator,
	// 		beneficiary,
	// 		pays_amount,
	// 		paid_amount,
	// 		expiration_block,
	// 		max_assignments,
	// 		 //validation_strategy: ValidationStrategy,
	// 		schedule_autorefund,
	// 	//	question: Option<String>,
	// 	//	description: Option<String>,
	// 	//	tags: Option<String>,
	// 		creation_block,
	// 	});

	// 	Self::deposit_event(Event::Created(index, now));
	// 	Ok(().into())
	// }









		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(2)]
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
		#[pallet::call_index(3)]
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


	impl<T: Config> Pallet<T> {
		/// The account ID of the fund pot.
		///
		/// This actually does computation. If you need to keep using it, then make sure you cache the
		/// value and only call this once.
		pub fn fund_account_id(index: FundIndex) -> T::AccountId {
			PALLET_ID.into_sub_account_truncating(index)
		}
	}
}
