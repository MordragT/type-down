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
      in rec
      {
        packages.parol = pkgs.rustPlatform.buildRustPackage rec {
          pname = "parol";
          version = "0.22.1";

          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "4+FAmQfCk0mkz13f5mfk2674CN2n4Y/OYeRFfbv0MvI=";
          };

          cargoSha256 = "cd2Sh0Q/+h8VGRRJcX+qNl8z1l1PKxyHt+e9a10wjto=";
          doCheck = false;
        };

        packages.parol-ls = pkgs.rustPlatform.buildRustPackage rec {
          pname = "parol-ls";
          version = "0.14.0";

          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "jc/khhYOntTa79+uu33m7xDVKuNvtJTDxeKLBsyaT/Y="; # 0.14
            #sha256 = "ZAKa6JWRoLZi55rdW0t8egDbWa3JSK9+wOPDfOqrXo0="; # 0.13
          };

          buildInputs = [
            packages.parol
          ];

          cargoSha256 = "mUl+MXE0ZcmSYEx1/kpr/RzTtyop9gbjJDMUp8z7NtM="; # 0.14
          #cargoSha256 = "qbVF6lsnZyx7IraHZlXXT2jvizkP4+G96kscHUi0wq0="; # 0.13
          doCheck = false;
        };

        # Executed by `nix build`
        packages.default =
          (pkgs.makeRustPlatform {
            # Use nightly rustc and cargo provided by fenix for building
            inherit (toolchain) cargo rustc;
          })
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
            #packages.parol-ls
            pkg-config
            pandoc
            typst
          ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
        };
      }
    );
}
