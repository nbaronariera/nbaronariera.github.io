//! Implementación de CanvasApp para el juego Snake

use rand::prelude::Rng;
use wasm_canvas_lib::{fill_rect, draw_text, CanvasApp, CanvasRenderingContext2d};

use crate::snake::{Direction, Snake};

const CELL_SIZE: u32 = 40;

pub struct SnakeApp {
    snake: Option<Snake>,
    apple: (u32, u32),
    score: u32,
    game_over: bool,
    canvas_width: u32,
    canvas_height: u32,
    max_x: u32,
    max_y: u32,
}

impl SnakeApp {
    pub fn new() -> Self {
        Self {
            snake: None,
            apple: (0, 0),
            score: 0,
            game_over: false,
            canvas_width: 0,
            canvas_height: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    fn spawn_apple(&mut self) {
        let mut rng = rand::thread_rng();
        self.apple = (
            rng.gen_range(1..(self.max_x - 1)),
            rng.gen_range(1..(self.max_y - 1)),
        );
    }
}

impl CanvasApp for SnakeApp {
    fn init(&mut self, canvas_width: u32, canvas_height: u32) {
        self.canvas_width = canvas_width;
        self.canvas_height = canvas_height;
        self.max_x = canvas_width / CELL_SIZE;
        self.max_y = canvas_height / CELL_SIZE;

        assert_eq!(canvas_width % CELL_SIZE, 0, "El ancho no es divisible entre el tamaño de celda");
        assert_eq!(canvas_height % CELL_SIZE, 0, "El alto no es divisible entre el tamaño de celda");

        // Resetear estado
        self.score = 0;
        self.game_over = false;

        // Crear snake en el centro
        let start_pos = (self.max_x / 2, self.max_y / 2);
        self.snake = Some(Snake::new(start_pos, self.max_x, self.max_y));

        // Spawn manzana
        self.spawn_apple();
    }

    fn update(&mut self) -> bool {
        if self.game_over {
            return false;
        }

        let snake = self.snake.as_mut().unwrap();

        // Verificar colisiones
        if snake.hit_wall() || snake.check_internal_collision() {
            self.game_over = true;
            return false;
        }

        // Verificar si comió la manzana
        let ate_apple = snake.get_position() == self.apple;
        if ate_apple {
            self.score += 1;
            snake.add_part();
        }

        // Mover snake
        snake.move_by();

        // Spawn nueva manzana después de liberar el borrow
        if ate_apple {
            self.spawn_apple();
        }

        true
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        // Fondo
        fill_rect(ctx, 0.0, 0.0, self.canvas_width as f64, self.canvas_height as f64, "gray");

        if let Some(snake) = &self.snake {
            // Dibujar manzana
            fill_rect(
                ctx,
                (self.apple.0 * CELL_SIZE) as f64,
                (self.apple.1 * CELL_SIZE) as f64,
                CELL_SIZE as f64,
                CELL_SIZE as f64,
                "red",
            );

            // Dibujar cuerpo de la serpiente
            for pos in snake.get_positions().iter().skip(1) {
                fill_rect(
                    ctx,
                    (pos.0 * CELL_SIZE) as f64,
                    (pos.1 * CELL_SIZE) as f64,
                    CELL_SIZE as f64,
                    CELL_SIZE as f64,
                    "green",
                );
            }

            // Dibujar cabeza
            let head = snake.get_position();
            fill_rect(
                ctx,
                (head.0 * CELL_SIZE) as f64,
                (head.1 * CELL_SIZE) as f64,
                CELL_SIZE as f64,
                CELL_SIZE as f64,
                "darkgreen",
            );
        }

        // Dibujar bordes
        let size = CELL_SIZE as f64;
        let w = self.canvas_width as f64;
        let h = self.canvas_height as f64;

        fill_rect(ctx, 0.0, 0.0, w, size, "black");           // Top
        fill_rect(ctx, 0.0, h - size, w, size, "black");      // Bottom
        fill_rect(ctx, 0.0, 0.0, size, h, "black");           // Left
        fill_rect(ctx, w - size, 0.0, size, h, "black");      // Right

        // Mostrar game over
        if self.game_over {
            draw_text(
                ctx,
                &format!("Fin de la partida, puntuación: {}", self.score),
                (self.canvas_width / 3) as f64,
                (self.canvas_height / 2) as f64,
                "bold 20px serif",
                "black",
            );
        }
    }

    fn on_key(&mut self, key: &str) {
        if let Some(snake) = self.snake.as_mut() {
            let current_dir = snake.get_direction();
            let new_dir = match key {
                "d" | "ArrowRight" if current_dir != Direction::Left => Some(Direction::Right),
                "a" | "ArrowLeft" if current_dir != Direction::Right => Some(Direction::Left),
                "w" | "ArrowUp" if current_dir != Direction::Down => Some(Direction::Up),
                "s" | "ArrowDown" if current_dir != Direction::Up => Some(Direction::Down),
                _ => None,
            };

            if let Some(dir) = new_dir {
                snake.change_direction(dir);
            }
        }
    }

    fn frame_delay(&self) -> u32 {
        100
    }
}
