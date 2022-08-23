# inside-docker

Detect if code is running inside a docker container.

## How does it work

Check the filesystem type of `/`, if it is [OverlayFS](https://en.wikipedia.org/wiki/OverlayFS) assume the code
is running inside a Docker container.

## Quick Start

```
git clone https://github.com/PicoJr/inside-docker
cd inside-docker/
cargo run --example test-inside-docker
```

output: `inside docker: Some(false)`

vs inside Docker

```
[root@0b132d21e4f0 project]# ./target/debug/examples/test-inside-docker 
```

output: `inside docker: Some(true)`

vs inside Podman

```
[root@97250d70ca08 project]# ./target/debug/examples/test-inside-docker 
```

output: `inside docker: Some(true)`

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

## License

Dual-licensed under MIT or the Apache License V2.0.