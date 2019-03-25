# Change Log

## [Unreleased] - ReleaseDate

### Improved

* Added option to strip ansi color codes before parsing extension
* Added help and version commands

## [0.5.0] - 2019-03-21

### Fixed

* Went back again to `rust-phf` since that was NOT what made this slow
* Removed my debugging sleep :facepalm:

## [YANKED :cry:] [0.4.0] - 2019-03-21

### Yanked

* Yanked due to having a 1.0 second delay on startup, because of accidental thread sleep

### Improved

* Reverted to `lazy-static` as after releasing I realized `0.3.0` has a noticeable startup lag

### Fixed

* `vi` icon was incorrectly marked as `vim`

## [YANKED :cry:] [0.3.0] - 2019-31-21

### Yanked

* Yanked due to having a 1.0 second delay on startup, because of accidental thread sleep

### Improved

* Using `phf` to do a compile time hash
* Added Tests

## [0.2.0] - 2019-01-28

### Improved

* Use default icon for txt files

## [0.1.0] - 2018-11-17

* Initial Release
