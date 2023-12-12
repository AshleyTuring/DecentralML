import optparse


# Entry point
def main():

    """ The script is command-line enabled and accepts the following parameters:

        1. The path of the Docker file (model) the Creator uploads to IPFS
        2. The path of the data the Creator uploads to IPFS
        3. The inputs to the job the Model Creator creates:

            validationStrategy (AutoAccept, ManualAccept, CustomAccept),
            OriginatorAccountId, JobLifetime, MaxTimeToComplete, MaxJobOccurences,
            PaymentPerJob, TotalPot, (whiteAccountIdList), (blackAccountIdList)

        Example: script.py -dck ./Dockerfile -mdl_data ./data.bin -validation_strategy AutoAccept
                           -originator 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY (Alice)
                           -max_time_to_complete 5
                           -max_job_occurrences 10
                           -payment_per_job 0.003
                           -total_pot X
                           -white_account_id_list 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
                           -black_account_id_list 5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw;5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL """

    parser = optparse.OptionParser(prog='DecentralML-Script',
                                   description='Automates models uploads / Federated Learning',
                                   epilog='Help')

    parser.add_option('-dck', '--docker_file', dest="docker_file_path")
    parser.add_option('-mdl_data', '--model_data', dest="data_file_path")

    parser.add_option('-validation_strategy', '--validation_strategy', dest="validation_strategy")

    parser.add_option('-originator', '--originator_account_id', dest="validation_strategy")

    parser.add_option('-job_lifetime', '--job_lifetime', dest="validation_strategy")
    parser.add_option('-max_time_to_complete', '--max_time_to_complete', dest="validation_strategy")
    parser.add_option('-max_job_occurrences', '--max_job_occurrences', dest="validation_strategy")
    parser.add_option('-payment_per_job', '--payment_per_job', dest="validation_strategy")
    parser.add_option('-total_pot', '--total_pot', dest="validation_strategy")
    parser.add_option('-white_account_id_list', '--white_account_id_list', dest="validation_strategy")  # Optional
    parser.add_option('-black_account_id_list', '--black_account_id_list', dest="validation_strategy")  # Optional

    parser.add_option('-list_jobs', '--list_jobs', dest="validation_strategy")  # Optional

    options, args = parser.parse_args()

    print('Upload docker to IPFS...')
    ipfs_docker_id = upload_file_to_ipfs(options.model_file_path)

    print('Upload data to IPFS...')
    ipfs_data_id = upload_file_to_ipfs(options.data_file_path)

    print('Creating model contributor job...')
    job_id = create_model_contributor_job(ipfs_docker_id, ipfs_data_id)


def upload_file_to_ipfs():
    return 0  # TODO Call IPFS module functions


def create_model_contributor_job(ipfs_model_id, ipfs_data_id, job_lifetime, max_time_to_complete, validation_strategy,
                                 ipfs_docker_id, originator_account_id, max_job_occurrences, payment_per_job,
                                 total_pot, white_account_id_list, black_account_id_list):

    return 0  # TODO Call substrate-client-decentralml to create job


def list_jobs(account_id, originator_account_id, payment_mode):
    return  # TODO Call substrate-client-decentralml to get job ids stored on chain


if __name__ == '__main__':
    main()
