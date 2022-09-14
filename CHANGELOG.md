# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added

### Changed

### Removed

## [0.2.1]
### Added

### Changed
* The error message when using `serde` now indicates that the empty string could not be deserialized.
* Bumped rust edition to `2021`

### Removed
## [0.2.0]
### Added
* `serde` support behind the `serde` feature flag.
* `Eq, PartialEq, Ord, PartialOrd` are now implemented for `NonEmptyString`.
* `get` to retrieve a reference to the inner value.

### Changed
* `new` constructor now returns a `Result` rather than an `Option`, which contains the original string 

### Removed

[Unreleased]: https://github.com/MidasLamb/non-empty-string/v0.2.1...HEAD
[0.2.1]: https://github.com/MidasLamb/non-empty-string/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/MidasLamb/non-empty-string/compare/v0.1.0...v0.2.0