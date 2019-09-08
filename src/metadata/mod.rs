/// Trait for a metadata-providing object.
///
/// For example, a single audio file or track.
pub trait Provider {
    /// Returns the value of a tag with the tag key given as parameter.
    ///
    /// If there is no corresponding entry in the metadata then `None` should be returned.
    fn tag_value(&self, tag_name: &str) -> Option<String>;
}
