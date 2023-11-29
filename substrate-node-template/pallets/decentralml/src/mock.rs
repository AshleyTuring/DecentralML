use crate as pallet_decentralml;
use frame_support::traits::{ConstU16, ConstU64};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};


//use frame_system::Event;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	// pub enum Test where Block = Block, NodeBlock = Block, UncheckedExtrinsic = UncheckedExtrinsic,
	// {
	// 	System: frame_system,
	// 	//DecentralMLModule: pallet_decentralml,
	// 	// System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
	// 	DecentralMLModule: pallet_decentralml::{Pallet, Call, Storage, Event<T>},
	// 	//Balances: pallet_balances::{Call, Storage, Config<T>, Event<T>},
	// }
	pub enum Test {
        //System: frame_system, //::{Pallet, Call, Config, Storage},
		//System: frame_system, //::{Pallet, Call, Config, Storage, Event<T>},
		System: frame_system, //::{Event<T>},
        DecentralMLModule: pallet_decentralml::{Pallet, Call, Storage, Event<T>},
        // Include other pallets as needed
    }
);

// pub trait Config: frame_system::Config {
//     type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
// }


impl frame_system::Config for Test {

   // type BlockNumber = u64;

   //type Event = Event;
	

	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;

}

impl pallet_decentralml::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = ();
	type MinContribution =  frame_support::traits::ConstU32<1>;
	type SubmissionDeposit = frame_support::traits::ConstU32<1>;
	type TaskReward = frame_support::traits::ConstU32<1>;
	type MaxLength = frame_support::traits::ConstU32<1024>;

}



// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let T = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
    T

	//frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
	//let mut ext = sp_io::TestExternalities::new(t);
}
