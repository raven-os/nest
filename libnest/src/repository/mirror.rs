//! Mirrors for a given repository

use std::fmt::{self, Display, Formatter};

use url::Url;

/// A mirror for a given repository.
///
/// It's basically a wrapper arround an [`Url`](https://docs.rs/url/1.7.0/url/struct.Url.html).
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// extern crate url;
///
/// use url::Url;
/// use libnest::repository::Mirror;
///
/// # fn func() -> Result<(), url::ParseError> {
/// let mirror = Mirror::new(Url::parse("http://stable.raven-os.org")?);
///
/// println!("Mirror's url: {}", mirror.url());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Mirror {
    url: Url,
}

impl Mirror {
    /// Creates a new mirror from an [`Url`](https://docs.rs/url/1.7.0/url/struct.Url.html).
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// extern crate url;
    ///
    /// use url::Url;
    /// use libnest::repository::Mirror;
    ///
    /// # fn func() -> Result<(), url::ParseError> {
    /// let m = Mirror::new(Url::parse("http://stable.raven-os.org/")?);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(url: Url) -> Mirror {
        Mirror { url }
    }

    /// Returns a reference to the mirror's [`Url`](https://docs.rs/url/1.7.0/url/struct.Url.html).
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// extern crate url;
    ///
    /// use url::Url;
    /// use libnest::repository::Mirror;
    ///
    /// # fn func() -> Result<(), url::ParseError> {
    /// let m = Mirror::new(Url::parse("http://stable.raven-os.org/")?);
    /// println!("Url is: {}", m.url());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns a mutable reference to the mirror's [`Url`](https://docs.rs/url/1.7.0/url/struct.Url.html).
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// extern crate url;
    ///
    /// use url::Url;
    /// use libnest::repository::Mirror;
    ///
    /// # fn func() -> Result<(), url::ParseError> {
    /// let mut m = Mirror::new(Url::parse("http://stable.raven-os.org/")?);
    /// *m.url_mut() = Url::parse("http://unstable.raven-os.org/")?;
    ///
    /// println!("New url is {}", m.url());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}

impl Display for Mirror {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}
