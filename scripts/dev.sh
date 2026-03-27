#!/bin/bash
# Script principal de desarrollo
# Ejecuta Zola y el watcher de CSS en paralelo

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Modo Desarrollo ===${NC}"
echo ""

# Verificar dependencias
if ! command -v zola &> /dev/null; then
    echo -e "${RED}Error: Zola no está instalado${NC}"
    echo "Instálalo con: sudo pacman -S zola"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js no está instalado${NC}"
    echo "Instálalo con: sudo pacman -S nodejs npm"
    exit 1
fi

# Verificar si las dependencias npm están instaladas
if [ ! -d "$PROJECT_DIR/themes/blow/node_modules" ]; then
    echo -e "${YELLOW}Instalando dependencias npm...${NC}"
    cd "$PROJECT_DIR/themes/blow"
    npm install
fi

# Construir CSS inicial
echo -e "${GREEN}Construyendo CSS...${NC}"
cd "$PROJECT_DIR/themes/blow"
npm run dev

echo ""
echo -e "${GREEN}Iniciando servidores...${NC}"
echo ""
echo -e "  ${BLUE}Zola:${NC}     http://127.0.0.1:1111"
echo -e "  ${BLUE}CSS Watch:${NC} Activo en segundo plano"
echo ""
echo -e "${YELLOW}Presiona Ctrl+C para detener ambos procesos${NC}"
echo ""

# Función para limpiar al salir
cleanup() {
    echo ""
    echo -e "${YELLOW}Deteniendo servidores...${NC}"
    kill $(jobs -p) 2>/dev/null
    exit 0
}

trap cleanup SIGINT SIGTERM

# Iniciar CSS watcher en segundo plano
cd "$PROJECT_DIR/themes/blow"
npm run watch &

# Iniciar Zola
cd "$PROJECT_DIR"
zola serve --port 1111

# Esperar a que terminen los procesos
wait
