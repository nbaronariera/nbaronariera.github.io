use std::cell::RefCell;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlCanvasElement};

#[macro_export]
macro_rules! define_state {
    ($name:ident, $type:ty, $initial:expr) => {
        thread_local! {
            static $name: RefCell<$type> = RefCell::new($initial);
        }

        paste::paste! {
            pub fn [<get_ $name:lower>]() -> $type {
                $name.with(|s| s.borrow().clone())
            }

            pub fn [<set_ $name:lower>](value: $type) {
                $name.with(|s| *s.borrow_mut() = value);
            }
        }
    };
}

define_state!(RUNNING, bool, false);
define_state!(MINIMIZE, bool, true);
define_state!(LAUNCHED, bool, false);
define_state!(PLAYPAUSE, Option<Element>, None);
define_state!(CANVAS, Option<HtmlCanvasElement>, None);
define_state!(START, Option<fn() -> Result<(), JsValue>>, None);
define_state!(INPUT, Option<fn(String) -> ()>, None);
