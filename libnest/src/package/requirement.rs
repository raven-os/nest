//! Package requirement, used to find packages matching given criteria

use failure::{Context, Error, ResultExt};
use semver::VersionReq;
use serde_derive::{Deserialize, Serialize};

use super::error::*;
use super::identification::{PackageFullName, PackageID};
use super::REGEX_PACKAGE_ID;
use super::{CategoryName, PackageName, RepositoryName};

/// A structure representing a soft package requirement: parts of a package name and a
/// version requirement.
///
/// Each part may be optional except the package name (you can match, for example, any
/// package named 'gcc' in any category in any repository)
///
/// The version requirement follows SemVer v2
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SoftPackageRequirement {
    repository: Option<RepositoryName>,
    category: Option<CategoryName>,
    name: PackageName,
    version_requirement: VersionReq,
}

impl SoftPackageRequirement {
    /// Creates a package requirement that matches the given [`PackageFullName`] and version requirement
    #[inline]
    pub fn from(full_name: &PackageFullName, version_req: VersionReq) -> SoftPackageRequirement {
        SoftPackageRequirement {
            repository: Some(full_name.repository().clone()),
            category: Some(full_name.category().clone()),
            name: full_name.name().clone(),
            version_requirement: version_req,
        }
    }

    /// Creates a package requirement that matches the given [`PackageFullName`] and version requirement.
    #[inline]
    pub fn from_id(id: &PackageID) -> SoftPackageRequirement {
        SoftPackageRequirement {
            repository: Some(id.repository().clone()),
            category: Some(id.category().clone()),
            name: id.name().clone(),
            version_requirement: VersionReq::exact(id.version()),
        }
    }

    /// Parses a string into a [`SoftPackageRequirement`], or returns a [`PackageRequirementParseError`]
    /// if the parsing failed.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::package::{CategoryName, SoftPackageRequirement};
    ///
    /// let req = SoftPackageRequirement::parse("sys-bin/coreutils#^1.0")?;
    /// assert!(req.repository().is_none());
    /// assert_eq!(*req.category(), Some(CategoryName::parse("sys-bin")?));
    /// assert_eq!(req.name().as_str(), "coreutils");
    /// assert_eq!(req.version_requirement().to_string(), "^1.0");
    ///
    /// assert!(SoftPackageRequirement::parse("sys-bin/coreutils#not_a_version").is_err());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn parse(repr: &str) -> Result<SoftPackageRequirement, Error> {
        let matches = REGEX_PACKAGE_ID
            .captures(repr)
            .ok_or_else(|| Context::from(repr.to_string()))
            .context(SoftPackageRequirementParseErrorKind::InvalidFormat(
                repr.to_string(),
            ))?;

        let version_req = {
            if let Some(req) = matches.name("version") {
                VersionReq::parse(req.as_str())
                    .context(repr.to_string())
                    .context(SoftPackageRequirementParseErrorKind::InvalidVersion)?
            } else {
                VersionReq::any()
            }
        };

        let repository = if let Some(repository) = matches.name("repository") {
            Some(
                RepositoryName::parse(repository.as_str())
                    .map_err(SoftPackageRequirementParseErrorKind::InvalidRepository)?,
            )
        } else {
            None
        };

        let category = if let Some(category) = matches.name("category") {
            Some(
                CategoryName::parse(category.as_str())
                    .map_err(SoftPackageRequirementParseErrorKind::InvalidCategory)?,
            )
        } else {
            None
        };

