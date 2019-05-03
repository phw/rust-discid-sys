# Unsafe FFI bindings for MusicBrainz libdiscid
[![Build Status](https://travis-ci.org/phw/rust-discid-sys.svg?branch=master)](https://travis-ci.org/phw/rust-discid-sys)
[![crates.io](https://img.shields.io/crates/v/discid-sys.svg)](https://crates.io/crates/discid-sys)
[![License](https://img.shields.io/crates/l/discid-sys.svg)](https://crates.io/crates/discid-sys)

## About
discid-sys provides automatically generated, unsafe Rust bindings for
MusicBrainz [libdiscid](http://musicbrainz.org/doc/libdiscid).

You usually don't want to use these bindings directly but instead use the
safe [discid](https://github.com/phw/rust-discid) wrapper library.

## Requirements
* libdiscid >= 0.6.0

## Contribute
The source code for discid-sys is available on
[GitHub](https://github.com/phw/rust-discid-sys).

Please report any issues on the
[issue tracker](https://github.com/phw/rust-discid-sys/issues).

## License
discid-sys Copyright (c) 2019 by Philipp Wolfer <ph.wolfer@gmail.com>

discid-sys is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

See LICENSE for details.
