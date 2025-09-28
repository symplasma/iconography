{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    cargo
    rustc
    
    # X11 libraries needed for egui/eframe
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXi
    
    # OpenGL libraries
    libGL
    
    # Additional libraries that might be needed
    pkg-config
    fontconfig
    freetype
  ];

  # Set environment variables for linking
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXinerama
    pkgs.xorg.libXi
    pkgs.libGL
  ];
}
