// ./src/feature/id_kinship.rs

/*
 * # LITEROWY ZAPIS ŚCIEŻKI RELACJI
 *
 * ZASADA
 *   Ścieżkę czyta się od lewej do prawej, litera po literze; każda litera to jeden krok.
 *   np. PD – dziecko ojca (rodzeństwo przyrodnie ojczyste), MS – siostra matki.
 *
 * ## System "1" - Główny
 *
 * ### GRUPY LITER WEDŁUG KSZTAŁTU
 *
 * * męska    – pionowa kreska po lewej z poprzeczką w połowie wysokości (lub jej zaczątkiem)
 *              E, P, F, B, H, R, A
 * * żeńska   – skos z lewego górnego punktu, do prawego dolnego.
 *              X, M, N, S, W, K, Y
 * * nijaka   – obłość
 *              U, O, D, G, C, Q, J
 * * bez płci – trzon lub poprzeczki, bez cech płci; służą jako modyfikatory
 *              I, T, Z, L
 *
 * ### LITERY RELACJI
 *
 *               męska           żeńska           nijaka
 *   osoba       E – on          X – ona          U – unisex/Oboje
 *   rodzice     P – ojciec      M – matka        O – rodzic/rodzice
 *   dzieci      F – syn         N – córka        D – dziecko/dzieci
 *   rodzeństwo  B – brat        S – siostra      G – brat/siostra//rodzeństwo
 *   małżonkowie H – mąż         W – żona         C – małżonek
 *   przyjaciele A – przyjaciel  Y – przyjaciółka J – przyjaciel/przyjaciółka
 *   krewni      R – krewny      K – krewna       Q – krewny/krewna
 *               (krewny o nieustalonej ścieżce: z metryki lub dopasowania DNA)
 *
 *     Pochodzenie liter:
 *      E Ego, X chromosomy XX (i optyczna para z E), U Unisex,
 *      P Pater, M Mater, O Origin / Origo,
 *      F Filius, N nata (łac. córka), D Descendant,
 *      B Brother / Bruder, S Sister / Schwester, G Generation / Geschwister,
 *      H Husband, W Wife, C Couples,
 *      A amicus (łac. przyjaciel), Y yedidah (hebr. przyjaciółka) i podobne do odwróconego A,
 *      J Jedidah – biblijna pisownia tego samego słowa; rdzeń yadad (kochać), bez rodzaju, i połowa z U
 *      R Relative, K kin / kinswoman (ang. krewna) i podobne do K,
 *      Q qarīb / qarībah / aqārib (arab. krewny / krewna / krewni), oraz Query - szukane.
 *
 * ### MODYFIKATORY (stoją przed literą relacji)
 *
 * Modyfikator zmienia rodzaj więzi, a rolę zostawia literze po nim.
 *
 *   Z – przybrane, adopcja, przyszywane   (niem. Zieh-: Ziehvater, Ziehkind)
 *   L – chrzestne                          (łac. levantes – chrzestni w metrykach)
 *   T – świadek                            (łac. testis, testes – świadkowie w metrykach)
 *   I – relacja intymna                    (łac. intimus)
 *   [] – szczególny modyfikator gdy trzeba wydłużyć zakres modyfikatora
 *
 * ### PRZYKŁADY
 *   ZP   – przybrany ojciec              LP   – ojciec chrzestny
 *   ZF   – adoptowany syn                LM   – matka chrzestna
 *   ZS   – przybrana siostra             LF   – syn chrzestny
 *   MZS  – przyszywana siostra matki     LN   – córka chrzestna
 *   TE   – świadek (on)                  IY   – kochanka
 *   TX   – świadek (ona)                 IA   – kochanek
 *   TA   – świadek, przyjaciel           BA   – przyjaciel brata
 *   MR   – nieustalony krewny matki      WB   – brat żony
 *
 *  ## System "2" - potrzebny dla (wybiurczo-uciętej) kompatybilności z pewnym słownikiem
 *
 * P – rodzic, F – ojciec, M – matka,
 * C – dziecko, S – syn, D – córka,
 * G – rodzeństwo, B – brat, Z – siostra,
 * E – małżonek, H – mąż, W – żona.
 * Znak „+” po symbolu oznacza „starszy”, znak „−” po symbolu oznacza „młodszy”.
 *
 */

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// ZNACZENIA

/// Rola kroku ścieżki (wiersz tabeli liter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KinRole {
	Person,
	Parent,
	Child,
	Sibling,
	Spouse,
	Friend,
	Relative,
}

