#!/bin/bash
echo "Setting up environment for model creator"

python3 -m venv .venv
source .venv/bin/activate
pip install -r ./requirements.txt