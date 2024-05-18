# Photo Explorer

Displayed metadata about photos in a given directory.
E.g. ISO, aperture, focal length, shutter speed.

Uses [rexiv2](https://crates.io/crates/rexiv2) for photo metadata.

## Installation

Using Cargo:
```shell
cargo install --path .
```

Requires that you have `gexiv2` installed on your machine.
See the instructions [here](https://github.com/felixc/rexiv2/blob/HEAD/SETUP.md).

## Usage

To find metadata of photos in a directory:
```shell
photo-explorer <directory>
```

To display the average values:
```shell
photo-explorer -a <directory>
```

Help page gives more options:
```shell
photo-explorer --help
```
