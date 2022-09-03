{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell rec {
  nativeBuildInputs = [
    pkgconfig
    clang
    mold
    rustup
  ];
  buildInputs = [
    udev alsaLib vulkan-loader
    xlibsWrapper xorg.libXcursor xorg.libXrandr xorg.libXi # To use x11 feature
    libxkbcommon wayland # To use wayland feature
  ];
  RUSTC_VERSION = 1.63;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
