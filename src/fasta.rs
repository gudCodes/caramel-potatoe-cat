use nom::not_line_ending;

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
