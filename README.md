# IconWriter Lib.
[![Crate](https://img.shields.io/crates/v/iconwriter.svg)](https://crates.io/crates/iconwriter)
[![API](https://docs.rs/iconwriter/badge.svg)](https://docs.rs/iconwriter)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.32+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)

A simple solution for generating `.ico` and `.icns` icons. This crate serves as **IconWriter CLI's** internal library.

## Basic usage
```rust
use iconwriter::prelude::*;
const n_entrie: usize = 1;
fn main() {
    // Creating the icon
    let mut icon = Icon::ico(n_entries);
    // Importing the source image
    let source_image = SourceImage::from_file("img.jpg").unwrap();
    // Configuring the entry
    let opts = IconOptions::new(
        vec![(32, 32), (64, 64)] /* 32x32 and 64x64 icon */,
        ResamplingFilter::Linear /* Interpolate the image */,
        Crop::Square             /* Square image */
    );
    // Adding the entry
    icon.add_entry(opts, &source_image).unwrap();
}
```

It is important to note that although the `Icon` returned by the `Icon::ico`, `Icon::icns`, `Icon::png_sequece` and `Icon::new` methods has the capacity specified, the `Icon` will have zero entries.

For an explanation of the difference between length and capacity, see
[*Capacity and reallocation*](https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation).

## Writing to files
```rust
use iconwriter::prelude::*;
use std::fs::File;
const n_entries: usize = ...;
fn main() {
    let mut icon = Icon::ico(n_entries);
    /* Process the icon */
    if let Ok(&file) = File::create("myfile.ico") {
        match icon.write(file) {
            Ok(()) => println!("File 'myfile.ico' saved!"),
            Err(_) => println!("An error occured ;-;")
        }
    }
}
```