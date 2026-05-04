#![no_implicit_prelude]
#![allow(
    dead_code,
    non_shorthand_field_patterns,
    non_camel_case_types,
    non_upper_case_globals,
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
    pub a: A,
    pub b: B,
}
pub struct Environment·fn<Environment, Fn> {
    pub environment: Environment,
    pub fn_: Fn,
}
pub struct Environment·in<Environment, In> {
    pub environment: Environment,
    pub in_: In,
}
pub struct Vec·Range<Vec, Range> {
    pub vec: Vec,
    pub range: Range,
}
pub struct Min·Max<Min, Max> {
    pub min: Min,
    pub max: Max,
}

pub enum Blank {
    Blank,
}
pub enum Opt<Present> {
    Absent,
    Present(Present),
}
pub struct Vec<Origin, Element> {
    pub origin: Origin,
    pub vec: std::vec::Vec<Element>, // TODO vacated list
}
pub struct Slot<Origin> {
    pub origin: Origin,
    pub index: u32,
}
pub type Range<Origin> = Opt<RangeFilled<Origin>>;
pub struct RangeFilled<Origin> {
    origin: Origin,
    start: u32,
    length: std::num::NonZeroU32,
}

pub fn fn_once_call<Environment, In, Out>(fn_once: FnOnce<Environment, In, Out>, in_: In) -> Out {
    (fn_once.fn_)(Environment·in {
        environment: fn_once.environment,
        in_,
    })
}

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

pub type Thread<Out> = std::thread::JoinHandle<Out>;

pub type FnOnce<Environment, In, Out> =
    Environment·fn<Environment, fn(Environment·in<Environment, In>) -> Out>;

pub fn spawn_thread_and_run<
    Environment: std::marker::Send + 'static,
    Out: std::marker::Send + 'static,
>(
    run: FnOnce<Environment, Blank, Out>,
) -> Thread<Out> {
    std::thread::spawn(move || fn_once_call(run, Blank::Blank))
}
// non_exhaustive makes it impossible to construct from outside the crate,
// allowing only origin_new!()
#[non_exhaustive]
pub struct Origin<const Line: usize, const Column: usize>();
#[macro_export]
macro_rules! origin_new {
    () => {{
        const line: usize = std::line!() as usize;
        const column: usize = std::column!() as usize;
        Origin::<line, column> { private: () }
    }};
}
#[macro_export]
macro_rules! origin_new_with_alias {
    (name) => {{
        const line: usize = std::line!() as usize;
        const column: usize = std::column!() as usize;
        Origin::<line, column> { private: () }
    }};
}
