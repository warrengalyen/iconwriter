//! A simple solution for encoding common icon file-formats, such as `.ico` and `.icns`. 
//! 
//! This crate is mostly a wrapper for other libraries, unifying existing APIs into a single, cohesive 
//! interface.
//! 
//! **IcoWriter** serves as **[Iconiic's](https://github.com/warrengalyen/iconiic)** internal library.
//! 
//! # Overview
//! 
//! An _icon_ consists of a set of _entries_. An _entry_ is simply an image that has a particular size.
//! **IcoWriter** simply automates the process of re-scaling pictures and combining them into an _icon_.
//! 
//! Pictures are scaled using resampling filters, which are represented by _functions that take a source_ 
//! _image and a size and return a re-scaled image_.
//! 
//! This allows the users of this crate to provide their custom resampling filters. Common resampling 
//! filters are provided in the 
//! [`resample`](https://docs.rs/iconwriter/1.6.0/iconwriter/resample/index.html) module.
//! 
//! # Examples
//! 
//! ## General Usage
//! 
//! ```rust, ignore
//! use iconwriter::{Ico, SourceImage, Icon};
//! use iconwriter::Error as IconError;
//!  
//! fn example() -> Result<(), IconError> {
//!     let icon = Ico::new();
//! 
//!     match SourceImage::from_path("image.svg") {
//!         Some(img) => icon.add_entry(resample::linear, &img, 32),
//!         None      => Ok(())
//!     }
//! }
//! ```
//! 
//! ## Writing to a File
//! 
//! ```rust, ignore
//! use iconwriter::*;
//! use std::{io, fs::File};
//!  
//! fn example() -> io::Result<()> {
//!     let icon = PngSequence::new();
//! 
//!     /* Process the icon */
//! 
//!     let file = File::create("out.icns")?;
//!     icon.write(file)
//! }
//! ```
//! 

pub extern crate image;
pub extern crate resvg;

pub use resvg::{usvg, raqote};
use std::{result, error, convert::From, path::Path, io::{self, Write}, fs::File, fmt::{self, Display, Debug}};
use image::{DynamicImage, ImageError, GenericImageView};
use crate::usvg::Tree;

pub use crate::ico::Ico;
pub use crate::icns::Icns;
pub use crate::png_sequence::PngSequence;

type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod test;
mod ico;
mod icns;
mod png_sequence;
pub mod resample;

const INVALID_SIZE_ERROR: &str = "invalid size supplied to the add_entry method";

/// A generic representation of an icon encoder.
pub trait Icon<E: Size + Debug> {
    /// Creates a new icon.
    /// 
    /// # Example
    /// ```rust, ignore
    /// let icon = Ico::new();
    /// ```
    fn new() -> Self;

    /// Adds an individual entry to the icon.
    /// 
    /// # Arguments
    /// * `filter` The resampling filter that will be used to re-scale `source`.
    /// * `source` A reference to the source image this entry will be based on.
    /// * `size` The target size of the entry in pixels.
    /// 
    /// # Return Value
    /// * Returns `Err(Error::InvalidSize(_))` if the dimensions provided in the
    ///  `size` argument are not supported.
    /// * Returns `Err(Error::Image(ImageError::DimensionError))`
    ///  if the resampling filter provided in the `filter` argument produces
    ///  results of dimensions other than the ones specified by `size`.
    /// * Otherwise return `Ok(())`.
    /// 
    /// # Example
    /// ```rust, ignore
    /// use iconwriter::{Ico, SourceImage, Icon};
    /// use iconwriter::Error as IconError;
    ///  
    /// fn example() -> Result<(), IconError> {
    ///     let icon = Ico::new();
    /// 
    ///     match SourceImage::from_path("image.svg") {
    ///         Some(img) => icon.add_entry(resample::linear, &img, 32),
    ///         None      => Ok(())
    ///     }
    /// }
    /// ```
    fn add_entry<F: FnMut(&SourceImage, u32) -> Result<DynamicImage>>(
        &mut self,
        filter: F,
        source: &SourceImage,
        entry: E
    ) -> Result<()>;

    /// Adds a series of entries to the icon.
    /// # Arguments
    /// * `filter` The resampling filter that will be used to re-scale `source`.
    /// * `source` A reference to the source image this entry will be based on.
    /// * `size` A container for the target sizes of the entries in pixels.
    /// 
    /// # Return Value
    /// * Returns `Err(Error::InvalidSize(_))` if the dimensions provided in the
    ///  `size` argument are not supported.
    /// * Returns `Err(Error::Image(ImageError::DimensionError))`
    ///  if the resampling filter provided in the `filter` argument produces
    ///  results of dimensions other than the ones specified by `size`.
    /// * Otherwise return `Ok(())`.
    /// 
    /// # Example
    /// ```rust, ignore
    /// use iconwriter::{Icns, SourceImage, Icon};
    /// use iconwriter::Error as IconError;
    ///  
   /// fn example() -> Result<(), IconError> {
    ///     let icon = Icns::new();
    /// 
    ///     match SourceImage::from_path("image.svg") {
    ///         Some(img) => icon.add_entries(
    ///             resample::linear,
    ///             &img,
    ///             vec![32, 64, 128]
    ///         ),
    ///         None => Ok(())
    ///     }
    /// }
    /// ```
    fn add_entries<F: FnMut(&SourceImage, u32) -> Result<DynamicImage>,I: IntoIterator<Item = E>>(
        &mut self,
        mut filter: F,
        source: &SourceImage,
        entries: I
    ) -> Result<()> {
        for entry in entries {
            self.add_entry(|src, entry| filter(src, entry.size()), source, entry)?;
        }

        Ok(())
    }

