# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.3.2] - 2019-10-15

### Changed

- Updated the documentation to indicate the current behavior of `intrinsics::abort`.

## [v0.3.1] - 2018-09-09

### Added

- `panic-handler` to the list of keywords.

## [v0.3.0] - 2018-09-03

- [breaking-change] Move from the `panic_implementation` attribute to the
  `panic_handler` attribute, which will be stabilized.

## [v0.2.0] - 2018-06-04

### Changed

- [breaking-change] moved from the, now removed, `panic_fmt` lang item to the
  `#[panic_implementation]` attribute.

## [v0.1.1] - 2018-04-09

### Changed

- Tweaked the documentation and the crate metadata (keywords)

## v0.1.0 - 2018-04-09

Initial release

[Unreleased]: https://github.com/japaric/panic-abort/compare/v0.3.1...HEAD
[v0.3.1]: https://github.com/japaric/panic-abort/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/panic-abort/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/panic-abort/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/japaric/panic-abort/compare/v0.1.0...v0.1.1
