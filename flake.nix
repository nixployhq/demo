{
  description = "A Topcoat app for trying Nixploy";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: {
    packages = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "nixploy-demo";
          version = "0.1.0";
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          DEMO_REVISION = self.rev or "local-checkout";
          meta.mainProgram = "nixploy-demo";
          meta.license = pkgs.lib.licenses.mit;
        };
      }
    );
  };
}
