#[derive(Default)]
pub struct LessonPart {
	pub string: String,
	pub disambiguate: Option<String>,
	pub inform: Option<String>,
}

impl LessonPart {
	pub fn string(string: &str) -> Self {
		LessonPart { string: string.to_string(), ..Default::default() }
	}
}
