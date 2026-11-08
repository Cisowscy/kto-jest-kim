// examples/e_sosa.rs

use comfy_table::{Table, presets::UTF8_FULL};
use kim::feature::{
	id_sosa::PersonEntryFrom,
	table_id_sosa::{Kolumna, domyslne, wybrane},
};

// ======example usage =====
fn main() {
	let kolumny = wybrane(&domyslne());
	let mut table = new_table(&kolumny);
	add_rows(&mut table, &kolumny, PersonEntryFrom::Range(23..=26));
	add_rows(&mut table, &kolumny, PersonEntryFrom::Array(&[1, 4, 7, 0]));
	add_rows(&mut table, &kolumny, PersonEntryFrom::Given(44));

	println!("{table}");
}
// ======example usage =====

// styl i nagłówek, w jednym miejscu
fn new_table(kolumny: &[Kolumna]) -> Table {
	let mut table = Table::new();
	table.load_style(UTF8_FULL);
	table.set_header(kolumny.iter().map(|k| k.nazwa));
	table
}

// format wiersza, w jednym miejscu dla każdego źródła
fn add_rows(table: &mut Table, kolumny: &[Kolumna], source: PersonEntryFrom<'_>) {
	for entry in source.entries() {
		table.add_row(kolumny.iter().map(|k| k.tekst(&entry)));
	}
}
