let nixpkgs = import <nixpkgs> {};
in rec {
  fhsEnv = nixpkgs.buildFHSUserEnv {
    name = "fhs";
    targetPkgs = pkgs: with pkgs; [
    ];
    multiPkgs = pkgs: with pkgs; [ 
      gcc
      binutils
      xorg.libXxf86vm
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      libGL
      mesa_glu
      rustup
    ];
    runScript = "bash";
  };
}
