from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException
import binascii

# Constants
SOCKET_URL = "ws://127.0.0.1:9944"

# Helper Functions
def get_validation_strategy_dict(strategy):
    return {strategy: {
        'AutoAccept': 0,
        'ManualAccept': 1,
        'CustomAccept': 2,
    }.get(strategy, 0)}

def get_task_type_dict(task_type):
    return {task_type: {
        'DataAnnotators': 0,
        'ModelContributor': 1,
        'ModelEngineer': 2,
        'Client': 3,
    }.get(task_type, 1)}  # Default to ModelContributor

def get_storage_type_dict(storage_type):
    return {storage_type: {
        'IPFS': 0,
        'Crust': 1,
        'S3': 2,
        'GCP': 3,
        'Azure': 4,
    }.get(storage_type, {'IPFS': 0})}  # Default to IPFS

def get_annotation_type_dict(annotation_type):
    return {annotation_type: {
        'Image': 0,
        'Audio': 1,
        'Text': 2,
        'Video': 3,
    }.get(annotation_type, 0)}  # Default to Image



def create_task_data_annotator(expiration_block, substrate, sudoaccount, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, annotation_type, annotation_media_samples, annotation_files, annotation_class_labels, annotation_class_coordinates, annotation_json, annotation_files_storage_type, annotation_files_storage_credentials):
    """
    Creates a new data annotator task on the Substrate blockchain.

    Args:
        expiration_block (int): The block number when the task expires.
        substrate (SubstrateInterface): The Substrate interface to interact with the blockchain.
        sudoaccount (Keypair): The sudo account's keypair, if available.
        passphrase (str): The passphrase for generating a keypair, if sudoaccount is not provided.
        task_type (str): The type of the task.
        question (str): The question or task description.
        pays_amount (int): The amount to be paid for the task.
        max_assignments (int): The maximum number of assignments for the task.
        validation_strategy (str): The validation strategy for the task.
        annotation_type (str): The type of annotation required.
        annotation_media_samples array of str: The path to the annotation media samples.
        annotation_files array of str: The path to the annotation files.
        annotation_class_labels (str): The class labels for the annotation.
        annotation_class_coordinates (str): The coordinates for the class labels.
        annotation_json (str): Additional JSON structured data for the task.
        annotation_files_storage_type (str): The storage type for the annotation files.
        annotation_files_storage_credentials (str): The credentials for accessing the storage.
    """

    # Determine the keypair to use
    keypair = sudoaccount if sudoaccount else Keypair.create_from_mnemonic(passphrase)

    # Serialize the parameters
    task_type_dict = get_task_type_dict(task_type)
    validation_strategy_dict = get_validation_strategy_dict(validation_strategy)
    annotation_type_dict = get_annotation_type_dict(annotation_type)
    storage_type_dict = get_storage_type_dict(annotation_files_storage_type)

    # Convert the strings to byte arrays
    question_bytes = question.encode()

    # Convert string arrays to arrays of hex-strings
    media_samples_hex_array = [binascii.hexlify(sample.encode()).decode() for sample in annotation_media_samples]
    files_hex_array = [binascii.hexlify(file.encode()).decode() for file in annotation_files]

    class_labels_bytes = annotation_class_labels.encode()
    class_coordinates_bytes = annotation_class_coordinates.encode()
    json_bytes = annotation_json.encode()
    storage_credentials_bytes = annotation_files_storage_credentials.encode()

    # Compose the call with the correct structure for arrays of arrays
    call_module = 'DecentralMLModule'
    call_function = 'create_task'

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={
            'task_type': task_type_dict,
            'question': question_bytes,
            'pays_amount': pays_amount,
            'expiration_block': expiration_block,
            'max_assignments': max_assignments,
            'validation_strategy': validation_strategy_dict,
            'model_contributor_script_path': None,
            'model_contributor_script_storage_type': None,
            'model_contributor_script_storage_credentials': None,
            'annotation_type': annotation_type_dict,
            'annotation_media_samples': [media_samples_hex_array],
            'annotation_files': [files_hex_array],
            'annotation_class_labels': class_labels_bytes,
            'annotation_class_coordinates': class_coordinates_bytes,
            'annotation_json': json_bytes,
            'annotation_files_storage_type': storage_type_dict,
            'annotation_files_storage_credentials': storage_credentials_bytes,
            'model_engineer_path': None,
            'model_engineer_storage_type': None,
            'model_engineer_storage_credentials': None,
        }
    )

    # Create and send the signed extrinsic
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print(f"create_task_data_annotator Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")
    except SubstrateRequestException as e:
        print(f"create_task_data_annotator Failed to send extrinsic: {e}")

