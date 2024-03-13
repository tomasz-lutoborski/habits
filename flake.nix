{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          shellHook = ''
            export DATABASE_URL="sqlite:todos.db"
          '';
          packages = with pkgs; [
            pkg-config
            openssl
            sqlite
          ];
          nativeBuildInputs =
            [
              fenix.packages.${system}.complete.toolchain
            ];
        };
      });
}
