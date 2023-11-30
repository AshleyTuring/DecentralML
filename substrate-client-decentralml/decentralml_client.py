from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

# Constants
SOCKET_URL = "ws://127.0.0.1:9944"



def create_task(substrate, passphrase_file_path, question, beneficiary_account_id, pays_amount, max_assignments, validation_strategy, schedule_autorefund, expiration_block):
    # Load the keypair
    with open(passphrase_file_path) as f:
        passphrase = f.read().strip()
    keypair = Keypair.create_from_mnemonic(passphrase)

    # Compose the call
    call_module = 'DecentralMLModule'
    call_function = 'create_task'
    validation_strategy_index = get_validation_strategy_index(validation_strategy)

    question_bytes = question.encode()  # Convert string to bytes

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={
            'question': question_bytes,
            'beneficiary': beneficiary_account_id,
            'pays_amount': pays_amount,
            'max_assignments': max_assignments,
            'validation_strategy': validation_strategy_index,
            'schedule_autorefund': schedule_autorefund,
            'expiration_block': expiration_block
        }
    )

    # Create and send the signed extrinsic
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print(f"Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")
    except SubstrateRequestException as e:
        print(f"Failed to send extrinsic: {e}")

def main():
    # Connect to the Substrate node
    substrate = SubstrateInterface(url=SOCKET_URL)
    import os
    current_path = os.getcwd()
    # Example values, replace with real data
    passphrase_file_path = r'/home/ashsubband/decentralML/substrate-client-decentralml/testwallet_passphrase.txt'
    with open(passphrase_file_path) as f:
        passphrase = f.read() # fatal inject wave unusual accuse suit divide grit equal bundle diet pistol

    question = "Some question"
    beneficiary_account_id = 1
    pays_amount = 1
    max_assignments = 5
    validation_strategy = "CustomAccept"
    schedule_autorefund = True
    expiration_block = 10

    # Create a task
    create_task(substrate, passphrase_file_path, question, beneficiary_account_id, pays_amount, max_assignments, validation_strategy, schedule_autorefund, expiration_block)

if __name__ == "__main__":
    main()

# Helper Functions
def get_validation_strategy_index(strategy):
    mapping = {
        'AutoAccept': 0,
        'ManualAccept': 1,
        'CustomAccept': 2,
    }
    return mapping.get(strategy, 0)
