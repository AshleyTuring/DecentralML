import glob
import shutil
import os

from os.path import join

from .settings import ASSETS_FOLDER, REMOTE_FOLDER

def upload_data_to_remote(role_folder, task_id):
    remote_folder = join(REMOTE_FOLDER, str(task_id), role_folder)
    os.makedirs(remote_folder, exist_ok=True)
    files = [f for f in glob.glob(join(ASSETS_FOLDER, role_folder)+"/**")]
    for local_f in files:
        remote_f = local_f.replace(ASSETS_FOLDER, f"{REMOTE_FOLDER}/{task_id}/")
        shutil.copy(local_f, remote_f)

def download_data_from_remote(role_folder, task_id):
    local_folder = join(ASSETS_FOLDER, str(task_id), role_folder)
    os.makedirs(local_folder, exist_ok=True)
    files = [f for f in glob.glob(join(REMOTE_FOLDER, str(task_id), role_folder)+"/**")]
    for remote_f in files:
        local_f = remote_f.replace(REMOTE_FOLDER, ASSETS_FOLDER)
        shutil.copy(remote_f, local_f)

def upload_results_to_remote(role_folder, task_id):
    remote_folder = join(REMOTE_FOLDER, str(task_id), role_folder)
    os.makedirs(remote_folder, exist_ok=True)
    files = [f for f in glob.glob(join(ASSETS_FOLDER, task_id, role_folder)+"/**")]
    for local_f in files:
        remote_f = local_f.replace(ASSETS_FOLDER, f"{REMOTE_FOLDER}/{task_id}/")
        shutil.copy(local_f, remote_f)

def download_results_from_remote(role_folder, task_id):
    local_folder = join(ASSETS_FOLDER, str(task_id), role_folder)
    os.makedirs(local_folder, exist_ok=True)
    files = [f for f in glob.glob(join(REMOTE_FOLDER, str(task_id), role_folder)+"/**")]
    for remote_f in files:
        local_f = remote_f.replace(REMOTE_FOLDER, ASSETS_FOLDER)
        shutil.copy(remote_f, local_f)