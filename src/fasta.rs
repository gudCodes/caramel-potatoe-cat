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

named!(description, preceded!(char!('>'), not_line_ending));

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use nom::IResult::Done;

    #[test]
    fn test_description() {
        let res = description(&b">abc 123"[..]);
        assert_eq!(res, Done(&b""[..], &b"abc 123"[..]));
    }

    #[bench]
    fn bench_description(b: &mut test::Bencher) {
        b.bytes = 8;
        b.iter(|| description(&b">abc 123"[..]));
    }
}
