# Change Log

## [Unreleased] - ReleaseDate

### Icon Updates

- Icons updated to match `vim-devicons` and correct Material Designs icons
  - Github PR [#899](https://github.com/coreyja/devicon-lookup/pull/899)

### Maintenance

- Various Dependencies Updates
- Clippy and rustfmt changes
  - Github PR [#900](https://github.com/coreyja/devicon-lookup/pull/900)

## [0.8.1] - 2023-02-02

### Dependencies

- Updated `regex` from `1.3.6` to `1.5.4`

Various other dependencies of dependencies, and dev dependencies, were also updated

## [0.8.0] - 2020-04-01

### Features

- Added Ability to use a regex for finding the filename
  - Github PR [#356](https://github.com/coreyja/devicon-lookup/pull/356)
- Added Ability to prefix deliminator for finding the filename
  - Github PR [#356](https://github.com/coreyja/devicon-lookup/pull/356)

### Dependencies

Various other dependencies of dependencies, and dev dependencies, were updated

## [0.7.1] - 2020-03-28

### Dependencies

- Updated `lazy_static` from `1.3.0` to `1.4.0`
- Updated `phf` from `0.7.24` to `0.8.0`
- Updated `regex` from `1.16` to `1.3.6`

Various other dependencies of dependencies, and dev dependencies, were also updated

## [0.7.0] - 2019-06-03

### Features

- Added Icons for Elixir
  - Github PR [#99](https://github.com/coreyja/devicon-lookup/pull/99) thanks [@nifox](https://github.com/nifoc)! :tada:

### Dependencies

- Updated `colored` from `1.7.0` to `1.8.0`
- Updated `regex` from `1.1.5` to `1.1.6`

Various other dependencies of dependencies were also updated

## [0.6.1] - 2019-04-09

### Fixes

- Fixed issue that caused a panic when it encountered a broken pipe
  - Fixes Github Issue #9

### Dependencies

- Updated `regex` from `1.1.2` to `1.1.5`
- [Dev] Updated `assert_cmd` from `0.6.0` to `0.11.1`

Various other dependencies of dependencies were also updated

## [0.6.0] - 2019-03-24

### Improved

- Added option to strip ansi color codes before parsing extension
- Added help and version commands

## [0.5.0] - 2019-03-21

### Fixed

- Went back again to `rust-phf` since that was NOT what made this slow
- Removed my debugging sleep :facepalm:

## [YANKED :cry:] [0.4.0] - 2019-03-21

### Yanked

- Yanked due to having a 1.0 second delay on startup, because of accidental thread sleep

### Improved

- Reverted to `lazy-static` as after releasing I realized `0.3.0` has a noticeable startup lag

### Fixed

- `vi` icon was incorrectly marked as `vim`

## [YANKED :cry:] [0.3.0] - 2019-31-21

### Yanked

- Yanked due to having a 1.0 second delay on startup, because of accidental thread sleep

### Improved

- Using `phf` to do a compile time hash
- Added Tests

## [0.2.0] - 2019-01-28

### Improved

- Use default icon for txt files

## [0.1.0] - 2018-11-17

- Initial Release
