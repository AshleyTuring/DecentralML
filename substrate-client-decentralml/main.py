import os
from os.path import exists, join, expanduser
import json

HOME_FOLDER = expanduser("~")
CONFIG_FOLDER = join(HOME_FOLDER, ".config/")
ROLE_FILE = join(CONFIG_FOLDER, "role.json")
ROLES = ["model_creator", "model_contributor", "data_annotator", "model_engineer"]

def configure_role() -> bool:
    print("Configure role for this node:")
    print(f"\t1\t-\tModel creator")
    print(f"\t2\t-\tModel contributor")
    print(f"\t3\t-\tData annotator")
    print(f"\t4\t-\tModel engineer")
    chosen_role = int(input("Select your role: "))
    if not exists(CONFIG_FOLDER):
        os.makedirs(CONFIG_FOLDER)
    with open(ROLE_FILE, "w+") as role_file:
        role_file.write(ROLES[chosen_role])
        return True

def main():
    if not exists(ROLE_FILE):
        configure_role()
    
    


if __name__ == "__main__":
    main()