{
  description = "QIR codegen crate for Rustc";

  inputs = {
    # Use upstream nixpkgs in order to be deterministic
    nixpkgs.url = "github:NixOS/nixpkgs";

    # Flake utils allows for reducing boilerplate when targetting multiple platforms
    # Note: This library does not make any use of nixpkgs, so we don't have to
    #  specify that it follows our version of nixpkgs.
    utils.url = "github:numtide/flake-utils";

    # Fenix contains derivations for various versions of the rust ecosystem
    fenix = {
      url = "github:nix-community/fenix";

      # Make sure to sync the nixpkgs version with ours
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, fenix }: utils.lib.eachDefaultSystem (system:
    let
      # Load the pkgs for the current system configuration, making sure to overlay
      #  derivations made available by fenix.
      pkgs = import nixpkgs {
        inherit system;

        overlays = [ fenix.overlay ];
      };

      # Instead of manually specifying the rust toolchain, we delegate to a standard
      #  `rust-toolchain.toml` file, whose hash is also calculated and declared for
      #  deterministic builds.
      rust-toolchain = pkgs.fenix.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "pKSkfGypl51d78GwXg6pFJeBxSQCQu7ZzTruYCvSkPA=";
      };

      # Cargo tries to update the crate index located in the read-only nix store
      # when used by rust analyzer to inspect rustc_* crates. Until that issue is
      # resolved, we patch the toolchain to include the missing lock file generated
      # manually.
      #
      # See the following links for more info:
      # - https://github.com/rust-lang/cargo/issues/10096
      # - https://github.com/rust-lang/rust-analyzer/issues/13393
      # - https://github.com/rust-lang/rust/issues/95736
      rustc-src-root = "lib/rustlib/rustc-src/rust/compiler/rustc";
      rust-toolchain-lockfile = pkgs.runCommand "Cargo.lock" {
        nativeBuildInputs = with pkgs; [ cargo ];
      } ''
        mkdir "$out"

        # Cargo does not allow us to generate a lockfile separately from the Cargo.toml location,
        # so we need to copy the files over and run it locally :'(
        export ROOT="$TMP/cargo-copy"

        cp -r "${rust-toolchain}/" "$ROOT/"
        chmod +w -R "$ROOT/${rustc-src-root}"
        cd "$ROOT/${rustc-src-root}"

        # Generate the lockfile
        CARGO_HOME=$TMP/.cargo cargo generate-lockfile

        # Write out the lockfile
        cp Cargo.lock "$out/Cargo.lock"
      '';

      # Note: Overriding the attributes causes nix to recompile the entire
      # toolchain, but using `symlinkJoin` requires wrapping rustc to also point
      # to the symlinked package, which is somewhat hacky.
      rust-toolchain-patched = rust-toolchain.overrideAttrs (old: {
        buildCommand = old.buildCommand + ''
          ln -s "${rust-toolchain-lockfile}/Cargo.lock" $out/${rustc-src-root}/Cargo.lock
        '';
      });

      # Pin to a specific version of LLVM that works with QIR.
      # TODO: Only LLVM v11 works on darwin aarch64, so it might be good to later
      #  have this version depend on the build host.
      #  https://github.com/NixOS/nixpkgs/issues/166205
      llvm-compat = pkgs.llvmPackages_11;
      mkClangShell = pkgs.mkShell.override { stdenv = llvm-compat.stdenv; };

    in {
      # Expose a number of development shells to be able to quickly switch between latest stable
      #  and nightly envs.
      #
      # Usage:
      # - nix develop          # For nightly
      # - nix develop .#stable # For stable
      #
      # TODO: We might want to pin to a version of LLVM as well in each dev shell.
      devShells =
        let
          # Specify the list of common dependencies used by both stable and nightly rust.
          commonsDeps = with pkgs; [libiconv libxml2 llvm-compat.llvm];

          # Set up the needed env vars for each session
          # TODO: The LLVM prefix is hardcoded to the version we selected earlier, but shouldn't be.
          mkPrompt = toolchain: ''
            # Specify a prompt prefix for easily seeing which rust toolchain we are currently using
            export PS1="[rust ${toolchain}] $PS1"

            # Specify the prefix to the installed LLVM so that inkwell / llvm-sys can find it at compile time.
            export LLVM_110_PREFIX="${llvm-compat.llvm}";

            # Always show debug message by default
            export RUST_LOG=DEBUG

            # Quick access to the root dir for this project
            ROOT_PATH=$(git rev-parse --show-toplevel)

            # Utility function for quickly testing if the bell example compiles correctly. Must be called from within
            #  the project directory.
            function compile_bell {
              pushd "$ROOT_PATH"

              # Build the codegen backend
              cd "$ROOT_PATH/"
              cargo build

              # Build the bell example
              cd "$ROOT_PATH/examples/bell"
              cargo build

              # Move back to old dir
              popd
            }
          '';

        in rec {
          default = nightly;

          stable = mkClangShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rust-analyzer
            ] ++ commonsDeps;

            shellHook = mkPrompt "stable";
          };

          nightly = mkClangShell {
            buildInputs = with pkgs; [
              rust-toolchain-patched
              rust-analyzer-nightly
            ] ++ commonsDeps;

            shellHook = mkPrompt "nightly";
          };
        };
    }
  );
}
