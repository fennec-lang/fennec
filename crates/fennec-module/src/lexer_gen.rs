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
    Number,
    Ident,
    Dot,
    Dash,
    Plus,
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
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 5, 5, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
            5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
            7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
            7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ];
        macro_rules ! _fast_loop { ($ lex : ident , $ test : ident , $ miss : expr) => { while let Some (arr) = $ lex . read :: < & [u8 ; 16] > () { if $ test (arr [0]) { if $ test (arr [1]) { if $ test (arr [2]) { if $ test (arr [3]) { if $ test (arr [4]) { if $ test (arr [5]) { if $ test (arr [6]) { if $ test (arr [7]) { if $ test (arr [8]) { if $ test (arr [9]) { if $ test (arr [10]) { if $ test (arr [11]) { if $ test (arr [12]) { if $ test (arr [13]) { if $ test (arr [14]) { if $ test (arr [15]) { $ lex . bump_unchecked (16) ; continue ; } $ lex . bump_unchecked (15) ; return $ miss ; } $ lex . bump_unchecked (14) ; return $ miss ; } $ lex . bump_unchecked (13) ; return $ miss ; } $ lex . bump_unchecked (12) ; return $ miss ; } $ lex . bump_unchecked (11) ; return $ miss ; } $ lex . bump_unchecked (10) ; return $ miss ; } $ lex . bump_unchecked (9) ; return $ miss ; } $ lex . bump_unchecked (8) ; return $ miss ; } $ lex . bump_unchecked (7) ; return $ miss ; } $ lex . bump_unchecked (6) ; return $ miss ; } $ lex . bump_unchecked (5) ; return $ miss ; } $ lex . bump_unchecked (4) ; return $ miss ; } $ lex . bump_unchecked (3) ; return $ miss ; } $ lex . bump_unchecked (2) ; return $ miss ; } $ lex . bump_unchecked (1) ; return $ miss ; } return $ miss ; } while $ lex . test ($ test) { $ lex . bump_unchecked (1) ; } $ miss } ; }
        #[inline]
        fn goto16_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorStringUnterminated));
        }
        #[inline]
        fn goto8_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::String));
        }
        #[inline]
        fn pattern0(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn goto12_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorStringWithBackslashes));
        }
        #[inline]
        fn goto200_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto16_ctx16_x(lex),
            };
            match byte {
                byte if pattern0(byte) => {
                    lex.bump_unchecked(1usize);
                    goto200_ctx16_x(lex)
                }
                b'"' => {
                    lex.bump_unchecked(1usize);
                    goto12_ctx16_x(lex)
                }
                _ => goto16_ctx16_x(lex),
            }
        }
        #[inline]
        fn goto199_ctx16_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J8,
                J199,
                J200,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, __, J199, J199, __,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J8, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J200, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199, J199,
                    J199, J199, J199, J199, J199, J199, J199, J199,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto16_ctx16_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J8 => {
                    lex.bump_unchecked(1usize);
                    goto8_ctx16_x(lex)
                }
                Jump::J199 => {
                    lex.bump_unchecked(1usize);
                    goto199_ctx16_x(lex)
                }
                Jump::J200 => {
                    lex.bump_unchecked(1usize);
                    goto200_ctx16_x(lex)
                }
                Jump::__ => goto16_ctx16_x(lex),
            }
        }
        #[inline]
        fn goto1_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto22_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Number));
        }
        #[inline]
        fn goto22_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Number));
        }
        #[inline]
        fn goto49_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto24_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(1usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto30_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto53_at1_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J49,
                J24,
                J30,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, J30, __, J30, __, J30, __, J30, __, J30, __, J30, __, J30,
                    __, J30, __, J30, __, J30, __, J49, __, J49, J24, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J49 => goto49_at2_ctx22_x(lex),
                Jump::J24 => {
                    lex.bump_unchecked(2usize);
                    goto24_ctx22_x(lex)
                }
                Jump::J30 => goto30_at2_ctx22_x(lex),
                Jump::__ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto25_at3_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto49_at3_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto26_at3_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto187_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J25,
                J49,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, __, __, __, __,
                    J25, __, __, __, __, __, __, __, J25, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J49, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J25 => goto25_at3_ctx22_x(lex),
                Jump::J49 => goto49_at3_ctx22_x(lex),
                Jump::J26 => goto26_at3_ctx22_x(lex),
                Jump::__ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto90_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto174_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto110_at3_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto30_at3_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto161_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J24,
                J110,
                J25,
                J30,
                J49,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, J30, __, J25, J110, __, __, J49, __, __,
                    __, J25, __, __, __, __, __, J49, __, J49, __, __, __, __, __, J49, __, J26,
                    J25, __, __, __, __, __, __, J24, __, J49, __, __, __, __, __, __, __, __, __,
                    __, __, J49, __, __, __, J49, J24, __, __, __, __, __, __, J49, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J24 => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx22_x(lex)
                }
                Jump::J110 => goto110_at3_ctx22_x(lex),
                Jump::J25 => goto25_at3_ctx22_x(lex),
                Jump::J30 => goto30_at3_ctx22_x(lex),
                Jump::J49 => goto49_at3_ctx22_x(lex),
                Jump::J26 => goto26_at3_ctx22_x(lex),
                Jump::__ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto171_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J49,
                J24,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, J24, __, J26, __, J49, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J49 => goto49_at3_ctx22_x(lex),
                Jump::J24 => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx22_x(lex)
                }
                Jump::J26 => goto26_at3_ctx22_x(lex),
                Jump::__ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto98_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match byte {
                180u8 => goto25_at3_ctx22_x(lex),
                146u8 => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto189_at1_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J187,
                J90,
                J174,
                J161,
                J171,
                J98,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, J98, J161, __, __, __, __, J171, __, __, __, __, __, __, J174,
                    J187, J90, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J187 => goto187_at2_ctx22_x(lex),
                Jump::J90 => goto90_at2_ctx22_x(lex),
                Jump::J174 => goto174_at2_ctx22_x(lex),
                Jump::J161 => goto161_at2_ctx22_x(lex),
                Jump::J171 => goto171_at2_ctx22_x(lex),
                Jump::J98 => goto98_at2_ctx22_x(lex),
                Jump::__ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto92_at1_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto63_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            match byte {
                128u8..=137u8 | 144u8..=153u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto70_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto25_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto26_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto78_at1_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J63,
                J70,
                J49,
                J24,
                J25,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, J26, J49, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J24, J49, __, __, __, __, J63, __, J49, __, __, J70, __, __, J49, J25, __, __,
                    J70, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J63 => goto63_at2_ctx22_x(lex),
                Jump::J70 => goto70_at2_ctx22_x(lex),
                Jump::J49 => goto49_at2_ctx22_x(lex),
                Jump::J24 => {
                    lex.bump_unchecked(2usize);
                    goto24_ctx22_x(lex)
                }
                Jump::J25 => goto25_at2_ctx22_x(lex),
                Jump::J26 => goto26_at2_ctx22_x(lex),
                Jump::__ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto25_at1_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn pattern2(byte: u8) -> bool {
            match byte {
                144u8..=153u8 | 176u8..=185u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto87_at2_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto91_at1_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J87,
                J49,
                J24,
                J25,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, J24, __, __, __, __, __, __,
                    __, __, __, __, J49, J26, __, __, J87, __, J49, __, __, __, __, __, J25, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J87 => goto87_at2_ctx22_x(lex),
                Jump::J49 => goto49_at2_ctx22_x(lex),
                Jump::J24 => {
                    lex.bump_unchecked(2usize);
                    goto24_ctx22_x(lex)
                }
                Jump::J25 => goto25_at2_ctx22_x(lex),
                Jump::J26 => goto26_at2_ctx22_x(lex),
                Jump::__ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto26_at1_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto23_ctx22_x(lex)
                }
                _ => goto22_x(lex),
            }
        }
        #[inline]
        fn goto23_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J53,
                J23,
                J189,
                J92,
                J78,
                J25,
                J24,
                J91,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J23, J23, J23, J23, J23, J23, J23, J23, J23,
                    J23, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J24, __, J25, __, __, __, J26, J53, J78, __, __, __, __, __, __, __, __,
                    J91, __, __, __, __, J92, J189, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto22_ctx22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J53 => goto53_at1_ctx22_x(lex),
                Jump::J23 => {
                    lex.bump_unchecked(1usize);
                    goto23_ctx22_x(lex)
                }
                Jump::J189 => goto189_at1_ctx22_x(lex),
                Jump::J92 => goto92_at1_ctx22_x(lex),
                Jump::J78 => goto78_at1_ctx22_x(lex),
                Jump::J25 => goto25_at1_ctx22_x(lex),
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx22_x(lex)
                }
                Jump::J91 => goto91_at1_ctx22_x(lex),
                Jump::J26 => goto26_at1_ctx22_x(lex),
                Jump::__ => goto22_ctx22_x(lex),
            }
        }
        #[inline]
        fn goto92_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto24_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(1usize);
                    goto23_ctx22_x(lex)
                }
                _ => lex.error(),
            }
        }
        #[inline]
        fn goto191_ctx191_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Ident));
        }
        #[inline]
        fn pattern3(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 2 > 0
        }
        #[inline]
        fn goto192_ctx191_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern3, goto191_ctx191_x(lex));
        }
        #[inline]
        fn goto203_ctx22_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J53,
                J91,
                J192,
                J189,
                J92,
                J78,
                J25,
                J24,
                J203,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, J192, __, __, J203, J203, J203, J203, J203, J203, J203,
                    J203, J203, J203, __, __, __, __, __, __, __, J192, J192, J192, J192, J192,
                    J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192,
                    J192, J192, J192, J192, J192, J192, J192, J192, __, __, __, __, __, __, J192,
                    J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192,
                    J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, J24, __, J25, __, __, __, J26,
                    J53, J78, __, __, __, __, __, __, __, __, J91, __, __, __, __, J92, J189, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto22_ctx22_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J53 => goto53_at1_ctx22_x(lex),
                Jump::J91 => goto91_at1_ctx22_x(lex),
                Jump::J192 => {
                    lex.bump_unchecked(1usize);
                    goto192_ctx191_x(lex)
                }
                Jump::J189 => goto189_at1_ctx22_x(lex),
                Jump::J92 => goto92_at1_ctx22_x(lex),
                Jump::J78 => goto78_at1_ctx22_x(lex),
                Jump::J25 => goto25_at1_ctx22_x(lex),
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx22_x(lex)
                }
                Jump::J203 => {
                    lex.bump_unchecked(1usize);
                    goto203_ctx22_x(lex)
                }
                Jump::J26 => goto26_at1_ctx22_x(lex),
                Jump::__ => goto22_ctx22_x(lex),
            }
        }
        #[inline]
        fn goto26_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto49_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto30_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto53_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J49,
                J24,
                J30,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, J30, __, J30, __, J30, __, J30, __, J30, __, J30, __, J30,
                    __, J30, __, J30, __, J30, __, J49, __, J49, J24, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J49 => goto49_at2(lex),
                Jump::J24 => {
                    lex.bump_unchecked(2usize);
                    goto24_x(lex)
                }
                Jump::J30 => goto30_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto3_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Whitespace));
        }
        #[inline]
        fn pattern4(byte: u8) -> bool {
            match byte {
                9u8 | 32u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto4_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern4, goto3_ctx3_x(lex));
        }
        #[inline]
        fn goto196_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Plus));
        }
        #[inline]
        fn goto7_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordFennec));
        }
        #[inline]
        fn goto210_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto7_ctx192_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(1usize);
                    goto192_ctx191_x(lex)
                }
                _ => goto7_ctx192_x(lex),
            }
        }
        #[inline]
        fn goto209_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"ennec") => {
                    lex.bump_unchecked(5usize);
                    goto210_ctx192_x(lex)
                }
                _ => goto192_ctx191_x(lex),
            }
        }
        #[inline]
        fn goto195_ctx195_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Dash));
        }
        #[inline]
        fn goto211_ctx195_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto195_ctx195_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(1usize);
                    goto192_ctx191_x(lex)
                }
                _ => goto195_ctx195_x(lex),
            }
        }
        #[inline]
        fn goto194_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Dot));
        }
        #[inline]
        fn goto25_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto49_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto26_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto187_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J25,
                J49,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, __, __, __, __,
                    J25, __, __, __, __, __, __, __, J25, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J49, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J25 => goto25_at3(lex),
                Jump::J49 => goto49_at3(lex),
                Jump::J26 => goto26_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto90_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto174_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto110_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto30_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto161_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J24,
                J110,
                J25,
                J30,
                J49,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, J30, __, J25, J110, __, __, J49, __, __,
                    __, J25, __, __, __, __, __, J49, __, J49, __, __, __, __, __, J49, __, J26,
                    J25, __, __, __, __, __, __, J24, __, J49, __, __, __, __, __, __, __, __, __,
                    __, __, J49, __, __, __, J49, J24, __, __, __, __, __, __, J49, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J24 => {
                    lex.bump_unchecked(3usize);
                    goto24_x(lex)
                }
                Jump::J110 => goto110_at3(lex),
                Jump::J25 => goto25_at3(lex),
                Jump::J30 => goto30_at3(lex),
                Jump::J49 => goto49_at3(lex),
                Jump::J26 => goto26_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto171_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J49,
                J24,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, J24, __, J26, __, J49, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J49 => goto49_at3(lex),
                Jump::J24 => {
                    lex.bump_unchecked(3usize);
                    goto24_x(lex)
                }
                Jump::J26 => goto26_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto98_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                180u8 => goto25_at3(lex),
                146u8 => {
                    lex.bump_unchecked(3usize);
                    goto24_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto189_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J187,
                J90,
                J174,
                J161,
                J171,
                J98,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, J98, J161, __, __, __, __, J171, __, __, __, __, __, __, J174,
                    J187, J90, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J187 => goto187_at2(lex),
                Jump::J90 => goto90_at2(lex),
                Jump::J174 => goto174_at2(lex),
                Jump::J161 => goto161_at2(lex),
                Jump::J171 => goto171_at2(lex),
                Jump::J98 => goto98_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto2_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto204_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(&[10u8]) => {
                    lex.bump_unchecked(2usize);
                    goto2_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto6_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordModule));
        }
        #[inline]
        fn goto207_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto6_ctx192_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(1usize);
                    goto192_ctx191_x(lex)
                }
                _ => goto6_ctx192_x(lex),
            }
        }
        #[inline]
        fn goto206_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"odule") => {
                    lex.bump_unchecked(5usize);
                    goto207_ctx192_x(lex)
                }
                _ => goto192_ctx191_x(lex),
            }
        }
        #[inline]
        fn goto25_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto63_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto70_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto25_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto26_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto78_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J63,
                J70,
                J49,
                J24,
                J25,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, J26, J49, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J24, J49, __, __, __, __, J63, __, J49, __, __, J70, __, __, J49, J25, __, __,
                    J70, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J63 => goto63_at2(lex),
                Jump::J70 => goto70_at2(lex),
                Jump::J49 => goto49_at2(lex),
                Jump::J24 => {
                    lex.bump_unchecked(2usize);
                    goto24_x(lex)
                }
                Jump::J25 => goto25_at2(lex),
                Jump::J26 => goto26_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto87_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto23_ctx22_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto91_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J87,
                J49,
                J24,
                J25,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, J24, __, __, __, __, __, __,
                    __, __, __, __, J49, J26, __, __, J87, __, J49, __, __, __, __, __, J25, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J87 => goto87_at2(lex),
                Jump::J49 => goto49_at2(lex),
                Jump::J24 => {
                    lex.bump_unchecked(2usize);
                    goto24_x(lex)
                }
                Jump::J25 => goto25_at2(lex),
                Jump::J26 => goto26_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto19_ctx19_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Comment));
        }
        #[inline]
        fn pattern5(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 4 > 0
        }
        #[inline]
        fn goto20_ctx19_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern5, goto19_ctx19_x(lex));
        }
        #[inline]
        fn goto202_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(b"/") => {
                    lex.bump_unchecked(2usize);
                    goto20_ctx19_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto212<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J199,
                J1,
                J92,
                J24,
                J203,
                J26,
                J53,
                J4,
                J196,
                J209,
                J192,
                J211,
                J194,
                J189,
                J204,
                J206,
                J25,
                J78,
                J91,
                J202,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, J4, J1, __, __, J204, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J4, __, J199, __, __, __,
                    __, __, __, __, __, J196, __, J211, J194, J202, J203, J203, J203, J203, J203,
                    J203, J203, J203, J203, J203, __, __, __, __, __, __, __, J192, J192, J192,
                    J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192,
                    J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, __, __, __, __, __,
                    __, J192, J192, J192, J192, J192, J209, J192, J192, J192, J192, J192, J192,
                    J206, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192, J192,
                    J192, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J24, __, J25,
                    __, __, __, J26, J53, J78, __, __, __, __, __, __, __, __, J91, __, __, __, __,
                    J92, J189, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return _end(lex),
            };
            match LUT[byte as usize] {
                Jump::J199 => {
                    lex.bump_unchecked(1usize);
                    goto199_ctx16_x(lex)
                }
                Jump::J1 => {
                    lex.bump_unchecked(1usize);
                    goto1_x(lex)
                }
                Jump::J92 => goto92_at1(lex),
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_x(lex)
                }
                Jump::J203 => {
                    lex.bump_unchecked(1usize);
                    goto203_ctx22_x(lex)
                }
                Jump::J26 => goto26_at1(lex),
                Jump::J53 => goto53_at1(lex),
                Jump::J4 => {
                    lex.bump_unchecked(1usize);
                    goto4_ctx3_x(lex)
                }
                Jump::J196 => {
                    lex.bump_unchecked(1usize);
                    goto196_x(lex)
                }
                Jump::J209 => {
                    lex.bump_unchecked(1usize);
                    goto209_ctx192_x(lex)
                }
                Jump::J192 => {
                    lex.bump_unchecked(1usize);
                    goto192_ctx191_x(lex)
                }
                Jump::J211 => {
                    lex.bump_unchecked(1usize);
                    goto211_ctx195_x(lex)
                }
                Jump::J194 => {
                    lex.bump_unchecked(1usize);
                    goto194_x(lex)
                }
                Jump::J189 => goto189_at1(lex),
                Jump::J204 => goto204_at1(lex),
                Jump::J206 => {
                    lex.bump_unchecked(1usize);
                    goto206_ctx192_x(lex)
                }
                Jump::J25 => goto25_at1(lex),
                Jump::J78 => goto78_at1(lex),
                Jump::J91 => goto91_at1(lex),
                Jump::J202 => goto202_at1(lex),
                Jump::__ => _error(lex),
            }
        }
        goto212(lex)
    }
}
