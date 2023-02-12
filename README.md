# uievents-code

[![crates.io](https://img.shields.io/crates/v/uievents-code.svg)][`uievents-code`]
[![crates.io](https://img.shields.io/crates/d/uievents-code.svg)][`uievents-code`]

Crate offering constants for the [`KeyboardEvent.code`] property, according to
the latest (as of 11 February, 2023) [W3C Candidate Recommendation, 01 June 2017].

When using `web_sys`, you can obtain the value of the `KeyboardEvent.code`
property from [`web_sys::KeyboardEvent::code(&self)`].

## Features

* `legacy` (disabled by default) - enables constants for legacy keys.
* `non_standard_intl` (disabled by default) - enables constants for non-standard.
keys on international keyboards.
* `enum` (disabled by default) - enables the `KeyboardEventCode` enum with all
the known constants.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`uievents-code`]: https://crates.io/crates/uievents-code
[`web_sys::KeyboardEvent::code(&self)`]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html#method.code
[W3C Candidate Recommendation, 01 June 2017]: https://www.w3.org/TR/2017/CR-uievents-code-20170601/#code-value-tables
[`KeyboardEvent.code`]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
