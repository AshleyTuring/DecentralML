import binascii

from substrateinterface.exceptions import SubstrateRequestException

from .utilities import get_storage_type_dict


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