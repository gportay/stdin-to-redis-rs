# stdin-to-redis-rs

stdin-to-redis-rs is a throwaway repository to get a first step in the world of
[Rust], and [Redis] but in a very narrow way.

It builds a binary adding the float values read from stdin to a [Time series],
then it publishes the value so it can be read from another binary subscribing
to the same key.

## BUILD AND RUN

Build the binaries from the source tree:

	cargo build

Run the publisher binary and type in float values:

	cargo run --bin publish -- my-time-serie-key

Run the subscriber binary and watch out for the float value evolution:

	cargo run --bin subscribe -- my-time-serie-key

## PATCHES

Sumbit patches at *https://github.com/gportay/stdin-to-redis-rs/pulls*

## BUGS

Report bugs at *https://github.com/gportay/stdin-to-redis-rs/issues*

## AUTHOR

Written by Gaël PORTAY *gael.portay@gmail.com*

## COPYRIGHT

Copyright (c) 2024 Gaël PORTAY

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Lesser General Public License as published by the Free
Software Foundation, either version 2.1 of the License, or (at your option) any
later version.

[Redis]: https://redis.io/
[Rust]: https://www.rust-lang.org/
[Time series]: https://redis.io/docs/latest/develop/data-types/timeseries/
