+++
title = "¡Bienvenido!"
date = 2024-11-22
description = "Bienvenido a mi portfolio. Aquí explico como funciona y muestro ejemplos interactivos"
weight = 0
[extra]
imagenurl = "/wasm/snake/snake.jpg"
highlight = true
+++
<link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">

# Bienvenido a mi portfolio interactivo

¡Hola! Soy Nicolás, estudiante de ingeniería informática. Como parte de mi proceso de aprendizaje he creado este pequeño portfolio interactivo.

No soy diseñador web, el estilo de la web es una versión reconfigurada de la plantilla [blow](https://github.com/tchartron/blow). Para crear esta web he usado [Zola](https://www.getzola.org/documentation/getting-started/overview/), que permite usar plantillas y markdown para construir la web.

# Ejemplos interactivos

Para lograr que este portfolio sea interesante, he creado un sistema de canvas interactivo. Por ejemplo, aquí hay uno vacío:

{{ canvas(id="0", module="placeholder") }}

Los botones hacen lo siguiente:
<i class="material-icons">play_arrow</i> y <i class="material-icons">pause</i> inician y pausan el programa,
<i class="material-icons">replay</i> lo reinicia y
**─** minimiza la ventana interactiva

Esto es posible gracias a Rust. Rust es un lenguaje de programación de bajo nivel que es seguro en memoria, y tiene la capacidad de compilar a WASM, o Web Assembly, permitiendo así que los programas se ejecuten en cualquier web. Para más información, [puedes leer aquí.](https://rustwasm.github.io/book/)

# ¡Snake!

![Imagen de Snake de Wikipedia](/wasm/snake/snake.jpg)

Snake es un videojuego que se origina en las máquinas arcade, en 1976. La idea del juego es maniobrar la serpiente para comer todas las manzanas posibles sin chocar con las paredes o tu propio cuerpo. Seguramente todos hayáis jugado a una versión de Snake en algún momento de vuestra vida, pues bien, aquí tenéis mi propia versión. No es muy increíble pero es disfrutón.

{{ canvas(id="1", module="snake") }}

## Controles:
- W -> Arriba
- S -> Abajo
- D -> Derecha
- A -> Izquierda
- Esc -> Finalizar partida

## Cómo está programado

Como ya mencioné, he utilizado Rust como lenguaje de programación. El sistema tiene un loop de juego en el cual la serpiente se mueve, realiza los checks, y luego es dibujada. Para detectar la entrada he añadido eventos en el canvas, que modifican la dirección de la serpiente.

## Qué mejoraría

### Principio KISS (Keep It Simple Stupid)

Creo que el mayor problema que he tenido con el juego ha sido la complejidad que yo mismo le he añadido. En un primer momento, los bloques de la serpiente eran guardados como una lista enlazada, es decir, que iban guardando las referencias en memoria de cada parte. Esto era complicado de navegar, sobre todo teniendo en cuenta que sólo guardaba la posición. Al final lo cambié a un vector y el resultado es mucho mejor.

### Librería como toca

Mi sistema para convertir mi código de rust en un programa en wasm no es perfecta. Necesito mejorar la librería (y tal vez publicarla) con tal de que sea fácilmente manejable. La idea principal es separar la lógica de la presentación lo máximo posible.

---

Eso es todo por hoy presentación. Espero que disfrutes de tu estancia por mi portfolio y si estás interesado en mi trabajo, mi correo electrónico es [nbaronariera@gmail.com](mailto:nbaronariera@gmail.com).
¡Hasta la próxima!
