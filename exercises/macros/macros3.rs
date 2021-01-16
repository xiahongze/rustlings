// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// needed for use in the same crate
#[macro_use]
mod macros {
    /**
     * To use your macro_rules! macro from other crates,
     * the macro itself needs the attribute #[macro_export].
     * The importing crate can then import the macro via use crate_name::macro_name;.
     */
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
