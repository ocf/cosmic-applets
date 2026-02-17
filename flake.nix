{
  description = "OCF COSMIC Applets";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        runtimeLibs = with pkgs; [ libGL libxkbcommon wayland vulkan-loader ];

        commonArgs = {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (craneLib.filterCargoSources path type) ||
              (builtins.match ".*\.desktop$" path != null);
          }; 
          strictDeps = true;
          nativeBuildInputs = with pkgs; [ pkg-config copyDesktopItems ];
          buildInputs = runtimeLibs;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        buildApplet = name: path: craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = name;
          version = "0.1.0";
          cargoExtraArgs = "-p ${name}"; 
          
          postInstall = ''
            install -Dm644 ${path}/${name}.desktop $out/share/applications/${name}.desktop
            
            substituteInPlace $out/share/applications/${name}.desktop \
              --replace "@EXEC_PATH@" "$out/bin/${name}"
          '';

          postFixup = ''
            patchelf --add-rpath "${pkgs.lib.makeLibraryPath runtimeLibs}" $out/bin/${name}
          '';
        });
      in
      {
        packages = {
          paper = buildApplet "ocf-paper-quota-applet" "./paper-quota-applet";
          status = buildApplet "ocf-logout-applet" "./logout-applet";
          
          default = pkgs.symlinkJoin {
            name = "all";
            paths = [ self.packages.${system}.paper self.packages.${system}.status ];
          };
        };

        devShells.default = craneLib.devShell {
          inputsFrom = [ self.packages.${system}.paper ];
          packages = with pkgs; [ cargo rustc rust-analyzer clippy rustfmt ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
        };
      }
    );
}
