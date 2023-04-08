use eframe::{run_native, NativeOptions};
use morse_toolkit::gui::app::App;

fn main() {
	run_native(
		"Morse Code Helper",
		NativeOptions::default(),
		Box::new(|_| Box::<App>::default()),
	)
	.expect("GUI crashed somehow");
}
