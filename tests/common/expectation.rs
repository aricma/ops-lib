use super::error_kind::ErrorKind;
use ops::Task;

#[derive(Debug, Clone)]
pub enum Expectation {
    Model(Vec<Task>),
    Error(ErrorKind),
}
