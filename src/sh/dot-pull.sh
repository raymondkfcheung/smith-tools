#!/usr/bin/env bash

# If POLKADOT_SDK_DIR is not set, use a default value.
POLKADOT_SDK_DIR=${POLKADOT_SDK_DIR:-~/projects/polkadot-sdk}

# Check if the directory exists.
if [ ! -d "${POLKADOT_SDK_DIR}" ]; then
    echo "Error: The specified directory does not exist: ${POLKADOT_SDK_DIR}"
    echo "Please set the POLKADOT_SDK_DIR environment variable or create the directory."
    exit 1
fi

while true
do
    # Get the commit hash before pulling
    before_pull=$(git rev-parse HEAD)

    echo "Pulling changes from the remote repository..."
    git pull

    # Get the new commit hash after pulling
    after_pull=$(git rev-parse HEAD)

    if [[ "$before_pull" != "$after_pull" ]]; then
        echo "New changes were pulled. Stopping the script."
        exit 0
    fi

    echo "No new changes. Waiting for 3 minutes before the next pull..."
    sleep 180
done
