{ 
  pkgs ? import (fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz) {}
}:

pkgs.stdenv.mkDerivation rec {
  name = "bpxe-shell";

  buildInputs = with pkgs; [ saxon-he rustc cargo clippy rustfmt 
                             openssl.dev pkgconfig # for cargo-release
  ];

  shellHook = ''
    # Useful for ensuring cargo tools are available (like cargo-do, for example)
    export PATH=$PATH:$HOME/.cargo/bin
  '';

}
