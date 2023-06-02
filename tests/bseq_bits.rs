use bit_seq::bseq;

#[test]
fn test_bseq_bits_empty() {
    let t = bseq!();
    assert_eq!(t, 0);
}

#[test]
fn test_bseq_bits_single_1() {
    let t = bseq!(1);
    assert_eq!(t, 1);
}

#[test]
fn test_bseq_bits_single_2() {
    let t = bseq!(0);
    assert_eq!(t, 0);
}

#[test]
fn test_bseq_bits_single_seq_1() {
    let t = bseq!(101);
    assert_eq!(t, 0b101);
}

#[test]
fn test_bseq_bits_single_seq_2() {
    let t = bseq!(00101);
    assert_eq!(t, 0b101);
}

#[test]
fn test_bseq_bits_multi_seq_1() {
    let t = bseq!(00 101);
    assert_eq!(t, 0b101);
}

#[test]
fn test_bseq_bits_multi_seq_2() {
    let t = bseq!(10 101);
    assert_eq!(t, 0b10101);
}

#[test]
fn test_bseq_bits_multi_seq_3() {
    let t = bseq!(1 0 11   0);
    assert_eq!(t, 0b10110);
}

#[test]
fn test_bseq_overflow_1() {
    let t: u8 = bseq!(111111111);
    assert_eq!(t, 255);
}

#[test]
fn test_bseq_overflow_2() {
    let t: i8 = bseq!(110000000);
    assert_eq!(t, -128);
}
