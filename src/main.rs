// ./src/main.rs

use egui_knex::{
	eframe,
	egui,
	element::{Ora, OraButonStatus},
};
use kim::template::kim_jest_antenat;

struct AppKtoJestKim {
	okno: Ora,
}

impl Default for AppKtoJestKim {
	fn default() -> Self {
		Self {
			okno: Ora::nova("kto_antenatem", "Kim jest antenat?").butones((
				OraButonStatus::Occultus,
				OraButonStatus::Visibilis,
				OraButonStatus::Occultus,
			)),
		}
	}
}

impl eframe::App for AppKtoJestKim {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		self.okno.show(ui, |ui| {
			kim_jest_antenat::tresc(ui);
		});
	}
}

fn main() -> eframe::Result<()> { egui_knex::run_app::<AppKtoJestKim>() }
