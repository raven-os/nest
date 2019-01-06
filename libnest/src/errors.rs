//! Errors that can be returned by libnest

use error_chain::*;

error_chain! {
    errors {
        #[doc = "Error related to the loading of configuration files"]
        ConfigLoad(path: std::path::PathBuf) {
            description("unable to load configuration file")
            display("unable to load configuration file: {}", path.display())
        }
    }
}
