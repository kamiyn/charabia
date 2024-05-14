#[cfg(feature = "japanese-segmentation-ipadic")]
use lindera::Penalty;
use lindera::{DictionaryConfig, DictionaryKind, Mode, Tokenizer, TokenizerConfig};
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Japanese specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct JapaneseSegmenter;

static LINDERA: Lazy<Tokenizer> = Lazy::new(|| {
    #[cfg(all(feature = "japanese-segmentation-ipadic", feature = "japanese-segmentation-unidic"))]
    compile_error!("Feature japanese-segmentation-ipadic and japanese-segmentation-unidic are mutually exclusive and cannot be enabled together");

    fn user_dictionary(kind: DictionaryKind) -> Option<lindera::UserDictionaryConfig> {
        let user_dictionary_path = std::env::var("CHARABIA_JAPANESE_USER_DICTIONARY").ok()?;
        let pathbuf = std::path::PathBuf::from(user_dictionary_path);
        Some(lindera::UserDictionaryConfig { kind: Some(kind), path: pathbuf })
        // ここの責務は config の構築であり pathbuf.is_file() 等による存在確認はしない
        // ファイルの存在・ファイルの中身の形式確認と、結果としてエラーが発生した場合のエラーの伝播は user_dictionary を使う時点の課題
    }

    #[cfg(feature = "japanese-segmentation-ipadic")]
    let config = TokenizerConfig {
        dictionary: DictionaryConfig { kind: Some(DictionaryKind::IPADIC), path: None },
        user_dictionary: user_dictionary(DictionaryKind::IPADIC),
        mode: Mode::Decompose(Penalty::default()),
        ..TokenizerConfig::default()
    };
    #[cfg(feature = "japanese-segmentation-unidic")]
    let config = TokenizerConfig {
        dictionary: DictionaryConfig { kind: Some(DictionaryKind::UniDic), path: None },
        user_dictionary: user_dictionary(DictionaryKind::UniDic),
        mode: Mode::Normal,
        ..TokenizerConfig::default()
    };
    Tokenizer::from_config(config).unwrap()
});

impl Segmenter for JapaneseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segment_iterator = LINDERA.tokenize(to_segment).unwrap();
        Box::new(segment_iterator.into_iter().map(|token| token.text))
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "関西国際空港限定トートバッグ すもももももももものうち";

    const SEGMENTED: &[&str] = if cfg!(feature = "japanese-segmentation-ipadic") {
        &[
            "関西",
            "国際",
            "空港",
            "限定",
            "トートバッグ",
            " ",
            "すもも",
            "も",
            "もも",
            "も",
            "もも",
            "の",
            "うち",
        ]
    } else if cfg!(feature = "japanese-segmentation-unidic") {
        &[
            "関西",
            "国際",
            "空港",
            "限定",
            "トート",
            "バッグ",
            " ",
            "すもも",
            "も",
            "もも",
            "も",
            "もも",
            "の",
            "うち",
        ]
    } else {
        &[]
    };

    const TOKENIZED: &[&str] = if cfg!(feature = "japanese-segmentation-ipadic") {
        &[
            "関西",
            "国際",
            "空港",
            "限定",
            // Use "とうとばっぐ" instead when feature "japanese-transliteration" is enabled or become default
            #[cfg(feature = "japanese-transliteration")]
            "とうとは\u{3099}っく\u{3099}",
            #[cfg(not(feature = "japanese-transliteration"))]
            "トートハ\u{3099}ック\u{3099}",
            " ",
            "すもも",
            "も",
            "もも",
            "も",
            "もも",
            "の",
            "うち",
        ]
    } else if cfg!(feature = "japanese-segmentation-unidic") {
        &[
            "関西",
            "国際",
            "空港",
            "限定",
            // Use "とうとばっぐ" instead when feature "japanese-transliteration" is enabled or become default
            #[cfg(feature = "japanese-transliteration")]
            "とうと",
            #[cfg(not(feature = "japanese-transliteration"))]
            "トート",
            #[cfg(feature = "japanese-transliteration")]
            "は\u{3099}っく\u{3099}",
            #[cfg(not(feature = "japanese-transliteration"))]
            "ハ\u{3099}ック\u{3099}",
            " ",
            "すもも",
            "も",
            "もも",
            "も",
            "もも",
            "の",
            "うち",
        ]
    } else {
        &[]
    };

    #[cfg(all(feature = "japanese-segmentation-ipadic", feature = "japanese-segmentation-unidic"))]
    compile_error!("Feature japanese-segmentation-ipadic and japanese-segmentation-unidic are mutually exclusive and cannot be enabled together");

    // Macro that run several tests on the Segmenter.
    test_segmenter!(JapaneseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Cj, Language::Jpn);
}