/// Grupa płci (kolumna tabeli liter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KinSex {
	Male,
	Female,
	Unisex,
}

/// Modyfikator więzi (system 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KinBond {
	/// Z – przybrane, adopcja, przyszywane
	Foster,
	/// L – chrzestne
	Godparent,
	/// T – świadek
	Witness,
	/// I – relacja intymna
	Intimate,
}

/// Wiek względny (system 2): `+` starszy, `−` młodszy, `!` bliźnak/równolat,
/// `*` - opcjonalne jawne podanie braku wiedy o wieku, domyślnie brak traktowany tak samo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum KinAge {
	Elder,
	Younger,
	Similar,
	#[default]
	Unspecified,
}

/// Krok ścieżki po rozbiorze. Wspólny dla obu systemów.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KinStep {
	/// Litera relacji: rola, płeć, wiek (wiek tylko w systemie 2).
	Relation(KinRole, KinSex, KinAge),
	/// Modyfikator nad jednym krokiem (`ZP`) albo nad grupą (`Z[MS]`).
	Bond(KinBond, Vec<KinStep>),
}

/// Wybór systemu zapisu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KinshipNotation {
	/// System 1 – główny.
	Main,
	/// System 2 – zgodność ze słownikiem.
	Dictionary,
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// TABLICE ZNAKÓW

/// System 1: znak → (rola, płeć).
fn main_letter(c: char) -> Option<(KinRole, KinSex)> {
	use KinRole::*;
	use KinSex::*;
	Some(match c {
		'E' => (Person, Male),
		'X' => (Person, Female),
		'U' => (Person, Unisex),
		'P' => (Parent, Male),
		'M' => (Parent, Female),
		'O' => (Parent, Unisex),
		'F' => (Child, Male),
		'N' => (Child, Female),
		'D' => (Child, Unisex),
		'B' => (Sibling, Male),
		'S' => (Sibling, Female),
		'G' => (Sibling, Unisex),
		'H' => (Spouse, Male),
		'W' => (Spouse, Female),
		'C' => (Spouse, Unisex),
		'A' => (Friend, Male),
		'Y' => (Friend, Female),
		'J' => (Friend, Unisex),
		'R' => (Relative, Male),
		'K' => (Relative, Female),
		'Q' => (Relative, Unisex),
		_ => return None,
	})
}

/// System 1: (rola, płeć) → znak. Tablica jest pełna, więc zawsze jest wynik.
fn main_char(role: KinRole, sex: KinSex) -> char {
	use KinRole::*;
	use KinSex::*;
	match (role, sex) {
		(Person, Male) => 'E',
		(Person, Female) => 'X',
		(Person, Unisex) => 'U',
		(Parent, Male) => 'P',
		(Parent, Female) => 'M',
		(Parent, Unisex) => 'O',
		(Child, Male) => 'F',
		(Child, Female) => 'N',
		(Child, Unisex) => 'D',
		(Sibling, Male) => 'B',
		(Sibling, Female) => 'S',
		(Sibling, Unisex) => 'G',
		(Spouse, Male) => 'H',
		(Spouse, Female) => 'W',
		(Spouse, Unisex) => 'C',
		(Friend, Male) => 'A',
		(Friend, Female) => 'Y',
		(Friend, Unisex) => 'J',
		(Relative, Male) => 'R',
		(Relative, Female) => 'K',
		(Relative, Unisex) => 'Q',
	}
}

/// System 1: znak modyfikatora.
fn main_bond(c: char) -> Option<KinBond> {
	Some(match c {
		'Z' => KinBond::Foster,
		'L' => KinBond::Godparent,
		'T' => KinBond::Witness,
		'I' => KinBond::Intimate,
		_ => return None,
	})
}

fn main_bond_char(bond: KinBond) -> char {
	match bond {
		KinBond::Foster => 'Z',
		KinBond::Godparent => 'L',
		KinBond::Witness => 'T',
		KinBond::Intimate => 'I',
	}
}

/// System 2: znak → (rola, płeć).
fn dict_letter(c: char) -> Option<(KinRole, KinSex)> {
	use KinRole::*;
	use KinSex::*;
	Some(match c {
		'P' => (Parent, Unisex),
		'F' => (Parent, Male),
		'M' => (Parent, Female),
		'C' => (Child, Unisex),
		'S' => (Child, Male),
		'D' => (Child, Female),
		'G' => (Sibling, Unisex),
		'B' => (Sibling, Male),
		'Z' => (Sibling, Female),
		'E' => (Spouse, Unisex),
		'H' => (Spouse, Male),
		'W' => (Spouse, Female),
		_ => return None,
	})
}

