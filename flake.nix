{
  description = "Rust development template using fenix";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    fenix,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        };
        toolchain = pkgs.fenix.complete;
        platform = pkgs.makeRustPlatform {
          # Use nightly rustc and cargo provided by fenix for building
          inherit (toolchain) cargo rustc;
        };
      in rec
      {
        packages.parol = platform.buildRustPackage rec {
          pname = "parol";
          version = "3.0.1";

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-CSPlFplnlVqm3vCE3a2k01z+MuqtNLa/eml0f8exa9U=";
          };

          useFetchCargoVendor = true;
          cargoHash = "sha256-xMV3VmvxYLehBuedLAYSHvOWuP4bOENVLtYZpsAWe0c=";
          doCheck = false;
        };

        packages.parol-ls = platform.buildRustPackage rec {
          pname = "parol-ls";
          version = "3.0.1";

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-C9JADDcTM/8b5K14wL5StXYqeN12KACIv5FQvFdSFy8=";
          };

          nativeBuildInputs = [
            packages.parol
          ];

          useFetchCargoVendor = true;
          cargoHash = "sha256-NDrSdr1+EIZVi8cj8Lyj6WiWg4ZehdwanumLwdlcYLo=";
          doCheck = false;

          meta.broken = true;
        };

        # Executed by `nix build`
        packages.default =
          platform
          .buildRustPackage {
            pname = "template";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            # For other makeRustPlatform features see:
            # https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md#cargo-features-cargo-features
          };

        # Executed by `nix run`
        apps.default = utils.lib.mkApp {drv = packages.default;};

        # Used by `nix develop`
        devShells.default = pkgs.mkShell {
          # Use nightly cargo & rustc provided by fenix. Add for packages for the dev shell here
          buildInputs = with pkgs; [
            (with toolchain; [
              cargo
              rustc
              rust-src
              clippy
              rustfmt
            ])
            cargo-flamegraph
            packages.parol
            # packages.parol-ls
            pkg-config
            pandoc
            typst
            nodejs
          ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
        };
      }
    );
}
