% IMAGE-FILE-NAME-FIXER(1) image-file-name-fixer 0.0.3
% Jeremy M. Reed
% 2021-03-21

# NAME
image-file-name-fixer - A tool to fix image file names.

# SYNOPSIS
**image-file-name-fixer** [OPTIONS] [INPUT]

# DESCRIPTION
This Rust program takes image files, and modifies their names to conform to a standard naming scheme.
The default behavior is to copy files, and does not include sha256 hash sums in the file names.

# OPTIONS
**-h, --help**
: Displays a useful help message.

**--hash**
: Add a sha256 hash sum to the file name.

**--move**
: Moves files rather than copying them.

**--d, --decode**
: Decode image data.  This will result in longer run time, but allows you to catch additional decode errors.

**--dry-run**
: Do not make any changes, but show what would be done.

# EXAMPLES
**image-file-name-fixer --help**
: Displays a useful help message.

**image-file-name-fixer ~/Pictures**
: Fixes the file names in the ~/Pictures directory recursively.  Files are copied.

**image-file-name-fixer --move --hash ~/Pictures**
: Fixes the file names in the ~/Pictures directory recursively, and includes a sha256 hash sum to the file name.  Files are moved.  

**image-file-name-fixer --dry-run ~/Pictures**
: Shows what would be done, but does not make any changes.

# BUGS
[Issue Tracker](https://github.com/jeremymreed/image-file-name-fixer/issues)

# COPYRIGHT
Copyright (C) 2021 Jeremy M. Reed  License: MIT.
