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
