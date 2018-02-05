use phf;


///@typedef {Object} DefaultOptions
///@property {Boolean} [use_obsolete_kana=false] - Set to true to use obsolete characters, such as ゐ and ゑ.
///@example
///to_hiragana('we', { use_obsolete_kana: true })
/// => 'ゑ'
///@property {Boolean} [pass_romaji=false] - Set to true to pass romaji when using mixed syllabaries with to_katakana() or to_hiragana()
///@example
///to_hiragana('only convert the katakana: ヒラガナ', { pass_romaji: true })
/// => "only convert the katakana: ひらがな"
///@property {Boolean} [upcase_katakana=false] - Set to true to convert katakana to uppercase using to_romaji()
///@example
///to_romaji('ひらがな カタカナ', { upcase_katakana: true })
/// => "hiragana KATAKANA"
///@property {Boolean} [imemode=false] - Set to true, 'to_hiragana', or 'to_katakana' to handle conversion from a text input while it is being typed


// 
// ///Default config for WanaKana, user passed options will be merged with this
// ///@type {DefaultOptions}
// ///@ignore
//  */
// export const DEFAULT_OPTIONS = {
//   use_obsolete_kana: false,
//   pass_romaji: false,
//   upcase_katakana: false,
//   imemode: false,
// };

// CharCode References
// http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
// http://unicode-table.com

pub const CJK_SYMBOLS_PUNCTUATION: [u32; 2] = [0x3000, 0x303F];
pub const KATAKANA_PUNCTUATION: [u32; 2] = [0x30FB, 0x30FC];
pub const HIRAGANA_CHARS: [u32; 2] = [0x3040, 0x309F];
pub const KATAKANA_CHARS: [u32; 2] = [0x30A0, 0x30FF];
pub const ZENKAKU_NUMBERS: [u32; 2] = [0xFF10, 0xFF19];
pub const ZENKAKU_PUNCTUATION_1: [u32; 2] = [0xFF01, 0xFF0F];
pub const ZENKAKU_PUNCTUATION_2: [u32; 2] = [0xFF1A, 0xFF1F];
pub const ZENKAKU_PUNCTUATION_3: [u32; 2] = [0xFF3B, 0xFF3F];
pub const ZENKAKU_PUNCTUATION_4: [u32; 2] = [0xFF5B, 0xFF60];
pub const ZENKAKU_SYMBOLS_CURRENCY: [u32; 2] = [0xFFE0, 0xFFEE];
pub const KANA_PUNCTUATION: [u32; 2] = [0xFF61, 0xFF65];
pub const HANKAKU_KATAKANA: [u32; 2] = [0xFF66, 0xFF9F];
pub const COMMON_CJK: [u32; 2] = [0x4E00, 0x9FFF];
pub const RARE_CJK: [u32; 2] = [0x3400, 0x4DBF];
pub const LATIN_NUMBERS: [u32; 2] = [0x0030, 0x0039];
pub const MODERN_ENGLISH: [u32; 2] = [0x0000, 0x007f];
pub const HEPBURN_MACRON_RANGES: [[u32; 2]; 5] = [
    [0x0100, 0x0101], // Ā ā
    [0x0112, 0x0113], // Ē ē
    [0x012a, 0x012b], // Ī ī
    [0x014c, 0x014d], // Ō ō
    [0x016a, 0x016b], // Ū ū
];
pub const SMART_QUOTE_RANGES: [[u32; 2]; 2] = [
    [0x2018, 0x2019], // ‘ ’
    [0x201C, 0x201D], // “ ”
];

pub const SMART_QUOTE_RANGES2: [[u32; 2]; 2] = [
    KATAKANA_PUNCTUATION, // ‘ ’
    [0x201C, 0x201D],     // “ ”
];

// // pub const FULL_LATIN_RANGES = [
// //   [0x0001-0x007F],
// //   [0x0080-0x00FF],
// //   [0x0100-0x017F],
// //   [0x0180-0x024F],
// // ];

