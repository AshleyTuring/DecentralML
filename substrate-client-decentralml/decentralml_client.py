from substrateinterface import SubstrateInterface
from substrateinterface import Keypair


def main():

    substrate = SubstrateInterface(url="ws://127.0.0.1:9944",)

    call_module = 'DecentralML'  # TemplateModule
    call_function = 'create_hit_'  # do_something

    call = substrate.compose_call(
        call_module=call_module,
        call_function=call_function,
        call_params={'title': 'Picture Job Request',
                     'description': 'Job request to describe a picture',
                     'question': 'What do you see?',
                     'assignmentDurationInSeconds': 3600,
                     'lifetimeInSeconds': 86400,
                     'maxAssignments': 5})

    keypair = Keypair.create_from_uri('//Alice')
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)

    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    print(f"Extrinsic '{receipt.extrinsic_hash}' sent and included in block '{receipt.block_hash}'")


if __name__ == "__main__":
    main()
