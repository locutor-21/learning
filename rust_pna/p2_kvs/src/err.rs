#[derive(Debug, Fail)]
pub enum KvError {
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
}

pub type Result<T> = std::result::Result<T, KvError>;
