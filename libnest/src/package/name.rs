use std::fmt::{self, Display, Formatter};

use regex::Regex;
use semver::{Version, VersionReq};
use serde::de::Visitor;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use lazy_static::lazy_static;

use crate::error::PackageRequirementParseError;

lazy_static! {
    static ref REGEX_PACKAGE_ID: Regex = Regex::new(
        r"^((?P<repository>[a-z\-]+)::)?((?P<category>[a-z\-]+)/)?(?P<package>([a-z\-*]+))(#((?P<exact_version>[0-9.]+)|(?P<version_req>[<>=~^ ]*\s?[0-9.*]+)))?$"
    ).unwrap();
}

/// A handler other a package's full name, which is the concatenation of it's repository's, category's and package's name.
// XXX Store this as a single string?
// TODO Make a Ref version
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageFullName {
    repository: String,
    category: String,
    name: String,
}

impl PackageFullName {
    /// Creates [`PackageFullName`] from it's repository, category and package's name.
    #[inline]
    pub fn from(repository: String, category: String, name: String) -> PackageFullName {
        PackageFullName {
            repository,
            category,
            name,
        }
    }

    /// Parses a string into a [`PackageFullName`].
    #[inline]
    pub fn parse(s: &str) -> Option<PackageFullName> {
        let matches = REGEX_PACKAGE_ID.captures(s)?;

        match (
            matches.name("repository"),
            matches.name("category"),
            matches.name("package"),
        ) {
            (Some(repository), Some(category), Some(name)) => Some(PackageFullName::from(
                repository.as_str().to_string(),
                category.as_str().to_string(),
                name.as_str().to_string(),
            )),
            _ => None,
        }
    }

    /// Returns a reference other the repository's name
    #[inline]
    pub fn repository(&self) -> &str {
        &self.repository
    }

    /// Returns a reference other the category's name
    #[inline]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Returns a reference other the package's name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for PackageFullName {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}::{}/{}",
            self.repository(),
            self.category(),
            self.name()
        )
    }
}

/// A handler other a package's id, which is the concatenation of it's full name and it's version.
// XXX: Store this as a single string?
// TODO Make a Ref version
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageId {
    full_name: PackageFullName,
    version: Version,
}

impl PackageId {
    /// Creates a [`PackageId`] from a package full name and version.
    #[inline]
    pub fn from(full_name: PackageFullName, version: Version) -> PackageId {
        PackageId { full_name, version }
    }

    /// Parses an `&str` into a [`PackageId`].
    #[inline]
    pub fn parse(s: &str) -> Option<PackageId> {
        let matches = REGEX_PACKAGE_ID.captures(s)?;

        match (
            matches.name("repository"),
            matches.name("category"),
            matches.name("package"),
            matches.name("exact_version"),
        ) {
            (Some(repository), Some(category), Some(name), Some(ver)) => Some(PackageId::from(
                PackageFullName::from(
                    repository.as_str().to_string(),
                    category.as_str().to_string(),
                    name.as_str().to_string(),
                ),
                Version::parse(ver.as_str()).ok()?,
            )),
            _ => None,
        }
    }

    /// Returns the [`PackageFullName`] part of this package id.
    #[inline]
    pub fn full_name(&self) -> &PackageFullName {
        &self.full_name
    }

    /// Returns the [`Version`] part of this package id.
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl Display for PackageId {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}#{}", self.full_name, self.version)
    }
}

/// A structure representing a package requirement: parts of a package name and a
/// version requirement.
///
/// Each parts may be optional except the package name (you can match, for exemple, any
/// package named 'gcc' in any category in any repository)
/// The version requirement follows SemVer v2
// TODO Make a Ref version
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageRequirement {
    repository: Option<String>,
    category: Option<String>,
    name: String,
    version_req: VersionReq,
}

impl PackageRequirement {
    /// Creates a new, empty package requirement that matches any package.
    #[inline]
    pub fn new() -> PackageRequirement {
        PackageRequirement {
            repository: None,
            category: None,
            name: String::new(),
            version_req: VersionReq::any(),
        }
    }

    /// Creates a package requirement that matches the given package full name and version requirement.
    #[inline]
    pub fn from(full_name: &PackageFullName, version_req: VersionReq) -> PackageRequirement {
        PackageRequirement {
            repository: Some(full_name.repository().to_string()),
            category: Some(full_name.category().to_string()),
            name: full_name.name().to_string(),
            version_req,
        }
    }

