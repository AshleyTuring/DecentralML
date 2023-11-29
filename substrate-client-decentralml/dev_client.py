    
from substrateinterface import SubstrateInterface
from substrateinterface import Keypair


def main():


    # upload the model to IPFS
    model_file = open(PATH_TO_FEDERATED_MODEL, 'rb')
    model_hash_id = upload_files_to_ipfs({'file1': model_file})[0]
    model_file.close()

    # upload the training data to IPFS
    training_data_file = open(PATH_TO_TRAINING_FEDERATED_DATA, 'rb')
    training_data_hash_id = upload_files_to_ipfs({'file1': training_data_file})[0]
    training_data_file.close()

    # upload the docker image to IPFS
    docker_image_file = open(PATH_TO_FEDERATED_MACHINE_LEARNING_DOCKER, 'rb')
    docker_image_hash_id = upload_files_to_ipfs({'file1': docker_image_file})[0]

    print("Connecting to substrate node...")

    substrate = SubstrateInterface(url="ws://127.0.0.1:9944",)

    


    create_task('C:\__DecentralML\TestWallets\Polkdot.jsMAIN',)

    print(f"Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")

    # call_module = 'TemplateModule'  # TemplateModule
    # call_function = 'do_something'  # do_something

    # call = substrate.compose_call(
    #     call_module=call_module,
    #     call_function=call_function,
    #     call_params={'something': 42})

    # call_module = 'DecentralML'  # TemplateModule
    # call_function = 'create_hit_'  # do_something

    # call = substrate.compose_call(
    #     call_module=call_module,
    #     call_function=call_function,
    #     call_params={'title': 'Picture Job Request',
    #                  'description': 'Job request to describe a picture',
    #                  'question': 'What do you see?',
    #                  'assignmentDurationInSeconds': 3600,
    #                  'lifetimeInSeconds': 86400,
    #                  'maxAssignments': 5})

    # keypair = Keypair.create_from_uri('//Alice')
    # extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)

    # receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    # print(f"Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")


def create_task(passphrase_file_path, beneficiary_account_id, goal, end):

    substrate = SubstrateInterface(url=SOCKET_URL)

    call_module = 'DecentralMLModule'  # TemplateModule
    call_function = 'create'  # do_something

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={'beneficiary': beneficiary_account_id, 'goal': goal, 'end': end})

    # keypair = Keypair.create_from_uri('//Alice')
    # with open(account_json_recovery_file_path, 'r') as j:
    #     json_data = json.loads(j.read())

    with open(passphrase_file_path) as f:
        passphrase = f.read() # fatal inject wave unusual accuse suit divide grit equal bundle diet pistol

    #keypair = Keypair.create_from_encrypted_json(json_data, passphrase)
    keypair = Keypair.create_from_mnemonic(passphrase)

    private_key = keypair.private_key
    public_key = keypair.public_key

    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    print(f"Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")



if __name__ == "__main__":
    main()
