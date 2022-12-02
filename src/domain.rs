use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
pub struct SubscriberName(String);

static EMPTY_NAME_ERROR: &'static str = "Name must not be empty";
static NAME_TOO_LONG_ERROR: &'static str =
    "Name must have less than 256 characters. Please use a nickname if needed";
static FORBIDDEN_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
static FORBIDDEN_CHARACTERS_ERROR: &'static str = "Name must not contain a forbidden character ( '/', '(', ')', '\"', '<', '>', '\\', '{', '}') Please use a nickname if needed";

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, &'static str> {
        if s.is_empty() {
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

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
