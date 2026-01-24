{
  lib,
  rustPlatform,
  pkg-config,
  installShellFiles,
  systemd,
  udevCheckHook,
  version ? "git",
  debug ? false,
}:
let
  fs = lib.fileset;
  inherit (lib.strings) optionalString;
in

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "l1ght${optionalString debug "-debug"}";
  version = version;

  cargoBuildType = if debug then "debug" else "release";
  dontStrip = debug;

  src = fs.toSource {
    root = ../.;
    fileset = (fs.gitTracked ../.);
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
    installShellFiles
    udevCheckHook
  ];

  buildInputs = [
    systemd
  ];

  postInstall = ''
    installManPage --name l1ght.1 ./man/l1ght.1  

    mkdir -p "$out/lib/udev/rules.d/"
    cp contrib/90-backlight.rules "$out/lib/udev/rules.d/"
  '';

  doInstallCheck = true;
})
