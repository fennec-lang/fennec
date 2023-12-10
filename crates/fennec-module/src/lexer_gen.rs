// Generated code; run `cargo x gen-lex` to re-generate.
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
    ErrorIdentifier,
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
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 0, 12, 12, 0, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 4, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 13, 12, 12, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 12, 12, 12, 12, 12, 12, 12, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 12, 12, 12, 12, 12, 12, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
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
        fn goto26_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto26_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto26_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto26_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto26_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto26_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn goto28_ctx27_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto27_ctx26_x(lex));
        }
        #[inline]
        fn goto29_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto28_ctx27_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto27_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto29_at1_ctx26_x(lex),
                _ => goto26_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto30_ctx27_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto27_ctx26_x(lex));
        }
        #[inline]
        fn goto31_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto30_ctx27_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto32_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto31_at1_ctx26_x(lex),
                _ => goto26_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto31_at1_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto32_ctx26_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto30_ctx27_x(lex)
                }
                _ => goto32_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto32_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto31_at1_ctx32_x(lex),
                _ => goto26_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto34_ctx33_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto33_ctx32_x(lex));
        }
        #[inline]
        fn goto35_at1_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto32_ctx26_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto34_ctx33_x(lex)
                }
                _ => goto32_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto33_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto35_at1_ctx32_x(lex),
                _ => goto32_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto36_ctx33_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto33_ctx32_x(lex));
        }
        #[inline]
        fn goto37_at1_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto32_ctx26_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx33_x(lex)
                }
                _ => goto32_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto38_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto37_at1_ctx32_x(lex),
                _ => goto32_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto31_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto30_ctx27_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto32_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto31_at1_ctx38_x(lex),
                _ => goto26_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto37_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx33_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto38_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto37_at1_ctx38_x(lex),
                _ => goto32_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto65_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto40_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto46_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto69_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J46,
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
                    __, __, __, __, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46,
                    __, J46, __, J46, __, J46, __, J65, __, J65, J40, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at2_ctx38_x(lex),
                Jump::J40 => goto40_at2_ctx38_x(lex),
                Jump::J46 => goto46_at2_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto41_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto42_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto203_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J41,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J42, __, __, __, __, __,
                    J41, __, __, __, __, __, __, __, J41, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J65, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J41 => goto41_at3_ctx38_x(lex),
                Jump::J65 => goto65_at3_ctx38_x(lex),
                Jump::J42 => goto42_at3_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto106_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto190_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto40_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto126_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto46_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto177_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J40,
                J126,
                J41,
                J46,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J46, __, J41, J126, __, __, J65, __, __,
                    __, J41, __, __, __, __, __, J65, __, J65, __, __, __, __, __, J65, __, J42,
                    J41, __, __, __, __, __, __, J40, __, J65, __, __, __, __, __, __, __, __, __,
                    __, __, J65, __, __, __, J65, J40, __, __, __, __, __, __, J65, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J40 => goto40_at3_ctx38_x(lex),
                Jump::J126 => goto126_at3_ctx38_x(lex),
                Jump::J41 => goto41_at3_ctx38_x(lex),
                Jump::J46 => goto46_at3_ctx38_x(lex),
                Jump::J65 => goto65_at3_ctx38_x(lex),
                Jump::J42 => goto42_at3_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto187_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J40, __, J42, __, J65, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at3_ctx38_x(lex),
                Jump::J40 => goto40_at3_ctx38_x(lex),
                Jump::J42 => goto42_at3_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto114_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                180u8 => goto41_at3_ctx38_x(lex),
                146u8 => goto40_at3_ctx38_x(lex),
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto205_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J203,
                J106,
                J190,
                J177,
                J187,
                J114,
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
                    __, __, __, __, J114, J177, __, __, __, __, J187, __, __, __, __, __, __, J190,
                    J203, J106, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J203 => goto203_at2_ctx38_x(lex),
                Jump::J106 => goto106_at2_ctx38_x(lex),
                Jump::J190 => goto190_at2_ctx38_x(lex),
                Jump::J177 => goto177_at2_ctx38_x(lex),
                Jump::J187 => goto187_at2_ctx38_x(lex),
                Jump::J114 => goto114_at2_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto108_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto79_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
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
        fn goto86_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto41_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto42_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto94_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J79,
                J86,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J42, J65, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J40, J65, __, __, __, __, J79, __, J65, __, __, J86, __, __, J65, J41, __, __,
                    J86, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J79 => goto79_at2_ctx38_x(lex),
                Jump::J86 => goto86_at2_ctx38_x(lex),
                Jump::J65 => goto65_at2_ctx38_x(lex),
                Jump::J40 => goto40_at2_ctx38_x(lex),
                Jump::J41 => goto41_at2_ctx38_x(lex),
                Jump::J42 => goto42_at2_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto41_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto40_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn pattern3(byte: u8) -> bool {
            match byte {
                144u8..=153u8 | 176u8..=185u8 => true,
                _ => false,
            }
        }
        #[inline]
        fn goto103_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto107_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J103,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J40, __, __, __, __, __, __,
                    __, __, __, __, J65, J42, __, __, J103, __, J65, __, __, __, __, __, J41, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J103 => goto103_at2_ctx38_x(lex),
                Jump::J65 => goto65_at2_ctx38_x(lex),
                Jump::J40 => goto40_at2_ctx38_x(lex),
                Jump::J41 => goto41_at2_ctx38_x(lex),
                Jump::J42 => goto42_at2_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto42_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto39_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J69,
                J39,
                J205,
                J108,
                J94,
                J41,
                J40,
                J107,
                J42,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J39, J39, J39, J39, J39, J39, J39, J39, J39,
                    J39, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J40, __, J41, __, __, __, J42, J69, J94, __, __, __, __, __, __, __, __,
                    J107, __, __, __, __, J108, J205, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto38_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J69 => goto69_at1_ctx38_x(lex),
                Jump::J39 => {
                    lex.bump_unchecked(1usize);
                    goto39_ctx38_x(lex)
                }
                Jump::J205 => goto205_at1_ctx38_x(lex),
                Jump::J108 => goto108_at1_ctx38_x(lex),
                Jump::J94 => goto94_at1_ctx38_x(lex),
                Jump::J41 => goto41_at1_ctx38_x(lex),
                Jump::J40 => goto40_at1_ctx38_x(lex),
                Jump::J107 => goto107_at1_ctx38_x(lex),
                Jump::J42 => goto42_at1_ctx38_x(lex),
                Jump::__ => goto38_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto65_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto69_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J46,
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
                    __, __, __, __, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46,
                    __, J46, __, J46, __, J46, __, J65, __, J65, J40, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at3_ctx38_x(lex),
                Jump::J40 => goto40_at3_ctx38_x(lex),
                Jump::J46 => goto46_at3_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto41_at4_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto65_at4_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto42_at4_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto203_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J41,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J42, __, __, __, __, __,
                    J41, __, __, __, __, __, __, __, J41, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J65, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J41 => goto41_at4_ctx38_x(lex),
                Jump::J65 => goto65_at4_ctx38_x(lex),
                Jump::J42 => goto42_at4_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto106_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto190_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto40_at4_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto126_at4_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto46_at4_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto177_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J40,
                J126,
                J41,
                J46,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J46, __, J41, J126, __, __, J65, __, __,
                    __, J41, __, __, __, __, __, J65, __, J65, __, __, __, __, __, J65, __, J42,
                    J41, __, __, __, __, __, __, J40, __, J65, __, __, __, __, __, __, __, __, __,
                    __, __, J65, __, __, __, J65, J40, __, __, __, __, __, __, J65, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J40 => goto40_at4_ctx38_x(lex),
                Jump::J126 => goto126_at4_ctx38_x(lex),
                Jump::J41 => goto41_at4_ctx38_x(lex),
                Jump::J46 => goto46_at4_ctx38_x(lex),
                Jump::J65 => goto65_at4_ctx38_x(lex),
                Jump::J42 => goto42_at4_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto187_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J40, __, J42, __, J65, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at4_ctx38_x(lex),
                Jump::J40 => goto40_at4_ctx38_x(lex),
                Jump::J42 => goto42_at4_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto114_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                180u8 => goto41_at4_ctx38_x(lex),
                146u8 => goto40_at4_ctx38_x(lex),
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto205_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J203,
                J106,
                J190,
                J177,
                J187,
                J114,
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
                    __, __, __, __, J114, J177, __, __, __, __, J187, __, __, __, __, __, __, J190,
                    J203, J106, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J203 => goto203_at3_ctx38_x(lex),
                Jump::J106 => goto106_at3_ctx38_x(lex),
                Jump::J190 => goto190_at3_ctx38_x(lex),
                Jump::J177 => goto177_at3_ctx38_x(lex),
                Jump::J187 => goto187_at3_ctx38_x(lex),
                Jump::J114 => goto114_at3_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto108_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto79_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto86_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto94_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J79,
                J86,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J42, J65, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J40, J65, __, __, __, __, J79, __, J65, __, __, J86, __, __, J65, J41, __, __,
                    J86, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J79 => goto79_at3_ctx38_x(lex),
                Jump::J86 => goto86_at3_ctx38_x(lex),
                Jump::J65 => goto65_at3_ctx38_x(lex),
                Jump::J40 => goto40_at3_ctx38_x(lex),
                Jump::J41 => goto41_at3_ctx38_x(lex),
                Jump::J42 => goto42_at3_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto103_at3_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto107_at2_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J103,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J40, __, __, __, __, __, __,
                    __, __, __, __, J65, J42, __, __, J103, __, J65, __, __, __, __, __, J41, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J103 => goto103_at3_ctx38_x(lex),
                Jump::J65 => goto65_at3_ctx38_x(lex),
                Jump::J40 => goto40_at3_ctx38_x(lex),
                Jump::J41 => goto41_at3_ctx38_x(lex),
                Jump::J42 => goto42_at3_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto206_at1_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J69,
                J39,
                J205,
                J108,
                J94,
                J41,
                J40,
                J107,
                J42,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J39, J39, J39, J39, J39, J39, J39, J39, J39,
                    J39, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J40, __, J41, __, __, __, J42, J69, J94, __, __, __, __, __, __, __, __,
                    J107, __, __, __, __, J108, J205, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto38_ctx32_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J69 => goto69_at2_ctx38_x(lex),
                Jump::J39 => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                Jump::J205 => goto205_at2_ctx38_x(lex),
                Jump::J108 => goto108_at2_ctx38_x(lex),
                Jump::J94 => goto94_at2_ctx38_x(lex),
                Jump::J41 => goto41_at2_ctx38_x(lex),
                Jump::J40 => goto40_at2_ctx38_x(lex),
                Jump::J107 => goto107_at2_ctx38_x(lex),
                Jump::J42 => goto42_at2_ctx38_x(lex),
                Jump::__ => goto38_ctx32_x(lex),
            }
        }
        #[inline]
        fn goto207_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto206_at1_ctx38_x(lex),
                _ => goto38_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto31_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto30_ctx27_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto32_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto31_at1_ctx207_x(lex),
                _ => goto26_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto37_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx33_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto38_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto37_at1_ctx207_x(lex),
                _ => goto32_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto65_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto40_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto46_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto69_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J46,
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
                    __, __, __, __, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46,
                    __, J46, __, J46, __, J46, __, J65, __, J65, J40, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at3_ctx207_x(lex),
                Jump::J40 => goto40_at3_ctx207_x(lex),
                Jump::J46 => goto46_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto41_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto65_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto42_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto203_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J41,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J42, __, __, __, __, __,
                    J41, __, __, __, __, __, __, __, J41, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J65, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J41 => goto41_at4_ctx207_x(lex),
                Jump::J65 => goto65_at4_ctx207_x(lex),
                Jump::J42 => goto42_at4_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto106_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto190_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto40_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto126_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto46_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto177_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J40,
                J126,
                J41,
                J46,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J46, __, J41, J126, __, __, J65, __, __,
                    __, J41, __, __, __, __, __, J65, __, J65, __, __, __, __, __, J65, __, J42,
                    J41, __, __, __, __, __, __, J40, __, J65, __, __, __, __, __, __, __, __, __,
                    __, __, J65, __, __, __, J65, J40, __, __, __, __, __, __, J65, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J40 => goto40_at4_ctx207_x(lex),
                Jump::J126 => goto126_at4_ctx207_x(lex),
                Jump::J41 => goto41_at4_ctx207_x(lex),
                Jump::J46 => goto46_at4_ctx207_x(lex),
                Jump::J65 => goto65_at4_ctx207_x(lex),
                Jump::J42 => goto42_at4_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto187_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J40, __, J42, __, J65, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at4_ctx207_x(lex),
                Jump::J40 => goto40_at4_ctx207_x(lex),
                Jump::J42 => goto42_at4_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto114_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                180u8 => goto41_at4_ctx207_x(lex),
                146u8 => goto40_at4_ctx207_x(lex),
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto205_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J203,
                J106,
                J190,
                J177,
                J187,
                J114,
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
                    __, __, __, __, J114, J177, __, __, __, __, J187, __, __, __, __, __, __, J190,
                    J203, J106, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J203 => goto203_at3_ctx207_x(lex),
                Jump::J106 => goto106_at3_ctx207_x(lex),
                Jump::J190 => goto190_at3_ctx207_x(lex),
                Jump::J177 => goto177_at3_ctx207_x(lex),
                Jump::J187 => goto187_at3_ctx207_x(lex),
                Jump::J114 => goto114_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto108_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto79_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto86_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto41_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto42_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto94_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J79,
                J86,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J42, J65, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J40, J65, __, __, __, __, J79, __, J65, __, __, J86, __, __, J65, J41, __, __,
                    J86, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J79 => goto79_at3_ctx207_x(lex),
                Jump::J86 => goto86_at3_ctx207_x(lex),
                Jump::J65 => goto65_at3_ctx207_x(lex),
                Jump::J40 => goto40_at3_ctx207_x(lex),
                Jump::J41 => goto41_at3_ctx207_x(lex),
                Jump::J42 => goto42_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto41_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto40_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto103_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto107_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J103,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J40, __, __, __, __, __, __,
                    __, __, __, __, J65, J42, __, __, J103, __, J65, __, __, __, __, __, J41, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J103 => goto103_at3_ctx207_x(lex),
                Jump::J65 => goto65_at3_ctx207_x(lex),
                Jump::J40 => goto40_at3_ctx207_x(lex),
                Jump::J41 => goto41_at3_ctx207_x(lex),
                Jump::J42 => goto42_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto42_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto206_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J69,
                J39,
                J205,
                J108,
                J94,
                J41,
                J40,
                J107,
                J42,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J39, J39, J39, J39, J39, J39, J39, J39, J39,
                    J39, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J40, __, J41, __, __, __, J42, J69, J94, __, __, __, __, __, __, __, __,
                    J107, __, __, __, __, J108, J205, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J69 => goto69_at2_ctx207_x(lex),
                Jump::J39 => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                Jump::J205 => goto205_at2_ctx207_x(lex),
                Jump::J108 => goto108_at2_ctx207_x(lex),
                Jump::J94 => goto94_at2_ctx207_x(lex),
                Jump::J41 => goto41_at2_ctx207_x(lex),
                Jump::J40 => goto40_at2_ctx207_x(lex),
                Jump::J107 => goto107_at2_ctx207_x(lex),
                Jump::J42 => goto42_at2_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto207_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto206_at1_ctx207_x(lex),
                _ => goto38_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto209_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(2usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto210_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto272_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto211_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto234_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto276_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J210,
                J272,
                J209,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J209, __, __, __, __, __, __,
                    __, __, __, __, J234, J211, __, __, J272, __, J234, __, __, __, __, __, J210,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J210 => goto210_at2_ctx207_x(lex),
                Jump::J272 => goto272_at2_ctx207_x(lex),
                Jump::J209 => goto209_at2_ctx207_x(lex),
                Jump::J211 => goto211_at2_ctx207_x(lex),
                Jump::J234 => goto234_at2_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto211_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto210_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto209_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto210_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto283_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                146u8 => goto209_at3_ctx207_x(lex),
                180u8 => goto210_at3_ctx207_x(lex),
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto275_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto234_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto215_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto295_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto211_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto346_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J234,
                J215,
                J209,
                J295,
                J211,
                J210,
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
                    __, __, __, __, __, __, __, __, __, J215, __, J210, J295, __, __, J234, __, __,
                    __, J210, __, __, __, __, __, J234, __, J234, __, __, __, __, __, J234, __,
                    J211, J210, __, __, __, __, __, __, J209, __, J234, __, __, __, __, __, __, __,
                    __, __, __, __, J234, __, __, __, J234, J209, __, __, __, __, __, __, J234, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J234 => goto234_at3_ctx207_x(lex),
                Jump::J215 => goto215_at3_ctx207_x(lex),
                Jump::J209 => goto209_at3_ctx207_x(lex),
                Jump::J295 => goto295_at3_ctx207_x(lex),
                Jump::J211 => goto211_at3_ctx207_x(lex),
                Jump::J210 => goto210_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto359_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto356_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, J209, __, J211, __, J234, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at3_ctx207_x(lex),
                Jump::J211 => goto211_at3_ctx207_x(lex),
                Jump::J234 => goto234_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto372_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J234,
                J211,
                J210,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J211, __, __, __, __, __,
                    J210, __, __, __, __, __, __, __, J210, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J234, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J234 => goto234_at3_ctx207_x(lex),
                Jump::J211 => goto211_at3_ctx207_x(lex),
                Jump::J210 => goto210_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto374_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J283,
                J275,
                J346,
                J359,
                J356,
                J372,
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
                    __, __, __, __, J283, J346, __, __, __, __, J356, __, __, __, __, __, __, J359,
                    J372, J275, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J283 => goto283_at2_ctx207_x(lex),
                Jump::J275 => goto275_at2_ctx207_x(lex),
                Jump::J346 => goto346_at2_ctx207_x(lex),
                Jump::J359 => goto359_at2_ctx207_x(lex),
                Jump::J356 => goto356_at2_ctx207_x(lex),
                Jump::J372 => goto372_at2_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto215_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto238_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J215,
                J234,
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
                    __, __, __, __, __, J215, __, J215, __, J215, __, J215, __, J215, __, J215, __,
                    J215, __, J215, __, J215, __, J215, __, J234, __, J234, J209, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at2_ctx207_x(lex),
                Jump::J215 => goto215_at2_ctx207_x(lex),
                Jump::J234 => goto234_at2_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto277_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto255_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto248_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto263_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J210,
                J255,
                J209,
                J248,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, J211, J234, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J209, J234, __, __, __, __, J248, __, J234, __, __, J255, __, __, J234, J210,
                    __, __, J255, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J210 => goto210_at2_ctx207_x(lex),
                Jump::J255 => goto255_at2_ctx207_x(lex),
                Jump::J209 => goto209_at2_ctx207_x(lex),
                Jump::J248 => goto248_at2_ctx207_x(lex),
                Jump::J211 => goto211_at2_ctx207_x(lex),
                Jump::J234 => goto234_at2_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto208_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J208,
                J276,
                J211,
                J210,
                J374,
                J238,
                J277,
                J263,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J208, J208, J208, J208, J208, J208, J208, J208,
                    J208, J208, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J209, __, J210, __, __, __, J211, J238, J263, __, __, __, __, __, __,
                    __, __, J276, __, __, __, __, J277, J374, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto207_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at1_ctx207_x(lex),
                Jump::J208 => {
                    lex.bump_unchecked(1usize);
                    goto208_ctx207_x(lex)
                }
                Jump::J276 => goto276_at1_ctx207_x(lex),
                Jump::J211 => goto211_at1_ctx207_x(lex),
                Jump::J210 => goto210_at1_ctx207_x(lex),
                Jump::J374 => goto374_at1_ctx207_x(lex),
                Jump::J238 => goto238_at1_ctx207_x(lex),
                Jump::J277 => goto277_at1_ctx207_x(lex),
                Jump::J263 => goto263_at1_ctx207_x(lex),
                Jump::__ => goto207_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto209_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto272_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto276_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J210,
                J272,
                J209,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J209, __, __, __, __, __, __,
                    __, __, __, __, J234, J211, __, __, J272, __, J234, __, __, __, __, __, J210,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J210 => goto210_at3_ctx207_x(lex),
                Jump::J272 => goto272_at3_ctx207_x(lex),
                Jump::J209 => goto209_at3_ctx207_x(lex),
                Jump::J211 => goto211_at3_ctx207_x(lex),
                Jump::J234 => goto234_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto209_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto210_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto283_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                146u8 => goto209_at4_ctx207_x(lex),
                180u8 => goto210_at4_ctx207_x(lex),
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto275_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto234_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto215_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto295_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto211_at4_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto346_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J234,
                J215,
                J209,
                J295,
                J211,
                J210,
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
                    __, __, __, __, __, __, __, __, __, J215, __, J210, J295, __, __, J234, __, __,
                    __, J210, __, __, __, __, __, J234, __, J234, __, __, __, __, __, J234, __,
                    J211, J210, __, __, __, __, __, __, J209, __, J234, __, __, __, __, __, __, __,
                    __, __, __, __, J234, __, __, __, J234, J209, __, __, __, __, __, __, J234, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J234 => goto234_at4_ctx207_x(lex),
                Jump::J215 => goto215_at4_ctx207_x(lex),
                Jump::J209 => goto209_at4_ctx207_x(lex),
                Jump::J295 => goto295_at4_ctx207_x(lex),
                Jump::J211 => goto211_at4_ctx207_x(lex),
                Jump::J210 => goto210_at4_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto359_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto356_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, J209, __, J211, __, J234, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at4_ctx207_x(lex),
                Jump::J211 => goto211_at4_ctx207_x(lex),
                Jump::J234 => goto234_at4_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto372_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J234,
                J211,
                J210,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J211, __, __, __, __, __,
                    J210, __, __, __, __, __, __, __, J210, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J234, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J234 => goto234_at4_ctx207_x(lex),
                Jump::J211 => goto211_at4_ctx207_x(lex),
                Jump::J210 => goto210_at4_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto374_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J283,
                J275,
                J346,
                J359,
                J356,
                J372,
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
                    __, __, __, __, J283, J346, __, __, __, __, J356, __, __, __, __, __, __, J359,
                    J372, J275, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J283 => goto283_at3_ctx207_x(lex),
                Jump::J275 => goto275_at3_ctx207_x(lex),
                Jump::J346 => goto346_at3_ctx207_x(lex),
                Jump::J359 => goto359_at3_ctx207_x(lex),
                Jump::J356 => goto356_at3_ctx207_x(lex),
                Jump::J372 => goto372_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto238_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J215,
                J234,
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
                    __, __, __, __, __, J215, __, J215, __, J215, __, J215, __, J215, __, J215, __,
                    J215, __, J215, __, J215, __, J215, __, J234, __, J234, J209, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at3_ctx207_x(lex),
                Jump::J215 => goto215_at3_ctx207_x(lex),
                Jump::J234 => goto234_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto277_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto255_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto248_at3_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto263_at2_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J210,
                J255,
                J209,
                J248,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, J211, J234, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J209, J234, __, __, __, __, J248, __, J234, __, __, J255, __, __, J234, J210,
                    __, __, J255, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J210 => goto210_at3_ctx207_x(lex),
                Jump::J255 => goto255_at3_ctx207_x(lex),
                Jump::J209 => goto209_at3_ctx207_x(lex),
                Jump::J248 => goto248_at3_ctx207_x(lex),
                Jump::J211 => goto211_at3_ctx207_x(lex),
                Jump::J234 => goto234_at3_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto375_at1_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J208,
                J276,
                J211,
                J210,
                J374,
                J238,
                J277,
                J263,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J208, J208, J208, J208, J208, J208, J208, J208,
                    J208, J208, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J209, __, J210, __, __, __, J211, J238, J263, __, __, __, __, __, __,
                    __, __, J276, __, __, __, __, J277, J374, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto207_ctx38_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at2_ctx207_x(lex),
                Jump::J208 => {
                    lex.bump_unchecked(2usize);
                    goto208_ctx207_x(lex)
                }
                Jump::J276 => goto276_at2_ctx207_x(lex),
                Jump::J211 => goto211_at2_ctx207_x(lex),
                Jump::J210 => goto210_at2_ctx207_x(lex),
                Jump::J374 => goto374_at2_ctx207_x(lex),
                Jump::J238 => goto238_at2_ctx207_x(lex),
                Jump::J277 => goto277_at2_ctx207_x(lex),
                Jump::J263 => goto263_at2_ctx207_x(lex),
                Jump::__ => goto207_ctx38_x(lex),
            }
        }
        #[inline]
        fn goto376_ctx207_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto375_at1_ctx207_x(lex),
                _ => goto207_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto31_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto30_ctx27_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto32_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"+") => goto31_at1_ctx376_x(lex),
                _ => goto26_ctx376_x(lex),
            }
        }
        #[inline]
        fn goto37_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx33_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto38_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b"-") => goto37_at1_ctx376_x(lex),
                _ => goto32_ctx376_x(lex),
            }
        }
        #[inline]
        fn goto65_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto40_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto46_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto69_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J46,
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
                    __, __, __, __, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46,
                    __, J46, __, J46, __, J46, __, J65, __, J65, J40, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at3_ctx376_x(lex),
                Jump::J40 => goto40_at3_ctx376_x(lex),
                Jump::J46 => goto46_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto41_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto65_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto42_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto203_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J41,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J42, __, __, __, __, __,
                    J41, __, __, __, __, __, __, __, J41, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J65, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J41 => goto41_at4_ctx376_x(lex),
                Jump::J65 => goto65_at4_ctx376_x(lex),
                Jump::J42 => goto42_at4_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto106_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto190_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto40_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto126_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto46_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto177_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J40,
                J126,
                J41,
                J46,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J46, __, J41, J126, __, __, J65, __, __,
                    __, J41, __, __, __, __, __, J65, __, J65, __, __, __, __, __, J65, __, J42,
                    J41, __, __, __, __, __, __, J40, __, J65, __, __, __, __, __, __, __, __, __,
                    __, __, J65, __, __, __, J65, J40, __, __, __, __, __, __, J65, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J40 => goto40_at4_ctx376_x(lex),
                Jump::J126 => goto126_at4_ctx376_x(lex),
                Jump::J41 => goto41_at4_ctx376_x(lex),
                Jump::J46 => goto46_at4_ctx376_x(lex),
                Jump::J65 => goto65_at4_ctx376_x(lex),
                Jump::J42 => goto42_at4_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto187_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J40, __, J42, __, J65, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at4_ctx376_x(lex),
                Jump::J40 => goto40_at4_ctx376_x(lex),
                Jump::J42 => goto42_at4_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto114_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                180u8 => goto41_at4_ctx376_x(lex),
                146u8 => goto40_at4_ctx376_x(lex),
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto205_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J203,
                J106,
                J190,
                J177,
                J187,
                J114,
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
                    __, __, __, __, J114, J177, __, __, __, __, J187, __, __, __, __, __, __, J190,
                    J203, J106, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J203 => goto203_at3_ctx376_x(lex),
                Jump::J106 => goto106_at3_ctx376_x(lex),
                Jump::J190 => goto190_at3_ctx376_x(lex),
                Jump::J177 => goto177_at3_ctx376_x(lex),
                Jump::J187 => goto187_at3_ctx376_x(lex),
                Jump::J114 => goto114_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto108_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto79_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto86_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto41_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto42_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto94_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J79,
                J86,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J42, J65, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J40, J65, __, __, __, __, J79, __, J65, __, __, J86, __, __, J65, J41, __, __,
                    J86, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J79 => goto79_at3_ctx376_x(lex),
                Jump::J86 => goto86_at3_ctx376_x(lex),
                Jump::J65 => goto65_at3_ctx376_x(lex),
                Jump::J40 => goto40_at3_ctx376_x(lex),
                Jump::J41 => goto41_at3_ctx376_x(lex),
                Jump::J42 => goto42_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto41_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto40_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto103_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto107_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J103,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J40, __, __, __, __, __, __,
                    __, __, __, __, J65, J42, __, __, J103, __, J65, __, __, __, __, __, J41, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J103 => goto103_at3_ctx376_x(lex),
                Jump::J65 => goto65_at3_ctx376_x(lex),
                Jump::J40 => goto40_at3_ctx376_x(lex),
                Jump::J41 => goto41_at3_ctx376_x(lex),
                Jump::J42 => goto42_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto42_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto206_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J69,
                J39,
                J205,
                J108,
                J94,
                J41,
                J40,
                J107,
                J42,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J39, J39, J39, J39, J39, J39, J39, J39, J39,
                    J39, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J40, __, J41, __, __, __, J42, J69, J94, __, __, __, __, __, __, __, __,
                    J107, __, __, __, __, J108, J205, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J69 => goto69_at2_ctx376_x(lex),
                Jump::J39 => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                Jump::J205 => goto205_at2_ctx376_x(lex),
                Jump::J108 => goto108_at2_ctx376_x(lex),
                Jump::J94 => goto94_at2_ctx376_x(lex),
                Jump::J41 => goto41_at2_ctx376_x(lex),
                Jump::J40 => goto40_at2_ctx376_x(lex),
                Jump::J107 => goto107_at2_ctx376_x(lex),
                Jump::J42 => goto42_at2_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto207_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto206_at1_ctx376_x(lex),
                _ => goto38_ctx376_x(lex),
            }
        }
        #[inline]
        fn goto209_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto210_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto272_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto209_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto211_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto234_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto276_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J210,
                J272,
                J209,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J209, __, __, __, __, __, __,
                    __, __, __, __, J234, J211, __, __, J272, __, J234, __, __, __, __, __, J210,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J210 => goto210_at3_ctx376_x(lex),
                Jump::J272 => goto272_at3_ctx376_x(lex),
                Jump::J209 => goto209_at3_ctx376_x(lex),
                Jump::J211 => goto211_at3_ctx376_x(lex),
                Jump::J234 => goto234_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto211_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto210_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto209_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto210_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto283_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                146u8 => goto209_at4_ctx376_x(lex),
                180u8 => goto210_at4_ctx376_x(lex),
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto275_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto234_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto215_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto295_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto211_at4_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto346_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J234,
                J215,
                J209,
                J295,
                J211,
                J210,
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
                    __, __, __, __, __, __, __, __, __, J215, __, J210, J295, __, __, J234, __, __,
                    __, J210, __, __, __, __, __, J234, __, J234, __, __, __, __, __, J234, __,
                    J211, J210, __, __, __, __, __, __, J209, __, J234, __, __, __, __, __, __, __,
                    __, __, __, __, J234, __, __, __, J234, J209, __, __, __, __, __, __, J234, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J234 => goto234_at4_ctx376_x(lex),
                Jump::J215 => goto215_at4_ctx376_x(lex),
                Jump::J209 => goto209_at4_ctx376_x(lex),
                Jump::J295 => goto295_at4_ctx376_x(lex),
                Jump::J211 => goto211_at4_ctx376_x(lex),
                Jump::J210 => goto210_at4_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto359_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto356_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, J209, __, J211, __, J234, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at4_ctx376_x(lex),
                Jump::J211 => goto211_at4_ctx376_x(lex),
                Jump::J234 => goto234_at4_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto372_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J234,
                J211,
                J210,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J211, __, __, __, __, __,
                    J210, __, __, __, __, __, __, __, J210, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J234, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J234 => goto234_at4_ctx376_x(lex),
                Jump::J211 => goto211_at4_ctx376_x(lex),
                Jump::J210 => goto210_at4_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto374_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J283,
                J275,
                J346,
                J359,
                J356,
                J372,
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
                    __, __, __, __, J283, J346, __, __, __, __, J356, __, __, __, __, __, __, J359,
                    J372, J275, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J283 => goto283_at3_ctx376_x(lex),
                Jump::J275 => goto275_at3_ctx376_x(lex),
                Jump::J346 => goto346_at3_ctx376_x(lex),
                Jump::J359 => goto359_at3_ctx376_x(lex),
                Jump::J356 => goto356_at3_ctx376_x(lex),
                Jump::J372 => goto372_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto215_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto238_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J215,
                J234,
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
                    __, __, __, __, __, J215, __, J215, __, J215, __, J215, __, J215, __, J215, __,
                    J215, __, J215, __, J215, __, J215, __, J234, __, J234, J209, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at3_ctx376_x(lex),
                Jump::J215 => goto215_at3_ctx376_x(lex),
                Jump::J234 => goto234_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto277_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto255_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto248_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto208_ctx207_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto263_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J210,
                J255,
                J209,
                J248,
                J211,
                J234,
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
                    __, __, __, __, __, __, __, __, __, J211, J234, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J209, J234, __, __, __, __, J248, __, J234, __, __, J255, __, __, J234, J210,
                    __, __, J255, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J210 => goto210_at3_ctx376_x(lex),
                Jump::J255 => goto255_at3_ctx376_x(lex),
                Jump::J209 => goto209_at3_ctx376_x(lex),
                Jump::J248 => goto248_at3_ctx376_x(lex),
                Jump::J211 => goto211_at3_ctx376_x(lex),
                Jump::J234 => goto234_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto375_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J209,
                J208,
                J276,
                J211,
                J210,
                J374,
                J238,
                J277,
                J263,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J208, J208, J208, J208, J208, J208, J208, J208,
                    J208, J208, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J209, __, J210, __, __, __, J211, J238, J263, __, __, __, __, __, __,
                    __, __, J276, __, __, __, __, J277, J374, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J209 => goto209_at2_ctx376_x(lex),
                Jump::J208 => {
                    lex.bump_unchecked(2usize);
                    goto208_ctx207_x(lex)
                }
                Jump::J276 => goto276_at2_ctx376_x(lex),
                Jump::J211 => goto211_at2_ctx376_x(lex),
                Jump::J210 => goto210_at2_ctx376_x(lex),
                Jump::J374 => goto374_at2_ctx376_x(lex),
                Jump::J238 => goto238_at2_ctx376_x(lex),
                Jump::J277 => goto277_at2_ctx376_x(lex),
                Jump::J263 => goto263_at2_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto376_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(b".") => goto375_at1_ctx376_x(lex),
                _ => goto207_ctx376_x(lex),
            }
        }
        #[inline]
        fn goto446_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto403_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto379_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto378_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(1usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto380_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto441_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto445_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J403,
                J379,
                J378,
                J380,
                J441,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J378, __, __, __, __, __, __,
                    __, __, __, __, J403, J380, __, __, J441, __, J403, __, __, __, __, __, J379,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J403 => goto403_at2_ctx376_x(lex),
                Jump::J379 => goto379_at2_ctx376_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_ctx376_x(lex)
                }
                Jump::J380 => goto380_at2_ctx376_x(lex),
                Jump::J441 => goto441_at2_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto379_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto384_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto464_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto403_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto380_at3_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto515_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J379,
                J384,
                J464,
                J403,
                J378,
                J380,
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
                    __, __, __, __, __, __, __, __, __, J384, __, J379, J464, __, __, J403, __, __,
                    __, J379, __, __, __, __, __, J403, __, J403, __, __, __, __, __, J403, __,
                    J380, J379, __, __, __, __, __, __, J378, __, J403, __, __, __, __, __, __, __,
                    __, __, __, __, J403, __, __, __, J403, J378, __, __, __, __, __, __, J403, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J379 => goto379_at3_ctx376_x(lex),
                Jump::J384 => goto384_at3_ctx376_x(lex),
                Jump::J464 => goto464_at3_ctx376_x(lex),
                Jump::J403 => goto403_at3_ctx376_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(3usize);
                    goto378_ctx376_x(lex)
                }
                Jump::J380 => goto380_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto528_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto541_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J379,
                J380,
                J403,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J380, __, __, __, __, __,
                    J379, __, __, __, __, __, __, __, J379, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J403, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J379 => goto379_at3_ctx376_x(lex),
                Jump::J380 => goto380_at3_ctx376_x(lex),
                Jump::J403 => goto403_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto444_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto452_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                146u8 => {
                    lex.bump_unchecked(3usize);
                    goto378_ctx376_x(lex)
                }
                180u8 => goto379_at3_ctx376_x(lex),
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto525_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J378,
                J403,
                J380,
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
                    __, __, __, __, __, __, __, __, __, J378, __, J380, __, J403, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J378 => {
                    lex.bump_unchecked(3usize);
                    goto378_ctx376_x(lex)
                }
                Jump::J403 => goto403_at3_ctx376_x(lex),
                Jump::J380 => goto380_at3_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto543_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J515,
                J528,
                J541,
                J444,
                J452,
                J525,
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
                    __, __, __, __, J452, J515, __, __, __, __, J525, __, __, __, __, __, __, J528,
                    J541, J444, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J515 => goto515_at2_ctx376_x(lex),
                Jump::J528 => goto528_at2_ctx376_x(lex),
                Jump::J541 => goto541_at2_ctx376_x(lex),
                Jump::J444 => goto444_at2_ctx376_x(lex),
                Jump::J452 => goto452_at2_ctx376_x(lex),
                Jump::J525 => goto525_at2_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto379_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto424_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto417_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto432_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J403,
                J424,
                J379,
                J378,
                J380,
                J417,
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
                    __, __, __, __, __, __, __, __, __, J380, J403, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J378, J403, __, __, __, __, J417, __, J403, __, __, J424, __, __, J403, J379,
                    __, __, J424, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J403 => goto403_at2_ctx376_x(lex),
                Jump::J424 => goto424_at2_ctx376_x(lex),
                Jump::J379 => goto379_at2_ctx376_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_ctx376_x(lex)
                }
                Jump::J380 => goto380_at2_ctx376_x(lex),
                Jump::J417 => goto417_at2_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto384_at2_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto407_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J378,
                J403,
                J384,
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
                    __, __, __, __, __, J384, __, J384, __, J384, __, J384, __, J384, __, J384, __,
                    J384, __, J384, __, J384, __, J384, __, J403, __, J403, J378, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto376_ctx207_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_ctx376_x(lex)
                }
                Jump::J403 => goto403_at2_ctx376_x(lex),
                Jump::J384 => goto384_at2_ctx376_x(lex),
                Jump::__ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto380_at1_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto376_ctx207_x(lex),
            }
        }
        #[inline]
        fn goto377_ctx376_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J446,
                J445,
                J378,
                J543,
                J377,
                J379,
                J432,
                J407,
                J380,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J377, J377, J377, J377, J377, J377, J377, J377,
                    J377, J377, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J378, __, J379, __, __, __, J380, J407, J432, __, __, __, __, __, __,
                    __, __, J445, __, __, __, __, J446, J543, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto376_ctx376_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J446 => goto446_at1_ctx376_x(lex),
                Jump::J445 => goto445_at1_ctx376_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(1usize);
                    goto378_ctx376_x(lex)
                }
                Jump::J543 => goto543_at1_ctx376_x(lex),
                Jump::J377 => {
                    lex.bump_unchecked(1usize);
                    goto377_ctx376_x(lex)
                }
                Jump::J379 => goto379_at1_ctx376_x(lex),
                Jump::J432 => goto432_at1_ctx376_x(lex),
                Jump::J407 => goto407_at1_ctx376_x(lex),
                Jump::J380 => goto380_at1_ctx376_x(lex),
                Jump::__ => goto376_ctx376_x(lex),
            }
        }
        #[inline]
        fn goto446_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto403_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto379_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto378_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(1usize);
                    goto377_ctx376_x(lex)
                }
                _ => lex.error(),
            }
        }
        #[inline]
        fn goto380_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto441_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto445_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J403,
                J379,
                J378,
                J380,
                J441,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J378, __, __, __, __, __, __,
                    __, __, __, __, J403, J380, __, __, J441, __, J403, __, __, __, __, __, J379,
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
                Jump::J403 => goto403_at2(lex),
                Jump::J379 => goto379_at2(lex),
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_x(lex)
                }
                Jump::J380 => goto380_at2(lex),
                Jump::J441 => goto441_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto1_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
        }
        #[inline]
        fn goto379_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto384_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto464_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto403_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto380_at3<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto515_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J379,
                J384,
                J464,
                J403,
                J378,
                J380,
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
                    __, __, __, __, __, __, __, __, __, J384, __, J379, J464, __, __, J403, __, __,
                    __, J379, __, __, __, __, __, J403, __, J403, __, __, __, __, __, J403, __,
                    J380, J379, __, __, __, __, __, __, J378, __, J403, __, __, __, __, __, __, __,
                    __, __, __, __, J403, __, __, __, J403, J378, __, __, __, __, __, __, J403, __,
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
                Jump::J379 => goto379_at3(lex),
                Jump::J384 => goto384_at3(lex),
                Jump::J464 => goto464_at3(lex),
                Jump::J403 => goto403_at3(lex),
                Jump::J378 => {
                    lex.bump_unchecked(3usize);
                    goto378_x(lex)
                }
                Jump::J380 => goto380_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto528_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto541_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J379,
                J380,
                J403,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J380, __, __, __, __, __,
                    J379, __, __, __, __, __, __, __, J379, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J403, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J379 => goto379_at3(lex),
                Jump::J380 => goto380_at3(lex),
                Jump::J403 => goto403_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto444_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto452_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                146u8 => {
                    lex.bump_unchecked(3usize);
                    goto378_x(lex)
                }
                180u8 => goto379_at3(lex),
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto525_at2<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J378,
                J403,
                J380,
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
                    __, __, __, __, __, __, __, __, __, J378, __, J380, __, J403, __, __, __, __,
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
                Jump::J378 => {
                    lex.bump_unchecked(3usize);
                    goto378_x(lex)
                }
                Jump::J403 => goto403_at3(lex),
                Jump::J380 => goto380_at3(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto543_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J515,
                J528,
                J541,
                J444,
                J452,
                J525,
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
                    __, __, __, __, J452, J515, __, __, __, __, J525, __, __, __, __, __, __, J528,
                    J541, J444, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J515 => goto515_at2(lex),
                Jump::J528 => goto528_at2(lex),
                Jump::J541 => goto541_at2(lex),
                Jump::J444 => goto444_at2(lex),
                Jump::J452 => goto452_at2(lex),
                Jump::J525 => goto525_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto23_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorIdentifier));
        }
        #[inline]
        fn pattern4(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 2 > 0
        }
        #[inline]
        fn goto24_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern4, goto23_ctx23_x(lex));
        }
        #[inline]
        fn goto7_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordModule));
        }
        #[inline]
        fn goto585_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto7_ctx24_x(lex),
            };
            match byte {
                byte if pattern4(byte) => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto7_ctx24_x(lex),
            }
        }
        #[inline]
        fn goto584_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"odule") => {
                    lex.bump_unchecked(5usize);
                    goto585_ctx24_x(lex)
                }
                _ => goto24_ctx23_x(lex),
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
        fn goto582_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(&[10u8]) => {
                    lex.bump_unchecked(1usize);
                    goto2_ctx3_x(lex)
                }
                _ => goto3_ctx3_x(lex),
            }
        }
        #[inline]
        fn goto20_ctx20_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Comment));
        }
        #[inline]
        fn pattern5(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 4 > 0
        }
        #[inline]
        fn goto21_ctx20_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern5, goto20_ctx20_x(lex));
        }
        #[inline]
        fn goto550_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(b"/") => {
                    lex.bump_unchecked(2usize);
                    goto21_ctx20_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto379_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto37_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx33_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto446_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto563_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto566_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto561_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto569_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto562_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto568_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto567_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
                J569,
                J562,
                J568,
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
                    __, __, __, __, __, __, __, __, __, J563, J566, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J561, J566, __, __, __, __, J568, __, J566, __, __, J569, __, __, J566, J562,
                    __, __, J569, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at2_ctx26_x(lex),
                Jump::J566 => goto566_at2_ctx26_x(lex),
                Jump::J561 => goto561_at2_ctx26_x(lex),
                Jump::J569 => goto569_at2_ctx26_x(lex),
                Jump::J562 => goto562_at2_ctx26_x(lex),
                Jump::J568 => goto568_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto572_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(1usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto563_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto562_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto566_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto579_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J562,
                J566,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J563, __, __, __, __, __,
                    J562, __, __, __, __, __, __, __, J562, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J566, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at3_ctx26_x(lex),
                Jump::J562 => goto562_at3_ctx26_x(lex),
                Jump::J566 => goto566_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto561_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto574_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                180u8 => goto562_at3_ctx26_x(lex),
                146u8 => goto561_at3_ctx26_x(lex),
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto577_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
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
                    __, __, __, __, __, __, __, __, __, J561, __, J563, __, J566, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at3_ctx26_x(lex),
                Jump::J566 => goto566_at3_ctx26_x(lex),
                Jump::J561 => goto561_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto580_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto565_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto576_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto575_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
                J562,
                J565,
                J576,
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
                    __, __, __, __, __, __, __, __, __, J565, __, J562, J576, __, __, J566, __, __,
                    __, J562, __, __, __, __, __, J566, __, J566, __, __, __, __, __, J566, __,
                    J563, J562, __, __, __, __, __, __, J561, __, J566, __, __, __, __, __, __, __,
                    __, __, __, __, J566, __, __, __, J566, J561, __, __, __, __, __, __, J566, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at3_ctx26_x(lex),
                Jump::J566 => goto566_at3_ctx26_x(lex),
                Jump::J561 => goto561_at3_ctx26_x(lex),
                Jump::J562 => goto562_at3_ctx26_x(lex),
                Jump::J565 => goto565_at3_ctx26_x(lex),
                Jump::J576 => goto576_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto578_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto573_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J579,
                J574,
                J577,
                J580,
                J575,
                J578,
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
                    __, __, __, __, J574, J575, __, __, __, __, J577, __, __, __, __, __, __, J578,
                    J579, J580, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J579 => goto579_at2_ctx26_x(lex),
                Jump::J574 => goto574_at2_ctx26_x(lex),
                Jump::J577 => goto577_at2_ctx26_x(lex),
                Jump::J580 => goto580_at2_ctx26_x(lex),
                Jump::J575 => goto575_at2_ctx26_x(lex),
                Jump::J578 => goto578_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto571_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto570_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
                J571,
                J562,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J561, __, __, __, __, __, __,
                    __, __, __, __, J566, J563, __, __, J571, __, J566, __, __, __, __, __, J562,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at2_ctx26_x(lex),
                Jump::J566 => goto566_at2_ctx26_x(lex),
                Jump::J561 => goto561_at2_ctx26_x(lex),
                Jump::J571 => goto571_at2_ctx26_x(lex),
                Jump::J562 => goto562_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto65_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto40_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto46_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto69_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J46,
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
                    __, __, __, __, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46, __, J46,
                    __, J46, __, J46, __, J46, __, J65, __, J65, J40, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at3_ctx26_x(lex),
                Jump::J40 => goto40_at3_ctx26_x(lex),
                Jump::J46 => goto46_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto41_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto65_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto42_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto203_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J41,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J42, __, __, __, __, __,
                    J41, __, __, __, __, __, __, __, J41, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J65, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J41 => goto41_at4_ctx26_x(lex),
                Jump::J65 => goto65_at4_ctx26_x(lex),
                Jump::J42 => goto42_at4_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto106_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto190_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto40_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto126_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto46_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto177_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J40,
                J126,
                J41,
                J46,
                J65,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J46, __, J41, J126, __, __, J65, __, __,
                    __, J41, __, __, __, __, __, J65, __, J65, __, __, __, __, __, J65, __, J42,
                    J41, __, __, __, __, __, __, J40, __, J65, __, __, __, __, __, __, __, __, __,
                    __, __, J65, __, __, __, J65, J40, __, __, __, __, __, __, J65, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J40 => goto40_at4_ctx26_x(lex),
                Jump::J126 => goto126_at4_ctx26_x(lex),
                Jump::J41 => goto41_at4_ctx26_x(lex),
                Jump::J46 => goto46_at4_ctx26_x(lex),
                Jump::J65 => goto65_at4_ctx26_x(lex),
                Jump::J42 => goto42_at4_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto187_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J65,
                J40,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J40, __, J42, __, J65, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J65 => goto65_at4_ctx26_x(lex),
                Jump::J40 => goto40_at4_ctx26_x(lex),
                Jump::J42 => goto42_at4_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto114_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                180u8 => goto41_at4_ctx26_x(lex),
                146u8 => goto40_at4_ctx26_x(lex),
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto205_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J203,
                J106,
                J190,
                J177,
                J187,
                J114,
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
                    __, __, __, __, J114, J177, __, __, __, __, J187, __, __, __, __, __, __, J190,
                    J203, J106, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J203 => goto203_at3_ctx26_x(lex),
                Jump::J106 => goto106_at3_ctx26_x(lex),
                Jump::J190 => goto190_at3_ctx26_x(lex),
                Jump::J177 => goto177_at3_ctx26_x(lex),
                Jump::J187 => goto187_at3_ctx26_x(lex),
                Jump::J114 => goto114_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto108_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto79_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto86_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto41_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto42_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto94_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J79,
                J86,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, J42, J65, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J40, J65, __, __, __, __, J79, __, J65, __, __, J86, __, __, J65, J41, __, __,
                    J86, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J79 => goto79_at3_ctx26_x(lex),
                Jump::J86 => goto86_at3_ctx26_x(lex),
                Jump::J65 => goto65_at3_ctx26_x(lex),
                Jump::J40 => goto40_at3_ctx26_x(lex),
                Jump::J41 => goto41_at3_ctx26_x(lex),
                Jump::J42 => goto42_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto41_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto40_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto103_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto107_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J103,
                J65,
                J40,
                J41,
                J42,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J40, __, __, __, __, __, __,
                    __, __, __, __, J65, J42, __, __, J103, __, J65, __, __, __, __, __, J41, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J103 => goto103_at3_ctx26_x(lex),
                Jump::J65 => goto65_at3_ctx26_x(lex),
                Jump::J40 => goto40_at3_ctx26_x(lex),
                Jump::J41 => goto41_at3_ctx26_x(lex),
                Jump::J42 => goto42_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto42_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto206_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J69,
                J39,
                J205,
                J108,
                J94,
                J41,
                J40,
                J107,
                J42,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J39, J39, J39, J39, J39, J39, J39, J39, J39,
                    J39, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J40, __, J41, __, __, __, J42, J69, J94, __, __, __, __, __, __, __, __,
                    J107, __, __, __, __, J108, J205, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J69 => goto69_at2_ctx26_x(lex),
                Jump::J39 => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                Jump::J205 => goto205_at2_ctx26_x(lex),
                Jump::J108 => goto108_at2_ctx26_x(lex),
                Jump::J94 => goto94_at2_ctx26_x(lex),
                Jump::J41 => goto41_at2_ctx26_x(lex),
                Jump::J40 => goto40_at2_ctx26_x(lex),
                Jump::J107 => goto107_at2_ctx26_x(lex),
                Jump::J42 => goto42_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto561_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(2usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto562_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto565_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto564_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J566,
                J565,
                J561,
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
                    __, __, __, __, __, J565, __, J565, __, J565, __, J565, __, J565, __, J565, __,
                    J565, __, J565, __, J565, __, J565, __, J566, __, J566, J561, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J566 => goto566_at2_ctx26_x(lex),
                Jump::J565 => goto565_at2_ctx26_x(lex),
                Jump::J561 => goto561_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto557_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J563,
                J567,
                J572,
                J573,
                J570,
                J31,
                J206,
                J561,
                J562,
                J557,
                J564,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, J31, __, J37, J206, __, J557, J557, J557, J557, J557, J557, J557,
                    J557, J557, J557, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, J561, __, J562, __, __, __, J563, J564, J567, __, __, __, __,
                    __, __, __, __, J570, __, __, __, __, J572, J573, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto26_ctx26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at1_ctx26_x(lex),
                Jump::J563 => goto563_at1_ctx26_x(lex),
                Jump::J567 => goto567_at1_ctx26_x(lex),
                Jump::J572 => goto572_at1_ctx26_x(lex),
                Jump::J573 => goto573_at1_ctx26_x(lex),
                Jump::J570 => goto570_at1_ctx26_x(lex),
                Jump::J31 => goto31_at1_ctx26_x(lex),
                Jump::J206 => goto206_at1_ctx26_x(lex),
                Jump::J561 => goto561_at1_ctx26_x(lex),
                Jump::J562 => goto562_at1_ctx26_x(lex),
                Jump::J557 => {
                    lex.bump_unchecked(1usize);
                    goto557_ctx26_x(lex)
                }
                Jump::J564 => goto564_at1_ctx26_x(lex),
                Jump::__ => goto26_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto563_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto572_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([188u8, 144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto571_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto570_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
                J571,
                J562,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J561, __, __, __, __, __, __,
                    __, __, __, __, J566, J563, __, __, J571, __, J566, __, __, __, __, __, J562,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at3_ctx26_x(lex),
                Jump::J566 => goto566_at3_ctx26_x(lex),
                Jump::J561 => goto561_at3_ctx26_x(lex),
                Jump::J571 => goto571_at3_ctx26_x(lex),
                Jump::J562 => goto562_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto564_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J566,
                J565,
                J561,
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
                    __, __, __, __, __, J565, __, J565, __, J565, __, J565, __, J565, __, J565, __,
                    J565, __, J565, __, J565, __, J565, __, J566, __, J566, J561, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J566 => goto566_at3_ctx26_x(lex),
                Jump::J565 => goto565_at3_ctx26_x(lex),
                Jump::J561 => goto561_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto569_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto568_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(4usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto567_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
                J569,
                J562,
                J568,
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
                    __, __, __, __, __, __, __, __, __, J563, J566, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J561, J566, __, __, __, __, J568, __, J566, __, __, J569, __, __, J566, J562,
                    __, __, J569, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at3_ctx26_x(lex),
                Jump::J566 => goto566_at3_ctx26_x(lex),
                Jump::J561 => goto561_at3_ctx26_x(lex),
                Jump::J569 => goto569_at3_ctx26_x(lex),
                Jump::J562 => goto562_at3_ctx26_x(lex),
                Jump::J568 => goto568_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto563_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto562_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto566_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto579_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J562,
                J566,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J563, __, __, __, __, __,
                    J562, __, __, __, __, __, __, __, J562, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J566, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at4_ctx26_x(lex),
                Jump::J562 => goto562_at4_ctx26_x(lex),
                Jump::J566 => goto566_at4_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto561_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto574_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                180u8 => goto562_at4_ctx26_x(lex),
                146u8 => goto561_at4_ctx26_x(lex),
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto577_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
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
                    __, __, __, __, __, __, __, __, __, J561, __, J563, __, J566, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at4_ctx26_x(lex),
                Jump::J566 => goto566_at4_ctx26_x(lex),
                Jump::J561 => goto561_at4_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto580_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto565_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto576_at4_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(4usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto575_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J566,
                J561,
                J562,
                J565,
                J576,
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
                    __, __, __, __, __, __, __, __, __, J565, __, J562, J576, __, __, J566, __, __,
                    __, J562, __, __, __, __, __, J566, __, J566, __, __, __, __, __, J566, __,
                    J563, J562, __, __, __, __, __, __, J561, __, J566, __, __, __, __, __, __, __,
                    __, __, __, __, J566, __, __, __, J566, J561, __, __, __, __, __, __, J566, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(3usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at4_ctx26_x(lex),
                Jump::J566 => goto566_at4_ctx26_x(lex),
                Jump::J561 => goto561_at4_ctx26_x(lex),
                Jump::J562 => goto562_at4_ctx26_x(lex),
                Jump::J565 => goto565_at4_ctx26_x(lex),
                Jump::J576 => goto576_at4_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto578_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(3usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(5usize);
                    goto557_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto573_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J579,
                J574,
                J577,
                J580,
                J575,
                J578,
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
                    __, __, __, __, J574, J575, __, __, __, __, J577, __, __, __, __, __, __, J578,
                    J579, J580, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J579 => goto579_at3_ctx26_x(lex),
                Jump::J574 => goto574_at3_ctx26_x(lex),
                Jump::J577 => goto577_at3_ctx26_x(lex),
                Jump::J580 => goto580_at3_ctx26_x(lex),
                Jump::J575 => goto575_at3_ctx26_x(lex),
                Jump::J578 => goto578_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto556_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J563,
                J572,
                J570,
                J561,
                J564,
                J567,
                J562,
                J557,
                J573,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, J557, J557, J557, J557, J557, J557, J557, J557,
                    J557, J557, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, J561, __, J562, __, __, __, J563, J564, J567, __, __, __, __, __, __,
                    __, __, J570, __, __, __, __, J572, J573, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J563 => goto563_at2_ctx26_x(lex),
                Jump::J572 => goto572_at2_ctx26_x(lex),
                Jump::J570 => goto570_at2_ctx26_x(lex),
                Jump::J561 => goto561_at2_ctx26_x(lex),
                Jump::J564 => goto564_at2_ctx26_x(lex),
                Jump::J567 => goto567_at2_ctx26_x(lex),
                Jump::J562 => goto562_at2_ctx26_x(lex),
                Jump::J557 => {
                    lex.bump_unchecked(2usize);
                    goto557_ctx26_x(lex)
                }
                Jump::J573 => goto573_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto378_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some([160u8..=169u8]) => {
                    lex.bump_unchecked(1usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto403_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto379_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto380_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto441_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern3(byte) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto445_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J403,
                J379,
                J378,
                J380,
                J441,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, J378, __, __, __, __, __, __,
                    __, __, __, __, J403, J380, __, __, J441, __, J403, __, __, __, __, __, J379,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J403 => goto403_at2_ctx26_x(lex),
                Jump::J379 => goto379_at2_ctx26_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_ctx26_x(lex)
                }
                Jump::J380 => goto380_at2_ctx26_x(lex),
                Jump::J441 => goto441_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto379_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto384_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto464_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([182u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto403_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([144u8..=153u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto380_at3_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(3usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto515_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J379,
                J384,
                J464,
                J403,
                J378,
                J380,
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
                    __, __, __, __, __, __, __, __, __, J384, __, J379, J464, __, __, J403, __, __,
                    __, J379, __, __, __, __, __, J403, __, J403, __, __, __, __, __, J403, __,
                    J380, J379, __, __, __, __, __, __, J378, __, J403, __, __, __, __, __, __, __,
                    __, __, __, __, J403, __, __, __, J403, J378, __, __, __, __, __, __, J403, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J379 => goto379_at3_ctx26_x(lex),
                Jump::J384 => goto384_at3_ctx26_x(lex),
                Jump::J464 => goto464_at3_ctx26_x(lex),
                Jump::J403 => goto403_at3_ctx26_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(3usize);
                    goto378_ctx26_x(lex)
                }
                Jump::J380 => goto380_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto528_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([159u8, 142u8..=191u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto541_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J379,
                J380,
                J403,
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
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J380, __, __, __, __, __,
                    J379, __, __, __, __, __, __, __, J379, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, J403, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J379 => goto379_at3_ctx26_x(lex),
                Jump::J380 => goto380_at3_ctx26_x(lex),
                Jump::J403 => goto403_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto444_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 2usize]>(2usize) {
                Some([175u8, 176u8..=185u8]) => {
                    lex.bump_unchecked(4usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto452_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                146u8 => {
                    lex.bump_unchecked(3usize);
                    goto378_ctx26_x(lex)
                }
                180u8 => goto379_at3_ctx26_x(lex),
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto525_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J378,
                J403,
                J380,
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
                    __, __, __, __, __, __, __, __, __, J378, __, J380, __, J403, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J378 => {
                    lex.bump_unchecked(3usize);
                    goto378_ctx26_x(lex)
                }
                Jump::J403 => goto403_at3_ctx26_x(lex),
                Jump::J380 => goto380_at3_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto543_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J515,
                J528,
                J541,
                J444,
                J452,
                J525,
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
                    __, __, __, __, J452, J515, __, __, __, __, J525, __, __, __, __, __, __, J528,
                    J541, J444, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J515 => goto515_at2_ctx26_x(lex),
                Jump::J528 => goto528_at2_ctx26_x(lex),
                Jump::J541 => goto541_at2_ctx26_x(lex),
                Jump::J444 => goto444_at2_ctx26_x(lex),
                Jump::J452 => goto452_at2_ctx26_x(lex),
                Jump::J525 => goto525_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto379_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([176u8..=185u8]) => {
                    lex.bump_unchecked(2usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto424_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto417_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto432_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J403,
                J424,
                J379,
                J378,
                J380,
                J417,
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
                    __, __, __, __, __, __, __, __, __, J380, J403, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J378, J403, __, __, __, __, J417, __, J403, __, __, J424, __, __, J403, J379,
                    __, __, J424, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J403 => goto403_at2_ctx26_x(lex),
                Jump::J424 => goto424_at2_ctx26_x(lex),
                Jump::J379 => goto379_at2_ctx26_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_ctx26_x(lex)
                }
                Jump::J380 => goto380_at2_ctx26_x(lex),
                Jump::J417 => goto417_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto380_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto384_at2_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto407_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J378,
                J403,
                J384,
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
                    __, __, __, __, __, J384, __, J384, __, J384, __, J384, __, J384, __, J384, __,
                    J384, __, J384, __, J384, __, J384, __, J403, __, J403, J378, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __,
                ]
            };
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_ctx26_x(lex)
                }
                Jump::J403 => goto403_at2_ctx26_x(lex),
                Jump::J384 => goto384_at2_ctx26_x(lex),
                Jump::__ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto551_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J446,
                J556,
                J378,
                J445,
                J543,
                J379,
                J31,
                J432,
                J551,
                J24,
                J380,
                J407,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, J31, __, J37, J556, __, J551, J551, J551, J551, J551, J551, J551,
                    J551, J551, J551, __, __, __, __, __, __, __, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, __, __, __, __, __, __, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J378,
                    __, J379, __, __, __, J380, J407, J432, __, __, __, __, __, __, __, __, J445,
                    __, __, __, __, J446, J543, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto26_ctx26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at1_ctx26_x(lex),
                Jump::J446 => goto446_at1_ctx26_x(lex),
                Jump::J556 => goto556_at1_ctx26_x(lex),
                Jump::J378 => {
                    lex.bump_unchecked(1usize);
                    goto378_ctx26_x(lex)
                }
                Jump::J445 => goto445_at1_ctx26_x(lex),
                Jump::J543 => goto543_at1_ctx26_x(lex),
                Jump::J379 => goto379_at1_ctx26_x(lex),
                Jump::J31 => goto31_at1_ctx26_x(lex),
                Jump::J432 => goto432_at1_ctx26_x(lex),
                Jump::J551 => {
                    lex.bump_unchecked(1usize);
                    goto551_ctx26_x(lex)
                }
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                Jump::J380 => goto380_at1_ctx26_x(lex),
                Jump::J407 => goto407_at1_ctx26_x(lex),
                Jump::__ => goto26_ctx26_x(lex),
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
        fn pattern6(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 8 > 0
        }
        #[inline]
        fn goto548_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto17_ctx17_x(lex),
            };
            match byte {
                b'"' => {
                    lex.bump_unchecked(1usize);
                    goto13_ctx17_x(lex)
                }
                byte if pattern6(byte) => {
                    lex.bump_unchecked(1usize);
                    goto548_ctx17_x(lex)
                }
                _ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto547_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J547,
                J9,
                J548,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, __, J547, J547, __,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J9, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J548, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547, J547,
                    J547, J547, J547, J547, J547, J547, J547, J547,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto17_ctx17_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J547 => {
                    lex.bump_unchecked(1usize);
                    goto547_ctx17_x(lex)
                }
                Jump::J9 => {
                    lex.bump_unchecked(1usize);
                    goto9_ctx17_x(lex)
                }
                Jump::J548 => {
                    lex.bump_unchecked(1usize);
                    goto548_ctx17_x(lex)
                }
                Jump::__ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto8_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordFennec));
        }
        #[inline]
        fn goto588_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto8_ctx24_x(lex),
            };
            match byte {
                byte if pattern4(byte) => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto8_ctx24_x(lex),
            }
        }
        #[inline]
        fn goto587_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"ennec") => {
                    lex.bump_unchecked(5usize);
                    goto588_ctx24_x(lex)
                }
                _ => goto24_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto424_at2<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(2usize) {
                Some(byte) => byte,
                None => return _error(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto417_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([134u8..=143u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto432_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J403,
                J424,
                J379,
                J378,
                J380,
                J417,
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
                    __, __, __, __, __, __, __, __, __, J380, J403, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    J378, J403, __, __, __, __, J417, __, J403, __, __, J424, __, __, J403, J379,
                    __, __, J424, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J403 => goto403_at2(lex),
                Jump::J424 => goto424_at2(lex),
                Jump::J379 => goto379_at2(lex),
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_x(lex)
                }
                Jump::J380 => goto380_at2(lex),
                Jump::J417 => goto417_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto384_at2<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(2usize) {
                Some([166u8..=175u8]) => {
                    lex.bump_unchecked(3usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto407_at1<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J378,
                J403,
                J384,
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
                    __, __, __, __, __, J384, __, J384, __, J384, __, J384, __, J384, __, J384, __,
                    J384, __, J384, __, J384, __, J384, __, J403, __, J403, J378, __, __, __, __,
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
                Jump::J378 => {
                    lex.bump_unchecked(2usize);
                    goto378_x(lex)
                }
                Jump::J403 => goto403_at2(lex),
                Jump::J384 => goto384_at2(lex),
                Jump::__ => _error(lex),
            }
        }
        #[inline]
        fn goto380_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([128u8..=137u8]) => {
                    lex.bump_unchecked(2usize);
                    goto377_ctx376_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto589<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J5,
                J446,
                J445,
                J1,
                J378,
                J543,
                J584,
                J582,
                J550,
                J379,
                J551,
                J24,
                J547,
                J587,
                J432,
                J407,
                J380,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, J5, J1, __, __, J582, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J5, __, J547, __, __, __,
                    __, __, __, __, __, __, __, __, __, J550, J551, J551, J551, J551, J551, J551,
                    J551, J551, J551, J551, __, __, __, __, __, __, __, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, __, __, __, __, __, __, J24, J24, J24, J24, J24, J587,
                    J24, J24, J24, J24, J24, J24, J584, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, J378, __, J379, __, __, __, J380, J407, J432, __, __, __, __, __, __, __,
                    __, J445, __, __, __, __, J446, J543, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __,
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
                Jump::J446 => goto446_at1(lex),
                Jump::J445 => goto445_at1(lex),
                Jump::J1 => {
                    lex.bump_unchecked(1usize);
                    goto1_x(lex)
                }
                Jump::J378 => {
                    lex.bump_unchecked(1usize);
                    goto378_x(lex)
                }
                Jump::J543 => goto543_at1(lex),
                Jump::J584 => {
                    lex.bump_unchecked(1usize);
                    goto584_ctx24_x(lex)
                }
                Jump::J582 => {
                    lex.bump_unchecked(1usize);
                    goto582_ctx3_x(lex)
                }
                Jump::J550 => goto550_at1(lex),
                Jump::J379 => goto379_at1(lex),
                Jump::J551 => {
                    lex.bump_unchecked(1usize);
                    goto551_ctx26_x(lex)
                }
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                Jump::J547 => {
                    lex.bump_unchecked(1usize);
                    goto547_ctx17_x(lex)
                }
                Jump::J587 => {
                    lex.bump_unchecked(1usize);
                    goto587_ctx24_x(lex)
                }
                Jump::J432 => goto432_at1(lex),
                Jump::J407 => goto407_at1(lex),
                Jump::J380 => goto380_at1(lex),
                Jump::__ => _error(lex),
            }
        }
        goto589(lex)
    }
}
