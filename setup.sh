#!/bin/bash

# Exit on error
set -e

# Create a virtual environment named 'venv'
python3.9 -m venv venv

# Activate the virtual environment
source venv/bin/activate

# Install required packages from requirements.txt
pip install -r requirements.txt

# Deactivate the virtual environment
deactivate

echo "Setup complete. You can activate the virtual environment by running 'source venv/bin/activate'."
