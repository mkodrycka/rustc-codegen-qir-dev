# Rust QIR codegen

This project aims to create a valid rustc backend target for [quantum intermediate representation (QIR)](https://github.com/qir-alliance/qir-spec),
which is a subset of LLVM's IR tailored for quantum applications.

Custom rustc codegen crates require hooking into the unstable `rustc_private` feature,
thus requiring use of the latest nightly compiler.

## Building

TODO

## Developing

This project makes use of the [Nix package manager](https://nixos.org/) for ensuring deterministic builds
across multiple platforms. The top-level [flake.nix](flake.nix) defines a set of development shells for
quick switching between the stable and nightly versions of the rust toolchain, as described in the top-level
[rust-toolchain.toml](rust-toolchain.toml).

The following commands show how to drop into a development shell using either the newer [nix develop command](https://nixos.org/manual/nix/unstable/command-ref/new-cli/nix3-develop.html)
or older [nix-shell](https://nixos.org/manual/nix/unstable/command-ref/nix-shell.html) command.

__Note: Use of the `nix develop` command requires enabling both the `nix-command` and `flakes` experimental features.__

```bash
# Rust nightly
$ nix develop # or nix-shell

# Rust stable
$ nix develop ".#stable" # or nix-shell -A devShells.x86_64-linux.stable
```
