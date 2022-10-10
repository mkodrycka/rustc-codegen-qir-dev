# For compatibility with pre-flake nix setups. Refer to the devShell section in `flake.nix`
(import (fetchTarball https://github.com/edolstra/flake-compat/archive/master.tar.gz) {
  src = builtins.fetchGit ./.;
}).shellNix
