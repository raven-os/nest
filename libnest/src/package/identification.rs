use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;
use serde::de::Visitor;

use super::error::{
    CategoryNameParseError, PackageFullNameParseError, PackageFullNameParseErrorKind,
    PackageIDParseError, PackageIDParseErrorKind, PackageNameParseError,
    PackageShortNameParseError, PackageShortNameParseErrorKind, RepositoryNameParseError,
};
use super::REGEX_PACKAGE_ID;

/// Identitier of a package, which is the combination of its full name and its version
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageID {
    full_name: PackageFullName,
    version: Version,
}

impl PackageID {
    /// Creates a [`PackageID`] from a [`PackageFullName`] and a [`Version`]
    #[inline]
    pub fn from(full_name: PackageFullName, version: Version) -> Self {
        Self { full_name, version }
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

impl TryFrom<&str> for PackageID {
    type Error = PackageIDParseError;

    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        let matches = REGEX_PACKAGE_ID
            .captures(repr)
            .ok_or(PackageIDParseErrorKind::InvalidFormat(repr.to_string()))?;

        match (
            matches.name("repository"),
            matches.name("category"),
            matches.name("package"),
            matches.name("version"),
        ) {
            (Some(repository), Some(category), Some(name), Some(version)) => {
                let repository = RepositoryName::try_from(name.as_str()).or(Err(
                    PackageIDParseErrorKind::InvalidRepository(RepositoryNameParseError(
                        repository.as_str().to_string(),
                    )),
                ))?;

                let category = CategoryName::try_from(name.as_str()).or(Err(
                    PackageIDParseErrorKind::InvalidCategory(CategoryNameParseError(
                        category.as_str().to_string(),
                    )),
                ))?;

                let name = PackageName::try_from(name.as_str()).or(Err(
                    PackageIDParseErrorKind::InvalidName(PackageNameParseError(
                        name.as_str().to_string(),
                    )),
                ))?;

                let version = Version::parse(version.as_str())
                    .or(Err(PackageIDParseErrorKind::InvalidVersion))?;

                Ok(PackageID::from(
                    PackageFullName::from(repository, category, name),
                    version,
                ))
            }
            _ => Err(From::from(PackageIDParseErrorKind::InvalidFormat(
                repr.to_string(),
            ))),
        }
    }
}

impl Display for PackageID {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}#{}", self.full_name, self.version,)
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
        PackageID::try_from(value).map_err(|_| {
            E::custom("the package's identifier doesn't follow the convention `repository::category/name#version`")
        })
    }
}

impl_serde_visitor!(PackageID, PackageIDVisitor);

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

impl TryFrom<&str> for PackageFullName {
    type Error = PackageFullNameParseError;

    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        let matches =
            REGEX_PACKAGE_ID
                .captures(repr)
                .ok_or(PackageFullNameParseErrorKind::InvalidFormat(
                    repr.to_string(),
                ))?;

        match (
            matches.name("repository"),
            matches.name("category"),
            matches.name("package"),
            matches.name("version"),
        ) {
            (Some(repository), Some(category), Some(name), None) => {
                let repository = RepositoryName::try_from(name.as_str()).or(Err(
                    PackageFullNameParseErrorKind::InvalidRepository(RepositoryNameParseError(
                        repository.as_str().to_string(),
                    )),
                ))?;

                let category = CategoryName::try_from(name.as_str()).or(Err(
                    PackageFullNameParseErrorKind::InvalidCategory(CategoryNameParseError(
                        category.as_str().to_string(),
                    )),
                ))?;

                let name = PackageName::try_from(name.as_str()).or(Err(
                    PackageFullNameParseErrorKind::InvalidName(PackageNameParseError(
                        name.as_str().to_string(),
                    )),
                ))?;

                Ok(PackageFullName::from(repository, category, name))
            }
            _ => Err(From::from(PackageFullNameParseErrorKind::InvalidFormat(
                repr.to_string(),
            ))),
        }
    }
}

impl Display for PackageFullName {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}::{}/{}", self.repository, self.category, self.name,)
    }
}

struct PackageFullNameVisitor;

impl<'de> Visitor<'de> for PackageFullNameVisitor {
    type Value = PackageFullName;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a package full name")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageFullName::try_from(value).map_err(|_| {
            E::custom(
                "the package's full name doesn't follow the convention `repository::category/name`",
            )
        })
    }
}

impl_serde_visitor!(PackageFullName, PackageFullNameVisitor);

/// Short name of a package, which is the combination of its category and name
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageShortName {
    category: CategoryName,
    name: PackageName,
}

