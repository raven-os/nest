use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::Visitor;
use serde_derive::{Deserialize, Serialize};
use url_serde::SerdeUrl;

use super::error::TagParseError;

/// A package's metadata, like its description, tags, maintainer etc.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    description: String,
    tags: Vec<Tag>,
    maintainer: Maintainer,
    licenses: Vec<License>,
    upstream_url: Option<UpstreamURL>,
}

impl Metadata {
    /// Creates a new, empty [`Metadata`].
    pub fn new() -> Self {
        Self {
            description: String::new(),
            tags: Vec::new(),
            maintainer: Maintainer::new(),
            licenses: Vec::new(),
            upstream_url: None,
        }
    }

    /// Returns a reference over the description of the package
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns a mutable reference over the description of the package
    pub fn description_mut(&mut self) -> &mut str {
        &mut self.description
    }

    /// Returns a reference over the list of tags of the package
    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    /// Returns a mutable reference the list of the tags of the package
    pub fn tags_mut(&mut self) -> &mut Vec<Tag> {
        &mut self.tags
    }

    /// Returns a reference over the maintainer of the package
    pub fn maintainer(&self) -> &Maintainer {
        &self.maintainer
    }

    /// Returns a mutable reference over the maintainer of the package
    pub fn maintainer_mut(&mut self) -> &mut Maintainer {
        &mut self.maintainer
    }

    /// Returns a reference over the list of licenses of the package
    pub fn licenses(&self) -> &Vec<License> {
        &self.licenses
    }

    /// Returns a mutable reference over the licenses of the package
    pub fn licenses_mut(&mut self) -> &mut Vec<License> {
        &mut self.licenses
    }

    /// Returns a reference over the upstream_url of the package
    pub fn upstream_url(&self) -> &Option<UpstreamURL> {
        &self.upstream_url
    }

    /// Returns a mutable reference over the upstream_url of the package
    pub fn upstream_url_mut(&mut self) -> &mut Option<UpstreamURL> {
        &mut self.upstream_url
    }
}

/// A string representing the name of the maintainer and its email address.
pub type Maintainer = String;

/// An URL pointing to the upstream source of the package, usually its home page.
pub type UpstreamURL = SerdeUrl;

/// A Tag describing a package.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Tag(String);

impl Display for Tag {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Deref for Tag {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Tag {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Tag {
    type Error = TagParseError;

    #[inline]
    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref TAG_REGEX: Regex = Regex::new(r"^[a-z0-9\-]+$").unwrap();
        }

        if TAG_REGEX.is_match(repr) {
            Ok(Self(String::from(repr)))
        } else {
            Err(TagParseError(repr.to_string()))
        }
    }
}

struct TagVisitor;

impl<'de> Visitor<'de> for TagVisitor {
    type Value = Tag;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a tag")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Tag::try_from(value).map_err(|_| E::custom("the tag doesn't follow the kebab-case"))
    }
}

impl_serde_visitor!(Tag, TagVisitor);

/// The many licenses a package can be licensed by.
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum License {
    #[serde(rename = "agpl_v3")]
    /// Affero Gnu Public License v3
    AGPL_v3,

    #[serde(rename = "apache")]
    /// Apache License
    Apache,

    #[serde(rename = "bsd")]
    /// BSD License
    BSD,

    #[serde(rename = "custom")]
    /// Custom license
    Custom,

    #[serde(rename = "gpl_v1")]
    /// GNU General Public License v1
    GPL_v1,

    #[serde(rename = "gpl_v2")]
    /// GNU General Public License v2
    GPL_v2,

    #[serde(rename = "gpl_v3")]
    /// GNU General Public License v3
    GPL_v3,

    #[serde(rename = "lgpl_v3")]
    /// Lesser General Public License v3
    LGPL_v3,

    #[serde(rename = "mit")]
    /// MIT License
    MIT,

    #[serde(rename = "mozilla")]
    /// Mozilla License
    Mozilla,

    #[serde(rename = "public_domain")]
    /// Public Domain
    PublicDomain,
}
