use warp::{body::BodyDeserializeError, hyper::StatusCode, reject::Reject, Rejection, Reply};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotFound,
    Unauthorized,
    InvalidToken,
    ValidationError(String),
    DatabaseOperationFailed(String),
    MissingHeader(String),
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
    ServerError,
    InvalidUrn,
    NotModified,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not found"),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::InvalidToken => write!(f, "Invalid token"),
            Error::ValidationError(msg) => write!(f, "Validation error(s): {}", msg),
            Error::DatabaseOperationFailed(msg) => write!(f, "Database: {}", msg),
            Error::MissingHeader(header) => write!(f, "Missing header: {}", header),
            Error::InvalidRequest => write!(f, "Invalid request"),
            Error::InvalidClient => write!(f, "Invalid client"),
            Error::InvalidGrant => write!(f, "Invalid grant"),
            Error::UnauthorizedClient => write!(f, "Unauthorized client"),
            Error::UnsupportedGrantType => write!(f, "Unsupported grant type"),
            Error::InvalidScope => write!(f, "Invalid scope"),
            Error::ServerError => write!(f, "Internal server error"),
            Error::InvalidUrn => write!(f, "Invalid URN"),
            Error::NotModified => write!(f, "Not modified"),
        }
    }
}

impl Reject for Error {}

pub async fn return_error(err: Rejection) -> Result<impl Reply, Rejection> {
    let (code, message) = if let Some(error) = err.find::<Error>() {
        match error {
            Error::NotFound => (StatusCode::NOT_FOUND, error.to_string()),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, error.to_string()),
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, error.to_string()),
            Error::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                format!("Validation error(s): {}", msg),
            ),
            Error::DatabaseOperationFailed(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string())
            }
            Error::MissingHeader(header) => (
                StatusCode::BAD_REQUEST,
                format!("Missing header: {}", header),
            ),
            Error::InvalidRequest => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::InvalidClient => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::InvalidGrant => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::UnauthorizedClient => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::UnsupportedGrantType => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::InvalidScope => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::InvalidUrn => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::NotModified => (StatusCode::NOT_MODIFIED, error.to_string()),
        }
    } else if let Some(error) = err.find::<BodyDeserializeError>() {
        (StatusCode::UNPROCESSABLE_ENTITY, error.to_string())
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed".to_string(),
        )
    } else if err.find::<warp::reject::UnsupportedMediaType>().is_some() {
        (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Unsupported media type".to_string(),
        )
    } else if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not found".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
