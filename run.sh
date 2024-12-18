#!/usr/bin/env bash

SCRIPT_DIR=$(dirname "$0")

VENV_DIR="$SCRIPT_DIR/venv"

if [ ! -d "$VENV_DIR" ]; then
    echo "Virtual environment not found. Creating it..."
    python -m venv "$VENV_DIR"
    if [ $? -eq 0 ]; then
        echo "Virtual environment created successfully."
    else
        echo "Failed to create virtual environment."
        exit 1
    fi
else
    echo "Virtual environment already exists."
fi

echo "Activating virtual environment..."
source "$VENV_DIR/bin/activate"

REQUIREMENTS_FILE="$SCRIPT_DIR/requirements.txt"
if [ -f "$REQUIREMENTS_FILE" ]; then
    echo "Installing Python dependencies from requirements.txt..."
    pip install -r "$REQUIREMENTS_FILE"
else
    echo "Warning: requirements.txt not found, skipping dependency installation."
fi

PYTHON_SCRIPT="$SCRIPT_DIR/src/main.py"
if [ ! -f "$PYTHON_SCRIPT" ]; then
    echo "Error: $PYTHON_SCRIPT not found!"
    exit 1
fi

python "$PYTHON_SCRIPT"

EXIT_CODE=$?

if [ $EXIT_CODE -eq 2 ]; then
    echo "Python script failed. Running ydotool..."
    ydotool click 0xC0
fi

