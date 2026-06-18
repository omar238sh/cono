{
  description = "Rust Development Environment";

  inputs = {
    nixpkgs.url = "path:/nix/store/pfwrb65dsv8phlsf1m98bvz11cvgb290-source";
    
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      rustToolchain = fenix.packages.${system}.combine [
        
        fenix.packages.${system}.stable.cargo
        fenix.packages.${system}.stable.rustc
        fenix.packages.${system}.stable.rust-src
        
        
        fenix.packages.${system}.stable.rust-analyzer
        fenix.packages.${system}.stable.clippy
        fenix.packages.${system}.stable.rustfmt

      ];
      
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "rust-dev";

        buildInputs = [
          rustToolchain
          pkgs.pkg-config
          pkgs.openssl
        ];

        shellHook = ''
          echo "🚀 Rust development environment ready!"
          echo "Nixpkgs: ${pkgs.lib.version}"
        '';
      };
    };
}
