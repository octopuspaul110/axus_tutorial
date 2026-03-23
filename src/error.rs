use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    LoginFail,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    TicketDeleteFailIdNotFound {id : u64}
}
impl IntoResponse for Error{
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}","INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR,"UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt,"{self:?}")
    }
}

impl std::error::Error for Error {

}