impl PackageShortName {
    /// Creates a [`PackageShortName`] from a [`CategoryName`] and a [`PackageName`]
    #[inline]
    pub fn from(category: CategoryName, name: PackageName) -> Self {
        PackageShortName { category, name }
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

impl TryFrom<&str> for PackageShortName {
    type Error = PackageShortNameParseError;

    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        let matches = REGEX_PACKAGE_ID.captures(repr).ok_or(
            PackageShortNameParseErrorKind::InvalidFormat(repr.to_string()),
        )?;

        match (
            matches.name("repository"),
            matches.name("category"),
            matches.name("package"),
            matches.name("version"),
        ) {
            (None, Some(category), Some(name), None) => {
                let category = CategoryName::try_from(name.as_str()).or(Err(
                    PackageShortNameParseErrorKind::InvalidCategory(CategoryNameParseError(
                        category.as_str().to_string(),
                    )),
                ))?;

                let name = PackageName::try_from(name.as_str()).or(Err(
                    PackageShortNameParseErrorKind::InvalidName(PackageNameParseError(
                        name.as_str().to_string(),
                    )),
                ))?;

                Ok(PackageShortName::from(category, name))
            }
            _ => Err(From::from(PackageShortNameParseErrorKind::InvalidFormat(
                repr.to_string(),
            ))),
        }
    }
}

impl Display for PackageShortName {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.category, self.name,)
    }
}

struct PackageShortNameVisitor;

impl<'de> Visitor<'de> for PackageShortNameVisitor {
    type Value = PackageShortName;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a package short name")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageShortName::try_from(value).map_err(|_| {
            E::custom("the package's short name doesn't follow the convention `category/name`")
        })
    }
}

impl_serde_visitor!(PackageShortName, PackageShortNameVisitor);

/// A package's name.
///
/// A [`&PackageName`] can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a package's name should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageName(String);

impl Display for PackageName {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Deref for PackageName {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PackageName {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for PackageName {
    type Error = PackageNameParseError;

    #[inline]
    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref PACKAGE_NAME_REGEX: Regex = Regex::new(r"^[a-z0-9\-]+$").unwrap();
        }

        if PACKAGE_NAME_REGEX.is_match(repr) {
            Ok(Self(String::from(repr)))
        } else {
            Err(PackageNameParseError(repr.to_string()))
        }
    }
}

struct PackageNameVisitor;

impl<'de> Visitor<'de> for PackageNameVisitor {
    type Value = PackageName;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a package name")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PackageName::try_from(value)
            .map_err(|_| E::custom("the package name doesn't follow the kebab-case"))
    }
}

impl_serde_visitor!(PackageName, PackageNameVisitor);

/// A category's name.
///
/// A [`&CategoryName`] can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a category's name should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CategoryName(String);

impl Display for CategoryName {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Deref for CategoryName {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CategoryName {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for CategoryName {
    type Error = CategoryNameParseError;

    #[inline]
    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref CATEGORY_NAME_REGEX: Regex = Regex::new(r"^[a-z0-9\-]+$").unwrap();
        }

        if CATEGORY_NAME_REGEX.is_match(repr) {
            Ok(Self(String::from(repr)))
        } else {
            Err(CategoryNameParseError(repr.to_string()))
        }
    }
}

struct CategoryNameVisitor;

impl<'de> Visitor<'de> for CategoryNameVisitor {
    type Value = CategoryName;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a category name")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        CategoryName::try_from(value)
            .map_err(|_| E::custom("the category name doesn't follow the kebab-case"))
    }
}

impl_serde_visitor!(CategoryName, CategoryNameVisitor);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// A repository's name.
///
/// A [`&RepositoryName`] can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a repository's name should look like.
pub struct RepositoryName(String);

impl Display for RepositoryName {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Deref for RepositoryName {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RepositoryName {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for RepositoryName {
    type Error = RepositoryNameParseError;

    #[inline]
    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref REPOSITORY_NAME_REGEX: Regex = Regex::new(r"^[a-z0-9\-]+$").unwrap();
        }

        if REPOSITORY_NAME_REGEX.is_match(repr) {
            Ok(Self(String::from(repr)))
        } else {
            Err(RepositoryNameParseError(repr.to_string()))
        }
    }
}

struct RepositoryNameVisitor;

impl<'de> Visitor<'de> for RepositoryNameVisitor {
    type Value = RepositoryName;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a repository name")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        RepositoryName::try_from(value)
            .map_err(|_| E::custom("the repository name doesn't follow the kebab-case"))
    }
}

impl_serde_visitor!(RepositoryName, RepositoryNameVisitor);
