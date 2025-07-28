use crate::environment::Environment;

pub mod arithmetic;
pub mod lists;
pub mod logic;
pub mod io;

pub fn register_all(env: &mut Environment) {
    arithmetic::register(env);
    lists::register(env);
    logic::register(env);
    io::register(env);
}