//! Package requirement, used to find packages matching given criteria

use semver::VersionReq;

use super::errors::*;
use super::identification::{PackageFullName, PackageID};
use super::{CategoryName, PackageName, RepositoryName, REGEX_PACKAGE_ID};
use failure::{Context, Error, ResultExt};

/// A structure representing a package requirement: parts of a package name and a
/// version requirement.
///
/// Each parts may be optional except the package name (you can match, for exemple, any
/// package named 'gcc' in any category in any repository)
/// The version requirement follows SemVer v2
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageRequirement {
    repository: Option<RepositoryName>,
    category: Option<CategoryName>,
    name: PackageName,
    version_requirement: VersionReq,
}

impl PackageRequirement {
    /// Creates a new, empty package requirement that matches any package
    #[inline]
    pub fn new() -> Self {
        PackageRequirement {
            repository: None,
            category: None,
            name: PackageName::from(String::new()),
            version_requirement: VersionReq::any(),
        }
    }

    /// Creates a package requirement that matches the given [`PackageFullName`] and version requirement
    #[inline]
    pub fn from(full_name: &PackageFullName, version_req: VersionReq) -> PackageRequirement {
        PackageRequirement {
            repository: Some(full_name.repository().clone()),
            category: Some(full_name.category().clone()),
            name: full_name.name().clone(),
            version_requirement: version_req,
        }
    }

    /// Creates a package requirement that matches the given [`PackageFullName`] and version requirement.
    #[inline]
    pub fn from_id(id: &PackageID) -> PackageRequirement {
        let full_name = id.full_name();
        PackageRequirement {
            repository: Some(full_name.repository().clone()),
            category: Some(full_name.category().clone()),
            name: full_name.name().clone(),
            version_requirement: VersionReq::exact(id.version()),
        }
    }

    /// Parses a string into a [`PackageFullName`], or returns a [`PackageRequirementParseError`]
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
    /// assert_eq!(*req.category(), Some(CategoryName::from("sys-bin".to_string())));
    /// assert_eq!(req.name().as_str(), "coreutils");
    /// assert_eq!(req.version_requirement().to_string(), "^1.0");
    ///
    /// assert!(PackageRequirement::parse("sys-bin/coreutils#not_a_version").is_err());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn parse(repr: &str) -> Result<PackageRequirement, Error> {
        let matches = REGEX_PACKAGE_ID
            .captures(repr)
            .ok_or_else(|| Context::from(repr.to_string()))
            .context(PackageErrorKind::InvalidPackageRequirement)?;

        let version_req = {
            if let Some(req) = matches.name("version") {
                VersionReq::parse(req.as_str())
                    .context(repr.to_string())
                    .context(PackageErrorKind::InvalidPackageRequirement)?
            } else {
                VersionReq::any()
            }
        };
        Ok(PackageRequirement {
            repository: matches
                .name("repository")
                .map(|m| RepositoryName::from(m.as_str().to_string())),
            category: matches
                .name("category")
                .map(|m| CategoryName::from(m.as_str().to_string())),
            name: PackageName::from(matches.name("package").unwrap().as_str().to_string()),
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
    /// use libnest::package::{PackageID, PackageRequirement};
    ///
    /// let req = PackageRequirement::parse("sys-bin/coreutils#^1.0")?;
    /// let id = PackageID::parse("stable::sys-bin/coreutils#1.0.1").unwrap();
    /// assert!(req.matches(&id));
    ///
    /// let any_req = PackageRequirement::new();
    /// assert!(any_req.matches(&id));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn matches(&self, id: &PackageID) -> bool {
        let mut out = true;
        if let Some(repository) = &self.repository {
            out &= repository == id.full_name().repository();
        }
        if let Some(category) = &self.category {
            out &= category == id.full_name().category();
        }
        out && (id.full_name().name().as_str().contains(self.name.as_str()))
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
    ///
    /// let any_req = PackageRequirement::new();
    /// assert!(!any_req.matches_precisely(&id));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn matches_precisely(&self, id: &PackageID) -> bool {
        let mut out = true;
        if let Some(repository) = &self.repository {
            out &= repository == id.full_name().repository();
        }
        if let Some(category) = &self.category {
            out &= category == id.full_name().category();
        }
        out && (id.full_name().name() == &self.name)
            && (self.version_requirement.matches(id.version()))
    }
}

impl std::fmt::Display for PackageRequirement {
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

impl Default for PackageRequirement {
    #[inline]
    fn default() -> PackageRequirement {
        PackageRequirement::new()
    }
}
