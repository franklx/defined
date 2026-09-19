use ts_rs::{Config, TS, TypeVisitor};

use crate::defined::Defined;

impl<T: TS> TS for Defined<T> {
    type WithoutGenerics = Self;
    type OptionInnerType = T;
    const IS_OPTION: bool = true;

    fn name(cfg: &Config) -> String {
        format!("{} | null", T::name(cfg))
    }

    fn inline(cfg: &Config) -> String {
        format!("{} | null", T::inline(cfg))
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as TS>::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as TS>::visit_generics(v);
        v.visit::<T>();
    }
}