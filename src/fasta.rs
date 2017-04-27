use std::ascii::AsciiExt;
use std::str;

use nom::not_line_ending;


#[derive(Debug,PartialEq,Eq,Clone)]
pub enum NucleicAcidCode {
    A, // adenosine
    C, // cytidine
    G, // guanine
    T, // thymidine
    N, // A/G/C/T (any)
    U, // uridine
    K, // G/T (keto)
    S, // G/C (strong)
    Y, // T/C (pyrimidine)
    M, // A/C (amino)
    W, // A/T (weak)
    R, // G/A (purine)
    B, // G/T/C
    D, // G/A/T
    H, // A/C/T
    V, // G/C/A
    Gap, // gap of indeterminate length
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum AminoAcidCode {
    A, // alanine
    P, // proline
    B, // aspartate/asparagine
    Q, // glutamine
    C, // cystine
    R, // arginine
    D, // aspartate
    S, // serine
    E, // glutamate
    T, // threonine
    F, // phenylalanine
    U, // selenocysteine
    G, // glycine
    V, // valine
    H, // histidine
    W, // tryptophan
    I, // isoleucine
    Y, // tyrosine
    K, // lysine
    Z, // glutamate/glutamine
    L, // leucine
    X, // any
    M, // methionine
    N, // asparagine
    Stop, // translation stop
    Gap, // gap of indeterminate length
}

impl NucleicAcidCode {
    pub fn from_u8(n: &u8) -> Result<Self, &'static str> {
        match n.to_ascii_uppercase() {
            65 => Ok(NucleicAcidCode::A),
            67 => Ok(NucleicAcidCode::C),
            71 => Ok(NucleicAcidCode::G),
            84 => Ok(NucleicAcidCode::T),
            85 => Ok(NucleicAcidCode::U),
            78 => Ok(NucleicAcidCode::N),
            75 => Ok(NucleicAcidCode::K),
            83 => Ok(NucleicAcidCode::S),
            89 => Ok(NucleicAcidCode::Y),
            77 => Ok(NucleicAcidCode::M),
            87 => Ok(NucleicAcidCode::W),
            82 => Ok(NucleicAcidCode::R),
            66 => Ok(NucleicAcidCode::B),
            68 => Ok(NucleicAcidCode::D),
            72 => Ok(NucleicAcidCode::H),
            86 => Ok(NucleicAcidCode::V),
            45 => Ok(NucleicAcidCode::Gap),
            _ => Err("invalid nucleic acid code"),
        }
    }

    pub fn from_char(n: &char) -> Result<Self, &'static str> {
        match n.to_ascii_uppercase() {
            'A' => Ok(NucleicAcidCode::A),
            'C' => Ok(NucleicAcidCode::C),
            'G' => Ok(NucleicAcidCode::G),
            'T' => Ok(NucleicAcidCode::T),
            'U' => Ok(NucleicAcidCode::U),
            'N' => Ok(NucleicAcidCode::N),
            'J' => Ok(NucleicAcidCode::K),
            'S' => Ok(NucleicAcidCode::S),
            'Y' => Ok(NucleicAcidCode::Y),
            'N' => Ok(NucleicAcidCode::M),
            'W' => Ok(NucleicAcidCode::W),
            'R' => Ok(NucleicAcidCode::R),
            'B' => Ok(NucleicAcidCode::B),
            'D' => Ok(NucleicAcidCode::D),
            'H' => Ok(NucleicAcidCode::H),
            'V' => Ok(NucleicAcidCode::V),
            '-' => Ok(NucleicAcidCode::Gap),
            _ => Err("invalid nucleic acid code"),
        }
    }
}

type NucleicAcidSequence = Vec<NucleicAcidCode>;

