use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

use crate::{ResultSubmissionStatus, TaskResultSubmission, ValidationStrategy,TaskType,StorageType,TaskStatus,TaskInfo,Tasks, TaskResultSubmissionCountByTaskId,TaskResultSubmissionCount,TaskResultSubmissions};
use frame_support::BoundedVec;


#[test]
fn create_task_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let creator = 1; // Example creator AccountId
        let pays_amount = 1000u32.into(); // Example pays amount as Balance
        let expiration_block: u64 = 10; // Example expiration block
        let max_assignments = 5u32; // Example max assignments
        let validation_strategy = ValidationStrategy::AutoAccept; // Example validation strategy

        // Example task type
        let task_type = TaskType::ModelContributor;

        // Example question
        let question_str = "Train the model using this script".as_bytes().to_vec();
        let question_bounded = BoundedVec::try_from(question_str).unwrap();

        // Example model contributor script path
        let script_path_str = "/path/to/ipfs-id".as_bytes().to_vec();
        let script_path_bounded = BoundedVec::try_from(script_path_str).unwrap();

        // Example storage type
        let storage_type = Some(StorageType::IPFS);

        // Act
        let create_task_result = DecentralMLModule::create_task(
            RuntimeOrigin::signed(creator),
            task_type,
            Some(question_bounded),
            pays_amount,
            expiration_block,
            max_assignments,
            validation_strategy,
            Some(script_path_bounded),
            storage_type,
            None, // model_contributor_script_storage_credentials
            None, // annotation_type
            None, // annotation_media_samples
            None, // annotation_files
            None, // annotation_class_labels
            None, // annotation_class_coordinates
            None, // annotation_json
            None, // annotation_files_storage_type
            None, // annotation_files_storage_credentials
            None, // model_engineer_path
            None, // model_engineer_storage_type
            None, // model_engineer_storage_credentials
        );

        // Assert
        assert_ok!(create_task_result);

        // Assert the task count has increased
        assert_eq!(DecentralMLModule::task_count(), 1);

        // Assert the task is created with correct details
        let task = DecentralMLModule::tasks(0).expect("Task should be created");
        assert_eq!(task.creator, creator);
        assert_eq!(task.pays_amount, pays_amount);
        assert_eq!(task.max_assignments, max_assignments);
        assert_eq!(task.validation_strategy, ValidationStrategy::AutoAccept);
        assert_eq!(task.task_type, TaskType::ModelContributor);
        assert_eq!(task.expiration_block, expiration_block);


        //System::assert_last_event(Event::TaskCreated { taskid: 1, created: 1 }.into());


    });
}


#[test]
fn assign_task_successfully() {
    new_test_ext().execute_with(|| {
        // Arrange: Create a task with a certain ID and maximum assignments
        let task_id = 1;
        let max_assignments = 3;
        let task = TaskInfo {
            status: TaskStatus::Created,
            task_type: TaskType::DataAnnotators, // Example task type
            creator: 1, // Example creator AccountId
            pays_amount: 1000, // Example pays amount
            creation_block: 1, // Example creation block number
            expiration_block: 10, // Example expiration block number
            max_assignments,
            validation_strategy: ValidationStrategy::AutoAccept, // Example validation strategy
            question: Some(vec![1, 2, 3, 4].try_into().unwrap()), // Example question
            model_contributor_script_path: None,
            model_contributor_script_storage_type: None,
            model_contributor_script_storage_credentials: None,
            annotation_type: None,
            annotation_media_samples: None,
            annotation_files: None,
            annotation_class_labels: None,
            annotation_class_coordinates: None,
            annotation_json: None,
            annotation_files_storage_type: None,
            annotation_files_storage_credentials: None,
            model_engineer_path: None,
            model_engineer_storage_type: None,
            model_engineer_storage_credentials: None,
        };
        Tasks::<Test>::insert(task_id, task);

        // Act: Assign the task
        assert_ok!(DecentralMLModule::assign_task(RuntimeOrigin::signed(1), task_id));

        // Assert: Check that the task result submission count increased
        let submission_count = TaskResultSubmissionCountByTaskId::<Test>::get(task_id).unwrap();
        assert_eq!(submission_count, 1);

        // Assert: Check that the task result submission is stored correctly
        let submission_index = TaskResultSubmissionCount::<Test>::get();
        let submission = TaskResultSubmissions::<Test>::get(submission_index-1).unwrap();
        assert_eq!(submission.task_id, task_id);
        assert_eq!(submission.worker, 1);

    });
}




