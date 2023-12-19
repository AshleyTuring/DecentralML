import glob
import shutil
import os

from os.path import join

from .settings import ASSETS_FOLDER, REMOTE_FOLDER

def upload_data(role_folder, task_id):
    remote_folder = join(REMOTE_FOLDER, task_id, role_folder)
    os.mkdir(remote_folder)
    files = [f for f in glob.glob(join(ASSETS_FOLDER, role_folder)+"**")]
    for old_f in files:
        new_f = old_f.replace(ASSETS_FOLDER, f"{REMOTE_FOLDER}/{task_id}")
        shutil.copy(old_f, new_f)