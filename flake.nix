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
              rust-toolchain
              rust-analyzer-nightly
            ] ++ commonsDeps;

            shellHook = mkPrompt "nightly";
          };
        };
    }
  );
}
