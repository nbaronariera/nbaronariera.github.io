#!/bin/bash
# Crea posts de prueba para verificar la funcionalidad de posts programados
# Uso: ./scripts/create-test-post.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BLOG_DIR="$PROJECT_DIR/content/blog"

# Fechas
TODAY=$(date +%Y-%m-%d)
YESTERDAY=$(date -d "yesterday" +%Y-%m-%d 2>/dev/null || date -v-1d +%Y-%m-%d)
FUTURE=$(date -d "+7 days" +%Y-%m-%d 2>/dev/null || date -v+7d +%Y-%m-%d)

echo "=== Creando Posts de Prueba ==="
echo ""
echo "Fecha de hoy: $TODAY"
echo "Fecha pasada: $YESTERDAY"
echo "Fecha futura: $FUTURE"
echo ""

# Crear directorio si no existe
mkdir -p "$BLOG_DIR"

# Post con fecha pasada (debería aparecer)
cat > "$BLOG_DIR/test-post-pasado.md" << EOF
+++
title = "Post de Prueba - Fecha Pasada"
date = $YESTERDAY
description = "Este post tiene fecha pasada y DEBERÍA aparecer en el listado"

[extra]
highlight = false
+++

Este es un post de prueba con fecha **pasada**.

<!-- more -->

Si puedes ver este post en el listado del blog, el filtro de fechas está funcionando correctamente para posts con fecha pasada.

## Verificación

- Fecha del post: $YESTERDAY
- Estado esperado: **Visible**
EOF

echo "Creado: $BLOG_DIR/test-post-pasado.md"

# Post con fecha de hoy (debería aparecer)
cat > "$BLOG_DIR/test-post-hoy.md" << EOF
+++
title = "Post de Prueba - Fecha de Hoy"
date = $TODAY
description = "Este post tiene fecha de hoy y DEBERÍA aparecer en el listado"

[extra]
highlight = false
+++

Este es un post de prueba con fecha de **hoy**.

<!-- more -->

Si puedes ver este post en el listado del blog, el filtro de fechas está funcionando correctamente para posts con fecha actual.

## Verificación

- Fecha del post: $TODAY
- Estado esperado: **Visible**
EOF

echo "Creado: $BLOG_DIR/test-post-hoy.md"

# Post con fecha futura (NO debería aparecer)
cat > "$BLOG_DIR/test-post-futuro.md" << EOF
+++
title = "Post de Prueba - Fecha Futura (OCULTO)"
date = $FUTURE
description = "Este post tiene fecha futura y NO debería aparecer en el listado"

[extra]
highlight = false
+++

Este es un post de prueba con fecha **futura**.

<!-- more -->

Si puedes ver este post en el listado del blog, hay un problema con el filtro de fechas.

Si accedes directamente a la URL de este post, deberías ver un mensaje de "Próximamente".

## Verificación

- Fecha del post: $FUTURE
- Estado esperado: **Oculto** (mostrar "Próximamente" si se accede directamente)
EOF

echo "Creado: $BLOG_DIR/test-post-futuro.md"

# Post destacado con fecha pasada
cat > "$BLOG_DIR/test-destacado-visible.md" << EOF
+++
title = "Post Destacado - Visible"
date = $YESTERDAY
description = "Post destacado con fecha pasada, debería aparecer en la página principal"

[extra]
highlight = true
+++

Este es un post **destacado** con fecha pasada.

<!-- more -->

Debería aparecer en la sección de proyectos destacados de la página principal.

## Verificación

- Fecha del post: $YESTERDAY
- Destacado: Sí
- Estado esperado: **Visible en inicio**
EOF

echo "Creado: $BLOG_DIR/test-destacado-visible.md"

# Post destacado con fecha futura
cat > "$BLOG_DIR/test-destacado-oculto.md" << EOF
+++
title = "Post Destacado - Oculto (Futuro)"
date = $FUTURE
description = "Post destacado con fecha futura, NO debería aparecer"

[extra]
highlight = true
+++

Este es un post **destacado** con fecha futura.

<!-- more -->

NO debería aparecer en la sección de proyectos destacados de la página principal.

## Verificación

- Fecha del post: $FUTURE
- Destacado: Sí
- Estado esperado: **Oculto**
EOF

echo "Creado: $BLOG_DIR/test-destacado-oculto.md"

echo ""
echo "=== Posts de Prueba Creados ==="
echo ""
echo "Para probar:"
echo "  1. Ejecuta: ./scripts/serve.sh"
echo "  2. Abre: http://127.0.0.1:1111/blog/"
echo ""
echo "Verificaciones:"
echo "  - Posts con fecha pasada/hoy: Deben aparecer en /blog/"
echo "  - Post con fecha futura: NO debe aparecer en /blog/"
echo "  - Acceso directo a post futuro: Debe mostrar 'Próximamente'"
echo "  - Post destacado pasado: Debe aparecer en página principal"
echo "  - Post destacado futuro: NO debe aparecer en página principal"
echo ""
echo "Para limpiar los posts de prueba:"
echo "  ./scripts/cleanup-test-posts.sh"
