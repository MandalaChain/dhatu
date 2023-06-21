#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    CouldNotExtractPort,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {err}"),
            Error::CouldNotExtractPort => write!(
                f,
                "could not extract port from running substrate node's stdout"
            ),
        }
    }
}

impl std::error::Error for Error {}
