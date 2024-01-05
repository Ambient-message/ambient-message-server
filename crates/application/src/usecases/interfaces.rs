use domain::api_error::ApiError;

pub trait AbstractUseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}