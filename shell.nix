{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "portfolio-dev";

  buildInputs = with pkgs; [
    zola
    nodejs_20
    nodePackages.npm
    python3 # Para servidor HTTP simple
  ];

  shellHook = ''
    echo ""
    echo "=== Portfolio Environment ==="
    echo ""
    echo "Versiones:"
    echo "  Zola:    $(zola --version)"
    echo "  Node:    $(node --version)"
    echo "  npm:     $(npm --version)"
    echo ""
    echo "Comandos disponibles:"
    echo "  ./scripts/setup.sh    - Configuración inicial"
    echo "  ./scripts/dev.sh      - Modo desarrollo"
    echo "  ./scripts/serve.sh    - Solo servidor Zola"
    echo "  ./scripts/build.sh    - Build de producción"
    echo ""
  '';
}
