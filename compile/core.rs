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
// Notes:
// - no value except for arena and vec requires drop for release of memory (not even e.g. elements<> or thread handles).
//   This means all elements in arenas etc can be and should be forgotten (std::mem::forget) at the end of their lexical scope.
//   As even when e.g. an arena contains another arena, the inner arena would drop itself by itself.

#[derive(Clone, Copy, Debug)]
pub struct Start·After<Start, After> {
    pub start: Start,
    pub after: After,
}
#[derive(Clone, Copy, Debug)]
pub struct End·Before<End, Before> {
    pub end: End,
    pub before: Before,
}
#[derive(Clone, Copy, Debug)]
pub struct Environment·fn<Environment, Fn> {
    pub environment: Environment,
    pub fn_: Fn,
}
#[derive(Clone, Copy, Debug)]
pub struct Environment·in<Environment, In> {
    pub environment: Environment,
    pub in_: In,
}
#[derive(Clone, Copy, Debug)]
pub struct Vec·Span<Vec, Span> {
    pub vec: Vec,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Vec·Element<Vec, Element> {
    pub vec: Vec,
    pub element: Element,
}
#[derive(Clone, Copy, Debug)]
pub struct Grown·Shrunk·Span<Grown, Shrunk, Span> {
    pub grown: Grown,
    pub shrunk: Shrunk,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Vec·Elements<Vec, Elements> {
    vec: Vec,
    elements: Elements,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·Elements<Arena, Elements> {
    arena: Arena,
    elements: Elements,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·Element<Arena, Element> {
    pub arena: Arena,
    pub element: Element,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·Slot<Arena, Slot> {
    pub arena: Arena,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Vec·Slot<Vec, Slot> {
    pub vec: Vec,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·Span<Arena, Span> {
    pub arena: Arena,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·Old_element·Slot<Arena, Old_element, Slot> {
    pub arena: Arena,
    pub old_element: Old_element,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Vec·Old_element·Slot<Vec, Old_element, Slot> {
    pub vec: Vec,
    pub old_element: Old_element,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Min·Max<Min, Max> {
    pub min: Min,
    pub max: Max,
}

#[derive(Clone, Copy, Debug)]
pub enum Blank {
    Blank,
}
#[derive(Clone, Copy, Debug)]
pub enum Opt<Present> {
    Absent,
    Present(Present),
}
#[derive(Clone, Debug)]
pub struct Arena<Origin, Element> {
    pub origin: Origin,
    elements: std::vec::Vec<Element>,
}
#[derive(Debug)]
pub struct Vec<Origin, Element> {
    pub origin: Origin,
    elements: std::vec::Vec<Element>,
    // Assumption is that neighboring elements are way more likely to be vacated together.
    // Think e.g. vec_element_span but also
    // regular chunks of nested individual slots which were likely allocated close to their neighbors
    vacant: std::vec::Vec<Span_filled<Origin>>,
    // occupied count = elements.len() - vacant.iter().map(|r| r.length).sum()
}
#[derive(Debug)]
#[non_exhaustive]
pub struct Slot<Origin> {
    pub origin: std::marker::PhantomData<Origin>,
    // consider switching to NonZeroU32 to create a niche for use with Option<Slot<>>
    pub index: u32,
}
pub type Span<Origin> = Opt<Span_filled<Origin>>;
#[derive(Debug)]
#[non_exhaustive]
pub struct Span_filled<Origin> {
    pub start: Slot<Origin>,
    pub length: std::num::NonZeroU32,
}

/// safe if the referenced segment of memory is known to never be accessed again
unsafe fn copy_ref_to_owned<A>(reference: &A) -> A {
    unsafe { std::ptr::NonNull::read(std::ptr::NonNull::from_ref(reference)) }
}

pub fn fn_once_call<Environment, In, Out>(fn_once: Fn_once<Environment, In, Out>, in_: In) -> Out {
    (fn_once.fn_)(Environment·in {
        environment: fn_once.environment,
        in_,
    })
}

/// While its constructor is exposed (because sadly macros (namely origin_new!) require that),
/// I strongly recommend not using it and instead only constructing a new origin with `origin_new!()`
pub struct Origin<const Line: usize, const Column: usize>();
#[macro_export]
macro_rules! origin_new {
    () => {{
        const line: usize = std::line!() as usize;
        const column: usize = std::column!() as usize;
        $crate::core::Origin::<line, column>()
    }};
}
pub use origin_new;

fn non_zero_u32_predecessor(p32: std::num::NonZeroU32) -> u32 {
    p32.get() - 1
}

/// use with caution
fn index_to_slot<Origin>(index: u32) -> Slot<Origin> {
    Slot {
        origin: std::marker::PhantomData::<Origin>,
        index: index,
    }
}

pub fn slot_to_span_filled<Origin>(slot: Slot<Origin>) -> Span_filled<Origin> {
    Span_filled {
        start: slot,
        length: std::num::NonZeroU32::MIN,
    }
}
pub fn span_to_range<Origin>(span: &Span<Origin>) -> std::ops::Range<usize> {
    match span {
        Opt::Absent => <std::ops::Range<usize> as std::default::Default>::default(),
        Opt::Present(span_filled) => span_filled_to_range(span_filled),
    }
}
pub fn span_filled_to_range<Origin>(span: &Span_filled<Origin>) -> std::ops::Range<usize> {
    let start_index = span.start.index as usize;
    start_index..(start_index + span.length.get() as usize)
}
pub fn span_filled_start<Origin>(
    span_filled: Span_filled<Origin>,
) -> Start·After<Slot<Origin>, Span<Origin>> {
    Start·After {
        after: match std::num::NonZeroU32::new(non_zero_u32_predecessor(span_filled.length)) {
            std::option::Option::None => Opt::Absent,
            std::option::Option::Some(after_length) => Opt::Present(Span_filled {
                start: index_to_slot::<Origin>(span_filled.start.index + 1),
                length: after_length,
            }),
        },
        start: span_filled.start,
    }
}
pub fn span_filled_end<Origin>(
    span_filled: Span_filled<Origin>,
) -> End·Before<Slot<Origin>, Span<Origin>> {
    End·Before {
        end: index_to_slot::<Origin>(span_filled_end_index(&span_filled)),
        before: match std::num::NonZeroU32::new(non_zero_u32_predecessor(span_filled.length)) {
            std::option::Option::None => Opt::Absent,
            std::option::Option::Some(before_length) => Opt::Present(Span_filled {
                start: index_to_slot::<Origin>(span_filled.start.index - 1),
                length: before_length,
            }),
        },
    }
}
pub fn span_filled_end_index<Origin>(span_filled: &Span_filled<Origin>) -> u32 {
    span_filled.start.index + span_filled.length.get()
}
pub fn span_length<Origin>(span: Span<Origin>) -> u32 {
    match span {
        Opt::Absent => 0,
        Opt::Present(span_filled) => span_filled.length.get(),
    }
}
// consider adding span_split_after(u32 length_) -> earlier later

// TODO go through all vec_ and arena_ functions and if they take (collection, span) switch to (element_span)
#[derive(Debug, Clone)]
pub struct Element_span<Origin, Element> {
    pub origin: std::marker::PhantomData<Origin>,
    // a raw pointer because dropping should _not_ free the underlying slice
    slice: std::ptr::NonNull<[Element]>,
}
/// the resulting iterator isn't bound to an origin.
/// This is safe when the resulting iterator is consumed before `Origin` goes out of scope.
/// Best consume it immediately
pub unsafe fn element_span_into_iter<'a, Origin, Element: 'a>(
    elements: Element_span<Origin, Element>,
) -> impl std::iter::Iterator<Item = Element> {
    // .slice has exclusive access rights over the elements it references
    unsafe {
        std::iter::Iterator::map(elements.slice.as_ref().iter(), |element_ref| {
            copy_ref_to_owned(element_ref)
        })
    }
}
pub fn element_span_start<Origin, Element>(
    elements: Element_span<Origin, Element>,
) -> Opt<Start·After<Element, Element_span<Origin, Element>>> {
    match unsafe { elements.slice.as_ref() }.split_first() {
        std::option::Option::None => Opt::Absent,
        std::option::Option::Some((start, after)) => Opt::Present(Start·After {
            // since the Element_span.slice holds exclusive access rights
            start: unsafe { copy_ref_to_owned(start) },
            after: Element_span {
                origin: std::marker::PhantomData::<Origin>,
                slice: std::ptr::NonNull::from_ref(after),
            },
        }),
    }
}
// consider adding pub fn element_span_length()

pub fn arena_empty<Origin, Element>(origin: Origin) -> Arena<Origin, Element> {
    Arena {
        origin: origin,
        elements: std::vec::Vec::new(),
    }
}
pub fn arena_element<Origin, Element>(
    arena: Arena<Origin, Element>,
    slot: Slot<Origin>,
) -> Arena·Element<Arena<Origin, Element>, Element> {
    // the .elements never shortened and new slots are bound to this collection origin and contain a known valid index.
    // the element in the slot will never be accessed again, since only one slot exists for it
    let element = unsafe { copy_ref_to_owned(arena.elements.get_unchecked(slot.index as usize)) };
    Arena·Element {
        arena: arena,
        element: element,
    }
}
pub fn arena_element_span<Origin, Element>(
    mut arena: Arena<Origin, Element>,
    span: Span<Origin>,
) -> Arena·Elements<Arena<Origin, Element>, Element_span<Origin, Element>> {
    // the extracted elements are marked as vacated below and not accessed in between.
    // After that, given that at most one valid Span_filled exists for that slice which is required for accessing,
    // that span will never be accessed again.
    let slice = std::ptr::NonNull::from_mut(unsafe {
        arena.elements.get_unchecked_mut(span_to_range(&span))
    });
    Arena·Elements {
        arena: arena,
        elements: Element_span {
            origin: std::marker::PhantomData::<Origin>,
            slice: slice,
        },
    }
}
pub fn arena_add<Origin, Element>(
    mut arena: Arena<Origin, Element>,
    new_element: Element,
) -> Arena·Slot<Arena<Origin, Element>, Slot<Origin>> {
    let added_index = arena.elements.len();
    arena.elements.push(new_element);
    Arena·Slot {
        arena: arena,
        slot: index_to_slot(added_index as u32),
    }
}
pub fn arena_add_element_span<Origin, ElementSpanOrigin, Element>(
    mut arena: Arena<Origin, Element>,
    elements: Element_span<ElementSpanOrigin, Element>,
) -> Arena·Span<Arena<Origin, Element>, Span<Origin>> {
    let span_to_populate = match std::num::NonZeroU32::new(elements.slice.len() as u32) {
        std::option::Option::None => Opt::Absent,
        std::option::Option::Some(elements_length) => Opt::Present(Span_filled {
            start: index_to_slot(arena.elements.len() as u32),
            length: elements_length,
        }),
    };
    std::iter::Extend::extend(
        &mut arena.elements,
        // directly consumed, the Iterator does not outlive Origin
        unsafe { element_span_into_iter(elements) },
    );
    Arena·Span {
        arena: arena,
        span: span_to_populate,
    }
}
pub fn arena_replace<Origin, Element>(
    mut arena: Arena<Origin, Element>,
    slot: Slot<Origin>,
    new_element: Element,
) -> Arena·Old_element·Slot<Arena<Origin, Element>, Element, Slot<Origin>> {
    // the .vec never shortened and new slots are bound to this collection origin and contain a known valid index.
    let old_element = std::mem::replace(
        unsafe { arena.elements.get_unchecked_mut(slot.index as usize) },
        new_element,
    );
    Arena·Old_element·Slot {
        arena: arena,
        old_element: old_element,
        slot: slot,
    }
}
pub fn arena_update<Environment, Origin, Element>(
    mut arena: Arena<Origin, Element>,
    slot: Slot<Origin>,
    element_change: Fn_once<Environment, Element, Element>,
) -> Arena·Slot<Arena<Origin, Element>, Slot<Origin>> {
    // this should just be an in-place edit at one index. rust does not yet have a primitive for this
    let index = slot.index as usize;
    let last_index = arena.elements.len() - 1;
    let element = arena.elements.swap_remove(index);
    arena.elements.push(fn_once_call(element_change, element));
    arena.elements.swap(index, last_index);
    Arena·Slot {
        arena: arena,
        slot: slot,
    }
}

pub fn vec_empty<Origin, Element>(origin: Origin) -> Vec<Origin, Element> {
    Vec {
        origin: origin,
        elements: std::vec::Vec::new(),
        vacant: std::vec::Vec::new(),
    }
}
pub fn vec_element<Origin, Element>(
    mut vec: Vec<Origin, Element>,
    slot: Slot<Origin>,
) -> Vec·Element<Vec<Origin, Element>, Element> {
    // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index.
    // vacated span elements are never accessed, not even while vacating them
    let element = unsafe { copy_ref_to_owned(vec.elements.get_unchecked(slot.index as usize)) };
    vec = vec_vacate_span_filled(vec, slot_to_span_filled(slot));
    Vec·Element {
        element: element,
        vec: vec,
    }
}
pub fn vec_element_span<Origin, Element>(
    mut vec: Vec<Origin, Element>,
    span: Span<Origin>,
) -> Vec·Elements<Vec<Origin, Element>, Element_span<Origin, Element>> {
    // the extracted elements are marked as vacated below and not accessed in between.
    // After that, given that at most one valid Span_filled exists for that slice which is required for accessing,
    // that span will never be accessed again.
    let slice = std::ptr::NonNull::from_mut(unsafe {
        vec.elements.get_unchecked_mut(span_to_range(&span))
    });
    if let Opt::Present(span_filled_to_vacate) = span {
        vec = vec_vacate_span_filled(vec, span_filled_to_vacate);
    };
    Vec·Elements {
        vec: vec,
        elements: Element_span {
            origin: std::marker::PhantomData::<Origin>,
            slice: slice,
        },
    }
}
/// sloe user should prefer `vec_element_span_filled` ignoring the result slot
pub fn vec_vacate_span_filled<Origin, Element>(
    mut vec: Vec<Origin, Element>,
    span_filled_to_vacate: Span_filled<Origin>,
) -> Vec<Origin, Element> {
    let maybe_vacant_span_index_and_length_connecting_earlier: std::option::Option<usize> =
        std::iter::Iterator::rposition(&mut vec.vacant.iter(), |vacant_span| {
            std::cmp::PartialEq::<u32>::eq(
                &span_filled_end_index(&vacant_span),
                &span_filled_to_vacate.start.index,
            )
        });
    let maybe_vacant_span_index_and_length_connecting_later: std::option::Option<usize> =
        std::iter::Iterator::rposition(&mut vec.vacant.iter(), |vacant_span| {
            std::cmp::PartialEq::<u32>::eq(
                &span_filled_end_index(&span_filled_to_vacate),
                &vacant_span.start.index,
            )
        });
    match (
        maybe_vacant_span_index_and_length_connecting_earlier,
        maybe_vacant_span_index_and_length_connecting_later,
    ) {
        (std::option::Option::None, std::option::Option::None) => {
            if (span_filled_to_vacate.start.index + span_filled_to_vacate.length.get() + 1) as usize
                == vec.elements.len()
            {
                vec.elements
                    .truncate(vec.elements.len() - span_filled_to_vacate.length.get() as usize);
            } else {
                vec.vacant.push(span_filled_to_vacate);
            }
        }
        (
            std::option::Option::Some(index_connecting_earlier),
            std::option::Option::Some(index_connecting_later),
        ) => {
            // if spans start connecting now, combine them
            let earlier_span_ref = &vec.vacant[index_connecting_earlier];
            let earlier_span = Span_filled {
                start: index_to_slot(earlier_span_ref.start.index),
                length: earlier_span_ref.length,
            };
            let later_span_to_extend = &mut vec.vacant[index_connecting_later];
            *later_span_to_extend = Span_filled {
                start: earlier_span.start,
                length: std::num::NonZeroU32::saturating_add(
                    std::num::NonZeroU32::saturating_add(
                        earlier_span.length,
                        later_span_to_extend.length.get(),
                    ),
                    span_filled_to_vacate.length.get(),
                ),
            };
            vec.vacant.swap_remove(index_connecting_earlier);
        }
        (std::option::Option::Some(index_connecting_earlier), std::option::Option::None) => {
            let earlier_span_to_extend = &mut vec.vacant[index_connecting_earlier];
            earlier_span_to_extend.length = std::num::NonZeroU32::saturating_add(
                span_filled_to_vacate.length,
                earlier_span_to_extend.length.get(),
            );
        }
        (std::option::Option::None, std::option::Option::Some(index_connecting_after)) => {
            let later_span_to_extend = &mut vec.vacant[index_connecting_after];
            *later_span_to_extend = Span_filled {
                start: span_filled_to_vacate.start,
                length: std::num::NonZeroU32::saturating_add(
                    span_filled_to_vacate.length,
                    later_span_to_extend.length.get(),
                ),
            };
        }
    }
    vec
}
pub fn vec_add<Origin, Element>(
    mut vec: Vec<Origin, Element>,
    new_element: Element,
) -> Vec·Slot<Vec<Origin, Element>, Slot<Origin>> {
    match vec.vacant.pop() {
        std::option::Option::None => {
            let added_index = vec.elements.len();
            vec.elements.push(new_element);
            Vec·Slot {
                vec: vec,
                slot: index_to_slot(added_index as u32),
            }
        }
        std::option::Option::Some(next_vacant_index) => {
            let vacant_start_and_after = span_filled_start(next_vacant_index);
            // since the .elements are never shortened vacant can only be populated with slots with the same collection origin (a known valid index)
            unsafe {
                *vec.elements
                    .get_unchecked_mut(vacant_start_and_after.start.index as usize) = new_element;
            }
            if let Opt::Present(shortened_vacant_span) = vacant_start_and_after.after {
                vec.vacant.push(shortened_vacant_span);
            }
            Vec·Slot {
                vec: vec,
                slot: vacant_start_and_after.start,
            }
        }
    }
}
pub fn vec_add_element_span<Origin, ElementsOrigin, Element>(
    mut vec: Vec<Origin, Element>,
    elements: Element_span<ElementsOrigin, Element>,
) -> Vec·Span<Vec<Origin, Element>, Span<Origin>> {
    let vacant_span_to_reuse_index =
        std::iter::Iterator::rposition(&mut vec.vacant.iter(), |vacant_span| {
            std::cmp::PartialOrd::ge(&(vacant_span.length.get() as usize), &elements.slice.len())
        });
    let index_to_populate_from = match vacant_span_to_reuse_index {
        std::option::Option::None => vec.elements.len() as u32,
        std::option::Option::Some(vacant_span_to_reuse_index) => {
            let vacant_span_to_reuse = &vec.vacant[vacant_span_to_reuse_index];
            let populate_start = vacant_span_to_reuse.start.index;
            if vacant_span_to_reuse.length.get() as usize == elements.slice.len() {
                vec.vacant.swap_remove(vacant_span_to_reuse_index);
            }
            populate_start
        }
    };
    let span_to_populate = match std::num::NonZeroU32::new(elements.slice.len() as u32) {
        std::option::Option::None => Opt::Absent,
        std::option::Option::Some(elements_length) => Opt::Present(Span_filled {
            start: index_to_slot(index_to_populate_from),
            length: elements_length,
        }),
    };
    vec.elements.splice(
        (index_to_populate_from as usize)..(index_to_populate_from as usize + elements.slice.len()),
        // directly consumed, the Iterator does not outlive Origin
        unsafe { element_span_into_iter(elements) },
    );
    Vec·Span {
        vec: vec,
        span: span_to_populate,
    }
}
pub fn vec_replace<Origin, Element>(
    mut vec: Vec<Origin, Element>,
    slot: Slot<Origin>,
    new_element: Element,
) -> Vec·Old_element·Slot<Vec<Origin, Element>, Element, Slot<Origin>> {
    // the .vec never shortened and new slots are bound to this collection origin and contain a known valid index.
    let old_element = std::mem::replace(
        unsafe { vec.elements.get_unchecked_mut(slot.index as usize) },
        new_element,
    );
    Vec·Old_element·Slot {
        vec: vec,
        old_element: old_element,
        slot: slot,
    }
}
pub fn vec_update<Environment, Origin, Element>(
    mut vec: Vec<Origin, Element>,
    slot: Slot<Origin>,
    element_change: Fn_once<Environment, Element, Element>,
) -> Vec·Slot<Vec<Origin, Element>, Slot<Origin>> {
    // this should just be an in-place edit at one index. rust does not yet have a primitive for this
    let index = slot.index as usize;
    let last_index = vec.elements.len() - 1;
    let element = vec.elements.swap_remove(index);
    vec.elements.push(fn_once_call(element_change, element));
    vec.elements.swap(index, last_index);
    Vec·Slot {
        vec: vec,
        slot: slot,
    }
}

pub type Thread<Out> = std::thread::JoinHandle<Out>;

pub type Fn_once<Environment, In, Out> =
    Environment·fn<Environment, fn(Environment·in<Environment, In>) -> Out>;

pub fn spawn_thread_and_run<
    Environment: std::marker::Send + 'static,
    Out: std::marker::Send + 'static,
>(
    run: Fn_once<Environment, Blank, Out>,
) -> Thread<Out> {
    std::thread::spawn(move || fn_once_call(run, Blank::Blank))
}
