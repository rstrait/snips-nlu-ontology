use crate::language_enum;
use failure::bail;

language_enum!([DE, EN, ES, FR, IT, PT_PT, PT_BR, JA, KO, RU]);

impl Language {
    pub fn full_name(&self) -> &'static str {
        match *self {
            Language::DE => "German",
            Language::EN => "English",
            Language::ES => "Spanish",
            Language::FR => "French",
            Language::IT => "Italian",
            Language::PT_PT => "Portuguese - Europe",
            Language::PT_BR => "Portuguese - Brazil",
            Language::JA => "Japanese",
            Language::KO => "Korean",
            Language::RU => "Russian",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn init_from_lowercased_string_works() {
        let raw_lang = vec!["de", "en", "es", "fr", "it", "pt_pt", "pt_br", "ja", "ko", "ru"];
        for raw_lang in raw_lang {
            let lang = Language::from_str(raw_lang);
            assert!(lang.is_ok(), raw_lang);
        }
    }

    #[test]
    fn init_from_uppercased_string_works() {
        let raw_lang = vec!["DE", "EN", "ES", "FR", "IT", "PT_PT", "PT_BR", "JA", "KO", "RU"];
        for raw_lang in raw_lang {
            let lang = Language::from_str(raw_lang);
            assert!(lang.is_ok(), raw_lang);
        }
    }
}
