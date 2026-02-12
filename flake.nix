{
  description = "ZincShell flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;
        guiLibs = with pkgs;[
          # misc. libraries
          openssl
          pkg-config
          glib
          pango
          atkmm
          gdk-pixbuf
          gtk3
          libsoup_3
          webkitgtk_4_1
          # GUI libs
          libxkbcommon
          libGL
          fontconfig
          # wayland libraries
          wayland
          # x11 libraries
          libxcursor
          libxrandr
          libxi
          libx11
          libxcb
          # vulkan
          vulkan-loader
        ];
        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = guiLibs;
          nativeBuildInputs = [ pkgs.pkg-config ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # 1. 编译应用
        ZincShellUnwrapped = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "ZincShell";
          version = "0.1.0";
          propagatedBuildInputs = guiLibs;
        });

        # 2. 应用编译后需要使用wrapProgram处理动态链接库的路径问题
        zinc_shell = pkgs.stdenv.mkDerivation {
          pname = "ZincShell";
          version = "0.1.0";
          src = ZincShellUnwrapped;
          buildInputs = [ pkgs.makeWrapper ];
          installPhase = ''
            mkdir -p $out/bin
            cp ${ZincShellUnwrapped}/bin/zinc_shell $out/bin/zinc_shell
            wrapProgram $out/bin/zinc_shell \
              --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath guiLibs}
          '';
        };
      in
      {
        packages.default = zinc_shell;
        devShells.default = pkgs.mkShell rec {
          buildInputs = guiLibs;
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        };
      });
}
