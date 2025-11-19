use crate::engine::options::Options;

pub struct Engine {
    pub options: Options
}

impl Engine {
    pub fn new() -> Engine {
        Engine {

            options: Options::default()
        }
    }

    pub fn set_debug(&mut self, cmd: &str) {
        let cmd_option: &str = cmd.split_whitespace().nth(1).unwrap();
        let mut debug: bool = false;
        if (cmd_option == "on") {
            debug = true
        }
        self.options.debug = debug;
    }

    pub fn search(&self, cmd: &str) {
        let cmd_options: Vec<&str> = cmd.split_whitespace().collect();
        
    }
}
