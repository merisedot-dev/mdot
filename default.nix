{
  pkgs,
  rustPlatform,
  stdenv,
  ...
}:

let
  mdot_version = "0.1.0";
in
stdenv.mkDerivation (finalAttrs: {
  # core project info
  pname = "mdot";
  version = mdot_version;

  # sources and dependencies
  src = ./.;
  cargoDeps = rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };

  # runtime inputs
  nativeBuildInputs = with pkgs; [
    meson
    ninja
    pkg-config
    rustPlatform.cargoSetupHook
    rustc
    cargo
    gettext
    desktop-file-utils
    wrapGAppsHook4
  ];

  # compiling inputs
  propagatedBuildInputs = with pkgs; [
    gtk4
    glib
    libadwaita
    gnome-desktop
    desktop-file-utils
  ];

  # FIXME post-build
})
