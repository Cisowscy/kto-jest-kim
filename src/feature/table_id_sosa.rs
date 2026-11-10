// ./src/feature/table_id_sosa.rs

// Tryb wejścia sosy (pojedyncza / lista / zakres) oraz pula kolumn tabeli.
// Pula to lista z README w `id_sosa.rs`: każda kolumna to jedna ścieżka z tego README.
// Korzysta z niej UI (`template::kim_jest_antenat`) i przykład (`examples/e_sosa.rs`).

use std::ops::RangeInclusive;

use crate::feature::id_sosa::{DualRadixPair, LineCode, PersonEntry, PersonEntryFrom, RadixDec, RadixFormat};

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// SosaEntryMode
//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

/// Właścicielska wersja `PersonEntryFrom`. Stan UI musi trzymać dane między klatkami,
/// a `PersonEntryFrom` tylko je pożycza. Warianty są te same.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SosaEntryMode {
	Given(RadixDec),
	Array(Vec<RadixDec>),
	Range(RangeInclusive<RadixDec>),
}

impl SosaEntryMode {
	/// Widok na `PersonEntryFrom` bez kopiowania danych.
	pub fn as_from(&self) -> PersonEntryFrom<'_> {
		match self {
			Self::Given(sosa) => PersonEntryFrom::Given(*sosa),
			Self::Array(arr) => PersonEntryFrom::Array(arr.as_slice()),
			Self::Range(range) => PersonEntryFrom::Range(range.clone()),
		}
	}

	/// Same numery sosa, zera pominięte. Nie buduje `PersonEntry`, więc jest tani.
	pub fn numbers(&self) -> impl Iterator<Item = RadixDec> + '_ {
		let it: Box<dyn Iterator<Item = RadixDec> + '_> = match self {
			Self::Given(sosa) => Box::new(std::iter::once(*sosa)),
			Self::Array(arr) => Box::new(arr.iter().copied()),
			Self::Range(range) => Box::new(range.clone()),
		};
		it.filter(|&sosa| sosa > 0)
	}
}

/// Parsuje tekst w stylu `1, 4, 7, 0` (przecinki, średniki lub spacje).
/// Przy błędzie zwraca komunikat o pierwszym błędnym fragmencie.
pub fn parsuj_liste(tekst: &str) -> Result<Vec<RadixDec>, String> {
	tekst
		.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
		.filter(|t| !t.is_empty())
		.map(|t| t.parse::<RadixDec>().map_err(|_| format!("„{t}” nie jest liczbą")))
		.collect()
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// Kolumny
//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

/// `DualRadixPair` jako kod linii (np. `12⁝3`). Trait `LineCode` jest tylko dla krotki,
/// a README woła `lines.dad.get_code()`.
impl LineCode for DualRadixPair {
	fn get_code(&self) -> String {
		self.dec.get_code()
	}
}

/// Jedna kolumna z puli: nazwa (ścieżka z README) i sposób wyliczenia tekstu komórki.
#[derive(Clone, Copy)]
pub struct Kolumna {
	pub nazwa: &'static str,
	wartosc: fn(&PersonEntry) -> String,
}

impl Kolumna {
	pub fn tekst(&self, e: &PersonEntry) -> String {
		(self.wartosc)(e)
	}

	/// Grupa w oknie wyboru: pierwszy segment nazwy (`sosa`, `index`, `lines`).
	pub fn grupa(&self) -> &'static str {
		self.nazwa.split('.').next().unwrap_or("")
	}
}

/// Każda pozycja to jedna ścieżka z kodu. Z zapisu `sosa.ego.gennum.dec.0.get_text`
/// powstaje kolumna o nazwie `sosa.ego.gennum.dec.0.get_text` i wartości `e.sosa.ego.gennum.dec.0.get_text()`.
/// Ostatni segment to metoda. `get_char()` zwraca `&str`, więc wynik idzie przez `to_string()`.
macro_rules! pula_kolumn {
	($($pierwszy:tt $(. $czesc:tt)*),* $(,)?) => {
		vec![
			$(
				Kolumna {
					nazwa: concat!(stringify!($pierwszy) $(, ".", stringify!($czesc))*),
					wartosc: |e| (e.$pierwszy $(. $czesc)*()).to_string(),
				}
			),*
		]
	};
}

