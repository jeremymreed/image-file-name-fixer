Image File Name Fixer
======================================

### STATUS:
* master:  TODO: CI Integration.
* develop:  TODO: CI Integration.

======================================
### Table of Contents
1. [Purpose](https://github.com/jeremymreed/image-file-name-fixer#purpose)
2. [Installation](https://github.com/jeremymreed/image-file-name-fixer#installation)
3. [Usage](https://github.com/jeremymreed/image-file-name-fixer#usage)
4. [License](https://github.com/jeremymreed/image-file-name-fixer#license)


# Purpose:
This Rust program takes image files, and modifies their names to
conform to a standard naming scheme.

The naming scheme is: (image-name)-(image-resolution).(extension)

To add a sha256 hash sum to the file name, pass the --hash command flag.  The resulting naming scheme is:

(image-name)-(sha256-hash-sum)-(image-resolution).(extension)

To run the app without any destructive changes, pass in --dry-run
as an argument

Files are written in the input directories.

Fields:
image-name: String, just a descriptive string of what the image is.
This tool does not modify this field.

sha256-hash-sum: String.  This is just the sha256 hash for the file. (optional)

image-resolution:  String, in the format of XRESxYRES, where:
  XRES is a numeric string representing the x resolution.
  YRES is a numeric string representing the y resolution.
  These two numeric strings should be separated by a lower case 'x'.  This character is the ascii 'x', not unicode.
  e.g. '1920x1080'

This program should not alter image-name, but ensure that an image-resolution
field exists.  If this field already exists, we check the image to be sure
that the image-resolution field is accurate, and is in the correct
position within the file name.

If it does not exist, we check the image for its resolution, and add the
image-resolution field to the file name in the correct position.

This program uses [image](https://crates.io/crates/image) to get image-resolution information.

# Installation:
This program is packaged for Arch Linux on the AUR.
Run: `yay -Syu image-file-name-fixer` to install.

To install from source, clone this repository and run these commands:
```
cargo build --release --locked
cargo install
```

Cargo will install the `image-file-name-fixer` binary in `$HOME/.cargo/bin`.  Ensure that this directory is on your $PATH.
The AUR package will install to `/usr/bin`.

# Usage:

Help message:
```
Normalizes image file names to standard naming scheme.

Usage: image-file-name-fixer [OPTIONS] [raw_path]

Arguments:
  [raw_path]

Options:
  -m, --move     Move the files instead of copying them.
      --hash     Calculate and include sha256 hashes in file names.
  -d, --decode   Decode image data.
      --dry-run  Don't actually do anything, just print what would happen.
  -h, --help     Print help
  -V, --version  Print version
```

# License:
This program is licensed under the MIT License.
