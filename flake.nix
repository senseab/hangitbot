{
  nixConfig = rec {
    experimental-features = [ "nix-command" "flakes" ];

    substituters = [
      # Replace official cache with a mirror located in China
      #
      # Feel free to remove this line if you are not in China
      "https://mirrors.ustc.edu.cn/nix-channels/store"
      "https://mirrors.ustc.edu.cn/nix-channels/store" # 中科大
      "https://mirrors.tuna.tsinghua.edu.cn/nix-channels/store" #清华
      "https://mirrors.bfsu.edu.cn/nix-channels/store" # 北外
      "https://mirror.sjtu.edu.cn/nix-channels/store" #交大
      #"https://cache.nixos.org"
    ];
    trusted-substituters = substituters;
    trusted-users =  [
      "coder"
    ];
  };

  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          buildInputs = with pkgs; [
            pkg-config
            openssl
          ];
        };
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy 
            pkg-config
            openssl
            gcc
            sqlite
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