def create_task_model_engineer(expiration_block, substrate, sudoaccount, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, model_engineer_path, model_engineer_storage_type, model_engineer_storage_credentials):
    """
    Creates a new model engineer task on the Substrate blockchain.
    """

    # Determine the keypair to use
    keypair = sudoaccount if sudoaccount else Keypair.create_from_mnemonic(passphrase)

    # Serialize the parameters
    task_type_dict = get_task_type_dict(task_type)
    validation_strategy_dict = get_validation_strategy_dict(validation_strategy)
    storage_type_dict = get_storage_type_dict(model_engineer_storage_type)

    # Convert the strings to byte arrays
    question_bytes = question.encode()
    engineer_path_bytes = model_engineer_path.encode()
    storage_credentials_bytes = model_engineer_storage_credentials.encode()

    # Compose the call
    call_module = 'DecentralMLModule'
    call_function = 'create_task'

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={
            'task_type': task_type_dict,
            'question': question_bytes,
            'pays_amount': pays_amount,
            'expiration_block': expiration_block,
            'max_assignments': max_assignments,
            'validation_strategy': validation_strategy_dict,
            'model_contributor_script_path': None,  # Not applicable for Model Engineer
            'model_contributor_script_storage_type': None,
            'model_contributor_script_storage_credentials': None,
            'annotation_type': None,  # Not applicable for Model Engineer
            'annotation_media_samples': None,
            'annotation_files': None,
            'annotation_class_labels': None,
            'annotation_class_coordinates': None,
            'annotation_json': None,
            'annotation_files_storage_type': None,
            'annotation_files_storage_credentials': None,
            'model_engineer_path': engineer_path_bytes,
            'model_engineer_storage_type': storage_type_dict,
            'model_engineer_storage_credentials': storage_credentials_bytes,
        }
    )

    # Create and send the signed extrinsic
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print(f"create_task_model_engineer Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")
    except SubstrateRequestException as e:
        print(f"create_task_model_engineer Failed to send extrinsic: {e}")

def create_task_model_contributor(expiration_block, substrate, sudoaccount, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, model_contributor_script_path, model_contributor_script_storage_type, model_contributor_script_storage_credentials):
    # Determine the keypair to use
    keypair = sudoaccount if sudoaccount else Keypair.create_from_mnemonic(passphrase)

    # Serialize the parameters
    task_type_dict = get_task_type_dict(task_type)
    validation_strategy_dict = get_validation_strategy_dict(validation_strategy)
    storage_type_dict = get_storage_type_dict(model_contributor_script_storage_type)

    # Convert the strings to byte arrays
    question_bytes = question.encode()
    script_path_bytes = model_contributor_script_path.encode()
    script_credentials_bytes = model_contributor_script_storage_credentials.encode()

    # Compose the call
    call_module = 'DecentralMLModule'
    call_function = 'create_task'

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={
            'task_type': task_type_dict,
            'question': question_bytes,
            'pays_amount': pays_amount,
            'expiration_block': expiration_block,  
            'max_assignments': max_assignments,
            'validation_strategy': validation_strategy_dict,
            'model_contributor_script_path': script_path_bytes,
            'model_contributor_script_storage_type': storage_type_dict,
            'model_contributor_script_storage_credentials': script_credentials_bytes,
            # Explicitly set the rest of the parameters to None
            'annotation_type': None,
            'annotation_media_samples': None,
            'annotation_files': None,
            'annotation_class_labels': None,
            'annotation_class_coordinates': None,
            'annotation_json': None,
            'annotation_files_storage_type': None,
            'annotation_files_storage_credentials': None,
            'model_engineer_path': None,
            'model_engineer_storage_type': None,
            'model_engineer_storage_credentials': None,
        }
    )

    # Create and send the signed extrinsic
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print(f"create_task_model_contributor Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")
    except SubstrateRequestException as e:
        print(f"create_task_model_contributor Failed to send extrinsic: {e}")

