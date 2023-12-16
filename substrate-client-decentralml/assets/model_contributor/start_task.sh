#!/bin/bash
echo "Setting up environment"

python3 -m venv .venv
source .venv/bin/activate
pip install -r ./requirements.txt

echo "Executing task script"
python3 -m model_contributor



