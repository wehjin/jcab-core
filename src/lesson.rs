//! Lesson points and functions
use std::io;

use echo_lib::{Chamber, Echo, ObjectId, Point, Target};

use crate::{lesson, LessonPart, SpeechPart};

pub const ENGLISH: &Point = &Point::Static { aspect: "Lesson", name: "english" };
pub const ENGLISH_DISAMBIGUATE: &Point = &Point::Static { aspect: "Lesson", name: "english-disambiguate" };
pub const KANA: &Point = &Point::Static { aspect: "Lesson", name: "kana" };
pub const KANA_DISAMBIGUATE: &Point = &Point::Static { aspect: "Lesson", name: "kana-disambiguate" };
pub const KANA_INFORM: &Point = &Point::Static { aspect: "Lesson", name: "kana-inform" };
pub const KANJI: &Point = &Point::Static { aspect: "Lesson", name: "kanji" };
pub const SPEECH_PART: &Point = &Point::Static { aspect: "Lesson", name: "speech-part" };
pub const LEVEL: &Point = &Point::Static { aspect: "Lesson", name: "level" };
pub const VOCABULARY_ID: &Point = &Point::Static { aspect: "Lesson", name: "vocabulary-id" };


pub fn read(vocabulary_id: &ObjectId, chamber: &Chamber) -> io::Result<Vec<ObjectId>> {
	chamber.objects_with_property(lesson::VOCABULARY_ID, &Target::Object(vocabulary_id.to_owned()))
}

pub fn create(english: &LessonPart, kana: &LessonPart, kanji: &Option<String>, speech_part: &SpeechPart, level: u64, vocabulary_id: &ObjectId, echo: &Echo) -> io::Result<ObjectId> {
	echo.write(|scope| {
		let lesson_id = scope.new_object_id("lesson");
		let mut properties = vec![
			(lesson::ENGLISH, Target::String(english.string.to_owned())),
			(lesson::KANA, Target::String(kana.string.to_owned())),
			(lesson::LEVEL, Target::Number(level)),
			(lesson::SPEECH_PART, Target::String(speech_part.to_string())),
			(lesson::VOCABULARY_ID, Target::Object(vocabulary_id.clone())),
		];
		if let Some(ref optional) = english.disambiguate { properties.push((lesson::ENGLISH_DISAMBIGUATE, Target::String(optional.to_string()))) }
		if let Some(ref optional) = kana.disambiguate { properties.push((lesson::KANA_DISAMBIGUATE, Target::String(optional.to_string()))) }
		if let Some(ref optional) = kana.inform { properties.push((lesson::KANA_INFORM, Target::String(optional.to_string()))) }
		if let Some(ref optional) = kanji { properties.push((lesson::KANJI, Target::String(optional.to_string()))) }
		scope.write_object_properties(&lesson_id, properties);
		lesson_id
	})
}
