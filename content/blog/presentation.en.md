+++
title = "Welcome!"
date = 2024-11-22
description = "Welcome to my portfolio. Here I explain how it works and show interactive examples"
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

Hello! I'm Nicolás, a computer engineering student. As part of my learning process, I've created this small interactive portfolio.

I'm not a web designer, the style of the website is a reconfigured version of the [blow](https://github.com/tchartron/blow) template. To create this website I used [Zola](https://www.getzola.org/documentation/getting-started/overview/), which allows using templates and markdown to build the site.

# Interactive examples

To make this portfolio interesting, I've created an interactive canvas system. For example, here's an empty one:

{{ canvas(id="0", module="placeholder") }}

The buttons do the following:
<i class="material-icons">play_arrow</i> and <i class="material-icons">pause</i> start and pause the program,
<i class="material-icons">replay</i> restarts it and
**─** minimizes the interactive window

This is possible thanks to Rust. Rust is a low-level programming language that is memory-safe, and has the ability to compile to WASM, or Web Assembly, thus allowing programs to run on any website. For more information, [you can read here.](https://rustwasm.github.io/book/)

# Snake!

![Snake image from Wikipedia](/wasm/snake/snake.jpg)

Snake is a video game that originated in arcade machines in 1976. The idea of the game is to maneuver the snake to eat as many apples as possible without hitting the walls or your own body. You've probably all played a version of Snake at some point in your life, so here's my own version. It's not very impressive but it's enjoyable.

{{ canvas(id="1", module="snake") }}

## Controls:
- W -> Up
- S -> Down
- D -> Right
- A -> Left
- Esc -> End game

## How it's programmed

As I mentioned, I used Rust as the programming language. The system has a game loop in which the snake moves, performs checks, and then is drawn. To detect input I added events on the canvas that modify the snake's direction.

## What I would improve

### KISS Principle (Keep It Simple Stupid)

I think the biggest problem I had with the game was the complexity I added myself. At first, the snake blocks were stored as a linked list, meaning they kept memory references to each part. This was complicated to navigate, especially considering it only stored the position. In the end I changed it to a vector and the result is much better.

### Proper library

My system for converting my Rust code into a WASM program isn't perfect. I need to improve the library (and maybe publish it) so that it's easily manageable. The main idea is to separate logic from presentation as much as possible.

---

That's all for today's presentation. I hope you enjoy your stay at my portfolio and if you're interested in my work, my email is [nbaronariera@gmail.com](mailto:nbaronariera@gmail.com).
See you next time!
