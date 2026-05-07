# g++ src/main.cpp src/glad.c -Iinclude -o main $(pkg-config --cflags --libs glfw3)

{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gcc
    glfw
    libGL
    pkg-config
  ];

  shellHook = ''
    echo "C development environment loaded"
    echo "Compiler: $(gcc --version | head -n1)"
  '';
}

