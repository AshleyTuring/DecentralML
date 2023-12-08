from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException
import os
import binascii
from storage_ipfs import upload_files_to_ipfs
from utilities import get_files_from_folder

# Constants
SOCKET_URL = "ws://127.0.0.1:9944"

# Helper Functions


def get_storage_type_dict(storage_type):
    return {storage_type: {
        'IPFS': 0,
        'Crust': 1,
        'S3': 2,
        'GCP': 3,
        'Azure': 4,
    }.get(storage_type, {'IPFS': 0})}  # Default to IPFS


def get_result_path():
    
    assets_directory = get_assets_directory()
    result_path_weights = get_files_from_folder(os.path.join(assets_directory, 'result_path'))
    return upload_files(result_path_weights)[0] # We only have one result of weights per task

def get_status_dict(strategy):
    return {strategy: {
        'Assigned': 0,
        'PendingValidation': 1,
        'Validated': 2,
        'Accepted': 2,
        'Rejected': 3,
    }.get(strategy, 0)}

def upload_files(files):
    
    ipfs_ids = []

    for file in files:
        params = {f'file': file}
        asset_ipfs_id = upload_files_to_ipfs(params)
        ipfs_ids.append(asset_ipfs_id)

    return ipfs_ids

def get_assets_directory():
    
    working_directory = os.getcwd()
    return os.path.join(working_directory, 'substrate-client-decentralml', 'assets')


def send_task_result(substrate, keypair, submission_id, result, result_path, result_storage_type, result_storage_credentials):
    """
    Send task result to the Substrate blockchain.
    
    Args:
        substrate (SubstrateInterface): The interface to interact with the blockchain.
        keypair (Keypair): The keypair used to sign the transaction.
        submission_id (int): The ID of the submission, this will be generated so it is 0
        task_id (int): The ID of the task this result is for
        result (str): The result of the task.
        result_path (str): The path where the weights, result file path is stored.
        result_storage_type (str): The type of storage where the result is kept.
        result_storage_credentials (str): The credentials to access the storage.
    """

    # Convert the strings to byte arrays
    result_bytes = binascii.hexlify(result.encode()).decode()
    result_path_bytes = binascii.hexlify(result_path.encode()).decode()
    storage_credentials_bytes = binascii.hexlify(result_storage_credentials.encode()).decode()

    # Compose the call
    call_module = 'DecentralMLModule'
    call_function = 'send_task_result'
    call_params = {
        'submission_id': submission_id,
        'result': result_bytes,
        'result_path': result_path_bytes,
        'result_storage_type': get_storage_type_dict(result_storage_type),
        'result_storage_credentials': storage_credentials_bytes
    }

    call = substrate.compose_call(
        call_module=call_module, 
        call_function=call_function, 
        call_params=call_params
    )

    # Create and send the signed extrinsic
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print(f"send_task_result Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")
    except SubstrateRequestException as e:
        print(f"send_task_result Failed to send extrinsic: {e}")

def main():
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

if __name__ == "__main__":
    main()