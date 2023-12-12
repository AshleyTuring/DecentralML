import os

from .settings import ASSETS_FOLDER

def remove_characters(input_string, characters_to_remove):
    for char in characters_to_remove:
        input_string = input_string.replace(char, '')
    return input_string


def find_character_position(input_string, target_character):
    try:
        index = input_string.index(target_character)
        return index
    except ValueError:
        # If the character is not found in the string
        return -1


def remove_spaces(input_string):
    # Use str.replace() to replace spaces with an empty string
    result = input_string.replace(" ", "")
    return result


def get_substring(input_string, start_index, end_index):
    # Use slicing to extract the substring
    substring = input_string[start_index:end_index]
    return substring

def get_files_from_folder(folder_path):

    files = os.listdir(folder_path)
    file_paths = []

    for file in files:
        file_paths.append(os.path.join(folder_path, file))

    return file_paths

# Helper Functions
def get_validation_strategy_dict(strategy):
    return {strategy: {
        'AutoAccept': 0,
        'ManualAccept': 1,
        'CustomAccept': 2,
    }.get(strategy, 0)}

def get_task_type_dict(task_type):
    return {task_type: {
        'DataAnnotators': 0,
        'ModelContributor': 1,
        'ModelEngineer': 2,
        'Client': 3,
    }.get(task_type, 1)}  # Default to ModelContributor

def get_storage_type_dict(storage_type):
    return {storage_type: {
        'IPFS': 0,
        'Crust': 1,
        'S3': 2,
        'GCP': 3,
        'Azure': 4,
    }.get(storage_type, {'IPFS': 0})}  # Default to IPFS

def get_annotation_type_dict(annotation_type):
    return {annotation_type: {
        'Image': 0,
        'Audio': 1,
        'Text': 2,
        'Video': 3,
    }.get(annotation_type, 0)}  # Default to Image

def get_status_dict(strategy):
    return {strategy: {
        'Assigned': 0,
        'PendingValidation': 1,
        'Validated': 2,
        'Accepted': 2,
        'Rejected': 3,
    }.get(strategy, 0)}
