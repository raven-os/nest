//! Package identification

use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};

use crate::package::{CategoryName, PackageName, RepositoryName};

lazy_static! {
    static ref REGEX_PACKAGE_ID: Regex = Regex::new(
        r"^((?P<repository>[a-z\-]+)::)?((?P<category>[a-z\-]+)/)?(?P<package>([a-z0-9\-*]+))(#(?P<version>(.+)))?$"
    ).unwrap();
}

/// Full name of a package, which is the combination of its repository, category and name
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageFullName {
    repository: RepositoryName,
    category: CategoryName,
    name: PackageName,
}

impl PackageFullName {
    /// Creates a [`PackageFullName`] from a [`RepositoryName`], a [`CategoryName`] and a [`PackageName`]
    #[inline]
    pub fn from(repository: RepositoryName, category: CategoryName, name: PackageName) -> Self {
        PackageFullName {
            repository,
            category,
            name,
        }
    }

    /// Parses a string into a [`PackageFullName`].
    #[inline]
    pub fn parse(repr: &str) -> Option<PackageFullName> {
        let matches = REGEX_PACKAGE_ID.captures(repr)?;

        match (
            matches.name("repository"),
            matches.name("category"),
            matches.name("package"),
        ) {
            (Some(repository), Some(category), Some(name)) => Some(PackageFullName::from(
                RepositoryName::from(repository.as_str().to_string()),
                CategoryName::from(category.as_str().to_string()),
                PackageName::from(name.as_str().to_string()),
            )),
            _ => None,
        }
    }

    /// Returns a reference over the repository name
    #[inline]
    pub fn repository(&self) -> &RepositoryName {
        &self.repository
    }

    /// Returns a reference over the category name
    #[inline]
    pub fn category(&self) -> &CategoryName {
        &self.category
    }

    /// Returns a reference over the package name
    #[inline]
    pub fn name(&self) -> &PackageName {
        &self.name
    }
}

impl std::fmt::Display for PackageFullName {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}::{}/{}",
            self.repository(),
            self.category(),
            self.name()
        )
    }
}

/// Identity of a package, which is the combination of its full name and its version
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageID {
    full_name: PackageFullName,
    version: Version,
}

impl PackageID {
    /// Creates a [`PackageID`] from a [`PackageFullName`] and a [`Version`]
    #[inline]
    pub fn from(full_name: PackageFullName, version: Version) -> Self {
        PackageID { full_name, version }
    }

    /// Parses a string into a [`PackageId`]
    #[inline]
    pub fn parse(s: &str) -> Option<PackageID> {
        let matches = REGEX_PACKAGE_ID.captures(s)?;

        match (
            matches.name("repository"),
            matches.name("category"),
            matches.name("package"),
            matches.name("version"),
        ) {
            (Some(repository), Some(category), Some(name), Some(ver)) => Some(PackageID::from(
                PackageFullName::from(
                    RepositoryName::from(repository.as_str().to_string()),
                    CategoryName::from(category.as_str().to_string()),
                    PackageName::from(name.as_str().to_string()),
                ),
                Version::parse(ver.as_str()).ok()?,
            )),
            _ => None,
        }
    }

    /// Returns a reference over the package's full name
    #[inline]
    pub fn full_name(&self) -> &PackageFullName {
        &self.full_name
    }

    /// Returns a reference over the package's version
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl std::fmt::Display for PackageID {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}#{}", self.full_name, self.version)
    }
}

struct PackageFullNameVisitor;

impl<'de> Visitor<'de> for PackageFullNameVisitor {
    type Value = PackageFullName;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a fully-qualified package name")
    }

    fn visit_str<E>(self, repr: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageFullName::parse(repr).ok_or_else(|| {
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
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for PackageFullName {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_str(PackageFullNameVisitor)
    }
}

struct PackageIDVisitor;

impl<'de> Visitor<'de> for PackageIDVisitor {
    type Value = PackageID;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a package identifier")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageID::parse(value).ok_or_else(|| {
            E::custom("the package's full name doesn't follow the convention `repository::category/name#version`".to_string())
        })
    }
}

impl Serialize for PackageID {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for PackageID {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_str(PackageIDVisitor)
    }
}
