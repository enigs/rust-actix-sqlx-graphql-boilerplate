#[derive(Default)]
pub enum Response {
    #[default]
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    InternalServerError,
    ErrorWithoutExtensions,
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }
}