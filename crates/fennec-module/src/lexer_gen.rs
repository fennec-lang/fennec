#![allow(unused_imports)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum LogosToken {
    Newline,
    ErrorSingleCarriageReturn,
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
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 2, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
            6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
            7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
            7, 7, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        ];
        macro_rules ! _fast_loop { ($ lex : ident , $ test : ident , $ miss : expr) => { while let Some (arr) = $ lex . read :: < & [u8 ; 16] > () { if $ test (arr [0]) { if $ test (arr [1]) { if $ test (arr [2]) { if $ test (arr [3]) { if $ test (arr [4]) { if $ test (arr [5]) { if $ test (arr [6]) { if $ test (arr [7]) { if $ test (arr [8]) { if $ test (arr [9]) { if $ test (arr [10]) { if $ test (arr [11]) { if $ test (arr [12]) { if $ test (arr [13]) { if $ test (arr [14]) { if $ test (arr [15]) { $ lex . bump_unchecked (16) ; continue ; } $ lex . bump_unchecked (15) ; return $ miss ; } $ lex . bump_unchecked (14) ; return $ miss ; } $ lex . bump_unchecked (13) ; return $ miss ; } $ lex . bump_unchecked (12) ; return $ miss ; } $ lex . bump_unchecked (11) ; return $ miss ; } $ lex . bump_unchecked (10) ; return $ miss ; } $ lex . bump_unchecked (9) ; return $ miss ; } $ lex . bump_unchecked (8) ; return $ miss ; } $ lex . bump_unchecked (7) ; return $ miss ; } $ lex . bump_unchecked (6) ; return $ miss ; } $ lex . bump_unchecked (5) ; return $ miss ; } $ lex . bump_unchecked (4) ; return $ miss ; } $ lex . bump_unchecked (3) ; return $ miss ; } $ lex . bump_unchecked (2) ; return $ miss ; } $ lex . bump_unchecked (1) ; return $ miss ; } return $ miss ; } while $ lex . test ($ test) { $ lex . bump_unchecked (1) ; } $ miss } ; }
        #[inline]
        fn goto4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
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
        fn goto5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern0, goto4_ctx4_x(lex));
        }
        #[inline]
        fn goto197_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Plus));
        }
        #[inline]
        fn goto23_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Number));
        }
        #[inline]
        fn goto23_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Number));
        }
        #[inline]
        fn goto25_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto31_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto50_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto54_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J25,
                J31,
                J50,
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
                    __, __, __, __, __, J31, __, J31, __, J31, __, J31, __, J31, __, J31, __, J31,
                    __, J31, __, J31, __, J31, __, J50, __, J50, J25, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J25 => {
                    lex.bump_unchecked(2usize);
                    goto25_ctx23_x(lex)
                }
                Jump::J31 => goto31_at2_ctx23_x(lex),
                Jump::J50 => goto50_at2_ctx23_x(lex),
                Jump::__ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto93_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto26_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            match byte {
                144u8..=153u8 | 176u8..=185u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto88_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto27_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto92_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J26,
                J25,
                J88,
                J27,
                J50,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J25, __, __, __, __, __, __,
                    __, __, __, __, J50, J27, __, __, J88, __, J50, __, __, __, __, __, J26, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J26 => goto26_at2_ctx23_x(lex),
                Jump::J25 => {
                    lex.bump_unchecked(2usize);
                    goto25_ctx23_x(lex)
                }
                Jump::J88 => goto88_at2_ctx23_x(lex),
                Jump::J27 => goto27_at2_ctx23_x(lex),
                Jump::J50 => goto50_at2_ctx23_x(lex),
                Jump::__ => goto23_x(lex),
            }
        }
        #[inline]
        fn pattern2(byte: u8) -> bool {
            match byte {
                128u8..=137u8 | 144u8..=153u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto71_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto64_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto79_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J26,
                J71,
                J25,
                J64,
                J27,
                J50,
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
                    __, __, __, __, __, __, __, __, __, J27, J50, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J25, J50, __, __, __, __, J64, __, J50, __, __, J71, __, __, J50, J26, __, __,
                    J71, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J26 => goto26_at2_ctx23_x(lex),
                Jump::J71 => goto71_at2_ctx23_x(lex),
                Jump::J25 => {
                    lex.bump_unchecked(2usize);
                    goto25_ctx23_x(lex)
                }
                Jump::J64 => goto64_at2_ctx23_x(lex),
                Jump::J27 => goto27_at2_ctx23_x(lex),
                Jump::J50 => goto50_at2_ctx23_x(lex),
                Jump::__ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto91_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto27_at3_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto50_at3_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto172_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J25,
                J27,
                J50,
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
                    __, __, __, __, __, __, __, __, __, J25, __, J27, __, J50, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J25 => {
                    lex.bump_unchecked(3usize);
                    goto25_ctx23_x(lex)
                }
                Jump::J27 => goto27_at3_ctx23_x(lex),
                Jump::J50 => goto50_at3_ctx23_x(lex),
                Jump::__ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto175_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto26_at3_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto188_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J50,
                J27,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J27, __, __, __, __, __,
                    J26, __, __, __, __, __, __, __, J26, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J50, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J50 => goto50_at3_ctx23_x(lex),
                Jump::J27 => goto27_at3_ctx23_x(lex),
                Jump::J26 => goto26_at3_ctx23_x(lex),
                Jump::__ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto99_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match byte {
                146u8 => {
                    lex.bump_unchecked(3usize);
                    goto25_ctx23_x(lex)
                }
                180u8 => goto26_at3_ctx23_x(lex),
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto111_at3_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto31_at3_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto162_at2_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J50,
                J111,
                J31,
                J25,
                J27,
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
                    __, __, __, __, __, __, __, __, __, J31, __, J26, J111, __, __, J50, __, __,
                    __, J26, __, __, __, __, __, J50, __, J50, __, __, __, __, __, J50, __, J27,
                    J26, __, __, __, __, __, __, J25, __, J50, __, __, __, __, __, __, __, __, __,
                    __, __, J50, __, __, __, J50, J25, __, __, __, __, __, __, J50, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J50 => goto50_at3_ctx23_x(lex),
                Jump::J111 => goto111_at3_ctx23_x(lex),
                Jump::J31 => goto31_at3_ctx23_x(lex),
                Jump::J25 => {
                    lex.bump_unchecked(3usize);
                    goto25_ctx23_x(lex)
                }
                Jump::J27 => goto27_at3_ctx23_x(lex),
                Jump::J26 => goto26_at3_ctx23_x(lex),
                Jump::__ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto190_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J91,
                J172,
                J175,
                J188,
                J99,
                J162,
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
                    __, __, __, __, J99, J162, __, __, __, __, J172, __, __, __, __, __, __, J175,
                    J188, J91, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J91 => goto91_at2_ctx23_x(lex),
                Jump::J172 => goto172_at2_ctx23_x(lex),
                Jump::J175 => goto175_at2_ctx23_x(lex),
                Jump::J188 => goto188_at2_ctx23_x(lex),
                Jump::J99 => goto99_at2_ctx23_x(lex),
                Jump::J162 => goto162_at2_ctx23_x(lex),
                Jump::__ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto27_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto26_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto24_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J54,
                J93,
                J92,
                J79,
                J190,
                J25,
                J24,
                J27,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J25, __, J26, __, __, __, J27, J54, J79, __, __, __, __, __, __, __, __,
                    J92, __, __, __, __, J93, J190, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto23_ctx23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J54 => goto54_at1_ctx23_x(lex),
                Jump::J93 => goto93_at1_ctx23_x(lex),
                Jump::J92 => goto92_at1_ctx23_x(lex),
                Jump::J79 => goto79_at1_ctx23_x(lex),
                Jump::J190 => goto190_at1_ctx23_x(lex),
                Jump::J25 => {
                    lex.bump_unchecked(1usize);
                    goto25_ctx23_x(lex)
                }
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                Jump::J27 => goto27_at1_ctx23_x(lex),
                Jump::J26 => goto26_at1_ctx23_x(lex),
                Jump::__ => goto23_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto25_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                _ => lex.error(),
            }
        }
        #[inline]
        fn goto31_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto50_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto54_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J25,
                J31,
                J50,
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
                    __, __, __, __, __, J31, __, J31, __, J31, __, J31, __, J31, __, J31, __, J31,
                    __, J31, __, J31, __, J31, __, J50, __, J50, J25, __, __, __, __, __, __, __,
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
                Jump::J25 => {
                    lex.bump_unchecked(2usize);
                    goto25_x(lex)
                }
                Jump::J31 => goto31_at2(lex),
                Jump::J50 => goto50_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto1_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto192_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Ident));
        }
        #[inline]
        fn pattern3(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn goto193_ctx192_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern3, goto192_ctx192_x(lex));
        }
        #[inline]
        fn goto195_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Dot));
        }
        #[inline]
        fn goto7_ctx193_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordModule));
        }
        #[inline]
        fn goto209_ctx193_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto7_ctx193_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(1usize);
                    goto193_ctx192_x(lex)
                }
                _ => goto7_ctx193_x(lex),
            }
        }
        #[inline]
        fn goto208_ctx193_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"odule") => {
                    lex.bump_unchecked(5usize);
                    goto209_ctx193_x(lex)
                }
                _ => goto193_ctx192_x(lex),
            }
        }
        #[inline]
        fn goto26_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto88_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto27_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto92_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J26,
                J25,
                J88,
                J27,
                J50,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J25, __, __, __, __, __, __,
                    __, __, __, __, J50, J27, __, __, J88, __, J50, __, __, __, __, __, J26, __,
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
                Jump::J26 => goto26_at2(lex),
                Jump::J25 => {
                    lex.bump_unchecked(2usize);
                    goto25_x(lex)
                }
                Jump::J88 => goto88_at2(lex),
                Jump::J27 => goto27_at2(lex),
                Jump::J50 => goto50_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto71_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto64_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto79_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J26,
                J71,
                J25,
                J64,
                J27,
                J50,
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
                    __, __, __, __, __, __, __, __, __, J27, J50, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J25, J50, __, __, __, __, J64, __, J50, __, __, J71, __, __, J50, J26, __, __,
                    J71, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J26 => goto26_at2(lex),
                Jump::J71 => goto71_at2(lex),
                Jump::J25 => {
                    lex.bump_unchecked(2usize);
                    goto25_x(lex)
                }
                Jump::J64 => goto64_at2(lex),
                Jump::J27 => goto27_at2(lex),
                Jump::J50 => goto50_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto91_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto27_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto50_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto172_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J25,
                J27,
                J50,
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
                    __, __, __, __, __, __, __, __, __, J25, __, J27, __, J50, __, __, __, __, __,
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
                Jump::J25 => {
                    lex.bump_unchecked(3usize);
                    goto25_x(lex)
                }
                Jump::J27 => goto27_at3(lex),
                Jump::J50 => goto50_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto175_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto26_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto188_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J50,
                J27,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J27, __, __, __, __, __,
                    J26, __, __, __, __, __, __, __, J26, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J50, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J50 => goto50_at3(lex),
                Jump::J27 => goto27_at3(lex),
                Jump::J26 => goto26_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto99_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                146u8 => {
                    lex.bump_unchecked(3usize);
                    goto25_x(lex)
                }
                180u8 => goto26_at3(lex),
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto111_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto31_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto162_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J50,
                J111,
                J31,
                J25,
                J27,
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
                    __, __, __, __, __, __, __, __, __, J31, __, J26, J111, __, __, J50, __, __,
                    __, J26, __, __, __, __, __, J50, __, J50, __, __, __, __, __, J50, __, J27,
                    J26, __, __, __, __, __, __, J25, __, J50, __, __, __, __, __, __, __, __, __,
                    __, __, J50, __, __, __, J50, J25, __, __, __, __, __, __, J50, __, __, __, __,
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
                Jump::J50 => goto50_at3(lex),
                Jump::J111 => goto111_at3(lex),
                Jump::J31 => goto31_at3(lex),
                Jump::J25 => {
                    lex.bump_unchecked(3usize);
                    goto25_x(lex)
                }
                Jump::J27 => goto27_at3(lex),
                Jump::J26 => goto26_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto190_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J91,
                J172,
                J175,
                J188,
                J99,
                J162,
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
                    __, __, __, __, J99, J162, __, __, __, __, J172, __, __, __, __, __, __, J175,
                    J188, J91, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J91 => goto91_at2(lex),
                Jump::J172 => goto172_at2(lex),
                Jump::J175 => goto175_at2(lex),
                Jump::J188 => goto188_at2(lex),
                Jump::J99 => goto99_at2(lex),
                Jump::J162 => goto162_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto20_ctx20_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Comment));
        }
        #[inline]
        fn pattern4(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 2 > 0
        }
        #[inline]
        fn goto21_ctx20_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern4, goto20_ctx20_x(lex));
        }
        #[inline]
        fn goto203_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(b"/") => {
                    lex.bump_unchecked(2usize);
                    goto21_ctx20_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto26_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto196_ctx196_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Dash));
        }
        #[inline]
        fn goto213_ctx196_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto196_ctx196_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(1usize);
                    goto193_ctx192_x(lex)
                }
                _ => goto196_ctx196_x(lex),
            }
        }
        #[inline]
        fn goto8_ctx193_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordFennec));
        }
        #[inline]
        fn goto212_ctx193_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto8_ctx193_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(1usize);
                    goto193_ctx192_x(lex)
                }
                _ => goto8_ctx193_x(lex),
            }
        }
        #[inline]
        fn goto211_ctx193_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"ennec") => {
                    lex.bump_unchecked(5usize);
                    goto212_ctx193_x(lex)
                }
                _ => goto193_ctx192_x(lex),
            }
        }
        #[inline]
        fn goto93_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto204_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J54,
                J193,
                J93,
                J204,
                J79,
                J92,
                J25,
                J190,
                J27,
                J26,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, J193, __, __, J204, J204, J204, J204, J204, J204, J204,
                    J204, J204, J204, __, __, __, __, __, __, __, J193, J193, J193, J193, J193,
                    J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193,
                    J193, J193, J193, J193, J193, J193, J193, J193, __, __, __, __, __, __, J193,
                    J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193,
                    J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, J25, __, J26, __, __, __, J27,
                    J54, J79, __, __, __, __, __, __, __, __, J92, __, __, __, __, J93, J190, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto23_ctx23_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J54 => goto54_at1_ctx23_x(lex),
                Jump::J193 => {
                    lex.bump_unchecked(1usize);
                    goto193_ctx192_x(lex)
                }
                Jump::J93 => goto93_at1_ctx23_x(lex),
                Jump::J204 => {
                    lex.bump_unchecked(1usize);
                    goto204_ctx23_x(lex)
                }
                Jump::J79 => goto79_at1_ctx23_x(lex),
                Jump::J92 => goto92_at1_ctx23_x(lex),
                Jump::J25 => {
                    lex.bump_unchecked(1usize);
                    goto25_ctx23_x(lex)
                }
                Jump::J190 => goto190_at1_ctx23_x(lex),
                Jump::J27 => goto27_at1_ctx23_x(lex),
                Jump::J26 => goto26_at1_ctx23_x(lex),
                Jump::__ => goto23_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto3_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorSingleCarriageReturn));
        }
        #[inline]
        fn goto2_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto206_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(&[10u8]) => {
                    lex.bump_unchecked(1usize);
                    goto2_ctx3_x(lex)
                }
                _ => goto3_ctx3_x(lex),
            }
        }
        #[inline]
        fn goto17_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorStringUnterminated));
        }
        #[inline]
        fn goto9_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::String));
        }
        #[inline]
        fn pattern5(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 4 > 0
        }
        #[inline]
        fn goto13_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorStringWithBackslashes));
        }
        #[inline]
        fn goto201_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto17_ctx17_x(lex),
            };
            match byte {
                byte if pattern5(byte) => {
                    lex.bump_unchecked(1usize);
                    goto201_ctx17_x(lex)
                }
                b'"' => {
                    lex.bump_unchecked(1usize);
                    goto13_ctx17_x(lex)
                }
                _ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto200_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J9,
                J200,
                J201,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, __, J200, J200, __,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J9, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J201, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200, J200,
                    J200, J200, J200, J200, J200, J200, J200, J200,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto17_ctx17_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J9 => {
                    lex.bump_unchecked(1usize);
                    goto9_ctx17_x(lex)
                }
                Jump::J200 => {
                    lex.bump_unchecked(1usize);
                    goto200_ctx17_x(lex)
                }
                Jump::J201 => {
                    lex.bump_unchecked(1usize);
                    goto201_ctx17_x(lex)
                }
                Jump::__ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto27_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto24_ctx23_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto214<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J5,
                J197,
                J54,
                J1,
                J193,
                J195,
                J208,
                J92,
                J79,
                J190,
                J203,
                J26,
                J213,
                J211,
                J93,
                J204,
                J206,
                J25,
                J200,
                J27,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, J5, J1, __, __, J206, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J5, __, J200, __, __, __,
                    __, __, __, __, __, J197, __, J213, J195, J203, J204, J204, J204, J204, J204,
                    J204, J204, J204, J204, J204, __, __, __, __, __, __, __, J193, J193, J193,
                    J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193,
                    J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, __, __, __, __, __,
                    __, J193, J193, J193, J193, J193, J211, J193, J193, J193, J193, J193, J193,
                    J208, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193, J193,
                    J193, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J25, __, J26,
                    __, __, __, J27, J54, J79, __, __, __, __, __, __, __, __, J92, __, __, __, __,
                    J93, J190, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return _end(lex),
            };
            match LUT[byte as usize] {
                Jump::J5 => {
                    lex.bump_unchecked(1usize);
                    goto5_ctx4_x(lex)
                }
                Jump::J197 => {
                    lex.bump_unchecked(1usize);
                    goto197_x(lex)
                }
                Jump::J54 => goto54_at1(lex),
                Jump::J1 => {
                    lex.bump_unchecked(1usize);
                    goto1_x(lex)
                }
                Jump::J193 => {
                    lex.bump_unchecked(1usize);
                    goto193_ctx192_x(lex)
                }
                Jump::J195 => {
                    lex.bump_unchecked(1usize);
                    goto195_x(lex)
                }
                Jump::J208 => {
                    lex.bump_unchecked(1usize);
                    goto208_ctx193_x(lex)
                }
                Jump::J92 => goto92_at1(lex),
                Jump::J79 => goto79_at1(lex),
                Jump::J190 => goto190_at1(lex),
                Jump::J203 => goto203_at1(lex),
                Jump::J26 => goto26_at1(lex),
                Jump::J213 => {
                    lex.bump_unchecked(1usize);
                    goto213_ctx196_x(lex)
                }
                Jump::J211 => {
                    lex.bump_unchecked(1usize);
                    goto211_ctx193_x(lex)
                }
                Jump::J93 => goto93_at1(lex),
                Jump::J204 => {
                    lex.bump_unchecked(1usize);
                    goto204_ctx23_x(lex)
                }
                Jump::J206 => {
                    lex.bump_unchecked(1usize);
                    goto206_ctx3_x(lex)
                }
                Jump::J25 => {
                    lex.bump_unchecked(1usize);
                    goto25_x(lex)
                }
                Jump::J200 => {
                    lex.bump_unchecked(1usize);
                    goto200_ctx17_x(lex)
                }
                Jump::J27 => goto27_at1(lex),
                Jump::__ => _error(lex),
            }
        }
        goto214(lex)
    }
}
