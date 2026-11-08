// examples/e_sosa_new.rs

use comfy_table::{Table, presets::UTF8_FULL};
use kim::feature::id_sosa::PersonEntryFrom;

// ======example usage =====
fn main() {
	let mut table = new_table();
	add_rows(&mut table, PersonEntryFrom::Range(23..=26));
	add_rows(&mut table, PersonEntryFrom::Array(&[1, 4, 7, 0]));
	add_rows(&mut table, PersonEntryFrom::Given(44));

	//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
	let x44 = PersonEntryFrom::Given(44);
	println!(
		"DLA 44::  FROM-0: {} FROM-1:  {}",
		x44.to_entry().unwrap().generation.from0,
		x44.to_entry().unwrap().generation.from1
	);
	let x88 = PersonEntryFrom::Given(88);
	println!(
		"DLA 88::  FROM-0: {} FROM-1:  {}",
		x88.to_entry().unwrap().generation.from0,
		x88.to_entry().unwrap().generation.from1
	);
	let x89 = PersonEntryFrom::Given(89);
	println!(
		"DLA 89::  FROM-0: {} FROM-1:  {}",
		x89.to_entry().unwrap().generation.from0,
		x89.to_entry().unwrap().generation.from1
	);
	let x22 = PersonEntryFrom::Given(22);
	println!(
		"DLA 22::  FROM-0: {} FROM-1:  {}",
		x22.to_entry().unwrap().generation.from0,
		x22.to_entry().unwrap().generation.from1
	);

	println!("{:?}", x44.to_entry().unwrap());
	println!("{}", "▒".repeat(100));
	println!("{}", "▒".repeat(100));

	//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
	println!("{table}");
}
// ======example usage =====

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// styl i nagłówek, w jednym miejscu
fn new_table() -> Table {
	let mut table = Table::new();
	table.load_style(UTF8_FULL);
	table.set_header(vec!["SEX+GEN", "SOSA", "LINE-Y", "LINE-X", "SOSA-DAD", "SOSA-MOM", "SOSA-KID"]);
	table
}

// format wiersza, w jednym miejscu dla każdego źródła
fn add_rows(table: &mut Table, source: PersonEntryFrom<'_>) {
	for entry in source.entries() {
		table.add_row(vec![
			format!("{} {}", entry.sosa.ego.2.get_char(), entry.generation.from0.dec),
			entry.sosa.ego.0.dec.to_string(),
			format!("{} — {}", entry.lines.dad.dec.0, entry.lines.dad.dec.1),
			format!("{} — {}", entry.lines.mom.dec.0, entry.lines.mom.dec.1),
			entry.sosa.dad.0.dec.to_string(),
			entry.sosa.mom.0.dec.to_string(),
			entry.sosa.kid.0.dec.to_string(),
		]);
	}
}
