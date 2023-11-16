use async_graphql::MaybeUndefined;

pub fn mui642str(data: MaybeUndefined<i64>) -> String {
    data.take().unwrap_or_default().to_string()
}

pub fn mui322i32(data: MaybeUndefined<i32>) -> i32 {
    data.take().unwrap_or(0)
}

pub fn mustr2str(data: MaybeUndefined<String>) -> String {
    data.take().unwrap_or_default()
}

pub fn str2optstr(data: String) -> Option<String> {
    match data.is_empty() {
        true => None,
        false => Some(data)
    }
}

pub fn optstr2str(data: Option<String>) -> String {
    data.unwrap_or_default()
}

pub fn skip<T>(data: T) -> T {
    data
}