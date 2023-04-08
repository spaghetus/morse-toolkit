use lazy_static::lazy_static;
use serde::Deserialize;

use crate::symbols::MorseSymbol;

const TREE_STR: &str = include_str!("../tree.xml");

lazy_static! {
	pub static ref TREE: TranslationLeaf = serde_xml_rs::from_str(TREE_STR).unwrap();
}

#[test]
fn print_deser_tree() {
	let tree = TREE.clone();
	dbg!(tree);
}

#[derive(Deserialize, Debug, Clone)]
pub struct TranslationLeaf {
	pub value: char,
	pub dit: Box<Option<TranslationLeaf>>,
	pub dah: Box<Option<TranslationLeaf>>,
}

impl TranslationLeaf {
	#[must_use]
	pub fn max_length(&self) -> usize {
		let dit = (*self.dit).as_ref().map_or(0, TranslationLeaf::max_length);
		let dah = (*self.dah).as_ref().map_or(0, TranslationLeaf::max_length);
		dit.max(dah)
	}

	#[must_use]
	pub fn translate_into_symbols(&self, from: char) -> Option<Vec<MorseSymbol>> {
		if from == self.value {
			Some(vec![])
		} else if let Some(mut into) = (*self.dit)
			.as_ref()
			.and_then(|dit| dit.translate_into_symbols(from))
		{
			let mut out = vec![MorseSymbol::Dit];
			out.append(&mut into);
			Some(out)
		} else if let Some(mut into) = (*self.dah)
			.as_ref()
			.and_then(|dah| dah.translate_into_symbols(from))
		{
			let mut out = vec![MorseSymbol::Dah];
			out.append(&mut into);
			Some(out)
		} else {
			None
		}
	}

	#[must_use]
	pub fn translate_from_symbols(&self, from: &[MorseSymbol]) -> Option<char> {
		if from.is_empty() {
			return Some(self.value);
		}

		match from[0] {
			MorseSymbol::Dit => (*self.dit)
				.as_ref()
				.and_then(|dit| dit.translate_from_symbols(&from[1..])),
			MorseSymbol::Dah => (*self.dah)
				.as_ref()
				.and_then(|dah| dah.translate_from_symbols(&from[1..])),
			_ => None,
		}
	}
}
