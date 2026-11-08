// src/template/kim_jest_antenat.rs

use egui_knex::{
	egui::{Color32, DragValue, Id, Label, Rect, ScrollArea, TextEdit, Ui, UiBuilder, Window},
	element::{Vas, VasDispositio, VasFovea, VasRedundantia},
	pos2, vec2,
};

use crate::feature::{
	id_sosa::PersonEntry,
	table_id_sosa::{self, SosaEntryMode},
};

const TLO_WIERSZA: Color32 = Color32::from_rgb(34, 36, 44);
const BLAD: Color32 = Color32::from_rgb(230, 110, 110);
const ODSTEP: f32 = 8.0;
const WYS_PASKA: f32 = 40.0;
const WYS_OBSZARU_ZRODEL: f32 = 180.0;
const WYS_ZRODLA: f32 = 34.0;
const WYS_NAGLOWKA: f32 = 26.0;
const WYS_WIERSZA: f32 = 24.0;

/// Tryb wejścia i tekst pola edycji (używany przez listę, żeby dało się pisać po kolei).
type Wpis = (SosaEntryMode, String);

/// Stan widżetu. Pięć pól, więc struktura zamiast krotki.
#[derive(Clone, Default)]
struct Stan {
	zrodla: Vec<Wpis>,
	kolumny: Vec<&'static str>,
	przesun_zrodla: f32,
	przesun_tabeli: f32,
	okno_kolumn: bool,
}

pub fn tresc(ui: &mut Ui) {
	let id = Id::new("kim_jest_antenat_stan");
	let mut stan: Stan = ui.data(|d| d.get_temp(id)).unwrap_or_else(|| Stan {
		zrodla: vec![
			(SosaEntryMode::Range(23..=26), String::new()),
			(SosaEntryMode::Array(vec![1, 4, 7, 0]), "1, 4, 7, 0".to_owned()),
			(SosaEntryMode::Given(44), String::new()),
		],
		kolumny: table_id_sosa::domyslne(),
		..Default::default()
	});

	let glowny = Vas::verticalis(VasRedundantia::Compressio)
		.padding((ODSTEP, ODSTEP, ODSTEP, ODSTEP, ODSTEP))
		.foveae([VasFovea::pixel(0, WYS_OBSZARU_ZRODEL), VasFovea::pixel(1, WYS_PASKA), VasFovea::auto(2)])
		.resolve(ui.max_rect());
	let zrodla_r = glowny.rect(0);
	let pasek_r = glowny.rect(1);
	let tabela_r = glowny.rect(2);

	// 1. Tryby wejścia: edycja i usuwanie
	let mut usun = None;
	let ile = stan.zrodla.len();
	przewijana(ui, zrodla_r, ile, WYS_ZRODLA, &mut stan.przesun_zrodla, |i, rect, ui| {
		let (zrodlo, tekst) = &mut stan.zrodla[i];
		ui.painter().rect_filled(rect, 6.0, TLO_WIERSZA);
		let mut wiersz = ui.new_child(UiBuilder::new().max_rect(rect.shrink(4.0)));
		wiersz.horizontal(|ui| {
			if ui.button("x").clicked() {
				usun = Some(i);
			}
			match zrodlo {
				SosaEntryMode::Given(s) => {
					ui.label("Jedna");
					ui.add(DragValue::new(s).speed(1.0));
				}
				SosaEntryMode::Array(liczby) => {
					ui.label("Lista");
					ui.add(TextEdit::singleline(&mut *tekst).desired_width(220.0));
					match table_id_sosa::parsuj_liste(tekst.as_str()) {
						Ok(nowe) => *liczby = nowe,
						Err(blad) => {
							ui.colored_label(BLAD, blad);
						}
					}
				}
				SosaEntryMode::Range(r) => {
					ui.label("Przedział");
					let (mut od, mut do_) = (*r.start(), *r.end());
					ui.add(DragValue::new(&mut od).speed(1.0));
					ui.label("–");
					ui.add(DragValue::new(&mut do_).speed(1.0));
					*r = od..=do_;
					if od > do_ {
						ui.colored_label(BLAD, "pusty przedział");
					}
				}
			}
		});
	});
	if let Some(i) = usun {
		stan.zrodla.remove(i);
	}

	// 2. Pasek narzędzi: dodawanie trybów wejścia i okno kolumn
	let mut pasek = ui.new_child(UiBuilder::new().max_rect(pasek_r));
	pasek.horizontal(|ui| {
		if ui.button("+ Jedna").clicked() {
			stan.zrodla.push((SosaEntryMode::Given(1), String::new()));
		}
		if ui.button("+ Lista").clicked() {
			stan.zrodla.push((SosaEntryMode::Array(Vec::new()), String::new()));
		}
		if ui.button("+ Przedział").clicked() {
			stan.zrodla.push((SosaEntryMode::Range(1..=1), String::new()));
		}
		ui.separator();
		if ui.button("Kolumny").clicked() {
			stan.okno_kolumn = !stan.okno_kolumn;
		}
	});

	// 3. Okno wyboru kolumn: pula z id_sosa.rs, pogrupowana po pierwszym segmencie nazwy
	let pula = table_id_sosa::pula();
	let mut otwarte = stan.okno_kolumn;
	Window::new("Kolumny").open(&mut otwarte).default_width(380.0).show(ui.ctx(), |ui| {
		ui.horizontal(|ui| {
			if ui.button("wszystkie").clicked() {
				stan.kolumny = pula.iter().map(|k| k.nazwa).collect();
			}
			if ui.button("żadne").clicked() {
				stan.kolumny.clear();
			}
		});
		ScrollArea::vertical().max_height(460.0).show(ui, |ui| {
			for grupa in ["sosa", "index", "lines"] {
				ui.collapsing(grupa, |ui| {
					for k in pula.iter().filter(|k| k.grupa() == grupa) {
						let mut wybrana = stan.kolumny.contains(&k.nazwa);
						if ui.checkbox(&mut wybrana, k.nazwa).changed() {
							if wybrana {
								stan.kolumny.push(k.nazwa);
							} else {
								stan.kolumny.retain(|n| *n != k.nazwa);
							}
						}
					}
				});
			}
		});
	});
	stan.okno_kolumn = otwarte;

	// 4. Tabela: nagłówek i wirtualizowane wiersze
	let kolumny = table_id_sosa::wybrane(&stan.kolumny);
	if kolumny.is_empty() {
		ui.put(tabela_r, Label::new("Wybierz kolumny przyciskiem „Kolumny”."));
	} else {
		let nag_r = Rect::from_min_size(tabela_r.min, vec2(tabela_r.width(), WYS_NAGLOWKA));
		let cialo_r = Rect::from_min_max(pos2(tabela_r.min.x, tabela_r.min.y + WYS_NAGLOWKA), tabela_r.max);

		let uklad = uklad_kolumn(nag_r, kolumny.len());
		for (c, k) in kolumny.iter().enumerate() {
			ui.put(uklad.rect(c), Label::new(k.nazwa));
		}

		// Numery sosa ze wszystkich trybów wejścia. Zera są pomijane.
		let numery: Vec<_> = stan.zrodla.iter().flat_map(|(z, _)| z.numbers()).collect();
		przewijana(ui, cialo_r, numery.len(), WYS_WIERSZA, &mut stan.przesun_tabeli, |i, rect, ui| {
			// PersonEntry liczymy tylko dla widocznych wierszy.
			let e = PersonEntry::from_sosa(numery[i]);
			ui.painter().rect_filled(rect, 4.0, TLO_WIERSZA);
			let uklad = uklad_kolumn(rect, kolumny.len());
			for (c, k) in kolumny.iter().enumerate() {
				ui.put(uklad.rect(c), Label::new(k.tekst(&e)));
			}
		});
	}

	ui.data_mut(|d| d.insert_temp(id, stan));
}

