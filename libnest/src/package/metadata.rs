use std::convert::TryFrom;

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::Visitor;
use serde_derive::{Deserialize, Serialize};
use url_serde::SerdeUrl;

use super::error::{TagParseError, LicenseParseError};

/// A package's metadata, like its description, tags, maintainer etc.
#[derive(Default, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    description: String,
    tags: Vec<Tag>,
    maintainer: Maintainer,
    licenses: Vec<License>,
    upstream_url: Option<UpstreamURL>,
}

impl Metadata {
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

strong_name_impl!(Tag, r"^[a-z0-9\-]+$", TagParseError);

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

/// The license a package can be licensed by.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct License(String);

strong_name_impl!(License, r"^[a-z0-9_]+$", LicenseParseError);

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = License;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a license")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        License::try_from(value).map_err(|_| E::custom("the license doesn't follow the snake_case"))
    }
}

impl_serde_visitor!(License, LicenseVisitor);
