#!/bin/bash

echo "Creating virtual environment 'lambeq_env'..."
python3 -m venv ../data/lambeq/lambeq_env

echo "Activating virtual environment..."
source ../data/lambeq/lambeq_env/bin/activate

echo "Upgrading pip..."
pip install --upgrade pip

echo "Installing dependencies from requirements.txt..."
pip install -r ../data/lambeq/requirements.txt

echo "Virtual environment 'lambeq_env' is ready."
