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

}
