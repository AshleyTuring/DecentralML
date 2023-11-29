import requests
from utilities import find_character_position
from utilities import get_substring
from utilities import remove_spaces
from utilities import remove_characters

API_KEY = "2YBXx5wpHlPPOrL91T8VwedHdz7"
API_KEY_SECRET = "497193a34170a89435eb4d9582dab729"
END_POINT = "https://ipfs.infura.io:5001"

HASH_COLON = 'Hash:'

PATH_TO_FEDERATED_MODEL = ''
PATH_TO_TRAINING_FEDERATED_DATA = ''
PATH_TO_FEDERATED_MACHINE_LEARNING_DOCKER = ''
PATH_TO_DECENTRALML_CONFIG = ''


def upload_files_to_ipfs(files):

    # files is passed as a dictionary of key (identifier) / file (filename)

    response = requests.post(f'{END_POINT}/api/v0/add', files=files, auth=(API_KEY, API_KEY_SECRET))
    hash_values = response.text.split("\n")
    hash_ids = []

    for hash_value in hash_values:

        if len(hash_value) > 1:
            hash_part = hash_value.split(',')
            hash_part_without_quotes = remove_characters(hash_part[1], '""')
            hash_colon_string_index_position = find_character_position(hash_part_without_quotes, HASH_COLON)
            hash_id = get_substring(hash_part_without_quotes, hash_colon_string_index_position + len(HASH_COLON), len(hash_part_without_quotes))

            hash_ids.append(hash_id)

    return hash_ids


def retrieve_files_from_ipfs(hash_ids):

    filenames = []

    if hash_ids is not None:

        for hash_id in hash_ids:
            params = {'arg': hash_id}
            response = requests.post(f'{END_POINT}/api/v0/cat', params=params, auth=(API_KEY, API_KEY_SECRET))
            filenames.append(response.text)

    return filenames


def delete_files_from_ipfs(hash_ids):

    responses = []
    if hash_ids is not None:

        for hash_id in hash_ids:
            params = {'arg': hash_id}
            response = requests.post(END_POINT + '/api/v0/pin/rm', params=params, auth=(API_KEY, API_KEY_SECRET))
            responses.append(response.json())

    return responses


if __name__ == "__main__":

    files = {
                'file1': '/Users/youss/Projects/Substrate/ipfs/infura_test.txt',
                'file2': '/Users/youss/Projects/Substrate/ipfs/infura_test_1.txt'
             }

    hash_ids = upload_files_to_ipfs(files)
    # files = retrieve_files_from_ipfs(hash_ids)
    # responses = delete_files_from_ipfs(files)
