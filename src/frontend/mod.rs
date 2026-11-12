pub mod terminal;

pub trait WindowedApp: eframe::App {
    fn on_open(&mut self, _open: bool) {}
}