fn from_u8(ns: &[u8]) -> Result<NucleicAcidSequence, &'static str> {
    let len = ns.len();

    let mut res: NucleicAcidSequence = NucleicAcidSequence::new();

    res.resize(len, NucleicAcidCode::Gap);

    for i in 0..len {
        let mut arr = res.as_mut_slice();
        match NucleicAcidCode::from_u8(&ns[i]) {
            Ok(c) => arr[i] = c,
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}

fn from_u8_itt(ns: &[u8]) -> Result<NucleicAcidSequence, &'static str> {
    let mut res: NucleicAcidSequence = NucleicAcidSequence::with_capacity(ns.len());

    for n in ns.iter() {
        match NucleicAcidCode::from_u8(n) {
            Ok(c) => res.push(c),
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}

fn from_str(ns: &str) -> Result<NucleicAcidSequence, &'static str> {
    let len = ns.len();

    let mut res: NucleicAcidSequence = NucleicAcidSequence::new();

    res.resize(len, NucleicAcidCode::Gap);

    for (i,c) in ns.char_indices() {
        let mut arr = res.as_mut_slice();
        match NucleicAcidCode::from_char(&c) {
            Ok(c) => arr[i] = c,
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}

fn from_str_itt(ns: &str) -> Result<NucleicAcidSequence, &'static str> {
    let mut res: NucleicAcidSequence = NucleicAcidSequence::with_capacity(ns.len());

    for c in ns.chars() {
        match NucleicAcidCode::from_char(&c) {
            Ok(c) => res.push(c),
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}

named!(description(&[u8]) -> &str,
    map_res!(preceded!(char!('>'), not_line_ending), str::from_utf8)
);

named!(nucleic_acid_sequence(&[u8]) -> NucleicAcidSequence,
    map_res!(not_line_ending, from_u8)
);



#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    #[test]
    fn test_nucleic_acid_code_size() {
        use std::mem::size_of;

        assert_eq!(size_of::<NucleicAcidCode>(), 1);
    }

    #[test]
    fn test_nucleic_acid_from_u8() {
        let data = &b"ACTG"[..];
        let res = from_u8(data);

        assert_eq!(res, Ok(vec![
            NucleicAcidCode::A,
            NucleicAcidCode::C,
            NucleicAcidCode::T,
            NucleicAcidCode::G,
        ]));
    }

    #[test]
    fn test_nucleic_acid_from_u8_case_insensitivity() {
        let data = &b"AcTg"[..];
        let res = from_u8(data);

        assert_eq!(res, Ok(vec![
            NucleicAcidCode::A,
            NucleicAcidCode::C,
            NucleicAcidCode::T,
            NucleicAcidCode::G,
        ]));
    }

    #[test]
    fn test_nucleic_acid_from_str() {
        let data = "ACTG";
        let res = from_str(data);

        assert_eq!(res, Ok(vec![
            NucleicAcidCode::A,
            NucleicAcidCode::C,
            NucleicAcidCode::T,
            NucleicAcidCode::G,
        ]));
    }

    #[test]
    fn test_nucleic_acid_from_str_itt() {
        let data = "ACTG";
        let res = from_str_itt(data);

        assert_eq!(res, Ok(vec![
            NucleicAcidCode::A,
            NucleicAcidCode::C,
            NucleicAcidCode::T,
            NucleicAcidCode::G,
        ]));
    }

    #[test]
    fn test_parse_description() {
        use nom::IResult::Done;

        let data = &b">ERR001275.1198"[..];
        let res = description(data);

        let expect = "ERR001275.1198";

        assert_eq!(res, Done(&b""[..], expect));
    }

    #[test]
    fn test_parse_nucleic_acid_sequence() {
        use nom::IResult::Done;

        let data = &b"ACTG"[..];
        let res = nucleic_acid_sequence(data);

        let expect = vec![
            NucleicAcidCode::A,
            NucleicAcidCode::C,
            NucleicAcidCode::T,
            NucleicAcidCode::G,
        ];

        assert_eq!(res, Done(&b""[..], expect));
    }

    #[bench]
    fn bench_parse_description(b: &mut test::Bencher) {
        let data = include_bytes!("../data/bench_description.fa");

        b.bytes = data.len() as u64;
        b.iter(|| description(data));
    }

    #[bench]
    fn bench_sequence_from_u8(b: &mut test::Bencher) {
        let data = include_bytes!("../data/test.sequence");

        b.bytes = data.len() as u64;
        b.iter(|| from_u8(data));
    }

    #[bench]
    fn bench_sequence_from_str(b: &mut test::Bencher) {
        let data = include_bytes!("../data/test.sequence");
        let data = str::from_utf8(data).unwrap();

        b.bytes = data.len() as u64;
        b.iter(|| from_str(data));
    }

    #[bench]
    fn bench_sequence_from_str_itt(b: &mut test::Bencher) {
        let data = include_bytes!("../data/test.sequence");
        let data = str::from_utf8(data).unwrap();

        b.bytes = data.len() as u64;
        b.iter(|| from_str_itt(data));
    }

    #[bench]
    fn bench_parse_nucleic_acid_sequence(b: &mut test::Bencher) {
        let data = include_bytes!("../data/test.sequence");

        b.bytes = data.len() as u64;
        b.iter(|| nucleic_acid_sequence(data));
    }
}