/// Pula wszystkich kolumn: pola z `id_sosa.rs`, w tej kolejności.
pub fn pula() -> Vec<Kolumna> {
	pula_kolumn!(
		// sosa.<ego|dad|mom|kid>
		sosa.ego.normal.get_text,
		sosa.ego.normal.dec.get_text,
		sosa.ego.normal.bin.get_text,
		sosa.ego.revers.get_text,
		sosa.ego.revers.dec.get_text,
		sosa.ego.revers.bin.get_text,
		sosa.ego.gennum.get_text,
		sosa.ego.gennum.dec.0.get_text,
		sosa.ego.gennum.bin.0.get_text,
		sosa.ego.gennum.dec.1.get_text,
		sosa.ego.gennum.bin.1.get_text,
		sosa.ego.gender.get_char,
		sosa.dad.normal.get_text,
		sosa.dad.normal.dec.get_text,
		sosa.dad.normal.bin.get_text,
		sosa.dad.revers.get_text,
		sosa.dad.revers.dec.get_text,
		sosa.dad.revers.bin.get_text,
		sosa.dad.gennum.get_text,
		sosa.dad.gennum.dec.0.get_text,
		sosa.dad.gennum.bin.0.get_text,
		sosa.dad.gennum.dec.1.get_text,
		sosa.dad.gennum.bin.1.get_text,
		sosa.dad.gender.get_char,
		sosa.mom.normal.get_text,
		sosa.mom.normal.dec.get_text,
		sosa.mom.normal.bin.get_text,
		sosa.mom.revers.get_text,
		sosa.mom.revers.dec.get_text,
		sosa.mom.revers.bin.get_text,
		sosa.mom.gennum.get_text,
		sosa.mom.gennum.dec.0.get_text,
		sosa.mom.gennum.bin.0.get_text,
		sosa.mom.gennum.dec.1.get_text,
		sosa.mom.gennum.bin.1.get_text,
		sosa.mom.gender.get_char,
		sosa.kid.normal.get_text,
		sosa.kid.normal.dec.get_text,
		sosa.kid.normal.bin.get_text,
		sosa.kid.revers.get_text,
		sosa.kid.revers.dec.get_text,
		sosa.kid.revers.bin.get_text,
		sosa.kid.gennum.get_text,
		sosa.kid.gennum.dec.0.get_text,
		sosa.kid.gennum.bin.0.get_text,
		sosa.kid.gennum.dec.1.get_text,
		sosa.kid.gennum.bin.1.get_text,
		sosa.kid.gender.get_char,
		// index.from0 / index.from1
		index.from0.normal.get_text,
		index.from0.normal.dec.get_text,
		index.from0.normal.bin.get_text,
		index.from0.revers.get_text,
		index.from0.revers.dec.get_text,
		index.from0.revers.bin.get_text,
		index.from0.gennum.get_text,
		index.from0.gennum.dec.get_text,
		index.from0.gennum.bin.get_text,
		index.from0.gender.get_char,
		index.from1.normal.get_text,
		index.from1.normal.dec.get_text,
		index.from1.normal.bin.get_text,
		index.from1.revers.get_text,
		index.from1.revers.dec.get_text,
		index.from1.revers.bin.get_text,
		index.from1.gennum.get_text,
		index.from1.gennum.dec.get_text,
		index.from1.gennum.bin.get_text,
		index.from1.gender.get_char,
		// index.limit
		index.limit.min.get_text,
		index.limit.min.dec.get_text,
		index.limit.min.bin.get_text,
		index.limit.max.get_text,
		index.limit.max.dec.get_text,
		index.limit.max.bin.get_text,
		// lines.dad / lines.mom
		lines.dad.get_code,
		lines.dad.get_text,
		lines.dad.dec.0.get_text,
		lines.dad.dec.1.get_text,
		lines.dad.bin.0.get_text,
		lines.dad.bin.1.get_text,
		lines.generation_where_start_dad_line.get_text,
		lines.generation_where_start_dad_line.dec.get_text,
		lines.generation_where_start_dad_line.bin.get_text,
		lines.auxiliary_index_within_dad_line.get_text,
		lines.auxiliary_index_within_dad_line.dec.get_text,
		lines.auxiliary_index_within_dad_line.bin.get_text,
		lines.mom.get_code,
		lines.mom.get_text,
		lines.mom.dec.0.get_text,
		lines.mom.dec.1.get_text,
		lines.mom.bin.0.get_text,
		lines.mom.bin.1.get_text,
		lines.generation_where_start_mom_line.get_text,
		lines.generation_where_start_mom_line.dec.get_text,
		lines.generation_where_start_mom_line.bin.get_text,
		lines.auxiliary_index_within_mom_line.get_text,
		lines.auxiliary_index_within_mom_line.dec.get_text,
		lines.auxiliary_index_within_mom_line.bin.get_text,
	)
}

/// Kolumny z puli, których nazwy są podane. Kolejność jak w puli, nie jak w `nazwy`.
pub fn wybrane(nazwy: &[&str]) -> Vec<Kolumna> {
	pula().into_iter().filter(|k| nazwy.contains(&k.nazwa)).collect()
}

/// Domyślny zestaw, odpowiadający nagłówkowi z `e_sosa.rs`.
pub fn domyslne() -> Vec<&'static str> {
	vec![
		"sosa.ego.gender.get_char",
		"sosa.ego.gennum.dec.0.get_text",
		"sosa.ego.normal.dec.get_text",
		"lines.dad.get_code",
		"lines.mom.get_code",
		"sosa.dad.normal.dec.get_text",
		"sosa.mom.normal.dec.get_text",
		"sosa.kid.normal.dec.get_text",
	]
}

#[cfg(test)]
mod testy {
	use super::*;

	#[test]
	fn domyslne_istnieja_w_puli() {
		let nazwy: Vec<&str> = pula().iter().map(|k| k.nazwa).collect();
		for d in domyslne() {
			assert!(nazwy.contains(&d), "brak kolumny w puli: {d}");
		}
	}
}
