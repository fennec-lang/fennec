#![allow(unused_imports)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum LogosToken {
    Newline,
    Whitespace,
    KeywordModule,
    KeywordFennec,
    String,
    ErrorStringWithBackslashes,
    ErrorStringUnterminated,
    Comment,
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
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        ];
        macro_rules ! _fast_loop { ($ lex : ident , $ test : ident , $ miss : expr) => { while let Some (arr) = $ lex . read :: < & [u8 ; 16] > () { if $ test (arr [0]) { if $ test (arr [1]) { if $ test (arr [2]) { if $ test (arr [3]) { if $ test (arr [4]) { if $ test (arr [5]) { if $ test (arr [6]) { if $ test (arr [7]) { if $ test (arr [8]) { if $ test (arr [9]) { if $ test (arr [10]) { if $ test (arr [11]) { if $ test (arr [12]) { if $ test (arr [13]) { if $ test (arr [14]) { if $ test (arr [15]) { $ lex . bump_unchecked (16) ; continue ; } $ lex . bump_unchecked (15) ; return $ miss ; } $ lex . bump_unchecked (14) ; return $ miss ; } $ lex . bump_unchecked (13) ; return $ miss ; } $ lex . bump_unchecked (12) ; return $ miss ; } $ lex . bump_unchecked (11) ; return $ miss ; } $ lex . bump_unchecked (10) ; return $ miss ; } $ lex . bump_unchecked (9) ; return $ miss ; } $ lex . bump_unchecked (8) ; return $ miss ; } $ lex . bump_unchecked (7) ; return $ miss ; } $ lex . bump_unchecked (6) ; return $ miss ; } $ lex . bump_unchecked (5) ; return $ miss ; } $ lex . bump_unchecked (4) ; return $ miss ; } $ lex . bump_unchecked (3) ; return $ miss ; } $ lex . bump_unchecked (2) ; return $ miss ; } $ lex . bump_unchecked (1) ; return $ miss ; } return $ miss ; } while $ lex . test ($ test) { $ lex . bump_unchecked (1) ; } $ miss } ; }
        #[inline]
        fn goto6_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordModule));
        }
        #[inline]
        fn goto29_at1<'s>(lex: &mut Lexer<'s>) {
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
        fn goto28_at1<'s>(lex: &mut Lexer<'s>) {
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
            lex.set(Ok(LogosToken::KeywordFennec));
        }
        #[inline]
        fn goto30_at1<'s>(lex: &mut Lexer<'s>) {
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
        fn goto16_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorStringUnterminated));
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn goto12_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorStringWithBackslashes));
        }
        #[inline]
        fn goto25_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto16_ctx16_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(1usize);
                    goto25_ctx16_x(lex)
                }
                b'"' => {
                    lex.bump_unchecked(1usize);
                    goto12_ctx16_x(lex)
                }
                _ => goto16_ctx16_x(lex),
            }
        }
        #[inline]
        fn goto8_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::String));
        }
        #[inline]
        fn goto24_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J25,
                J24,
                J8,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, __, J24, J24, __, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J8, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J25, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto16_ctx16_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J25 => {
                    lex.bump_unchecked(1usize);
                    goto25_ctx16_x(lex)
                }
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx16_x(lex)
                }
                Jump::J8 => {
                    lex.bump_unchecked(1usize);
                    goto8_ctx16_x(lex)
                }
                Jump::__ => goto16_ctx16_x(lex),
            }
        }
        #[inline]
        fn goto19_ctx19_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Comment));
        }
        #[inline]
        fn pattern2(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 2 > 0
        }
        #[inline]
        fn goto20_ctx19_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern2, goto19_ctx19_x(lex));
        }
        #[inline]
        fn goto27_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(b"/") => {
                    lex.bump_unchecked(2usize);
                    goto20_ctx19_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto31<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J29,
                J4,
                J28,
                J30,
                J1,
                J24,
                J27,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, J4, J1, __, __, J28, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J4, __, J24, __, __, __,
                    __, __, __, __, __, __, __, __, __, J27, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, J30, __, __, __, __, __, __, J29, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return _end(lex),
            };
            match LUT[byte as usize] {
                Jump::J29 => goto29_at1(lex),
                Jump::J4 => {
                    lex.bump_unchecked(1usize);
                    goto4_ctx3_x(lex)
                }
                Jump::J28 => goto28_at1(lex),
                Jump::J30 => goto30_at1(lex),
                Jump::J1 => {
                    lex.bump_unchecked(1usize);
                    goto1_x(lex)
                }
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx16_x(lex)
                }
                Jump::J27 => goto27_at1(lex),
                Jump::__ => _error(lex),
            }
        }
        goto31(lex)
    }
}
