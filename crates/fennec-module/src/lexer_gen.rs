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
    Version,
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
        fn goto23_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto23_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto23_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto23_ctx29_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto23_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto23_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn goto25_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto24_ctx23_x(lex));
        }
        #[inline]
        fn goto26_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto25_ctx24_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto24_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto26_at1_ctx23_x(lex),
                _ => goto23_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto27_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto24_ctx23_x(lex));
        }
        #[inline]
        fn goto28_at1_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto23_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto27_ctx24_x(lex)
                }
                _ => goto23_x(lex),
            }
        }
        #[inline]
        fn goto29_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto28_at1_ctx23_x(lex),
                _ => goto23_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto28_at1_ctx29_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto29_ctx23_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto27_ctx24_x(lex)
                }
                _ => goto29_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto29_ctx29_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto28_at1_ctx29_x(lex),
                _ => goto23_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto31_ctx30_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto30_ctx29_x(lex));
        }
        #[inline]
        fn goto32_at1_ctx29_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto29_ctx23_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto31_ctx30_x(lex)
                }
                _ => goto29_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto30_ctx29_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto32_at1_ctx29_x(lex),
                _ => goto29_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto33_ctx30_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto30_ctx29_x(lex));
        }
        #[inline]
        fn goto34_at1_ctx29_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto29_ctx23_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto33_ctx30_x(lex)
                }
                _ => goto29_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto35_ctx29_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto34_at1_ctx29_x(lex),
                _ => goto29_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto28_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto27_ctx24_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto29_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto28_at1_ctx35_x(lex),
                _ => goto23_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto34_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto33_ctx30_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto35_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto34_at1_ctx35_x(lex),
                _ => goto29_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto37_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto39_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto38_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto43_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto62_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto66_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J43,
                J62,
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
                    __, __, __, __, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43,
                    __, J43, __, J43, __, J43, __, J62, __, J62, J37, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at2_ctx35_x(lex),
                Jump::J43 => goto43_at2_ctx35_x(lex),
                Jump::J62 => goto62_at2_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto105_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
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
        fn goto100_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto39_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto38_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto104_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J100,
                J39,
                J62,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J37, __, __, __, __, __, __,
                    __, __, __, __, J62, J39, __, __, J100, __, J62, __, __, __, __, __, J38, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at2_ctx35_x(lex),
                Jump::J100 => goto100_at2_ctx35_x(lex),
                Jump::J39 => goto39_at2_ctx35_x(lex),
                Jump::J62 => goto62_at2_ctx35_x(lex),
                Jump::J38 => goto38_at2_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto76_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn pattern3(byte: u8) -> bool {
            match byte {
                128u8..=137u8 | 144u8..=153u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto83_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto91_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J76,
                J39,
                J62,
                J38,
                J83,
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
                    __, __, __, __, __, __, __, __, __, J39, J62, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J37, J62, __, __, __, __, J76, __, J62, __, __, J83, __, __, J62, J38, __, __,
                    J83, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at2_ctx35_x(lex),
                Jump::J76 => goto76_at2_ctx35_x(lex),
                Jump::J39 => goto39_at2_ctx35_x(lex),
                Jump::J62 => goto62_at2_ctx35_x(lex),
                Jump::J38 => goto38_at2_ctx35_x(lex),
                Jump::J83 => goto83_at2_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto37_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto38_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto111_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                146u8 => goto37_at3_ctx35_x(lex),
                180u8 => goto38_at3_ctx35_x(lex),
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto39_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto62_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto123_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto43_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto174_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J38,
                J62,
                J123,
                J43,
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
                    __, __, __, __, __, __, __, __, __, J43, __, J38, J123, __, __, J62, __, __,
                    __, J38, __, __, __, __, __, J62, __, J62, __, __, __, __, __, J62, __, J39,
                    J38, __, __, __, __, __, __, J37, __, J62, __, __, __, __, __, __, __, __, __,
                    __, __, J62, __, __, __, J62, J37, __, __, __, __, __, __, J62, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx35_x(lex),
                Jump::J39 => goto39_at3_ctx35_x(lex),
                Jump::J38 => goto38_at3_ctx35_x(lex),
                Jump::J62 => goto62_at3_ctx35_x(lex),
                Jump::J123 => goto123_at3_ctx35_x(lex),
                Jump::J43 => goto43_at3_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto103_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto184_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J62,
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
                    __, __, __, __, __, __, __, __, __, J37, __, J39, __, J62, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx35_x(lex),
                Jump::J39 => goto39_at3_ctx35_x(lex),
                Jump::J62 => goto62_at3_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto187_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto200_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J62,
                J39,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J39, __, __, __, __, __,
                    J38, __, __, __, __, __, __, __, J38, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J62, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J62 => goto62_at3_ctx35_x(lex),
                Jump::J39 => goto39_at3_ctx35_x(lex),
                Jump::J38 => goto38_at3_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto202_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J111,
                J174,
                J103,
                J184,
                J187,
                J200,
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
                    __, __, __, __, J111, J174, __, __, __, __, J184, __, __, __, __, __, __, J187,
                    J200, J103, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J111 => goto111_at2_ctx35_x(lex),
                Jump::J174 => goto174_at2_ctx35_x(lex),
                Jump::J103 => goto103_at2_ctx35_x(lex),
                Jump::J184 => goto184_at2_ctx35_x(lex),
                Jump::J187 => goto187_at2_ctx35_x(lex),
                Jump::J200 => goto200_at2_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto36_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J36,
                J39,
                J38,
                J66,
                J105,
                J104,
                J91,
                J202,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J36, J36, J36, J36, J36, J36, J36, J36, J36,
                    J36, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J37, __, J38, __, __, __, J39, J66, J91, __, __, __, __, __, __, __, __,
                    J104, __, __, __, __, J105, J202, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto35_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at1_ctx35_x(lex),
                Jump::J36 => {
                    lex.bump_unchecked(1usize);
                    goto36_ctx35_x(lex)
                }
                Jump::J39 => goto39_at1_ctx35_x(lex),
                Jump::J38 => goto38_at1_ctx35_x(lex),
                Jump::J66 => goto66_at1_ctx35_x(lex),
                Jump::J105 => goto105_at1_ctx35_x(lex),
                Jump::J104 => goto104_at1_ctx35_x(lex),
                Jump::J91 => goto91_at1_ctx35_x(lex),
                Jump::J202 => goto202_at1_ctx35_x(lex),
                Jump::__ => goto35_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto37_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto66_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J43,
                J62,
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
                    __, __, __, __, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43,
                    __, J43, __, J43, __, J43, __, J62, __, J62, J37, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx35_x(lex),
                Jump::J43 => goto43_at3_ctx35_x(lex),
                Jump::J62 => goto62_at3_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto105_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto100_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto104_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J100,
                J39,
                J62,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J37, __, __, __, __, __, __,
                    __, __, __, __, J62, J39, __, __, J100, __, J62, __, __, __, __, __, J38, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx35_x(lex),
                Jump::J100 => goto100_at3_ctx35_x(lex),
                Jump::J39 => goto39_at3_ctx35_x(lex),
                Jump::J62 => goto62_at3_ctx35_x(lex),
                Jump::J38 => goto38_at3_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto76_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto83_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto91_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J76,
                J39,
                J62,
                J38,
                J83,
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
                    __, __, __, __, __, __, __, __, __, J39, J62, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J37, J62, __, __, __, __, J76, __, J62, __, __, J83, __, __, J62, J38, __, __,
                    J83, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx35_x(lex),
                Jump::J76 => goto76_at3_ctx35_x(lex),
                Jump::J39 => goto39_at3_ctx35_x(lex),
                Jump::J62 => goto62_at3_ctx35_x(lex),
                Jump::J38 => goto38_at3_ctx35_x(lex),
                Jump::J83 => goto83_at3_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto37_at4_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto38_at4_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto111_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match byte {
                146u8 => goto37_at4_ctx35_x(lex),
                180u8 => goto38_at4_ctx35_x(lex),
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto39_at4_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto62_at4_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto123_at4_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto43_at4_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto174_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J38,
                J62,
                J123,
                J43,
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
                    __, __, __, __, __, __, __, __, __, J43, __, J38, J123, __, __, J62, __, __,
                    __, J38, __, __, __, __, __, J62, __, J62, __, __, __, __, __, J62, __, J39,
                    J38, __, __, __, __, __, __, J37, __, J62, __, __, __, __, __, __, __, __, __,
                    __, __, J62, __, __, __, J62, J37, __, __, __, __, __, __, J62, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at4_ctx35_x(lex),
                Jump::J39 => goto39_at4_ctx35_x(lex),
                Jump::J38 => goto38_at4_ctx35_x(lex),
                Jump::J62 => goto62_at4_ctx35_x(lex),
                Jump::J123 => goto123_at4_ctx35_x(lex),
                Jump::J43 => goto43_at4_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto103_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto184_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J62,
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
                    __, __, __, __, __, __, __, __, __, J37, __, J39, __, J62, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at4_ctx35_x(lex),
                Jump::J39 => goto39_at4_ctx35_x(lex),
                Jump::J62 => goto62_at4_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto187_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto200_at3_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J62,
                J39,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J39, __, __, __, __, __,
                    J38, __, __, __, __, __, __, __, J38, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J62, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J62 => goto62_at4_ctx35_x(lex),
                Jump::J39 => goto39_at4_ctx35_x(lex),
                Jump::J38 => goto38_at4_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto202_at2_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J111,
                J174,
                J103,
                J184,
                J187,
                J200,
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
                    __, __, __, __, J111, J174, __, __, __, __, J184, __, __, __, __, __, __, J187,
                    J200, J103, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J111 => goto111_at3_ctx35_x(lex),
                Jump::J174 => goto174_at3_ctx35_x(lex),
                Jump::J103 => goto103_at3_ctx35_x(lex),
                Jump::J184 => goto184_at3_ctx35_x(lex),
                Jump::J187 => goto187_at3_ctx35_x(lex),
                Jump::J200 => goto200_at3_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto203_at1_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J36,
                J39,
                J38,
                J66,
                J105,
                J104,
                J91,
                J202,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J36, J36, J36, J36, J36, J36, J36, J36, J36,
                    J36, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J37, __, J38, __, __, __, J39, J66, J91, __, __, __, __, __, __, __, __,
                    J104, __, __, __, __, J105, J202, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto35_ctx29_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at2_ctx35_x(lex),
                Jump::J36 => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx35_x(lex)
                }
                Jump::J39 => goto39_at2_ctx35_x(lex),
                Jump::J38 => goto38_at2_ctx35_x(lex),
                Jump::J66 => goto66_at2_ctx35_x(lex),
                Jump::J105 => goto105_at2_ctx35_x(lex),
                Jump::J104 => goto104_at2_ctx35_x(lex),
                Jump::J91 => goto91_at2_ctx35_x(lex),
                Jump::J202 => goto202_at2_ctx35_x(lex),
                Jump::__ => goto35_ctx29_x(lex),
            }
        }
        #[inline]
        fn goto204_ctx35_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto203_at1_ctx35_x(lex),
                _ => goto35_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto28_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto27_ctx24_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto29_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto28_at1_ctx204_x(lex),
                _ => goto23_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto34_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto33_ctx30_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto35_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto34_at1_ctx204_x(lex),
                _ => goto29_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto37_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto39_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto38_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto37_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto43_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto62_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto66_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J43,
                J62,
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
                    __, __, __, __, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43,
                    __, J43, __, J43, __, J43, __, J62, __, J62, J37, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx204_x(lex),
                Jump::J43 => goto43_at3_ctx204_x(lex),
                Jump::J62 => goto62_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto105_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto100_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto39_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto38_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto104_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J100,
                J39,
                J62,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J37, __, __, __, __, __, __,
                    __, __, __, __, J62, J39, __, __, J100, __, J62, __, __, __, __, __, J38, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx204_x(lex),
                Jump::J100 => goto100_at3_ctx204_x(lex),
                Jump::J39 => goto39_at3_ctx204_x(lex),
                Jump::J62 => goto62_at3_ctx204_x(lex),
                Jump::J38 => goto38_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto76_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto83_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto91_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J76,
                J39,
                J62,
                J38,
                J83,
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
                    __, __, __, __, __, __, __, __, __, J39, J62, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J37, J62, __, __, __, __, J76, __, J62, __, __, J83, __, __, J62, J38, __, __,
                    J83, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx204_x(lex),
                Jump::J76 => goto76_at3_ctx204_x(lex),
                Jump::J39 => goto39_at3_ctx204_x(lex),
                Jump::J62 => goto62_at3_ctx204_x(lex),
                Jump::J38 => goto38_at3_ctx204_x(lex),
                Jump::J83 => goto83_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto37_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto38_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto111_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                146u8 => goto37_at4_ctx204_x(lex),
                180u8 => goto38_at4_ctx204_x(lex),
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto39_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto62_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto123_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto43_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto174_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J38,
                J62,
                J123,
                J43,
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
                    __, __, __, __, __, __, __, __, __, J43, __, J38, J123, __, __, J62, __, __,
                    __, J38, __, __, __, __, __, J62, __, J62, __, __, __, __, __, J62, __, J39,
                    J38, __, __, __, __, __, __, J37, __, J62, __, __, __, __, __, __, __, __, __,
                    __, __, J62, __, __, __, J62, J37, __, __, __, __, __, __, J62, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at4_ctx204_x(lex),
                Jump::J39 => goto39_at4_ctx204_x(lex),
                Jump::J38 => goto38_at4_ctx204_x(lex),
                Jump::J62 => goto62_at4_ctx204_x(lex),
                Jump::J123 => goto123_at4_ctx204_x(lex),
                Jump::J43 => goto43_at4_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto103_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto184_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J62,
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
                    __, __, __, __, __, __, __, __, __, J37, __, J39, __, J62, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at4_ctx204_x(lex),
                Jump::J39 => goto39_at4_ctx204_x(lex),
                Jump::J62 => goto62_at4_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto187_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto200_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J62,
                J39,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J39, __, __, __, __, __,
                    J38, __, __, __, __, __, __, __, J38, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J62, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J62 => goto62_at4_ctx204_x(lex),
                Jump::J39 => goto39_at4_ctx204_x(lex),
                Jump::J38 => goto38_at4_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto202_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J111,
                J174,
                J103,
                J184,
                J187,
                J200,
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
                    __, __, __, __, J111, J174, __, __, __, __, J184, __, __, __, __, __, __, J187,
                    J200, J103, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J111 => goto111_at3_ctx204_x(lex),
                Jump::J174 => goto174_at3_ctx204_x(lex),
                Jump::J103 => goto103_at3_ctx204_x(lex),
                Jump::J184 => goto184_at3_ctx204_x(lex),
                Jump::J187 => goto187_at3_ctx204_x(lex),
                Jump::J200 => goto200_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto203_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J36,
                J39,
                J38,
                J66,
                J105,
                J104,
                J91,
                J202,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J36, J36, J36, J36, J36, J36, J36, J36, J36,
                    J36, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J37, __, J38, __, __, __, J39, J66, J91, __, __, __, __, __, __, __, __,
                    J104, __, __, __, __, J105, J202, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at2_ctx204_x(lex),
                Jump::J36 => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx35_x(lex)
                }
                Jump::J39 => goto39_at2_ctx204_x(lex),
                Jump::J38 => goto38_at2_ctx204_x(lex),
                Jump::J66 => goto66_at2_ctx204_x(lex),
                Jump::J105 => goto105_at2_ctx204_x(lex),
                Jump::J104 => goto104_at2_ctx204_x(lex),
                Jump::J91 => goto91_at2_ctx204_x(lex),
                Jump::J202 => goto202_at2_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto204_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto203_at1_ctx204_x(lex),
                _ => goto35_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto208_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto207_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto231_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto369_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J208,
                J207,
                J231,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J208, __, __, __, __, __,
                    J207, __, __, __, __, __, __, __, J207, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J231, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J208 => goto208_at3_ctx204_x(lex),
                Jump::J207 => goto207_at3_ctx204_x(lex),
                Jump::J231 => goto231_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto206_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto280_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                180u8 => goto207_at3_ctx204_x(lex),
                146u8 => goto206_at3_ctx204_x(lex),
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto272_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto212_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto292_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto343_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J212,
                J207,
                J231,
                J206,
                J208,
                J292,
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
                    __, __, __, __, __, __, __, __, __, J212, __, J207, J292, __, __, J231, __, __,
                    __, J207, __, __, __, __, __, J231, __, J231, __, __, __, __, __, J231, __,
                    J208, J207, __, __, __, __, __, __, J206, __, J231, __, __, __, __, __, __, __,
                    __, __, __, __, J231, __, __, __, J231, J206, __, __, __, __, __, __, J231, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J212 => goto212_at3_ctx204_x(lex),
                Jump::J207 => goto207_at3_ctx204_x(lex),
                Jump::J231 => goto231_at3_ctx204_x(lex),
                Jump::J206 => goto206_at3_ctx204_x(lex),
                Jump::J208 => goto208_at3_ctx204_x(lex),
                Jump::J292 => goto292_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto356_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto353_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J208,
                J231,
                J206,
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
                    __, __, __, __, __, __, __, __, __, J206, __, J208, __, J231, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J208 => goto208_at3_ctx204_x(lex),
                Jump::J231 => goto231_at3_ctx204_x(lex),
                Jump::J206 => goto206_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto371_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J369,
                J280,
                J272,
                J343,
                J356,
                J353,
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
                    __, __, __, __, J280, J343, __, __, __, __, J353, __, __, __, __, __, __, J356,
                    J369, J272, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J369 => goto369_at2_ctx204_x(lex),
                Jump::J280 => goto280_at2_ctx204_x(lex),
                Jump::J272 => goto272_at2_ctx204_x(lex),
                Jump::J343 => goto343_at2_ctx204_x(lex),
                Jump::J356 => goto356_at2_ctx204_x(lex),
                Jump::J353 => goto353_at2_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto208_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto245_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto252_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto231_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto206_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto207_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto208_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto260_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J245,
                J252,
                J231,
                J206,
                J207,
                J208,
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
                    __, __, __, __, __, __, __, __, __, J208, J231, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J206, J231, __, __, __, __, J245, __, J231, __, __, J252, __, __, J231, J207,
                    __, __, J252, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J245 => goto245_at2_ctx204_x(lex),
                Jump::J252 => goto252_at2_ctx204_x(lex),
                Jump::J231 => goto231_at2_ctx204_x(lex),
                Jump::J206 => goto206_at2_ctx204_x(lex),
                Jump::J207 => goto207_at2_ctx204_x(lex),
                Jump::J208 => goto208_at2_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto207_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto206_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(2usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto274_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto212_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto235_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J212,
                J231,
                J206,
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
                    __, __, __, __, __, J212, __, J212, __, J212, __, J212, __, J212, __, J212, __,
                    J212, __, J212, __, J212, __, J212, __, J231, __, J231, J206, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J212 => goto212_at2_ctx204_x(lex),
                Jump::J231 => goto231_at2_ctx204_x(lex),
                Jump::J206 => goto206_at2_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto269_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto273_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J231,
                J206,
                J269,
                J208,
                J207,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J206, __, __, __, __, __, __,
                    __, __, __, __, J231, J208, __, __, J269, __, J231, __, __, __, __, __, J207,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J231 => goto231_at2_ctx204_x(lex),
                Jump::J206 => goto206_at2_ctx204_x(lex),
                Jump::J269 => goto269_at2_ctx204_x(lex),
                Jump::J208 => goto208_at2_ctx204_x(lex),
                Jump::J207 => goto207_at2_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto205_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J371,
                J208,
                J260,
                J205,
                J207,
                J206,
                J274,
                J235,
                J273,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J205, J205, J205, J205, J205, J205, J205, J205,
                    J205, J205, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J206, __, J207, __, __, __, J208, J235, J260, __, __, __, __, __, __,
                    __, __, J273, __, __, __, __, J274, J371, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto204_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J371 => goto371_at1_ctx204_x(lex),
                Jump::J208 => goto208_at1_ctx204_x(lex),
                Jump::J260 => goto260_at1_ctx204_x(lex),
                Jump::J205 => {
                    lex.bump_unchecked(1usize);
                    goto205_ctx204_x(lex)
                }
                Jump::J207 => goto207_at1_ctx204_x(lex),
                Jump::J206 => goto206_at1_ctx204_x(lex),
                Jump::J274 => goto274_at1_ctx204_x(lex),
                Jump::J235 => goto235_at1_ctx204_x(lex),
                Jump::J273 => goto273_at1_ctx204_x(lex),
                Jump::__ => goto204_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto208_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto207_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto231_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto369_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J208,
                J207,
                J231,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J208, __, __, __, __, __,
                    J207, __, __, __, __, __, __, __, J207, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J231, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J208 => goto208_at4_ctx204_x(lex),
                Jump::J207 => goto207_at4_ctx204_x(lex),
                Jump::J231 => goto231_at4_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto206_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto280_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                180u8 => goto207_at4_ctx204_x(lex),
                146u8 => goto206_at4_ctx204_x(lex),
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto272_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto212_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto292_at4_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto343_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J212,
                J207,
                J231,
                J206,
                J208,
                J292,
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
                    __, __, __, __, __, __, __, __, __, J212, __, J207, J292, __, __, J231, __, __,
                    __, J207, __, __, __, __, __, J231, __, J231, __, __, __, __, __, J231, __,
                    J208, J207, __, __, __, __, __, __, J206, __, J231, __, __, __, __, __, __, __,
                    __, __, __, __, J231, __, __, __, J231, J206, __, __, __, __, __, __, J231, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J212 => goto212_at4_ctx204_x(lex),
                Jump::J207 => goto207_at4_ctx204_x(lex),
                Jump::J231 => goto231_at4_ctx204_x(lex),
                Jump::J206 => goto206_at4_ctx204_x(lex),
                Jump::J208 => goto208_at4_ctx204_x(lex),
                Jump::J292 => goto292_at4_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto356_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto353_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J208,
                J231,
                J206,
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
                    __, __, __, __, __, __, __, __, __, J206, __, J208, __, J231, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J208 => goto208_at4_ctx204_x(lex),
                Jump::J231 => goto231_at4_ctx204_x(lex),
                Jump::J206 => goto206_at4_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto371_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J369,
                J280,
                J272,
                J343,
                J356,
                J353,
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
                    __, __, __, __, J280, J343, __, __, __, __, J353, __, __, __, __, __, __, J356,
                    J369, J272, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J369 => goto369_at3_ctx204_x(lex),
                Jump::J280 => goto280_at3_ctx204_x(lex),
                Jump::J272 => goto272_at3_ctx204_x(lex),
                Jump::J343 => goto343_at3_ctx204_x(lex),
                Jump::J356 => goto356_at3_ctx204_x(lex),
                Jump::J353 => goto353_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto245_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto252_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto260_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J245,
                J252,
                J231,
                J206,
                J207,
                J208,
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
                    __, __, __, __, __, __, __, __, __, J208, J231, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J206, J231, __, __, __, __, J245, __, J231, __, __, J252, __, __, J231, J207,
                    __, __, J252, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J245 => goto245_at3_ctx204_x(lex),
                Jump::J252 => goto252_at3_ctx204_x(lex),
                Jump::J231 => goto231_at3_ctx204_x(lex),
                Jump::J206 => goto206_at3_ctx204_x(lex),
                Jump::J207 => goto207_at3_ctx204_x(lex),
                Jump::J208 => goto208_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto274_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto235_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J212,
                J231,
                J206,
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
                    __, __, __, __, __, J212, __, J212, __, J212, __, J212, __, J212, __, J212, __,
                    J212, __, J212, __, J212, __, J212, __, J231, __, J231, J206, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J212 => goto212_at3_ctx204_x(lex),
                Jump::J231 => goto231_at3_ctx204_x(lex),
                Jump::J206 => goto206_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto269_at3_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto273_at2_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J231,
                J206,
                J269,
                J208,
                J207,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J206, __, __, __, __, __, __,
                    __, __, __, __, J231, J208, __, __, J269, __, J231, __, __, __, __, __, J207,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J231 => goto231_at3_ctx204_x(lex),
                Jump::J206 => goto206_at3_ctx204_x(lex),
                Jump::J269 => goto269_at3_ctx204_x(lex),
                Jump::J208 => goto208_at3_ctx204_x(lex),
                Jump::J207 => goto207_at3_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto372_at1_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J371,
                J208,
                J260,
                J205,
                J207,
                J206,
                J274,
                J235,
                J273,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J205, J205, J205, J205, J205, J205, J205, J205,
                    J205, J205, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J206, __, J207, __, __, __, J208, J235, J260, __, __, __, __, __, __,
                    __, __, J273, __, __, __, __, J274, J371, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto204_ctx35_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J371 => goto371_at2_ctx204_x(lex),
                Jump::J208 => goto208_at2_ctx204_x(lex),
                Jump::J260 => goto260_at2_ctx204_x(lex),
                Jump::J205 => {
                    lex.bump_unchecked(2usize);
                    goto205_ctx204_x(lex)
                }
                Jump::J207 => goto207_at2_ctx204_x(lex),
                Jump::J206 => goto206_at2_ctx204_x(lex),
                Jump::J274 => goto274_at2_ctx204_x(lex),
                Jump::J235 => goto235_at2_ctx204_x(lex),
                Jump::J273 => goto273_at2_ctx204_x(lex),
                Jump::__ => goto204_ctx35_x(lex),
            }
        }
        #[inline]
        fn goto373_ctx204_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto372_at1_ctx204_x(lex),
                _ => goto204_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto28_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto27_ctx24_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto29_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto28_at1_ctx373_x(lex),
                _ => goto23_ctx373_x(lex),
            }
        }
        #[inline]
        fn goto34_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto33_ctx30_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto35_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto34_at1_ctx373_x(lex),
                _ => goto29_ctx373_x(lex),
            }
        }
        #[inline]
        fn goto37_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto39_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto38_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto37_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto43_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto62_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto66_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J43,
                J62,
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
                    __, __, __, __, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43, __, J43,
                    __, J43, __, J43, __, J43, __, J62, __, J62, J37, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx373_x(lex),
                Jump::J43 => goto43_at3_ctx373_x(lex),
                Jump::J62 => goto62_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto105_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto100_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto39_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto38_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto104_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J100,
                J39,
                J62,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J37, __, __, __, __, __, __,
                    __, __, __, __, J62, J39, __, __, J100, __, J62, __, __, __, __, __, J38, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx373_x(lex),
                Jump::J100 => goto100_at3_ctx373_x(lex),
                Jump::J39 => goto39_at3_ctx373_x(lex),
                Jump::J62 => goto62_at3_ctx373_x(lex),
                Jump::J38 => goto38_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto76_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto83_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto91_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J76,
                J39,
                J62,
                J38,
                J83,
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
                    __, __, __, __, __, __, __, __, __, J39, J62, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J37, J62, __, __, __, __, J76, __, J62, __, __, J83, __, __, J62, J38, __, __,
                    J83, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at3_ctx373_x(lex),
                Jump::J76 => goto76_at3_ctx373_x(lex),
                Jump::J39 => goto39_at3_ctx373_x(lex),
                Jump::J62 => goto62_at3_ctx373_x(lex),
                Jump::J38 => goto38_at3_ctx373_x(lex),
                Jump::J83 => goto83_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto37_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto38_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto111_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                146u8 => goto37_at4_ctx373_x(lex),
                180u8 => goto38_at4_ctx373_x(lex),
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto39_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto62_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto123_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto43_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto174_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J38,
                J62,
                J123,
                J43,
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
                    __, __, __, __, __, __, __, __, __, J43, __, J38, J123, __, __, J62, __, __,
                    __, J38, __, __, __, __, __, J62, __, J62, __, __, __, __, __, J62, __, J39,
                    J38, __, __, __, __, __, __, J37, __, J62, __, __, __, __, __, __, __, __, __,
                    __, __, J62, __, __, __, J62, J37, __, __, __, __, __, __, J62, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at4_ctx373_x(lex),
                Jump::J39 => goto39_at4_ctx373_x(lex),
                Jump::J38 => goto38_at4_ctx373_x(lex),
                Jump::J62 => goto62_at4_ctx373_x(lex),
                Jump::J123 => goto123_at4_ctx373_x(lex),
                Jump::J43 => goto43_at4_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto103_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto184_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J39,
                J62,
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
                    __, __, __, __, __, __, __, __, __, J37, __, J39, __, J62, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at4_ctx373_x(lex),
                Jump::J39 => goto39_at4_ctx373_x(lex),
                Jump::J62 => goto62_at4_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto187_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto36_ctx35_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto200_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J62,
                J39,
                J38,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J39, __, __, __, __, __,
                    J38, __, __, __, __, __, __, __, J38, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J62, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J62 => goto62_at4_ctx373_x(lex),
                Jump::J39 => goto39_at4_ctx373_x(lex),
                Jump::J38 => goto38_at4_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto202_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J111,
                J174,
                J103,
                J184,
                J187,
                J200,
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
                    __, __, __, __, J111, J174, __, __, __, __, J184, __, __, __, __, __, __, J187,
                    J200, J103, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J111 => goto111_at3_ctx373_x(lex),
                Jump::J174 => goto174_at3_ctx373_x(lex),
                Jump::J103 => goto103_at3_ctx373_x(lex),
                Jump::J184 => goto184_at3_ctx373_x(lex),
                Jump::J187 => goto187_at3_ctx373_x(lex),
                Jump::J200 => goto200_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto203_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J36,
                J39,
                J38,
                J66,
                J105,
                J104,
                J91,
                J202,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J36, J36, J36, J36, J36, J36, J36, J36, J36,
                    J36, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J37, __, J38, __, __, __, J39, J66, J91, __, __, __, __, __, __, __, __,
                    J104, __, __, __, __, J105, J202, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at2_ctx373_x(lex),
                Jump::J36 => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx35_x(lex)
                }
                Jump::J39 => goto39_at2_ctx373_x(lex),
                Jump::J38 => goto38_at2_ctx373_x(lex),
                Jump::J66 => goto66_at2_ctx373_x(lex),
                Jump::J105 => goto105_at2_ctx373_x(lex),
                Jump::J104 => goto104_at2_ctx373_x(lex),
                Jump::J91 => goto91_at2_ctx373_x(lex),
                Jump::J202 => goto202_at2_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto204_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto203_at1_ctx373_x(lex),
                _ => goto35_ctx373_x(lex),
            }
        }
        #[inline]
        fn goto208_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto207_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto231_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto369_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J208,
                J207,
                J231,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J208, __, __, __, __, __,
                    J207, __, __, __, __, __, __, __, J207, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J231, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J208 => goto208_at4_ctx373_x(lex),
                Jump::J207 => goto207_at4_ctx373_x(lex),
                Jump::J231 => goto231_at4_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto206_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto280_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                180u8 => goto207_at4_ctx373_x(lex),
                146u8 => goto206_at4_ctx373_x(lex),
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto272_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto212_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto292_at4_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto343_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J212,
                J207,
                J231,
                J206,
                J208,
                J292,
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
                    __, __, __, __, __, __, __, __, __, J212, __, J207, J292, __, __, J231, __, __,
                    __, J207, __, __, __, __, __, J231, __, J231, __, __, __, __, __, J231, __,
                    J208, J207, __, __, __, __, __, __, J206, __, J231, __, __, __, __, __, __, __,
                    __, __, __, __, J231, __, __, __, J231, J206, __, __, __, __, __, __, J231, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J212 => goto212_at4_ctx373_x(lex),
                Jump::J207 => goto207_at4_ctx373_x(lex),
                Jump::J231 => goto231_at4_ctx373_x(lex),
                Jump::J206 => goto206_at4_ctx373_x(lex),
                Jump::J208 => goto208_at4_ctx373_x(lex),
                Jump::J292 => goto292_at4_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto356_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto353_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J208,
                J231,
                J206,
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
                    __, __, __, __, __, __, __, __, __, J206, __, J208, __, J231, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J208 => goto208_at4_ctx373_x(lex),
                Jump::J231 => goto231_at4_ctx373_x(lex),
                Jump::J206 => goto206_at4_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto371_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J369,
                J280,
                J272,
                J343,
                J356,
                J353,
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
                    __, __, __, __, J280, J343, __, __, __, __, J353, __, __, __, __, __, __, J356,
                    J369, J272, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J369 => goto369_at3_ctx373_x(lex),
                Jump::J280 => goto280_at3_ctx373_x(lex),
                Jump::J272 => goto272_at3_ctx373_x(lex),
                Jump::J343 => goto343_at3_ctx373_x(lex),
                Jump::J356 => goto356_at3_ctx373_x(lex),
                Jump::J353 => goto353_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto208_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto245_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto252_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto231_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto206_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto207_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto208_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto260_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J245,
                J252,
                J231,
                J206,
                J207,
                J208,
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
                    __, __, __, __, __, __, __, __, __, J208, J231, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J206, J231, __, __, __, __, J245, __, J231, __, __, J252, __, __, J231, J207,
                    __, __, J252, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J245 => goto245_at3_ctx373_x(lex),
                Jump::J252 => goto252_at3_ctx373_x(lex),
                Jump::J231 => goto231_at3_ctx373_x(lex),
                Jump::J206 => goto206_at3_ctx373_x(lex),
                Jump::J207 => goto207_at3_ctx373_x(lex),
                Jump::J208 => goto208_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto207_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto206_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto274_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto212_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto235_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J212,
                J231,
                J206,
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
                    __, __, __, __, __, J212, __, J212, __, J212, __, J212, __, J212, __, J212, __,
                    J212, __, J212, __, J212, __, J212, __, J231, __, J231, J206, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J212 => goto212_at3_ctx373_x(lex),
                Jump::J231 => goto231_at3_ctx373_x(lex),
                Jump::J206 => goto206_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto269_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto205_ctx204_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto273_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J231,
                J206,
                J269,
                J208,
                J207,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J206, __, __, __, __, __, __,
                    __, __, __, __, J231, J208, __, __, J269, __, J231, __, __, __, __, __, J207,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J231 => goto231_at3_ctx373_x(lex),
                Jump::J206 => goto206_at3_ctx373_x(lex),
                Jump::J269 => goto269_at3_ctx373_x(lex),
                Jump::J208 => goto208_at3_ctx373_x(lex),
                Jump::J207 => goto207_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto372_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J371,
                J208,
                J260,
                J205,
                J207,
                J206,
                J274,
                J235,
                J273,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J205, J205, J205, J205, J205, J205, J205, J205,
                    J205, J205, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J206, __, J207, __, __, __, J208, J235, J260, __, __, __, __, __, __,
                    __, __, J273, __, __, __, __, J274, J371, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J371 => goto371_at2_ctx373_x(lex),
                Jump::J208 => goto208_at2_ctx373_x(lex),
                Jump::J260 => goto260_at2_ctx373_x(lex),
                Jump::J205 => {
                    lex.bump_unchecked(2usize);
                    goto205_ctx204_x(lex)
                }
                Jump::J207 => goto207_at2_ctx373_x(lex),
                Jump::J206 => goto206_at2_ctx373_x(lex),
                Jump::J274 => goto274_at2_ctx373_x(lex),
                Jump::J235 => goto235_at2_ctx373_x(lex),
                Jump::J273 => goto273_at2_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto373_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto372_at1_ctx373_x(lex),
                _ => goto204_ctx373_x(lex),
            }
        }
        #[inline]
        fn goto376_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto414_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto400_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto421_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto376_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto375_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto377_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto429_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J414,
                J400,
                J421,
                J376,
                J375,
                J377,
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
                    __, __, __, __, __, __, __, __, __, J377, J400, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J375, J400, __, __, __, __, J414, __, J400, __, __, J421, __, __, J400, J376,
                    __, __, J421, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J414 => goto414_at2_ctx373_x(lex),
                Jump::J400 => goto400_at2_ctx373_x(lex),
                Jump::J421 => goto421_at2_ctx373_x(lex),
                Jump::J376 => goto376_at2_ctx373_x(lex),
                Jump::J375 => goto375_at2_ctx373_x(lex),
                Jump::J377 => goto377_at2_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto438_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto442_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J438,
                J400,
                J376,
                J375,
                J377,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J375, __, __, __, __, __, __,
                    __, __, __, __, J400, J377, __, __, J438, __, J400, __, __, __, __, __, J376,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J438 => goto438_at2_ctx373_x(lex),
                Jump::J400 => goto400_at2_ctx373_x(lex),
                Jump::J376 => goto376_at2_ctx373_x(lex),
                Jump::J375 => goto375_at2_ctx373_x(lex),
                Jump::J377 => goto377_at2_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto375_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto376_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto449_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match byte {
                146u8 => goto375_at3_ctx373_x(lex),
                180u8 => goto376_at3_ctx373_x(lex),
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto400_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto377_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto538_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J400,
                J376,
                J377,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J377, __, __, __, __, __,
                    J376, __, __, __, __, __, __, __, J376, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J400, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J400 => goto400_at3_ctx373_x(lex),
                Jump::J376 => goto376_at3_ctx373_x(lex),
                Jump::J377 => goto377_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto441_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto522_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J375,
                J400,
                J377,
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
                    __, __, __, __, __, __, __, __, __, J375, __, J377, __, J400, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J375 => goto375_at3_ctx373_x(lex),
                Jump::J400 => goto400_at3_ctx373_x(lex),
                Jump::J377 => goto377_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto525_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto461_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto381_at3_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto512_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J376,
                J461,
                J381,
                J400,
                J375,
                J377,
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
                    __, __, __, __, __, __, __, __, __, J381, __, J376, J461, __, __, J400, __, __,
                    __, J376, __, __, __, __, __, J400, __, J400, __, __, __, __, __, J400, __,
                    J377, J376, __, __, __, __, __, __, J375, __, J400, __, __, __, __, __, __, __,
                    __, __, __, __, J400, __, __, __, J400, J375, __, __, __, __, __, __, J400, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J376 => goto376_at3_ctx373_x(lex),
                Jump::J461 => goto461_at3_ctx373_x(lex),
                Jump::J381 => goto381_at3_ctx373_x(lex),
                Jump::J400 => goto400_at3_ctx373_x(lex),
                Jump::J375 => goto375_at3_ctx373_x(lex),
                Jump::J377 => goto377_at3_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto540_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J449,
                J538,
                J441,
                J522,
                J525,
                J512,
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
                    __, __, __, __, J449, J512, __, __, __, __, J522, __, __, __, __, __, __, J525,
                    J538, J441, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J449 => goto449_at2_ctx373_x(lex),
                Jump::J538 => goto538_at2_ctx373_x(lex),
                Jump::J441 => goto441_at2_ctx373_x(lex),
                Jump::J522 => goto522_at2_ctx373_x(lex),
                Jump::J525 => goto525_at2_ctx373_x(lex),
                Jump::J512 => goto512_at2_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto381_at2_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto404_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J375,
                J400,
                J381,
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
                    __, __, __, __, __, J381, __, J381, __, J381, __, J381, __, J381, __, J381, __,
                    J381, __, J381, __, J381, __, J381, __, J400, __, J400, J375, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto373_ctx204_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J375 => goto375_at2_ctx373_x(lex),
                Jump::J400 => goto400_at2_ctx373_x(lex),
                Jump::J381 => goto381_at2_ctx373_x(lex),
                Jump::__ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto377_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto443_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto375_at1_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(2usize);
                    goto374_ctx373_x(lex)
                }
                _ => goto373_ctx204_x(lex),
            }
        }
        #[inline]
        fn goto374_ctx373_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J376,
                J429,
                J442,
                J540,
                J404,
                J377,
                J374,
                J443,
                J375,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J374, J374, J374, J374, J374, J374, J374, J374,
                    J374, J374, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J375, __, J376, __, __, __, J377, J404, J429, __, __, __, __, __, __,
                    __, __, J442, __, __, __, __, J443, J540, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto373_ctx373_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J376 => goto376_at1_ctx373_x(lex),
                Jump::J429 => goto429_at1_ctx373_x(lex),
                Jump::J442 => goto442_at1_ctx373_x(lex),
                Jump::J540 => goto540_at1_ctx373_x(lex),
                Jump::J404 => goto404_at1_ctx373_x(lex),
                Jump::J377 => goto377_at1_ctx373_x(lex),
                Jump::J374 => {
                    lex.bump_unchecked(1usize);
                    goto374_ctx373_x(lex)
                }
                Jump::J443 => goto443_at1_ctx373_x(lex),
                Jump::J375 => goto375_at1_ctx373_x(lex),
                Jump::__ => goto373_ctx373_x(lex),
            }
        }
        #[inline]
        fn goto376_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto1_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto438_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto400_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto376_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto375_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto377_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto442_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J438,
                J400,
                J376,
                J375,
                J377,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J375, __, __, __, __, __, __,
                    __, __, __, __, J400, J377, __, __, J438, __, J400, __, __, __, __, __, J376,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J438 => goto438_at2(lex),
                Jump::J400 => goto400_at2(lex),
                Jump::J376 => goto376_at2(lex),
                Jump::J375 => goto375_at2(lex),
                Jump::J377 => goto377_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto377_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto7_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordModule));
        }
        #[inline]
        fn goto550_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 5usize]>(1usize) {
                Some(b"odule") => {
                    lex.bump_unchecked(6usize);
                    goto7_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto443_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto8_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordFennec));
        }
        #[inline]
        fn goto551_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 5usize]>(1usize) {
                Some(b"ennec") => {
                    lex.bump_unchecked(6usize);
                    goto8_x(lex)
                }
                _ => _error(lex),
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
        fn goto547_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(b"/") => {
                    lex.bump_unchecked(2usize);
                    goto21_ctx20_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto414_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto421_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto429_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J414,
                J400,
                J421,
                J376,
                J375,
                J377,
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
                    __, __, __, __, __, __, __, __, __, J377, J400, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J375, J400, __, __, __, __, J414, __, J400, __, __, J421, __, __, J400, J376,
                    __, __, J421, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J414 => goto414_at2(lex),
                Jump::J400 => goto400_at2(lex),
                Jump::J421 => goto421_at2(lex),
                Jump::J376 => goto376_at2(lex),
                Jump::J375 => goto375_at2(lex),
                Jump::J377 => goto377_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto375_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto376_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto449_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                146u8 => goto375_at3(lex),
                180u8 => goto376_at3(lex),
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto400_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto377_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto538_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J400,
                J376,
                J377,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J377, __, __, __, __, __,
                    J376, __, __, __, __, __, __, __, J376, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J400, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J400 => goto400_at3(lex),
                Jump::J376 => goto376_at3(lex),
                Jump::J377 => goto377_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto441_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto522_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J375,
                J400,
                J377,
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
                    __, __, __, __, __, __, __, __, __, J375, __, J377, __, J400, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J375 => goto375_at3(lex),
                Jump::J400 => goto400_at3(lex),
                Jump::J377 => goto377_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto525_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto461_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto381_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto512_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J376,
                J461,
                J381,
                J400,
                J375,
                J377,
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
                    __, __, __, __, __, __, __, __, __, J381, __, J376, J461, __, __, J400, __, __,
                    __, J376, __, __, __, __, __, J400, __, J400, __, __, __, __, __, J400, __,
                    J377, J376, __, __, __, __, __, __, J375, __, J400, __, __, __, __, __, __, __,
                    __, __, __, __, J400, __, __, __, J400, J375, __, __, __, __, __, __, J400, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J376 => goto376_at3(lex),
                Jump::J461 => goto461_at3(lex),
                Jump::J381 => goto381_at3(lex),
                Jump::J400 => goto400_at3(lex),
                Jump::J375 => goto375_at3(lex),
                Jump::J377 => goto377_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto540_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J449,
                J538,
                J441,
                J522,
                J525,
                J512,
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
                    __, __, __, __, J449, J512, __, __, __, __, J522, __, __, __, __, __, __, J525,
                    J538, J441, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J449 => goto449_at2(lex),
                Jump::J538 => goto538_at2(lex),
                Jump::J441 => goto441_at2(lex),
                Jump::J522 => goto522_at2(lex),
                Jump::J525 => goto525_at2(lex),
                Jump::J512 => goto512_at2(lex),
                Jump::__ => _error(lex),
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
        fn goto549_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(&[10u8]) => {
                    lex.bump_unchecked(1usize);
                    goto2_ctx3_x(lex)
                }
                _ => goto3_ctx3_x(lex),
            }
        }
        #[inline]
        fn goto381_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto404_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J375,
                J400,
                J381,
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
                    __, __, __, __, __, J381, __, J381, __, J381, __, J381, __, J381, __, J381, __,
                    J381, __, J381, __, J381, __, J381, __, J400, __, J400, J375, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match LUT[byte as usize] {
                Jump::J375 => goto375_at2(lex),
                Jump::J400 => goto400_at2(lex),
                Jump::J381 => goto381_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto375_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(2usize);
                    goto374_ctx373_x(lex)
                }
                _ => _error(lex),
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
        fn goto13_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorStringWithBackslashes));
        }
        #[inline]
        fn pattern5(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 4 > 0
        }
        #[inline]
        fn goto545_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto17_ctx17_x(lex),
            };
            match byte {
                b'"' => {
                    lex.bump_unchecked(1usize);
                    goto13_ctx17_x(lex)
                }
                byte if pattern5(byte) => {
                    lex.bump_unchecked(1usize);
                    goto545_ctx17_x(lex)
                }
                _ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto544_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J9,
                J545,
                J544,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, __, J544, J544, __,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J9, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J545, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544, J544,
                    J544, J544, J544, J544, J544, J544, J544, J544,
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
                Jump::J545 => {
                    lex.bump_unchecked(1usize);
                    goto545_ctx17_x(lex)
                }
                Jump::J544 => {
                    lex.bump_unchecked(1usize);
                    goto544_ctx17_x(lex)
                }
                Jump::__ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto552<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J5,
                J376,
                J1,
                J442,
                J377,
                J550,
                J443,
                J551,
                J547,
                J429,
                J540,
                J549,
                J404,
                J374,
                J375,
                J544,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, J5, J1, __, __, J549, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J5, __, J544, __, __, __,
                    __, __, __, __, __, __, __, __, __, J547, J374, J374, J374, J374, J374, J374,
                    J374, J374, J374, J374, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, J551, __, __, __, __, __, __, J550, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, J375, __, J376, __, __, __, J377, J404, J429, __, __,
                    __, __, __, __, __, __, J442, __, __, __, __, J443, J540, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __,
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
                Jump::J376 => goto376_at1(lex),
                Jump::J1 => {
                    lex.bump_unchecked(1usize);
                    goto1_x(lex)
                }
                Jump::J442 => goto442_at1(lex),
                Jump::J377 => goto377_at1(lex),
                Jump::J550 => goto550_at1(lex),
                Jump::J443 => goto443_at1(lex),
                Jump::J551 => goto551_at1(lex),
                Jump::J547 => goto547_at1(lex),
                Jump::J429 => goto429_at1(lex),
                Jump::J540 => goto540_at1(lex),
                Jump::J549 => {
                    lex.bump_unchecked(1usize);
                    goto549_ctx3_x(lex)
                }
                Jump::J404 => goto404_at1(lex),
                Jump::J374 => {
                    lex.bump_unchecked(1usize);
                    goto374_ctx373_x(lex)
                }
                Jump::J375 => goto375_at1(lex),
                Jump::J544 => {
                    lex.bump_unchecked(1usize);
                    goto544_ctx17_x(lex)
                }
                Jump::__ => _error(lex),
            }
        }
        goto552(lex)
    }
}
