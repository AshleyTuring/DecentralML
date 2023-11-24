use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};



#[test]
fn create_fund_works() {
	new_test_ext().execute_with(|| {
		// Arrange
		let beneficiary = 1u64; // Assuming AccountId is u64 in the mock
		let goal = 1000u32;      // Example goal amount
		let end = 10u64;         // Example block number for end

		// Act
		let create_result = DecentralMLModule::create(
            RuntimeOrigin::signed(1), //Origin::signed(1), // Simulating a signed call from account 1
            beneficiary,
            goal,
            end
        );

        // Check the result of the create function
        if let Err(ref e) = create_result {
            // If there's an error, print it for debugging
            println!("Create function failed with error: {:?}", e);
        }

        // Assert that create function was successful
        assert_ok!(create_result);

		// Assert
		// Check that the fund count has increased
		assert_eq!(DecentralMLModule::fund_count(), 1);

		// Check that the fund is created with correct details
		let fund = DecentralMLModule::funds(0).expect("Fund should be created");
		// assert_eq!(fund.beneficiary, beneficiary);
		// assert_eq!(fund.goal, goal);
		// assert_eq!(fund.end, end);


		// Debug: Print all events
		let events = frame_system::Pallet::<Test>::events();
		for event in events.iter() {
			println!("{:?}", event); // This will print each event to the console
		}

		System::assert_last_event(Event::Created(0,1).into());

		// Check for the expected event
		let _events = frame_system::Pallet::<Test>::events();
		assert!(_events.len() > 0);
		
		// assert!(matches!(
		// 	events[events.len() - 1].event,
		// 	RuntimeEvent::Created(index, _block_number) if index == 0
		// ));
	});
}




#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(DecentralMLModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(DecentralMLModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			DecentralMLModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