    /// Writes the contents of the icon to `w`.
    /// 
    /// # Example
    /// ```rust, ignore
    /// use iconwriter::*;
    /// use std::{io, fs::File};
    ///  
    /// fn example() -> io::Result<()> {
    ///     let icon = PngSequence::new();
    /// 
    ///     /* Process the icon */
    /// 
    ///     let file = File::create("out.icns")?;
    ///     icon.write(file)
    /// }
    /// ```
    fn write<W: Write>(&mut self, w: &mut W) -> io::Result<()>;

    /// Writes the contents of the icon to a file on disk.
    /// 
    /// # Example
    /// ```rust
    /// use iconwriter::*;
    /// use std::{io, fs::File};
    ///  
    /// fn example() -> io::Result<()> {
    ///     let icon = Ico::new();
    /// 
    ///     /* Process the icon */
    /// 
    ///     icon.save("./output/out.ico")
    /// }
    /// ```
    fn save<P: AsRef<Path>>(&mut self, path: &mut P) -> io::Result<()> {
        let mut file = File::create(path.as_ref())?;
        self.write(&mut file)
    }
}

pub trait Size {
    fn size(&self) -> u32;
}

#[derive(Copy, Clone, Debug)]
pub struct Entry(u32);

#[derive(Clone)]
/// A representation of a source image.
pub enum SourceImage {
    /// A generic raster image.
    Raster(DynamicImage),
    /// A svg-encoded vector image.
    Svg(Tree)
}

#[derive(Debug)]
/// The error type for operations of the `Icon` trait.
pub enum Error {
    /// Error from the `usvg` crate.
    Usvg(usvg::Error),
    /// Error from the `image` crate.
    Image(ImageError),
    /// An unsupported size was suplied to an `Icon` operation.
    InvalidSize(u32),
    /// Generic I/O error.
    Io(io::Error)
}

impl<T: Copy + Into<u32>> Size for T {
    fn size(&self) -> u32 {
        (*self).into()
    }
}

impl Size for Entry {
    fn size(&self) -> u32 {
        self.0
    }
}

impl SourceImage {
    /// Attempts to create a `SourceImage` from a given path.
    /// 
    /// The `SourceImage::from::<image::DynamicImage>` and `SourceImage::from::<usvg::Tree>`
    /// methods should always be preferred.
    /// 
    /// # Return Value
    /// * Returns `Some(src)` if the file indicated by the `path` argument could be 
    ///   successfully parsed into an image.
    /// * Returns `None` otherwise.
    /// 
    /// # Example
    /// ```rust, ignore
    /// let img = SourceImage::from_path("source.png")?;
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        if let Ok(ras) = image::open(&path) {
            return Some(SourceImage::from(ras));
        }

        Tree::from_file(&path, &usvg::Options::default())
        .ok().map(|svg| SourceImage::from(svg))
    }

    /// Returns the width of the original image in pixels.
    pub fn width(&self) -> f64 {
        match self {
            SourceImage::Raster(ras) => ras.width() as f64,
            SourceImage::Svg(svg)    => svg.svg_node().view_box.rect.width()
        }
    }

    /// Returns the height of the original image in pixels.
    pub fn height(&self) -> f64 {
        match self {
            SourceImage::Raster(ras) => ras.height() as f64,
            SourceImage::Svg(svg)    => svg.svg_node().view_box.rect.height()
        }
    }

    /// Returns the dimensions of the original image in pixels.
    pub fn dimensions(&self) -> (f64, f64) {
        (self.width(), self.height())
    }
}

impl From<Tree> for SourceImage {
    fn from(svg: Tree) -> Self {
        SourceImage::Svg(svg)
    }
}

impl From<DynamicImage> for SourceImage {
    fn from(bit: DynamicImage) -> Self {
        SourceImage::Raster(bit)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usvg(err)      => write!(f, "{}", err),
            Error::Image(err)     => write!(f, "{}", err),
            Error::Io(err)        => write!(f, "{}", err),
            Error::InvalidSize(_) => write!(f, "{}", INVALID_SIZE_ERROR)
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Usvg(err)      => err.source(),
            Error::Image(err)     => err.source(),
            Error::Io(ref err)    => Some(err),
            Error::InvalidSize(_) => None
        }
    }
}

impl From<usvg::Error> for Error {
    fn from(err: usvg::Error) -> Self {
        Error::Usvg(err)
    }
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::IoError(err) => Error::Io(err),
            err => Error::Image(err)
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        match self {
              Error::Image(ImageError::IoError(err))
            | Error::Io(err) => err,

              Error::InvalidSize(_) 
            | Error::Usvg(usvg::Error::NotAnUtf8Str)
            | Error::Usvg(usvg::Error::InvalidSize)
            | Error::Usvg(usvg::Error::InvalidFileSuffix)
            => io::Error::from(io::ErrorKind::InvalidInput),

              Error::Image(ImageError::DimensionError)
            | Error::Image(ImageError::FormatError(_))
            | Error::Image(ImageError::UnsupportedColor(_))
            | Error::Image(ImageError::UnsupportedError(_))
            | Error::Usvg(usvg::Error::MalformedGZip)
            | Error::Usvg(usvg::Error::ParsingFailed(_))
            => io::Error::from(io::ErrorKind::InvalidData),

              Error::Image(ImageError::ImageEnd)
            | Error::Image(ImageError::NotEnoughData)
            => io::Error::from(io::ErrorKind::UnexpectedEof),

              Error::Image(ImageError::InsufficientMemory)
            | Error::Usvg(usvg::Error::FileOpenFailed)
            => io::Error::from(io::ErrorKind::Other)
        }
    }
}