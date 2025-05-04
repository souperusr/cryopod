use serde::Deserialize;
use std::fmt;

pub enum RestMethod {
    Post,
    Get,
    Del,
    Put
}

impl fmt::Display for RestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            RestMethod::Post => "POST",
            RestMethod::Get => "GET",
            RestMethod::Del => "DEL",
            RestMethod::Put => "PUT",
        })
    }
}

/// General error returned by non-200 http codes.
#[derive(Deserialize)]
pub struct ResponseError {
    pub cause: String,
    pub message: String,
    pub response: i64,
}

pub struct PodmanResponse<T: for<'a> Deserialize<'a>> {
    pub status_code: u16,
    pub response: Result<T, ResponseError>,
}

/// Represents what a generic request to Podman looks like. Includes the request type (GET, DEL, PUT, etc.) via RestMethod, a method for getting the POST request, and a struct to deserialize the response into.
pub trait PodmanRequest {
    type Response: for<'a> Deserialize<'a>;
    const REQUEST_METHOD: RestMethod;

    fn get_request(&self) -> String;
}
