{ 
  pkgs ? import (fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz) {}
}:

pkgs.stdenv.mkDerivation rec {
  name = "bpxe-shell";

  buildInputs = with pkgs; [ saxon-he ];

}
