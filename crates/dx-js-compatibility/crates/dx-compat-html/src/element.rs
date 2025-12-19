//! Element manipulation.

/// Content type for insertion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// HTML content
    Html,
    /// Plain text content
    Text,
}

/// Element wrapper for manipulation.
pub struct Element {
    _placeholder: (),
}

impl Element {
    /// Get an attribute value.
    pub fn get_attribute(&self, _name: &str) -> Option<String> {
        // TODO: Implement
        None
    }

    /// Set an attribute value.
    pub fn set_attribute(&mut self, _name: &str, _value: &str) {
        // TODO: Implement
    }

    /// Remove an attribute.
    pub fn remove_attribute(&mut self, _name: &str) {
        // TODO: Implement
    }

    /// Check if attribute exists.
    pub fn has_attribute(&self, _name: &str) -> bool {
        // TODO: Implement
        false
    }

    /// Get the tag name.
    pub fn tag_name(&self) -> &str {
        // TODO: Implement
        ""
    }

    /// Insert content before the element.
    pub fn before(&mut self, _content: &str, _content_type: ContentType) {
        // TODO: Implement
    }

    /// Insert content after the element.
    pub fn after(&mut self, _content: &str, _content_type: ContentType) {
        // TODO: Implement
    }

    /// Prepend content inside the element.
    pub fn prepend(&mut self, _content: &str, _content_type: ContentType) {
        // TODO: Implement
    }

    /// Append content inside the element.
    pub fn append(&mut self, _content: &str, _content_type: ContentType) {
        // TODO: Implement
    }

    /// Replace the element with content.
    pub fn replace(&mut self, _content: &str, _content_type: ContentType) {
        // TODO: Implement
    }

    /// Remove the element.
    pub fn remove(&mut self) {
        // TODO: Implement
    }

    /// Set the inner content.
    pub fn set_inner_content(&mut self, _content: &str, _content_type: ContentType) {
        // TODO: Implement
    }
}