        Ok(SoftPackageRequirement {
            repository,
            category,
            name: PackageName::parse(matches.name("package").unwrap().as_str())?,
            version_requirement: version_req,
        })
    }

    /// Changes the version requirement to match any version
    #[inline]
    pub fn any_version(mut self) -> Self {
        self.version_requirement = VersionReq::any();
        self
    }

    /// Returns an [`Option`] over the repository part of this package requirement
    #[inline]
    pub fn repository(&self) -> &Option<RepositoryName> {
        &self.repository
    }

    /// Returns an [`Option`] over the category part of this package requirement
    #[inline]
    pub fn category(&self) -> &Option<CategoryName> {
        &self.category
    }

    /// Returns the package name that the target package must have
    #[inline]
    pub fn name(&self) -> &PackageName {
        &self.name
    }

    /// Returns the version requirement that the target package's version must match
    #[inline]
    pub fn version_requirement(&self) -> &VersionReq {
        &self.version_requirement
    }

    /// Tests if a given [`PackageID`] matches this package requirement, matching the name imprecisely
    /// The name of the package only needs to contain the name of the requirement to match
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::package::{PackageID, SoftPackageRequirement};
    ///
    /// let req = SoftPackageRequirement::parse("sys-bin/coreutils#^1.0")?;
    /// let id = PackageID::parse("stable::sys-bin/coreutils#1.0.1").unwrap();
    /// assert!(req.matches(&id));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn matches(&self, id: &PackageID) -> bool {
        let mut out = true;
        if let Some(repository) = &self.repository {
            out &= repository == id.repository();
        }
        if let Some(category) = &self.category {
            out &= category == id.category();
        }
        out && (id.name().contains(self.name.as_ref()))
            && (self.version_requirement.matches(id.version()))
    }

    /// Tests if a given [`PackageID`] matches this package requirement, matching the name precisely
    /// The name of the package needs to be exactly equal to the name of the requirement to match
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::package::{PackageID, SoftPackageRequirement};
    ///
    /// let req = SoftPackageRequirement::parse("sys-bin/coreutils#^1.0")?;
    /// let id = PackageID::parse("stable::sys-bin/coreutils#1.0.1").unwrap();
    /// assert!(req.matches(&id));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn matches_precisely(&self, id: &PackageID) -> bool {
        let mut out = true;
        if let Some(repository) = &self.repository {
            out &= repository == id.repository();
        }
        if let Some(category) = &self.category {
            out &= category == id.category();
        }
        out && (id.name() == &self.name) && (self.version_requirement.matches(id.version()))
    }
}

impl std::fmt::Display for SoftPackageRequirement {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(repository) = &self.repository {
            write!(f, "{}::", repository)?;
        }
        if let Some(category) = &self.category {
            write!(f, "{}/", category)?;
        }
        write!(f, "{}#{}", self.name, self.version_requirement)
    }
}

/// A structure representing a package requirement: parts of a package name and a
/// version requirement.
///
/// Each part may be optional except the category and the package name (you can match,
/// for exemple, any version of a package named 'sys-bin/gcc' in any repository)
///
/// The version requirement follows SemVer v2
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageRequirement {
    repository: Option<RepositoryName>,
    category: CategoryName,
    name: PackageName,
    version_requirement: VersionReq,
}

impl PackageRequirement {
    /// Creates a package requirement that matches the given [`PackageFullName`] and version requirement
    #[inline]
    pub fn from(full_name: &PackageFullName, version_req: VersionReq) -> PackageRequirement {
        PackageRequirement {
            repository: Some(full_name.repository().clone()),
            category: full_name.category().clone(),
            name: full_name.name().clone(),
            version_requirement: version_req,
        }
    }

    /// Creates a package requirement that matches the given [`PackageFullName`] and version requirement.
    #[inline]
    pub fn from_id(id: &PackageID) -> PackageRequirement {
        PackageRequirement {
            repository: Some(id.repository().clone()),
            category: id.category().clone(),
            name: id.name().clone(),
            version_requirement: VersionReq::exact(id.version()),
        }
    }

    /// Parses a string into a [`PackageRequirement`], or returns a [`PackageRequirementParseError`]
    /// if the parsing failed.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::package::{CategoryName, PackageRequirement};
    ///
    /// let req = PackageRequirement::parse("sys-bin/coreutils#^1.0")?;
    /// assert!(req.repository().is_none());
    /// assert_eq!(*req.category(), CategoryName::parse("sys-bin")?);
    /// assert_eq!(req.name().as_str(), "coreutils");
    /// assert_eq!(req.version_requirement().to_string(), "^1.0");
    ///
    /// assert!(PackageRequirement::parse("sys-bin/coreutils#not_a_version").is_err());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn parse(repr: &str) -> Result<PackageRequirement, PackageRequirementParseError> {
        let matches = REGEX_PACKAGE_ID
            .captures(repr)
            .ok_or_else(|| Context::from(repr.to_string()))
            .context(PackageRequirementParseErrorKind::InvalidFormat(
                repr.to_string(),
            ))?;

