use hotfix::message::FixMessage;
use hotfix_message::{Field, FieldMap, Part, TagU32};
use hotfix_message::message::Config;
use pyo3::{pyclass, pyfunction, pymethods, PyClassInitializer};

#[pyclass]
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

    fn insert(&mut self, tag: u32, value: Vec<u8>) {
        let field = Field::new(TagU32::new(tag).unwrap(), value);
        self.field_map.insert(field)
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

    fn parse(_message: &hotfix_message::message::Message) -> Self {
        todo!()
    }
}