    /// Creates a package requirement that matches the given package full name and version requirement.
    #[inline]
    pub fn from_id(id: &PackageId) -> PackageRequirement {
        let full_name = id.full_name();
        PackageRequirement {
            repository: Some(full_name.repository().to_string()),
            category: Some(full_name.category().to_string()),
            name: full_name.name().to_string(),
            version_req: VersionReq::exact(id.version()),
        }
    }

    /// Changes the version requirement to match any version
    #[inline]
    pub fn any_version(mut self) -> Self {
        self.version_req = VersionReq::any();
        self
    }

    /// Returns an [`Option`] over the repository part of this package requirement.
    #[inline]
    pub fn repository(&self) -> &Option<String> {
        &self.repository
    }

    /// Returns an [`Option`] over the category part of this package requirement.
    #[inline]
    pub fn category(&self) -> &Option<String> {
        &self.category
    }

    /// Returns the package's name that the target package must have.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the version's requirement that the target package's version must match.
    #[inline]
    pub fn version_req(&self) -> &VersionReq {
        &self.version_req
    }

    /// Parses a string into a [`PackageFUllName`]. Returns a [`PackageRequirementParseError`]
    /// if the parsing failed.
    #[inline]
    pub fn parse(s: &str) -> Result<PackageRequirement, PackageRequirementParseError> {
        let matches = REGEX_PACKAGE_ID
            .captures(s)
            .ok_or_else(|| PackageRequirementParseError::InvalidPackageName(s.to_string()))?;
        let version_req = {
            if let Some(exa) = matches.name("exact_version") {
                VersionReq::parse(&format!("={}", exa.as_str()))
                    .map_err(|_| PackageRequirementParseError::InvalidPackageName(s.to_string()))?
            } else if let Some(req) = matches.name("version_req") {
                VersionReq::parse(req.as_str())
                    .map_err(|_| PackageRequirementParseError::InvalidPackageName(s.to_string()))?
            } else {
                VersionReq::any()
            }
        };
        Ok(PackageRequirement {
            repository: matches.name("repository").map(|m| m.as_str().to_string()),
            category: matches.name("category").map(|m| m.as_str().to_string()),
            name: matches.name("package").unwrap().as_str().to_string(),
            version_req,
        })
    }

    /// Tests if the given [`PackageId`] matches this package requirement.
    #[inline]
    pub fn matches(&self, id: &PackageId) -> bool {
        let mut out = true;
        if let Some(repository) = &self.repository {
            out &= repository == id.full_name().repository();
        }
        if let Some(category) = &self.category {
            out &= category == id.full_name().category();
        }
        out && (self.name == id.full_name().name()) && (self.version_req.matches(id.version()))
    }
}

impl Display for PackageRequirement {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(repository) = &self.repository {
            write!(f, "{}::", repository)?;
        }
        if let Some(category) = &self.category {
            write!(f, "{}/", category)?;
        }
        write!(f, "{}#{}", self.name, self.version_req)
    }
}

impl Default for PackageRequirement {
    fn default() -> PackageRequirement {
        PackageRequirement::new()
    }
}

// The following are implementation of serde's `Serialize` and `Deserialize` traits and their associated visitor.

struct PackageFullNameVisitor;

impl<'de> Visitor<'de> for PackageFullNameVisitor {
    type Value = PackageFullName;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a package name")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageFullName::parse(value).ok_or_else(|| {
            E::custom(
                "the package's full name doesn't follow the convention `repository::category/name`"
                    .to_string(),
            )
        })
    }
}

impl Serialize for PackageFullName {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for PackageFullName {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_str(PackageFullNameVisitor)
    }
}

struct PackageIdVisitor;

impl<'de> Visitor<'de> for PackageIdVisitor {
    type Value = PackageId;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a package identifier")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageId::parse(value).ok_or_else(|| {
            E::custom("the package's full name doesn't follow the convention `repository::category/name#version`".to_string())
        })
    }
}

impl Serialize for PackageId {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for PackageId {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_str(PackageIdVisitor)
    }
}

struct PackageRequirementVisitor;

impl<'de> Visitor<'de> for PackageRequirementVisitor {
    type Value = PackageRequirement;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a package identifier")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageRequirement::parse(value).map_err(|_| {
            E::custom("the package requirement doesn't follow the convention `repository::category/name#version_req`".to_string())
        })
    }
}

impl Serialize for PackageRequirement {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for PackageRequirement {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_str(PackageRequirementVisitor)
    }
}
