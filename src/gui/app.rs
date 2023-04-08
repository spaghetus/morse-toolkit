use eframe::egui;

use crate::translations::TREE;

#[derive(Default)]
pub struct App {
	pub tree: crate::gui::tree::MorseTree,
	pub search_str: String,
}

impl eframe::App for App {
	fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			self.tree.show(ui);
			ui.separator();
			// String display
			{
				ui.label("Input a string, and see it visualized here.");
				ui.text_edit_singleline(&mut self.search_str);
				let mut morse_str = String::new();
				for c in self.search_str.chars() {
					if c == ' ' {
						morse_str.push_str("  ");
						continue;
					}
					if let Some(morse) = TREE.translate_into_symbols(c.to_ascii_uppercase()) {
						for sym in morse {
							match sym {
								crate::symbols::MorseSymbol::WordBoundary => {
									morse_str.push_str("  ");
								}
								crate::symbols::MorseSymbol::CharacterBoundary => {
									morse_str.push(' ');
								}
								crate::symbols::MorseSymbol::Dit => morse_str.push('.'),
								crate::symbols::MorseSymbol::Dah => morse_str.push('-'),
							}
						}
						morse_str.push(' ');
					} else {
						morse_str.push('?');
					}
				}
				ui.heading(morse_str)
			};
			// Map highlight
			if let Some(newest_character) = self
				.search_str
				.chars()
				.last()
				.and_then(|c| TREE.translate_into_symbols(c.to_ascii_uppercase()))
			{
				self.tree.current_sequence = newest_character;
			}
		});
	}
}
