# My personal homepage

This repo is built with Bazel.

## Prerequisites

Assuming Debian 11.

```sh
sudo apt install texlive texlive-latex-extra cm-super
go get github.com/bazelbuild/bazelisk
go get github.com/bazelbuild/buildtools/buildifier
```

## To build

```sh
bazelisk build //...  # Builds files in bazel output directories.
```

## To format BUILD files

```sh
buildifier -r .
```

## To deploy the site

```sh
./release.sh          # Copies selected output files to public/.
firebase deploy       # Uploads public/ to cloud hosting, sets up routes, etc.
```

## Tips and tricks

### List files generated by Bazel target

``` sh
bazelisk cquery --output=files //the/target
```