def main():
    substrate = SubstrateInterface(url=SOCKET_URL)
    passphrase = None  # Assuming no passphrase is provided uses Alice

    # Common parameters for all tasks
    task_type = "ModelContributor"
    question = "Explain the functionality of the model"
    pays_amount = 1000 * 10**18  # Example amount in the smallest unit
    max_assignments = 10
    validation_strategy = 'AutoAccept'
    expiration_block = 100

    # Model Contributor specific parameters
    model_contributor_script_path = "QmeaXFDrJJdZsQo7SYMP2GBoX83Ee2sx5XNQHA5vBXP2uB"
    model_contributor_script_storage_type = "IPFS"
    model_contributor_script_storage_credentials = "ipfs_access_credentials"

    # Data Annotator specific parameters
    annotation_type = "Image"
    annotation_media_samples = ["QmeLf6QXM5AZngDaaBARN3JavpKdQmc4SgV89LUd93hBbg", "QmafZQ5WubBn5UANPYVvv7XUfBR2kP6rtLJtmBaxYseupi"]
    annotation_files = ["QmNzH7YAjr4VYsVXqJLJrjdC94rs5s487R15rXfMJtc3E7", "QmWHGj31VYv1gJvNdss5QATQkHeck2EgBg85QixkU7aKFF"]
    annotation_class_labels = "Label1,Label2"
    annotation_class_coordinates = "0,0,10,10"
    annotation_json = "{\"key\": \"value\"}"
    annotation_files_storage_type = "S3"
    annotation_files_storage_credentials = "s3_access_credentials"

    # Model Engineer specific parameters
    model_engineer_path = "QmauWpePXRSqWpvi1n9D3QA7vyZvsAvVSbVv6anAvrAahQ"
    model_engineer_storage_type = "GCP"
    model_engineer_storage_credentials = "gcp_access_credentials"

    # Determine the account to use based on passphrase availability
    if passphrase:
        create_task_model_contributor(expiration_block, substrate, None, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, model_contributor_script_path, model_contributor_script_storage_type, model_contributor_script_storage_credentials)
        create_task_data_annotator(expiration_block, substrate, None, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, annotation_type, annotation_media_samples, annotation_files, annotation_class_labels, annotation_class_coordinates, annotation_json, annotation_files_storage_type, annotation_files_storage_credentials)
        create_task_model_engineer(expiration_block, substrate, None, passphrase, task_type, question, pays_amount, max_assignments, validation_strategy, model_engineer_path, model_engineer_storage_type, model_engineer_storage_credentials)
    else:
        alice = Keypair.create_from_uri('//Alice')
        create_task_model_contributor(expiration_block, substrate, alice, None, task_type, question, pays_amount, max_assignments, validation_strategy, model_contributor_script_path, model_contributor_script_storage_type, model_contributor_script_storage_credentials)
        create_task_data_annotator(expiration_block, substrate, alice, None, task_type, question, pays_amount, max_assignments, validation_strategy, annotation_type, annotation_media_samples, annotation_files, annotation_class_labels, annotation_class_coordinates, annotation_json, annotation_files_storage_type, annotation_files_storage_credentials)
        create_task_model_engineer(expiration_block, substrate, alice, None, task_type, question, pays_amount, max_assignments, validation_strategy, model_engineer_path, model_engineer_storage_type, model_engineer_storage_credentials)

if __name__ == "__main__":
    main()


