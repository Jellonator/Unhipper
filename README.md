# Unhipper
A tool for managing Heavy Iron's HIP/HOP archive file format.

Currently supports Spongebob Squarepants: Battle for Bikini Bottom

## Compiling
To compile, you need to install the latest version of 
[The Rust Programming Language](https://www.rust-lang.org/en-US/), 
and you need to be familiar with command line utilities.

Once you have installed Rust, you need to download this repository and
put it somewhere. Then you should `cd` into the repository's directory
and run the following command:
```
cargo build --release
```

The above command will compile Unhipper; this may take a moment. Once this
has finished, the resulting executable will can be found in the folder 
'target/release/'.

## Usage
### Extracting
To extract a HIP/HOP file, run the following command if you are on Linux:
```
./unhipper extract {path_to_hip} {path_to_extract_to}
```

Or if you are on Windows:
```
unhipper extract {path_to_hip} {path_to_extract_to}
```

Where 'path_to_hip' is a path that refers to a valid HIP/HOP file, and 'path_to_extract_to' is
a path to a directory where you want to extract the HIP/HOP file to.

These commands assume that the Unhipper is in your current working directory. If it is not,
simply copy it from the 'target/release' folder to your current working directory.

### Packing
To pack a directory back into a HIP/HOP file, run the following command if you are on Linux:
```
./unhipper pack {directory_to_pack} {resulting_file}
```

Or if you are on Windows:
```
unhipper pack {directory_to_pack} {resulting_file}
```

Where 'directory_to_pack' is the directory you want to pack back into a HIP/HOP file, and
'resulting_file' is the path to a file that will become the packed HIP/HOP file.

Again, these commands assume that the Unhipper is in your current working directory.

### Examples:
```
./unhipper extract boot.HIP boot_hip_dir
./unhipper pack boot_hip_dir boot.HIP
```

## Directory Structure
The resulting directory has a very specific structure in order to keep metadata and file data
organized. Files are sorted into data types, e.g. 'ANIM' or 'TEXT'. All data types are 4 characters.
Inside of each folder referring to a data type, there are two folders named 'data' and 'meta'. The 'meta'
folder contains metadata that generally should not be tampered with. The 'data' folder contains the actual
contents of each file and can be edited freely.
