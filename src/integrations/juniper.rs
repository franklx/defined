use crate::optional::Optional;
use juniper::{
    marker::IsInputType, marker::IsOutputType, meta::MetaType, BoxFuture, ExecutionResult,
    Executor, FromInputValue, GraphQLType, GraphQLValue, GraphQLValueAsync,
    InputValue as JuniperInputValue, Registry, ScalarValue, Selection, ToInputValue,
    Value as JuniperValue,
};

/// Implementation of Juniper::GraphQLType Trait
/// https://docs.rs/juniper/latest/juniper/trait.GraphQLType.html
impl<S, T> GraphQLType<S> for Optional<T>
where
    T: GraphQLType<S>,
    S: ScalarValue,
{
    fn name(_: &Self::TypeInfo) -> Option<&'static str> {
        None
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        registry.build_nullable_type::<T>(info).into_meta()
    }
}

/// Implementation of Juniper::GraphQLValue Trait
/// https://docs.rs/juniper/latest/juniper/trait.GraphQLValue.html
impl<S, T> GraphQLValue<S> for Optional<T>
where
    S: ScalarValue,
    T: GraphQLValue<S>,
{
    type Context = T::Context;
    type TypeInfo = T::TypeInfo;

    fn type_name(&self, _: &Self::TypeInfo) -> Option<&'static str> {
        None
    }

    fn resolve(
        &self,
        info: &Self::TypeInfo,
        _: Option<&[Selection<S>]>,
        executor: &Executor<Self::Context, S>,
    ) -> ExecutionResult<S> {
        match *self {
            Optional::Def(ref obj) => executor.resolve(info, obj),
            Optional::Undef => Ok(JuniperValue::null()),
            Optional::Null => Ok(JuniperValue::null()),
        }
    }
}

/// Implementation of Juniper::IsInputType Trait
/// https://docs.rs/juniper/latest/juniper/marker/trait.IsInputType.html
impl<S, T> IsInputType<S> for Optional<T>
where
    T: IsInputType<S>,
    S: ScalarValue,
{
    #[inline]
    fn mark() {
        T::mark()
    }
}

/// Implementation of Juniper::FromInputValue Trait
/// https://docs.rs/juniper/latest/juniper/trait.FromInputValue.html
impl<S, T> FromInputValue<S> for Optional<T>
where
    T: FromInputValue<S>,
    S: ScalarValue,
{
    fn from_input_value(v: &JuniperInputValue<S>) -> Option<Optional<T>> {
        match v {
            &JuniperInputValue::Null => Some(Optional::Null),
            v => v.convert().map(Optional::Def),
        }
    }
    fn from_implicit_null() -> Self {
        Optional::Undef
    }
}

/// Implementation of Juniper::ToInputValue Trait
/// https://docs.rs/juniper/latest/juniper/trait.ToInputValue.html
impl<S, T> ToInputValue<S> for Optional<T>
where
    T: ToInputValue<S>,
    S: ScalarValue,
{
    fn to_input_value(&self) -> JuniperInputValue<S> {
        match *self {
            Optional::Def(ref v) => v.to_input_value(),
            _ => JuniperInputValue::null(),
        }
    }
}

/// Implementation of Juniper::IsOutputType Trait
/// https://docs.rs/juniper/latest/juniper/marker/trait.IsOutputType.html
impl<S, T> IsOutputType<S> for Optional<T>
where
    T: IsOutputType<S>,
    S: ScalarValue,
{
    #[inline]
    fn mark() {
        T::mark()
    }
}

/// Implementation of Juniper::GraphQLValueAsync Trait
/// https://docs.rs/juniper/latest/juniper/trait.GraphQLValueAsync.html
impl<S, T> GraphQLValueAsync<S> for Optional<T>
where
    T: GraphQLValueAsync<S>,
    T::TypeInfo: Sync,
    T::Context: Sync,
    S: ScalarValue + Send + Sync,
{
    fn resolve_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        _: Option<&'a [Selection<S>]>,
        executor: &'a Executor<Self::Context, S>,
    ) -> BoxFuture<'a, ExecutionResult<S>> {
        let f = async move {
            let value = match self {
                Optional::Def(obj) => executor.resolve_into_value_async(info, obj).await,
                _ => JuniperValue::null(),
            };
            Ok(value)
        };
        Box::pin(f)
    }
}
