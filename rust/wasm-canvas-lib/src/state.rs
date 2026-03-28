//! Macro para definir estado global thread-local con getters y setters

/// Macro que define una variable de estado global thread-local con getters/setters automáticos.
///
/// # Ejemplo
/// ```
/// define_state!(COUNTER, u32, 0);
/// // Genera: get_counter() -> u32, set_counter(value: u32)
/// ```
#[macro_export]
macro_rules! define_state {
    ($name:ident, $type:ty, $initial:expr) => {
        thread_local! {
            static $name: std::cell::RefCell<$type> = std::cell::RefCell::new($initial);
        }

        paste::paste! {
            #[allow(dead_code)]
            pub fn [<get_ $name:lower>]() -> $type {
                $name.with(|s| s.borrow().clone())
            }

            #[allow(dead_code)]
            pub fn [<set_ $name:lower>](value: $type) {
                $name.with(|s| *s.borrow_mut() = value);
            }
        }
    };
}
