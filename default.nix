let

  fetchNixpkgs = {rev, sha256}: builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs-channels/archive/${rev}.tar.gz";
    inherit sha256;
  };

  # nixpkgs-unstable of 17.05.2020
  pkgs = import (fetchNixpkgs {
    rev = "b47873026c7e356a340d0e1de7789d4e8428ac66";
    sha256 = "0wlhlmghfdvqqw2k7nyiiz4p9762aqbb2q88p6sswmlv499x5hb3";
  }) {};

in

pkgs.mkShell {

  buildInputs = (with pkgs; [
    
    rustup cargo rust-analyzer
  ]);

}

