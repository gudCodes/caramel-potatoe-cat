use std::str::from_utf8;

use nom::not_line_ending;

#[derive(Debug,PartialEq,Eq)]
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

#[derive(Debug,PartialEq,Eq)]
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

fn str_to_nucleic_acid_code(s: &str) -> Result<NucleicAcidCode, &str> {
    match s.to_uppercase().as_ref() {
        "A" => Ok(NucleicAcidCode::A), // adenosine
        "C" => Ok(NucleicAcidCode::C), // cytidine
        "G" => Ok(NucleicAcidCode::G), // guanine
        "T" => Ok(NucleicAcidCode::T), // thymidine
        "N" => Ok(NucleicAcidCode::N), // A/G/C/T (any)
        "U" => Ok(NucleicAcidCode::U), // uridine
        "K" => Ok(NucleicAcidCode::K), // G/T (keto)
        "S" => Ok(NucleicAcidCode::S), // G/C (strong)
        "Y" => Ok(NucleicAcidCode::Y), // T/C (pyrimidine)
        "M" => Ok(NucleicAcidCode::M), // A/C (amino)
        "W" => Ok(NucleicAcidCode::W), // A/T (weak)
        "R" => Ok(NucleicAcidCode::R), // G/A (purine)
        "B" => Ok(NucleicAcidCode::B), // G/T/C
        "D" => Ok(NucleicAcidCode::D), // G/A/T
        "H" => Ok(NucleicAcidCode::H), // A/C/T
        "V" => Ok(NucleicAcidCode::V), // G/C/A
        "-" => Ok(NucleicAcidCode::Gap), // gap of indeterminate length
        _ => Err("unknown nucleic acid code '{}'"),
    }
}

named!(description,
    preceded!(char!('>'), not_line_ending)
);

named!(nucleic_acid_str(&[u8]) -> &str,
    map_res!(take!(1), from_utf8)
);

named!(nucleic_acid_code(&[u8]) -> NucleicAcidCode,
    map_res!(nucleic_acid_str, str_to_nucleic_acid_code)
);

named!(nucleic_acid_sequence(&[u8]) -> Vec<NucleicAcidCode>,
    many0!(nucleic_acid_code)
);

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use nom::IResult::Done;

    #[test]
    fn test_description() {
        let data = &b">ERR001275.1198"[..];

        let res = description(data);
        assert_eq!(res, Done(&b""[..], &data[1..]));
    }

    #[test]
    fn test_nucleic_acid_str() {
        let data = &b"A"[..];

        let res = nucleic_acid_str(data);
        assert_eq!(res, Done(&b""[..], "A"));
    }

    #[test]
    fn test_nucleic_acid_code() {
        let data = &b"A"[..];

        let res = nucleic_acid_code(data);
        assert_eq!(res, Done(&b""[..], NucleicAcidCode::A));
    }

    #[test]
    fn test_nucleic_acid_sequence() {
        let data = &b"ACTG"[..];

        let res = nucleic_acid_sequence(data);
        assert_eq!(res, Done(&b""[..], vec![
            NucleicAcidCode::A,
            NucleicAcidCode::C,
            NucleicAcidCode::T,
            NucleicAcidCode::G,
        ]));
    }

    #[bench]
    fn bench_description(b: &mut test::Bencher) {
        let data = include_bytes!("../data/bench_description.fa");

        b.bytes = data.len() as u64;
        b.iter(|| description(data));
    }

    #[bench]
    fn bench_nucleic_acid_str(b: &mut test::Bencher) {
        let data = &b"A"[..];

        b.bytes = 1;
        b.iter(|| nucleic_acid_str(data));
    }

    #[bench]
    fn bench_nucleic_acid_code(b: &mut test::Bencher) {
        let data = &b"A"[..];

        b.bytes = 1;
        b.iter(|| nucleic_acid_code(data));
    }

    #[bench]
    fn bench_nucleic_acid_sequence(b: &mut test::Bencher) {
        let data = &b"ACTG"[..];

        b.bytes = 4;
        b.iter(|| nucleic_acid_sequence(data));
    }
}
