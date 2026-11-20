"use strict";
// table.js — rysuje tabelę #tab01 z danych `dane` (tab01.js).
//
//  • lp      – numer relacji, przyklejony do lewej krawędzi (jak numery wierszy w Excelu),
//  • <thead> – nagłówki kolumn przyklejone do górnej krawędzi,
//  • konfigurator kolejności kolumn: przeciąganie kafelka / nagłówka albo przyciski ◀ ▶;
//    kolejność zapamiętywana w localStorage.

(() => {
	const tabela = document.getElementById("tab01");
	if (!tabela || typeof dane === "undefined") return;

	/* ════════════════════════════════════════════════════════════════════
	   1. Formatowanie zawartości komórek
	   ════════════════════════════════════════════════════════════════════ */

	// [["✾","+","5",["3","4"]], …] → wiersze z ułamkiem ⁿ⁄ₘ, łączone <br>
	const formatDec = (arr) => {
		if (!Array.isArray(arr)) return arr ?? "";
		return arr
			.map((row) => {
				const copy = [...row];
				const last = copy.length - 1;
				if (last >= 0 && Array.isArray(copy[last])) {
					const [frNum = "", frDen = ""] = copy[last];
					copy[last] = `<sup>${frNum}</sup>&frasl;<sub>${frDen}</sub>`;
				}
				return `<span class="nw">${copy.join(" ")}</span>`;
			})
			.join("<br>");
	};

	// [["+","10","0.1"], …] → ostatni człon (od 3. elementu) podkreślony, indeks ₂
	const formatBin = (arr) => {
		if (!Array.isArray(arr)) return arr ?? "";
		return arr
			.map((row) => {
				const copy = [...row];
				const last = copy.length - 1;
				if (last >= 2) {
					copy[last] = `<u>${copy[last]}</u>`;
				}
				return `<span class="nw">${copy.join(" ")}<sub>2</sub></span>`;
			})
			.join("<br>");
	};

	// wartość prosta albo { ego2, ego3 } → { isSplit, first, second }
	const parseCell = (val) => {
		const isObj = typeof val === "object" && val !== null;
		const hasEgo3 = isObj && val.ego3 !== undefined;
		const isSplit = hasEgo3 && val.ego2 !== val.ego3;
		return {
			isSplit,
			first: isObj ? (val.ego2 ?? "") : (val ?? ""),
			second: isSplit ? (val.ego3 ?? "") : "",
		};
	};

	const isObj = (v) => typeof v === "object" && v !== null;

	const getSosaFields = (s, legacyBin = undefined) => {
		// { ego2: { dec, bin }, ego3: { dec, bin } }
		if (isObj(s) && ("ego2" in s || "ego3" in s) && (isObj(s.ego2) || isObj(s.ego3))) {
			const hasEgo3 = s.ego3 !== undefined;
			const decObj = { ego2: formatDec(s.ego2?.dec), ...(hasEgo3 ? { ego3: formatDec(s.ego3?.dec) } : {}) };
			const binObj = { ego2: formatBin(s.ego2?.bin), ...(hasEgo3 ? { ego3: formatBin(s.ego3?.bin) } : {}) };
			return { dec: parseCell(decObj), bin: parseCell(binObj) };
		}
		// { dec, bin } bez podziału na ego2/ego3
		if (isObj(s) && ("dec" in s || "bin" in s)) {
			return { dec: parseCell(formatDec(s.dec)), bin: parseCell(formatBin(s.bin)) };
		}
		// stary zapis: osobne sosa i sbin
		return { dec: parseCell(s), bin: parseCell(legacyBin) };
	};

	// wszystkie pola relacji, które mogą mieć podział ego2/ego3
	const komorkiRelacji = (rel) => {
		const { dec: sosaDec, bin: sosaBin } = getSosaFields(rel.sosa, rel.sbin);
		const { dec: sosaRelDec, bin: sosaRelBin } = getSosaFields(rel.sosa_rel);
		return {
			sosaDec,
			sosaBin,
			sosaRelDec,
			sosaRelBin,
			pl: parseCell(rel.pl),
			la: parseCell(rel.la),
			en: parseCell(rel.en),
		};
	};

	/* ════════════════════════════════════════════════════════════════════
	   1b. Generator otoczenia ↂn (ten sam układ obiektów co `dane`)
	   ────────────────────────────────────────────────────────────────────
	   klasa  = P^a G^b C^c,  b ∈ {0,1},  n = a + b + c
	   g      = c − a  (każde g od −n do +n daje dokładnie jedną klasę)
	   sp     = a + 2b + c,  spg = n
	   rp     = w (same P) / z (same C) / b (pozostałe)
	   CISO   r = a − c:
				r ≥ 0 → A = +ścieżka(ego, bity[0..r)),  B = 1‖bity[r..n)
				r < 0 → A = −ścieżka(ego, bity[k..n)),  B = 1‖bity[0..k),  k = 2a + b
	   ogniwo = same P → A>>1;  koniec na G → para rodziców;  inaczej CISO prefiksu
	   Odtwarza ↂ1–ↂ3 z tab01.js bez różnic (poza r = odsyłaczem, nazwami i kolejnością w ᛝPPP, ᛝGCC).
	   ════════════════════════════════════════════════════════════════════ */

	const MINUS = "−";
	const LITERY = { P: ["F", "M"], G: ["B", "Z"], C: ["S", "D"] };
	const PLEC = ["♂️", "♀️"]; // ♂️ ♀️

	const sciezka = (ego, bity) => bity.reduce((x, b) => 2 * x + b, ego);

	// słowo klasy (np. "PPGC") + bity płci → [A, B | null]
	const ciso = (slowo, bity, ego) => {
		if (!slowo) return [ego, null];
		const ile = (l) => [...slowo].filter((x) => x === l).length;
		const a = ile("P"), b = ile("G"), c = ile("C");
		if (b === 0 && c === 0) return [sciezka(ego, bity), null];
		if (a === 0 && b === 0) return [-sciezka(ego, bity), null];
		const r = a - c;
		if (r >= 0) return [sciezka(ego, bity.slice(0, r)), parseInt("1" + bity.slice(r).join(""), 2)];
		const k = 2 * a + b;
		return [-sciezka(ego, bity.slice(k)), parseInt("1" + bity.slice(0, k).join(""), 2)];
	};

	const znak = (A) => (A < 0 ? MINUS : "+");
	const ulamek = (B) => {
		const L = Math.floor(Math.log2(B));
		return [String(2 * (B - 2 ** L) + 1), String(2 ** (L + 1))];
	};
	const zapisDec = ([A, B], gwiazdka) => [
		...(gwiazdka ? ["✾"] : []), znak(A), String(Math.abs(A)), ...(B === null ? [] : [ulamek(B)]),
	];
	const zapisBin = ([A, B]) => {
		const s = Math.abs(A).toString(2);
		const reszta = s.slice(2) + (B === null ? "" : "." + B.toString(2).slice(1) + "1");
		return reszta ? [znak(A), s.slice(0, 2), reszta] : [znak(A), s.slice(0, 2)];
	};

	// poprzednie ogniwo sosa
	const ogniwo = (slowo, bity, ego) => {
		if (!/[GC]/.test(slowo)) return ["+", String(sciezka(ego, bity) >> 1)];
		if (slowo.endsWith("G")) {
			const p = sciezka(ego, bity.slice(0, -1));
			return ["+", `${2 * p}&${2 * p + 1}`];
		}
		return zapisDec(ciso(slowo.slice(0, -1), bity.slice(0, -1), ego), false);
	};

	// klasy otoczenia n w kolejności g = −n … +n
	const klasyOtoczenia = (n) =>
		Array.from({ length: 2 * n + 1 }, (_, i) => {
			const g = i - n;
			const b = (n + g) % 2 === 0 ? 0 : 1;
			const a = (n - b - g) / 2, c = (n - b + g) / 2;
			return { a, b, c, g, slowo: "P".repeat(a) + "G".repeat(b) + "C".repeat(c) };
		});

	// kolejność relacji w klasie: gdy a > c — KNOT czytany od krewnego ku ego, inaczej od ego
	const wzorcePlci = (n, odKonca) => {
		const klucz = (bity) => sciezka(0, odKonca ? [...bity].reverse() : bity);
		return Array.from({ length: 2 ** n }, (_, i) => [...i.toString(2).padStart(n, "0")].map(Number))
			.sort((x, y) => klucz(x) - klucz(y));
	};

	const generujOtoczenie = (n) => ({
		dystans: `ↂ${n}`,
		grupy: klasyOtoczenia(n).map(({ a, b, c, g, slowo }) => ({
			r: " ", // odsyłacz do dokumentacji — dla otoczeń generowanych brak
			pokolenie: `䷀${Math.abs(g)}<sub>${g < 0 ? "+" : g > 0 ? MINUS : "="}</sub>`,
			otoczenie: `ↂ${n}`,
			symbol: "ᛝ" + slowo,
			rp: b === 0 && c === 0 ? "w" : a === 0 && b === 0 ? "z" : "b",
			sp: String(a + 2 * b + c),
			spg: String(n),
			relacje: wzorcePlci(n, a > c).map((bity) => {
				const sosa = (ego) => {
					const v = ciso(slowo, bity, ego);
					return { dec: [zapisDec(v, true)], bin: [zapisBin(v)] };
				};
				return {
					sex: PLEC[bity[n - 1]],
					knot: "ᛟ" + bity.map((x, i) => LITERY[slowo[i]][x]).join(""),
					sosa: { ego2: sosa(2), ego3: sosa(3) },
					sosa_rel: { ego2: { dec: [ogniwo(slowo, bity, 2)] }, ego3: { dec: [ogniwo(slowo, bity, 3)] } },
					en: " ",
					la: " ",
					pl: " ",
				};
			}),
		})),
	});

	/* ════════════════════════════════════════════════════════════════════
	   2. Definicje kolumn
	   ────────────────────────────────────────────────────────────────────
	   rodzaj "G" – komórka grupy: tylko w 1. wierszu grupy, rowspan = wiersze grupy
	   rodzaj "R" – komórka relacji: rowspan = wiersze relacji (1 lub 2)
	   rodzaj "S" – komórka dzielona: ego2 w 1. wierszu, ego3 w 2. (gdy różne)
	   x          – wartość atrybutu x (kolor z table.css); "R" dokleja przyrostek płci,
					"S" przyrostek ego2/ego3
	   ════════════════════════════════════════════════════════════════════ */

	const KOLUMNY = {
		r: { th: "L§", rodzaj: "G", tresc: (g) => g.r || "" },
		rpsp: { th: "L°", rodzaj: "G", tresc: (g) => `<b>rp:</b>${g.rp}<hr/><b>sp:</b>${g.sp}<hr/><b>spg:<br/>${g.spg || ""}</b>` },
		//pokolenie: { th: "pokolenie", rodzaj: "G", x: "TIER", tresc: (g) => g.pokolenie ?? "" },
		otoczenie: { th: "Lᯤ", rodzaj: "G", x: "RING", tresc: (g) => `${g.otoczenie || ""}<hr/>${g.pokolenie || ""}` },
		symbol: { th: "Kᛝ", rodzaj: "G", x: "KNOT_HEAP", tresc: (g) => `${g.symbol}` },
		sex: { th: "Kᛯ", rodzaj: "R", x: "SEX", tresc: (rel) => rel.sex ?? "-" },
		knot: { th: "Kᛟ", rodzaj: "R", x: "KNOT_ATOM", tresc: (rel) => `${rel.knot}` },
		sosaDec: { th: "S✾<sub>10</sub>", rodzaj: "S", x: "SOSA_DEC" },
		sosaBin: { th: "S✾<sub>2</sub>", rodzaj: "S", x: "SOSA_BIN" },
		sosaRelDec: { th: "SR※<sub>10</sub>", rodzaj: "S", x: "SOSA_REL_DEC", owin: (t) => `⦗${t}⦘` },
		// sosaRelBin: { th: "SOSA_REL_BIN",  rodzaj: "S", x: "SOSA_REL_BIN" },
		pl: { th: "PL", rodzaj: "S" },
		la: { th: "LA", rodzaj: "S" },
		en: { th: "EN", rodzaj: "S" },
	};

	// Bloki = jednostki przesuwania. Blok "rdzen" ma stały układ wewnętrzny.
	const BLOKI = {
		r1: { nazwa: "r", kolumny: ["r"] },
		rpsp: { nazwa: "stopnie", kolumny: ["rpsp"] },
		rdzen: {
			nazwa: "symbole",
			kolumny: ["otoczenie", "symbol", "sex", "knot", "sosaDec", "sosaBin", "sosaRelDec"]
		},
		pl: { nazwa: "PL", kolumny: ["pl"] },
		la: { nazwa: "LA", kolumny: ["la"] },
		en: { nazwa: "EN", kolumny: ["en"] },
		//r2: { nazwa: "r", kolumny: ["r"] },
	};
	const DOMYSLNA = ["r1", "rdzen", "pl", "la", "en", "rpsp"/*, "r2"*/];

	const KLUCZ = "tab01.kolejnoscKolumn";
	const wczytaj = () => {
		try {
			const v = JSON.parse(localStorage.getItem(KLUCZ));
			if (Array.isArray(v) && v.length === DOMYSLNA.length && DOMYSLNA.every((id) => v.includes(id))) return v;
		} catch (_) { /* brak dostępu do localStorage */ }
		return [...DOMYSLNA];
	};
	const zapisz = (v) => {
		try { localStorage.setItem(KLUCZ, JSON.stringify(v)); } catch (_) { /* j.w. */ }
	};

	let kolejnosc = wczytaj();

	// otoczenia generowane skryptem (ptaszki w ustawieniach)
	const OTOCZENIA_GEN = [4, 5];
	const KLUCZ_OT = "tab01.otoczenia";
	const wczytajOtoczenia = () => {
		try {
			const v = JSON.parse(localStorage.getItem(KLUCZ_OT));
			if (Array.isArray(v)) return OTOCZENIA_GEN.filter((n) => v.includes(n));
		} catch (_) { /* brak dostępu do localStorage */ }
		return [];
	};
	let otoczenia = wczytajOtoczenia();

	const wygenerowane = new Map();
	const jestWDanych = (n) => dane.some((s) => s.dystans === `ↂ${n}`);
	const daneTabeli = () => [
		...dane,
		...otoczenia.filter((n) => !jestWDanych(n)).map((n) => {
			if (!wygenerowane.has(n)) wygenerowane.set(n, generujOtoczenie(n));
			return wygenerowane.get(n);
		}),
	];

	/* ════════════════════════════════════════════════════════════════════
	   3. Rysowanie tabeli
	   ════════════════════════════════════════════════════════════════════ */

	const atr = (nazwa, wartosc) => (wartosc === undefined || wartosc === "" ? "" : ` ${nazwa}="${wartosc}"`);
	const xAtr = (kol, przyrostek) => (kol.x ? atr("x", kol.x + przyrostek) : "");
	const owin = (kol, t) => (kol.owin ? kol.owin(t) : t);

	// kolumny w bieżącej kolejności: [[idBloku, idKolumny, definicja], …]
	const ukladKolumn = () =>
		kolejnosc.flatMap((idB) => BLOKI[idB].kolumny.map((idK) => [idB, idK, KOLUMNY[idK]]));

	// klucze pól dzielonych, które faktycznie są w tabeli
	const KLUCZE_S = Object.keys(KOLUMNY).filter((k) => KOLUMNY[k].rodzaj === "S");

	// 2 wiersze, gdy którakolwiek komórka dzielona ma różne ego2/ego3
	const liczbaWierszy = (k) => (KLUCZE_S.some((klucz) => k[klucz].isSplit) ? 2 : 1);

	const rysujNaglowek = (uklad) => {
		let thead = tabela.tHead;
		if (!thead) thead = tabela.createTHead();
		thead.innerHTML =
			`<tr><th class="lp">lp</th>` +
			uklad.map(([idB, , kol]) => `<th draggable="true" data-blok="${idB}" title="przeciągnij, aby przesunąć">${kol.th}</th>`).join("") +
			`</tr>`;
	};

	const rysujCialo = (uklad) => {
		tabela.querySelectorAll(":scope > tbody").forEach((tb) => tb.remove());
		let lp = 0;

		daneTabeli().forEach((sekcja) => {
			let html = `<tr><td class="lp"></td><td class="ac dystans-naglowek" x="RING" colspan="${uklad.length}">${sekcja.dystans}</td></tr>`;

			sekcja.grupy.forEach((grupa) => {
				const komorki = grupa.relacje.map(komorkiRelacji);
				const wierszeGrupy = komorki.reduce((suma, k) => suma + liczbaWierszy(k), 0);

				grupa.relacje.forEach((rel, index) => {
					const k = komorki[index];
					const span = liczbaWierszy(k);

					const kobieta = rel.sex?.startsWith("♀");
					const pR = kobieta ? "_WW" : "";  // SEX, KNOT
					const pS1 = kobieta ? "_WM" : ""; // SOSA_* – ego2
					const pS2 = kobieta ? "_WW" : "_MW"; // SOSA_* – ego3

					let w1 = `<td class="lp" rowspan="${span}">${++lp}</td>`;
					let w2 = "";

					for (const [, idK, kol] of uklad) {
						if (kol.rodzaj === "G") {
							if (index === 0) w1 += `<td${atr("rowspan", wierszeGrupy)}${xAtr(kol, "")}>${kol.tresc(grupa)}</td>`;
						} else if (kol.rodzaj === "R") {
							w1 += `<td${atr("rowspan", span)}${xAtr(kol, pR)}>${kol.tresc(rel)}</td>`;
						} else {
							const c = k[idK];
							w1 += `<td${c.isSplit ? "" : atr("rowspan", span)}${xAtr(kol, pS1)}>${owin(kol, c.first)}</td>`;
							if (c.isSplit) w2 += `<td${xAtr(kol, pS2)}>${owin(kol, c.second)}</td>`;
						}
					}

					html += `<tr>${w1}</tr>`;
					if (span === 2) html += `<tr>${w2}</tr>`;
				});
			});

			const tbody = document.createElement("tbody");
			tbody.innerHTML = html;
			tabela.appendChild(tbody);
		});
	};

	/* ════════════════════════════════════════════════════════════════════
	   4. Konfigurator kolejności kolumn
	   ════════════════════════════════════════════════════════════════════ */

	const pudlo = tabela.closest(".table-box") ?? tabela;
	const panel = document.createElement("details");
	panel.className = "konfig-kolumn";
	panel.innerHTML =
		`<summary>⚙️ Ustawienia</summary>` +
		`<div class="kk-lista"></div>` +
		`<div class="kk-stopka">` +
		`<button type="button" class="kk-reset">Przywróć domyślną</button>` +
		`<span>Przeciągnij kafelek albo nagłówek kolumny w tabeli, lub użyj ◀ ▶.</span>` +
		`</div>` +
		`<div class="kk-stopka kk-otoczenia">` +
		`<b>Generuj otoczenia:</b>` +
		OTOCZENIA_GEN.map((n) => `<label><input type="checkbox" data-otoczenie="${n}"> ↂ${n}</label>`).join("") +
		`</div>`;
	pudlo.before(panel);
	const lista = panel.querySelector(".kk-lista");

	const rysujPanel = () => {
		const n = kolejnosc.length;
		lista.innerHTML = kolejnosc
			.map((idB, i) => {
				const b = BLOKI[idB];
				const tytul = b.kolumny.map((idK) => KOLUMNY[idK].th).join(" · ");
				return (
					`<span class="kk-kafel" draggable="true" data-blok="${idB}" title="${tytul}">` +
					`<button type="button" data-ruch="-1"${i === 0 ? " disabled" : ""} aria-label="w lewo">◀</button>` +
					`<span class="kk-nazwa">${b.nazwa}</span>` +
					`<button type="button" data-ruch="1"${i === n - 1 ? " disabled" : ""} aria-label="w prawo">▶</button>` +
					`</span>`
				);
			})
			.join("");
	};

	const rysuj = () => {
		const { scrollLeft, scrollTop } = pudlo;
		const uklad = ukladKolumn();
		rysujNaglowek(uklad);
		rysujCialo(uklad);
		rysujPanel();
		pudlo.scrollLeft = scrollLeft;
		pudlo.scrollTop = scrollTop;
	};

	const ustaw = (nowa) => {
		kolejnosc = nowa;
		zapisz(nowa);
		rysuj();
	};

	// przenieś blok `id` przed/po blok `cel`
	const przenies = (id, cel, po) => {
		if (!id || id === cel) return;
		const k = kolejnosc.filter((x) => x !== id);
		k.splice(k.indexOf(cel) + (po ? 1 : 0), 0, id);
		if (k.join() !== kolejnosc.join()) ustaw(k);
	};

	// przesuń blok `id` o `d` pozycji (−1 / +1)
	const przesun = (id, d) => {
		const i = kolejnosc.indexOf(id);
		const j = i + d;
		if (i < 0 || j < 0 || j >= kolejnosc.length) return;
		const k = [...kolejnosc];
		[k[i], k[j]] = [k[j], k[i]];
		ustaw(k);
	};

	lista.addEventListener("click", (e) => {
		const btn = e.target.closest("button[data-ruch]");
		if (btn) przesun(btn.closest("[data-blok]").dataset.blok, Number(btn.dataset.ruch));
	});
	panel.querySelector(".kk-reset").addEventListener("click", () => ustaw([...DOMYSLNA]));

	panel.querySelectorAll("input[data-otoczenie]").forEach((pole) => {
		pole.checked = otoczenia.includes(Number(pole.dataset.otoczenie));
		pole.addEventListener("change", () => {
			otoczenia = OTOCZENIA_GEN.filter((n) => panel.querySelector(`input[data-otoczenie="${n}"]`).checked);
			try { localStorage.setItem(KLUCZ_OT, JSON.stringify(otoczenia)); } catch (_) { /* j.w. */ }
			rysuj();
		});
	});

	// ── przeciąganie (wspólne dla kafelków i nagłówków) ──
	let ciagniety = null;

	const czysc = () => {
		document.querySelectorAll(".kk-ciagniety, .kk-przed, .kk-po").forEach((el) =>
			el.classList.remove("kk-ciagniety", "kk-przed", "kk-po"));
	};

	// elementy bloku `id` w kontenerze + czy kursor jest za środkiem bloku
	const cel = (kontener, id, x) => {
		const els = kontener.querySelectorAll(`[data-blok="${id}"]`);
		const l = els[0].getBoundingClientRect().left;
		const r = els[els.length - 1].getBoundingClientRect().right;
		return { pierwszy: els[0], ostatni: els[els.length - 1], po: x > (l + r) / 2 };
	};

	const podlaczPrzeciaganie = (kontener) => {
		kontener.addEventListener("dragstart", (e) => {
			const el = e.target.closest?.("[data-blok]");
			if (!el) return;
			ciagniety = el.dataset.blok;
			e.dataTransfer.effectAllowed = "move";
			e.dataTransfer.setData("text/plain", ciagniety);
			kontener.querySelectorAll(`[data-blok="${ciagniety}"]`).forEach((x) => x.classList.add("kk-ciagniety"));
		});
		kontener.addEventListener("dragover", (e) => {
			const el = ciagniety && e.target.closest?.("[data-blok]");
			if (!el) return;
			e.preventDefault();
			e.dataTransfer.dropEffect = "move";
			const c = cel(kontener, el.dataset.blok, e.clientX);
			kontener.querySelectorAll(".kk-przed, .kk-po").forEach((x) => x.classList.remove("kk-przed", "kk-po"));
			if (el.dataset.blok !== ciagniety) (c.po ? c.ostatni : c.pierwszy).classList.add(c.po ? "kk-po" : "kk-przed");
		});
		kontener.addEventListener("drop", (e) => {
			const el = ciagniety && e.target.closest?.("[data-blok]");
			if (!el) return;
			e.preventDefault();
			const id = ciagniety;
			const c = cel(kontener, el.dataset.blok, e.clientX);
			ciagniety = null;
			czysc();
			przenies(id, el.dataset.blok, c.po);
		});
	};

	podlaczPrzeciaganie(lista);
	podlaczPrzeciaganie(tabela.tHead ?? tabela.createTHead());
	document.addEventListener("dragend", () => { ciagniety = null; czysc(); });

	rysuj();
})();