/// A metadata-providing object
/// Most likely a single audio file or track
pub trait Provider {
	/// Returns the value of a tag given as parameter
	/// If there is no such tag in the metada then return None
	fn tag_value(&self, &str) -> Option<String>;
}
