use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

use crate::ValidationStrategy;
use frame_support::BoundedVec;
use sp_core::ConstU32;

#[test]
    fn create_task_works() {
        new_test_ext().execute_with(|| {
            // Arrange
            let creator = 1; // Example creator AccountId
            let beneficiary = 1; // Example beneficiary AccountId
            let pays_amount = 1000u32; // Example pays amount
            let max_assignments = 5u32; // Example max assignments
            let validation_strategy = ValidationStrategy::AutoAccept; // Example validation strategy
            let schedule_autorefund = true;
            let expiration_block = 10; // Example expiration block

			    // Convert question string to BoundedVec
				let question_str = b"Task Question"; // Byte string
				let question_vec: Vec<u8> = question_str.to_vec(); // Convert to Vec<u8>
				let question_bounded: BoundedVec<u8, _> = question_vec.try_into().expect("Question string is too long"); // Convert to BoundedVec
				let question = Some(question_bounded); // Now the correct type
		

            // Act
            let create_task_result = DecentralMLModule::create_task(
                RuntimeOrigin::signed(creator),
                question,
                beneficiary,
                pays_amount,
                max_assignments,
                validation_strategy,
                schedule_autorefund,
                expiration_block
            );

            // Assert
            assert_ok!(create_task_result);

      // Assert the task count has increased
	  assert_eq!(DecentralMLModule::task_count(), 1);

	  // Assert the task is created with correct details
	  let task = DecentralMLModule::tasks(0).expect("Task should be created");
	  assert_eq!(task.creator, creator);
	  assert_eq!(task.beneficiary, beneficiary);
	  assert_eq!(task.pays_amount, pays_amount);
	  assert_eq!(task.max_assignments, max_assignments);
	  assert_eq!(task.validation_strategy, ValidationStrategy::AutoAccept);
	  assert_eq!(task.schedule_autorefund, schedule_autorefund);
	  assert_eq!(task.expiration_block, expiration_block);

	  // Optionally check the question if it's critical
		    // Convert question string to BoundedVec
			let questiontest_str = b"Task Question"; // Byte string
			let questiontest_vec: Vec<u8> = questiontest_str.to_vec(); // Convert to Vec<u8>
			let questiontest_bounded: BoundedVec<u8, ConstU32<1024>> = questiontest_vec.try_into()
			.expect("Question string is too long");

	  if let Some(question) = task.question {
		  assert_eq!(question, questiontest_bounded);
	  }

	  // Assert the correct event was emitted
	 // let expected_event = Event::TaskCreated(0, 1).into();

	 // let _events = System::events();

	  //assert!(System::events().iter().any(|a| a.event == expected_event));

	  //System::assert_last_event(expected_event);
			

        });
    }









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
