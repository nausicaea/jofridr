pub trait TerminalBackend {
    fn spawn_shell(&mut self);
    fn destroy_shell(&mut self);
    fn output_buffer_mut(&mut self) -> &mut String;
}

#[derive(Debug, Default)]
pub struct Terminal(String);

impl TerminalBackend for Terminal {
    fn spawn_shell(&mut self) {}

    fn destroy_shell(&mut self) {}

    fn output_buffer_mut(&mut self) -> &mut String {
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
        pub output_buffer_accesses: usize,
        output_buffer: String,
    }

    impl TerminalBackend for FakeTerminalBackend {
        fn spawn_shell(&mut self) {
            eprintln!("Spawn called!");
            self.spawns += 1
        }

        fn destroy_shell(&mut self) {
            eprintln!("Destroy called!");
            self.destroys += 1
        }

        fn output_buffer_mut(&mut self) -> &mut String {
            self.output_buffer_accesses += 1;
            &mut self.output_buffer
        }
    }
}
