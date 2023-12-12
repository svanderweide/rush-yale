use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use sea_orm::DbErr;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "An internal error has occured. Please try again later.")]
    _InternalError,
    #[display(fmt = "A database error has occured. Please try again later.")]
    DatabaseError,
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::_InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DbErr> for Error {
    fn from(_err: DbErr) -> Self {
        Error::DatabaseError
    }
}
