use crate::defined::Defined;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<'de, T> Deserialize<'de> for Defined<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Defined::Def)
    }
}

impl<T> Serialize for Defined<T>
where
    T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Defined::Def(ref value) => serializer.serialize_some(value),
            _ => serializer.serialize_none(),
        }
    }
}
