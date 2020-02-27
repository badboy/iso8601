# Changelog

<!-- next-header -->

## [Unreleased](https://github.com/badboy/iso8601/compare/v0.3.0...master) - ReleaseDate

* Upgrade to [nom 5](http://unhandledexpression.com/general/2019/06/17/nom-5-is-here.html), getting rid of all parser macros ([#22](https://github.com/badboy/iso8601/pull/22))

## [0.3.0](https://github.com/badboy/iso8601/compare/v0.2.0...v0.3.0) - 2019-01-31

* Add `Display` implementations for exported structures
* Implemented `FromStr` for `Date`, `Time` and `DateTime`
* Upgraded to nom 4
* Formatted everything with `rustfmt`

## [0.2.0](https://github.com/badboy/iso8601/compare/v0.1.0...v0.2.0) - 2017-11-06

* Upgraded nom
* Added fuzzing targets
* Correctly overwrite hours and minutes in `Time#set_tz`
* Correct small error in README example

## 0.1.0 - 2017-11-06

Initial release
