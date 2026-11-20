//🅰️🅱️🆎🅾️

const dane = [
	{
		dystans: "ↂ0",
		grupy: [
			{
				r: " ",
				pokolenie: "🔀<br/>䷀0<sub>=</sub><br/>(=0)",
				otoczenie: "ↂ0",
				symbol: "ᛝ",
				rp: " ",
				sp: " ",
				spg: " ",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟ",
						sosa: { dec: [["✾", "+", "2"]], bin: [["+", "10"]] },
						sosa_rel: { dec: [["✾"]] },
						en: " ",
						la: " ",
						pl: "probant męski",
					},
					{
						sex: "♀️",
						knot: "ᛟ",
						sosa: { dec: [["✾", "+", "3"]], bin: [["+", "11"]] },
						sosa_rel: { dec: [["✾"]] },
						en: " ",
						la: " ",
						pl: "probant żeński",
					},
				],
			},
		],
	},
	{
		dystans: "ↂ1",
		grupy: [
			{
				r: "1.1",
				pokolenie: "⤵️<br/>䷀1<sub>+</sub><br/>(−1)",
				otoczenie: "ↂ1",
				symbol: "ᛝP",
				rp: "w",
				sp: "1",
				spg: "1",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟF",
						sosa: {
							ego2: { dec: [["✾", "+", "4"]], bin: [["+", "10", "0"]] },
							ego3: { dec: [["✾", "+", "6"]], bin: [["+", "11", "0"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "2"]] },
							ego3: { dec: [["+", "3"]] },
						},
						en: "father",
						la: "pater",
						pl: "ojciec",
					},
					{
						sex: "♀️",
						knot: "ᛟM",
						sosa: {
							ego2: { dec: [["✾", "+", "5"]], bin: [["+", "10", "1"]] },
							ego3: { dec: [["✾", "+", "7"]], bin: [["+", "11", "1"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "2"]] },
							ego3: { dec: [["+", "3"]] },
						},
						en: "mother",
						la: "māter",
						pl: "matka",
					},
				],
			},
			{
				r: "1.2",
				pokolenie: "🔀<br/>䷀0<sub>=</sub><br/>(=0)",
				otoczenie: "ↂ1",
				symbol: "ᛝG",
				rp: "b",
				sp: "2",
				spg: "1",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟB",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*2*/["1", "4"]]],
								bin: [["+", "10", ".01"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*2*/["1", "4"]]],
								bin: [["+", "11", ".01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2"]] },
							ego3: { dec: [["+", "3"]] },
						},
						en: "brother",
						la: "frāter",
						pl: "brat",
					},
					{
						sex: "♀️",
						knot: "ᛟZ",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*3*/["3", "4"]]],
								bin: [["+", "10", ".11"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*3*/["3", "4"]]],
								bin: [["+", "11", ".11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2"]] },
							ego3: { dec: [["+", "3"]] },
						},
						en: "sister",
						la: "soror",
						pl: "siostra",
					},
				],
			},
			{
				r: "1.1",
				pokolenie: "⤴️<br/>䷀1<sub>−</sub><br/>(+1)",
				otoczenie: "ↂ1",
				symbol: "ᛝC",
				rp: "z",
				sp: "1",
				spg: "1",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟS",
						sosa: {
							ego2: { dec: [["✾", "−", "4"]], bin: [["−", "10", "0"]] },
							ego3: { dec: [["✾", "−", "6"]], bin: [["−", "11", "0"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "2"]] },
							ego3: { dec: [["+", "3"]] },
						},
						en: "son",
						la: "fīlius",
						pl: "syn",
					},
					{
						sex: "♀️",
						knot: "ᛟD",
						sosa: {
							ego2: { dec: [["✾", "−", "5"]], bin: [["−", "10", "1"]] },
							ego3: { dec: [["✾", "−", "7"]], bin: [["−", "11", "1"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "2"]] },
							ego3: { dec: [["+", "3"]] },
						},
						en: "daughter",
						la: "fīlia",
						pl: "córka",
					},
				],
			},
		],
	},
	{
		dystans: "ↂ2",
		grupy: [
			{
				r: "2.1",
				pokolenie: "⤵️<br/>䷀2<sub>+</sub><br/>(−2)",
				otoczenie: "ↂ2",
				symbol: "ᛝPP",
				rp: "w",
				sp: "2",
				spg: "2",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFF",
						sosa: {
							ego2: { dec: [["✾", "+", "8"]], bin: [["+", "10", "00"]] },
							ego3: { dec: [["✾", "+", "12"]], bin: [["+", "11", "00"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "4"]] },
							ego3: { dec: [["+", "6"]] },
						},
						en: "grandfather",
						la: "avus",
						pl: "dziadek ojczysty",
					},
					{
						sex: "♂️",
						knot: "ᛟMF",
						sosa: {
							ego2: { dec: [["✾", "+", "10"]], bin: [["+", "10", "10"]] },
							ego3: { dec: [["✾", "+", "14"]], bin: [["+", "11", "10"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "5"]] },
							ego3: { dec: [["+", "7"]] },
						},
						en: "grandfather",
						la: "avus",
						pl: "dziadek macierzysty",
					},
					{
						sex: "♀️",
						knot: "ᛟFM",
						sosa: {
							ego2: { dec: [["✾", "+", "9"]], bin: [["+", "10", "01"]] },
							ego3: { dec: [["✾", "+", "13"]], bin: [["+", "11", "01"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "4"]] },
							ego3: { dec: [["+", "6"]] },
						},
						en: "grandmother",
						la: "avia",
						pl: "babka ojczysta",
					},
					{
						sex: "♀️",
						knot: "ᛟMM",
						sosa: {
							ego2: { dec: [["✾", "+", "11"]], bin: [["+", "10", "11"]] },
							ego3: { dec: [["✾", "+", "15"]], bin: [["+", "11", "11"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "5"]] },
							ego3: { dec: [["+", "7"]] },
						},
						en: "grandmother",
						la: "avia",
						pl: "babka macierzysta",
					},
				],
			},

			{
				r: "2.3",
				pokolenie: "⤵️<br/>䷀1<sub>+</sub><br/>(−1)",
				otoczenie: "ↂ2",
				symbol: "ᛝPG",
				rp: "b",
				sp: "3",
				spg: "2",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFB",
						sosa: {
							ego2: {
								dec: [["✾", "+", "4", /*2*/["1", "4"]]],
								bin: [["+", "10", "0.01"]],
							},
							ego3: {
								dec: [["✾", "+", "6", /*2*/["1", "4"]]],
								bin: [["+", "11", "0.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "8&9"]] },
							ego3: { dec: [["+", "12&13"]] },
						},
						en: "uncle",
						la: "patruus",
						pl: "stryj <br/> (brat ojca)",
					},
					{
						sex: "♂️",
						knot: "ᛟMB",
						sosa: {
							ego2: {
								dec: [["✾", "+", "5", /*2*/["1", "4"]]],
								bin: [["+", "10", "1.01"]],
							},
							ego3: {
								dec: [["✾", "+", "7", /*2*/["1", "4"]]],
								bin: [["+", "11", "1.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "10&11"]] },
							ego3: { dec: [["+", "14&15"]] },
						},
						en: "uncle",
						la: "avunculus",
						pl: "wuj <br/> (brat matki)",
					},
					{
						sex: "♀️",
						knot: "ᛟFZ",
						sosa: {
							ego2: {
								dec: [["✾", "+", "4", /*3*/["3", "4"]]],
								bin: [["+", "10", "0.11"]],
							},
							ego3: {
								dec: [["✾", "+", "6", /*3*/["3", "4"]]],
								bin: [["+", "11", "0.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "8&9"]] },
							ego3: { dec: [["+", "12&13"]] },
						},
						en: "aunt",
						la: "amita",
						pl: "ciotka <br/> (siostra ojca)",
					},
					{
						sex: "♀️",
						knot: "ᛟMZ",
						sosa: {
							ego2: {
								dec: [["✾", "+", "5", /*3*/["3", "4"]]],
								bin: [["+", "10", "1.11"]],
							},
							ego3: {
								dec: [["✾", "+", "7", /*3*/["3", "4"]]],
								bin: [["+", "11", "1.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "10&11"]] },
							ego3: { dec: [["+", "14&15"]] },
						},
						en: "aunt",
						la: "mātertera",
						pl: "sioła <br/> (siostra matki)",
					},
				],
			},
			{
				r: "2.2",
				pokolenie: "🔀<br/>䷀0<sub>=</sub><br/>(=0)",
				otoczenie: "ↂ2",
				symbol: "ᛝPC",
				rp: "b",
				sp: "2",
				spg: "2",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*4*/["1", "8"]]],
								bin: [["+", "10", ".001"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*4*/["1", "8"]]],
								bin: [["+", "11", ".001"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "4"]] },
							ego3: { dec: [["+", "6"]] },
						},
						en: "half brother",
						la: "frāter āgnātus",
						pl: "półbrat",
					},
					{
						sex: "♀️",
						knot: "ᛟFD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*5*/["3", "8"]]],
								bin: [["+", "10", ".011"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*5*/["3", "8"]]],
								bin: [["+", "11", ".011"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "4"]] },
							ego3: { dec: [["+", "6"]] },
						},
						en: "half sister",
						la: "soror āgnāta",
						pl: "półsiostra",
					},
					{
						sex: "♂️",
						knot: "ᛟMS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*6*/["5", "8"]]],
								bin: [["+", "10", ".101"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*6*/["5", "8"]]],
								bin: [["+", "11", ".101"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "5"]] },
							ego3: { dec: [["+", "7"]] },
						},
						en: "half brother",
						la: "frāter āgnātus",
						pl: "półbrat",
					},
					{
						sex: "♀️",
						knot: "ᛟMD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*7*/["7", "8"]]],
								bin: [["+", "10", ".111"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*7*/["7", "8"]]],
								bin: [["+", "11", ".111"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "5"]] },
							ego3: { dec: [["+", "7"]] },
						},
						en: "half sister",
						la: "soror āgnāta",
						pl: "półsiostra",
					},
				],
			},

			{
				r: "2.3",
				pokolenie: "⤴️<br/>䷀1<sub>−</sub><br/>(+1)",
				otoczenie: "ↂ2",
				symbol: "ᛝGC",
				rp: "b",
				sp: "3",
				spg: "2",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟBS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "4", /*2*/["1", "4"]]],
								bin: [["−", "10", "0.01"]],
							},
							ego3: {
								dec: [["✾", "−", "6", /*2*/["1", "4"]]],
								bin: [["−", "11", "0.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*2*/["1", "4"]]] },
							ego3: { dec: [["+", "3", /*2*/["1", "4"]]] },
						},
						en: "nephew",
						la: "frātruēlis",
						pl: { ego2: "synowiec <br/> (brataniec)", ego3: "brataniec" },
					},
					{
						sex: "♀️",
						knot: "ᛟBD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "5", /*2*/["1", "4"]]],
								bin: [["−", "10", "1.01"]],
							},
							ego3: {
								dec: [["✾", "−", "7", /*2*/["1", "4"]]],
								bin: [["−", "11", "1.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*2*/["1", "4"]]] },
							ego3: { dec: [["+", "3", /*2*/["1", "4"]]] },
						},
						en: "niece",
						la: "frātruēlis",
						pl: { ego2: "synowica <br/> (bratanica)", ego3: "bratanica" },
					},
					{
						sex: "♂️",
						knot: "ᛟZS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "4", /*3*/["3", "4"]]],
								bin: [["−", "10", "0.11"]],
							},
							ego3: {
								dec: [["✾", "−", "6", /*3*/["3", "4"]]],
								bin: [["−", "11", "0.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*2*/["3", "4"]]] },
							ego3: { dec: [["+", "3", /*2*/["3", "4"]]] },
						},
						en: "nephew",
						la: "sobrīnus",
						pl: {
							ego2: "siostrzeniec",
							ego3: "córowiec<sup>*</sup> <br/> (siostrzeniec)",
						},
					},
					{
						sex: "♀️",
						knot: "ᛟZD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "5", /*3*/["3", "4"]]],
								bin: [["−", "10", "1.11"]],
							},
							ego3: {
								dec: [["✾", "−", "7", /*3*/["3", "4"]]],
								bin: [["−", "11", "1.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*2*/["3", "4"]]] },
							ego3: { dec: [["+", "3", /*2*/["3", "4"]]] },
						},
						en: "niece",
						la: "sobrīna",
						pl: {
							ego2: "siostrzenica",
							ego3: "córowica<sup>*</sup> <br/> (siostrzenica)",
						},
					},
				],
			},
			{
				r: "2.1",
				pokolenie: "⤴️<br/>䷀2<sub>−</sub><br/>(+2)",
				otoczenie: "ↂ2",
				symbol: "ᛝCC",
				rp: "z",
				sp: "2",
				spg: "2",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟSS",
						sosa: {
							ego2: { dec: [["✾", "−", "8"]], bin: [["−", "10", "00"]] },
							ego3: { dec: [["✾", "−", "12"]], bin: [["−", "11", "00"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "4"]] },
							ego3: { dec: [["−", "6"]] },
						},
						en: "grandson",
						la: "nepōs",
						pl: "wnuk",
					},
					{
						sex: "♀️",
						knot: "ᛟSD",
						sosa: {
							ego2: { dec: [["✾", "−", "9"]], bin: [["−", "10", "01"]] },
							ego3: { dec: [["✾", "−", "13"]], bin: [["−", "11", "01"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "4"]] },
							ego3: { dec: [["−", "6"]] },
						},
						en: "granddaughter",
						la: "neptis",
						pl: "wnuczka",
					},
					{
						sex: "♂️",
						knot: "ᛟDS",
						sosa: {
							ego2: { dec: [["✾", "−", "10"]], bin: [["−", "10", "10"]] },
							ego3: { dec: [["✾", "−", "14"]], bin: [["−", "11", "10"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "5"]] },
							ego3: { dec: [["−", "7"]] },
						},
						en: "grandson",
						la: "nepōs",
						pl: "wnuk",
					},
					{
						sex: "♀️",
						knot: "ᛟDD",
						sosa: {
							ego2: { dec: [["✾", "−", "11"]], bin: [["−", "10", "11"]] },
							ego3: { dec: [["✾", "−", "15"]], bin: [["−", "11", "11"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "5"]] },
							ego3: { dec: [["−", "7"]] },
						},
						en: "granddaughter",
						la: "neptis",
						pl: "wnuczka",
					},
				],
			},
		],
	},

	{
		dystans: "ↂ3",
		grupy: [
			{
				r: "3.1",
				pokolenie: "⤵️<br/>䷀3<sub>+</sub><br/>(−3)",
				otoczenie: "ↂ3",
				symbol: "ᛝPPP",
				rp: "w",
				sp: "3",
				spg: "3",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFFF",
						sosa: {
							ego2: { dec: [["✾", "+", "16"]], bin: [["+", "10", "000"]] },
							ego3: { dec: [["✾", "+", "24"]], bin: [["+", "11", "000"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "8"]] },
							ego3: { dec: [["+", "12"]] },
						},
						en: "great grandfather",
						la: "proavus",
						pl: "pradziadek ojczysto-ojczysty",
					},
					{
						sex: "♂️",
						knot: "ᛟFMF",
						sosa: {
							ego2: { dec: [["✾", "+", "18"]], bin: [["+", "10", "010"]] },
							ego3: { dec: [["✾", "+", "26"]], bin: [["+", "11", "010"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "9"]] },
							ego3: { dec: [["+", "13"]] },
						},
						en: "great grandfather",
						la: "proavus",
						pl: "pradziadek macierzysto-ojczysty",
					},
					{
						sex: "♂️",
						knot: "ᛟMFF",
						sosa: {
							ego2: { dec: [["✾", "+", "20"]], bin: [["+", "10", "100"]] },
							ego3: { dec: [["✾", "+", "28"]], bin: [["+", "11", "100"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "10"]] },
							ego3: { dec: [["+", "14"]] },
						},
						en: "great grandfather",
						la: "proavus",
						pl: "pradziadek ojczysto-macierzysty",
					},
					{
						sex: "♂️",
						knot: "ᛟMMF",
						sosa: {
							ego2: { dec: [["✾", "+", "22"]], bin: [["+", "10", "110"]] },
							ego3: { dec: [["✾", "+", "30"]], bin: [["+", "11", "110"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "11"]] },
							ego3: { dec: [["+", "15"]] },
						},
						en: "great grandfather",
						la: "proavus",
						pl: "pradziadek macierzysto-macierzysty",
					},
					{
						sex: "♀️",
						knot: "ᛟFFM",
						sosa: {
							ego2: { dec: [["✾", "+", "17"]], bin: [["+", "10", "001"]] },
							ego3: { dec: [["✾", "+", "25"]], bin: [["+", "11", "001"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "8"]] },
							ego3: { dec: [["+", "12"]] },
						},
						en: "great grandmother",
						la: "proavia",
						pl: "prababka ojczysto-ojczysta",
					},
					{
						sex: "♀️",
						knot: "ᛟFMM",
						sosa: {
							ego2: { dec: [["✾", "+", "19"]], bin: [["+", "10", "011"]] },
							ego3: { dec: [["✾", "+", "27"]], bin: [["+", "11", "011"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "9"]] },
							ego3: { dec: [["+", "13"]] },
						},
						en: "great grandmother",
						la: "proavia",
						pl: "prababka macierzysto-ojczysta",
					},
					{
						sex: "♀️",
						knot: "MFM",
						sosa: {
							ego2: { dec: [["✾", "+", "21"]], bin: [["+", "10", "101"]] },
							ego3: { dec: [["✾", "+", "29"]], bin: [["+", "11", "101"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "10"]] },
							ego3: { dec: [["+", "14"]] },
						},
						en: "great grandmother",
						la: "proavia",
						pl: "prababka ojczysto-macierzysta",
					},
					{
						sex: "♀️",
						knot: "ᛟMMM",
						sosa: {
							ego2: { dec: [["✾", "+", "23"]], bin: [["+", "10", "111"]] },
							ego3: { dec: [["✾", "+", "31"]], bin: [["+", "11", "111"]] },
						},
						sosa_rel: {
							ego2: { dec: [["+", "11"]] },
							ego3: { dec: [["+", "15"]] },
						},
						en: "great grandmother",
						la: "proavia",
						pl: "prababka macierzysto-macierzysta",
					},
				],
			},

			{
				r: "3.3",
				pokolenie: "⤵️<br/>䷀2<sub>+</sub><br/>(−2)",
				otoczenie: "ↂ3",
				symbol: "ᛝPPG",
				rp: "b",
				sp: "4",
				spg: "3",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFFB",
						sosa: {
							ego2: {
								dec: [["✾", "+", "8", /*2*/["1", "4"]]],
								bin: [["+", "10", "00.01"]],
							},
							ego3: {
								dec: [["✾", "+", "12", /*2*/["1", "4"]]],
								bin: [["+", "11", "00.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "16&17"]] },
							ego3: { dec: [["+", "24&25"]] },
						},
						en: "great uncle",
						la: "prōpatruus",
						pl: "prastryj <br/> (dziadek stryjeczny)",
					},
					{
						sex: "♂️",
						knot: "ᛟMFB",
						sosa: {
							ego2: {
								dec: [["✾", "+", "10", /*2*/["1", "4"]]],
								bin: [["+", "10", "10.01"]],
							},
							ego3: {
								dec: [["✾", "+", "14", /*2*/["1", "4"]]],
								bin: [["+", "11", "10.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "20&21"]] },
							ego3: { dec: [["+", "28&29"]] },
						},
						en: "great uncle",
						la: "prōpatruus",
						pl: "prastryj <br/> (dziadek stryjeczny)",
					},

					{
						sex: "♂️",
						knot: "ᛟFMB",
						sosa: {
							ego2: {
								dec: [["✾", "+", "9", /*2*/["1", "4"]]],
								bin: [["+", "10", "01.01"]],
							},
							ego3: {
								dec: [["✾", "+", "13", /*2*/["1", "4"]]],
								bin: [["+", "11", "01.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "18&19"]] },
							ego3: { dec: [["+", "26&27"]] },
						},
						en: "great uncle",
						la: "proavunculus",
						pl: "prawuj <br/> (dziadek wujeczny)",
					},
					{
						sex: "♂️",
						knot: "ᛟMMB",
						sosa: {
							ego2: {
								dec: [["✾", "+", "11", /*2*/["1", "4"]]],
								bin: [["+", "10", "11.01"]],
							},
							ego3: {
								dec: [["✾", "+", "15", /*2*/["1", "4"]]],
								bin: [["+", "11", "11.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "22&23"]] },
							ego3: { dec: [["+", "30&31"]] },
						},
						en: "great uncle",
						la: "proavunculus",
						pl: "prawuj <br/> (dziadek wujeczny)",
					},
					{
						sex: "♀️",
						knot: "ᛟFFZ",
						sosa: {
							ego2: {
								dec: [["✾", "+", "8", /*3*/["3", "4"]]],
								bin: [["+", "10", "00.11"]],
							},
							ego3: {
								dec: [["✾", "+", "12", /*3*/["3", "4"]]],
								bin: [["+", "11", "00.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "16&17"]] },
							ego3: { dec: [["+", "24&25"]] },
						},
						en: "great aunt",
						la: "proamita",
						pl: "praciotka <br/> (babka cioteczna)",
					},
					{
						sex: "♀️",
						knot: "ᛟMFZ",
						sosa: {
							ego2: {
								dec: [["✾", "+", "10", /*3*/["3", "4"]]],
								bin: [["+", "10", "10.11"]],
							},
							ego3: {
								dec: [["✾", "+", "14", /*3*/["3", "4"]]],
								bin: [["+", "11", "10.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "20&21"]] },
							ego3: { dec: [["+", "28&29"]] },
						},
						en: "great aunt",
						la: "proamita",
						pl: "praciotka <br/> (babka cioteczna)",
					},

					{
						sex: "♀️",
						knot: "ᛟFMZ",
						sosa: {
							ego2: {
								dec: [["✾", "+", "9", /*3*/["3", "4"]]],
								bin: [["+", "10", "01.11"]],
							},
							ego3: {
								dec: [["✾", "+", "13", /*3*/["3", "4"]]],
								bin: [["+", "11", "01.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "18&19"]] },
							ego3: { dec: [["+", "26&27"]] },
						},
						en: "great aunt",
						la: "prōmātertera",
						pl: "prasioła <br/> (babka siołeczna)",
					},

					{
						sex: "♀️",
						knot: "ᛟMMZ",
						sosa: {
							ego2: {
								dec: [["✾", "+", "11", /*3*/["3", "4"]]],
								bin: [["+", "10", "11.11"]],
							},
							ego3: {
								dec: [["✾", "+", "15", /*3*/["3", "4"]]],
								bin: [["+", "11", "11.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "22&23"]] },
							ego3: { dec: [["+", "30&31"]] },
						},
						en: "great aunt",
						la: "prōmātertera",
						pl: "prasioła <br/> (babka siołeczna)",
					},
				],
			},
			{
				r: "3.2",
				pokolenie: "⤵️<br/>䷀1<sub>+</sub><br/>(−1)",
				otoczenie: "ↂ3",
				symbol: "ᛝPPC",
				rp: "b",
				sp: "3",
				spg: "3",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFFS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "4", /*4*/["1", "8"]]],
								bin: [["+", "10", "0.001"]],
							},
							ego3: {
								dec: [["✾", "+", "6", /*4*/["1", "8"]]],
								bin: [["+", "11", "0.001"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "8"]] },
							ego3: { dec: [["+", "12"]] },
						},
						en: "half uncle",
						la: " ",
						pl: "półstryj",
					},
					{
						sex: "♂️",
						knot: "ᛟMFS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "5", /*4*/["1", "8"]]],
								bin: [["+", "10", "1.001"]],
							},
							ego3: {
								dec: [["✾", "+", "7", /*4*/["1", "8"]]],
								bin: [["+", "11", "1.001"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "10"]] },
							ego3: { dec: [["+", "14"]] },
						},
						en: "half uncle",
						la: " ",
						pl: "półwuj",
					},
					{
						sex: "♂️",
						knot: "ᛟFMS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "4", /*6*/["5", "8"]]],
								bin: [["+", "10", "0.101"]],
							},
							ego3: {
								dec: [["✾", "+", "6", /*6*/["5", "8"]]],
								bin: [["+", "11", "0.101"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "9"]] },
							ego3: { dec: [["+", "13"]] },
						},
						en: "half uncle",
						la: " ",
						pl: "półstryj",
					},

					{
						sex: "♂️",
						knot: "ᛟMMS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "5", /*6*/["5", "8"]]],
								bin: [["+", "10", "1.101"]],
							},
							ego3: {
								dec: [["✾", "+", "7", /*6*/["5", "8"]]],
								bin: [["+", "11", "1.101"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "11"]] },
							ego3: { dec: [["+", "15"]] },
						},
						en: "half uncle",
						la: " ",
						pl: "półwuj",
					},
					{
						sex: "♀️",
						knot: "ᛟFFD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "4", /*5*/["3", "8"]]],
								bin: [["+", "10", "0.011"]],
							},
							ego3: {
								dec: [["✾", "+", "6", /*5*/["3", "8"]]],
								bin: [["+", "11", "0.011"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "8"]] },
							ego3: { dec: [["+", "12"]] },
						},
						en: "half aunt",
						la: " ",
						pl: "półciotka",
					},
					{
						sex: "♀️",
						knot: "ᛟMFD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "5", /*5*/["3", "8"]]],
								bin: [["+", "10", "1.011"]],
							},
							ego3: {
								dec: [["✾", "+", "7", /*5*/["3", "8"]]],
								bin: [["+", "11", "1.011"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "10"]] },
							ego3: { dec: [["+", "14"]] },
						},
						en: "half aunt",
						la: " ",
						pl: "półsioła",
					},
					{
						sex: "♀️",
						knot: "ᛟFMD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "4", /*7*/["7", "8"]]],
								bin: [["+", "10", "0.111"]],
							},
							ego3: {
								dec: [["✾", "+", "6", /*7*/["7", "8"]]],
								bin: [["+", "11", "0.111"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "9"]] },
							ego3: { dec: [["+", "13"]] },
						},
						en: "half aunt",
						la: " ",
						pl: "półciotka",
					},
					{
						sex: "♀️",
						knot: "ᛟMMD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "5", /*7*/["7", "8"]]],
								bin: [["+", "10", "1.111"]],
							},
							ego3: {
								dec: [["✾", "+", "7", /*7*/["7", "8"]]],
								bin: [["+", "11", "1.111"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "11"]] },
							ego3: { dec: [["+", "15"]] },
						},
						en: "half aunt",
						la: " ",
						pl: "półsioła",
					},
				],
			},
			{
				r: "3.6",
				pokolenie: "🔀<br/>䷀0<sub>=</sub><br/>(=0)",
				otoczenie: "ↂ3",
				symbol: "ᛝPGC",
				rp: "b",
				sp: "4",
				spg: "3",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFBS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*8*/["1", "16"]]],
								bin: [["+", "10", ".0001"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*8*/["1", "16"]]],
								bin: [["+", "11", ".0001"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "4",/*2*/["1", "4"]]] },
							ego3: { dec: [["+", "6",/*2*/["1", "4"]]] },
						},
						en: "first cousin",
						la: "patruēlis",
						pl: "brat stryjeczny",
					},
					{
						sex: "♀️",
						knot: "ᛟFBD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*9*/["3", "16"]]],
								bin: [["+", "10", ".0011"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*9*/["3", "16"]]],
								bin: [["+", "11", ".0011"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "4",/*2*/["1", "4"]]] },
							ego3: { dec: [["+", "6",/*2*/["1", "4"]]] },
						},
						en: "first cousin",
						la: "patruēlis",
						pl: "siostra stryjeczna",
					},
					{
						sex: "♂️",
						knot: "ᛟFZS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*10*/["5", "16"]]],
								bin: [["+", "10", ".0101"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*10*/["5", "16"]]],
								bin: [["+", "11", ".0101"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "4",/*3*/["3", "4"]]] },
							ego3: { dec: [["+", "6",/*3*/["3", "4"]]] },
						},
						en: "first cousin",
						la: "amitīnus",
						pl: "brat cioteczny",
					},
					{
						sex: "♀️",
						knot: "ᛟFZD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*11*/["7", "16"]]],
								bin: [["+", "10", ".0111"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*11*/["7", "16"]]],
								bin: [["+", "11", ".0111"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "4",/*3*/["3", "4"]]] },
							ego3: { dec: [["+", "6",/*3*/["3", "4"]]] },
						},
						en: "first cousin",
						la: "amitīna",
						pl: "siostra cioteczna",
					},

					{
						sex: "♂️",
						knot: "ᛟMBS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*12*/["9", "16"]]],
								bin: [["+", "10", ".1001"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*12*/["9", "16"]]],
								bin: [["+", "11", ".1001"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "5",/*2*/["1", "4"]]] },
							ego3: { dec: [["+", "7",/*2*/["1", "4"]]] },
						},
						en: "first cousin",
						la: "mātruēlis",
						pl: "brat wujeczny",
					},
					{
						sex: "♀️",
						knot: "ᛟMBD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*13*/["11", "16"]]],
								bin: [["+", "10", ".1011"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*13*/["11", "16"]]],
								bin: [["+", "11", ".1011"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "5",/*2*/["1", "4"]]] },
							ego3: { dec: [["+", "7",/*2*/["1", "4"]]] },
						},
						en: "first cousin",
						la: "mātruēlis",
						pl: "siostra wujeczna",
					},
					{
						sex: "♂️",
						knot: "ᛟMZS",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*14*/["13", "16"]]],
								bin: [["+", "10", ".1101"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*14*/["13", "16"]]],
								bin: [["+", "11", ".1101"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "5",/*3*/["3", "4"]]] },
							ego3: { dec: [["+", "7",/*3*/["3", "4"]]] },
						},
						en: "first cousin",
						la: "cōnsobrīnus",
						pl: "brat siołeczny",
					},
					{
						sex: "♀️",
						knot: "ᛟMZD",
						sosa: {
							ego2: {
								dec: [["✾", "+", "2", /*15*/["15", "16"]]],
								bin: [["+", "10", ".1111"]],
							},
							ego3: {
								dec: [["✾", "+", "3", /*15*/["15", "16"]]],
								bin: [["+", "11", ".1111"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "5",/*3*/["3", "4"]]] },
							ego3: { dec: [["+", "7",/*3*/["3", "4"]]] },
						},
						en: "first cousin",
						la: "cōnsobrīna",
						pl: "siostra siołeczna",
					},
				],
			},



			{
				r: "3.2",
				pokolenie: "⤴️<br/>䷀1<sub>−</sub><br/>(+1)",
				otoczenie: "ↂ3",
				symbol: "ᛝPCC",
				rp: "b",
				sp: "3",
				spg: "3",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟFSS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "4", /*4*/["1", "8"]]],
								bin: [["−", "10", "0.001"]],
							},
							ego3: {
								dec: [["✾", "−", "6", /*4*/["1", "8"]]],
								bin: [["−", "11", "0.001"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*4*/["1", "8"]]] },
							ego3: { dec: [["+", "3", /*4*/["1", "8"]]] },
						},
						en: "half nephew",
						la: " ",
						pl: { ego2: "półsynowiec <br/> (brataniec przyrodni)", ego3: "półbrataniec <br/> (brataniec przyrodni)" },
					},
					{
						sex: "♀️",
						knot: "ᛟFSD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "5", /*4*/["1", "8"]]],
								bin: [["−", "10", "1.001"]],
							},
							ego3: {
								dec: [["✾", "−", "7", /*4*/["1", "8"]]],
								bin: [["−", "11", "1.001"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*4*/["1", "8"]]] },
							ego3: { dec: [["+", "3", /*4*/["1", "8"]]] },
						},
						en: "half niece",
						la: " ",
						pl: { ego2: "półsynowica <br/> (bratanica przyrodnia)", ego3: "półbratanica <br/> (bratanica przyrodnia)" },
					},
					{
						sex: "♂️",
						knot: "ᛟFDS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "4", /*5*/["3", "8"]]],
								bin: [["−", "10", "0.011"]],
							},
							ego3: {
								dec: [["✾", "−", "6", /*5*/["3", "8"]]],
								bin: [["−", "11", "0.011"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*5*/["3", "8"]]] },
							ego3: { dec: [["+", "3", /*5*/["3", "8"]]] },
						},
						en: "half nephew",
						la: " ",
						pl: "półsiostrzeniec",
					},
					{
						sex: "♀️",
						knot: "ᛟFDD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "5", /*5*/["3", "8"]]],
								bin: [["−", "10", "1.011"]],
							},
							ego3: {
								dec: [["✾", "−", "7", /*5*/["3", "8"]]],
								bin: [["−", "11", "1.011"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*5*/["3", "8"]]] },
							ego3: { dec: [["+", "3", /*5*/["3", "8"]]] },
						},
						en: "half niece",
						la: " ",
						pl: "półsiostrzenica",
					},

					{
						sex: "♂️",
						knot: "ᛟMSS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "4", /*6*/["5", "8"]]],
								bin: [["−", "10", "0.101"]],
							},
							ego3: {
								dec: [["✾", "−", "6", /*6*/["5", "8"]]],
								bin: [["−", "11", "0.101"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*6*/["5", "8"]]] },
							ego3: { dec: [["+", "3", /*6*/["5", "8"]]] },
						},
						en: "half nephew",
						la: " ",
						pl: { ego2: "półsynowiec <br/> (brataniec przyrodni)", ego3: "półbrataniec <br/> (brataniec przyrodni)" },
					},
					{
						sex: "♀️",
						knot: "ᛟMSD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "5", /*6*/["5", "8"]]],
								bin: [["−", "10", "1.101"]],
							},
							ego3: {
								dec: [["✾", "−", "7", /*6*/["5", "8"]]],
								bin: [["−", "11", "1.101"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*6*/["5", "8"]]] },
							ego3: { dec: [["+", "3", /*6*/["5", "8"]]] },
						},
						en: "half niece",
						la: " ",
						pl: { ego2: "półsynowica <br/> (bratanica przyrodnia)", ego3: "półbratanica <br/> (bratanica przyrodnia)" },
					},
					{
						sex: "♂️",
						knot: "ᛟMDS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "4", /*7*/["7", "8"]]],
								bin: [["−", "10", "0.111"]],
							},
							ego3: {
								dec: [["✾", "−", "6", /*7*/["7", "8"]]],
								bin: [["−", "11", "0.111"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*7*/["7", "8"]]] },
							ego3: { dec: [["+", "3", /*7*/["7", "8"]]] },
						},
						en: "half nephew",
						la: " ",
						pl: "półsiostrzeniec",
					},
					{
						sex: "♀️",
						knot: "ᛟMDD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "5", /*7*/["7", "8"]]],
								bin: [["−", "10", "1.111"]],
							},
							ego3: {
								dec: [["✾", "−", "7", /*7*/["7", "8"]]],
								bin: [["−", "11", "1.111"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["+", "2", /*7*/["7", "8"]]] },
							ego3: { dec: [["+", "3", /*7*/["7", "8"]]] },
						},
						en: "half niece",
						la: " ",
						pl: "półsiostrzenica",
					},
				],
			},


			{
				r: "3.3",
				pokolenie: "⤴️<br/>䷀2<sub>−</sub><br/>(+2)",
				otoczenie: "ↂ3",
				symbol: "ᛝGCC",
				rp: "b",
				sp: "4",
				spg: "3",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟBSS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "8", /*2*/["1", "4"]]],
								bin: [["−", "10", "00.01"]],
							},
							ego3: {
								dec: [["✾", "−", "12", /*2*/["1", "4"]]],
								bin: [["−", "11", "00.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "4", /*2*/["1", "4"]]] },
							ego3: { dec: [["−", "6", /*2*/["1", "4"]]] },
						},
						en: "great nephew",
						la: " ",
						pl: {
							ego2: "wnuk stryjeczny <br/> (wnuk brata, syn bratanka)",
							ego3: "wnuk siołeczny <br/> (wnuk brata, syn bratanka)"
						},
					},
					{
						sex: "♀️",
						knot: "ᛟBSD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "9", /*2*/["1", "4"]]],
								bin: [["−", "10", "01.01"]],
							},
							ego3: {
								dec: [["✾", "−", "13", /*2*/["1", "4"]]],
								bin: [["−", "11", "01.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "4", /*2*/["1", "4"]]] },
							ego3: { dec: [["−", "6", /*2*/["1", "4"]]] },
						},
						en: "great niece",
						la: " ",
						pl: {
							ego2: "wnuczka stryjeczna <br/> (wnuczka brata, córka bratanka)",
							ego3: "wnuczka cioteczna <br/> (wnuczka brata, córka bratanka)"
						},
					},
					{
						sex: "♂️",
						knot: "ᛟZSS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "8", /*3*/["3", "4"]]],
								bin: [["−", "10", "00.11"]],
							},
							ego3: {
								dec: [["✾", "−", "12", /*3*/["3", "4"]]],
								bin: [["−", "11", "00.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "4", /*3*/["3", "4"]]] },
							ego3: { dec: [["−", "6", /*3*/["3", "4"]]] },
						},
						en: "great nephew",
						la: " ",
						pl: {
							ego2: "wnuk wujeczny <br/> (wnuk siostry, syn siostrzeńca)",
							ego3: "wnuk siołeczny <br/> (wnuk siostry, syn siostrzeńca)"
						},
					},
					{
						sex: "♀️",
						knot: "ᛟZSD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "9", /*3*/["3", "4"]]],
								bin: [["−", "10", "01.11"]],
							},
							ego3: {
								dec: [["✾", "−", "13", /*3*/["3", "4"]]],
								bin: [["−", "11", "01.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "4", /*3*/["3", "4"]]] },
							ego3: { dec: [["−", "6", /*3*/["3", "4"]]] },
						},
						en: "great niece",
						la: " ",
						pl: {
							ego2: "wnuczka wujeczna <br/> (wnuczka siostry, córka siostrzeńc)",
							ego3: "wnuczka siołeczna <br/> (wnuczka siostry, córka siostrzeńca)"
						},
					},

					{
						sex: "♂️",
						knot: "ᛟBDS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "10", /*2*/["1", "4"]]],
								bin: [["−", "10", "10.01"]],
							},
							ego3: {
								dec: [["✾", "−", "14", /*2*/["1", "4"]]],
								bin: [["−", "11", "10.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "5", /*2*/["1", "4"]]] },
							ego3: { dec: [["−", "7", /*2*/["1", "4"]]] },
						},
						en: "great nephew",
						la: " ",
						pl: {
							ego2: "wnuk stryjeczny <br/> (wnuk brata, syn bratanicy)",
							ego3: "wnuk cioteczny <br/> (wnuk brata, syn bratanicy)"
						},
					},
					{
						sex: "♀️",
						knot: "ᛟBDD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "11", /*2*/["1", "4"]]],
								bin: [["−", "10", "11.01"]],
							},
							ego3: {
								dec: [["✾", "−", "15", /*2*/["1", "4"]]],
								bin: [["−", "11", "11.01"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "5", /*2*/["1", "4"]]] },
							ego3: { dec: [["−", "7", /*2*/["1", "4"]]] },
						},
						en: "great niece",
						la: " ",
						pl: {
							ego2: "wnuczka stryjeczna <br/> (wnuczka brata, córka bratanicy)",
							ego3: "wnuczka cioteczna <br/> (wnuczka brata, córka bratanicy)"
						},
					},
					{
						sex: "♂️",
						knot: "ᛟZDS",
						sosa: {
							ego2: {
								dec: [["✾", "−", "10", /*3*/["3", "4"]]],
								bin: [["−", "10", "10.11"]],
							},
							ego3: {
								dec: [["✾", "−", "14", /*3*/["3", "4"]]],
								bin: [["−", "11", "10.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "5", /*3*/["3", "4"]]] },
							ego3: { dec: [["−", "7", /*3*/["3", "4"]]] },
						},
						en: "great nephew",
						la: " ",
						pl: {
							ego2: "wnuk wujeczny <br/> (wnuk siostry, syn siostrzenicy)",
							ego3: "wnuk siołeczny <br/> (wnuk siostry, syn siostrzenicy)"
						},
					},
					{
						sex: "♀️",
						knot: "ᛟZDD",
						sosa: {
							ego2: {
								dec: [["✾", "−", "11", /*3*/["3", "4"]]],
								bin: [["−", "10", "11.11"]],
							},
							ego3: {
								dec: [["✾", "−", "15", /*3*/["3", "4"]]],
								bin: [["−", "11", "11.11"]],
							},
						},
						sosa_rel: {
							ego2: { dec: [["−", "5", /*3*/["3", "4"]]] },
							ego3: { dec: [["−", "7", /*3*/["3", "4"]]] },
						},
						en: "great niece",
						la: " ",
						pl: {
							ego2: "wnuczka wujeczna <br/> (wnuczka siostry, córka siostrzenicy)",
							ego3: "wnuczka siołeczna <br/> (wnuczka siostry, córka siostrzenicy)"
						},
					},
				],
			},




			{
				r: "3.1",
				pokolenie: "⤴️<br/>䷀3<sub>−</sub><br/>(+3)",
				otoczenie: "ↂ3",
				symbol: "ᛝCCC",
				rp: "z",
				sp: "3",
				spg: "3",
				relacje: [
					{
						sex: "♂️",
						knot: "ᛟSSS",
						sosa: {
							ego2: { dec: [["✾", "−", "16"]], bin: [["−", "10", "000"]] },
							ego3: { dec: [["✾", "−", "24"]], bin: [["−", "11", "000"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "8"]] },
							ego3: { dec: [["−", "12"]] },
						},
						en: "great grandson",
						la: "prōnepōs",
						pl: "prawnuk",
					},
					{
						sex: "♀️",
						knot: "ᛟSSD",
						sosa: {
							ego2: { dec: [["✾", "−", "17"]], bin: [["−", "10", "001"]] },
							ego3: { dec: [["✾", "−", "25"]], bin: [["−", "11", "001"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "8"]] },
							ego3: { dec: [["−", "12"]] },
						},
						en: "great granddaughter",
						la: "prōneptis",
						pl: "prawnuczka",
					},
					{
						sex: "♂️",
						knot: "ᛟSDS",
						sosa: {
							ego2: { dec: [["✾", "−", "18"]], bin: [["−", "10", "010"]] },
							ego3: { dec: [["✾", "−", "26"]], bin: [["−", "11", "010"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "9"]] },
							ego3: { dec: [["−", "13"]] },
						},
						en: "great grandson",
						la: "prōnepōs",
						pl: "prawnuk",
					},
					{
						sex: "♀️",
						knot: "ᛟSDD",
						sosa: {
							ego2: { dec: [["✾", "−", "19"]], bin: [["−", "10", "011"]] },
							ego3: { dec: [["✾", "−", "27"]], bin: [["−", "11", "011"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "9"]] },
							ego3: { dec: [["−", "13"]] },
						},
						en: "great granddaughter",
						la: "prōneptis",
						pl: "prawnuczka",
					},
					{
						sex: "♂️",
						knot: "ᛟDSS",
						sosa: {
							ego2: { dec: [["✾", "−", "20"]], bin: [["−", "10", "100"]] },
							ego3: { dec: [["✾", "−", "28"]], bin: [["−", "11", "100"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "10"]] },
							ego3: { dec: [["−", "14"]] },
						},
						en: "great grandson",
						la: "prōnepōs",
						pl: "prawnuk",
					},
					{
						sex: "♀️",
						knot: "ᛟDSD",
						sosa: {
							ego2: { dec: [["✾", "−", "21"]], bin: [["−", "10", "101"]] },
							ego3: { dec: [["✾", "−", "29"]], bin: [["−", "11", "101"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "10"]] },
							ego3: { dec: [["−", "14"]] },
						},
						en: "great granddaughter",
						la: "prōneptis",
						pl: "prawnuczka",
					},
					{
						sex: "♂️",
						knot: "ᛟDDS",
						sosa: {
							ego2: { dec: [["✾", "−", "22"]], bin: [["−", "10", "110"]] },
							ego3: { dec: [["✾", "−", "30"]], bin: [["−", "11", "110"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "11"]] },
							ego3: { dec: [["−", "15"]] },
						},
						en: "great grandson",
						la: "prōnepōs",
						pl: "prawnuk",
					},
					{
						sex: "♀️",
						knot: "ᛟDDD",
						sosa: {
							ego2: { dec: [["✾", "−", "23"]], bin: [["−", "10", "111"]] },
							ego3: { dec: [["✾", "−", "31"]], bin: [["−", "11", "111"]] },
						},
						sosa_rel: {
							ego2: { dec: [["−", "11"]] },
							ego3: { dec: [["−", "15"]] },
						},
						en: "great granddaughter",
						la: "prōneptis",
						pl: "prawnuczka",
					},
				],
			},
		],
	},
];
