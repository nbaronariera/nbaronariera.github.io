#!/bin/bash
# Construye el CSS con Tailwind
# Uso: ./scripts/build-css.sh [dev|prod|watch]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
THEME_DIR="$PROJECT_DIR/themes/blow"

MODE="${1:-prod}"

cd "$THEME_DIR"

case "$MODE" in
    dev)
        echo "Construyendo CSS en modo desarrollo..."
        npm run dev
        ;;
    prod)
        echo "Construyendo CSS en modo producción..."
        npm run prod
        ;;
    watch)
        echo "Modo watch: el CSS se reconstruirá automáticamente..."
        echo "Presiona Ctrl+C para detener"
        npm run watch
        ;;
    *)
        echo "Uso: $0 [dev|prod|watch]"
        echo "  dev   - Build de desarrollo (sin minificar)"
        echo "  prod  - Build de producción (minificado)"
        echo "  watch - Reconstruye automáticamente al detectar cambios"
        exit 1
        ;;
esac

echo ""
echo "CSS generado: $THEME_DIR/static/css/main.css"
