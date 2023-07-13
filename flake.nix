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
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.stable.rustc
          fenix.packages.${system}.stable.cargo
          fenix.packages.${system}.targets.wasm32-unknown-unknown.stable.rust-std
        ];
        buildInputs = with pkgs; [
          # rust toolchain
          toolchain
          # rust native lib
          pkg-config
          openssl
          # utility
          just
          binaryen
          # bevy
          gtk3
          xorg.libX11
          xorg.libxcb
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          alsa-lib
          vulkan-loader
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
