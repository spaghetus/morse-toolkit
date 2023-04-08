use eframe::{
	egui::{RichText, Sense, Ui},
	epaint::{Color32, Stroke, Vec2},
};

use crate::{
	symbols::MorseSymbol,
	translations::{TranslationLeaf, TREE},
};

#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct MorseTree {
	pub current_sequence: Vec<MorseSymbol>,
}

impl MorseTree {
	pub fn show(&self, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.label("Dits are left and red, dahs are right and blue.");
			self.show_leaf(ui, &TREE, &[]);
		});
	}

	fn show_leaf(&self, ui: &mut Ui, leaf: &TranslationLeaf, path: &[MorseSymbol]) {
		let highlight =
			!self.current_sequence.is_empty() && self.current_sequence.starts_with(path);
		let next_symbol = if highlight {
			self.current_sequence.get(path.len())
		} else {
			None
		};

		ui.label(RichText::new(leaf.value.to_string()).color(if highlight {
			Color32::GREEN
		} else {
			Color32::WHITE
		}));
		// Render arrows
		{
			let (res, painter) =
				ui.allocate_painter(Vec2::new(ui.available_width(), 10.0), Sense::hover());
			let left = res.rect.left();
			let width = res.rect.width();
			if leaf.dit.is_some() {
				let color = if next_symbol == Some(&MorseSymbol::Dit) {
					Color32::GREEN
				} else {
					Color32::DARK_RED
				};
				painter.hline(
					(left + width / 4.0)..=(left + width / 2.0),
					res.rect.top(),
					Stroke::new(8.0, color),
				);
				painter.vline(
					left + width / 4.0,
					res.rect.bottom()..=res.rect.top(),
					Stroke::new(1.0, color),
				);
			}
			if leaf.dah.is_some() {
				let color = if next_symbol == Some(&MorseSymbol::Dah) {
					Color32::GREEN
				} else {
					Color32::DARK_BLUE
				};
				painter.hline(
					(left + width / 2.0)..=(left + 3.0 * width / 4.0),
					res.rect.top(),
					Stroke::new(8.0, color),
				);
				painter.vline(
					left + 3.0 * width / 4.0,
					res.rect.bottom()..=res.rect.top(),
					Stroke::new(1.0, color),
				);
			}
		}
		ui.horizontal(|ui| {
			ui.columns(2, |uis| {
				if let [dit, dah] = uis {
					// Dit side
					dit.vertical_centered(|ui| {
						if let Some(dit) = (*leaf.dit).as_ref() {
							let mut path = path.to_vec();
							path.push(MorseSymbol::Dit);
							self.show_leaf(ui, dit, &path);
						}
					});
					// Dah side
					dah.vertical_centered(|ui| {
						if let Some(dah) = (*leaf.dah).as_ref() {
							let mut path = path.to_vec();
							path.push(MorseSymbol::Dah);
							self.show_leaf(ui, dah, &path);
						}
					});
				}
			});
		});
	}
}
