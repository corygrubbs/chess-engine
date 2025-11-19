use std::io;
use std::io::BufRead;
mod uci;
mod engine;

use crate::uci::protocol::handle_command;
use crate::engine::core::Engine;

fn main() {
    let stdin = io::stdin();
    let mut engine = Engine::new();
    for line in stdin.lock().lines() {
        if let Ok(cmd) = line {
            handle_command(&cmd, &mut engine);
        }
    }
}