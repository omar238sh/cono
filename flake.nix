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
      pkgs = import nixpkgs {
        inherit system;
        config = {
          allowUnfree = true;
        };
      };
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
          pkgs.vulkan-loader
          pkgs.mesa
          pkgs.antigravity-cli
        ];

        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.vulkan-loader ]}:$LD_LIBRARY_PATH"
          export VK_ICD_FILENAMES="${pkgs.mesa.drivers}/share/vulkan/icd.d/lvp_icd.x86_64.json"
          echo "🚀 Rust development environment ready!"
          echo "Nixpkgs: ${pkgs.lib.version}"
        '';
      };
    };
}
