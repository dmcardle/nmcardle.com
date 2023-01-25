#!/usr/bin/env sh

set -e

echo "::Building everything with Bazel..."
bazelisk build //...

PUBLIC_CV_DIR=public/gen/cv/
echo "::Copying generated //cv files to public/"
rm -rf "${PUBLIC_CV_DIR}"
mkdir -p "${PUBLIC_CV_DIR}"
bazelisk cquery --output=files //cv | tee | xargs cp -t "${PUBLIC_CV_DIR}"

echo "::Run [firebase deploy] when you're ready."
