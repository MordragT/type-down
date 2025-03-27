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
        toolchain = with pkgs.fenix;
          combine [
            latest.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
            targets.wasm32-wasip1.latest.rust-std
            targets.wasm32-wasip2.latest.rust-std
          ];

        platform = pkgs.makeRustPlatform {
          # Use nightly rustc and cargo provided by fenix for building
          inherit (toolchain) cargo rustc;
        };
      in {
        # Executed by `nix build`
        packages.default = self.packages."${system}".type-down;

        packages.type-down-language-server = platform.buildRustPackage {
          pname = "type-down-language-server";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = ["-p tyd-language-server"];
        };

        packages.type-down =
          platform
          .buildRustPackage {
            pname = "type-down";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            # For other makeRustPlatform features see:
            # https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md#cargo-features-cargo-features
          };

        # Executed by `nix run`
        apps.default = utils.lib.mkApp {drv = self.packages."${system}".default;};

        # Used by `nix develop`
        devShells.default = pkgs.mkShell {
          # Use nightly cargo & rustc provided by fenix. Add for packages for the dev shell here
          buildInputs = with pkgs; [
            toolchain
            lld
            cargo-flamegraph
            pkg-config
            pandoc
            typst
            nodejs
            pnpm
            wasm-tools
            # tree-sitter
            # self.packages."${system}".type-down-language-server
          ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${pkgs.fenix.complete.rust-src}/lib/rustlib/src/rust/library";
          # TODO provide a better way to define this
          TYPE_DOWN_LANGUAGE_SERVER = "/home/tom/Desktop/Mordrag/type-down/target/debug/tyd-language-server";
        };
      }
    );
}
