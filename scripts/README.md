# Scripts de Desarrollo

Scripts para automatizar el desarrollo y testing del portfolio.

## Requisitos

- **Node.js** y **npm**: Para compilar Tailwind CSS
- **Zola**: Generador de sitios estáticos

### Instalación de requisitos (Arch Linux)

```bash
sudo pacman -S nodejs npm zola
```

## Scripts Disponibles

### `setup.sh` - Configuración Inicial

Instala todas las dependencias y construye el CSS. Ejecutar después de clonar el repo.

```bash
./scripts/setup.sh
```

---

### `dev.sh` - Modo Desarrollo (Recomendado)

Inicia Zola y el watcher de CSS en paralelo. Los cambios en CSS se recompilan automáticamente.

```bash
./scripts/dev.sh
```

Abre automáticamente: http://127.0.0.1:1111

---

### `serve.sh` - Solo Servidor Zola

Inicia solo el servidor de Zola sin watch de CSS.

```bash
./scripts/serve.sh [puerto]

# Ejemplos:
./scripts/serve.sh        # Puerto 1111 (default)
./scripts/serve.sh 8080   # Puerto 8080
```

---

### `build-css.sh` - Compilar CSS

Compila el CSS de Tailwind.

```bash
./scripts/build-css.sh [modo]

# Modos:
./scripts/build-css.sh dev    # Desarrollo (sin minificar)
./scripts/build-css.sh prod   # Producción (minificado)
./scripts/build-css.sh watch  # Auto-recompila al detectar cambios
```

---

### `build.sh` - Build de Producción

Genera el sitio estático completo listo para deploy.

```bash
./scripts/build.sh
```

El sitio se genera en `public/`.

---

## Testing de Posts Programados

### `create-test-post.sh` - Crear Posts de Prueba

Crea 5 posts de prueba con diferentes fechas para verificar el filtro de posts programados.

```bash
./scripts/create-test-post.sh
```

Posts creados:
| Post | Fecha | Esperado |
|------|-------|----------|
| `test-post-pasado.md` | Ayer | Visible en listado |
| `test-post-hoy.md` | Hoy | Visible en listado |
| `test-post-futuro.md` | +7 días | Oculto (muestra "Próximamente") |
| `test-destacado-visible.md` | Ayer | Visible en inicio |
| `test-destacado-oculto.md` | +7 días | Oculto de inicio |

---

### `test-scheduled-posts.sh` - Test Automatizado

Ejecuta un test completo del sistema de posts programados.

```bash
./scripts/test-scheduled-posts.sh
```

Este script:
1. Crea posts de prueba
2. Construye el sitio
3. Verifica que los filtros funcionen correctamente
4. Muestra un resumen de resultados
5. Ofrece limpiar los posts de prueba

---

### `cleanup-test-posts.sh` - Limpiar Posts de Prueba

Elimina todos los posts de prueba creados.

```bash
./scripts/cleanup-test-posts.sh
```

---

## Flujo de Trabajo Típico

### Desarrollo diario

```bash
# 1. Iniciar entorno de desarrollo
./scripts/dev.sh

# 2. Hacer cambios en templates/CSS
# 3. Los cambios se reflejan automáticamente
# 4. Ctrl+C para detener
```

### Probar posts programados

```bash
# 1. Crear posts de prueba
./scripts/create-test-post.sh

# 2. Iniciar servidor
./scripts/serve.sh

# 3. Verificar en el navegador:
#    - http://127.0.0.1:1111/blog/ (posts pasados visibles)
#    - http://127.0.0.1:1111/blog/test-post-futuro/ (muestra "Próximamente")

# 4. Limpiar cuando termines
./scripts/cleanup-test-posts.sh
```

### Deploy

```bash
# 1. Build de producción
./scripts/build.sh

# 2. El sitio está en public/
# 3. Subir a GitHub Pages o tu hosting
```

---

## Estructura de Archivos

```
scripts/
├── README.md              # Este archivo
├── setup.sh               # Configuración inicial
├── dev.sh                 # Desarrollo (Zola + CSS watch)
├── serve.sh               # Solo servidor Zola
├── build.sh               # Build de producción
├── build-css.sh           # Compilar CSS
├── create-test-post.sh    # Crear posts de prueba
├── cleanup-test-posts.sh  # Limpiar posts de prueba
└── test-scheduled-posts.sh # Test automatizado
```
