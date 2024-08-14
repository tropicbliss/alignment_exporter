//! This crate provides alignment info for a user-defined struct, though this crate has increased in scope over time to include other information, such as the type name of struct fields.
//!
//! ```
//! use alignment_exporter::{Alignment, AlignmentExporter, export_alignment};
//!
//! // `export_alignment` already annotates #[repr(C)] to the struct, so adding that yourself is not required. However, it is always better to include it in your code for the sake of explicitness.
//! #[export_alignment]
//! struct Example {
//!     a: u8,
//!     b: u32,
//!     c: u16
//! }
//!
//! fn main() {
//!     let alignment = Example::get_alignment();
//!     assert_eq!(alignment, vec![
//!         Alignment { size: 1, offset: 0, ty_name: "u8" },
//!         Alignment { size: 4, offset: 1, ty_name: "u32" },
//!         Alignment { size: 2, offset: 8, ty_name: "u16" },
//!     ]);
//! }
//! ```

pub use alignment_exporter_derive::export_alignment;

/// Contains alignment info of a field in a struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alignment {
    /// Size of the type in bytes
    pub size: usize,
    /// Starting index of a field in a struct
    pub offset: usize,
    /// Name of type
    pub ty_name: &'static str,
}

/// Any type that uses the procedural macro automatically has this trait implemented for it. Use [`AlignmentExporter::get_alignment`] to get the alignment information of a struct.
pub trait AlignmentExporter {
    /// Get the alignment of a struct.
    fn get_alignment() -> &'static [Alignment];
}
