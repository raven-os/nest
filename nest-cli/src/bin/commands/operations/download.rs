use std::io::{Seek, SeekFrom, Write};

use curl::easy::Easy;
use failure::{format_err, Error};
use libnest::config::MirrorUrl;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Download<'a> {
    target_route: &'a str,
}

impl<'a> Download<'a> {
    /// Creates a download from a given route
    pub fn from(target_route: &'a str) -> Self {
        Download { target_route }
    }

    /// Performs the download, using any of the specified mirrors
    pub fn perform_with_mirrors<W>(
        &self,
        writer: &mut W,
        mirrors: &[MirrorUrl],
    ) -> Result<(), Error>
    where
        W: Write + Seek,
    {
        let mut curl = Easy::new();
        curl.follow_location(true)?;
        curl.fail_on_error(true)?;
        curl.progress(true)?;

        let succeeded = mirrors.iter().any(|mirror| {
            let res: Result<_, Error> = try {
                // Overwrite any data from a previous failed attempt
                writer.seek(SeekFrom::Start(0))?;

                let url = mirror.join(self.target_route)?;
                curl.url(url.as_str())?;

                let mut transfer = curl.transfer();
                transfer.write_function(|data| Ok(writer.write(data).unwrap_or(0)))?;
                transfer.perform()?;
            };
            res.is_ok()
        });

        if !succeeded {
            Err(format_err!("no working mirror found"))
        } else {
            Ok(())
        }
    }
}
