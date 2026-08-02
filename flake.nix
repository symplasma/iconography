{
  description = "Icon Viewer - a Rust/egui application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Use the toolchain pinned in rust-toolchain.toml
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Runtime libraries needed by eframe/egui (GUI/Wayland/X11/Vulkan)
        libPath = with pkgs;
          lib.makeLibraryPath [
            libglvnd
            libxkbcommon
            wayland
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

        # Matches [package] name / [[bin]] name in Cargo.toml
        binName = "iconography";

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          buildInputs = with pkgs; [
            pkg-config
            fontconfig
            freetype
          ] ++ [
            libglvnd
            libxkbcommon
            wayland
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

          nativeBuildInputs = with pkgs; [
            clang
            llvmPackages.bintools
            pkg-config
            mold
          ];

          LIBCLANG_PATH =
            pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

          BINDGEN_EXTRA_CLANG_ARGS =
            (builtins.map (a: ''-I"${a}/include"'') [
              pkgs.glibc.dev
            ])
            ++ [
              ''
                -I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
              ''-I"${pkgs.glib.dev}/include/glib-2.0"''
              "-I${pkgs.glib.out}/lib/glib-2.0/include/"
            ];

          RUSTFLAGS = (builtins.map (a: "-L ${a}/lib") [
            pkgs.libglvnd
            pkgs.libxkbcommon
            pkgs.wayland
            pkgs.vulkan-loader
            pkgs.xorg.libX11
            pkgs.xorg.libXcursor
            pkgs.xorg.libXi
            pkgs.xorg.libXrandr
          ]) ++ [
            "-C"
            "link-arg=-Wl,-rpath,${libPath}"
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        iconViewer = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;

          nativeBuildInputs = commonArgs.nativeBuildInputs ++ [ pkgs.makeWrapper ];

          postInstall = ''
            wrapProgram $out/bin/${binName} \
              --set LD_LIBRARY_PATH ${libPath}
          '';
        });
      in {
        packages.default = iconViewer;

        apps.default = flake-utils.lib.mkApp {
          drv = iconViewer;
          name = binName;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = commonArgs.buildInputs ++ [ rustToolchain ];
          nativeBuildInputs = commonArgs.nativeBuildInputs;

          LIBCLANG_PATH = commonArgs.LIBCLANG_PATH;
          BINDGEN_EXTRA_CLANG_ARGS = commonArgs.BINDGEN_EXTRA_CLANG_ARGS;
          RUSTFLAGS = commonArgs.RUSTFLAGS;
          LD_LIBRARY_PATH = libPath;

          shellHook = ''
            export PATH=$PATH:${rustToolchain}/bin
          '';
        };
      });
}
