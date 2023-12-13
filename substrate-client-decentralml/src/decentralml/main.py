import os
from os.path import exists, join, expanduser
import json
from .create_task import create_task_data_annotator, create_task_model_contributor, create_task_model_engineer

HOME_FOLDER = expanduser("~")
CONFIG_FOLDER = join(HOME_FOLDER, ".config/")
ROLE_FILE = join(CONFIG_FOLDER, "role.json")
ROLES = ["model_creator", "model_contributor", "data_annotator", "model_engineer"]
CURRENT_ROLE = None

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

def run_as_model_creator():
    choice = -1
    while choice != 5:
        print("Choose your command")
        print(f"\t1\t-\tCreate task")
        print(f"\t2\t-\tList tasks")
        print(f"\t3\t-\tList tasks results")
        print(f"\t4\t-\tAccept task results")
        print(f"\t4\t-\tReject task results")
        print(f"\t5\t-\tExit")
        try:
            choice = int(input(choice))
            match choice:
                case 1:
                    subchoice = -1
                    while subchoice != 4:
                        print("What kind of task you want to create")
                        print(f"\t1\t-\tData annotator")
                        print(f"\t2\t-\tModel contributor")
                        print(f"\t3\t-\tModel engineer")
                        print(f"\t4\t-\tCancel")
                        try:
                            subchoice = int(input(choice))
                        except ValueError as e:
                            print("Choice must be one value in the interval [1-4]")
                            subchoice = -1
                        match subchoice:
                            case 1:
                                create_task_data_annotator()
                                subchoice = 4
                            case 2:
                                create_task_model_contributor()
                                subchoice = 4
                            case 3:
                                create_task_model_engineer()
                                subchoice = 4
                            case _:
                                print("Choice must be one value in the interval [1-4]")
                                subchoice = -1
                case 2:
                    pass
        except ValueError as e:
            print("Choice must be one value in the interval [1-5]")
            choice = -1

def run_as_model_contributor():
    pass

def run_as_data_annotator():
    pass

def run_as_model_engineer():
    pass

def main():
    if not exists(ROLE_FILE):
        configure_role()
    with open(ROLE_FILE, 'r') as role_file:
        CURRENT_ROLE = role_file.readline()
    match CURRENT_ROLE:
        case "model_creator":
            pass
        case "model_contributor":
            pass
        case "data_annotator":
            pass
        case "model_engineer":
            pass

if __name__ == "__main__":
    main()