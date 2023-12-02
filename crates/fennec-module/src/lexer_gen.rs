#![allow(unused_imports)]
#[derive(PartialEq, Eq)]
enum LogosToken {
    Newline,
    Whitespace,
    Module,
    Fennec,
    String,
}
impl<'s> ::logos::Logos<'s> for LogosToken {
    type Error = ();
    type Extras = ();
    type Source = str;
    fn lex(lex: &mut ::logos::Lexer<'s, Self>) {
        use logos::internal::{CallbackResult, LexerInternal};
        type Lexer<'s> = ::logos::Lexer<'s, LogosToken>;
        fn _end<'s>(lex: &mut Lexer<'s>) {
            lex.end()
        }
        fn _error<'s>(lex: &mut Lexer<'s>) {
            lex.bump_unchecked(1);
            lex.error();
        }
        static COMPACT_TABLE_0: [u8; 256] = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        macro_rules ! _fast_loop { ($ lex : ident , $ test : ident , $ miss : expr) => { while let Some (arr) = $ lex . read :: < & [u8 ; 16] > () { if $ test (arr [0]) { if $ test (arr [1]) { if $ test (arr [2]) { if $ test (arr [3]) { if $ test (arr [4]) { if $ test (arr [5]) { if $ test (arr [6]) { if $ test (arr [7]) { if $ test (arr [8]) { if $ test (arr [9]) { if $ test (arr [10]) { if $ test (arr [11]) { if $ test (arr [12]) { if $ test (arr [13]) { if $ test (arr [14]) { if $ test (arr [15]) { $ lex . bump_unchecked (16) ; continue ; } $ lex . bump_unchecked (15) ; return $ miss ; } $ lex . bump_unchecked (14) ; return $ miss ; } $ lex . bump_unchecked (13) ; return $ miss ; } $ lex . bump_unchecked (12) ; return $ miss ; } $ lex . bump_unchecked (11) ; return $ miss ; } $ lex . bump_unchecked (10) ; return $ miss ; } $ lex . bump_unchecked (9) ; return $ miss ; } $ lex . bump_unchecked (8) ; return $ miss ; } $ lex . bump_unchecked (7) ; return $ miss ; } $ lex . bump_unchecked (6) ; return $ miss ; } $ lex . bump_unchecked (5) ; return $ miss ; } $ lex . bump_unchecked (4) ; return $ miss ; } $ lex . bump_unchecked (3) ; return $ miss ; } $ lex . bump_unchecked (2) ; return $ miss ; } $ lex . bump_unchecked (1) ; return $ miss ; } return $ miss ; } while $ lex . test ($ test) { $ lex . bump_unchecked (1) ; } $ miss } ; }
        #[inline]
        fn goto6_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Module));
        }
        #[inline]
        fn goto13_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 5usize]>(1usize) {
                Some(b"odule") => {
                    lex.bump_unchecked(6usize);
                    goto6_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto3_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Whitespace));
        }
        #[inline]
        fn pattern0(byte: u8) -> bool {
            match byte {
                9u8 | 32u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto4_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern0, goto3_ctx3_x(lex));
        }
        #[inline]
        fn goto2_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto12_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(&[10u8]) => {
                    lex.bump_unchecked(2usize);
                    goto2_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto7_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Fennec));
        }
        #[inline]
        fn goto14_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 5usize]>(1usize) {
                Some(b"ennec") => {
                    lex.bump_unchecked(6usize);
                    goto7_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto1_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto8_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::String));
        }
        #[inline]
        fn goto9_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"\"") => {
                    lex.bump_unchecked(1usize);
                    goto8_x(lex)
                }
                _ => lex.error(),
            }
        }
        #[inline]
        fn goto8_ctx9_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::String));
        }
        #[inline]
        fn goto9_ctx9_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"\"") => {
                    lex.bump_unchecked(1usize);
                    goto8_ctx9_x(lex)
                }
                _ => goto9_x(lex),
            }
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn goto10_ctx9_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto9_ctx9_x(lex));
        }
        #[inline]
        fn goto15<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J13,
                J4,
                J12,
                J14,
                J1,
                J10,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, J4, J1, __, __, J12, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J4, __, J10, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, J14, __, __, __, __, __, __, J13, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return _end(lex),
            };
            match LUT[byte as usize] {
                Jump::J13 => goto13_at1(lex),
                Jump::J4 => {
                    lex.bump_unchecked(1usize);
                    goto4_ctx3_x(lex)
                }
                Jump::J12 => goto12_at1(lex),
                Jump::J14 => goto14_at1(lex),
                Jump::J1 => {
                    lex.bump_unchecked(1usize);
                    goto1_x(lex)
                }
                Jump::J10 => {
                    lex.bump_unchecked(1usize);
                    goto10_ctx9_x(lex)
                }
                Jump::__ => _error(lex),
            }
        }
        goto15(lex)
    }
}
