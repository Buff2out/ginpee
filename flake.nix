{
  description = "Rust Project with rust-analyzer, clippy, rustfmt support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      buildInputs = with pkgs; [
        rustc
        cargo
        rustfmt
        clippy
        rust-analyzer
        lldb
        cargo-expand
        cargo-deny
        cargo-outdated
        # Дополнительно, если нужно:
        # cargo-audit
        # cargo-edit
        # cargo-watch
      ];
    in {
      devShells.default = pkgs.mkShell {
        inherit buildInputs;

        shellHook = ''
          export PATH="$HOME/.cargo/bin:$PATH"
          echo "✅ Rust Project Dev Environment Loaded"
          echo "🔧 Available tools:"
          echo "   - rustc, cargo, rustfmt, clippy"
          echo "   - rust-analyzer, lldb, cargo-expand"
          echo "🚀 Run 'cargo run' or 'cargo test' to start"
        '';
      };
    });
}
