#!/bin/bash
# Inicia el servidor de desarrollo de Zola
# Uso: ./scripts/serve.sh [puerto]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

PORT="${1:-1111}"

# Verificar si Zola está instalado
if ! command -v zola &> /dev/null; then
    echo "Error: Zola no está instalado"
    echo ""
    echo "Opciones de instalación:"
    echo "  Arch Linux:  sudo pacman -S zola"
    echo "  Nix:         nix-shell -p zola"
    echo "  Cargo:       cargo install zola"
    echo "  Homebrew:    brew install zola"
    echo ""
    echo "Más info: https://www.getzola.org/documentation/getting-started/installation/"
    exit 1
fi

echo "=== Servidor de Desarrollo ==="
echo ""
echo "Zola: $(zola --version)"
echo "Puerto: $PORT"
echo "URL: http://127.0.0.1:$PORT"
echo ""
echo "Presiona Ctrl+C para detener"
echo ""

cd "$PROJECT_DIR"
zola serve --port "$PORT" --open
