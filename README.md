[![Build Status](https://travis-ci.org/Metaswitch/cassandra-sys-rs.svg?branch=master)](https://travis-ci.org/Metaswitch/cassandra-sys-rs)
[![Current Version](http://img.shields.io/crates/v/cassandra-sys-metaswitch.svg)](https://crates.io/crates/cassandra-sys-metaswitch)
[![License](https://img.shields.io/github/license/Metaswitch/cassandra-sys-rs.svg)](#LICENSE)

# cassandra-sys

This is a fork of the source for the cassandra-sys crate on crates.io.

It is mostly an autogenerated wrapper around the DataStax C++ CQL driver.

It also includes a fairly complete set of examples equivalent to the ones in the C++ repository.

It is quite possible to use this crate directly from your rust code, but it will mean littering unsafe all over the place.

Instead it is recommended that you use the safe wrapper of this FFI interface: [cassandra-rs](https://github.com/Metaswitch/cassandra-rs).

## License

This code is open source, licensed under the Apache License Version 2.0 as
described in [`LICENSE`](LICENSE).


## Contributing

Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for details on how to contribute 
to this project.


## Compilation

By default, `/usr/lib`, `/usr/local/lib64`, and `/usr/local/lib` are added to the linker search path.

A semicolon separated list of additional directories to add to the linker search path may be specified through the `CASSANDRA_SYS_LIB_PATH` environment variable.

