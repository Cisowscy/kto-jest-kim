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
}

impl KinshipJago {
	pub fn get_polski(&self) -> &'static str {
		match self {
			Self::P => "rodzic/rodzice",
			Self::F => "ojciec",
			Self::M => "matka",
			Self::C => "dziecko/dzieci",
			Self::S => "syn",
			Self::D => "córka",
			Self::G => "brat/siostra/rodzeństwo",
			Self::B => "brat",
			Self::Z => "siostra",
			Self::E => "małżonek/małżonkowie",
			Self::H => "mąż",
			Self::W => "żona",
			//---------------------------------
			Self::PP => "dziadkowie",
			Self::FP => "dziadkowie ojczyści",
			Self::MP => "dziadkowie macierzyści",
			Self::CC => "wnukowie",
			Self::SC => "wnukowie po synu",
			Self::DC => "wnukowie po córce",
			Self::FF => "dziadek ojczysty",
			Self::FM => "babcia ojczysta",
			Self::MF => "dziadek macierzysty",
			Self::MM => "babcia macierzysta",
			Self::SS => "wnuk po synu",
			Self::SD => "wnuczka po synu",
			Self::DS => "wnuk po córce",
			Self::DD => "wnuczka po córce",
			//---------------------------------
			Self::PC => "rodzeństwo przyrodnie",
			Self::FC => "rodzeństwo przyrodnie ojczyste",
			Self::MC => "rodzeństwo przyrodnie macierzyste",
			Self::FS => "półbrat ojczysty",
			Self::FD => "półsuiostra ojczysta",
			Self::MS => "półbrat macierzysty",
			Self::MD => "półsiostra macierzysta",
			//---------------------------------
		}
	}
}
