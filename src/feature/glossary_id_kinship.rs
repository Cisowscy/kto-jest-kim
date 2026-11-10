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

	//FG,	MG,	PG, // 2.3 (3/2)
	//BC,	ZC,	GC, // 2.3 (3/2)
	//FB,	FZ,	MB,	MZ, // 2.3 (3/2)
	//BS,	BD,	ZS,	ZD, // 2.3 (3/2)
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
	// trójka <=> trójka: nijaka ↔ nijaka oraz każda płeć z lewej z każdą z prawej
	(@zbierz [$($acc:tt)*]
		($u1:ident $nu1:literal, $m1:ident $nm1:literal, $f1:ident $nf1:literal) <=>
		($u2:ident $nu2:literal, $m2:ident $nm2:literal, $f2:ident $nf2:literal); $($rest:tt)*) => {
		pary!(@zbierz [$($acc)*
			($u1 $nu1, $u2 $nu2)
			($m1 $nm1, $m2 $nm2) ($m1 $nm1, $f2 $nf2)
			($f1 $nf1, $m2 $nm2) ($f1 $nf1, $f2 $nf2)
		] $($rest)*);
	};
	// trójka symetryczna: ta sama trójka po obu stronach, bez powtórzeń
	(@zbierz [$($acc:tt)*]
		sym ($u:ident $nu:literal, $m:ident $nm:literal, $f:ident $nf:literal); $($rest:tt)*) => {
		pary!(@zbierz [$($acc)*
			($u $nu, $u $nu)
			($m $nm, $m $nm) ($m $nm, $f $nf) ($f $nf, $f $nf)
		] $($rest)*);
	};
	// pojedyncza para, jak dotąd
	(@zbierz [$($acc:tt)*] $a:ident $na:literal <=> $b:ident $nb:literal; $($rest:tt)*) => {
		pary!(@zbierz [$($acc)* ($a $na, $b $nb)] $($rest)*);
	};
	// wejście
	($($t:tt)*) => { pary!(@zbierz [] $($t)*); };
}

pary! {
	(P "rodzic/rodzice", F "ojciec", M "matka") <=> (C "dziecko/dzieci", S "syn", D "córka");
	sym (G "brat/siostra/rodzeństwo", B "brat", Z "siostra");
	sym (E "małżonek/małżonkowie", H "mąż", W "żona");

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
			DS "wnuk po córce"
		);

	//E "małżonek/małżonkowie" <=> E "małżonek/małżonkowie";
	//H "mąż" <=> W "żona";
}

//pary! {
//	PC "rodzeństwo przyrodnie" 	<=> PC "rodzeństwo przyrodnie";
//	FC "rodzeństwo przyrodnie ojczyste" 	<=> FC "rodzeństwo przyrodnie ojczyste";
//	MC "rodzeństwo przyrodnie macierzyste" 	<=> MC "rodzeństwo przyrodnie macierzyste";
//	FS "półbrat ojczysty" 		<=> FS "półbrat ojczysty";
//	FS "półbrat ojczysty" 		<=> FD "półsuiostra ojczysta";
//	FD "półsuiostra ojczysta" 	<=> FD "półsuiostra ojczysta";
//	MS "półbrat macierzysty" 	<=> MS "półbrat macierzysty";
//	MS "półbrat macierzysty" 	<=> MD "półsiostra macierzysta";
//	MD "półsiostra macierzysta" <=> MD "półsiostra macierzysta";
//}
