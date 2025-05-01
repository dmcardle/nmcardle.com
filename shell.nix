{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    bazel-buildtools # Provides buildifier and friends
    bazelisk
    firebase-tools
    git
    libtool
    pandoc
    python3
    texliveFull
    which
  ];

  shellHook = ''
  if [ -e "/usr/bin/security" ]; then
    export PATH="$PATH:/usr/bin"
  fi
'';
}
