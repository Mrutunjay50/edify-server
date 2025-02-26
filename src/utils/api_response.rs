use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    status: u16,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(status: u16, message: &str, data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            status,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(status: u16, message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            status,
            message: message.to_string(),
            data: None, // No need for Option<T> since T is already ()
        }
    }
}
