use crate::engine::core::Engine;
use super::output_commands::{identify_engine, identify_author, uciok, ready};

pub fn handle_uci()
{
    identify_engine();
    identify_author();
    uciok();
}

pub fn handle_isready() {
    ready();
}

pub fn set_debug(cmd: &str, engine: &mut Engine) {
    let option: &str = cmd.split_whitespace().nth(1).unwrap();
    let mut debug: bool = false;
    if (option == "on") {
        debug = true
    }
    engine.options.debug = debug;
}

pub fn handle_go(cmd: &str, engine: &mut Engine) {

}