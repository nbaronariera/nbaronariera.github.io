#!/bin/bash

# Script para compilar módulos WASM del portfolio
# Uso:
#   ./scripts/build-wasm.sh          # Compila todos los módulos
#   ./scripts/build-wasm.sh snake    # Compila solo un módulo específico

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RUST_DIR="$PROJECT_ROOT/rust"
WASM_OUTPUT_DIR="$PROJECT_ROOT/static/wasm"
WASM_LIB_DIR="$RUST_DIR/wasm-canvas-lib"

# Módulos que no se compilan (librerías o plantillas)
SKIP_MODULES=("template" "wasm-canvas-lib")

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

echo_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Verificar dependencias
check_dependencies() {
    if ! command -v wasm-pack &> /dev/null; then
        echo_error "wasm-pack no está instalado"
        echo "Instálalo con: cargo install wasm-pack"
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        echo_error "cargo no está instalado"
        echo "Instala Rust desde: https://rustup.rs/"
        exit 1
    fi
}

# Compilar un módulo específico
build_module() {
    local module_name="$1"
    local module_path="$RUST_DIR/$module_name"
    local output_path="$WASM_OUTPUT_DIR/$module_name"

    if [ ! -d "$module_path" ]; then
        echo_error "El módulo '$module_name' no existe en $RUST_DIR"
        return 1
    fi

    # Verificar si el módulo debe saltarse
    for skip in "${SKIP_MODULES[@]}"; do
        if [ "$module_name" = "$skip" ]; then
            echo_warn "Saltando '$module_name' (librería/plantilla)"
            return 0
        fi
    done

    echo_info "Compilando módulo: $module_name"

    # Compilar con wasm-pack
    cd "$module_path"
    wasm-pack build --target web --out-name "$module_name"

    # Crear directorio de salida si no existe
    mkdir -p "$output_path"

    # Copiar archivos generados
    cp "pkg/${module_name}.js" "$output_path/"
    cp "pkg/${module_name}_bg.wasm" "$output_path/"

    # Copiar archivos .d.ts si existen (opcional, para TypeScript)
    if [ -f "pkg/${module_name}.d.ts" ]; then
        cp "pkg/${module_name}.d.ts" "$output_path/"
    fi
    if [ -f "pkg/${module_name}_bg.wasm.d.ts" ]; then
        cp "pkg/${module_name}_bg.wasm.d.ts" "$output_path/"
    fi

    echo_info "Módulo '$module_name' compilado exitosamente"
}

# Verificar que la librería compile
check_library() {
    if [ -d "$WASM_LIB_DIR" ]; then
        echo_info "Verificando librería wasm-canvas-lib..."
        cd "$WASM_LIB_DIR"
        cargo check --quiet
        echo_info "Librería wasm-canvas-lib OK"
    fi
}

# Compilar todos los módulos
build_all() {
    echo_info "Compilando todos los módulos WASM..."

    # Primero verificar la librería
    check_library

    for module_dir in "$RUST_DIR"/*/; do
        if [ -d "$module_dir" ]; then
            local module_name=$(basename "$module_dir")
            build_module "$module_name"
        fi
    done

    echo_info "¡Todos los módulos compilados!"
}

# Main
check_dependencies

if [ -n "$1" ]; then
    # Compilar módulo específico
    build_module "$1"
else
    # Compilar todos
    build_all
fi

echo_info "Los archivos WASM están en: $WASM_OUTPUT_DIR"