pub const JA_PUNCTUATION_RANGES: [[u32; 2]; 8] = [
    CJK_SYMBOLS_PUNCTUATION,
    KANA_PUNCTUATION,
    KATAKANA_PUNCTUATION,
    ZENKAKU_PUNCTUATION_1,
    ZENKAKU_PUNCTUATION_2,
    ZENKAKU_PUNCTUATION_3,
    ZENKAKU_PUNCTUATION_4,
    ZENKAKU_SYMBOLS_CURRENCY,
];

pub const KANA_RANGES: [[u32; 2]; 4] = [
    // const KANA_RANGES = [
    HIRAGANA_CHARS,
    KATAKANA_CHARS,
    KANA_PUNCTUATION,
    HANKAKU_KATAKANA,
];

lazy_static! {
  // *
  // ///All Japanese unicode start and end ranges
  // ///Includes full-width punctuation and number ranges.
  // ///Incudes latin numbers since they are used in Japanese text as well.
  // ///@type {Array}
  // ///@ignore

    pub static ref JAPANESE_RANGES: Vec<[u32; 2]> = {
        let mut m = vec![LATIN_NUMBERS, ZENKAKU_NUMBERS, COMMON_CJK, RARE_CJK,];
        m.extend(&KANA_RANGES);
        m.extend(&JA_PUNCTUATION_RANGES);
        m
    };
    // 
    ///Basic Latin unicode regex, for determining Romaji + Hepburn romanisation
    ///Includes upper/lowercase long vowels like "ā, ī, ū, ē, ō"
    ///Includes smart quotes ‘’ “”
    ///@type {Array}
    ///@ignore
    // */
    pub static ref ROMAJI_RANGES: Vec<[u32; 2]> = {
        let mut m = vec![MODERN_ENGLISH,];
        m.extend(&HEPBURN_MACRON_RANGES);
        m.extend(&SMART_QUOTE_RANGES);
        m
    };
    pub static ref EN_PUNCTUATION_RANGES: Vec<[u32; 2]> = {
        let mut m = vec![[0x21, 0x2F], [0x3A, 0x3F], [0x5B, 0x60], [0x7B, 0x7E],];
        m.extend(&SMART_QUOTE_RANGES);
        m
    };

}

pub const LOWERCASE_START: u32 = 0x61;
pub const LOWERCASE_END: u32 = 0x7A;
pub const UPPERCASE_START: u32 = 0x41;
pub const UPPERCASE_END: u32 = 0x5A;
pub const LOWERCASE_FULLWIDTH_START: u32 = 0xFF41;
pub const LOWERCASE_FULLWIDTH_END: u32 = 0xFF5A;
pub const UPPERCASE_FULLWIDTH_START: u32 = 0xFF21;
pub const UPPERCASE_FULLWIDTH_END: u32 = 0xFF3A;
pub const HIRAGANA_START: u32 = 0x3041;
pub const HIRAGANA_END: u32 = 0x3096;
pub const KATAKANA_START: u32 = 0x30A1;
pub const KATAKANA_END: u32 = 0x30FC;
// export const KANJI_START = 0x4E00;
// export const KANJI_END = 0x9FAF;
pub const PROLONGED_SOUND_MARK: u32 = 0x30FC;
pub const KANA_SLASH_DOT: u32 = 0x30FB;

pub const LONG_VOWELS: phf::Map<char, char> = phf_map! {
  'a' => 'あ',
  'i' => 'い',
  'u' => 'う',
  'e' => 'え',
  'o' => 'う',
};

pub const FOUR_CHAR_EDGECASES: &'static [&'static str] = &["lts", "chy", "shy"];

