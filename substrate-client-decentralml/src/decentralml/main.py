from os.path import join, expanduser

from .settings import DECENTRALML_ROLE
from .roles import ModelCreator, ModelContributor, ModelEngineer, DataAnnotator

HOME_FOLDER = expanduser("~")
CONFIG_FOLDER = join(HOME_FOLDER, ".config/")
ROLE_FILE = join(CONFIG_FOLDER, "role.json")
ROLES = ["model_creator", "model_contributor", "data_annotator", "model_engineer"]
CURRENT_ROLE = None

def main():
    if DECENTRALML_ROLE:
        match DECENTRALML_ROLE:
            case "model_creator":
                ModelCreator.menu()
            case "model_contributor":
                ModelContributor.menu()
            case "model_engineer":
                ModelEngineer.menu()
            case "data_annotators":
                DataAnnotator.menu()

if __name__ == "__main__":
    main()