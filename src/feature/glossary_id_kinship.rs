// ./src/feature/glossary_id_kinship.rs

#[rustfmt::skip]
pub enum KinshipJago {
	F,	M,	P, // 1.1 (1)
	S,	D,	C, // 1.1 (1)
	B,	Z,	G, // 1.2 (2/1)
	H,	W,	E, // 1.3 (K)

	FP,	MP,	PP, // 2.1 (2)
	SC,	DC,	CC, // 2.1 (2)
	FF,	FM,	MF,	MM, // 2.1 (2)
	SS,	SD,	DS,	DD, // 2.1 (2)

	FC,	MC,	PC, // 2.2 (2)
	FS,	FD,	MS,	MD,  // 2.2 (2)

	FG,	MG,	PG, // 2.3 (3/2)
	BC,	ZC,	GC, // 2.3 (3/2)
	FB,	FZ,	MB,	MZ, // 2.3 (3/2)
	BS,	BD,	ZS,	ZD, // 2.3 (3/2)

	MH,FW,PE,EC, // 2.4 (P1)
	WS,WD,WC,// 2.4 (P1)
	HS,HD,HC,// 2.4 (P1)

	DH,SW,CE,EP, // 2.5 (P1)
	WF,WM,WP,// 2.5 (P1)
	HF,HM,HP,// 2.5 (P1)

	ZH,BW,GE,EG, // 2.6 (P2)
	WB,WZ,WG,// 2.6 (P2)
	HB,HZ,HG,// 2.6 (P2)

	PPP, FFP, MFP, FMP, MMP, // 3.1 (3)
	CCC, SSC, SDC, DSC, DDC, // 3.1 (3)
	FFF, FFM, MFF, MFM, FMF, FMM, MMF, MMM, // 3.1 (3)
	SSS, SSD, SDS, SDD, DSS, DSD, DDS, DDD, // 3.1 (3)

}

macro_rules! pary {
	// koniec: wszystkie pary zebrane, generujemy kod
	(@zbierz [$( ($a:ident $na:literal, $b:ident $nb:literal) )*]) => {
		impl KinshipJago {
			/// Wszystkie pary, w kolejności zapisu.
			pub const PARY: &[((KinshipJago, &str), (KinshipJago, &str))] =
				&[$( ((KinshipJago::$a, $na), (KinshipJago::$b, $nb)) ),*];

			/// Nazwy obu stron relacji; kolejność kluczy dowolna.
			#[allow(unreachable_patterns)]
			pub fn nazwy(a: Self, b: Self) -> Option<(&'static str, &'static str)> {
				match (a, b) {
					$(
						(Self::$a, Self::$b) => Some(($na, $nb)),
						(Self::$b, Self::$a) => Some(($nb, $na)),
					)*
					_ => None,
				}
			}
		}
	};
	// grupa <=> grupa: każdy z lewej z każdym z prawej
	(@zbierz [$($acc:tt)*]
		($($a:ident $na:literal),+ $(,)?) <=> ($($b:ident $nb:literal),+ $(,)?); $($rest:tt)*) => {
		pary!(@krzyz [$($acc)*] [$($a $na),+] [$($b $nb),+] $($rest)*);
	};
	// sym grupa: każdy z każdym w tej samej grupie, także sam ze sobą, każda para raz
	(@zbierz [$($acc:tt)*] sym ($($a:ident $na:literal),+ $(,)?); $($rest:tt)*) => {
		pary!(@sym [$($acc)*] [$($a $na),+] $($rest)*);
	};
	// pojedyncza para
	(@zbierz [$($acc:tt)*] $a:ident $na:literal <=> $b:ident $nb:literal; $($rest:tt)*) => {
		pary!(@zbierz [$($acc)* ($a $na, $b $nb)] $($rest)*);
	};
	// żaden wzór nie pasuje: czytelny błąd zamiast nieskończonej rekurencji
	(@zbierz [$($acc:tt)*] $($zly:tt)*) => {
		compile_error!(concat!("pary!: nieznana postać wiersza: ", stringify!($($zly)*)));
	};

	// pomocnicze: iloczyn kartezjański lewa × prawa
	(@krzyz [$($acc:tt)*] [$a:ident $na:literal $(, $at:ident $nat:literal)*] [$($b:ident $nb:literal),+] $($rest:tt)*) => {
		pary!(@krzyz [$($acc)* $( ($a $na, $b $nb) )+] [$($at $nat),*] [$($b $nb),+] $($rest)*);
	};
	(@krzyz [$($acc:tt)*] [] [$($b:ident $nb:literal),+] $($rest:tt)*) => {
		pary!(@zbierz [$($acc)*] $($rest)*);
	};
	// pomocnicze: pary nieuporządkowane w grupie
	(@sym [$($acc:tt)*] [$a:ident $na:literal $(, $t:ident $nt:literal)*] $($rest:tt)*) => {
		pary!(@sym [$($acc)* ($a $na, $a $na) $( ($a $na, $t $nt) )*] [$($t $nt),*] $($rest)*);
	};
	(@sym [$($acc:tt)*] [] $($rest:tt)*) => {
		pary!(@zbierz [$($acc)*] $($rest)*);
	};

	// wejście
	($($t:tt)*) => { pary!(@zbierz [] $($t)*); };
}