pub const FROM_ROMAJI: phf::Map<&'static str, &'static str> = phf_map! {
  "." => "。",
  "," => "、",
  " =>" => "：",
  "/" => "・",
  "!" => "！",
  "?" => "？",
  "~" => "〜",
  "-" => "ー",
  "‘" => "「",
  "’" => "」",
  "“" => "『",
  "”" => "』",
  "[" => "［",
  "]" => "］",
  "(" => "（",
  ")" => "）",
  "{" => "｛",
  "}" => "｝",

  "a" => "あ",
  "i" => "い",
  "u" => "う",
  "e" => "え",
  "o" => "お",
  "yi" => "い",
  "wu" => "う",
  "whu" => "う",
  "xa" => "ぁ",
  "xi" => "ぃ",
  "xu" => "ぅ",
  "xe" => "ぇ",
  "xo" => "ぉ",
  "xyi" => "ぃ",
  "xye" => "ぇ",
  "ye" => "いぇ",
  "wha" => "うぁ",
  "whi" => "うぃ",
  "whe" => "うぇ",
  "who" => "うぉ",
  "wi" => "うぃ",
  "we" => "うぇ",
  "va" => "ゔぁ",
  "vi" => "ゔぃ",
  "vu" => "ゔ",
  "ve" => "ゔぇ",
  "vo" => "ゔぉ",
  "vya" => "ゔゃ",
  "vyi" => "ゔぃ",
  "vyu" => "ゔゅ",
  "vye" => "ゔぇ",
  "vyo" => "ゔょ",
  "ka" => "か",
  "ki" => "き",
  "ku" => "く",
  "ke" => "け",
  "ko" => "こ",
  "lka" => "ヵ",
  "lke" => "ヶ",
  "xka" => "ヵ",
  "xke" => "ヶ",
  "kya" => "きゃ",
  "kyi" => "きぃ",
  "kyu" => "きゅ",
  "kye" => "きぇ",
  "kyo" => "きょ",
  "ca" => "か",
  "ci" => "き",
  "cu" => "く",
  "ce" => "け",
  "co" => "こ",
  "lca" => "ヵ",
  "lce" => "ヶ",
  "xca" => "ヵ",
  "xce" => "ヶ",
  "qya" => "くゃ",
  "qyu" => "くゅ",
  "qyo" => "くょ",
  "qwa" => "くぁ",
  "qwi" => "くぃ",
  "qwu" => "くぅ",
  "qwe" => "くぇ",
  "qwo" => "くぉ",
  "qa" => "くぁ",
  "qi" => "くぃ",
  "qe" => "くぇ",
  "qo" => "くぉ",
  "kwa" => "くぁ",
  "qyi" => "くぃ",
  "qye" => "くぇ",
  "ga" => "が",
  "gi" => "ぎ",
  "gu" => "ぐ",
  "ge" => "げ",
  "go" => "ご",
  "gya" => "ぎゃ",
  "gyi" => "ぎぃ",
  "gyu" => "ぎゅ",
  "gye" => "ぎぇ",
  "gyo" => "ぎょ",
  "gwa" => "ぐぁ",
  "gwi" => "ぐぃ",
  "gwu" => "ぐぅ",
  "gwe" => "ぐぇ",
  "gwo" => "ぐぉ",
  "sa" => "さ",
  "si" => "し",
  "shi" => "し",
  "su" => "す",
  "se" => "せ",
  "so" => "そ",
  "za" => "ざ",
  "zi" => "じ",
  "zu" => "ず",
  "ze" => "ぜ",
  "zo" => "ぞ",
  "ji" => "じ",
  "sya" => "しゃ",
  "syi" => "しぃ",
  "syu" => "しゅ",
  "sye" => "しぇ",
  "syo" => "しょ",
  "sha" => "しゃ",
  "shu" => "しゅ",
  "she" => "しぇ",
  "sho" => "しょ",
  "shya" => "しゃ", // 4 character code
  "shyu" => "しゅ", // 4 character code
  "shye" => "しぇ", // 4 character code
  "shyo" => "しょ", // 4 character code
  "swa" => "すぁ",
  "swi" => "すぃ",
  "swu" => "すぅ",
  "swe" => "すぇ",
  "swo" => "すぉ",
  "zya" => "じゃ",
  "zyi" => "じぃ",
  "zyu" => "じゅ",
  "zye" => "じぇ",
  "zyo" => "じょ",
  "ja" => "じゃ",
  "ju" => "じゅ",
  "je" => "じぇ",
  "jo" => "じょ",
  "jya" => "じゃ",
  "jyi" => "じぃ",
  "jyu" => "じゅ",
  "jye" => "じぇ",
  "jyo" => "じょ",
  "ta" => "た",
  "ti" => "ち",
  "tu" => "つ",
  "te" => "て",
  "to" => "と",
  "chi" => "ち",
  "tsu" => "つ",
  "ltu" => "っ",
  "xtu" => "っ",
  "tya" => "ちゃ",
  "tyi" => "ちぃ",
  "tyu" => "ちゅ",
  "tye" => "ちぇ",
  "tyo" => "ちょ",
  "cha" => "ちゃ",
  "chu" => "ちゅ",
  "che" => "ちぇ",
  "cho" => "ちょ",
  "cya" => "ちゃ",
  "cyi" => "ちぃ",
  "cyu" => "ちゅ",
  "cye" => "ちぇ",
  "cyo" => "ちょ",
  "chya" => "ちゃ", // 4 character code
  "chyu" => "ちゅ", // 4 character code
  "chye" => "ちぇ", // 4 character code
  "chyo" => "ちょ", // 4 character code
  "tsa" => "つぁ",
  "tsi" => "つぃ",
  "tse" => "つぇ",
  "tso" => "つぉ",
  "tha" => "てゃ",
  "thi" => "てぃ",
  "thu" => "てゅ",
  "the" => "てぇ",
  "tho" => "てょ",
  "twa" => "とぁ",
  "twi" => "とぃ",
  "twu" => "とぅ",
  "twe" => "とぇ",
  "two" => "とぉ",
  "da" => "だ",
  "di" => "ぢ",
  "du" => "づ",
  "de" => "で",
  "do" => "ど",
  "dya" => "ぢゃ",
  "dyi" => "ぢぃ",
  "dyu" => "ぢゅ",
  "dye" => "ぢぇ",
  "dyo" => "ぢょ",
  "dha" => "でゃ",
  "dhi" => "でぃ",
  "dhu" => "でゅ",
  "dhe" => "でぇ",
  "dho" => "でょ",
  "dwa" => "どぁ",
  "dwi" => "どぃ",
  "dwu" => "どぅ",
  "dwe" => "どぇ",
  "dwo" => "どぉ",
  "na" => "な",
  "ni" => "に",
  "nu" => "ぬ",
  "ne" => "ね",
  "no" => "の",
  "nya" => "にゃ",
  "nyi" => "にぃ",
  "nyu" => "にゅ",
  "nye" => "にぇ",
  "nyo" => "にょ",
  "ha" => "は",
  "hi" => "ひ",
  "hu" => "ふ",
  "he" => "へ",
  "ho" => "ほ",
  "fu" => "ふ",
  "hya" => "ひゃ",
  "hyi" => "ひぃ",
  "hyu" => "ひゅ",
  "hye" => "ひぇ",
  "hyo" => "ひょ",
  "fya" => "ふゃ",
  "fyu" => "ふゅ",
  "fyo" => "ふょ",
  "fwa" => "ふぁ",
  "fwi" => "ふぃ",
  "fwu" => "ふぅ",
  "fwe" => "ふぇ",
  "fwo" => "ふぉ",
  "fa" => "ふぁ",
  "fi" => "ふぃ",
  "fe" => "ふぇ",
  "fo" => "ふぉ",
  "fyi" => "ふぃ",
  "fye" => "ふぇ",
  "ba" => "ば",
  "bi" => "び",
  "bu" => "ぶ",
  "be" => "べ",
  "bo" => "ぼ",
  "bya" => "びゃ",
  "byi" => "びぃ",
  "byu" => "びゅ",
  "bye" => "びぇ",
  "byo" => "びょ",
  "pa" => "ぱ",
  "pi" => "ぴ",
  "pu" => "ぷ",
  "pe" => "ぺ",
  "po" => "ぽ",
  "pya" => "ぴゃ",
  "pyi" => "ぴぃ",
  "pyu" => "ぴゅ",
  "pye" => "ぴぇ",
  "pyo" => "ぴょ",
  "ma" => "ま",
  "mi" => "み",
  "mu" => "む",
  "me" => "め",
  "mo" => "も",
  "mya" => "みゃ",
  "myi" => "みぃ",
  "myu" => "みゅ",
  "mye" => "みぇ",
  "myo" => "みょ",
  "ya" => "や",
  "yu" => "ゆ",
  "yo" => "よ",
  "xya" => "ゃ",
  "xyu" => "ゅ",
  "xyo" => "ょ",
  "ra" => "ら",
  "ri" => "り",
  "ru" => "る",
  "re" => "れ",
  "ro" => "ろ",
  "rya" => "りゃ",
  "ryi" => "りぃ",
  "ryu" => "りゅ",
  "rye" => "りぇ",
  "ryo" => "りょ",
  "la" => "ら",
  "li" => "り",
  "lu" => "る",
  "le" => "れ",
  "lo" => "ろ",
  "lya" => "りゃ",
  "lyi" => "りぃ",
  "lyu" => "りゅ",
  "lye" => "りぇ",
  "lyo" => "りょ",
  "wa" => "わ",
  "wo" => "を",
  "lwe" => "ゎ",
  "xwa" => "ゎ",
  "n" => "ん",
  "nn" => "ん",
  "n'" => "ん", // n" should equal single ん
  "n " => "ん", // n + space
  "xn" => "ん",
  "ltsu" => "っ",  // 4 character code
};

