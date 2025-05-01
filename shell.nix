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
    which

    # https://nixos.wiki/wiki/TexLive
    (texlive.combine {
      inherit (texlive)
        scheme-small
        collection-fontsrecommended
        sectsty
        titlesec;
    })
  ];

  shellHook = ''
  if [ -e "/usr/bin/security" ]; then
    export PATH="$PATH:/usr/bin"
  fi
'';
}