/// Pionowa lista o stałej wysokości wiersza, przewijana kółkiem. Rysuje tylko widoczne wiersze.
fn przewijana(
	ui: &mut Ui,
	obszar: Rect,
	ile: usize,
	wys: f32,
	przesuniecie: &mut f32,
	mut rysuj: impl FnMut(usize, Rect, &mut Ui),
) {
	let max_przesuniecie = (ile as f32 * wys - obszar.height()).max(0.0);
	if ui.rect_contains_pointer(obszar) {
		*przesuniecie -= ui.input(|i| i.smooth_scroll_delta.y);
	}
	*przesuniecie = (*przesuniecie).clamp(0.0, max_przesuniecie);

	let mut dziecko = ui.new_child(UiBuilder::new().max_rect(obszar));
	dziecko.set_clip_rect(obszar);
	let pierwszy = (*przesuniecie / wys).floor().max(0.0) as usize;
	let koniec = (pierwszy + (obszar.height() / wys).ceil() as usize + 2).min(ile);
	for i in pierwszy..koniec {
		let y = obszar.min.y + i as f32 * wys - *przesuniecie;
		rysuj(i, Rect::from_min_size(pos2(obszar.min.x, y), vec2(obszar.width(), wys)), &mut dziecko);
	}
}

/// Podział wiersza na równe kolumny (`auto`). Szerokości można później ustawić per kolumna.
fn uklad_kolumn(rect: Rect, ile: usize) -> VasDispositio {
	Vas::horizontalis(VasRedundantia::Compressio)
		.padding((4.0, 4.0, 2.0, 2.0, 4.0))
		.foveae((0..ile).map(|i| VasFovea::auto(i)))
		.resolve(rect)
}
