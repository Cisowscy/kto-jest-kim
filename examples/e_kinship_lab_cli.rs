// ./examples/e_kinship_lab_cli.rs

//! Zamiana literowego zapisu ścieżki relacji między systemami.
//!
//! `--toCiso` – z systemu "2" (Jagodzińskiego) na system "1" (Cisowskiego)
//! `--toJago` – z systemu "1" (Cisowskiego) na system "2" (Jagodzińskiego)
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
		Some("--toCiso") => (
			KinshipNotation::Dictionary,
			KinshipNotation::Main,
			"system \"1\" nie ma znaków wieku (+, −, !)",
		),
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
				eprintln!("{text}\tbłąd: {err}");
				failed = true;
			}
			Ok(steps) => match to.render(&steps) {
				Some(out) => println!("{text}\t→\t{out}"),
				None => {
					eprintln!("{text}\tbrak odpowiednika: {missing}");
					failed = true;
				}
			},
		}
	}

	if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
