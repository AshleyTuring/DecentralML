from substrateinterface import Keypair
from substrateinterface.exceptions import SubstrateRequestException


def assign_task(substrate, sudoaccount, passphrase, task_id):
    """
    Assigns a task to a worker on the Substrate blockchain.

    Args:
        substrate (SubstrateInterface): The Substrate interface to interact with the blockchain.
        sudoaccount (Keypair): The sudo account's keypair, if available.
        passphrase (str): The passphrase for generating a keypair, if sudoaccount is not provided.
        task_id (int): The ID of the task to be assigned.
    """

    # Determine the keypair to use
    keypair = sudoaccount if sudoaccount else Keypair.create_from_mnemonic(passphrase)

    # Compose the call
    call_module = 'DecentralMLModule'
    call_function = 'assign_task'

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={
            'task_id': task_id,
        }
    )

    # Create and send the signed extrinsic
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print(f"assign_task Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")
    except SubstrateRequestException as e:
        print(f"assign_task Failed to send extrinsic: {e}")