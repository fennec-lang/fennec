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
            12, 14, 12, 12, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 12, 12, 12, 12, 12, 12, 12, 15,
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
        fn goto23_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::ErrorIdentifier));
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn goto24_ctx23_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern1, goto23_ctx23_x(lex));
        }
        #[inline]
        fn goto8_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordFennec));
        }
        #[inline]
        fn goto70_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto8_ctx24_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto8_ctx24_x(lex),
            }
        }
        #[inline]
        fn goto69_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"ennec") => {
                    lex.bump_unchecked(5usize);
                    goto70_ctx24_x(lex)
                }
                _ => goto24_ctx23_x(lex),
            }
        }
        #[inline]
        fn goto1_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Newline));
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
        fn goto64_ctx3_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 1usize]>() {
                Some(&[10u8]) => {
                    lex.bump_unchecked(1usize);
                    goto2_ctx3_x(lex)
                }
                _ => goto3_ctx3_x(lex),
            }
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
        fn goto26_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn pattern2(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 2 > 0
        }
        #[inline]
        fn goto28_ctx27_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern2, goto27_ctx26_x(lex));
        }
        #[inline]
        fn goto29_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
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
            _fast_loop!(lex, pattern2, goto27_ctx26_x(lex));
        }
        #[inline]
        fn goto31_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
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
                byte if pattern2(byte) => {
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
            _fast_loop!(lex, pattern2, goto33_ctx32_x(lex));
        }
        #[inline]
        fn goto35_at1_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto32_ctx26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
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
            _fast_loop!(lex, pattern2, goto33_ctx32_x(lex));
        }
        #[inline]
        fn goto37_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
                    lex.bump_unchecked(2usize);
                    goto36_ctx33_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto26_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Version));
        }
        #[inline]
        fn goto37_at1_ctx32_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read_at::<u8>(1usize) {
                Some(byte) => byte,
                None => return goto32_ctx26_x(lex),
            };
            match byte {
                byte if pattern2(byte) => {
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
                byte if pattern2(byte) => {
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
                byte if pattern2(byte) => {
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
        fn pattern3(byte: u8) -> bool {
            match byte {
                b'0'..=b'9' => true,
                _ => false,
            }
        }
        #[inline]
        fn goto39_ctx38_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern3, goto38_ctx38_x(lex));
        }
        #[inline]
        fn goto55_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([b'0'..=b'9']) => {
                    lex.bump_unchecked(2usize);
                    goto39_ctx38_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto59_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J31,
                J55,
                J59,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, J31, __, J37, J55, __, J59, J59, J59, J59, J59, J59, J59, J59, J59,
                    J59, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                None => return goto26_ctx26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at1_ctx26_x(lex),
                Jump::J31 => goto31_at1_ctx26_x(lex),
                Jump::J55 => goto55_at1_ctx26_x(lex),
                Jump::J59 => {
                    lex.bump_unchecked(1usize);
                    goto59_ctx26_x(lex)
                }
                Jump::__ => goto26_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto58_at1_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some([b'0'..=b'9']) => {
                    lex.bump_unchecked(2usize);
                    goto59_ctx26_x(lex)
                }
                _ => goto26_x(lex),
            }
        }
        #[inline]
        fn goto51_ctx26_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J37,
                J31,
                J24,
                J51,
                J58,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, J31, __, J37, J58, __, J51, J51, J51, J51, J51, J51, J51, J51, J51,
                    J51, __, __, __, __, __, __, __, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, __, __, __, __, __, __, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __,
                ]
            };
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto26_ctx26_x(lex),
            };
            match LUT[byte as usize] {
                Jump::J37 => goto37_at1_ctx26_x(lex),
                Jump::J31 => goto31_at1_ctx26_x(lex),
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                Jump::J51 => {
                    lex.bump_unchecked(1usize);
                    goto51_ctx26_x(lex)
                }
                Jump::J58 => goto58_at1_ctx26_x(lex),
                Jump::__ => goto26_ctx26_x(lex),
            }
        }
        #[inline]
        fn goto20_ctx20_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::Comment));
        }
        #[inline]
        fn pattern4(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 4 > 0
        }
        #[inline]
        fn goto21_ctx20_x<'s>(lex: &mut Lexer<'s>) {
            _fast_loop!(lex, pattern4, goto20_ctx20_x(lex));
        }
        #[inline]
        fn goto50_at1<'s>(lex: &mut Lexer<'s>) {
            match lex.read_at::<&[u8; 1usize]>(1usize) {
                Some(b"/") => {
                    lex.bump_unchecked(2usize);
                    goto21_ctx20_x(lex)
                }
                _ => _error(lex),
            }
        }
        #[inline]
        fn goto7_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            lex.set(Ok(LogosToken::KeywordModule));
        }
        #[inline]
        fn goto67_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            let byte = match lex.read::<u8>() {
                Some(byte) => byte,
                None => return goto7_ctx24_x(lex),
            };
            match byte {
                byte if pattern1(byte) => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                _ => goto7_ctx24_x(lex),
            }
        }
        #[inline]
        fn goto66_ctx24_x<'s>(lex: &mut Lexer<'s>) {
            match lex.read::<&[u8; 5usize]>() {
                Some(b"odule") => {
                    lex.bump_unchecked(5usize);
                    goto67_ctx24_x(lex)
                }
                _ => goto24_ctx23_x(lex),
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
            COMPACT_TABLE_0[byte as usize] & 8 > 0
        }
        #[inline]
        fn goto48_ctx17_x<'s>(lex: &mut Lexer<'s>) {
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
                    goto48_ctx17_x(lex)
                }
                _ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto47_ctx17_x<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J9,
                J48,
                J47,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, __, J47, J47, __, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J9, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J48, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
                    J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47, J47,
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
                Jump::J48 => {
                    lex.bump_unchecked(1usize);
                    goto48_ctx17_x(lex)
                }
                Jump::J47 => {
                    lex.bump_unchecked(1usize);
                    goto47_ctx17_x(lex)
                }
                Jump::__ => goto17_ctx17_x(lex),
            }
        }
        #[inline]
        fn goto71<'s>(lex: &mut Lexer<'s>) {
            enum Jump {
                __,
                J5,
                J69,
                J1,
                J64,
                J51,
                J50,
                J66,
                J47,
                J24,
            }
            const LUT: [Jump; 256] = {
                use Jump::*;
                [
                    __, __, __, __, __, __, __, __, __, J5, J1, __, __, J64, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, J5, __, J47, __, __, __,
                    __, __, __, __, __, __, __, __, __, J50, J51, J51, J51, J51, J51, J51, J51,
                    J51, J51, J51, __, __, __, __, __, __, __, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, J24, __, __, __, __, __, __, J24, J24, J24, J24, J24, J69, J24, J24,
                    J24, J24, J24, J24, J66, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24, J24,
                    J24, J24, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
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
                Jump::J69 => {
                    lex.bump_unchecked(1usize);
                    goto69_ctx24_x(lex)
                }
                Jump::J1 => {
                    lex.bump_unchecked(1usize);
                    goto1_x(lex)
                }
                Jump::J64 => {
                    lex.bump_unchecked(1usize);
                    goto64_ctx3_x(lex)
                }
                Jump::J51 => {
                    lex.bump_unchecked(1usize);
                    goto51_ctx26_x(lex)
                }
                Jump::J50 => goto50_at1(lex),
                Jump::J66 => {
                    lex.bump_unchecked(1usize);
                    goto66_ctx24_x(lex)
                }
                Jump::J47 => {
                    lex.bump_unchecked(1usize);
                    goto47_ctx17_x(lex)
                }
                Jump::J24 => {
                    lex.bump_unchecked(1usize);
                    goto24_ctx23_x(lex)
                }
                Jump::__ => _error(lex),
            }
        }
        goto71(lex)
    }
}
