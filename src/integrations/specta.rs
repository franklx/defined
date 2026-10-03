use specta::{Type, datatype::{DataType, DefOpts}, r#type::{ExportError, TypeCategory}};

use crate::Defined;

impl<T: Type> Type for Defined<T> {
    fn inline(opts: DefOpts, generics: &[DataType]) -> Result<DataType, ExportError> {
        Ok(DataType::Nullable(Box::new(
            generics
                .first()
                .cloned()
                .map_or_else(|| T::inline(opts, generics), Ok)?,
        )))
    }

    fn category_impl(opts: DefOpts, generics: &[DataType]) -> Result<TypeCategory, ExportError> {
        Ok(TypeCategory::Inline(DataType::Nullable(Box::new(
            generics
                .first()
                .cloned()
                .map_or_else(|| T::reference(opts, generics), Ok)?,
        ))))
    }
}