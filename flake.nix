{
  description = "github:moeru-ai/plast-mem";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          # https://github.com/NixOS/nixpkgs/blob/nixos-unstable/pkgs/by-name/se/sea-orm-cli/package.nix
          sea-orm-cli = pkgs.rustPlatform.buildRustPackage rec {
            pname = "sea-orm-cli";
            version = "2.0.0-rc.30";

            src = pkgs.fetchCrate {
              inherit pname version;
              hash = "sha256-rG7BygsEEbmHcM3t9FlzNUsX7tfoyZkmCtBB4wakZso=";
            };

            cargoHash = "sha256-uPcuzd3QOHTN3N+p+X2oWLDqyVhmxWqDsCiFxA9UorU=";

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [ openssl ];

            meta = {
              mainProgram = pname;
              license = with pkgs.lib.licenses; [
                mit # or
                asl20
              ];
              maintainers = with pkgs.lib.maintainers; [ kwaa ];
            };
          };
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs =
              (with pkgs; [
                # rust
                rustc
                cargo
                rustfmt
                clippy
                rust-analyzer

                # misc
                bacon # `cargo-watch` alternative
              ])
              ++ [ sea-orm-cli ];

            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };

          packages = forAllSystems (
            system:
            let
              pkgs = import nixpkgs { inherit system; };
              version = (fromTOML (builtins.readFile ./Cargo.toml)).package.version;
            in
            {
              default = pkgs.rustPlatform.buildRustPackage {
                inherit version;
                pname = "plast-mem";
                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;
              };
            }
          );
        }
      );
    };
}
