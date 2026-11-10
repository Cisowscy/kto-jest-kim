// ./examples/e_kinship_lab_cli.rs

//! Zamiana literowego zapisu ścieżki relacji między systemami.
//!
//! `--toCiso` – z systemu "2" (Jagodzińskiego) na system "1" (Cisowskiego)
//! `--toJago` – z systemu "1" (Cisowskiego) na system "2" (Jagodzińskiego)
//!
//! Gdy kod systemu "2" jest w słowniku (`glossary_id_kinship`), wypisuje też jego nazwy
//! razem z drugą stroną relacji, np. `F (ojciec ↔ S syn, D córka)`.
//!
//! Ścieżki podaje się jako argumenty, a bez argumentów – po jednej w wierszu na wejściu:
//!
//! ```text
//! cargo run --example e_kinship_lab_cli -- --toCiso MZ FB SD
//! cargo run --example e_kinship_lab_cli -- --toJago MS PB "Z[MS]"
//! echo MZ | cargo run --example e_kinship_lab_cli -- --toCiso
//! ```

use std::io::BufRead;
use std::process::ExitCode;

use kim::feature::glossary_id_kinship::KinshipJago;
use kim::feature::id_kinship::KinshipNotation;

const HELP: &str = "\
użycie: e_kinship_lab_cli (--toCiso | --toJago) [ŚCIEŻKA...]

  --toCiso   system \"2\" (Jagodzińskiego) → system \"1\" (Cisowskiego)
  --toJago   system \"1\" (Cisowskiego) → system \"2\" (Jagodzińskiego)
  -h, --help ta pomoc

Bez ścieżek w argumentach czyta je ze standardowego wejścia, po jednej w wierszu.";

fn main() -> ExitCode {
	let mut args = std::env::args().skip(1);

	// (system wejścia, system wyjścia, czego brakuje w systemie wyjścia)
	let (from, to, missing) = match args.next().as_deref() {
		Some("--toCiso") => {
			(KinshipNotation::Dictionary, KinshipNotation::Main, "system \"1\" nie ma znaków wieku (+, −, !)")
		}
		Some("--toJago") => (
			KinshipNotation::Main,
			KinshipNotation::Dictionary,
			"system \"2\" nie ma osoby, przyjaciół, krewnych ani modyfikatorów",
		),
		Some("-h" | "--help") => {
			println!("{HELP}");
			return ExitCode::SUCCESS;
		}
		_ => {
			eprintln!("{HELP}");
			return ExitCode::from(2);
		}
	};

	// etykiety kodów: kJago – system "2", kCiso – system "1"
	let (from_label, to_label) =
		if from == KinshipNotation::Dictionary { ("kJago", "kCiso") } else { ("kCiso", "kJago") };

	let rest: Vec<String> = args.collect();
	let inputs: Vec<String> = if rest.is_empty() {
		std::io::stdin()
			.lock()
			.lines()
			.map_while(Result::ok)
			.map(|line| line.trim().to_string())
			.filter(|line| !line.is_empty())
			.collect()
	} else {
		rest
	};

	let mut failed = false;
	for text in &inputs {
		match from.parse(text) {
			Err(err) => {
				eprintln!("{from_label}: {text} | błąd: {err}");
				failed = true;
			}
			Ok(steps) => match to.render(&steps) {
				Some(out) => {
					// po stronie systemu "2" szukamy nazwy w słowniku
					let jago = if from == KinshipNotation::Dictionary { text.as_str() } else { out.as_str() };
					match opis(jago) {
						Some(opis) => println!("{from_label}: {text} → {to_label}: {out} | opis: ({opis})"),
						None => println!("{from_label}: {text} → {to_label}: {out}"),
					}
				}
				None => {
					eprintln!("{from_label}: {text} | brak odpowiednika: {missing}");
					failed = true;
				}
			},
		}
	}

	if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

/// Kody systemu "2" i odpowiadające im pozycje słownika.
#[rustfmt::skip]
const KODY: &[(&str, KinshipJago)] = {
	use KinshipJago::*;
	&[
		("F", F), ("M", M), ("P", P),
		("S", S), ("D", D), ("C", C),
		("B", B), ("Z", Z), ("G", G),
		("H", H), ("W", W), ("E", E),
		("FP", FP), ("MP", MP), ("PP", PP),
		("SC", SC), ("DC", DC), ("CC", CC),
		("FF", FF), ("FM", FM), ("MF", MF), ("MM", MM),
		("SS", SS), ("SD", SD), ("DS", DS), ("DD", DD),
		("FC", FC), ("MC", MC), ("PC", PC),
		("FS", FS), ("FD", FD), ("MS", MS), ("MD", MD),
	]
};

/// Porównanie wariantów bez `PartialEq` w `KinshipJago`.
fn ten_sam(a: &KinshipJago, b: &KinshipJago) -> bool {
	std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn kod(wariant: &KinshipJago) -> &'static str {
	KODY.iter().find(|(_, w)| ten_sam(w, wariant)).map_or("?", |(k, _)| k)
}

/// Opis kodu ze słownika par, np. `ojciec ↔ S syn, D córka`.
/// Gdy kod ma różne nazwy zależnie od drugiej strony, każda nazwa jest osobno, rozdzielona `;`.
/// `None`, gdy kodu nie ma w słowniku.
fn opis(code: &str) -> Option<String> {
	let (_, wariant) = KODY.iter().find(|(k, _)| *k == code)?;

	// (nazwa tej strony, [druga strona: "KOD nazwa"])
	let mut nazwy: Vec<(&str, Vec<String>)> = Vec::new();
	let mut dodaj = |nazwa: &'static str, druga: &KinshipJago, druga_nazwa: &str| {
		let wpis = format!("{} {druga_nazwa}", kod(druga));
		match nazwy.iter_mut().find(|(n, _)| *n == nazwa) {
			Some((_, lista)) => lista.push(wpis),
			None => nazwy.push((nazwa, vec![wpis])),
		}
	};
	for ((a, na), (b, nb)) in KinshipJago::PARY {
		if ten_sam(a, wariant) {
			dodaj(na, b, nb);
		}
		// para symetryczna (np. B ↔ B) liczy się raz
		if ten_sam(b, wariant) && !ten_sam(a, b) {
			dodaj(nb, a, na);
		}
	}
	if nazwy.is_empty() {
		return None;
	}
	Some(nazwy.iter().map(|(n, lista)| format!("{n} ↔ {}", lista.join(", "))).collect::<Vec<_>>().join("; "))
}
