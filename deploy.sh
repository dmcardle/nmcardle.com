#!/usr/bin/env sh

set -ex

make -C resume
cp resume/cv.html public/
firebase deploy
