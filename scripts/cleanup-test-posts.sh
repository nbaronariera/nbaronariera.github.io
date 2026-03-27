#!/bin/bash
# Elimina los posts de prueba creados por create-test-post.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BLOG_DIR="$PROJECT_DIR/content/blog"

echo "=== Limpiando Posts de Prueba ==="
echo ""

# Lista de posts de prueba
TEST_POSTS=(
    "test-post-pasado.md"
    "test-post-hoy.md"
    "test-post-futuro.md"
    "test-destacado-visible.md"
    "test-destacado-oculto.md"
)

for post in "${TEST_POSTS[@]}"; do
    if [ -f "$BLOG_DIR/$post" ]; then
        rm "$BLOG_DIR/$post"
        echo "Eliminado: $post"
    else
        echo "No encontrado: $post"
    fi
done

echo ""
echo "=== Limpieza Completada ==="
