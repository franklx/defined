use crate::defined::Defined;
use serde::{de::Error, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

struct DefinedVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for DefinedVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Defined<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("defined")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Defined::Undef)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Defined::Undef)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Defined::Def)
    }
}

impl<'de, T> Deserialize<'de> for Defined<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(DefinedVisitor {
            marker: PhantomData,
        })
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