/// System 2: (rola, płeć) → znak; `None` dla ról spoza słownika.
fn dict_char(role: KinRole, sex: KinSex) -> Option<char> {
	use KinRole::*;
	use KinSex::*;
	Some(match (role, sex) {
		(Parent, Unisex) => 'P',
		(Parent, Male) => 'F',
		(Parent, Female) => 'M',
		(Child, Unisex) => 'C',
		(Child, Male) => 'S',
		(Child, Female) => 'D',
		(Sibling, Unisex) => 'G',
		(Sibling, Male) => 'B',
		(Sibling, Female) => 'Z',
		(Spouse, Unisex) => 'E',
		(Spouse, Male) => 'H',
		(Spouse, Female) => 'W',
		_ => return None,
	})
}

/// System 2: znak wieku. Przyjmuje `−` (U+2212) i `-` (ASCII).
fn dict_age(c: char) -> Option<KinAge> {
	match c {
		'+' => Some(KinAge::Elder),
		'−' | '-' => Some(KinAge::Younger),
		'!' => Some(KinAge::Similar),
		'*' => Some(KinAge::Unspecified),
		_ => None,
	}
}

fn dict_age_char(age: KinAge) -> Option<char> {
	match age {
		KinAge::Elder => Some('+'),
		KinAge::Younger => Some('−'),
		KinAge::Similar => Some('!'),
		KinAge::Unspecified => None,
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// BŁĘDY (pozycja = numer znaku od 0, nie bajtu)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KinParseError {
	/// Znak spoza systemu.
	UnknownChar(usize, char),
	/// Modyfikator bez kroku po nim (`Z` na końcu albo `Z]`).
	DanglingModifier(usize),
	/// `[` bez modyfikatora przed nim.
	GroupWithoutModifier(usize),
	/// `[` bez `]`.
	UnclosedGroup(usize),
	/// `]` bez `[`.
	UnexpectedClose(usize),
	/// `[]`.
	EmptyGroup(usize),
	/// `+`/`−` nie po literze albo drugi raz po tej samej literze.
	MisplacedAge(usize),
}

impl std::fmt::Display for KinParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnknownChar(i, c) => write!(f, "nieznany znak `{c}` na pozycji {i}"),
			Self::DanglingModifier(i) => write!(f, "modyfikator na pozycji {i} nie ma kroku po sobie"),
			Self::GroupWithoutModifier(i) => write!(f, "grupa na pozycji {i} nie ma modyfikatora przed sobą"),
			Self::UnclosedGroup(i) => write!(f, "grupa otwarta na pozycji {i} nie jest zamknięta"),
			Self::UnexpectedClose(i) => write!(f, "zamknięcie grupy na pozycji {i} bez otwarcia"),
			Self::EmptyGroup(i) => write!(f, "pusta grupa na pozycji {i}"),
			Self::MisplacedAge(i) => write!(f, "znak wieku na pozycji {i} nie stoi po literze"),
		}
	}
}

impl std::error::Error for KinParseError {}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// ROZBIÓR I ZAPIS

impl KinshipNotation {
	/// Rozbiera tekst ścieżki na kroki.
	pub fn parse(self, text: &str) -> Result<Vec<KinStep>, KinParseError> {
		let chars: Vec<char> = text.chars().collect();
		match self {
			Self::Main => {
				let mut pos = 0;
				parse_main_seq(&chars, &mut pos, None)
			}
			Self::Dictionary => parse_dict(&chars),
		}
	}

	/// Zapisuje kroki w danym systemie; `None`, gdy system nie ma takiego znaku
	/// (np. przyjaciel w systemie 2 albo wiek w systemie 1).
	pub fn render(self, steps: &[KinStep]) -> Option<String> {
		let mut out = String::new();
		for step in steps {
			match self {
				Self::Main => render_main(step, &mut out)?,
				Self::Dictionary => render_dict(step, &mut out)?,
			}
		}
		Some(out)
	}
}

