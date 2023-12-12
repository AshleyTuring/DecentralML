from substrateinterface import SubstrateInterface, Keypair

from .utilities import get_annotation_files_ids, get_annotation_samples_ids, get_model_contributor_script_id, get_model_engineer_model_id, get_result_path
from .create_task import create_task_data_annotator, create_task_model_contributor, create_task_model_engineer
from .send_task_result import send_task_result

from .settings import SOCKET_URL

def create_data_annotator_task():
    
    substrate = SubstrateInterface(url=SOCKET_URL)
    passphrase = None  # Assuming no passphrase is provided uses Alice

    # Upload assets to IPFS
    annotation_files_ids = get_annotation_files_ids()
    annotation_samples_ids = get_annotation_samples_ids()

    # Common parameters for all tasks
    task_type = "ModelContributor"
    question = "Explain the functionality of the model"
    pays_amount = 1000 * 10**18  # Example amount in the smallest unit
    max_assignments = 10
    validation_strategy = 'AutoAccept'
    expiration_block = 100

    # Data Annotator specific parameters
    annotation_type = "Image"
    annotation_media_samples = annotation_samples_ids
    annotation_files = annotation_files_ids
    annotation_class_labels = "Label1,Label2"
    annotation_class_coordinates = "0,0,10,10"
    annotation_json = "{\"key\": \"value\"}"
    annotation_files_storage_type = "S3"
    annotation_files_storage_credentials = "s3_access_credentials"

    # Determine the account to use based on passphrase availability
    if passphrase:
        create_task_data_annotator(expiration_block, substrate, None, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, annotation_type, annotation_media_samples, annotation_files, annotation_class_labels, annotation_class_coordinates, annotation_json, annotation_files_storage_type, annotation_files_storage_credentials)
    else:
        alice = Keypair.create_from_uri('//Alice')
        create_task_data_annotator(expiration_block, substrate, alice, None, task_type, question, pays_amount, max_assignments, validation_strategy, annotation_type, annotation_media_samples, annotation_files, annotation_class_labels, annotation_class_coordinates, annotation_json, annotation_files_storage_type, annotation_files_storage_credentials)



def create_model_engineer_task():
    
    substrate = SubstrateInterface(url=SOCKET_URL)
    passphrase = None  # Assuming no passphrase is provided uses Alice

    # Upload assets to IPFS
    model_engineer_path = get_model_engineer_model_id()

    # Common parameters for all tasks
    task_type = "ModelContributor"
    question = "Explain the functionality of the model"
    pays_amount = 1000 * 10**18  # Example amount in the smallest unit
    max_assignments = 10
    validation_strategy = 'AutoAccept'
    expiration_block = 100

    # Model Engineer specific parameters
    model_engineer_path = model_engineer_path
    model_engineer_storage_type = "GCP"
    model_engineer_storage_credentials = "gcp_access_credentials"

    # Determine the account to use based on passphrase availability
    if passphrase:
        create_task_model_engineer(expiration_block, substrate, None, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, model_engineer_path, model_engineer_storage_type, model_engineer_storage_credentials)
    else:
        alice = Keypair.create_from_uri('//Alice')
        create_task_model_engineer(expiration_block, substrate, alice, None, task_type, question, pays_amount, max_assignments, validation_strategy, model_engineer_path, model_engineer_storage_type, model_engineer_storage_credentials)



def create_model_contributor_task():
    
    substrate = SubstrateInterface(url=SOCKET_URL)
    passphrase = None  # Assuming no passphrase is provided uses Alice

    # Upload assets to IPFS
    model_contributor_script_path = get_model_contributor_script_id()

    # Common parameters for all tasks
    task_type = "ModelContributor"
    question = "Explain the functionality of the model"
    pays_amount = 1000 * 10**18  # Example amount in the smallest unit
    max_assignments = 10
    validation_strategy = 'AutoAccept'
    expiration_block = 100

    # Model Contributor specific parameters
    model_contributor_script_path = model_contributor_script_path
    model_contributor_script_storage_type = "IPFS"
    model_contributor_script_storage_credentials = "ipfs_access_credentials"

    # Determine the account to use based on passphrase availability
    if passphrase:
        create_task_model_contributor(expiration_block, substrate, None, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, model_contributor_script_path, model_contributor_script_storage_type, model_contributor_script_storage_credentials)
    else:
        alice = Keypair.create_from_uri('//Alice')
        create_task_model_contributor(expiration_block, substrate, alice, None, task_type, question, pays_amount, max_assignments, validation_strategy, model_contributor_script_path, model_contributor_script_storage_type, model_contributor_script_storage_credentials)


def create_all_tasks_ipfs():
    create_data_annotator_task()
    create_model_contributor_task()
    create_model_engineer_task()

def assign_task():
    substrate = SubstrateInterface(url=SOCKET_URL)
    passphrase = None  # Replace with actual passphrase or keep as None to use sudoaccount
    task_id = 1  # Replace with the actual task ID to assign

    # Determine the account to use based on passphrase availability
    if passphrase:
        assign_task(substrate, None, passphrase, task_id)
    else:
        alice = Keypair.create_from_uri('//Alice')
        assign_task(substrate, alice, None, task_id)

def send_task_results():
    substrate = SubstrateInterface(url=SOCKET_URL)
    passphrase = None  # Assuming no passphrase is provided

    # Sample data for sending task result
    submission_id = 1  # Example submission ID
    result = "result_labels_or_weights_string"
    result_path = get_result_path()  # file with the weights  
    result_storage_type = "IPFS"
    result_storage_credentials = "ipfs_access_credentials"

    # Determine the account to use based on passphrase availability
    if passphrase:
        keypair = Keypair.create_from_mnemonic(passphrase)
    else:
        keypair = Keypair.create_from_uri('//Alice')

    send_task_result(substrate, keypair, submission_id, result, result_path, result_storage_type, result_storage_credentials)

def main():
    choice = 0
    while choice != -1:
        print("Functionality to tests:")
        print("1 - Create tasks")
        print("2 - Assign tasks")
        print("3 - Send task results")
        print("4 - Exit")
        try:
            choice = int(input("Select your example: "))
            if choice > 4 or choice < 1:
                raise ValueError()
            match choice:
                case 1:
                    create_all_tasks_ipfs()
                case 2:
                    assign_task()
                case 3:
                    send_task_results()
        except ValueError as e:
            print("Choice must be one value in the interval [1-5]")
            choice = -1
    

if __name__ == "__main__":
    main()