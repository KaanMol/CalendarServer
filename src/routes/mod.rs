use core::fmt;

use dotenv::Error;
use serde::ser::SerializeStruct;

pub mod calendar;
pub mod source;
pub mod user;
pub mod filter;
pub mod modifiers;
pub mod calendar_event;

pub fn parse_id(
    id: impl Into<String>,
) -> Result<mongodb::bson::oid::ObjectId, actix_web::error::Error> {
    mongodb::bson::oid::ObjectId::parse_str(id.into())
        .map_err(|e| actix_web::error::ErrorBadRequest(ErrorResponse::new("Could not parse id", e)))
}

fn to_json<T>(value: T) -> String
where
    T: serde::Serialize,
{
    match serde_json::to_string(&value) {
        Ok(json) => json,
        Err(e) => format!("{}:#?", e),
    }
}

fn to_bson<T>(value: T) -> mongodb::bson::Bson
where
    T: serde::Serialize,
{
    match mongodb::bson::to_bson(&value) {
        Ok(bson) => bson,
        Err(e) => mongodb::bson::Bson::String(format!("{}:#?", e)),
    }
}

// Response
#[derive(serde::Serialize, Debug)]
pub struct Response {
    pub message: String,
}

impl Response {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_json(&self))
    }
}

// Error response
#[derive(Debug)]
pub struct ErrorResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    pub message: String,
    pub error: T,
}

impl<T> ErrorResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    pub fn new(message: impl Into<String>, error: T) -> Self {
        Self {
            message: message.into(),
            error,
        }
    }
}

impl<T> serde::Serialize for ErrorResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("error", &self.error.to_string())?;
        state.end()
    }
}

impl<T> std::fmt::Display for ErrorResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_json(&self))
    }
}

// Data response
#[derive(serde::Serialize, Debug)]
pub struct DataResponse<T>
where
    T: serde::Serialize,
{
    pub message: String,
    pub data: T,
}

impl<T> DataResponse<T>
where
    T: serde::Serialize,
{
    pub fn new(message: impl Into<String>, data: T) -> Self {
        Self {
            message: message.into(),
            data,
        }
    }
}

impl<T> std::fmt::Display for DataResponse<T>
where
    T: serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_json(&self))
    }
}

// Debug data response
#[derive(Debug)]
pub struct DebugDataResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    pub message: String,
    pub data: T,
}

impl<T> DebugDataResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    pub fn new(message: impl Into<String>, data: T) -> Self {
        Self {
            message: message.into(),
            data,
        }
    }
}

impl<T> serde::Serialize for DebugDataResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DebugData", 2)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("data", &format!("{}:?", self.data))?;
        state.end()
    }
}

impl<T> std::fmt::Display for DebugDataResponse<T>
where
    T: fmt::Debug + fmt::Display + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_json(&self))
    }
}
