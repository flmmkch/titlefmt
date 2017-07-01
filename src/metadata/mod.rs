/// A metadata-providing object
/// Most likely a single audio file or track
pub trait MetadataObject {
	/// Returns the value of a tag given as parameter
	/// If there is no such tag in the metada then return None
	fn read_tag(&self, &str) -> Option<String>;
}
