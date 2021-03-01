let
  pkgs = import <nixpkgs> {};
in
pkgs.stdenv.mkDerivation rec {
  name = "bpxe-shell";

  buildInputs = with pkgs; [ saxon-he
                             rustup
                             chromedriver # for wasm-pack test --chrome
                             openssl.dev pkgconfig # for cargo-release
  ];

  shellHook = ''
    # Useful for ensuring cargo tools are available (like cargo-do, for example)
    export PATH=$PATH:$HOME/.cargo/bin
  '';

}
