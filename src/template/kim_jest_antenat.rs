use egui_knex::{
	egui::{Button, Color32, DragValue, Id, Label, Rect, ScrollArea, TextEdit, TextStyle, Ui, UiBuilder},
	element::{Vas, VasDispositio, VasFovea, VasRedundantia},
	pos2,
	vec2,
};

type Pozycja = (String, f64);
type Stan = (Vec<Pozycja>, f32); // (pozycje, przesunięcie listy)

const TLO_POZYCJI: Color32 = Color32::from_rgb(34, 36, 44);
const WYS_MIN_WIERSZA: f32 = 40.0;
const WYS_PASKA: f32 = 44.0;
const DYLATACJA_SLUP: f32 = 6.0;
const DYLATACJA_WERS: f32 = 8.0;
const ODSTEP_WIERSZY: f32 = 6.0;

fn uklad_wiersza(rect: Rect) -> VasDispositio {
	Vas::horizontalis(VasRedundantia::Compressio)
		.padding((DYLATACJA_WERS, DYLATACJA_WERS, DYLATACJA_SLUP, DYLATACJA_SLUP, DYLATACJA_WERS))
		.foveae([VasFovea::pixel(0, 28.0), VasFovea::auto(1), VasFovea::pixel(2, 96.0)])
		.resolve(rect)
}

pub fn tresc(ui: &mut Ui) {
	let id = Id::new("kim_jest_antenat_stan");
	let (mut pozycje, mut przesuniecie): Stan = ui.data(|d| d.get_temp(id)).unwrap_or_else(|| {
		(
			vec![("Pozycja 1".to_owned(), 0.0), ("Pozycja 2".to_owned(), 0.0), ("Pozycja 3".to_owned(), 0.0)],
			0.0,
		)
	});

	let glowny = Vas::verticalis(VasRedundantia::Compressio)
		.padding((12.0, 12.0, 12.0, 12.0, 12.0))
		.foveae([VasFovea::auto(0), VasFovea::pixel(1, WYS_PASKA)])
		.resolve(ui.max_rect());
	let lista_r = glowny.rect(0);
	let pasek_r = glowny.rect(1);

	// Szerokość etykiety (kolumna auto) potrzebna do policzenia wysokości
	// zawiniętego tekstu.
	let szer_etykiety = uklad_wiersza(Rect::from_min_size(pos2(0.0, 0.0), vec2(lista_r.width(), 1.0))).rect(1).width();

	let wysokosci: Vec<f32> = pozycje
		.iter()
		.map(|(tekst, _)| {
			let galeria =
				ui.painter().layout(tekst.clone(), TextStyle::Body.resolve(ui.style()), Color32::WHITE, szer_etykiety);
			(galeria.size().y + 2.0 * DYLATACJA_SLUP).max(WYS_MIN_WIERSZA)
		})
		.collect();

	let lista_vas = Vas::verticalis(VasRedundantia::Volutatio)
		.padding((0.0, 0.0, 0.0, 0.0, ODSTEP_WIERSZY))
		.foveae(wysokosci.iter().enumerate().map(|(i, &h)| VasFovea::pixel(i, h)));

	let max_przesuniecie = (lista_vas.total_content_length(lista_r) - lista_r.height()).max(0.0);
	if ui.rect_contains_pointer(lista_r) {
		przesuniecie -= ui.input(|i| i.smooth_scroll_delta.y);
	}
	przesuniecie = przesuniecie.clamp(0.0, max_przesuniecie);

	let mut usun = None;
	let mut lista_ui = ui.new_child(UiBuilder::new().max_rect(lista_r));
	lista_ui.set_clip_rect(lista_r);

	for (numerus, rect) in lista_vas.resolve_scroll(lista_r, przesuniecie) {
		let (tekst, liczba) = &mut pozycje[numerus];
		lista_ui.painter().rect_filled(rect, 6.0, TLO_POZYCJI);

		let uklad = uklad_wiersza(rect);
		if lista_ui.put(uklad.rect(0), Button::new("x")).clicked() {
			usun = Some(numerus);
		}
		lista_ui.put(uklad.rect(1), Label::new(tekst.as_str()).wrap());
		lista_ui.put(uklad.rect(2), DragValue::new(liczba).speed(1.0));
	}

	if let Some(i) = usun {
		pozycje.remove(i);
	}

	if ui.put(pasek_r, Button::new("dodaj pozycję")).clicked() {
		pozycje.push((String::new(), 0.0));
	}

	ui.data_mut(|d| d.insert_temp(id, (pozycje, przesuniecie)));
}
