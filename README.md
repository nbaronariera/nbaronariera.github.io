# Portfolio Interactivo con Zola + WASM

Portfolio personal construido con [Zola](https://www.getzola.org/) que permite incrustar aplicaciones interactivas escritas en Rust y compiladas a WebAssembly.

## Requisitos

- [Zola](https://www.getzola.org/documentation/getting-started/installation/) (generador de sitios estáticos)
- [Rust](https://rustup.rs/) (lenguaje de programación)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (compilador Rust → WASM)

```bash
# Instalar wasm-pack
cargo install wasm-pack
```

## Estructura del Proyecto

```
nbaronariera.github.io/
├── config.toml              # Configuración de Zola
├── content/                 # Contenido en Markdown
│   ├── _index.md           # Página de inicio
│   └── blog/               # Posts del blog
│       └── presentation.md # Post con demos WASM
├── rust/                    # Código fuente Rust
│   ├── snake/              # Juego Snake
│   └── template/           # Plantilla para nuevos módulos
├── scripts/
│   └── build-wasm.sh       # Script de compilación
├── static/
│   ├── css/                # Estilos adicionales
│   └── wasm/               # Módulos WASM compilados
│       ├── snake/
│       └── placeholder/
├── templates/
│   ├── index.html          # Template de página principal
│   └── shortcodes/
│       ├── canvas.html     # Shortcode para WASM
│       └── featured_projects.html
└── themes/blow/            # Tema (Tailwind CSS)
```

## Comandos de Desarrollo

### Servidor de desarrollo

```bash
zola serve
```

Abre http://127.0.0.1:1111 en tu navegador.

### Compilar módulos WASM

```bash
# Compilar todos los módulos
./scripts/build-wasm.sh

# Compilar solo un módulo específico
./scripts/build-wasm.sh snake
```

### Generar sitio para producción

```bash
zola build
```

Los archivos generados estarán en `public/`.

## Cómo Agregar un Nuevo Módulo WASM

### 1. Copiar la plantilla

```bash
cp -r rust/template rust/mi-modulo
```

### 2. Configurar el módulo

Edita `rust/mi-modulo/Cargo.toml`:

```toml
[package]
name = "mi-modulo"
description = "Descripción de tu módulo"
# ... resto de la configuración
```

### 3. Implementar la lógica

Edita `rust/mi-modulo/src/lib.rs`. La función principal es `start_system(id)`:

```rust
#[wasm_bindgen]
pub fn start_system(id: &str) -> Result<(), JsValue> {
    let canvas = get_canvas(id);
    let ctx = get_context(&canvas);

    // Tu código aquí...

    Ok(())
}
```

### 4. Compilar

```bash
./scripts/build-wasm.sh mi-modulo
```

### 5. Usar en un post

En tu archivo Markdown:

```markdown
{{ canvas(id="1", module="mi-modulo") }}
```

**Nota:** Cada canvas en la misma página debe tener un `id` único.

## Shortcodes Disponibles

### `canvas`

Incrusta un módulo WASM interactivo.

```markdown
{{ canvas(id="1", module="snake") }}
```

Parámetros:
- `id`: Identificador único del canvas (número o string)
- `module`: Nombre del módulo en `static/wasm/`

### `featured_projects`

Muestra los proyectos destacados (páginas con `highlight = true`).

```markdown
{{ featured_projects() }}
```

## Marcar un Post como Destacado

En el frontmatter del post:

```toml
+++
title = "Mi Proyecto"
date = 2024-01-01
[extra]
highlight = true
imagenurl = "/wasm/mi-modulo/preview.jpg"
+++
```

## Deployment

El sitio está configurado para GitHub Pages. Al hacer push a `main`, se despliega automáticamente (si tienes configurado GitHub Actions).

### Despliegue manual

```bash
zola build
# Copiar contenido de public/ a tu servidor
```

## Recursos

- [Documentación de Zola](https://www.getzola.org/documentation/)
- [Rust + WASM Book](https://rustwasm.github.io/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Tema Blow](https://github.com/tchartron/blow)
