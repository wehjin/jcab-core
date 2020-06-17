#[derive(Debug, Copy, Clone)]
pub enum SpeechPart { Noun, Verb, Adverb, Particle }

impl SpeechPart {
	pub fn to_string(&self) -> String {
		match self {
			SpeechPart::Noun => "noun".to_string(),
			SpeechPart::Verb => "verb".to_string(),
			SpeechPart::Adverb => "adverb".to_string(),
			SpeechPart::Particle => "particle".to_string(),
		}
	}
}
