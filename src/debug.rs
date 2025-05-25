// Debug module for REPLisp
// Contains debug-related functionality

// Global debug flag
pub static mut DEBUG: bool = false;

// Helper function to safely check if debug mode is enabled
pub fn is_debug_enabled() -> bool {
    unsafe { DEBUG }
}

// Helper function to safely print debug messages with formatting
pub fn debug_print_fmt(fmt: std::fmt::Arguments<'_>) {
    if is_debug_enabled() {
        print!("DEBUG: ");
        std::io::Write::write_fmt(&mut std::io::stdout(), fmt).unwrap();
        println!();
    }
}

// Macro for debug printing with format
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        crate::debug::debug_print_fmt(format_args!($($arg)*));
    };
}
