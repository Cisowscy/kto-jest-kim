// examples/e_sosa_new.rs

use comfy_table::{Table, presets::UTF8_FULL};
use kim::feature::id_sosa::{LineCode, PersonEntryFrom, RadixFormat};

// ======example usage =====
fn main() {
	let mut table = new_table();
	add_rows(&mut table, PersonEntryFrom::Range(23..=26));
	add_rows(&mut table, PersonEntryFrom::Array(&[1, 4, 7, 0]));
	add_rows(&mut table, PersonEntryFrom::Given(44));

	//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
	let x44 = PersonEntryFrom::Given(44).to_entry().unwrap();

	println!(
		"\n{}\n{}\n{}\n{}\n",
		format!("DLA 44::  {}", x44.sosa.ego.gennum,),
		format!("DLA 88::  {}", x44.sosa.dad.gennum,),
		format!("DLA 89::  {}", x44.sosa.mom.gennum,),
		format!("DLA 22::  {}", x44.sosa.kid.gennum,)
	);

	println!("{:?}", x44);
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
			format!("{} {}", entry.sosa.ego.gender.get_char(), entry.sosa.ego.gennum.dec.0),
			entry.sosa.ego.normal.dec.to_string(),
			entry.lines.dad.dec.get_code(),
			entry.lines.mom.dec.get_code(),
			entry.sosa.dad.normal.dec.to_string(),
			entry.sosa.mom.normal.dec.to_string(),
			entry.sosa.kid.normal.dec.to_string(),
		]);
	}
}
