# IconWriter
[![Crate](https://img.shields.io/crates/v/iconwriter.svg)](https://crates.io/crates/iconwriter)
[![API](https://docs.rs/iconwriter/badge.svg)](https://docs.rs/iconwriter)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.32+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)

A simple solution for generating `.ico` and `.icns` icons. This crate serves as **[IconWriter CLI's](https://github.com/warrengalyen/iconwriter)** internal library.

## Supported Image Formats
| Format | Supported?                                         | 
| ------ | -------------------------------------------------- | 
| `PNG`  | All supported color types                          | 
| `JPEG` | Baseline and progressive                           | 
| `GIF`  | Yes                                                | 
| `BMP`  | Yes                                                | 
| `ICO`  | Yes                                                | 
| `TIFF` | Baseline(no fax support), `LZW`, PackBits          | 
| `WEBP` | Lossy(Luma channel only)                           | 
| `PNM ` | `PBM`, `PGM`, `PPM`, standard `PAM`                |
| `SVG`  | Limited([flat filled shapes only](#svg-support))   |

## Usage
This crate's API revolves around the concept of binding source images to a set of sizes. The following example demonstrates this principle:

```rust
use iconwriter::prelude::*;
const N_ENTRIES: usize = 1;
fn main() {
    // Creating the icon
    let mut icon = Icon::ico(N_ENTRIES);

    // Importing the source image
    let src_image = SourceImage::from_path("img.jpg").unwrap();

    // Configuring the entry
    let entry = entry![(32, 32), (64, 64)]; // 32x32 and 64x64 sizes

    // Adding the entry
   icon.add_entry(entry, &src_image).unwrap();
}
```

Note that the `capacity` argument in the `Icon::ico`, `Icon::icns`, `Icon::png_sequence` and `Icon::new` methods specifies the expected number of _entries_ in a given `Icon`, **_NOT_** the number of _sizes_ (since a single entry can contain multiple sizes).

### Sampling From Multiple Sources
Let's say you want to customize your icon so that the smaller versions of it are less detailed. **IconWriter** helps you achieve this by allowing to sample from multiple sources.

You can simply combine separate source images by specifying to which entry they should be assigned:

```rust
#[macro_use]
use iconwriter::prelude::*;

fn main() {
    let mut icon = Icon::ico(N_ENTRIES);

    // Importing the source images
    let small = SourceImage::from_path("small.jpg").unwrap();
    let large = SourceImage::from_path("small.png").unwrap();

    // Adding the entries
    icon.add_entry(entry![(16, 16)], &small).unwrap();
    icon.add_entry(entry![(32, 32)], &large).unwrap();
}
```

Note that different entries can share a common source image, but they cannot share a common size. For example, the following program will panic at second call of `icon.add_entry`:

```rust
#[macro_use]
use iconwriter::prelude::*;

const N_ENTRIES: usize = 2;
fn main() {
    let mut icon = Icon::ico(N_ENTRIES);

        // Importing the source images
    let src1 = SourceImage::form_path("src1.jpg").unwrap();
    let src2 = SourceImage::form_path("src2.png").unwrap();

    let entry1 = entry![(16, 16)];
    let entry2 = entry![(16, 16)];

    // Adding the entries
    icon.add_entry(entry1, &src1).expect("Returns Ok(())");
    icon.add_entry(entry2, &src2).expect(
          "Returns Err(Error::SizeAlreadyIncluded((16, 16))))");
}
```

### Rasterizing
Icons can be rasterized to a series of bitmap imaged with the help of the `Icon::rasterize` method. The sources will be scalled using the resampling filter provided in the `resampler` argument. The `iconwriter::resample` mod provides a series of standard resampling filters.

```rust
#[macro_use]
use iconwriter::prelude::*;

const N_ENTRIES: usize = 1;

fn main() {
    // Creating the icon
    let mut icon = Icon::ico(N_ENTRIES);

    // Importing the source image
    let src_image = SourceImage::from_path("img.jpg").unwrap();

    // Configuring the entry
    let entry = entry![(32, 32), (64, 64)]; // 32x32 and 64x64 sizes

    // Adding the entry
    icon.add_entry(entry, &src_image).unwrap();

    // Rasterize the sources
    let rasters = icon.rasterize(iconwriter::resample::linear)
        .unwrap_or_default();
}
```

### Writing to Files
Writing to files can be easily done by calling the [`Icon::write`](https://docs.rs/iconwriter/struct.Icon.html#method.write) method:

```rust
#[macro_use]
use iconwriter::prelude::*;
use std::fs::File;

/* Const declarations */
fn main() {
    let mut icon = Icon::ico(N_ENTRIES);

    /* Process the icon */
    if let Ok(&file) = File::create("myfile.ico") {
             match icon.write(file, iconwriter::resample::linear) {
            Ok(()) => println!("File 'myfile.ico' saved!"),
            Err(_) => println!("An error occurred ;-;")
        }
    }
}
```

Note that the `Icon::write` method can also write to instances of any type which implements [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html).

## Limitations
There are two main limitations in this crate: both `ICNS` and `SVG` are not fully supported. Due to the use of external dependencies, this crate is not able to fully support the formal specifications of those two file formats.

### ICNS Support

However, the coverage provided by these external dependencies should be more than enought for most use cases.

**Icon Writer** uses the `icns` crate for generating `.icns` files. The [supported icon types](https://github.com/mdsteele/rust-icns/blob/master/README.md#supported-icon-types) are specified by the creators of such crate as follows:

| OSType | Description                             | Supported? |
|--------|-----------------------------------------|------------|
| `ICON` | 32×32 1-bit icon                        | No         |
| `ICN#` | 32×32 1-bit icon with 1-bit mask        | No         |
| `icm#` | 16×12 1-bit icon with 1-bit mask        | No         |
| `icm4` | 16×12 4-bit icon                        | No         |
| `icm8` | 16×12 8-bit icon                        | No         |
| `ics#` | 16×16 1-bit mask                        | No         |
| `ics4` | 16×16 4-bit icon                        | No         |
| `ics8` | 16x16 8-bit icon                        | No         |
| `is32` | 16×16 24-bit icon                       | Yes        |
| `s8mk` | 16x16 8-bit mask                        | Yes        |
| `icl4` | 32×32 4-bit icon                        | No         |
| `icl8` | 32×32 8-bit icon                        | No         |
| `il32` | 32x32 24-bit icon                       | Yes        |
| `l8mk` | 32×32 8-bit mask                        | Yes        |
| `ich#` | 48×48 1-bit mask                        | No         |
| `ich4` | 48×48 4-bit icon                        | No         |
| `ich8` | 48×48 8-bit icon                        | No         |
| `ih32` | 48×48 24-bit icon                       | Yes        |
| `h8mk` | 48×48 8-bit mask                        | Yes        |
| `it32` | 128×128 24-bit icon                     | Yes        |
| `t8mk` | 128×128 8-bit mask                      | Yes        |
| `icp4` | 16x16 32-bit PNG/JP2 icon               | PNG only   |
| `icp5` | 32x32 32-bit PNG/JP2 icon               | PNG only   |
| `icp6` | 64x64 32-bit PNG/JP2 icon               | PNG only   |
| `ic07` | 128x128 32-bit PNG/JP2 icon             | PNG only   |
| `ic08` | 256×256 32-bit PNG/JP2 icon             | PNG only   |
| `ic09` | 512×512 32-bit PNG/JP2 icon             | PNG only   |
| `ic10` | 512x512@2x "retina" 32-bit PNG/JP2 icon | PNG only   |
| `ic11` | 16x16@2x "retina" 32-bit PNG/JP2 icon   | PNG only   |
| `ic12` | 32x32@2x "retina" 32-bit PNG/JP2 icon   | PNG only   |
| `ic13` | 128x128@2x "retina" 32-bit PNG/JP2 icon | PNG only   |
| `ic14` | 256x256@2x "retina" 32-bit PNG/JP2 icon | PNG only   |

### SVG Support

**IconWriter** uses the `nsvg` crate for rasterizing `.svg` files. According to the authors of the crate:

> Like NanoSVG, the rasterizer only renders flat filled shapes. It is not particularly fast or accurate, but it is a simple way to bake vector graphics into textures.

The author of `iconwriter` is inclined to search for alternatives to `nsvg` if inquired to. Help would be appreciated. 