// examples/e_sosa_new.rs

use comfy_table::{Table, presets::UTF8_FULL};
use kim::feature::id_sosa::PersonEntryFrom;

// ======example usage =====
fn main() {
	let mut table = new_table();
	add_rows(&mut table, PersonEntryFrom::Range(23..=26));
	add_rows(&mut table, PersonEntryFrom::Array(&[1, 4, 7, 0]));
	add_rows(&mut table, PersonEntryFrom::Given(44));

	println!("{table}");
}
// ======example usage =====
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
