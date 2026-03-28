+++
title = "¡Bienvenido!"
date = 2024-11-22
description = "Bienvenido a mi portfolio. Aquí explico cómo funciona y muestro ejemplos interactivos."
weight = 0

[taxonomies]
categories = ["Proyectos"]
tags = ["rust", "wasm", "juegos"]

[extra]
imagenurl = "/wasm/snake/snake.jpg"
highlight = true
+++
<link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">

# Bienvenido a mi portfolio interactivo

¡Hola! Soy Nicolás, estudiante de ingeniería informática centrada en el desarrollo de software robusto. Este espacio no es solo un currículum estático, sino un portfolio interactivo que he construido como parte de mi aprendizaje continuo. 

Dado que mi enfoque principal es la arquitectura de sistemas y el backend, he optado por adaptar la excelente plantilla [blow](https://github.com/tchartron/blow) de Zola. Esto me permite usar Markdown y plantillas para construir la web de forma eficiente, centrando mis esfuerzos en lo que realmente me interesa: el código.

# Ejemplos interactivos

Para darle vida a este portfolio, he implementado un sistema de *canvas* interactivos. Aquí tienes un lienzo de prueba:

{{ canvas(id="0", module="placeholder") }}

Los controles son sencillos:
* <i class="material-icons">play_arrow</i> y <i class="material-icons">pause</i> inician y pausan la ejecución.
* <i class="material-icons">replay</i> reinicia el estado.
* **─** minimiza la ventana interactiva.

La magia detrás de esto es **Rust** compilado a **WebAssembly (WASM)**. Rust es un lenguaje de programación de sistemas que garantiza la seguridad de memoria sin necesidad de un recolector de basura. Al compilar este código a WASM, podemos ejecutar lógica directamente en el navegador, integrándose de forma fluida con la web. 

Para más información sobre este ecosistema, te recomiendo [el libro oficial de Rust y WASM.](https://rustwasm.github.io/book/)

# ¡Snake!

![Imagen de Snake de Wikipedia](/wasm/snake/snake.jpg)

Todo el mundo conoce el clásico Snake de las máquinas arcade de 1976. El objetivo es simple: maniobrar la serpiente para comer el mayor número de manzanas sin chocar contra las paredes o tu propio cuerpo. 

Esta es mi propia implementación. Es un proyecto conciso, pero es un excelente campo de pruebas para demostrar la viabilidad de integrar lógica de bajo nivel en un entorno web interactivo. ¡Anímate a probarlo!

{{ canvas(id="1", module="snake") }}

## Controles:
- **W** -> Arriba
- **S** -> Abajo
- **D** -> Derecha
- **A** -> Izquierda
- **Esc** -> Finalizar partida

*(Nota: Asegúrate de hacer clic dentro del recuadro del juego para que registre las teclas).*

## Cómo está programado bajo el capó

El núcleo del sistema es un bucle de juego (game loop) escrito íntegramente en Rust. En cada iteración (*tick*), el motor actualiza la posición de la serpiente, evalúa las colisiones y calcula el nuevo estado. Para la entrada del usuario, intercepto los eventos del teclado en el navegador y los transmito al estado del juego en WASM para modificar el vector de dirección.

## Iteración y lecciones aprendidas

El desarrollo de software es un proceso constante de refactorización. Aquí expongo algunos desafíos técnicos que he enfrentado y cómo planeo mejorarlos.

### Estructuras de datos: La evolución de la serpiente

Inicialmente, modelé el cuerpo de la serpiente utilizando una lista enlazada (`LinkedList`), almacenando las coordenadas de cada segmento. Sin embargo, en lenguajes como Rust, las listas enlazadas dispersan las asignaciones en el *heap*, destruyendo la localidad de caché de la CPU. 

Para optimizarlo, migré a un vector estándar (`Vec`). El rendimiento mejoró, pero arquitectónicamente sigue sin ser la solución matemáticamente óptima. Al avanzar la serpiente, elimino la cola y añado una nueva cabeza. En un `Vec`, eliminar el primer elemento obliga a desplazar todos los demás en memoria, lo que resulta en una complejidad de tiempo de O(n). 

El siguiente paso lógico para aplicar el principio de eficiencia es refactorizar a una cola de doble extremo, específicamente un `VecDeque` en Rust. Al funcionar como un búfer circular, me permitirá añadir elementos a la cabeza y eliminarlos de la cola con una complejidad de O(1), logrando un sistema mucho más robusto. No obstante, para la demo esto es suficiente, la serpiente no va a alcanzar el tamaño suficiente como para que sea un problema.

### Arquitectura limpia: Separando capas

Actualmente, el puente entre mi código en Rust y el renderizado en el navegador funciona bien, pero es acoplado. Mi objetivo a corto plazo es encapsular esta integración en una librería más manejable. La meta es alcanzar una arquitectura donde el estado del juego (Rust) sea puro y agnóstico, limitándose a exponer un búfer de memoria lineal que la capa de presentación (JavaScript/HTML5 Canvas) simplemente se encargue de dibujar. Desconozco si publicaré la librería por separado, pero puedes acceder a todos los ficheros de esta página a partir del [repositorio oficial](https://github.com/nbaronariera/nbaronariera.github.io)

---

Espero que disfrutes explorando estos experimentos tanto como yo he disfrutado programándolos. Si te interesa hablar sobre Rust, arquitectura de software o simplemente comentar el portfolio, puedes contactarme en [nbaronariera@gmail.com](mailto:nbaronariera@gmail.com). 

¡Hasta la próxima!