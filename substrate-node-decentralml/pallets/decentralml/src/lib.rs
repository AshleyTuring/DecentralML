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

		/// The denominated currency for the task
		type Currency: ReservableCurrency<Self::AccountId>;

		type MinContribution: Get<u128>;
		// /// max length of string question
		// #[pallet::constant]
		// type MaxLength: Get<u32>;
		// #[pallet::constant]
		// type MaxFileCount: Get<u32>;
		// #[pallet::constant]
		// type MaxFilePathLength: Get<u32>;
		// #[pallet::constant]
		// type MaxFileCredentialLength: Get<u32>;
		// #[pallet::constant]
		// type MaxFileInstructionLength: Get<u32>;
		// #[pallet::constant]
		// type MaxQuestionLength: Get<u32>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		
	}

	pub type TaskIndex = u32;
	type TaskInfoOf<T> = TaskInfo<AccountIdOf<T>, BalanceOf<T>, BlockNumberFor<T>>; // <T as pallet::Config>::MaxQuestionLength>;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;



	#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen, Debug)]
	pub enum ValidationStrategy {
		#[default]
		AutoAccept,
		ManualAccept,
		CustomAccept,
	}

	#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen, Debug)]
	pub enum StorageType {
		#[default]
		IPFS,
		Crust,
		S3,
		GCP,
		Azure,
	}


	#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen, Debug)]
	pub enum AnnotationType {
		#[default]
		Image,
		Audio,
		Text,
		Video,
	}
	#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen, Debug)]
	pub enum TaskType {
		#[default]
		DataAnnotators,
		ModelContributor,
		ModelEngineer,
	}
	#[derive(Clone, Encode, Default, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Debug)]
	pub enum ResultSubmissionStatus {
		#[default]
		PendingValidation,
		Validated,
		Accepted,
		Rejected,
		
	}

	#[derive(Clone, Encode, Default, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Debug)]
	pub enum TaskStatus {
		
		/// The task has been created and is awaiting assignment
		#[default]
		Created,
		/// The task has been assigned to a worker
		InProgress,
		/// The task has been completed by the worker
		Completed,
		/// The task has been rejected by the worker
		Withdrawn,
	}


	// pub type FileInfoOf<T> = FileInfo<T> ;
	// pub type FileInfoListsOf<T> = BoundedVec<FileInfoOf<T>, <T as pallet::Config>::MaxFileCount>;
		

	// pub struct FileInfo<T: pallet::Config> {
	// 	pub file_path: BoundedVec<u8, <T as pallet::Config>::MaxFilePathLength>,
	// 	pub file_credentials: BoundedVec<u8, <T as pallet::Config>::MaxFileCredentialLength>,
	// 	pub file_instructions: BoundedVec<u8, <T as pallet::Config>::MaxFileInstructionLength>,
	// //		pub file_type: FileType,
	// //		pub storage_type: StorageType,
	// }



	// #[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
	// #[scale_info(skip_type_params(T))]
	// #[codec(mel_bound(T: pallet::Config))]
	// #[derive(frame_support::DebugNoBound)]
	// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

	// #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	// #[scale_info(skip_type_params(T))]
	// #[codec(mel_bound())]
	// //#[cfg_attr(feature = "std", derive(Debug))]

	#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct TaskResultSubmission<AccountId, Balance, BlockNumber> 
	{

		/// the task id note there is a 1 to many relationship between tasks and submissions
		pub task_id: u32,

		/// the worker that submitted the task
		pub worker: AccountId,
		
		/// the block the task was created
		pub created_block: BlockNumber,
		
		/// optional send result as a string if possiblw
		pub result: Option<BoundedVec<u8,ConstU32<1024>>>,// string
		/// path to weights / id / or annotator results
		pub result_path: Option<BoundedVec<u8,ConstU32<1024>>>,
		/// this is where the weights  will be stored e.g. IPFS, S3, GCP, Azure, etc
		pub result_storage_type: Option<StorageType>,
		/// credentials to access the weights
		pub result_storage_credentials: Option<BoundedVec<u8, ConstU32<1024>>>,
		/// status of the submission PendingValidation, Validated, Accepted, Rejected
		pub status: ResultSubmissionStatus,
		/// the amount paid for the task
		pub paid_amount: Option<Balance>,
		/// the block the task was paid
		pub paid_block: Option<BlockNumber>,


	}


	/// This struct is a denormalised representation of tasks for model contributor, model engineer, data annotator
	/// We may refactor this structure and clean it into separate structs for now we are leaving it as is based on KISS
	#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct TaskInfo<AccountId, Balance, BlockNumber> {
		/// Task status of created, in progress, completed, rejected.
		pub status: TaskStatus,
	
		/// Task type of model contributor, model engineer, data annotator, client.
		pub task_type: TaskType,
	
		/// Account that created the task.
		pub creator: AccountId,
	
		/// Reward paid for task completion.
		pub pays_amount: Balance,
	
		/// Set when the task is created.
		pub creation_block: BlockNumber,
	
		/// Task expires at this block.
		pub expiration_block: BlockNumber,
	
		/// Max number of assignments for this task.
		pub max_assignments: u32,
	
		/// Validation strategy for this task AutoAccept, ManualAccept, CustomAccept.
		pub validation_strategy: ValidationStrategy,
	
		/// Question to be answered by workers. Increased to 2048 to accommodate detailed questions.
		pub question: Option<BoundedVec<u8, ConstU32<2048>>>,
	
		/// Path to script that will be executed by model contributors. 
		/// Length 512 as script paths are usually not very long.
		pub model_contributor_script_path: Option<BoundedVec<u8, ConstU32<512>>>,
	
		/// This is where the script will be stored e.g. IPFS etc.
		/// Short names for storage types.
		pub model_contributor_script_storage_type: Option<StorageType>,
	
		/// Credentials to access the file if needed. Length 1024 for keys or tokens.
		pub model_contributor_script_storage_credentials: Option<BoundedVec<u8, ConstU32<1024>>>,
	
		/// Annotation type e.g. image, audio, text, video. Short names for annotation types.
		pub annotation_type: Option<AnnotationType>,
	
		/// Path to annotation samples (e.g. set of images, audio files). Length 512 for variable media 100 paths.
		pub annotation_media_samples: Option<BoundedVec<BoundedVec<u8, ConstU32<512>>, ConstU32<100>>>,
	
		/// This is a list of the file names for the annotation samples. 
		/// Length 512 for each file name and for the list to accommodate 100 files.
		pub annotation_files: Option<BoundedVec<BoundedVec<u8, ConstU32<512>>, ConstU32<100>>>,
	
		/// Class labels for the annotation samples. Length 1024 for multiple short labels.
		pub annotation_class_labels: Option<BoundedVec<u8, ConstU32<1024>>>,
	
		/// Class coordinates for the annotation samples. Length 2048 for detailed coordinates.
		pub annotation_class_coordinates: Option<BoundedVec<u8, ConstU32<2048>>>,
	
		/// Contains structured parameters for the annotation samples. Length 4096 for potentially large JSON data.
		pub annotation_json: Option<BoundedVec<u8, ConstU32<4096>>>,
	
		/// Where the model will be stored e.g. IPFS, S3, GCP, Azure, etc. Short names for storage types.
		pub annotation_files_storage_type: Option<StorageType>,
	
		/// Credentials to access the model. Length 1024 for keys or tokens.
		pub annotation_files_storage_credentials: Option<BoundedVec<u8, ConstU32<1024>>>,
	
		/// Path to model / ID. Length 512 as model paths are usually not very long.
		pub model_engineer_path: Option<BoundedVec<u8, ConstU32<512>>>,
	
		/// Where the model will be stored e.g. IPFS, S3, GCP, Azure, etc. Short names for storage types.
		pub model_engineer_storage_type: Option<StorageType>,
	
		/// Credentials to access the model. Length 1024 for keys or tokens.
		pub model_engineer_storage_credentials: Option<BoundedVec<u8, ConstU32<1024>>>,

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
	

		TaskCreated{ taskid: TaskIndex, created: BlockNumberFor<T>},
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
		/// if the task type is data annotation then we must have a annotation type
		MissingAnnotationType,
		/// if the task type is data annotation then we must have files to annotate
		MissingAnnotationFiles,
		/// if the task type is data annotation  we need to know where the files are coming from
		MissingAnnotationFilesStorageType,
		/// if the task type is model contributor we need to know the script path
		MissingModelContributorScriptPath,
		/// if the task type is model contributor we need to know where the file is stored
		MissingModelContributorScriptStorageType,
		/// if the task type is model engineer we need to know the path of the model
		MissingModelEngineerPath,
		/// if the task type is model engineer we to knwo the storage type
		MissingModelEngineerStorageType,
		/// need to have at least 1 assignment otherwise it is an invalid task 
		InvalidMaxAssignments,
		/// need to have at least 1 character in the question otherwise it's an invalid task
		InvalidQuestion,
		/// Invalid task type Model Engineer, 
		InvalidTaskType

	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {




	/// Create a new task
	#[pallet::call_index(0)]
	#[pallet::weight(10_000)]
	pub fn create_task(
		origin: OriginFor<T>,
		task_type: TaskType,
		question: Option<BoundedVec<u8, ConstU32<2048>>>,
		pays_amount: BalanceOf<T>,
		expiration_block: BlockNumberFor<T>,
		max_assignments: u32,
		validation_strategy: ValidationStrategy,
		model_contributor_script_path: Option<BoundedVec<u8, ConstU32<512>>>,
		model_contributor_script_storage_type: Option<StorageType>,
		model_contributor_script_storage_credentials: Option<BoundedVec<u8, ConstU32<1024>>>,
		annotation_type: Option<AnnotationType>,
		annotation_media_samples: Option<BoundedVec<BoundedVec<u8, ConstU32<512>>, ConstU32<100>>>,
		annotation_files: Option<BoundedVec<BoundedVec<u8, ConstU32<512>>, ConstU32<100>>>,
		annotation_class_labels: Option<BoundedVec<u8, ConstU32<1024>>>,
		annotation_class_coordinates: Option<BoundedVec<u8, ConstU32<2048>>>,
		annotation_json: Option<BoundedVec<u8, ConstU32<4096>>>,
		annotation_files_storage_type: Option<StorageType>,
		annotation_files_storage_credentials: Option<BoundedVec<u8, ConstU32<1024>>>,
		model_engineer_path: Option<BoundedVec<u8, ConstU32<512>>>,
		model_engineer_storage_type: Option<StorageType>,
		model_engineer_storage_credentials: Option<BoundedVec<u8, ConstU32<1024>>>,
	) -> DispatchResultWithPostInfo {
		// Validates that the function caller is a signed account
		let creator = ensure_signed(origin)?;
	
		// Validates that the pays_amount is greater than zero
		ensure!(pays_amount > Zero::zero(), Error::<T>::TaskInvalidPaysAmount);
	
		// Validates that max_assignments is greater than zero
		ensure!(max_assignments > 0, Error::<T>::InvalidMaxAssignments);
	
		// Validates that the expiration block is in the future
		ensure!(expiration_block > frame_system::Pallet::<T>::block_number(), Error::<T>::TaskEndTooEarly);
	
		// Validates that the question is provided and not empty
		ensure!(question.as_ref().map(|q| !q.is_empty()).unwrap_or(false), Error::<T>::InvalidQuestion);
	
		// Validates specific fields based on TaskType
		match task_type {
			TaskType::DataAnnotators => {
				ensure!(annotation_type.is_some(), Error::<T>::MissingAnnotationType);
				ensure!(annotation_files.as_ref().map(|af| !af.is_empty()).unwrap_or(false), Error::<T>::MissingAnnotationFiles);
				ensure!(annotation_files_storage_type.is_some(), Error::<T>::MissingAnnotationFilesStorageType);
				
				// removed validation as per Data Scientist guidance Mathias
				//ensure!(annotation_media_samples.as_ref().map(|ams| !ams.is_empty()).unwrap_or(false), Error::<T>::MissingAnnotationMediaSamples);
				//ensure!(annotation_class_labels.is_some(), Error::<T>::MissingAnnotationClassLabels);
				//ensure!(annotation_class_coordinates.is_some(), Error::<T>::MissingAnnotationClassCoordinates);
				//ensure!(annotation_json.is_some(), Error::<T>::MissingAnnotationJson);
				

			},
			TaskType::ModelContributor => {
				ensure!(model_contributor_script_path.is_some(), Error::<T>::MissingModelContributorScriptPath);
				ensure!(model_contributor_script_storage_type.is_some(), Error::<T>::MissingModelContributorScriptStorageType);
			

			
			},
			TaskType::ModelEngineer => {
				ensure!(model_engineer_path.is_some(), Error::<T>::MissingModelEngineerPath);
				ensure!(model_engineer_storage_type.is_some(), Error::<T>::MissingModelEngineerStorageType);



			},
			_ => {
				ensure!(false, Error::<T>::InvalidTaskType);
			}
		}
	
		// Generates a unique index for the new task
		let task_index = TaskCount::<T>::get();
		TaskCount::<T>::put(task_index.checked_add(1).ok_or(Error::<T>::Overflow)?);
		let creation_block = frame_system::Pallet::<T>::block_number();
	
		// Transfers the specified amount from the creator's account to the task's fund account
		T::Currency::transfer(
			&creator,
			&Self::fund_task_account_id(task_index),
			pays_amount,
			ExistenceRequirement::AllowDeath
		)?;
	
		// Records the task reward
		let balance = Self::task_reward_get(task_index, &creator);
		let balance = balance.saturating_add(pays_amount);
		Self::task_reward_put(task_index, &creator, &balance);
	
		// Defines the new task information
		let new_task = TaskInfo {
			status: TaskStatus::Created, // Task is always created initially
			task_type,
			creator,
			question,
			pays_amount,
			creation_block,
			expiration_block,
			max_assignments,
			validation_strategy,
			model_contributor_script_path,
			model_contributor_script_storage_type,
			model_contributor_script_storage_credentials,
			annotation_type,
			annotation_media_samples,
			annotation_files,
			annotation_class_labels,
			annotation_class_coordinates,
			annotation_json,
			annotation_files_storage_type,
			annotation_files_storage_credentials,
			model_engineer_path,
			model_engineer_storage_type,
			model_engineer_storage_credentials,
		};
	
		// Inserts the new task into storage
		Tasks::<T>::insert(task_index, new_task);
	
		// Emits an event for task creation
		Self::deposit_event(Event::TaskCreated{taskid:task_index, created:creation_block});
	
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
