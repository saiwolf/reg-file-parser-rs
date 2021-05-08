use snafu::Snafu;

/**
Custom Error implementation

Derives: Debug, Snafu

*/
#[derive(Debug, Snafu)]
pub enum Error {
    /**
    Indicates that a HashMap is empty when it shouldn't be.
    */
    #[snafu(display("No Keys returned. Aborting."))]
    HashMapEmpty,
}

/// Custom type alias for use with Snafu
pub type Result<T, E = Error> = std::result::Result<T, E>;