pary! {
	// 1.1 (1)
	(P "rodzic/rodzice", F "ojciec", M "matka") <=> (C "dziecko/dzieci", S "syn", D "córka");
	// 1.2 (2/1)
	sym (G "brat/siostra/rodzeństwo", B "brat", Z "siostra");
	// 1.3 (K)
	sym (E "małżonek/małżonkowie", H "mąż", W "żona");

	// 2.1 (2)
	(
		PP "dziadkowie",
		FP "dziadkowie ojczyści",
		FF "dziadek ojczysty",
		FM "babcia ojczysta"
	) <=> ( CC "wnukowie",
			SC "wnukowie po synu",
			SS "wnuk po synu",
			SD "wnuczka po synu"
		);
	(
		PP "dziadkowie",
		MP "dziadkowie macierzyści",
		MF "dziadek macierzysty",
		MM "babcia macierzysta"
	) <=> ( CC "wnukowie",
			DC "wnukowie po córce",
			DS "wnuk po córce",
			DD "wnuczka po córce"
		);

	// 2.2 (2)
	sym (
		PC "rodzeństwo przyrodnie",
		FC "rodzeństwo przyrodnie ojczyste",
		FS "półbrat ojczysty",
		FD "półsiostra ojczysta"
	);
	sym (
		PC "rodzeństwo przyrodnie",
		MC "rodzeństwo przyrodnie macierzyste",
		MS "półbrat macierzysty",
		MD "półsiostra macierzysta"
	);
	// 2.3 (3/2)
	(PG "stryjkowie,wujkowie", FG "stryjkowie", MG "wujkowie") <=> (GC "nieciowie");
	FG "stryjkowie" <=> BC "nieciowie";
	(FB "stryj") <=> ( BC "synowiec,synowica", BS "synowiec", BD "synowica" );
	(FZ "ciota") <=> ( BC "brataniec,bratanica", BS "brataniec", BD "bratanica" );
	MG "wujkowie" <=> ZC "nieciowie";
	(MB "wuj") <=> ( ZC "siostrzeniec,siostrzenica", ZS "siostrzeniec", ZD "siostrzenica" );
	(MZ "sioła") <=> ( ZC "córowiec,córowica", ZS "córowiec", ZD "córowica" );

		// 2.4 (P1)
	(
		PE "rodzice przybrani",
		MH "ojczym"
	) <=> ( EC "dzieci przybrane",
			WC "pasierb,pasierbica po żonie",
			WS "pasierb po żonie",
			WD "pasierbica po żonie"
		);
	(
		PE "rodzice przybrani",
		FW "macocha"
	) <=> ( EC "dzieci przybrane",
			HC "pasierb,pasierbica po mężu",
			HS "pasierb po mężu",
			HD "pasierbica po mężu"
		);

	// 2.5 (P1)
	(
		CE "małżonkowie dzieci",
		DH "zięć"
	) <=> ( EP "teściowie",
			WP "teść,teściowa",
			WF "teść",
			WM "teściowa"
		);
	(
		CE "małżonkowie dzieci",
		SW "snecha"
	) <=> ( EP "teściowie",
			HP "świekier,świekra",
			HF "świekier",
			HM "świekra"
		);

	// 2.6 (P2)
	(
		GE "małżonkowie rodzeństwa",
		ZH "swak"
	) <=> ( EG "rodzeństwo małżonka",
			WG "szurzy,świeść",
			WB "szurzy",
			WZ "świeść"
		);
	(
		GE "małżonkowie rodzeństwa",
		BW "bratowa"
	) <=> ( EG "rodzeństwo małżonka",
			HG "dziewierz,zełwa",
			HB "dziewierz",
			HZ "zełwa"
		);

		// 3.1 (3)
	(
		PPP "pradziadkowie",
		FFP "rodzice dziadka ojczystego",
		FFF "ojciec dziadka ojczystego",
		FFM "matka dziadka ojczystego"
	) <=> ( CCC "prawnukowie",
			SSC "prawnukowie po synu syna",
			SSS "prawnuk po synu syna",
			SSD "prawnuczka po synu syna"
		);
	(
		PPP "pradziadkowie",
		MFP "rodzice dziadka macierzystego",
		MFF "ojciec dziadka macierzystego",
		MFM "matka dziadka macierzystego"
	) <=> ( CCC "prawnukowie",
			SDC "prawnukowie po córce syna",
			SDS "prawnuk po córce syna",
			SDD "prawnuczka po córce syna"
		);
	(
		PPP "pradziadkowie",
		FMP "rodzice babki ojczystej",
		FMF "ojciec babki ojczystej",
		FMM "matka babki ojczystej"
	) <=> ( CCC "prawnukowie",
			DSC "prawnukowie po synu córki",
			DSS "prawnuk po synu córki",
			DSD "prawnuczka po synu córki"
		);
	(
		PPP "pradziadkowie",
		MMP "rodzice babki macierzystej",
		MMF "ojciec babki macierzystej",
		MMM "matka babki macierzystej"
	) <=> ( CCC "prawnukowie",
			DDC "prawnukowie po córce córki",
			DDS "prawnuk po córce córki",
			DDD "prawnuczka po córce córki"
		);

}
