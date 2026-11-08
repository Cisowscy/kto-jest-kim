// // src/feature/id_sosa.rs

use std::ops::{Index, RangeInclusive};

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonEntryFrom<'a> {
	Given(RadixDec),
	Array(&'a [RadixDec]),
	Range(RangeInclusive<RadixDec>),
}

impl<'a> PersonEntryFrom<'a> {
	/// Iterator po `PersonEntry`; sosa równe 0 są pomijane.
	pub fn entries(self) -> impl Iterator<Item = PersonEntry> + 'a {
		let sosas: Box<dyn Iterator<Item = RadixDec> + 'a> = match self {
			Self::Given(sosa) => Box::new(std::iter::once(sosa)),
			Self::Array(arr) => Box::new(arr.iter().copied()),
			Self::Range(range) => Box::new(range),
		};
		sosas.filter(|&sosa| sosa > 0).map(PersonEntry::from_sosa)
	}

	/// Pobiera pierwszy poprawny rekord (działa spójnie dla Given, pierwszego elementu Array oraz Range).
	pub fn to_entry(&self) -> Option<PersonEntry> {
		match *self {
			PersonEntryFrom::Given(sosa) if sosa > 0 => Some(PersonEntry::from_sosa(sosa)),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonEntry {
	pub sosa: Sosa,
	pub generation: GenerationOfSosa,
	pub min_max: GenerationLimitOfSosa,
	pub index: GenerationIndex,
	pub lines: LinesOfSosa,
}

impl PersonEntry {
	pub fn from_sosa(sosa: RadixDec) -> Self {
		Self {
			sosa: Sosa::from(sosa),
			generation: GenerationOfSosa::from(sosa),
			min_max: GenerationLimitOfSosa::from(sosa),
			index: GenerationIndex::from(sosa),
			lines: LinesOfSosa::from(sosa),
		}
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexingMode {
	From0, // 0, 1, 2, 3...
	From1, // 1, 2, 3, 4...
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sosa {
	pub ego: (DualRadix, DualRadix, SosaSex),
	pub dad: (DualRadix, DualRadix, SosaSex),
	pub mom: (DualRadix, DualRadix, SosaSex),
	pub kid: (DualRadix, DualRadix, SosaSex),
}

impl From<RadixDec> for Sosa {
	fn from(sosa: RadixDec) -> Self {
		let ego_sex = SosaSex::from(sosa);
		let dad_sex = SosaSex::from(sosa * 2);
		let mom_sex = SosaSex::from(sosa * 2 + 1);
		let kid_sex = SosaSex::from(sosa / 2);

		let limit = GenerationLimitOfSosa::from(sosa);
		let order = GenerationIndex::from(sosa);

		let ego_nor = DualRadix::from(sosa);
		let ego_rev = DualRadix::from(limit.max.dec - order.from0_normal.dec);
		let dad_nor = DualRadix::from(ego_nor.dec * 2);
		let dad_rev = DualRadix::from(ego_rev.dec * 2 + 1);
		let mom_nor = DualRadix::from(ego_nor.dec * 2 + 1);
		let mom_rev = DualRadix::from(ego_rev.dec * 2);
		let kid_nor = DualRadix::from((ego_nor.dec / 2) as RadixDec);
		let kid_rev = DualRadix::from((ego_rev.dec / 2) as RadixDec);

		Self {
			ego: (ego_nor, ego_rev, ego_sex),
			dad: (dad_nor, dad_rev, dad_sex),
			mom: (mom_nor, mom_rev, mom_sex),
			kid: (kid_nor, kid_rev, kid_sex),
		}
	}
}

impl From<RadixBin> for Sosa {
	fn from(bin: RadixBin) -> Self {
		Self::from(RadixDec::from_str_radix(&bin, 2).unwrap())
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

// TODO: rozważyć użycie np bitvec::BitVec zamiast String dla RadixBin, aby uniknąć niepotrzebnych konwersji i operacji na stringach.
pub type RadixBin = String;
pub type RadixDec = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadixDecList(Vec<RadixDec>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadixBinList(Vec<RadixBin>);

impl Index<usize> for RadixDecList {
	type Output = RadixDec;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl Index<usize> for RadixBinList {
	type Output = RadixBin;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}
//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualRadix {
	pub dec: RadixDec,
	pub bin: RadixBin,
}
impl From<RadixDec> for DualRadix {
	fn from(dec: RadixDec) -> Self {
		Self { dec, bin: format!("{dec:b}") }
	}
}
impl From<RadixBin> for DualRadix {
	fn from(bin: RadixBin) -> Self {
		let dec = RadixDec::from_str_radix(&bin, 2).unwrap();

		Self { dec, bin }
	}
}
impl DualRadix {
	pub fn end_zeros_count(a: &str) -> RadixDec {
		let mut r = 0;
		let mut t = true;
		let e: Vec<char> = a.chars().collect();
		let mut l = e.len();
		while l > 0 && t {
			if e[l - 1] == '0' {
				r += 1;
			} else {
				t = false;
			}
			l -= 1;
		}
		r
	}
	pub fn end_zeros_clear(a: &str) -> RadixDec {
		let c = a.trim_end_matches('0');
		let r = RadixDec::from_str_radix(c, 2).unwrap();
		/* TODO: Zastanowić się nad tym, czy nie lepiej byłoby zwracać Option<u32> i w przypadku błędu zwracać None, zamiast panikować.
		 * lub zamienić na `if c.is_empty() { 0 } else { RadixDec::from_str_radix(c, 2).unwrap() }`
		 */
		r
	}
}
impl std::fmt::Display for DualRadix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\x1b[38;2;255;215;0m{}₍₁₀₎\x1b[0m · \x1b[38;2;165;102;42m{}₍₂₎\x1b[0m", self.dec, self.bin,)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualRadixPair {
	pub dec: (RadixDec, RadixDec),
	pub bin: (RadixBin, RadixBin),
}
impl From<(RadixDec, RadixDec)> for DualRadixPair {
	fn from((dec1, dec2): (RadixDec, RadixDec)) -> Self {
		Self { dec: (dec1, dec2), bin: (format!("{dec1:b}"), format!("{dec2:b}")) }
	}
}
impl From<(RadixBin, RadixBin)> for DualRadixPair {
	fn from((bin1, bin2): (RadixBin, RadixBin)) -> Self {
		Self {
			dec: (RadixDec::from_str_radix(&bin1, 2).unwrap(), RadixDec::from_str_radix(&bin2, 2).unwrap()),
			bin: (bin1, bin2),
		}
	}
}
impl From<(DualRadix, DualRadix)> for DualRadixPair {
	fn from((a, b): (DualRadix, DualRadix)) -> Self {
		Self { dec: (a.dec, b.dec), bin: (a.bin, b.bin) }
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualRadixList {
	pub dec: RadixDecList,
	pub bin: RadixBinList,
}

impl DualRadixList {
	pub fn new() -> Self {
		Self { dec: RadixDecList(Vec::new()), bin: RadixBinList(Vec::new()) }
	}

	pub fn len(&self) -> usize {
		self.dec.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.dec.0.is_empty()
	}

	pub fn push(&mut self, value: DualRadix) {
		self.dec.0.push(value.dec);
		self.bin.0.push(value.bin);
	}

	pub fn insert(&mut self, index: usize, value: DualRadix) {
		self.dec.0.insert(index, value.dec);
		self.bin.0.insert(index, value.bin);
	}

	pub fn remove(&mut self, index: usize) -> DualRadix {
		DualRadix { dec: self.dec.0.remove(index), bin: self.bin.0.remove(index) }
	}

	pub fn pop(&mut self) -> Option<DualRadix> {
		Some(DualRadix { dec: self.dec.0.pop()?, bin: self.bin.0.pop()? })
	}

	pub fn clear(&mut self) {
		self.dec.0.clear();
		self.bin.0.clear();
	}
}

impl Default for DualRadixList {
	fn default() -> Self {
		Self::new()
	}
}
//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum SosaSex {
	MaleOrFemale,
	Male,
	Female,
	MaleAndFemale,
}

impl From<RadixDec> for SosaSex {
	fn from(sosa: RadixDec) -> Self {
		if sosa < 2 {
			Self::MaleOrFemale
		//} else if sosa % 2 == 0 {
		} else if sosa.is_multiple_of(2) {
			Self::Male
		} else {
			Self::Female
		}
	}
}
impl From<RadixBin> for SosaSex {
	fn from(bin: RadixBin) -> Self {
		Self::from(RadixDec::from_str_radix(&bin, 2).unwrap())
	}
}
impl SosaSex {
	/// Zwraca znak płci w formacie Unicode.
	pub fn get_char(self) -> &'static str {
		match self {
			Self::MaleOrFemale => "⚥",
			Self::Male => "♂",
			Self::Female => "♀",
			Self::MaleAndFemale => "⚤",
		}
	}
}
impl std::fmt::Display for SosaSex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::MaleOrFemale => "⚥",
			Self::Male => "♂",
			Self::Female => "♀",
			Self::MaleAndFemale => "⚤",
		})
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationOfSosa {
	pub from0: DualRadix,
	pub from1: DualRadix,
}

impl From<RadixDec> for GenerationOfSosa {
	fn from(sosa: RadixDec) -> Self {
		// Pokolenie liczone od 0 (Ego = 0, Rodzice = 1, Dziadkowie = 2...)
		// wersja poprzednia: let g0 = (sosa as f64).log2().floor() as RadixDec;
		let g0 = sosa.ilog2() as RadixDec;
		// Pokolenie liczone od 1 (Ego = 1, Rodzice = 2, Dziadkowie = 3...)
		let g1 = g0 + 1;
		Self { from0: DualRadix::from(g0), from1: DualRadix::from(g1) }
	}
}

impl From<RadixBin> for GenerationOfSosa {
	fn from(bin: RadixBin) -> Self {
		Self::from(RadixDec::from_str_radix(&bin, 2).unwrap())
	}
}

impl GenerationOfSosa {
	pub fn sosa_minmax_in_generation(&self, generation_mode: IndexingMode) -> (DualRadix, DualRadix) {
		let generation = match generation_mode {
			IndexingMode::From0 => self.from0.dec,
			IndexingMode::From1 => self.from1.dec,
		};

		match generation_mode {
			IndexingMode::From0 => {
				if generation == 0 {
					// Bezpiecznik: pierwsze pokolenie ma zakres 1–1.
					(DualRadix::from(1), DualRadix::from(1))
				} else {
					(
						DualRadix::from(2_u64.pow(generation as u32)),
						DualRadix::from(2_u64.pow(generation as u32 + 1) - 1),
					)
				}
			}
			IndexingMode::From1 => {
				if generation == 0 {
					// Bezpiecznik: wartość poza standardową numeracją pokoleń.
					(DualRadix::from(0), DualRadix::from(0))
				} else {
					(
						DualRadix::from(2_u64.pow((generation - 1) as u32)),
						DualRadix::from(2_u64.pow(generation as u32) - 1),
					)
				}
			}
		}
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationLimitOfSosa {
	pub min: DualRadix,
	pub max: DualRadix,
}

impl From<RadixDec> for GenerationLimitOfSosa {
	fn from(sosa: RadixDec) -> Self {
		let generation = (sosa as f64).log2().floor() as u32;

		Self { min: DualRadix::from(2_u64.pow(generation)), max: DualRadix::from(2_u64.pow(generation + 1) - 1) }
	}
}

impl From<RadixBin> for GenerationLimitOfSosa {
	fn from(bin: RadixBin) -> Self {
		Self::from(RadixDec::from_str_radix(&bin, 2).unwrap())
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationIndex {
	pub from0_normal: DualRadix,
	pub from1_normal: DualRadix,
	pub from0_revers: DualRadix,
	pub from1_revers: DualRadix,
}

impl From<RadixDec> for GenerationIndex {
	fn from(sosa: RadixDec) -> Self {
		let limit = GenerationLimitOfSosa::from(sosa);
		let from1_normal = DualRadix::from(sosa + 1 - limit.min.dec);
		let from0_normal = DualRadix::from(from1_normal.dec - 1);
		let from1_revers = DualRadix::from(limit.max.dec + 1 - sosa);
		let from0_revers = DualRadix::from(from1_revers.dec - 1);

		Self { from0_normal, from1_normal, from0_revers, from1_revers }
	}
}
impl From<RadixBin> for GenerationIndex {
	fn from(bin: RadixBin) -> Self {
		Self::from(RadixDec::from_str_radix(&bin, 2).unwrap())
	}
}
//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineOfSosa {
	pub line_and_index: DualRadixPair,
	pub generation_where_start_line: DualRadix,
	pub auxiliary_index_within_line: DualRadix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinesOfSosa {
	pub dad: DualRadixPair,
	pub mom: DualRadixPair,
	pub generation_where_start_dad_line: DualRadix,
	pub generation_where_start_mom_line: DualRadix,
	pub auxiliary_index_within_dad_line: DualRadix,
	pub auxiliary_index_within_mom_line: DualRadix,
}
impl LinesOfSosa {
	fn from_sosa_get_line(sosa: RadixDec, line_mom: bool) -> LineOfSosa {
		let order = GenerationIndex::from(sosa);
		let limit = GenerationLimitOfSosa::from(sosa);
		let level = GenerationOfSosa::from(sosa).from1;

		let sosa_ego_nor = DualRadix::from(sosa);
		let sosa_ego_rev = DualRadix::from(limit.max.dec - order.from0_normal.dec);

		let (indexing_from1, indexing_from0, ego_sosa) = if line_mom {
			(order.from1_revers, order.from0_revers, &sosa_ego_rev)
		} else {
			(order.from1_normal, order.from0_normal, &sosa_ego_nor)
		};

		let line_sort = DualRadix::from(if indexing_from1.dec == 1 {
			level.dec
		} else {
			DualRadix::end_zeros_count(&indexing_from0.bin) + 1
		});
		// -----------------------------------------------------------------------------------------
		let line_tier = DualRadix::from(DualRadix::end_zeros_clear(&ego_sosa.bin));
		let line_name = DualRadix::from(line_tier.dec + if line_mom { 2 } else { 1 });
		let line_gene = DualRadix::from(line_tier.bin.len() as RadixDec);
		// -----------------------------------------------------------------------------------------
		LineOfSosa {
			line_and_index: DualRadixPair::from((line_name.dec, line_sort.dec)),
			generation_where_start_line: DualRadix { dec: line_gene.dec, bin: line_gene.bin },
			auxiliary_index_within_line: DualRadix { dec: line_tier.dec, bin: line_tier.bin },
		}
	}
}

impl From<RadixDec> for LinesOfSosa {
	fn from(sosa: RadixDec) -> Self {
		let line_dad = LinesOfSosa::from_sosa_get_line(sosa, false);
		let line_mom = LinesOfSosa::from_sosa_get_line(sosa, true);
		// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░
		Self {
			dad: line_dad.line_and_index,
			mom: line_mom.line_and_index,
			generation_where_start_dad_line: line_dad.generation_where_start_line,
			generation_where_start_mom_line: line_mom.generation_where_start_line,
			auxiliary_index_within_dad_line: line_dad.auxiliary_index_within_line,
			auxiliary_index_within_mom_line: line_mom.auxiliary_index_within_line,
		}
	}
}
