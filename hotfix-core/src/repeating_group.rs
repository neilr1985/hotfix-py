use pyo3::{pyclass, pymethods};
use hotfix_message::{Field, Part, RepeatingGroup as RustRepeatingGroup, TagU32};

#[pyclass]
pub struct RepeatingGroup {
    pub(crate) inner: RustRepeatingGroup
}

#[pymethods]
impl RepeatingGroup {
    #[new]
    fn new(start_tag: u32, delimiter_tag: u32) -> Self {
        let inner = RustRepeatingGroup::new_with_tags(TagU32::new(start_tag).unwrap(), TagU32::new(delimiter_tag).unwrap());
        RepeatingGroup { inner }
    }

    fn append(&mut self, tag: u32, value: String) {
        let field = Field::new(TagU32::new(tag).unwrap(), value.into_bytes());
        self.inner.get_field_map_mut().insert(field)
    }
}