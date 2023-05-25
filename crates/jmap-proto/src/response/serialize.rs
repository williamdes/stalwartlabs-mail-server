use serde::{ser::SerializeSeq, Serialize};

use crate::request::Call;

use super::ResponseMethod;

impl Serialize for Call<ResponseMethod> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(3.into())?;
        seq.serialize_element(&self.name.to_string())?;
        seq.serialize_element(&self.method)?;
        seq.serialize_element(&self.id)?;
        seq.end()
    }
}

pub fn serialize_hex<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    format!("{:x}", value).serialize(serializer)
}