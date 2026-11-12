use std::sync::atomic::{AtomicU16, Ordering};

use crate::backend::terminal::TerminalBackend;

static INSTANCE_COUNT: AtomicU16 = AtomicU16::new(0);

pub struct Terminal<T> {
    id: egui::Id,
    term: T,
    init: bool,
}

impl<T: TerminalBackend> Terminal<T> {
    pub fn new(backend: T) -> Self {
        Self {
            id: egui::Id::new(format!(
                "terminal_{}",
                INSTANCE_COUNT.fetch_add(1, Ordering::Relaxed)
            )),
            term: backend,
            init: false,
        }
    }
}

impl<T: TerminalBackend> super::WindowedApp for Terminal<T> {
    fn on_open(&mut self, open: bool) {
        if open {
            self.term.spawn_shell();
        } else {
            self.term.destroy_shell();
        }
    }
}

impl<T: TerminalBackend> eframe::App for Terminal<T> {
    fn on_exit(&mut self) {
        self.term.destroy_shell();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if !self.init {
            self.init = true;
            self.term.spawn_shell();
        }

        egui::ScrollArea::both().show(ui, |ui| {
            ui.add_sized(ui.available_size(), |ui: &mut egui::Ui| {
                let output = egui::TextEdit::multiline(self.term.output_buf_mut())
                    .font(egui::TextStyle::Monospace)
                    .frame(egui::Frame::NONE)
                    .code_editor()
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .id(self.id)
                    .show(ui);
                output.response.response
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::terminal::tests::FakeTerminalBackend;

    use super::*;
    use egui_kittest::Harness;
use rstest::{fixture, rstest};

    #[fixture]
    fn ft() -> Terminal<FakeTerminalBackend> {
        Terminal::new(FakeTerminalBackend::default())
    }

    #[fixture]
    fn harness<'a>(ft: Terminal<FakeTerminalBackend>) -> Harness<'a, Terminal<FakeTerminalBackend>> {
        Harness::builder()
            .with_size(egui::vec2(300.0, 200.0))
            .build_eframe(|_cc| ft)
    }

    #[rstest]
    fn terminal_on_open_true_invokes_spawn_shell(mut ft: Terminal<FakeTerminalBackend>) {
        use crate::frontend::WindowedApp;

        ft.on_open(true);
        assert_eq!(1, ft.term.spawns);
        assert_eq!(0, ft.term.destroys);
    }

    #[rstest]
    fn terminal_on_open_false_invokes_destroy_shell(mut ft: Terminal<FakeTerminalBackend>) {
        use crate::frontend::WindowedApp;

        ft.on_open(false);
        assert_eq!(0, ft.term.spawns);
        assert_eq!(1, ft.term.destroys);
    }

    #[rstest]
    fn terminal_on_exit_invokes_destroy_shell(mut ft: Terminal<FakeTerminalBackend>) {
        use eframe::App;

        ft.on_exit();
        assert_eq!(0, ft.term.spawns);
        assert_eq!(1, ft.term.destroys);
    }

    #[rstest]
    fn terminal_ui_invokes_spawn_shell_on_first_call(mut harness: Harness<Terminal<FakeTerminalBackend>>) {
        // This calls Terminal::ui at least once.
        eprintln!("After harness creation");
        assert!(harness.state().init);
        assert_eq!(1, harness.state().term.spawns);
        assert_eq!(0, harness.state().term.destroys);

        // Subsequent rendering calls.
        eprintln!("Before additional calls to Terminal::ui");
        harness.run();
        harness.run();
        assert_eq!(1, harness.state().term.spawns);
        assert_eq!(0, harness.state().term.destroys);
    }

    #[rstest]
    fn terminal_ui_invokes_output_buf_mut_on_every_call(mut harness: Harness<Terminal<FakeTerminalBackend>>) {
        eprintln!("After harness creation");
        assert_eq!(3, harness.state().term.output_buf_accesses);

        eprintln!("Before additional calls to Terminal::ui");
        harness.run();
        harness.run();
        assert_eq!(5, harness.state().term.output_buf_accesses);
    }
}
