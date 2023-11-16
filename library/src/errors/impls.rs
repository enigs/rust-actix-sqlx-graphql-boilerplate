use async_graphql::{ErrorExtensions, Value};
use serde_json::json;

use crate::{Errors, Response};
use crate::errors::result::ErrorResult;

impl Errors {
    pub fn to<T>(response: Response, error: T) -> async_graphql::Error
        where T: serde::Serialize
    {
        let errors = Errors {
            errors: Some(Value::from_json(json!(error)).unwrap_or_default()),
            ..Default::default()
        };

        match response {
            Response::BadRequest => errors.to_bad_request(),
            Response::Unauthorized => errors.to_unauthorized(),
            Response::PaymentRequired => errors.to_payment_required(),
            Response::Forbidden => errors.to_forbidden(),
            Response::NotFound => errors.to_not_found(),
            Response::InternalServerError => errors.to_internal_server_error(),
            Response::ErrorWithoutExtensions => errors.to_error_without_extensions(),
        }
    }

    #[allow(dead_code)]
    pub fn bad_request<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::to(Response::BadRequest, error.to_string())
    }

    pub fn to_bad_request(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::BadRequest;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    #[allow(dead_code)]
    pub fn unauthorized<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::to(Response::Unauthorized, error.to_string())
    }

    pub fn to_unauthorized(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::Unauthorized;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    #[allow(dead_code)]
    pub fn payment_required<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::to(Response::PaymentRequired, error.to_string())
    }

    pub fn to_payment_required(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::PaymentRequired;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    #[allow(dead_code)]
    pub fn forbidden<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::to(Response::Forbidden, error.to_string())
    }

    pub fn to_forbidden(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::Forbidden;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    #[allow(dead_code)]
    pub fn not_found<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::to(Response::NotFound, error.to_string())
    }

    pub fn to_not_found(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::NotFound;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    #[allow(dead_code)]
    pub fn internal_server_error<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::to(Response::InternalServerError, error.to_string())
    }

    pub fn to_internal_server_error(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::InternalServerError;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    #[allow(dead_code)]
    pub fn error_without_extensions<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::to(Response::ErrorWithoutExtensions, error.to_string())
    }

    pub fn to_error_without_extensions(&self) -> async_graphql::Error {
        // Set error
        ErrorResult::ErrorWithoutExtensions
            .extend()
    }
}