/// Ciąg kroków; `open` to pozycja `[`, gdy jesteśmy w grupie.
fn parse_main_seq(chars: &[char], pos: &mut usize, open: Option<usize>) -> Result<Vec<KinStep>, KinParseError> {
	let mut steps = Vec::new();
	while let Some(&c) = chars.get(*pos) {
		if c == ']' {
			return match open {
				Some(_) => Ok(steps),
				None => Err(KinParseError::UnexpectedClose(*pos)),
			};
		}
		steps.push(parse_main_step(chars, pos)?);
	}
	match open {
		Some(at) => Err(KinParseError::UnclosedGroup(at)),
		None => Ok(steps),
	}
}

/// Jeden krok: litera, albo modyfikator z krokiem lub grupą `[…]`.
fn parse_main_step(chars: &[char], pos: &mut usize) -> Result<KinStep, KinParseError> {
	let at = *pos;
	let c = chars[at];
	*pos += 1;

	if let Some((role, sex)) = main_letter(c) {
		return Ok(KinStep::Relation(role, sex, KinAge::Unspecified));
	}
	if c == '[' {
		return Err(KinParseError::GroupWithoutModifier(at));
	}
	let Some(bond) = main_bond(c) else {
		return Err(KinParseError::UnknownChar(at, c));
	};
	match chars.get(*pos) {
		None | Some(']') => Err(KinParseError::DanglingModifier(at)),
		Some('[') => {
			let open = *pos;
			*pos += 1;
			let inner = parse_main_seq(chars, pos, Some(open))?;
			*pos += 1; // `]`
			if inner.is_empty() {
				return Err(KinParseError::EmptyGroup(open));
			}
			Ok(KinStep::Bond(bond, inner))
		}
		Some(_) => Ok(KinStep::Bond(bond, vec![parse_main_step(chars, pos)?])),
	}
}

fn parse_dict(chars: &[char]) -> Result<Vec<KinStep>, KinParseError> {
	let mut steps: Vec<KinStep> = Vec::new();
	// czy ostatnia litera może jeszcze dostać znak wieku
	let mut age_open = false;
	for (i, &c) in chars.iter().enumerate() {
		if let Some((role, sex)) = dict_letter(c) {
			steps.push(KinStep::Relation(role, sex, KinAge::Unspecified));
			age_open = true;
		} else if let Some(age) = dict_age(c) {
			match steps.last_mut() {
				Some(KinStep::Relation(_, _, slot)) if age_open => *slot = age,
				_ => return Err(KinParseError::MisplacedAge(i)),
			}
			age_open = false;
		} else {
			return Err(KinParseError::UnknownChar(i, c));
		}
	}
	Ok(steps)
}

fn render_main(step: &KinStep, out: &mut String) -> Option<()> {
	match step {
		KinStep::Relation(role, sex, KinAge::Unspecified) => out.push(main_char(*role, *sex)),
		KinStep::Bond(bond, inner) => {
			out.push(main_bond_char(*bond));
			match inner.as_slice() {
				[] => return None,
				[one] => render_main(one, out)?,
				many => {
					out.push('[');
					for s in many {
						render_main(s, out)?;
					}
					out.push(']');
				}
			}
		}
		KinStep::Relation(..) => return None,
	}
	Some(())
}

