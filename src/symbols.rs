#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum MorseSymbol {
	WordBoundary,
	CharacterBoundary,
	Dit,
	Dah,
}

impl MorseSymbol {
	#[must_use]
	pub fn from_dots_and_dashes(from: &str) -> Vec<MorseSymbol> {
		from.split(' ')
			.flat_map(|section| {
				if section.is_empty() {
					vec![MorseSymbol::WordBoundary]
				} else {
					let mut out = vec![];
					for c in section.chars() {
						match c {
							'.' => out.push(MorseSymbol::Dit),
							'-' => out.push(MorseSymbol::Dah),
							_ => {}
						}
					}
					out.push(MorseSymbol::CharacterBoundary);
					out
				}
			})
			.collect()
	}
}

impl From<&MorseSymbol> for bool {
	fn from(value: &MorseSymbol) -> Self {
		match value {
			MorseSymbol::WordBoundary | MorseSymbol::CharacterBoundary => false,
			MorseSymbol::Dit | MorseSymbol::Dah => true,
		}
	}
}

impl From<&MorseSymbol> for u8 {
	fn from(value: &MorseSymbol) -> Self {
		match value {
			MorseSymbol::WordBoundary => 7,
			MorseSymbol::CharacterBoundary | MorseSymbol::Dah => 3,
			MorseSymbol::Dit => 1,
		}
	}
}

pub struct ToMorseIterator<I: Iterator<Item = bool>> {
	inner: std::iter::Peekable<I>,
	ending: bool,
}

pub trait IntoMorseIterator: Iterator<Item = bool> + Sized {
	fn into_morse(self) -> ToMorseIterator<Self> {
		ToMorseIterator {
			inner: self.peekable(),
			ending: false,
		}
	}
}

impl<I: Iterator<Item = bool>> IntoMorseIterator for I {}

impl<I: Iterator<Item = bool>> Iterator for ToMorseIterator<I> {
	type Item = MorseSymbol;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ending {
			return None;
		}
		let this_one = self.inner.next()?;
		let mut consecutive = 1;
		// Peek next,
		// If it is the same, consume and increment,
		// Otherwise, stop.
		loop {
			let orig = self.inner.peek();
			let next = orig.copied().unwrap_or(!this_one);
			if orig.is_none() {
				self.ending = true;
			}
			if next == this_one {
				consecutive += 1;
				self.inner.next();
			} else {
				break;
			}
		}
		match (this_one, consecutive) {
			(true, 1..=2) => Some(MorseSymbol::Dit),
			(true, 3..) => Some(MorseSymbol::Dah),
			(false, 5..) => Some(MorseSymbol::WordBoundary),
			(false, 2..=4) => Some(MorseSymbol::CharacterBoundary),
			_ => self.next(),
		}
	}
}

pub struct ToBoolsIterator<I: Iterator<Item = MorseSymbol>> {
	inner: std::iter::Peekable<I>,
	state: bool,
	counter: u8,
}

pub trait FromMorseIterator: Iterator<Item = MorseSymbol> + Sized {
	#[allow(clippy::wrong_self_convention)]
	fn from_morse(self) -> ToBoolsIterator<Self> {
		ToBoolsIterator {
			inner: self.peekable(),
			state: false,
			counter: 0,
		}
	}
}

impl<I: Iterator<Item = MorseSymbol>> FromMorseIterator for I {}

impl<I: Iterator<Item = MorseSymbol>> Iterator for ToBoolsIterator<I> {
	type Item = bool;

	fn next(&mut self) -> Option<Self::Item> {
		// If we are in the middle of writing a symbol...
		if self.counter > 0 {
			self.counter -= 1;
			return Some(self.state);
		}
		// We've finished writing a symbol, move onto the next one...
		let next = self.inner.next()?;
		let old_state = self.state;
		// Write new state info
		self.state = (&next).into();
		self.counter = (&next).into();
		// If we're between two truthy symbols, write a boundary.
		if old_state && self.state {
			return Some(false);
		}
		// Otherwise, move on.
		self.next()
	}
}

#[cfg(test)]
mod test {
	use arbitrary::{Arbitrary, Unstructured};
	use rand::{thread_rng, Rng};

	use super::{FromMorseIterator, IntoMorseIterator, MorseSymbol};

	#[test]
	fn arbitrary_valid_morse() {
		let mut rng = thread_rng();
		let noise: Vec<u8> = (0..u16::MAX).map(|_| rng.gen()).collect();
		let mut noise = Unstructured::new(&noise);

		let mut morse: Vec<MorseSymbol> = (0..rng.gen_range(10..20))
			.flat_map(|_| MorseSymbol::arbitrary(&mut noise))
			.fold(vec![], |mut acc, el| {
				if acc.is_empty() || (&acc[acc.len() - 1]).into() || (&el).into() {
					acc.push(el);
				}
				acc
			});
		while !bool::from(&morse[morse.len() - 1]) {
			morse.pop();
		}
		dbg!(&morse);
		let bits: Vec<_> = morse.iter().copied().from_morse().collect();
		let converted_morse: Vec<_> = dbg!(bits.iter().copied().into_morse().collect());
		assert_eq!(morse, converted_morse);
	}

	#[test]
	fn sos_into() {
		let sos = b"101010001110111011100010101";

		let morse: Vec<_> = sos.iter().map(|v| *v != b'0').into_morse().collect();

		assert_eq!(
			morse,
			[
				MorseSymbol::Dit,
				MorseSymbol::Dit,
				MorseSymbol::Dit,
				MorseSymbol::CharacterBoundary,
				MorseSymbol::Dah,
				MorseSymbol::Dah,
				MorseSymbol::Dah,
				MorseSymbol::CharacterBoundary,
				MorseSymbol::Dit,
				MorseSymbol::Dit,
				MorseSymbol::Dit,
			]
		);
	}
}
