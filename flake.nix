{
  inputs = {
    nixpkgs.url = "github:NickCao/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlay ];
          };
        in
        rec {
          packages = rec {
            ebpf-verifier = pkgs.stdenv.mkDerivation {
              name = "ebpf-verifier";
              src = pkgs.fetchFromGitHub {
                owner = "vbpf";
                repo = "ebpf-verifier";
                rev = "396f5923ad5a8b751b255e7cc4dae672c2cf54d4";
                fetchSubmodules = true;
                sha256 = "sha256-QQQixVdfi+0KUvrxFwbpyk2JqtDMH4UGHuC7qiWygdU=";
              };
              nativeBuildInputs = with pkgs; [ cmake ];
              buildInputs = with pkgs; [ libyamlcpp boost ];
              patches = [ ./src/binding/ebpf-verifier.patch ];
            };
          };
          devShell = pkgs.mkShell {
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            buildInputs = with pkgs; [
              (rust-bin.selectLatestNightlyWith (toolchain:
                toolchain.default.override {
                  extensions = [ "rust-analyzer-preview" ];
                }))
              linuxHeaders
              packages.ebpf-verifier
              boost
              rust-bindgen
            ];
          };
        }
      );
}
