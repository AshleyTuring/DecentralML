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

	use frame_support::dispatch::Vec;
	use frame_support::sp_runtime::traits::Hash;
	use frame_support::sp_runtime::Saturating;

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
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct TaskInfo<AccountId, Balance, BlockNumber>//, MaxLength> 
	//where MaxLength: Get<u32>,
	{

		pub creator: AccountId,
		// pub beneficiary: AccountId,
		pub pays_amount: Balance,
	//	pub paid_amount: Balance,
		pub expiration_block: BlockNumber,  
		pub max_assignments: u32,
		pub validation_strategy: ValidationStrategy,
		pub schedule_autorefund: bool,
		pub question: Option<BoundedVec<u8,ConstU32<1024>>>,
	//	description: Option<String>,
	//	tags: Option<String>,
		pub creation_block: BlockNumber,


	
	}

//Default,
	#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub enum ValidationStrategy {
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
	#[pallet::getter(fn task_count)]
	/// The total number of tasks that have so far been allocated.
	pub(super) type TaskCount<T: Config> = StorageValue<_, TaskIndex, ValueQuery>;



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

		TaskWithdrawal(TaskIndex, BlockNumberFor<T>),
		TaskDissolved(TaskIndex, BlockNumberFor<T>),
		TaskDispensed(TaskIndex, BlockNumberFor<T>),


	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

		// you must deposit something for every job
		TaskInvalidPaysAmount,

		/// Task must end after it starts
		TaskEndTooEarly,


		/// Addition triggered an Overflow
		Overflow,


		
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {




	/// Create a new task
	#[pallet::call_index(1)]
	#[pallet::weight(10_000)]
	pub fn create_task(
        origin: OriginFor<T>,
        question: Option<BoundedVec<u8,ConstU32<1024>>>,
		// beneficiary: AccountIdOf<T>,
        pays_amount: BalanceOf<T>,
        max_assignments: u32,
        validation_strategy: ValidationStrategy,
        schedule_autorefund: bool,
        expiration_block	: BlockNumberFor<T>,

    ) -> DispatchResultWithPostInfo {
		
        // Gets the creator of the task
        let creator = ensure_signed(origin)?;

        // Validate pays_amount and max_assignments
        ensure!(pays_amount > Zero::zero(), Error::<T>::TaskInvalidPaysAmount);
     
        // Ensure the expiration block is in the future
        ensure!(expiration_block > frame_system::Pallet::<T>::block_number(), Error::<T>::TaskEndTooEarly);

	   // Create a new task index
        let task_index = TaskCount::<T>::get();
		let next_task_index = task_index.checked_add(1u32.into()).ok_or(Error::<T>::Overflow)?;
		TaskCount::<T>::put(next_task_index);
		let creation_block = frame_system::Pallet::<T>::block_number();
       
		// Transfer specified amount from the creator's account
		// Add the reward to the task's fund account
		// Always do transfers last statement 
		T::Currency::transfer(
			&creator,
			&Self::fund_task_account_id(task_index),
			pays_amount,
			ExistenceRequirement::AllowDeath
		)?;

		let balance = Self::task_reward_get(task_index, &creator);
		let balance = balance.saturating_add(pays_amount);
		Self::task_reward_put(task_index, &creator, &balance);


		// Define the new task information
		let new_task = TaskInfo {
			creator, // needed as we want to do all transfers as last 
			question,
			pays_amount,
			expiration_block,
			max_assignments,
			validation_strategy,
			schedule_autorefund,
			creation_block,
		};
	
		// Insert the new task into storage
		Tasks::<T>::insert(task_index, new_task);

		// Emit an event for task creation
		Self::deposit_event(Event::<T>::TaskCreated(task_index,creation_block));

        Ok(().into())
    }
	
	
	
	
	
	}


	impl<T: Config> Pallet<T> {

		pub fn fund_task_account_id(index: TaskIndex) -> T::AccountId {
			PALLET_ID.into_sub_account_truncating(index)
		}

		/// Record a contribution in the associated child trie.
		pub fn task_reward_put(index: TaskIndex, who: &T::AccountId, balance: &BalanceOf<T>) {
			let id = Self::id_from_task_index(index);
			who.using_encoded(|b| child::put(&id, b, &balance));
		}

		/// Lookup a contribution in the associated child trie.
		pub fn task_reward_get(index: TaskIndex, who: &T::AccountId) -> BalanceOf<T> {
			let id = Self::id_from_task_index(index);
			who.using_encoded(|b| child::get_or_default::<BalanceOf<T>>(&id, b))
		}

		/// This helper function calculates the id of the associated child trie.
		pub fn id_from_task_index(index: TaskIndex) -> child::ChildInfo {
			let mut buf = Vec::new();
			buf.extend_from_slice(b"decentml");
			buf.extend_from_slice(&index.to_le_bytes()[..]);

			child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
		}


	}
}
