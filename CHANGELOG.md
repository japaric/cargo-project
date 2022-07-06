# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.3.0] - 2022-07-06

### Changed

- [breaking-change] changed error handling from `failure` to `thiserror` (`std::error::Error`)

## [v0.2.7] - 2021-09-22

### Fixed

- fixed support for suffixed Cargo configuration file (`.cargo/config.toml`) in presence of Cargo config files in outer directories, like the home directory

## [v0.2.6] - 2021-09-17

### Added

- support for suffixed Cargo configuration file (`.cargo/config.toml`)

## [v0.2.5] - 2021-09-17 - YANKED

## [v0.2.4] - 2020-05-29

### Fixed

- A workspace detection bug on Windows

## [v0.2.3] - 2019-11-14

### Fixed

- Fixed detection of workspaces (globs and relative paths)

## [v0.2.2] - 2019-03-22

### Added

- Added a `toml` method to `Project` to get the path to the project's
  `Cargo.toml`.

## [v0.2.1] - 2018-12-02

### Changed

- `Profile` is now `Copy`

## [v0.2.0] - 2018-10-27

### Changed

- [breaking-change] the signature of `Project.path` has changed to properly
  support file extensions which are required by Windows hosts and WASM targets.

## v0.1.0 - 2018-09-17

First release

[Unreleased]: https://github.com/japaric/cargo-project/compare/v0.3.0...HEAD
[v0.3.0]: https://github.com/japaric/cargo-project/compare/v0.2.7...v0.3.0
[v0.2.7]: https://github.com/japaric/cargo-project/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/japaric/cargo-project/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/japaric/cargo-project/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/japaric/cargo-project/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/japaric/cargo-project/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/japaric/cargo-project/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/japaric/cargo-project/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/cargo-project/compare/v0.1.0...v0.2.0
