+++
title = "Welcome!"
date = 2024-11-22
description = "Welcome to my portfolio. Here I explain how it works and showcase interactive examples."
weight = 0

[taxonomies]
categories = ["Projects"]
tags = ["rust", "wasm", "games"]

[extra]
imagenurl = "/wasm/snake/snake.jpg"
highlight = true
+++
<link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">

# Welcome to my interactive portfolio

Hi! I'm Nicolás, a computer engineering student focused on robust software development. This space is not just a static resume, but an interactive portfolio I've built as part of my continuous learning process.

Since my main focus is systems architecture and backend development, I chose to adapt the excellent [blow](https://github.com/tchartron/blow) theme for Zola. This allows me to use Markdown and templates to build the website efficiently, focusing my efforts on what really interests me: the code.

# Interactive examples

To bring this portfolio to life, I've implemented a system of interactive *canvases*. Here is a test canvas:

{{ canvas(id="0", module="placeholder") }}

The controls are simple:
* <i class="material-icons">play_arrow</i> and <i class="material-icons">pause</i> start and pause the execution.
* <i class="material-icons">replay</i> resets the state.
* **─** minimizes the interactive window.

The magic behind this is **Rust** compiled to **WebAssembly (WASM)**. Rust is a systems programming language that guarantees memory safety without needing a garbage collector. By compiling this code to WASM, we can runs logic directly in the browser, integrating seamlessly with the web. 

For more information about this ecosystem, I recommend [the official Rust and WebAssembly book.](https://rustwasm.github.io/book/)

# Snake!

!(/wasm/snake/snake.jpg)

Everyone knows the classic 1976 arcade game Snake. The goal is simple: maneuver the snake to eat as many apples as possible without crashing into the walls or your own body. 

This is my own implementation. It's a concise project, but it serves as an excellent proving ground to demonstrate the viability of integrating low-level logic in an interactive web environment. Go ahead and try it out!

{{ canvas(id="1", module="snake") }}

## Controls:
- **W** -> Up
- **S** -> Down
- **D** -> Right
- **A** -> Left
- **Esc** -> End game

*(Note: Make sure to click inside the game box so it registers your keystrokes).*

## Under the hood

The core of the system is a game loop written entirely in Rust. In each iteration (*tick*), the engine updates the snake's position, evaluates collisions, and calculates the new state. For user input, I intercept browser keyboard events and pass them to the WASM game state to modify the direction vector.

## Iteration and lessons learned

Software development is a constant process of refactoring. Here I outline some technical challenges I faced and how I plan to improve them.

### Data structures: The evolution of the snake

Initially, I modeled the snake's body using a linked list (`LinkedList`), storing the coordinates of each segment. However, in languages like Rust, linked lists scatter allocations across the *heap*, destroying CPU cache locality.

To optimize this, I migrated to a standard vector (`Vec`). Performance improved, but architecturally it is still not the mathematically optimal solution. As the snake moves forward, I remove the tail and add a new head. In a `Vec`, removing the first element forces all other elements to shift in memory, resulting in a time complexity of O(n). 

The logical next step to apply the principle of efficiency is to refactor to a double-ended queue, specifically a `VecDeque` in Rust. By functioning as a circular buffer, it will allow me to add elements to the head and remove them from the tail with O(1) complexity, achieving a much more robust system. However, for this demo, the current implementation is sufficient; the snake will not grow large enough for this to become a performance bottleneck.

### Clean architecture: Separating layers

Currently, the bridge between my Rust code and the browser rendering works well, but it is tightly coupled. My short-term goal is to encapsulate this integration into a more manageable library. The objective is to achieve an architecture where the game state (Rust) is pure and agnostic, limiting itself to exposing a linear memory buffer that the presentation layer (JavaScript/HTML5 Canvas) simply takes care of drawing. I am not sure yet if I will publish the library separately, but you can access all the files for this page from the [official repository](https://github.com/nbaronariera/nbaronariera.github.io).

---

I hope you enjoy exploring these experiments as much as I enjoyed programming them. If you are interested in talking about Rust, software architecture, or simply discussing the portfolio, you can contact me at [nbaronariera@gmail.com](mailto:nbaronariera@gmail.com). 

See you next time!