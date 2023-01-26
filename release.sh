#!/usr/bin/env sh

set -e

if [ -e public/ ]; then
    if [ -z "${NMCARDLE_DESTRUCTIVE}" ]; then
        echo "Error: public/ dir already exists. Define NMCARDLE_DESTRUCTIVE env"
        echo "var to enable this script to delete public/."
        exit 1
    fi
    rm -rf public/
fi

echo "::Building release tarfile."
TAR_FILE="$(bazelisk cquery --output=files //:release)"
bazelisk build //:release

echo
echo "::Extracting tarfile."
tar -xvf "$TAR_FILE"

echo
echo "::Run [firebase deploy] when you're ready."
