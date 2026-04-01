{
  description = "meteorite -- UI component library for Dioxus";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    ratcore = {
      url = "github:brittonr/ratcore";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, rust-overlay, flake-utils, ratcore, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Combine meteorite source with ratcore (path dep at ../ratcore)
        rawSrc = pkgs.lib.fileset.toSource {
          root = ./.;
          fileset = pkgs.lib.fileset.unions [
            ./Cargo.toml
            ./Cargo.lock
            ./rust-toolchain.toml
            (pkgs.lib.fileset.fileFilter (f: f.hasExt "rs" || f.hasExt "toml" || f.hasExt "css") ./crates)
          ];
        };

        src = pkgs.runCommand "meteorite-src" {} ''
          cp -r ${rawSrc} $out
          chmod -R u+w $out
          cp -r ${ratcore} $out/ratcore
          # Rewrite path dep from ../ratcore to ./ratcore for sandbox
          sed -i 's|path = "../ratcore"|path = "ratcore"|' $out/Cargo.toml
        '';

        nativeBuildInputs = with pkgs; [
          pkg-config
          clang
          mold
        ];

        buildInputs = with pkgs; [
          openssl
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src nativeBuildInputs buildInputs;
        };

        meteorite = craneLib.buildPackage {
          inherit src cargoArtifacts nativeBuildInputs buildInputs;
        };
      in
      {
        packages = {
          default = meteorite;
          meteorite = meteorite;
        };

        checks = {
          inherit meteorite;

          nextest = craneLib.cargoNextest {
            inherit src cargoArtifacts nativeBuildInputs buildInputs;
            partitions = 1;
            partitionType = "count";
          };

          clippy = craneLib.cargoClippy {
            inherit src cargoArtifacts nativeBuildInputs buildInputs;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          };

          fmt = craneLib.cargoFmt {
            inherit src;
          };
        };

        devShells.default = craneLib.devShell {
          inherit buildInputs;

          packages = with pkgs; [
            cargo-nextest
            cargo-watch
            rust-analyzer
            dioxus-cli
            wasm-bindgen-cli
          ];

          inputsFrom = [ meteorite ];
        };
      }
    );
}
