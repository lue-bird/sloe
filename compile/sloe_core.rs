#![no_implicit_prelude]
#![allow(
    dead_code,
    non_shorthand_field_patterns,
    non_camel_case_types,
    clippy::needless_pass_by_value,
    clippy::wrong_self_convention,
    clippy::redundant_field_names,
    clippy::type_complexity,
    clippy::match_single_binding,
    clippy::needless_update
)]
extern crate std;
// core //

pub struct A·B<A, B> {
    a: A,
    b: B,
}
pub struct Vec·Range<Vec, Range> {
    vec: Vec,
    range: Range,
}
pub struct Min·Max<Min, Max> {
    min: Min,
    max: Max,
}

pub enum Opt<Present> {
    Absent,
    Present(Present),
}
pub struct Vec<Origin, Element> {
    origin: Origin,
    vec: std::vec::Vec<Element>, // TODO vacated list
}
pub struct Slot<Origin> {
    origin: Origin,
    index: u32,
}
/// important for equality: when length is 0, disregard start
pub type Range<Origin> = Opt<RangeFilled<Origin>>;
pub struct RangeFilled<Origin> {
    origin: Origin,
    start: u32,
    length: std::num::NonZeroU32,
}
pub type Twice<Value> = A·B<Value, Value>;

pub fn vec_sort_range<Origin, Element>(
    vec: Vec<Origin, Element>,
    range: Range<Origin>,
    sort2: fn(A·B<Element, Element>) -> Min·Max<Element, Element>,
) -> Vec·Range<Vec<Origin, Element>, Range<Origin>> {
    std::todo!("is there anything in std that can be used for this?");
    Vec·Range {
        vec: vec,
        range: range,
    }
}
