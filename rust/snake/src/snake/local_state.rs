use crate::define_state;

use std::cell::RefCell;
use crate::snake::structures::Snake;

define_state!(CANVAS_WIDTH, u32, 0);
define_state!(CANVAS_HEIGHT, u32, 0);
define_state!(SNAKE, Option<Snake>, None);
define_state!(EXIT, bool, false);

define_state!(MAX_X, u32, 0);
define_state!(MAX_Y, u32, 0);

define_state!(MANZANA, (u32, u32), (0, 0));
define_state!(PUNTOS, u32, 0);