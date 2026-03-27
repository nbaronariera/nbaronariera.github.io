#!/bin/bash
# Script de build para producción
# Genera el sitio estático listo para deploy

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Build de Producción ==="
echo ""

# Verificar dependencias
if ! command -v zola &> /dev/null; then
    echo "Error: Zola no está instalado"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo "Error: Node.js no está instalado"
    exit 1
fi

# Instalar dependencias si es necesario
if [ ! -d "$PROJECT_DIR/themes/blow/node_modules" ]; then
    echo "Instalando dependencias npm..."
    cd "$PROJECT_DIR/themes/blow"
    npm install
fi

# Build CSS de producción
echo "Construyendo CSS de producción..."
cd "$PROJECT_DIR/themes/blow"
npm run prod

# Build de Zola
echo ""
echo "Construyendo sitio con Zola..."
cd "$PROJECT_DIR"
zola build

echo ""
echo "=== Build Completado ==="
echo ""
echo "El sitio está listo en: $PROJECT_DIR/public/"
echo ""
echo "Para previsualizar:"
echo "  cd $PROJECT_DIR/public && python -m http.server 8000"
echo "  Abre: http://localhost:8000"
