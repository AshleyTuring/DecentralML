use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};



use frame_support::traits::{Currency, ReservableCurrency};

#[test]
fn create_fund_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        // Define accounts for creator and beneficiary
        let creator: u64 = 1;
        let beneficiary: u64 = 2;
        let goal: u128 = 10_000; // Example goal
        let end_block: u32 = 10; // Example end block

        // Ensure the Balances pallet is included in your runtime for this to work
        // Transfer enough balance to the creator to cover the submission deposit
        let submission_deposit = 1_000; // Set this to the submission deposit amount
        assert_ok!(Balances::set_balance(
            Origin::root(),
            creator,
            submission_deposit,
            0
        ));

        // Act: Call the create function of TemplatePallet
        assert_ok!(TemplateModule::create(
            Origin::signed(creator),
            beneficiary,
            goal,
            end_block,
        ));

        // Assert: Verify that the state has changed as expected, events are emitted, etc.
        // Example: Check if a fund was created
        assert!(TemplateModule::funds(0).is_some());
    });
}



#[test]
fn it_create_works_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}




#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
