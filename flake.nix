{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      /*
      defaultPackage = (naersk.lib.${system}.override {
        inherit (fenix.packages.${system}.minimal) cargo rustc;
      }).buildPackage { src = ./.; };
       */
      let
        overlays = [ fenix.overlay ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        buildInputs = with pkgs; [
          #rust-analyzer-nightly
          clang
          pkg-config
          gtk3
          xorg.libX11
          xorg.libxcb
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          alsa-lib
          vulkan-loader
          wasm-bindgen-cli
          libxkbcommon
          wayland
        ];
      in with pkgs; {
        devShell = mkShell {
          buildInputs = buildInputs;
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
