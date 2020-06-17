extern crate echo_lib;

pub use lesson_part::*;
pub use speech_part::*;

#[cfg(test)]
mod tests {
	use std::error::Error;

	use echo_lib::{Echo, ObjectId};
	use echo_lib::util::unique_name;

	use crate::{lesson, LessonPart, SpeechPart, trainee, vocabulary};

	#[test]
	fn it_works() -> Result<(), Box<dyn Error>> {
		let echo_name = unique_name("it-works");
		let trainee_id = setup(&echo_name)?;
		let chamber = Echo::connect(&echo_name, &std::env::temp_dir()).chamber()?;
		assert!(trainee::exists(&trainee_id, &chamber)?);
		let vocabulary_id = vocabulary::read(&chamber)?.unwrap();
		let lesson_id = lesson::read(&vocabulary_id, &chamber)?.first().cloned().unwrap();
		let english = chamber.string(&lesson_id, lesson::ENGLISH);
		assert_eq!(english, "see");
		Ok(())
	}

	fn setup(echo_name: &String) -> Result<ObjectId, Box<dyn Error>> {
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
		Ok(trainee::create("Alice", &echo)?)
	}
}

pub mod vocabulary;
pub mod lesson;
pub mod trainee;

mod speech_part;
mod lesson_part;
