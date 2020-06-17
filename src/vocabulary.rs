//! Vocabulary points and functions
use std::io;

use echo_lib::{Chamber, Echo, ObjectId, Point, Target};

use crate::vocabulary;

/// The name of the vocabulary
pub const NAME: &Point = &Point::Static { aspect: "Vocabulary", name: "name" };

/// Creates the first vocabulary. If a vocabulary already exists, returns its id.
pub fn create_if_none(name: &str, echo: &Echo) -> io::Result<ObjectId> {
	match vocabulary::read(&echo.chamber()?)? {
		None => vocabulary::create(name, echo),
		Some(id) => Ok(id),
	}
}

/// Finds an existing vocabulary
pub fn read(chamber: &Chamber) -> io::Result<Option<ObjectId>> {
	let vocabularies = chamber.objects_with_point(vocabulary::NAME)?;
	Ok(vocabularies.first().cloned())
}

/// Creates a new vocabulary
pub fn create(name: &str, echo: &Echo) -> io::Result<ObjectId> {
	echo.write(|scope| {
		let vocabulary_id = scope.new_object_id("vocabulary");
		let properties = vec![(vocabulary::NAME, Target::String(name.to_string()))];
		scope.write_object_properties(&vocabulary_id, properties);
		vocabulary_id
	})
}
