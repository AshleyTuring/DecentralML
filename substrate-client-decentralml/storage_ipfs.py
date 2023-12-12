import requests
from .utilities import find_character_position
from .utilities import get_substring
from .utilities import remove_spaces
from .utilities import remove_characters

from .settings import IPFS_API_KEY, IPFS_API_SECRET

HASH_COLON = 'Hash:'

PATH_TO_FEDERATED_MODEL = ''
PATH_TO_TRAINING_FEDERATED_DATA = ''
PATH_TO_FEDERATED_MACHINE_LEARNING_DOCKER = ''
PATH_TO_DECENTRALML_CONFIG = ''


def upload_files_to_ipfs(files):

    response = requests.post(f'{IPFS_END_POINT}/api/v0/add', files=files, auth=(IPFS_API_KEY, IPFS_API_SECRET))
    hash_values = response.text.split("\n")
    hash_value = hash_values[0]

    if len(hash_value) > 1:
        hash_part = hash_value.split(',')
        hash_part_without_quotes = remove_characters(hash_part[1], '""')
        hash_colon_string_index_position = find_character_position(hash_part_without_quotes, HASH_COLON)
        return get_substring(hash_part_without_quotes, hash_colon_string_index_position + len(HASH_COLON), len(hash_part_without_quotes))


def retrieve_files_from_ipfs(hash_ids):

    filenames = []

    if hash_ids is not None:

        for hash_id in hash_ids:
            params = {'arg': hash_id}
            response = requests.post(f'{IPFS_END_POINT}/api/v0/cat', params=params, auth=(IPFS_API_KEY, IPFS_API_SECRET))
            filenames.append(response.text)

    return filenames


def delete_files_from_ipfs(hash_ids):

    responses = []
    if hash_ids is not None:

        for hash_id in hash_ids:
            params = {'arg': hash_id}
            response = requests.post(IPFS_END_POINT + '/api/v0/pin/rm', params=params, auth=(IPFS_API_KEY, IPFS_API_SECRET))
            responses.append(response.json())

    return responses


if __name__ == "__main__":

    print("Uploading files...")

    nft_picture = {'fox': './assets/fox_nft.jpeg'}
    ai_picture = {'ai': './assets/ai.jpeg'}
    braincells_picture = {'brain-cells': './assets/brain_cells.jpeg'}
    deeplearning_picture = {'deep-learning': './assets/deep_learning.jpeg'}
    model_contributor_script = {'model-contributor-script': './assets/model_contributor_script.py'}
    engineer_model = {'engineer-model': './assets/engineer_model.bin'}

    nft_picture_id = upload_files_to_ipfs(nft_picture) # 'QmeLf6QXM5AZngDaaBARN3JavpKdQmc4SgV89LUd93hBbg'
    ai_picture_id = upload_files_to_ipfs(ai_picture) # 'QmafZQ5WubBn5UANPYVvv7XUfBR2kP6rtLJtmBaxYseupi'
    braincells_picture_id = upload_files_to_ipfs(braincells_picture) # 'QmNzH7YAjr4VYsVXqJLJrjdC94rs5s487R15rXfMJtc3E7'
    deeplearning_picture_id = upload_files_to_ipfs(deeplearning_picture) # 'QmWHGj31VYv1gJvNdss5QATQkHeck2EgBg85QixkU7aKFF'
    model_contributor_script_id = upload_files_to_ipfs(model_contributor_script) # 'QmeaXFDrJJdZsQo7SYMP2GBoX83Ee2sx5XNQHA5vBXP2uB'
    engineer_model_id = upload_files_to_ipfs(engineer_model) # 'QmauWpePXRSqWpvi1n9D3QA7vyZvsAvVSbVv6anAvrAahQ'

    # file = retrieve_files_from_ipfs(nft_picture_id)
    # file = retrieve_files_from_ipfs(ai_picture_id)
    # file = retrieve_files_from_ipfs(braincells_picture_id)
    # file = retrieve_files_from_ipfs(deeplearning_picture_id)
    # file = retrieve_files_from_ipfs(model_contributor_script_id)
    # file = retrieve_files_from_ipfs(engineer_model_id)

    print("Upload completed successfully.")
