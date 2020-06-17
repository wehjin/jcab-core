//! Trainee model
use std::io;

use echo_lib::{Chamber, Echo, ObjectId, Point, Target};

/// The name of the trainee
pub const NAME: &Point = &Point::Static { aspect: "Trainee", name: "name" };

/// Creates a trainee
pub fn create(name: &str, echo: &Echo) -> io::Result<ObjectId> {
	echo.write(|scope| {
		let trainee_id = scope.new_object_id("trainee");
		scope.write_object_properties(&trainee_id, vec![
			(NAME, Target::String(name.to_string()))
		]);
		trainee_id
	})
}

/// Determines if a trainee exists
pub fn exists(trainee_id: &ObjectId, chamber: &Chamber) -> io::Result<bool> {
	let name = chamber.target_at_object_point_or_none(&trainee_id, NAME);
	Ok(name.is_some())
}