//! Mirrors for a given repository.

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use url_serde;

/// A mirror for a given repository.
///
/// It's basically a wrapper around an [`Url`](https://docs.rs/url/1.7.0/url/struct.Url.html).
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

impl From<Url> for Mirror {
    fn from(url: Url) -> Mirror {
        Mirror::new(url)
    }
}

// Transparent implementation of Serialize and deserialize for Mirror
impl Serialize for Mirror {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        url_serde::serialize(&self.url, serializer)
    }
}

impl<'de> Deserialize<'de> for Mirror {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        url_serde::deserialize::<Url, _>(deserializer).map(Mirror::from)
    }
}
