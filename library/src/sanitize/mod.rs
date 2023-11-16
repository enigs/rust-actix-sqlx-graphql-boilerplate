use async_graphql::MaybeUndefined;

pub fn mustring(data: &MaybeUndefined<String>) -> MaybeUndefined<String> {
    match data {
        MaybeUndefined::Undefined => MaybeUndefined::Undefined,
        MaybeUndefined::Null => MaybeUndefined::Null,
        MaybeUndefined::Value(s) => {
            let s = s.trim().to_string();

            match s.trim().to_string().is_empty() {
                true => MaybeUndefined::Undefined,
                false => MaybeUndefined::Value(s)
            }
        }
    }
}