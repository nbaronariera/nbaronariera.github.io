#!/bin/bash
# Script de configuración inicial
# Instala dependencias y construye el CSS

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
THEME_DIR="$PROJECT_DIR/themes/blow"

echo "=== Configuración del Portfolio ==="
echo ""

# Verificar Node.js
if ! command -v node &> /dev/null; then
    echo "Error: Node.js no está instalado"
    echo "Instálalo con: sudo pacman -S nodejs npm"
    exit 1
fi

echo "Node.js: $(node --version)"
echo "npm: $(npm --version)"
echo ""

# Instalar dependencias del tema
echo "Instalando dependencias del tema..."
cd "$THEME_DIR"
npm install

echo ""
echo "Construyendo CSS de producción..."
npm run prod

echo ""
echo "=== Configuración completada ==="
echo ""
echo "Archivos generados:"
ls -lh "$THEME_DIR/static/css/main.css"
ls -lh "$THEME_DIR/static/js/"
