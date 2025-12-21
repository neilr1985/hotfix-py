use hotfix::message::FixMessage;
use hotfix_message::{Field, FieldMap, Part, TagU32};
use hotfix_message::message::Config;
use hotfix_message::session_fields::MSG_TYPE;
use pyo3::{pyclass, pyfunction, pymethods, PyClassInitializer, PyRef, PyResult};
use crate::repeating_group::RepeatingGroup;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct Message {
    message_type: String,
    field_map: FieldMap
}

#[pymethods]
impl Message {
    #[new]
    fn new(message_type: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(Message { message_type, field_map: FieldMap::default() })
    }

    fn insert(&mut self, tag: u32, value: String) {
        let field = Field::new(TagU32::new(tag).unwrap(), value.into_bytes());
        self.field_map.insert(field)
    }

    fn insert_groups(&mut self, start_tag: u32, groups: Vec<PyRef<'_, RepeatingGroup>>) -> PyResult<()> {
        if groups.is_empty() {
            return Ok(());
        }

        // Automatically set the count field for the repeating group
        let count = groups.len().to_string();
        let count_field = Field::new(TagU32::new(start_tag).unwrap(), count.into_bytes());
        self.field_map.insert(count_field);

        // Store the group instances
        let groups = groups.into_iter().map(|g| g.inner.clone()).collect();
        self.field_map.set_groups(TagU32::new(start_tag).unwrap(), groups);

        Ok(())
    }
}

#[pyfunction]
pub fn encode_message(message: &Message, begin_string: &str, separator: u8) -> Vec<u8> {
    let mut msg = hotfix_message::message::Message::new(begin_string, &message.message_type);
    message.write(&mut msg);
    let config = Config::with_separator(separator);
    msg.encode(&config).unwrap()
}

impl FixMessage for Message {
    fn write(&self, msg: &mut hotfix_message::message::Message) {
        let target = msg.get_field_map_mut();
        for (_, field) in &self.field_map.fields {
            target.insert(field.clone());
        }
        for (start_tag, groups) in &self.field_map.groups {
            target.set_groups(*start_tag, groups.clone());
        }
    }

    fn message_type(&self) -> &str {
        &self.message_type
    }

    fn parse(msg: &hotfix_message::message::Message) -> Self {
        let message_type: &str = msg.header().get(MSG_TYPE).unwrap();
        Message { message_type: message_type.to_string(), field_map: FieldMap::default() }
    }
}