# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Types of changes:

- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities

## [Unreleased]

### Added

- Implement recipe editing
- Implement recipe step files (image/video/audio) with optical character recognition
- Add field "quality" to recipe step ingredients
- Implement getting external recipes
- Implement unit conversion

### Changed

- Improve user experience of adding multiple recipe step ingredients
- **BREAKING**: Refactor database migrations to only be one per major version

## [0.0.2] - 2023-02-15

### Added

- Implement deleting of recipe data

## [0.0.1] - 2023-01-16

### Added

- Implement basic GUI and creation and reading of recipes

[unreleased]: https://github.com/Toromyx/recipe-book/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/Toromyx/recipe-book/releases/tag/v0.0.2
[0.0.1]: https://github.com/Toromyx/recipe-book/releases/tag/v0.0.1
