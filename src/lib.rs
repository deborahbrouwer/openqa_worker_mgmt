pub mod openqa_vde;
pub mod openqa_worker;
pub mod systemd_escape;

#[derive(Debug, Clone)]
pub struct CustomError(pub String);
impl std::error::Error for CustomError {}
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}