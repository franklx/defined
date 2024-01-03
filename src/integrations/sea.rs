use sea_query::{Value, Nullable, ValueType, ValueTypeErr, ArrayType, ColumnType};

use crate::defined::Defined;

impl<T> From<Defined<T>> for Value
where
    T: Into<Value> + Nullable,
{
    fn from(x: Defined<T>) -> Value {
        match x {
            Defined::Def(v) => v.into(),
            Defined::Undef => T::null(),
        }
    }
}

impl<T> ValueType for Defined<T>
where
    T: ValueType + Nullable,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        if v == T::null() {
            Ok(Defined::Undef)
        } else {
            Ok(Defined::Def(T::try_from(v)?))
        }
    }

    fn type_name() -> String {
        format!("Defined<{}>", T::type_name())
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        T::column_type()
    }
}