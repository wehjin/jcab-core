extern crate echo_lib;

pub use lesson_part::*;
pub use speech_part::*;

#[cfg(test)]
mod tests {
	use std::error::Error;

	use echo_lib::Echo;
	use echo_lib::util::unique_name;

	use crate::{lesson, LessonPart, SpeechPart, vocabulary};

	#[test]
	fn it_works() -> Result<(), Box<dyn Error>> {
		let echo_name = unique_name("it-works");
		{
			let echo = Echo::connect(&echo_name, &std::env::temp_dir());
			let vocabulary_id = vocabulary::create_if_none("Test", &echo)?;
			lesson::create(
				&LessonPart::string("see"),
				&LessonPart::string("みる"),
				&Some("見る".to_string()),
				&SpeechPart::Verb,
				1,
				&vocabulary_id,
				&echo,
			)?;
		}
		let echo = Echo::connect(&echo_name, &std::env::temp_dir());
		let chamber = echo.chamber()?;
		let vocabulary_id = vocabulary::read(&chamber)?.unwrap();
		let lesson_id = lesson::read(&vocabulary_id, &chamber)?.first().cloned().unwrap();
		let english = chamber.string(&lesson_id, lesson::ENGLISH);
		assert_eq!(english, "see");
		Ok(())
	}
}

pub mod vocabulary;
pub mod lesson;
mod speech_part;
mod lesson_part;
