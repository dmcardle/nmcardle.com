#!/usr/bin/env bash

set -ex

DOC_ZIP="games/mercator/mercator_doc.zip"
DOC_DIR="extracted"

rm -rf "$DOC_DIR"
unzip -d "$DOC_DIR" "$DOC_ZIP"
xdg-open "${DOC_DIR}/games/mercator/mercator_doc.rustdoc/mercator_lib/index.html"
