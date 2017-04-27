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

type NucleicAcidSequence = Vec<NucleicAcidCode>;

fn from_u8(ns: &[u8]) -> Result<NucleicAcidSequence, &'static str> {
    let len = ns.len();

    let mut res: NucleicAcidSequence = NucleicAcidSequence::new();

    res.resize(len, NucleicAcidCode::Gap);

    for i in 0..len {
        let mut arr = res.as_mut_slice();
        match ns[i] {
            65 => { arr[i] = NucleicAcidCode::A },
            67 => { arr[i] = NucleicAcidCode::C },
            71 => { arr[i] = NucleicAcidCode::G },
            84 => { arr[i] = NucleicAcidCode::T },
            85 => { arr[i] = NucleicAcidCode::U },
            78 => { arr[i] = NucleicAcidCode::N },
            75 => { arr[i] = NucleicAcidCode::K },
            83 => { arr[i] = NucleicAcidCode::S },
            89 => { arr[i] = NucleicAcidCode::Y },
            77 => { arr[i] = NucleicAcidCode::M },
            87 => { arr[i] = NucleicAcidCode::W },
            82 => { arr[i] = NucleicAcidCode::R },
            66 => { arr[i] = NucleicAcidCode::B },
            68 => { arr[i] = NucleicAcidCode::D },
            72 => { arr[i] = NucleicAcidCode::H },
            86 => { arr[i] = NucleicAcidCode::V },
            45 => { arr[i] = NucleicAcidCode::Gap },
            _ => return Err("invalid nucleic acid code"),
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
    fn bench_parse_nucleic_acid_sequence(b: &mut test::Bencher) {
        let data = include_bytes!("../data/test.sequence");

        b.bytes = data.len() as u64;
        b.iter(|| nucleic_acid_sequence(data));
    }
}
