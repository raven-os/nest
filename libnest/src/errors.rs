/// Macro to associate Error types to ErrorKind types

#[macro_export]
macro_rules! use_as_error {
    ($Error:ident, $ErrorKind:ident) => {
        /// Forward the fail implementation to the inner context
        impl failure::Fail for $Error {
            fn cause(&self) -> Option<&failure::Fail> {
                self.inner.cause()
            }

            fn backtrace(&self) -> Option<&failure::Backtrace> {
                self.inner.backtrace()
            }
        }

        /// Forward the display to the inner context
        impl std::fmt::Display for $Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.inner, f)
            }
        }

        impl $Error {
            /// Extract the error kind from the error
            pub fn kind(&self) -> $ErrorKind {
                *self.inner.get_context()
            }
        }

        /// Allow converting from an ErrorKind to an Error
        impl From<$ErrorKind> for $Error {
            fn from(kind: $ErrorKind) -> Self {
                $Error {
                    inner: failure::Context::new(kind),
                }
            }
        }

        /// Allow converting from a failure::Context<ErrorKind> to an Error
        impl From<failure::Context<$ErrorKind>> for $Error {
            fn from(inner: failure::Context<$ErrorKind>) -> Self {
                $Error { inner }
            }
        }
    };
}
