pub trait TerminalBackend {
    fn spawn_shell(&mut self);
    fn destroy_shell(&mut self);
    fn output_buf_mut(&mut self) -> &mut String;
}

#[derive(Debug, Default)]
pub struct Terminal(String);

impl TerminalBackend for Terminal {
    fn spawn_shell(&mut self) {}

    fn destroy_shell(&mut self) {}

    fn output_buf_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct FakeTerminalBackend {
        pub spawns: usize,
        pub destroys: usize,
        pub output_buf_accesses: usize,
        output_buf: String,
    }

    impl TerminalBackend for FakeTerminalBackend {
        fn spawn_shell(&mut self) {
            eprintln!("FakeTerminalBackend::spawn_shell() called");
            self.spawns += 1
        }

        fn destroy_shell(&mut self) {
            eprintln!("FakeTerminalBackend::destroy_shell() called");
            self.destroys += 1
        }

        fn output_buf_mut(&mut self) -> &mut String {
            eprintln!("FakeTerminalBackend::output_buf_mut() called");
            self.output_buf_accesses += 1;
            &mut self.output_buf
        }
    }
}
