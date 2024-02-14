//! Convenient error handling

use crate::types::WSResult;
use async_tungstenite::tungstenite;

use std::fmt;
//use tokio_tungstenite::tungstenite;

pub type HassResult<T> = std::result::Result<T, HassError>;

/// The error enum for Hass
#[derive(Debug)]
pub enum HassError {
    /// Returned when the connection to gateway has failed
    CantConnectToGateway,

    /// Returned when it is unable to authenticate
    AuthenticationFailed(String),

    /// Returned when unable to parse the websocket server address
    WrongAddressProvided(url::ParseError),

    /// Returned when serde was unable to deserialize the values
    UnableToDeserialize(serde_json::error::Error),

    /// Returned when connection has unexpected failed
    ConnectionClosed,

    /// Tungstenite error
    TungsteniteError(tungstenite::error::Error),

    ///Tokio Tungstenite error
    //TokioTungsteniteError(tokio_tungstenite::tungstenite::Error),

    /// Returned when an unknown message format is received
    UnknownPayloadReceived,

    /// Returned the error received from the Home Assistant Gateway
    ReponseError(WSResult),

    /// Returned for errors which do not fit any of the above criterias
    Generic(String),
}

impl std::error::Error for HassError {}

impl fmt::Display for HassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CantConnectToGateway => write!(f, "Cannot connect to gateway"),
            Self::ConnectionClosed => write!(f, "Connection closed unexpectedly"),
            Self::AuthenticationFailed(e) => write!(f, "Authentication has failed: {}", e),
            Self::WrongAddressProvided(e) => {
                write!(f, "Could not parse the provided address: {}", e)
            }
            Self::UnableToDeserialize(e) => {
                write!(f, "Unable to deserialize the received value: {}", e)
            }
            Self::TungsteniteError(e) => write!(f, "Tungstenite Error: {}", e),
            //Self::TokioTungsteniteError(e) => write!(f, "Tokio Tungstenite Error: {}", e),
            Self::UnknownPayloadReceived => write!(f, "The received payload is unknown"),
            Self::ReponseError(e) => write!(
                f,
                "The error code:{} with the error message: {}",
                e.error.as_ref().unwrap().code,
                e.error.as_ref().unwrap().message
            ),
            Self::Generic(detail) => write!(f, "Generic Error: {}", detail),
        }
    }
}

// impl From<SendError> for HassError {
//     fn from(error: SendError) -> Self {
//         match error {
//             _ => HassError::ChannelSend(error),
//         }
//     }
// }

impl From<url::ParseError> for HassError {
    fn from(error: url::ParseError) -> Self {
        HassError::WrongAddressProvided(error)
    }
}

impl From<serde_json::error::Error> for HassError {
    fn from(error: serde_json::error::Error) -> Self {
        HassError::UnableToDeserialize(error)
    }
}

// impl From<tokio_tungstenite::tungstenite::Error> for HassError {
//     fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
//         match error {
//             tokio_tungstenite::tungstenite::Error::ConnectionClosed => HassError::ConnectionClosed,
//             _ => HassError::TokioTungsteniteError(error),
//         }
//     }
// }

impl From<tungstenite::error::Error> for HassError {
    fn from(error: tungstenite::error::Error) -> Self {
        match error {
            tungstenite::error::Error::ConnectionClosed => HassError::ConnectionClosed,
            _ => HassError::TungsteniteError(error),
        }
    }
}

impl From<&tungstenite::error::Error> for HassError {
    fn from(error: &tungstenite::error::Error) -> Self {
        let e = match error {
            tungstenite::error::Error::ConnectionClosed => {
                tungstenite::error::Error::ConnectionClosed
            }
            tungstenite::error::Error::AlreadyClosed => tungstenite::error::Error::AlreadyClosed,
            _ => return HassError::Generic(format!("Error from ws {}", error)),
        };
        HassError::TungsteniteError(e)
    }
}
