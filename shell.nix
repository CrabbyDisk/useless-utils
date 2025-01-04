{
  pkgs ?
    import <nixpkgs> {
      overlays = [(import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")];
      config.allowUnfree = true;
    },
}:
pkgs.mkShell {
  # Get dependencies from the main package
  # Additional tooling
  buildInputs = with pkgs; [
    (fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ])
    gtk4.dev
    pkg-config
    cargo-watch
  ];

  runtimeDependencies = [
    pkgs.cudaPackages.cuda_nvml_dev.lib
  ];

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
      pkgs.gtk4
      pkgs.glib
      pkgs.linuxKernel.packages.linux_zen.nvidia_x11
    ]}:$LD_LIBRARY_PATH;
  '';
}
