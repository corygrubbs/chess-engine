use crate::engine::core::Engine;
use super::input_commands::{handle_uci, handle_isready, set_debug};

pub fn handle_command(cmd: &str, engine: &mut Engine) {
    if cmd == "uci" {
        handle_uci();
    }
    else if cmd == "isready" {
        handle_isready()
    }
    else if cmd.starts_with("debug")
    {
        engine.set_debug(cmd);
    }
}