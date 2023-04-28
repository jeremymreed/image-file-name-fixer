Image File Name Fixer
======================================

### STATUS:
* master:  TODO: CI Integration.
* develop:  TODO: CI Integration.

======================================
### Table of Contents
1. [Purpose](https://gitlab.com/jeremymreed/image-file-name-fixer#purpose)
2. [Installation](https://gitlab.com/jeremymreed/image-file-name-fixer#installation)
3. [Usage](https://gitlab.com/jeremymreed/image-file-name-fixer#usage)
4. [License](https://gitlab.com/jeremymreed/image-file-name-fixer#license)


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

# Usage:

Help message:
```
usage: image-file-name-fixer [-h] [-i [PATHS ...]] [--version] [--dry-run] [-R] [-m]
                             [--hash] [-v]

optional arguments:
  -h, --help            show this help message and exit
  -i [PATHS ...], --input [PATHS ...]
                        paths to files or directories to be processed.
  --version             Output the version of this software and exit.
  --dry-run             Run normally, but don't write anything to disk
  -R, --recursive       Recursively process image files in a directory.
                        Ignored when input is a file
  -m, --move            Move the file, rather than copying it.
  --hash                Generate hash sums, and append to file name.
  -v, --verbose         Increase output detail. -v for basic output, -vv
                        for detailed output.

```

# License:
This program is licensed under the MIT License.
