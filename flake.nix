{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      perSystem =
        {
          pkgs,
          system,
          ...
        }:
        let
          # Fenix toolchain setup
          fenixPkgs = inputs.fenix.packages.${system};

          # Rust toolchain with components and WASM target
          toolchain = fenixPkgs.combine [
            fenixPkgs.stable.toolchain
            fenixPkgs.targets.wasm32-unknown-unknown.stable.rust-std
          ];

          buildInputs = with pkgs; [
            toolchain
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
            wasm-bindgen-cli
            lld_20
            tailwindcss_4
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            nodejs_latest
          ];
        in
        {
          # Rust dev environment
          devShells.default = pkgs.mkShell {
            RUST_BACKTRACE = "full";
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
            inherit nativeBuildInputs;
            inherit buildInputs;
            packages = [
              pkgs.watchman
              pkgs.rustywind
              pkgs.dioxus-cli
            ];
          };

        };
    };
}
