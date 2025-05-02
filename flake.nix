{
  description = "An isolated Nix-based development environment";

  inputs = {
    nixpkgs     .url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts .url = "github:hercules-ci/flake-parts";
    naersk      .url = "github:nix-community/naersk";
  };

  outputs = inputs @ { flake-parts, ... }:
  flake-parts.lib.mkFlake { inherit inputs; } {
    imports = [ ./modules/container.nix ];

    systems = [ "x86_64-linux" "aarch64-linux" ];

    perSystem = { pkgs, config, ... }: {
      # --- Rust package --------------------------------------------------
      packages.default = (pkgs.callPackage inputs.naersk {}).buildPackage {
        src       = ./.;
        RUSTFLAGS = "-C opt-level=3";

        # make the digest text available to build.rs via env
        env = {
          CRYOPOD_IMAGE        = "${config.packages.image}";
          CRYOPOD_IMAGE_DIGEST = "${builtins.readFile config.packages.imageDigest}";
        };
      };

      # --- Formatter -----------------------------------------------------
      formatter = pkgs.rustfmt;

      # --- Development shell --------------------------------------------
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
        ];

        shellHook = ''
          export CRYOPOD_IMAGE=${config.packages.image}
          export CRYOPOD_IMAGE_DIGEST=$(cat ${config.packages.imageDigest})
        '';
      };
    };
  };
}
