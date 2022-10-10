# For compatibility with pre-flake nix setups. Refer to the package section in `flake.nix`
(import (fetchTarball https://github.com/edolstra/flake-compat/archive/master.tar.gz) {
  src = builtins.fetchGit ./.;
}).defaultNix
