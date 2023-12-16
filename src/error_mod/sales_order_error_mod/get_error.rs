use strum_macros::AsRefStr;

#[derive(Clone, Debug, AsRefStr)]
pub enum GetError {
    // Specific Odbc Error
    FailedSettingConnectionPooling,
    FailedAllocatingEnvironment,
    NoDiagnostics {function: &'static str},
    AbortedConnectionStringCompletion,
    UnableToRepresentNull,
    TooLargeColumnBufferSize{buffer_index: u16, num_elements: usize, element_size: usize},
    TooLargeValueForBuffer{indicator: Option<usize>, buffer_index: usize},

    // General Error
    OdbcErr,

    // Null Value
    NullErr
}

impl From<odbc_api::Error> for GetError {
    fn from(value: odbc_api::Error) -> Self {
        match value {
            odbc_api::Error::FailedSettingConnectionPooling => Self::FailedSettingConnectionPooling,
            odbc_api::Error::FailedAllocatingEnvironment => Self::FailedAllocatingEnvironment,
            odbc_api::Error::NoDiagnostics {function} => Self::NoDiagnostics {function},
            odbc_api::Error::AbortedConnectionStringCompletion => Self::AbortedConnectionStringCompletion,
            odbc_api::Error::TooLargeColumnBufferSize { buffer_index, num_elements, element_size } => Self::TooLargeColumnBufferSize {buffer_index, num_elements, element_size},
            odbc_api::Error::TooLargeValueForBuffer { indicator, buffer_index } => Self::TooLargeValueForBuffer {indicator, buffer_index},
            _ => Self::OdbcErr
        }
    }
}
