let
  pkgs = import <nixpkgs> { overlays= [moz_overlay]; };
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz); 

  libraries = with pkgs;[
    webkitgtk
    gtk3
    cairo
    gdk-pixbuf
    glib
    dbus
    openssl_3
  ];

  packages = with pkgs; [
    latest.rustChannels.nightly.rust
    cargo
    rustc
    rustfmt
    cmake
    pkg-config
    dbus
    openssl_3
    glib
    gtk3
    libsoup
    webkitgtk
    appimagekit
    cargo-tauri
    nodePackages.pnpm
    nodejs
  ];
in
pkgs.mkShell {
  buildInputs = packages;

  shellHook =
    ''
      export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    '';
}