        match (matches.name("category"), matches.name("package")) {
            (Some(category), Some(package)) => {
                let version_req = {
                    if let Some(req) = matches.name("version") {
                        VersionReq::parse(req.as_str())
                            .context(repr.to_string())
                            .context(PackageRequirementParseErrorKind::InvalidVersion)?
                    } else {
                        VersionReq::any()
                    }
                };

                let repository = if let Some(repository) = matches.name("repository") {
                    Some(
                        RepositoryName::parse(repository.as_str())
                            .map_err(PackageRequirementParseErrorKind::InvalidRepository)?,
                    )
                } else {
                    None
                };

                let category = CategoryName::parse(category.as_str())
                    .map_err(PackageRequirementParseErrorKind::InvalidCategory)?;
                let name = PackageName::parse(package.as_str())
                    .map_err(PackageRequirementParseErrorKind::InvalidName)?;

                Ok(PackageRequirement {
                    repository,
                    category,
                    name,
                    version_requirement: version_req,
                })
            }
            _ => Err(PackageRequirementParseErrorKind::InvalidFormat(repr.to_string()).into()),
        }
    }

    /// Changes the version requirement to match any version
    #[inline]
    pub fn any_version(mut self) -> Self {
        self.version_requirement = VersionReq::any();
        self
    }

    /// Returns an [`Option`] over the repository part of this package requirement
    #[inline]
    pub fn repository(&self) -> &Option<RepositoryName> {
        &self.repository
    }

    /// Returns an [`Option`] over the category part of this package requirement
    #[inline]
    pub fn category(&self) -> &CategoryName {
        &self.category
    }

    /// Returns the package name that the target package must have
    #[inline]
    pub fn name(&self) -> &PackageName {
        &self.name
    }

    /// Returns the version requirement that the target package's version must match
    #[inline]
    pub fn version_requirement(&self) -> &VersionReq {
        &self.version_requirement
    }

    /// Tests if a given [`PackageID`] matches this package requirement, matching the name imprecisely
    /// The name of the package only needs to contain the name of the requirement to match
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::package::{PackageID, PackageRequirement};
    ///
    /// let req = PackageRequirement::parse("sys-bin/coreutils#^1.0")?;
    /// let id = PackageID::parse("stable::sys-bin/coreutils#1.0.1").unwrap();
    /// assert!(req.matches(&id));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn matches(&self, id: &PackageID) -> bool {
        let mut out = true;
        if let Some(repository) = &self.repository {
            out &= repository == id.repository();
        }
        out && (&self.category == id.category())
            && (id.name().contains(self.name.as_ref()))
            && (self.version_requirement.matches(id.version()))
    }

    /// Tests if a given [`PackageID`] matches this package requirement, matching the name precisely
    /// The name of the package needs to be exactly equal to the name of the requirement to match
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::package::{PackageID, PackageRequirement};
    ///
    /// let req = PackageRequirement::parse("sys-bin/coreutils#^1.0")?;
    /// let id = PackageID::parse("stable::sys-bin/coreutils#1.0.1").unwrap();
    /// assert!(req.matches(&id));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn matches_precisely(&self, id: &PackageID) -> bool {
        let mut out = true;
        if let Some(repository) = &self.repository {
            out &= repository == id.repository();
        }
        out && (&self.category == id.category())
            && (id.name() == &self.name)
            && (self.version_requirement.matches(id.version()))
    }
}

impl std::fmt::Display for PackageRequirement {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(repository) = &self.repository {
            write!(f, "{}::", repository)?;
        }
        write!(
            f,
            "{}/{}#{}",
            self.category, self.name, self.version_requirement
        )
    }
}

/// A structure represenging a hard package requirement.
/// This type of requirement is said to be "hard", because only the version has yet to be selected.
/// The other parts of the package information (repository, category and name) are already known.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HardPackageRequirement {
    full_name: PackageFullName,
    version_requirement: VersionReq,
}

impl HardPackageRequirement {
    /// Creates a [`HardPackageRequirement`] from a [`PackageFullName`] and a [`VersionReq`]
    pub fn from(full_name: PackageFullName, version_requirement: VersionReq) -> Self {
        HardPackageRequirement {
            full_name,
            version_requirement,
        }
    }

    /// Returns a reference to the [`PackageFullName`] fixed by this requirement
    #[inline]
    pub fn full_name(&self) -> &PackageFullName {
        &self.full_name
    }

    /// Changes the version requirement to match any version
    #[inline]
    pub fn any_version(mut self) -> Self {
        self.version_requirement = VersionReq::any();
        self
    }

    /// Returns whether the given [`PackageID`] matches this requirement
    #[inline]
    pub fn matches(&self, id: &PackageID) -> bool {
        self.version_requirement.matches(id.version())
    }
}

impl std::fmt::Display for HardPackageRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}#{}", self.full_name, self.version_requirement)
    }
}

impl std::convert::Into<SoftPackageRequirement> for HardPackageRequirement {
    fn into(self) -> SoftPackageRequirement {
        SoftPackageRequirement::from(&self.full_name, self.version_requirement)
    }
}
