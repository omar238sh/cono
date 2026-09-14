{
  description = "Rust Development Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs = {  nixpkgs, fenix,... }:
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
          pkgs.ffmpeg_8
          pkgs.clang
          pkgs.libclang
          pkgs.gdb
          pkgs.lua-language-server
        ];
        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
            pkgs.vulkan-loader
            pkgs.ffmpeg_8
          ]}:$LD_LIBRARY_PATH"

          export VK_ICD_FILENAMES="/run/opengl-driver/share/vulkan/icd.d/nvidia_icd.json"
          export VK_LAYER_PATH="/run/opengl-driver/share/vulkan/explicit_layer.d"
  
          # export VK_ICD_FILENAMES="${pkgs.mesa}/share/vulkan/icd.d/lvp_icd.x86_64.json"
          export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
  
          echo "🚀 Rust development environment ready!"
          echo "Nixpkgs: ${pkgs.lib.version}"
        '';
        
        };
    };
}
