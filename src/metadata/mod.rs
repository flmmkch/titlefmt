
pub trait MetadataObject {
	fn read_tag(&self, &str) -> Option<String>;
}
