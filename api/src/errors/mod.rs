use actix_web::ResponseError;

#[derive(Debug)]
pub struct LoginError {
    message: String,
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl ResponseError for LoginError {}
