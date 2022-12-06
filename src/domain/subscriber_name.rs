use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

static EMPTY_NAME_ERROR: &'static str = "Name must not be empty";
static NAME_TOO_LONG_ERROR: &'static str =
    "Name must have less than 256 characters. Please use a nickname if needed";
static FORBIDDEN_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
static FORBIDDEN_CHARACTERS_ERROR: &'static str = "Name must not contain a forbidden character ( '/', '(', ')', '\"', '<', '>', '\\', '{', '}') Please use a nickname if needed";

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, &'static str> {
        if s.trim().is_empty() {
            return Err(EMPTY_NAME_ERROR);
        }

        if s.graphemes(true).count() > 256 {
            return Err(NAME_TOO_LONG_ERROR);
        }

        let contains_forbidden_characters = s.chars().any(|g| FORBIDDEN_CHARACTERS.contains(&g));

        if contains_forbidden_characters {
            return Err(FORBIDDEN_CHARACTERS_ERROR);
        }

        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
