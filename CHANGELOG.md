# Changelog

## [Unreleased]


## [0.1.4] - 2018-12-05
### Fixed
- Make this crate work with a newer avr compiler


## [0.1.3] - 2018-10-09
### Fixed
- Fixed triggering a compiler segfault by attempting to compile avr assembly
  on non avr architectures. (See <https://github.com/rust-lang/rust/issues/51130>)


## [0.1.2] - 2018-10-07
### Changed
- `downgrade` goes directly to fully generic `Pin` type, the old behaviour
  is available as `downgrade_port`.
