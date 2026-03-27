#!/bin/bash
# Test completo de posts programados
# Crea posts de prueba, construye el sitio y verifica el comportamiento

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "=== Test de Posts Programados ==="
echo ""

# Verificar Zola
if ! command -v zola &> /dev/null; then
    echo -e "${RED}Error: Zola no está instalado${NC}"
    exit 1
fi

# Crear posts de prueba
echo "1. Creando posts de prueba..."
"$SCRIPT_DIR/create-test-post.sh" > /dev/null

# Construir sitio
echo ""
echo "2. Construyendo sitio..."
cd "$PROJECT_DIR"
zola build 2>/dev/null

# Verificar resultados
echo ""
echo "3. Verificando resultados..."
echo ""

# Función para verificar si un archivo existe
check_file() {
    local file="$1"
    local expected="$2"
    local name="$3"

    if [ -f "$file" ]; then
        if [ "$expected" = "exists" ]; then
            echo -e "  ${GREEN}✓${NC} $name - Existe (correcto)"
            return 0
        else
            echo -e "  ${RED}✗${NC} $name - Existe (debería estar oculto)"
            return 1
        fi
    else
        if [ "$expected" = "missing" ]; then
            echo -e "  ${GREEN}✓${NC} $name - No existe (correcto, está oculto)"
            return 0
        else
            echo -e "  ${RED}✗${NC} $name - No existe (debería existir)"
            return 1
        fi
    fi
}

# Verificar posts
ERRORS=0

echo "Posts individuales:"
check_file "$PROJECT_DIR/public/blog/test-post-pasado/index.html" "exists" "Post fecha pasada" || ((ERRORS++))
check_file "$PROJECT_DIR/public/blog/test-post-hoy/index.html" "exists" "Post fecha hoy" || ((ERRORS++))
check_file "$PROJECT_DIR/public/blog/test-post-futuro/index.html" "exists" "Post fecha futura (página existe)" || ((ERRORS++))

echo ""
echo "Listado del blog:"

# Verificar que los posts pasados aparecen en el listado
if grep -q "test-post-pasado" "$PROJECT_DIR/public/blog/index.html" 2>/dev/null; then
    echo -e "  ${GREEN}✓${NC} Post pasado aparece en listado"
else
    echo -e "  ${RED}✗${NC} Post pasado NO aparece en listado"
    ((ERRORS++))
fi

if grep -q "test-post-hoy" "$PROJECT_DIR/public/blog/index.html" 2>/dev/null; then
    echo -e "  ${GREEN}✓${NC} Post de hoy aparece en listado"
else
    echo -e "  ${RED}✗${NC} Post de hoy NO aparece en listado"
    ((ERRORS++))
fi

# Verificar que el post futuro NO aparece en el listado
if grep -q "test-post-futuro" "$PROJECT_DIR/public/blog/index.html" 2>/dev/null; then
    echo -e "  ${RED}✗${NC} Post futuro aparece en listado (NO debería)"
    ((ERRORS++))
else
    echo -e "  ${GREEN}✓${NC} Post futuro NO aparece en listado (correcto)"
fi

echo ""
echo "Página del post futuro:"

# Verificar que el post futuro muestra "Próximamente"
if grep -q "Próximamente" "$PROJECT_DIR/public/blog/test-post-futuro/index.html" 2>/dev/null; then
    echo -e "  ${GREEN}✓${NC} Muestra mensaje 'Próximamente'"
else
    echo -e "  ${RED}✗${NC} NO muestra mensaje 'Próximamente'"
    ((ERRORS++))
fi

echo ""
echo "Posts destacados en página principal:"

# Verificar destacados
if grep -q "test-destacado-visible" "$PROJECT_DIR/public/index.html" 2>/dev/null; then
    echo -e "  ${GREEN}✓${NC} Destacado pasado aparece en inicio"
else
    echo -e "  ${YELLOW}?${NC} Destacado pasado no encontrado (verificar manualmente)"
fi

if grep -q "test-destacado-oculto" "$PROJECT_DIR/public/index.html" 2>/dev/null; then
    echo -e "  ${RED}✗${NC} Destacado futuro aparece en inicio (NO debería)"
    ((ERRORS++))
else
    echo -e "  ${GREEN}✓${NC} Destacado futuro NO aparece en inicio (correcto)"
fi

# Resumen
echo ""
echo "=== Resumen ==="
if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}Todos los tests pasaron correctamente${NC}"
else
    echo -e "${RED}$ERRORS test(s) fallaron${NC}"
fi

# Limpiar
echo ""
read -p "¿Limpiar posts de prueba? [s/N] " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Ss]$ ]]; then
    "$SCRIPT_DIR/cleanup-test-posts.sh"
fi

exit $ERRORS
