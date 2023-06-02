use bit_seq::bseq;

#[test]
fn test_bseq_bits() {
    assert_eq!(bseq!(), 0);
    assert_eq!(bseq!(1), 1);
    assert_eq!(bseq!(0), 0);
    assert_eq!(bseq!(101), 0b101);
    assert_eq!(bseq!(00101), 0b101);
    assert_eq!(bseq!(10 1), 0b101);
    assert_eq!(bseq!(10 101), 0b10101);
    assert_eq!(bseq!(1 0 11     0), 0b10110);

    let t_u8: u8 = bseq!(111111111);
    assert_eq!(t_u8, 255);
}

#[test]
fn test_bseq_hex() {
    assert_eq!(bseq!(0x0), 0);
    assert_eq!(bseq!(0x0 0x0), 0);
    assert_eq!(bseq!(0x1    0x1), 0b00010001);
    assert_eq!(bseq!(0x2), 2);
    assert_eq!(bseq!(0xf), 15);
    assert_eq!(bseq!(0x10), 16);
    assert_eq!(bseq!(0xff), 255);
    assert_eq!(bseq!(0x100), 256);
    assert_eq!(bseq!(0xffff), 65535);

    let t_u8: u8 = bseq!(0xff);
    assert_eq!(t_u8, 255);

    let t_u32: u32 = bseq!(0xffffffff);
    assert_eq!(t_u32, u32::MAX);

    assert_eq!(bseq!(0xfffffffffffu64), 0xfffffffffffu64);
}

#[test]
fn test_bseq_int_expr() {
    assert_eq!(bseq!(0:0), 0);
    assert_eq!(bseq!(0:1), 0);
    assert_eq!(bseq!(1:0), 0);
    assert_eq!(bseq!(1:10), 1);
    assert_eq!(bseq!(3:2), 3);
    assert_eq!(bseq!(3:1), 1);
    assert_eq!(bseq!(2:1), 0);

    let t_u8: u8 = bseq!(1:8);
    assert_eq!(t_u8, 1);

    let t_u32: u32 = bseq!(1:32);
    assert_eq!(t_u32, 1);

    assert_eq!(bseq!(2:1 2:1), 0);
    assert_eq!(bseq!(3:1 3:2), 0b111);

    let t_u32: u32 = bseq!(1:1 0:31);
    assert_eq!(t_u32, 1 << 31);
}

#[test]
fn test_bseq_var_expr() {
    let s_0 = 0;
    assert_eq!(bseq!(s_0:0), 0);
    assert_eq!(bseq!(s_0:1), 0);
    let s_1 = 1;
    let s_1_u32: u32 = s_1 as u32;
    assert_eq!(bseq!(s_1_u32:0), 0);
    assert_eq!(bseq!(s_1_u32:10), 1);
    let s_3 = 3;
    assert_eq!(bseq!(s_3:2), 3);
    assert_eq!(bseq!(s_3:1), 1);

    let t_u8: u8 = bseq!(s_1:8);
    assert_eq!(t_u8, 1);

    let t_u32: u32 = bseq!(s_1_u32:32);
    assert_eq!(t_u32, 1);

    let s_2 = 2;
    assert_eq!(bseq!(s_2:1 s_2:1), 0);
    assert_eq!(bseq!(s_3:1 s_3:2), 0b111);

    let t_u32: u32 = bseq!(s_1_u32:1 s_0:31);
    assert_eq!(t_u32, 1 << 31);
}

#[test]
fn test_bseq_macro_mixed() {
    assert_eq!(bseq!(1 0x1:4), 0b1_0001);
    assert_eq!(bseq!(10 2:4 1 1), 0b10_0010_1_1);
    assert_eq!(bseq!(0x0f:4 1111), 255);
    let var = 0x10;
    assert_eq!(bseq!(10000 var:5), 528);
}