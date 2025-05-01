#!/usr/bin/env nix-shell
#!nix-shell -i bash --pure shell.nix

set -e

if [ -e public/ ]; then
    if [ "$1" != "--destructive" ]; then
        echo "Error: public/ dir already exists. Pass --destructive to" >&2
        echo "enable this script to delete public/." >&2
        exit 1 # Exit with a non-zero status code to indicate failure
    fi
    rm -rf public/
fi

echo "::Building release tarfile."
TAR_FILE="$(bazelisk cquery --output=files //:release)"
bazelisk build //:release

echo
echo "::Extracting tarfile."
tar -xvf "$TAR_FILE"

# This works around a longstanding issue I've had with `firebase deploy` where
# it succeeds without updating the hosted files. I suspect it's looking at file
# timestamps instead of comparing their contents.
echo
echo "::Touching all files."
find public/ -exec touch {} \+

echo
echo "::Run [firebase deploy] when you're ready."
