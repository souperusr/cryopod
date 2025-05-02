{ pkgs, ... }:

{
    config.perSystem = { pkgs, ... }:
    let
        # --- Container image --------------------
        image = pkgs.dockerTools.buildImage {
            name = "cryopod";
            tag  = "DEV";
            compressor = "none";
            copyToRoot = pkgs.buildEnv {
                name  = "image-root";
                paths = with pkgs; [ coreutils nix bashInteractive ];
            };
        };

        # --- Image digest -------------
        digest = pkgs.runCommand "cryopod-image-digest" {
            nativeBuildInputs = with pkgs; [ jq skopeo ];
            outputFile = true;
        } ''
            mkdir -p /var/tmp
            skopeo inspect docker-archive:${image} | jq -r .Digest > "$out"
        '';
    in
    {
        packages.image       = image;
        packages.imageDigest = digest;
    };
}