pub static TO_ROMAJI: phf::Map<&'static str, &'static str> = phf_map! {
  "　" => " ",
  "！" => "!",
  "？" => "?",
  "。" => ".",
  "：" => " =>",
  "・" => "/",
  "、" => ",",
  "〜" => "~",
  "ー" => "-",
  "「" => "‘",
  "」" => "’",
  "『" => "“",
  "』" => "”",
  "［" => "[",
  "］" => "]",
  "（" => "(",
  "）" => ")",
  "｛" => "{",
  "｝" => "}",

  "あ" => "a",
  "い" => "i",
  "う" => "u",
  "え" => "e",
  "お" => "o",
  "ゔぁ" => "va",
  "ゔぃ" => "vi",
  "ゔ" => "vu",
  "ゔぇ" => "ve",
  "ゔぉ" => "vo",
  "か" => "ka",
  "き" => "ki",
  "きゃ" => "kya",
  "きぃ" => "kyi",
  "きゅ" => "kyu",
  "く" => "ku",
  "け" => "ke",
  "こ" => "ko",
  "が" => "ga",
  "ぎ" => "gi",
  "ぐ" => "gu",
  "げ" => "ge",
  "ご" => "go",
  "ぎゃ" => "gya",
  "ぎぃ" => "gyi",
  "ぎゅ" => "gyu",
  "ぎぇ" => "gye",
  "ぎょ" => "gyo",
  "さ" => "sa",
  "す" => "su",
  "せ" => "se",
  "そ" => "so",
  "ざ" => "za",
  "ず" => "zu",
  "ぜ" => "ze",
  "ぞ" => "zo",
  "し" => "shi",
  "しゃ" => "sha",
  "しゅ" => "shu",
  "しょ" => "sho",
  "じ" => "ji",
  "じゃ" => "ja",
  "じゅ" => "ju",
  "じょ" => "jo",
  "た" => "ta",
  "ち" => "chi",
  "ちゃ" => "cha",
  "ちゅ" => "chu",
  "ちょ" => "cho",
  "つ" => "tsu",
  "て" => "te",
  "と" => "to",
  "だ" => "da",
  "ぢ" => "di",
  "づ" => "du",
  "で" => "de",
  "ど" => "do",
  "な" => "na",
  "に" => "ni",
  "にゃ" => "nya",
  "にゅ" => "nyu",
  "にょ" => "nyo",
  "ぬ" => "nu",
  "ね" => "ne",
  "の" => "no",
  "は" => "ha",
  "ひ" => "hi",
  "ふ" => "fu",
  "へ" => "he",
  "ほ" => "ho",
  "ひゃ" => "hya",
  "ひゅ" => "hyu",
  "ひょ" => "hyo",
  "ふぁ" => "fa",
  "ふぃ" => "fi",
  "ふぇ" => "fe",
  "ふぉ" => "fo",
  "ば" => "ba",
  "び" => "bi",
  "ぶ" => "bu",
  "べ" => "be",
  "ぼ" => "bo",
  "びゃ" => "bya",
  "びゅ" => "byu",
  "びょ" => "byo",
  "ぱ" => "pa",
  "ぴ" => "pi",
  "ぷ" => "pu",
  "ぺ" => "pe",
  "ぽ" => "po",
  "ぴゃ" => "pya",
  "ぴゅ" => "pyu",
  "ぴょ" => "pyo",
  "ま" => "ma",
  "み" => "mi",
  "む" => "mu",
  "め" => "me",
  "も" => "mo",
  "みゃ" => "mya",
  "みゅ" => "myu",
  "みょ" => "myo",
  "や" => "ya",
  "ゆ" => "yu",
  "よ" => "yo",
  "ら" => "ra",
  "り" => "ri",
  "る" => "ru",
  "れ" => "re",
  "ろ" => "ro",
  "りゃ" => "rya",
  "りゅ" => "ryu",
  "りょ" => "ryo",
  "わ" => "wa",
  "を" => "wo",
  "ん" => "n",

  // Archaic characters
  "ゐ" => "wi",
  "ゑ" => "we",

  // Uncommon character combos
  "きぇ" => "kye",
  "きょ" => "kyo",
  "じぃ" => "jyi",
  "じぇ" => "jye",
  "ちぃ" => "cyi",
  "ちぇ" => "che",
  "ひぃ" => "hyi",
  "ひぇ" => "hye",
  "びぃ" => "byi",
  "びぇ" => "bye",
  "ぴぃ" => "pyi",
  "ぴぇ" => "pye",
  "みぇ" => "mye",
  "みぃ" => "myi",
  "りぃ" => "ryi",
  "りぇ" => "rye",
  "にぃ" => "nyi",
  "にぇ" => "nye",
  "しぃ" => "syi",
  "しぇ" => "she",
  "いぇ" => "ye",
  "うぁ" => "wha",
  "うぉ" => "who",
  "うぃ" => "wi",
  "うぇ" => "we",
  "ゔゃ" => "vya",
  "ゔゅ" => "vyu",
  "ゔょ" => "vyo",
  "すぁ" => "swa",
  "すぃ" => "swi",
  "すぅ" => "swu",
  "すぇ" => "swe",
  "すぉ" => "swo",
  "くゃ" => "qya",
  "くゅ" => "qyu",
  "くょ" => "qyo",
  "くぁ" => "qwa",
  "くぃ" => "qwi",
  "くぅ" => "qwu",
  "くぇ" => "qwe",
  "くぉ" => "qwo",
  "ぐぁ" => "gwa",
  "ぐぃ" => "gwi",
  "ぐぅ" => "gwu",
  "ぐぇ" => "gwe",
  "ぐぉ" => "gwo",
  "つぁ" => "tsa",
  "つぃ" => "tsi",
  "つぇ" => "tse",
  "つぉ" => "tso",
  "てゃ" => "tha",
  "てぃ" => "thi",
  "てゅ" => "thu",
  "てぇ" => "the",
  "てょ" => "tho",
  "とぁ" => "twa",
  "とぃ" => "twi",
  "とぅ" => "twu",
  "とぇ" => "twe",
  "とぉ" => "two",
  "ぢゃ" => "dya",
  "ぢぃ" => "dyi",
  "ぢゅ" => "dyu",
  "ぢぇ" => "dye",
  "ぢょ" => "dyo",
  "でゃ" => "dha",
  "でぃ" => "dhi",
  "でゅ" => "dhu",
  "でぇ" => "dhe",
  "でょ" => "dho",
  "どぁ" => "dwa",
  "どぃ" => "dwi",
  "どぅ" => "dwu",
  "どぇ" => "dwe",
  "どぉ" => "dwo",
  "ふぅ" => "fwu",
  "ふゃ" => "fya",
  "ふゅ" => "fyu",
  "ふょ" => "fyo",

  //  Small Characters (normally not transliterated alone)
  "ぁ" => "a",
  "ぃ" => "i",
  "ぇ" => "e",
  "ぅ" => "u",
  "ぉ" => "o",
  "ゃ" => "ya",
  "ゅ" => "yu",
  "ょ" => "yo",
  "っ" => "",
  "ゕ" => "ka",
  "ゖ" => "ka",
  "ゎ" => "wa",

  // Ambiguous consonant vowel pairs
  "んあ" => "n'a",
  "んい" => "n'i",
  "んう" => "n'u",
  "んえ" => "n'e",
  "んお" => "n'o",
  "んや" => "n'ya",
  "んゆ" => "n'yu",
  "んよ" => "n'yo",
};
