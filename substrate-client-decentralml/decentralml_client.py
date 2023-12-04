from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

# Constants
SOCKET_URL = "ws://127.0.0.1:9944"

# Helper Functions
def get_validation_strategy_dict(strategy):
    # Updated to return a dictionary with the strategy as a key and its index as a value
    return {strategy: {
        'AutoAccept': 0,
        'ManualAccept': 1,
        'CustomAccept': 2,
    }.get(strategy, 0)}

def create_task(substrate, passphrase_file_path, question, pays_amount, max_assignments, validation_strategy, schedule_autorefund, expiration_block):
    # Load the keypair
    with open(passphrase_file_path) as f:
        passphrase = f.read().strip()
    keypair = Keypair.create_from_mnemonic(passphrase)

    # Compose the call
    call_module = 'DecentralMLModule'
    call_function = 'create_task'
    

    question_bytes = question.encode()  # Convert string to bytes


    print(f"pays_amount: {pays_amount}, type: {type(pays_amount)}")
    print(f"max_assignments: {max_assignments}, type: {type(max_assignments)}")
    print(f"schedule_autorefund: {schedule_autorefund}, type: {type(schedule_autorefund)}")
    print(f"expiration_block: {expiration_block}, type: {type(expiration_block)}")


    validation_strategy_dict = get_validation_strategy_dict(validation_strategy)

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={
           'question': question_bytes,
            'pays_amount': 1 * 10**12,
            'max_assignments': max_assignments,
            'validation_strategy': validation_strategy_dict,
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
    pays_amount = 1000000000000000000
    max_assignments = 5
    validation_strategy = 'CustomAccept'
    schedule_autorefund = True
    expiration_block = 10

    # Create a task
    create_task(substrate, passphrase_file_path, question, pays_amount, max_assignments, validation_strategy, schedule_autorefund, expiration_block)

if __name__ == "__main__":
    main()


