# alignment-exporter

This crate provides alignment info for a user-defined struct, though this crate has increased in scope over time to include other information, such as the type name of struct fields.

Using this crate requires a MSRV of 1.80.

```rs
use alignment_exporter_derive::export_alignment;

// `export_alignment` already annotates #[repr(C)] to the struct, so adding that yourself is not required. However, it is always better to include it in your code for the sake of explicitness.
#[export_alignment]
struct Example {
    a: u8,
    b: u32,
    c: u16
}

fn main() {
    let alignment = Example::get_alignment();
    assert_eq!(alignment, vec![
        Alignment { size: 1, offset: 0, ty_name: "u8" },
        Alignment { size: 4, offset: 1, ty_name: "u32" },
        Alignment { size: 2, offset: 8, ty_name: "u16" },
    ]);
}
```
