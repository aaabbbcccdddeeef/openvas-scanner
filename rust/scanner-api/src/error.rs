use std::{collections::HashMap, fmt::Display, io::Cursor};

use rocket::{http::Status, response::Responder, Request, Response};
use serde::{Deserialize, Serialize};

use crate::guards::{authorization::AuthorizationError, json_validation::JsonValidationError};

/// Errors, that might occur during request processing
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiError {
    /// The given body does not contain a valid JSON String
    BadJsonError {
        /// Error message generated by serde
        message: String,
    },
    /// The given JSON cannot be parsed correctly because of incorrect fields
    ParseJsonError {
        /// Error message generated by serde
        message: String,
        /// The line position of the error
        line: usize,
        /// The column position of the error
        column: usize,
    },
    /// A requested resource was not found
    ResourceNotFound {
        /// Error message
        message: String,
        /// ID of the requested resource
        id: String,
    },
    /// Cannot create a resource, that already exists
    ResourceExists {
        /// Error message
        message: String,
        /// ID of the existing resource
        id: String,
    },
    /// Cannot parse the quarry parameters
    ParseQueryError {
        /// Error message
        message: String,
        /// The erroneous parameter with a description of the error
        field_errors: HashMap<String, String>,
    },
    /// A resource is in a different state, that is needed for the requested action
    BadResourceState {
        /// Error message
        message: String,
        /// Expected state to perform the action
        expected: Vec<String>,
        /// Current state of the resource
        got: String,
    },
    /// The requested action is not implemented
    ActionNotSupported {
        /// Error message
        message: String,
        /// Available actions
        available: Vec<String>,
        /// Requested action
        got: String,
    },
    /// Unable to read data in request body. E.g. the given JSON is too large
    IOError {
        message: String,
    },
    /// Something unexpected happened, this is a server internal error
    Unexpected {
        message: String,
    },
    /// The given API key is invalid
    InvalidApiKey {
        message: String,
    },
    MissingApiKey {
        message: String,
    },
    InvalidCertificate {
        message: String,
    },
    MissingCertificate {
        message: String,
    },
    CertParseError {
        message: String,
    },
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadJsonError { message } => write!(f, "{message}"),
            Self::ParseJsonError { message, .. } => write!(f, "{message}"),
            Self::ResourceNotFound { message, .. } => write!(f, "{message}"),
            Self::ResourceExists { message, .. } => write!(f, "{message}"),
            Self::ParseQueryError { message, .. } => write!(f, "{message}"),
            Self::BadResourceState { message, .. } => write!(f, "{message}"),
            Self::ActionNotSupported { message, .. } => write!(f, "{message}"),
            Self::IOError { message, .. } => write!(f, "{message}"),
            Self::Unexpected { message } => write!(f, "{message}"),
            Self::InvalidApiKey { message } => write!(f, "{message}"),
            Self::MissingApiKey { message } => write!(f, "{message}"),
            Self::InvalidCertificate { message } => write!(f, "{message}"),
            Self::MissingCertificate { message } => write!(f, "{message}"),
            Self::CertParseError { message } => write!(f, "{message}"),
        }
    }
}

impl<'a> From<&JsonValidationError<'a>> for ApiError {
    fn from(value: &JsonValidationError<'a>) -> Self {
        match value {
            JsonValidationError::Io(e) => Self::IOError {
                message: e.to_string(),
            },
            JsonValidationError::Parse(_, e) => Self::ParseJsonError {
                message: e.to_string(),
                line: e.line(),
                column: e.column(),
            },
        }
    }
}

impl<'a> From<AuthorizationError<'a>> for ApiError {
    fn from(value: AuthorizationError) -> Self {
        match value {
            AuthorizationError::KeyInvalid => Self::InvalidApiKey {
                message: "The given API key is invalid".to_string(),
            },
            AuthorizationError::KeyMissing => Self::MissingApiKey {
                message: "No API key was given".to_string(),
            },
            AuthorizationError::CertificateInvalid => Self::InvalidCertificate {
                message: "The given Certificate has insufficient permission".to_string(),
            },
            AuthorizationError::CertificateMissing => Self::MissingCertificate {
                message: "The given Certificate has insufficient permission".to_string(),
            },
            AuthorizationError::InvalidAuthMethod(method) => Self::Unexpected {
                message: format!(
                    "An invalid auth method was provided in the webserver config ({method})"
                ),
            },
            AuthorizationError::InternalSerialError(e) => Self::Unexpected {
                message: e.to_string(),
            },
            AuthorizationError::ParseError(e) => Self::CertParseError {
                message: e.to_string(),
            },
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let msg = serde_json::to_string(&self).unwrap();
        let mut resp = Response::new();
        resp.set_sized_body(msg.len(), Cursor::new(msg));
        match self {
            Self::ResourceNotFound { .. } => resp.set_status(Status::NotFound),
            Self::ResourceExists { .. }
            | Self::ParseQueryError { .. }
            | Self::BadJsonError { .. }
            | Self::ParseJsonError { .. } => resp.set_status(Status::BadRequest),
            Self::BadResourceState { .. } => resp.set_status(Status::NotAcceptable),
            Self::ActionNotSupported { .. } => resp.set_status(Status::NotImplemented),
            Self::IOError { .. } => resp.set_status(Status::UnprocessableEntity),
            Self::Unexpected { .. } => resp.set_status(Status::InternalServerError),
            Self::InvalidApiKey { .. }
            | Self::MissingApiKey { .. }
            | Self::InvalidCertificate { .. }
            | Self::MissingCertificate { .. }
            | Self::CertParseError { .. } => resp.set_status(Status::Unauthorized),
        };
        Ok(resp)
    }
}
