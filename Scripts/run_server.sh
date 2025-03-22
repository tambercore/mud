#!/bin/bash

echo "Activating virtual environment 'lambeq_env'..."
source ../data/lambeq/lambeq_env/bin/activate

echo "Running the lambeq_server.py..."
python ../data/lambeq/lambeq_server.py

echo "Virtual environment 'lambeq_env' is active and server is running."
