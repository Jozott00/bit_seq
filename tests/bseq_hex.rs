
    use bit_seq::bseq;

    #[test]
    fn test_bseq_with_hex_input() {
        assert_eq!(bseq!(0x0), 0);
        assert_eq!(bseq!(0x1), 1);
        assert_eq!(bseq!(0x2), 2);
        assert_eq!(bseq!(0xf), 15);
        assert_eq!(bseq!(0x10), 16);
        assert_eq!(bseq!(0xff), 255);
        assert_eq!(bseq!(0x100), 256);
        assert_eq!(bseq!(0xffff), 65535);
    }