fn render_dict(step: &KinStep, out: &mut String) -> Option<()> {
	match step {
		KinStep::Bond(..) => return None,
		KinStep::Relation(role, sex, age) => {
			out.push(dict_char(*role, *sex)?);
			if let Some(c) = dict_age_char(*age) {
				out.push(c);
			}
		}
	}
	Some(())
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

#[cfg(test)]
mod tests {
	use super::KinRole::*;
	use super::KinSex::*;
	use super::KinshipNotation::{Dictionary, Main};
	use super::*;

	fn rel(role: KinRole, sex: KinSex) -> KinStep {
		KinStep::Relation(role, sex, KinAge::Unspecified)
	}

	#[test]
	fn main_examples() {
		assert_eq!(Main.parse("PD"), Ok(vec![rel(Parent, Male), rel(Child, Unisex)]));
		assert_eq!(Main.parse("MS"), Ok(vec![rel(Parent, Female), rel(Sibling, Female)]));
		assert_eq!(Main.parse("ZP"), Ok(vec![KinStep::Bond(KinBond::Foster, vec![rel(Parent, Male)])]));
		assert_eq!(
			Main.parse("MZS"),
			Ok(vec![rel(Parent, Female), KinStep::Bond(KinBond::Foster, vec![rel(Sibling, Female)])])
		);
		assert_eq!(Main.parse("TA"), Ok(vec![KinStep::Bond(KinBond::Witness, vec![rel(Friend, Male)])]));
		assert_eq!(Main.parse("IY"), Ok(vec![KinStep::Bond(KinBond::Intimate, vec![rel(Friend, Female)])]));
		assert_eq!(Main.parse("MR"), Ok(vec![rel(Parent, Female), rel(Relative, Male)]));
	}

	#[test]
	fn main_groups_and_stacking() {
		// przyszywana ciotka: cała ścieżka MS jest przybrana
		assert_eq!(
			Main.parse("Z[MS]"),
			Ok(vec![KinStep::Bond(KinBond::Foster, vec![rel(Parent, Female), rel(Sibling, Female)])])
		);
		// modyfikatory się nakładają; LZP i L[ZP] to to samo
		let stacked =
			vec![KinStep::Bond(KinBond::Godparent, vec![KinStep::Bond(KinBond::Foster, vec![rel(Parent, Male)])])];
		assert_eq!(Main.parse("LZP"), Ok(stacked.clone()));
		assert_eq!(Main.parse("L[ZP]"), Ok(stacked));
		// Z[P] to to samo co ZP
		assert_eq!(Main.parse("Z[P]"), Main.parse("ZP"));
	}

	#[test]
	fn main_errors() {
		assert_eq!(Main.parse("V"), Err(KinParseError::UnknownChar(0, 'V')));
		assert_eq!(Main.parse("PZ"), Err(KinParseError::DanglingModifier(1)));
		assert_eq!(Main.parse("Z[M]]"), Err(KinParseError::UnexpectedClose(4)));
		assert_eq!(Main.parse("Z[MS"), Err(KinParseError::UnclosedGroup(1)));
		assert_eq!(Main.parse("Z[]"), Err(KinParseError::EmptyGroup(1)));
		assert_eq!(Main.parse("Z[Z]"), Err(KinParseError::DanglingModifier(2)));
		assert_eq!(Main.parse("[MS]"), Err(KinParseError::GroupWithoutModifier(0)));
		assert_eq!(Main.parse("P+"), Err(KinParseError::UnknownChar(1, '+')));
	}

	#[test]
	fn dict_examples() {
		assert_eq!(Dictionary.parse("MZ"), Ok(vec![rel(Parent, Female), rel(Sibling, Female)]));
		assert_eq!(
			Dictionary.parse("FB+"),
			Ok(vec![rel(Parent, Male), KinStep::Relation(Sibling, Male, KinAge::Elder)])
		);
		assert_eq!(Dictionary.parse("B−"), Dictionary.parse("B-"));
		assert_eq!(Dictionary.parse("+B"), Err(KinParseError::MisplacedAge(0)));
		assert_eq!(Dictionary.parse("B++"), Err(KinParseError::MisplacedAge(2)));
		assert_eq!(Dictionary.parse("A"), Err(KinParseError::UnknownChar(0, 'A')));
		assert_eq!(Dictionary.parse("B!"), Ok(vec![KinStep::Relation(Sibling, Male, KinAge::Similar)]));
		assert_eq!(Dictionary.parse("B*"), Dictionary.parse("B"));
		assert_eq!(Dictionary.parse("B*+"), Err(KinParseError::MisplacedAge(2)));
	}

	#[test]
	fn round_trip_and_conversion() {
		for text in ["PD", "MZS", "Z[MS]", "LZP", "TA", "IY", "WB", "MR"] {
			let steps = Main.parse(text).unwrap();
			assert_eq!(Main.render(&steps).as_deref(), Some(text));
		}
		for text in ["MZ", "FB+", "SD−", "EHW", "B!"] {
			let steps = Dictionary.parse(text).unwrap();
			assert_eq!(Dictionary.render(&steps).as_deref(), Some(text));
		}
		// siostra matki: system 2 → system 1
		assert_eq!(Main.render(&Dictionary.parse("MZ").unwrap()).as_deref(), Some("MS"));
		// wiek nie istnieje w systemie 1, przyjaciel nie istnieje w systemie 2
		assert_eq!(Main.render(&Dictionary.parse("B+").unwrap()), None);
		assert_eq!(Dictionary.render(&Main.parse("BA").unwrap()), None);
		// `*` przy zapisie znika, bo brak znaku znaczy to samo
		assert_eq!(Dictionary.render(&Dictionary.parse("B*").unwrap()).as_deref(), Some("B"));
		// L[ZP] zapisuje się krócej jako LZP
		assert_eq!(Main.render(&Main.parse("L[ZP]").unwrap()).as_deref(), Some("LZP"));
	}
}