fn create_and_assign_task(task_id: u32, worker_account: u32) {
    let task_id = 0;
    let max_assignments = 3;
    // Create a task
    let task = TaskInfo {
        status: TaskStatus::Created,
        task_type: TaskType::DataAnnotators, // Example task type
        creator: 1, // Example creator AccountId
        pays_amount: 1000, // Example pays amount
        creation_block: 1, // Example creation block number
        expiration_block: 10, // Example expiration block number
        max_assignments,
        validation_strategy: ValidationStrategy::AutoAccept, // Example validation strategy
        question: Some(vec![1, 2, 3, 4].try_into().unwrap()), // Example question
        model_contributor_script_path: None,
        model_contributor_script_storage_type: None,
        model_contributor_script_storage_credentials: None,
        annotation_type: None,
        annotation_media_samples: None,
        annotation_files: None,
        annotation_class_labels: None,
        annotation_class_coordinates: None,
        annotation_json: None,
        annotation_files_storage_type: None,
        annotation_files_storage_credentials: None,
        model_engineer_path: None,
        model_engineer_storage_type: None,
        model_engineer_storage_credentials: None,
    };
    Tasks::<Test>::insert(task_id, task);

    // Assign the task
    let submission = TaskResultSubmission {
        task_id,
        submission_id: 0,
        worker: 1u64,
        created_block: 1u64,
        result: None,
        result_path: None,
        result_storage_type: None,
        result_storage_credentials: None,
        status: ResultSubmissionStatus::Assigned,
        paid_amount: None,
        paid_block: None,
    };
    let submission_index = TaskResultSubmissionCount::<Test>::get();
    TaskResultSubmissions::<Test>::insert(submission_index, submission);
    TaskResultSubmissionCount::<Test>::put(submission_index + 1);
    TaskResultSubmissionCountByTaskId::<Test>::insert(task_id, 1);
}



// #[test]
// fn send_task_result_success() {
//     new_test_ext().execute_with(|| {
//         // Arrange
//         let worker_account = 1;
//         let task_id = 0;
//         create_and_assign_task(task_id, worker_account);

//         // Act
//         assert_ok!(DecentralMLModule::send_task_result(RuntimeOrigin::signed(1), 0));

//         // Assert
//         let updated_submission = TaskResultSubmissions::<Test>::get(0).unwrap();
//         assert_eq!(updated_submission.status, ResultSubmissionStatus::PendingValidation);
//     });
// }

// ... other tests ...



// #[test]
// fn create_task_works() {
//     new_test_ext().execute_with(|| {
//         // Arrange
//         let creator = 1; // Example creator AccountId
//         let pays_amount = 1000u32; // Example pays amount
//         let max_assignments = 5u32; // Example max assignments
//         let validation_strategy = ValidationStrategy::AutoAccept; // Example validation strategy
//         let schedule_autorefund = true;
//         let expiration_block = 10; // Example expiration block

//         // Convert question string to BoundedVec
//         let question_str = b"Task Question"; // Byte string
//         let question_vec: Vec<u8> = question_str.to_vec(); // Convert to Vec<u8>
//         let question_bounded: Result<BoundedVec<u8, ConstU32<1024>>, _> = question_vec.try_into(); // Convert to BoundedVec
//         let question = question_bounded.ok(); // Convert Result to Option

//         // Check creator's balance before creating the task
//         //let initial_balance = pallet_balances::Pallet::<DecentralMLModule>::free_balance(&creator);

//         // Act
//         assert_ok!(DecentralMLModule::create_task(
//             RuntimeOrigin::signed(creator),
//             question,
//             pays_amount,
//             max_assignments,
//             validation_strategy,
//             schedule_autorefund,
//             expiration_block
//         ));

//         // Assert the task count has increased
//         assert_eq!(DecentralMLModule::task_count(), 1);

//         // Assert the task is created with correct details
//         let task = DecentralMLModule::tasks(0).expect("Task should be created");
//         assert_eq!(task.creator, creator);
//         assert_eq!(task.pays_amount, pays_amount);
//         assert_eq!(task.max_assignments, max_assignments);
//         assert_eq!(task.validation_strategy, ValidationStrategy::AutoAccept);
//         assert_eq!(task.schedule_autorefund, schedule_autorefund);
//         assert_eq!(task.expiration_block, expiration_block);

//         // Optionally check the question if it's critical
//         if let Some(ref question_in_task) = task.question {
//            // assert_eq!(*question_in_task, question.expect("Question should be valid"));
//         }

//         // Check creator's balance after creating the task
//         //let final_balance = pallet_balances::Pallet::<Test>::free_balance(&creator);
//         //assert!(final_balance < initial_balance, "Creator's balance should decrease");

//         // Assert the correct event was emitted
//         // let expected_event = Event::DecentralMLModule(crate::Event::TaskCreated(0, expiration_block));
//         // let events = frame_system::Pallet::<Test>::events();
//         // assert!(events.iter().any(|record| record.event == expected_event), "Expected TaskCreated event not found");
//     });
// }