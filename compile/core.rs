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

#[derive(Clone, Copy, Debug)]
pub struct A·b<A, B> {
    pub a: A,
    pub b: B,
}
#[derive(Clone, Copy, Debug)]
pub struct P·u<P, U> {
    pub p: P,
    pub u: U,
}
#[derive(Clone, Copy, Debug)]
pub struct Max·min<Max, Min> {
    pub max: Max,
    pub min: Min,
}
#[derive(Clone, Copy, Debug)]
pub struct Build·new<Build, New> {
    pub build: Build,
    pub new: New,
}
#[derive(Clone, Copy, Debug)]
pub struct Element·in<Element, In> {
    pub element: Element,
    pub in_: In,
}
#[derive(Clone, Copy, Debug)]
pub struct Element·out<Element, Out> {
    pub element: Element,
    pub out: Out,
}
#[derive(Clone, Copy, Debug)]
pub struct After·start<After, Start> {
    pub after: After,
    pub start: Start,
}
#[derive(Clone, Copy, Debug)]
pub struct Before·end<Before, End> {
    pub before: Before,
    pub end: End,
}
#[derive(Clone, Copy, Debug)]
pub struct Apart·connected<Apart, Connected> {
    pub apart: Apart,
    pub connected: Connected,
}
#[derive(Clone, Copy, Debug)]
pub struct Index·slot<Index, Slot> {
    pub index: Index,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Slot·state<Slot, State> {
    pub slot: Slot,
    pub state: State,
}
#[derive(Clone, Copy, Debug)]
pub struct Char·state<Char, State> {
    pub char: Char,
    pub state: State,
}
#[derive(Clone, Copy, Debug)]
pub struct Length·span<Length, Span> {
    pub length: Length,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Exit·remaining<Exit, Remaining> {
    pub exit: Exit,
    pub remaining: Remaining,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·length<Arena, Length> {
    pub arena: Arena,
    pub length: Length,
}
#[derive(Clone, Copy, Debug)]
pub struct Length·vec<Length, Vec> {
    pub length: Length,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Slot·vec<Slot, Vec> {
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·slot<Arena, Slot> {
    pub arena: Arena,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·new<Arena, New> {
    pub arena: Arena,
    pub new: New,
}
#[derive(Clone, Copy, Debug)]
pub struct New·vec<New, Vec> {
    pub new: New,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct New·slot·vec<New, Slot, Vec> {
    pub new: New,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·new·slot<Arena, New, Slot> {
    pub arena: Arena,
    pub new: New,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Span·vec<Span, Vec> {
    pub span: Span,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·span<Arena, Span> {
    pub arena: Arena,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Out·slot·vec<Out, Slot, Vec> {
    pub out: Out,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·out·slot<Arena, Out, Slot> {
    pub arena: Arena,
    pub out: Out,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Grown·shrunk·span<Grown, Shrunk, Span> {
    pub grown: Grown,
    pub shrunk: Shrunk,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Element·vec<Element, Vec> {
    pub element: Element,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·element<Arena, Element> {
    pub arena: Arena,
    pub element: Element,
}
#[derive(Clone, Copy, Debug)]
pub struct Arena·old·slot<Arena, Old_element, Slot> {
    pub arena: Arena,
    pub old: Old_element,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Old·slot·vec<Old_element, Slot, Vec> {
    pub old: Old_element,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub enum Absent·Present<Absent, Present> {
    Absent(Absent),
    Present(Present),
}
#[derive(Clone, Copy, Debug)]
pub enum Exit·Go_on<Exit, Go_on> {
    Exit(Exit),
    Go_on(Go_on),
}

#[derive(Clone, Copy, Debug)]
pub struct Blank {}
#[derive(Clone, Copy, Debug)]
pub enum Never {}

pub type P32 = std::num::NonZeroU32;
pub type U32 = u32;
pub type I32 = i32;
pub type F32 = f32;
pub type Char = char;
pub type Str = &'static str;
pub type Fn<In, Out> = fn(In) -> Out;
pub type Opt<Present> = Absent·Present<Blank, Present>;
pub type Exit_or_go_on<Exit, Go_on> = Exit·Go_on<Exit, Go_on>;

#[derive(Debug)]
pub struct Origin<LocalType>(LocalType);
#[derive(Debug)]
pub struct Arena<LocalOrigin, Element> {
    pub origin: Origin<LocalOrigin>,
    elements: std::vec::Vec<Element>,
}
#[derive(Debug)]
pub struct Vec<LocalOrigin, Element> {
    pub origin: Origin<LocalOrigin>,
    elements: std::vec::Vec<Element>,
    // Assumption is that neighboring elements are way more likely to be vacated together.
    // Think e.g. vec_elements but also
    // regular chunks of nested individual slots which were likely allocated close to their neighbors
    vacant: std::vec::Vec<SpanRaw>,
    // occupied count = elements.len() - vacant.iter().map(|r| r.length.get()).sum()
}
#[derive(Debug, Clone, Copy)]
struct SpanRaw {
    start: u32,
    length: std::num::NonZeroU32,
}
#[non_exhaustive]
pub struct Slot<LocalOrigin> {
    pub origin: std::marker::PhantomData<LocalOrigin>,
    // consider switching to NonZeroU32 to create a niche for use with Option<Slot<>>
    pub index: u32,
}
#[non_exhaustive]
pub struct Span<LocalOrigin> {
    pub start: Slot<LocalOrigin>,
    // consider instead: end_index: NonZeroU32.
    // This makes combining 2 opt_spans and converting to ops::Range a bit faster.
    pub length: std::num::NonZeroU32,
}
#[derive(Debug)]
#[non_exhaustive]
pub struct Opt_span_build<Backing> {
    pub backing: Backing,
    pub start: u32,
}
#[derive(Debug)]
#[non_exhaustive]
pub struct Span_build<Backing> {
    pub backing: Backing,
    pub start: u32,
}

impl<Origin> std::fmt::Debug for Slot<Origin> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slot").field("index", &self.index).finish()
    }
}
impl<Origin> std::fmt::Debug for Span<Origin> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("start", &self.start)
            .field("length", &self.length)
            .finish()
    }
}

impl<A> Opt<A> {
    pub fn from_option(option: std::option::Option<A>) -> Self {
        match option {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(present) => Opt::Present(present),
        }
    }
    pub fn into_option(self) -> std::option::Option<A> {
        match self {
            Opt::Absent(Blank {}) => std::option::Option::None,
            Opt::Present(present) => std::option::Option::Some(present),
        }
    }
    pub fn as_ref(&self) -> Opt<&A> {
        match self {
            Opt::Absent(Blank {}) => Opt::Absent(Blank {}),
            Opt::Present(present) => Opt::Present(present),
        }
    }
}

impl<Exit, GoOn> Exit_or_go_on<Exit, GoOn> {
    pub fn from_control_flow(control_flow: std::ops::ControlFlow<Exit, GoOn>) -> Self {
        match control_flow {
            std::ops::ControlFlow::Break(exit) => Exit_or_go_on::Exit(exit),
            std::ops::ControlFlow::Continue(go_on) => Exit_or_go_on::Go_on(go_on),
        }
    }
    pub fn into_control_flow(self) -> std::ops::ControlFlow<Exit, GoOn> {
        match self {
            Exit_or_go_on::Exit(exit) => std::ops::ControlFlow::Break(exit),
            Exit_or_go_on::Go_on(go_on) => std::ops::ControlFlow::Continue(go_on),
        }
    }
}

/// safe if the referenced segment of memory is known to never be accessed again
/// and the resulting iterator is dropped before the mutable reference goes out of scope.
/// Best to consume immediately
unsafe fn mut_slice_into_owned_iterator<'a, A>(slice: &'a mut [A]) -> OwnedSliceIterator<'a, A> {
    OwnedSliceIterator {
        ref_mut_iterator: slice.iter_mut(),
    }
}
// constructing is unsafe, use mut_slice_into_owned_iterator!
pub struct OwnedSliceIterator<'a, Element> {
    ref_mut_iterator: std::slice::IterMut<'a, Element>,
}
impl<'a, Element> std::iter::Iterator for OwnedSliceIterator<'a, Element> {
    type Item = Element;
    fn next(&mut self) -> std::option::Option<Self::Item> {
        // usage is safe when constructor is safe, see mut_slice_into_owned_iterator
        self.ref_mut_iterator
            .next()
            .map(|element_ref| unsafe { copy_ref_to_owned(element_ref) })
    }
}

/// This constructor is exposed because sadly macros (namely origin_new!) require it.
/// It's _very strongly_ recommended to instead only construct new origins with `origin_new!`.
/// Misusing this constructor can lead to UB like unchecked out of bounds access.
pub unsafe fn origin_new_use_macro_instead<LocalType>(
    local_type_instance: LocalType,
) -> Origin<LocalType> {
    Origin(local_type_instance)
}
#[macro_export]
macro_rules! origin_new {
    ($variable_name:ident, $type_name:ident) => {
        struct $type_name();
        let $variable_name = unsafe { $crate::core::origin_new_use_macro_instead($type_name()) };
    };
}
pub use origin_new;

impl<Origin, Element> Arena<Origin, Element> {
    /// Especially when working with estimates or future insertions, you usually want pre_allocate_at_least
    pub fn pre_allocate(&mut self, pre_allocated_length: u32) {
        self.elements.reserve_exact(pre_allocated_length as usize);
    }
    pub fn pre_allocate_at_least(&mut self, min_pre_allocated_length: u32) {
        self.elements.reserve(min_pre_allocated_length as usize);
    }
    pub fn element<'a>(&'a self, slot: &Slot<Origin>) -> &'a Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe { self.elements.get_unchecked(slot.index as usize) }
    }
    pub fn element_mut<'a>(&'a mut self, slot: &mut Slot<Origin>) -> &'a mut Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe { self.elements.get_unchecked_mut(slot.index as usize) }
    }
    pub fn opt_span_slice<'a>(&'a self, opt_span: Opt<&Span<Origin>>) -> &'a [Element] {
        match opt_span {
            Opt::Absent(Blank {}) => &[],
            Opt::Present(span) => self.span_slice(span),
        }
    }
    pub fn span_slice<'a>(&'a self, span: &Span<Origin>) -> &'a [Element] {
        // the .elements never shortened and new opt_spans are bound to this collection origin and contain a known valid range
        unsafe { self.elements.get_unchecked(span.to_range()) }
    }
    pub fn span_iterator_mut<'a>(
        &'a mut self,
        mut shrink_span: Span<Origin>,
    ) -> OwnedSliceIterator<'a, Element> {
        // elements in the opt_span are consumed and never accessed after.
        // While iterating, the elements are "locked" behind a mut ref
        unsafe { mut_slice_into_owned_iterator(self.span_slice_mut(&mut shrink_span)) }
    }
    pub fn opt_span_slice_mut<'a>(
        &'a mut self,
        opt_span: &mut Opt<Span<Origin>>,
    ) -> &'a mut [Element] {
        match opt_span {
            Opt::Absent(Blank {}) => &mut [],
            Opt::Present(span) => self.span_slice_mut(span),
        }
    }
    pub fn span_slice_mut<'a>(&'a mut self, span: &mut Span<Origin>) -> &'a mut [Element] {
        // the .elements never shortened and new opt_spans are bound to this collection origin and contain a known valid range
        unsafe { self.elements.get_unchecked_mut(span.to_range()) }
    }
    pub fn add(&mut self, new_element: Element) -> Slot<Origin> {
        let added_index = self.elements.len();
        self.elements.push(new_element);
        index_to_slot(added_index as u32)
    }
    pub fn add_iterator(
        &mut self,
        new_elements: impl std::iter::ExactSizeIterator<Item = Element>,
    ) -> Opt<Span<Origin>> {
        match std::num::NonZeroU32::new(new_elements.len() as u32) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(new_element_count) => {
                Opt::Present(self.add_iterator_filled(new_elements, new_element_count))
            }
        }
    }
    // invariant! new_element_count must equal new_elements.count()
    fn add_iterator_filled(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
        new_element_count: std::num::NonZeroU32,
    ) -> Span<Origin> {
        let grow_span = Span {
            start: index_to_slot(self.elements.len() as u32),
            length: new_element_count,
        };
        std::iter::Extend::extend(&mut self.elements, new_elements);
        grow_span
    }
    pub fn add_vec_span<ToShrinkOrigin>(
        &mut self,
        to_shrink: &mut Vec<ToShrinkOrigin, Element>,
        shrink_span: Span<ToShrinkOrigin>,
    ) -> Span<Origin> {
        let shrink_length = shrink_span.length;
        to_shrink.vacate_and_consume_span_iterator(shrink_span, |owned_slice_iterator| {
            self.add_iterator_filled(owned_slice_iterator, shrink_length)
        })
    }
    pub fn update<In, Out>(
        &mut self,
        slot: &mut Slot<Origin>,
        in_: In,
        element_visit: Fn<Element·in<Element, In>, Element·out<Element, Out>>,
    ) -> Out {
        // this should just be an in-place edit at one index. rust does not yet have a primitive for this
        let index = slot.index as usize;
        let last_index = self.elements.len() - 1;
        let element = self.elements.swap_remove(index);
        let element_updated = element_visit(Element·in {
            element: element,
            in_: in_,
        });
        self.elements.push(element_updated.element);
        self.elements.swap(index, last_index);
        element_updated.out
    }
}

impl<Origin, Element> Vec<Origin, Element> {
    /// Especially when working with estimates or future insertions, you usually want pre_allocate_at_least
    pub fn pre_allocate(&mut self, pre_allocated_length: u32) {
        self.elements.reserve_exact(pre_allocated_length as usize);
    }
    pub fn pre_allocate_at_least(&mut self, min_pre_allocated_length: u32) {
        self.elements.reserve(min_pre_allocated_length as usize);
    }
    pub fn element<'a>(&'a self, slot: &Slot<Origin>) -> &'a Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe { self.elements.get_unchecked(slot.index as usize) }
    }
    pub fn element_mut<'a>(&'a mut self, slot: &mut Slot<Origin>) -> &'a mut Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe { self.elements.get_unchecked_mut(slot.index as usize) }
    }
    pub fn opt_span_slice<'a>(&'a self, opt_span: Opt<&Span<Origin>>) -> &'a [Element] {
        match opt_span {
            Opt::Absent(Blank {}) => &[],
            Opt::Present(span) => self.span_slice(span),
        }
    }
    pub fn span_slice<'a>(&'a self, span: &Span<Origin>) -> &'a [Element] {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid range
        unsafe { self.elements.get_unchecked(span.to_range()) }
    }
    pub fn opt_span_slice_mut<'a>(
        &'a mut self,
        opt_span: &mut Opt<Span<Origin>>,
    ) -> &'a mut [Element] {
        match opt_span {
            Opt::Absent(Blank {}) => &mut [],
            Opt::Present(span) => self.span_slice_mut(span),
        }
    }
    pub fn span_slice_mut<'a>(&'a mut self, span: &mut Span<Origin>) -> &'a mut [Element] {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid range
        unsafe { self.elements.get_unchecked_mut(span.to_range()) }
    }
    pub fn vacate_and_consume_span_iterator<Out>(
        &mut self,
        mut shrink_span: Span<Origin>,
        consume_iterator: impl for<'iterator> std::ops::FnOnce(
            OwnedSliceIterator<'iterator, Element>,
        ) -> Out,
    ) -> Out {
        // elements in the opt_span are consumed and never accessed after. During this whole ordeal
        // the elements are "locked" behind a mut ref
        let munched = consume_iterator(unsafe {
            mut_slice_into_owned_iterator(self.span_slice_mut(&mut shrink_span))
        });
        self.span_vacate(shrink_span);
        munched
    }
    /// only use when the element values are safe to not handle
    pub fn span_vacate(&mut self, span_to_vacate: Span<Origin>) {
        let maybe_vacant_opt_span_index_and_length_connecting_earlier: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |&vacant_opt_span| {
                std::cmp::PartialEq::<u32>::eq(
                    &span_raw_end_index(vacant_opt_span).get(),
                    &span_to_vacate.start.index,
                )
            });
        let maybe_vacant_opt_span_index_and_length_connecting_later: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_opt_span| {
                std::cmp::PartialEq::<u32>::eq(
                    &span_to_vacate.end_index().get(),
                    &vacant_opt_span.start,
                )
            });
        match (
            maybe_vacant_opt_span_index_and_length_connecting_earlier,
            maybe_vacant_opt_span_index_and_length_connecting_later,
        ) {
            (std::option::Option::None, std::option::Option::None) => {
                if (span_to_vacate.start.index + span_to_vacate.length.get() + 1) as usize
                    == self.elements.len()
                {
                    self.elements
                        .truncate(self.elements.len() - span_to_vacate.length.get() as usize);
                } else {
                    self.vacant.push(SpanRaw {
                        start: span_to_vacate.start.index,
                        length: span_to_vacate.length,
                    });
                }
            }
            (
                std::option::Option::Some(index_connecting_earlier),
                std::option::Option::Some(index_connecting_later),
            ) => {
                // if opt_spans start connecting now, combine them
                let earlier_opt_span = self.vacant[index_connecting_earlier];
                let later_opt_span_to_extend = &mut self.vacant[index_connecting_later];
                *later_opt_span_to_extend = SpanRaw {
                    start: earlier_opt_span.start,
                    length: std::num::NonZeroU32::saturating_add(
                        std::num::NonZeroU32::saturating_add(
                            earlier_opt_span.length,
                            later_opt_span_to_extend.length.get(),
                        ),
                        span_to_vacate.length.get(),
                    ),
                };
                self.vacant.swap_remove(index_connecting_earlier);
            }
            (std::option::Option::Some(index_connecting_earlier), std::option::Option::None) => {
                let earlier_opt_span_to_extend = &mut self.vacant[index_connecting_earlier];
                earlier_opt_span_to_extend.length = std::num::NonZeroU32::saturating_add(
                    span_to_vacate.length,
                    earlier_opt_span_to_extend.length.get(),
                );
            }
            (std::option::Option::None, std::option::Option::Some(index_connecting_after)) => {
                let later_opt_span_to_extend = &mut self.vacant[index_connecting_after];
                *later_opt_span_to_extend = SpanRaw {
                    start: span_to_vacate.start.index,
                    length: std::num::NonZeroU32::saturating_add(
                        span_to_vacate.length,
                        later_opt_span_to_extend.length.get(),
                    ),
                };
            }
        }
    }
    pub fn add_ignoring_vacated(&mut self, new_element: Element) -> Slot<Origin> {
        let added_index = self.elements.len();
        self.elements.push(new_element);
        index_to_slot(added_index as u32)
    }
    pub fn add(&mut self, new_element: Element) -> Slot<Origin> {
        match self.vacant.pop() {
            std::option::Option::None => self.add_ignoring_vacated(new_element),
            std::option::Option::Some(vacant_opt_span_to_populate) => {
                unsafe {
                    *self
                        .elements
                        .get_unchecked_mut(vacant_opt_span_to_populate.start as usize) =
                        new_element;
                }
                if let std::option::Option::Some(remaining_length) = std::num::NonZeroU32::new(
                    non_zero_u32_predecessor(vacant_opt_span_to_populate.length),
                ) {
                    self.vacant.push(SpanRaw {
                        start: vacant_opt_span_to_populate.start + 1,
                        length: remaining_length,
                    });
                }
                index_to_slot(vacant_opt_span_to_populate.start)
            }
        }
    }
    fn mark_length_filled_as_occupied(
        &mut self,
        element_count: std::num::NonZeroU32,
    ) -> std::option::Option<u32> {
        let vacant_opt_span_to_reuse_index =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_opt_span| {
                std::cmp::PartialOrd::ge(&vacant_opt_span.length, &element_count)
            });
        match vacant_opt_span_to_reuse_index {
            std::option::Option::None => std::option::Option::None,
            std::option::Option::Some(vacant_opt_span_to_reuse_index) => {
                let vacant_opt_span_to_reuse = self.vacant[vacant_opt_span_to_reuse_index];
                if vacant_opt_span_to_reuse.length == element_count {
                    self.vacant.swap_remove(vacant_opt_span_to_reuse_index);
                }
                std::option::Option::Some(vacant_opt_span_to_reuse.start)
            }
        }
    }
    // invariant! new_element_count must equal new_elements.count()
    fn add_iterator_filled(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
        new_element_count: std::num::NonZeroU32,
    ) -> Span<Origin> {
        match self.mark_length_filled_as_occupied(new_element_count) {
            std::option::Option::None => {
                let length_without_new_elements = self.elements.len() as u32;
                std::iter::Extend::extend(&mut self.elements, new_elements);
                Span {
                    start: index_to_slot(length_without_new_elements),
                    length: new_element_count,
                }
            }
            std::option::Option::Some(index_to_populate_from) => {
                let grow_span = Span {
                    start: index_to_slot(index_to_populate_from),
                    length: new_element_count,
                };
                self.elements.splice(grow_span.to_range(), new_elements);
                grow_span
            }
        }
    }
    pub fn add_iterator(
        &mut self,
        new_elements: impl std::iter::ExactSizeIterator<Item = Element>,
    ) -> Opt<Span<Origin>> {
        match std::num::NonZeroU32::new(new_elements.len() as u32) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(new_element_count) => {
                Opt::Present(self.add_iterator_filled(new_elements, new_element_count))
            }
        }
    }
    pub fn add_iterator_without_known_size(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element> + std::clone::Clone,
    ) -> Opt<Span<Origin>> {
        let std::option::Option::Some(grow_length) =
            std::num::NonZeroU32::new(std::iter::Iterator::count(new_elements.clone()) as u32)
        else {
            return Opt::Absent(Blank {});
        };
        let grow_span = self.add_iterator_filled(new_elements, grow_length);
        Opt::Present(grow_span)
    }
    pub fn add_arena_span<ShrinkOrigin>(
        &mut self,
        to_shrink: &mut Arena<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        let shrink_span_length = shrink_span.length;
        self.add_iterator_filled(to_shrink.span_iterator_mut(shrink_span), shrink_span_length)
    }
    pub fn add_vec_span<ShrinkOrigin>(
        &mut self,
        shrink: &mut Vec<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        let shrink_span_length = shrink_span.length;
        let grow_span = shrink.vacate_and_consume_span_iterator(shrink_span, |shrink_elements| {
            self.add_iterator_filled(shrink_elements, shrink_span_length)
        });
        grow_span
    }
}
impl<Origin> Vec<Origin, Char> {
    pub fn add_str(&mut self, new_str: Str) -> Opt<Span<Origin>> {
        self.add_iterator_without_known_size(new_str.chars())
    }
}

impl<Origin, Element> Span_build<Vec<Origin, Element>> {
    pub fn length(&self) -> std::num::NonZeroU32 {
        // Span_build is only returned from functions adding >= 1 element
        // and span_build.start is `.elements.len() as u32` from before adding
        unsafe {
            std::num::NonZeroU32::new_unchecked(self.backing.elements.len() as u32 - self.start)
        }
    }
}

impl<Origin> Span<Origin> {
    pub fn to_range(&self) -> std::ops::Range<usize> {
        let start_index = self.start.index as usize;
        start_index..(start_index + self.length.get() as usize)
    }
    pub fn to_range_u32(&self) -> std::ops::Range<u32> {
        self.start.index..(self.start.index + self.length.get())
    }
    pub fn end_index(&self) -> std::num::NonZeroU32 {
        std::num::NonZeroU32::saturating_add(self.length, self.start.index)
    }
}

impl<'a, Origin> Opt<&'a Span<Origin>> {
    pub fn to_range(self) -> std::ops::Range<usize> {
        match self {
            Opt::Absent(Blank {}) => <std::ops::Range<usize> as std::default::Default>::default(),
            Opt::Present(span) => span.to_range(),
        }
    }
    pub fn to_range_u32(self) -> std::ops::Range<u32> {
        match self {
            Opt::Absent(Blank {}) => <std::ops::Range<u32> as std::default::Default>::default(),
            Opt::Present(span) => span.to_range_u32(),
        }
    }
    pub fn length(self) -> u32 {
        match self {
            Opt::Absent(Blank {}) => 0,
            Opt::Present(span) => span.length.get(),
        }
    }
}

fn non_zero_u32_predecessor(p32: std::num::NonZeroU32) -> u32 {
    p32.get() - 1
}

pub fn p32_dup(n: P32) -> A·b<P32, P32> {
    A·b { a: n, b: n }
}
pub fn p32_rid(_: P32) -> Blank {
    Blank {}
}
pub fn p32_predecessor(n: P32) -> U32 {
    non_zero_u32_predecessor(n)
}
pub fn p32_add(A·b { a, b }: A·b<P32, U32>) -> P32 {
    a.saturating_add(b)
}
pub fn p32_to_u32(n: P32) -> U32 {
    n.get()
}
pub fn u32_to_p32(n: U32) -> Opt<P32> {
    Opt::from_option(P32::new(n))
}
pub fn u32_rid(_: U32) -> Blank {
    Blank {}
}
pub fn u32_dup(n: U32) -> A·b<U32, U32> {
    A·b { a: n, b: n }
}
pub fn u32_to_f32(n: U32) -> F32 {
    n as F32
}
pub fn u32_add(A·b { a, b }: A·b<U32, U32>) -> U32 {
    a.saturating_add(b)
}
pub fn i32_dup(n: I32) -> A·b<I32, I32> {
    A·b { a: n, b: n }
}
pub fn i32_rid(_: I32) -> Blank {
    Blank {}
}
pub fn i32_to_f32(n: I32) -> F32 {
    n as F32
}
pub fn i32_to_u32(n: I32) -> Opt<U32> {
    match <U32 as std::convert::TryFrom<I32>>::try_from(n) {
        std::result::Result::Err(_) => Opt::Absent(Blank {}),
        std::result::Result::Ok(u) => Opt::Present(u),
    }
}
pub fn i32_to_p32(n: I32) -> Opt<P32> {
    match <U32 as std::convert::TryFrom<I32>>::try_from(n) {
        std::result::Result::Err(_) => Opt::Absent(Blank {}),
        std::result::Result::Ok(u) => u32_to_p32(u),
    }
}
pub fn i32_abs_u32(n: I32) -> U32 {
    n.unsigned_abs()
}
pub fn i32_negate(n: I32) -> I32 {
    -n
}
pub fn f32_dup(n: F32) -> A·b<F32, F32> {
    A·b { a: n, b: n }
}
pub fn f32_rid(_: F32) -> Blank {
    Blank {}
}
pub fn f32_add(A·b { a, b }: A·b<F32, F32>) -> F32 {
    a + b
}
pub fn f32_mul(A·b { a, b }: A·b<F32, F32>) -> F32 {
    a * b
}
pub fn f32_truncate_to_i32(n: F32) -> I32 {
    n as I32
}

pub fn fn_dup<In, Out>(fn_: Fn<In, Out>) -> A·b<Fn<In, Out>, Fn<In, Out>> {
    A·b { a: fn_, b: fn_ }
}
pub fn fn_rid<In, Out>(_: Fn<In, Out>) -> Blank {
    Blank {}
}

pub fn char_dup(char: Char) -> A·b<Char, Char> {
    A·b { a: char, b: char }
}
pub fn char_rid(_: Char) -> Blank {
    Blank {}
}

pub fn str_dup(str: Str) -> A·b<Str, Str> {
    A·b { a: str, b: str }
}
pub fn str_rid(_: Str) -> Blank {
    Blank {}
}
pub fn str_byte_count(str: Str) -> u32 {
    str.len() as u32
}
pub fn str_char_count(str: Str) -> u32 {
    std::iter::Iterator::count(str.chars()) as u32
}
pub fn str_start(str: Str) -> Opt<After·start<Str, Char>> {
    let mut chars = str.chars();
    Opt::from_option(std::iter::Iterator::next(&mut chars).map(|c| After·start {
        start: c,
        after: chars.as_str(),
    }))
}
pub fn str_end(str: Str) -> Opt<Before·end<Str, Char>> {
    let mut chars = str.chars();
    Opt::from_option(
        std::iter::Iterator::next(&mut std::iter::Iterator::rev(&mut chars)).map(|c| Before·end {
            end: c,
            before: chars.as_str(),
        }),
    )
}
pub fn str_chars_fold<State>(
    str: Str,
    initial_state: State,
    reduce: Fn<Char·state<Char, State>, State>,
) -> State {
    std::iter::Iterator::fold(&mut str.chars(), initial_state, |so_far, char| {
        reduce(Char·state {
            state: so_far,
            char,
        })
    })
}
pub fn str_chars_fold_backwards<State>(
    str: Str,
    initial_state: State,
    reduce: Fn<Char·state<Char, State>, State>,
) -> State {
    std::iter::Iterator::fold(
        &mut std::iter::Iterator::rev(str.chars()),
        initial_state,
        |so_far, char| {
            reduce(Char·state {
                state: so_far,
                char,
            })
        },
    )
}
pub fn str_chars_fold_while<Exit, GoOn>(
    str: Str,
    initial_state: GoOn,
    reduce: Fn<Char·state<Char, GoOn>, Exit_or_go_on<Exit, GoOn>>,
) -> Exit_or_go_on<Exit, GoOn> {
    Exit_or_go_on::from_control_flow(std::iter::Iterator::try_fold(
        &mut str.chars(),
        initial_state,
        |so_far, char| {
            Exit_or_go_on::into_control_flow(reduce(Char·state {
                state: so_far,
                char,
            }))
        },
    ))
}
pub fn str_chars_fold_backwards_while<Exit, GoOn>(
    str: Str,
    initial_state: GoOn,
    reduce: Fn<Char·state<Char, GoOn>, Exit_or_go_on<Exit, GoOn>>,
) -> Exit_or_go_on<Exit, GoOn> {
    Exit_or_go_on::from_control_flow(std::iter::Iterator::try_fold(
        &mut std::iter::Iterator::rev(str.chars()),
        initial_state,
        |so_far, char| {
            Exit_or_go_on::into_control_flow(reduce(Char·state {
                state: so_far,
                char,
            }))
        },
    ))
}

/// safe if the referenced segment of memory is known to never be accessed again
unsafe fn copy_ref_to_owned<A>(reference: &mut A) -> A {
    unsafe { std::ptr::NonNull::read(std::ptr::NonNull::from_ref(reference)) }
}

fn span_raw_end_index(vacant_span: SpanRaw) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::saturating_add(vacant_span.length, vacant_span.start)
}

pub fn slot_index<Origin>(slot: Slot<Origin>) -> Index·slot<u32, Slot<Origin>> {
    Index·slot {
        index: slot.index,
        slot: slot,
    }
}
/// use with caution and never expose
fn index_to_slot<Origin>(index: u32) -> Slot<Origin> {
    Slot {
        origin: std::marker::PhantomData::<Origin>,
        index: index,
    }
}

pub fn slot_to_span<Origin>(slot: Slot<Origin>) -> Span<Origin> {
    Span {
        start: slot,
        length: std::num::NonZeroU32::MIN,
    }
}

pub fn span_start<Origin>(span: Span<Origin>) -> After·start<Opt<Span<Origin>>, Slot<Origin>> {
    After·start {
        after: match std::num::NonZeroU32::new(non_zero_u32_predecessor(span.length)) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(after_length) => Opt::Present(Span {
                start: index_to_slot::<Origin>(span.start.index + 1),
                length: after_length,
            }),
        },
        start: span.start,
    }
}
pub fn span_end<Origin>(span: Span<Origin>) -> Before·end<Opt<Span<Origin>>, Slot<Origin>> {
    Before·end {
        end: index_to_slot::<Origin>(span.end_index().get()),
        before: match std::num::NonZeroU32::new(non_zero_u32_predecessor(span.length)) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(before_length) => Opt::Present(Span {
                start: index_to_slot::<Origin>(span.start.index - 1),
                length: before_length,
            }),
        },
    }
}
pub fn opt_span_length<Origin>(
    opt_span: Opt<Span<Origin>>,
) -> Length·span<u32, Opt<Span<Origin>>> {
    Length·span {
        length: opt_span.as_ref().length(),
        span: opt_span,
    }
}
pub fn opt_span_take_start<Origin>(
    opt_span: Opt<Span<Origin>>,
    length_to_take: u32,
) -> After·start<Opt<Span<Origin>>, Opt<Span<Origin>>> {
    match std::num::NonZeroU32::new(length_to_take) {
        std::option::Option::None => After·start {
            start: Opt::Absent(Blank {}),
            after: opt_span,
        },
        std::option::Option::Some(positive_length_to_take) => match opt_span {
            Opt::Absent(Blank {}) => After·start {
                start: Opt::Absent(Blank {}),
                after: Opt::Absent(Blank {}),
            },
            Opt::Present(span) => {
                let After·start {
                    start: start_filled,
                    after,
                } = span_take_start_filled(span, positive_length_to_take);
                After·start {
                    start: Opt::Present(start_filled),
                    after: after,
                }
            }
        },
    }
}
pub fn span_take_start_filled<Origin>(
    span: Span<Origin>,
    length_to_take: std::num::NonZeroU32,
) -> After·start<Opt<Span<Origin>>, Span<Origin>> {
    After·start {
        after: match std::num::NonZeroU32::new(u32::saturating_sub(
            span.length.get(),
            length_to_take.get(),
        )) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(after_length) => Opt::Present(Span {
                start: index_to_slot(span.start.index + length_to_take.get()),
                length: after_length,
            }),
        },
        start: Span {
            start: span.start,
            length: length_to_take,
        },
    }
}
pub fn opt_span_fold_from<Origin, State>(
    opt_span: Opt<Span<Origin>>,
    initial_state: State,
    reduce: Fn<Slot·state<Slot<Origin>, State>, State>,
) -> State {
    std::iter::Iterator::fold(
        &mut opt_span.as_ref().to_range_u32(),
        initial_state,
        |so_far, index| {
            reduce(Slot·state {
                state: so_far,
                slot: index_to_slot(index),
            })
        },
    )
}
pub fn span_fold_backwards_from<Origin, State>(
    opt_span: Opt<Span<Origin>>,
    initial_state: State,
    reduce: Fn<Slot·state<Slot<Origin>, State>, State>,
) -> State {
    std::iter::Iterator::fold(
        &mut std::iter::Iterator::rev(opt_span.as_ref().to_range_u32()),
        initial_state,
        |so_far, index| {
            reduce(Slot·state {
                state: so_far,
                slot: index_to_slot(index),
            })
        },
    )
}
pub fn span_fold_while_from<Origin, Exit, GoOn>(
    span: Span<Origin>,
    initial_state: GoOn,
    reduce: Fn<Slot·state<Slot<Origin>, GoOn>, Exit_or_go_on<Exit, GoOn>>,
) -> Exit_or_go_on<Exit·remaining<Exit, Opt<Span<Origin>>>, GoOn> {
    let state_after_fold =
        std::iter::Iterator::try_fold(&mut span.to_range_u32(), initial_state, |so_far, index| {
            Exit_or_go_on::into_control_flow(reduce(Slot·state {
                state: so_far,
                slot: index_to_slot(index),
            }))
            .map_break(|exit| (index, exit))
        });
    match state_after_fold {
        std::ops::ControlFlow::Continue(state) => Exit_or_go_on::Go_on(state),
        std::ops::ControlFlow::Break((exit_index, exit_state)) => {
            let folded_over_element_count =
                std::num::NonZeroU32::saturating_add(std::num::NonZeroU32::MIN, exit_index);
            let After·start {
                start: _,
                after: not_folded_over_opt_span,
            } = span_take_start_filled(span, folded_over_element_count);
            Exit_or_go_on::Exit(Exit·remaining {
                exit: exit_state,
                remaining: not_folded_over_opt_span,
            })
        }
    }
}
pub fn span_connect_slot<Origin>(
    span: Span<Origin>,
    slot_to_add: Slot<Origin>,
) -> Apart·connected<Opt<Slot<Origin>>, Span<Origin>> {
    if span.end_index().get() + 1 == slot_to_add.index {
        Apart·connected {
            connected: Span {
                start: span.start,
                length: span.length.saturating_add(1),
            },
            apart: Opt::Absent(Blank {}),
        }
    } else if slot_to_add.index + 1 == span.start.index {
        Apart·connected {
            connected: Span {
                start: slot_to_add,
                length: span.length.saturating_add(1),
            },
            apart: Opt::Absent(Blank {}),
        }
    } else {
        Apart·connected {
            connected: span,
            apart: Opt::Present(slot_to_add),
        }
    }
}
pub fn span_connect<Origin>(
    span: Span<Origin>,
    span_to_add: Span<Origin>,
) -> Apart·connected<Opt<Span<Origin>>, Span<Origin>> {
    if span.end_index().get() + 1 == span_to_add.start.index {
        Apart·connected {
            connected: Span {
                start: span.start,
                length: span.length.saturating_add(span_to_add.length.get()),
            },
            apart: Opt::Absent(Blank {}),
        }
    } else if span_to_add.end_index().get() + 1 == span.start.index {
        Apart·connected {
            connected: Span {
                start: span_to_add.start,
                length: span.length.saturating_add(span_to_add.length.get()),
            },
            apart: Opt::Absent(Blank {}),
        }
    } else {
        Apart·connected {
            connected: span,
            apart: Opt::Present(span_to_add),
        }
    }
}
pub fn arena_span_to_opt<Backing>(span_build: Span_build<Backing>) -> Opt_span_build<Backing> {
    Opt_span_build {
        backing: span_build.backing,
        start: span_build.start,
    }
}

pub fn vec_to_arena<Origin, Element>(vec: Vec<Origin, Element>) -> Arena<Origin, Element> {
    Arena {
        origin: vec.origin,
        elements: vec.elements,
    }
}
pub fn arena_to_vec<Origin, Element>(arena: Arena<Origin, Element>) -> Vec<Origin, Element> {
    Vec {
        origin: arena.origin,
        elements: arena.elements,
        vacant: std::vec::Vec::new(),
    }
}

pub fn arena_empty<LocalOrigin, Element>(
    origin: Origin<LocalOrigin>,
) -> Arena<LocalOrigin, Element> {
    Arena {
        origin: origin,
        elements: std::vec::Vec::new(),
    }
}
pub fn arena_pre_allocate_at_least<Origin, Element>(
    Arena·length {
        arena: mut arena,
        length: min_pre_allocated_length,
    }: Arena·length<Arena<Origin, Element>, u32>,
) -> Arena<Origin, Element> {
    arena.pre_allocate_at_least(min_pre_allocated_length);
    Arena {
        origin: arena.origin,
        elements: arena.elements,
    }
}
pub fn arena_element<Origin, Element>(
    Arena·slot {
        mut arena,
        mut slot,
    }: Arena·slot<Arena<Origin, Element>, Slot<Origin>>,
) -> Arena·element<Arena<Origin, Element>, Element> {
    // the element in the slot will never be accessed again, since only one slot exists for it
    let element = unsafe { copy_ref_to_owned(arena.element_mut(&mut slot)) };
    Arena·element {
        arena: arena,
        element: element,
    }
}
pub fn arena_add<Origin, Element>(
    Arena·new {
        mut arena,
        new: new_element,
    }: Arena·new<Arena<Origin, Element>, Element>,
) -> Arena·slot<Arena<Origin, Element>, Slot<Origin>> {
    let slot = arena.add(new_element);
    Arena·slot {
        arena: arena,
        slot: slot,
    }
}
// TODO only provide this for span build
pub fn arena_add_vec_span<Origin, ToShrinkOrigin, Element>(
    mut arena: Arena<Origin, Element>,
    mut to_shrink: Vec<ToShrinkOrigin, Element>,
    shrink_span: Span<ToShrinkOrigin>,
) -> Grown·shrunk·span<Arena<Origin, Element>, Vec<ToShrinkOrigin, Element>, Span<Origin>> {
    let gro_span = arena.add_vec_span(&mut to_shrink, shrink_span);
    Grown·shrunk·span {
        grown: arena,
        shrunk: to_shrink,
        span: gro_span,
    }
}
// TODO only provide this for span build
pub fn arena_add_arena_span<Origin, ToShrinkOrigin, Element>(
    mut arena: Arena<Origin, Element>,
    mut to_shrink: Arena<ToShrinkOrigin, Element>,
    shrink_span: Span<ToShrinkOrigin>,
) -> Grown·shrunk·span<Arena<Origin, Element>, Arena<ToShrinkOrigin, Element>, Span<Origin>> {
    let shrink_length = shrink_span.length;
    let grow_span =
        arena.add_iterator_filled(to_shrink.span_iterator_mut(shrink_span), shrink_length);
    Grown·shrunk·span {
        grown: arena,
        shrunk: to_shrink,
        span: grow_span,
    }
}
// TODO only provide this for span build
pub fn arena_add_str<Origin>(
    mut arena: Arena<Origin, Char>,
    str: Str,
) -> Arena·span<Arena<Origin, Char>, Opt<Span<Origin>>> {
    let previous_length = arena.elements.len() as u32;
    std::iter::Extend::extend(&mut arena.elements, str.chars());
    Arena·span {
        span: match std::num::NonZeroU32::new(arena.elements.len() as u32 - previous_length) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(grow_length) => Opt::Present(Span {
                start: index_to_slot(previous_length),
                length: grow_length,
            }),
        },
        arena: arena,
    }
}
pub fn arena_replace<Origin, Element>(
    Arena·new·slot {
        mut arena,
        mut slot,
        new: new_element,
    }: Arena·new·slot<Arena<Origin, Element>, Element, Slot<Origin>>,
) -> Arena·old·slot<Arena<Origin, Element>, Element, Slot<Origin>> {
    let old_element = std::mem::replace(arena.element_mut(&mut slot), new_element);
    Arena·old·slot {
        arena: arena,
        old: old_element,
        slot: slot,
    }
}
/// Often used for copying (dup) a value out or altering the element
pub fn arena_update<Origin, Element, In, Out>(
    mut arena: Arena<Origin, Element>,
    mut slot: Slot<Origin>,
    in_: In,
    element_update: Fn<Element·in<Element, In>, Element·out<Element, Out>>,
) -> Arena·out·slot<Arena<Origin, Element>, Out, Slot<Origin>> {
    let out = arena.update(&mut slot, in_, element_update);
    Arena·out·slot {
        arena: arena,
        slot: slot,
        out,
    }
}
pub fn arena_opt_span_reverse<Origin, Element>(
    Arena·span {
        mut arena,
        span: mut opt_span,
    }: Arena·span<Arena<Origin, Element>, Opt<Span<Origin>>>,
) -> Arena·span<Arena<Origin, Element>, Opt<Span<Origin>>> {
    arena.opt_span_slice_mut(&mut opt_span).reverse();
    Arena·span {
        arena,
        span: opt_span,
    }
}
pub fn arena_span_reverse<Origin, Element>(
    Arena·span {
        mut arena,
        mut span,
    }: Arena·span<Arena<Origin, Element>, Span<Origin>>,
) -> Arena·span<Arena<Origin, Element>, Span<Origin>> {
    arena.span_slice_mut(&mut span).reverse();
    Arena·span { arena, span }
}
pub fn arena_span_empty<Origin, Element>(
    arena: Arena<Origin, Element>,
) -> Opt_span_build<Arena<Origin, Element>> {
    Opt_span_build {
        start: arena.elements.len() as u32,
        backing: arena,
    }
}
pub fn arena_opt_span_add<Origin, Element>(
    Build·new {
        build: mut span_build,
        new: new_element,
    }: Build·new<Opt_span_build<Arena<Origin, Element>>, Element>,
) -> Span_build<Arena<Origin, Element>> {
    span_build.backing.elements.push(new_element);
    Span_build {
        backing: span_build.backing,
        start: span_build.start,
    }
}
pub fn arena_span_add<Origin, Element>(
    Build·new {
        build: mut span_build,
        new: new_element,
    }: Build·new<Span_build<Arena<Origin, Element>>, Element>,
) -> Span_build<Arena<Origin, Element>> {
    span_build.backing.elements.push(new_element);
    span_build
}
pub fn arena_opt_span_add_str<Origin>(
    Build·new {
        build: mut span_build,
        new: new_str,
    }: Build·new<Opt_span_build<Arena<Origin, Char>>, Str>,
) -> Opt_span_build<Arena<Origin, Char>> {
    std::iter::Extend::extend(&mut span_build.backing.elements, new_str.chars());
    Opt_span_build {
        backing: span_build.backing,
        start: span_build.start,
    }
}
pub fn arena_span_add_str<Origin>(
    Build·new {
        build: mut span_build,
        new: new_str,
    }: Build·new<Opt_span_build<Arena<Origin, Char>>, Str>,
) -> Span_build<Arena<Origin, Char>> {
    std::iter::Extend::extend(&mut span_build.backing.elements, new_str.chars());
    Span_build {
        backing: span_build.backing,
        start: span_build.start,
    }
}
pub fn arena_opt_span_build<Origin, Element>(
    span_build: Opt_span_build<Arena<Origin, Element>>,
) -> Arena·span<Arena<Origin, Element>, Opt<Span<Origin>>> {
    Arena·span {
        span: match std::num::NonZeroU32::new(
            span_build.backing.elements.len() as u32 - span_build.start,
        ) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(span_length) => Opt::Present(Span {
                start: index_to_slot::<Origin>(span_build.start),
                length: span_length,
            }),
        },
        arena: span_build.backing,
    }
}
pub fn arena_span_build<Origin, Element>(
    span_build: Span_build<Arena<Origin, Element>>,
) -> Arena·span<Arena<Origin, Element>, Span<Origin>> {
    Arena·span {
        span: Span {
            start: index_to_slot::<Origin>(span_build.start),
            // Span_build is only returned from functions adding >= 1 element
            // and span_build.start is `.elements.len() as u32` from before adding
            length: unsafe {
                std::num::NonZeroU32::new_unchecked(
                    span_build.backing.elements.len() as u32 - span_build.start,
                )
            },
        },
        arena: span_build.backing,
    }
}

pub fn vec_empty<LocalOrigin, Element>(origin: Origin<LocalOrigin>) -> Vec<LocalOrigin, Element> {
    Vec {
        origin: origin,
        elements: std::vec::Vec::new(),
        vacant: std::vec::Vec::new(),
    }
}
pub fn vec_pre_allocate_at_least<Origin, Element>(
    Length·vec {
        vec: mut vec,
        length: min_pre_allocated_length,
    }: Length·vec<u32, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.pre_allocate_at_least(min_pre_allocated_length);
    Vec {
        origin: vec.origin,
        elements: vec.elements,
        vacant: vec.vacant,
    }
}
pub fn vec_element<Origin, Element>(
    Slot·vec { mut vec, mut slot }: Slot·vec<Slot<Origin>, Vec<Origin, Element>>,
) -> Element·vec<Element, Vec<Origin, Element>> {
    // vacated opt_span elements are never accessed, not even while vacating them
    let element = unsafe { copy_ref_to_owned(vec.element_mut(&mut slot)) };
    vec.span_vacate(slot_to_span(slot));
    Element·vec {
        element: element,
        vec: vec,
    }
}
pub fn vec_opt_span_fold<Origin, Element, State>(
    vec: Vec<Origin, Element>,
    span: Opt<Span<Origin>>,
    state: State,
    reduce: Fn<Element·in<Element, State>, State>,
) -> Vec<Origin, Element> {
    match span {
        Opt::Absent(Blank {}) => vec,
        Opt::Present(shrink_span) => vec_span_fold(vec, shrink_span, state, reduce),
    }
}
pub fn vec_span_fold<Origin, Element, State>(
    mut vec: Vec<Origin, Element>,
    shrink_span: Span<Origin>,
    state: State,
    reduce: Fn<Element·in<Element, State>, State>,
) -> Vec<Origin, Element> {
    vec.vacate_and_consume_span_iterator(shrink_span, |mut elements| {
        std::iter::Iterator::fold(&mut elements, state, |so_far, element| {
            reduce(Element·in {
                element: element,
                in_: so_far,
            })
        })
    });
    vec
}
// If you want to optimize an optimal initial populate round,
// use an `arena`, then convert it with `arena_to_vec`.
// Use `vec_add_ignoring_vacated` only if you e.g.
// - want to temporarily write something to the end and then vacate it
// - you are sure there are is no vacated space
// - a bunch of followup vec_add operations are likely to fill any existing vacated space anyway
//
// Note that keeping the amount of vacated space as small as possible helps insertions like vec_add be fast
// and reduces the used memory. Avoiding checking for existing vacated space might even unnecessarily
// re-allocate the whole underlying array if you're unlucky
pub fn vec_add_ignoring_vacated<Origin, Element>(
    New·vec {
        mut vec,
        new: new_element,
    }: New·vec<Element, Vec<Origin, Element>>,
) -> Slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add_ignoring_vacated(new_element);
    Slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add<Origin, Element>(
    New·vec {
        mut vec,
        new: new_element,
    }: New·vec<Element, Vec<Origin, Element>>,
) -> Slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add(new_element);
    Slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add_arena_span<Origin, ShrinkOrigin, Element>(
    mut grow: Vec<Origin, Element>,
    mut shrink: Arena<ShrinkOrigin, Element>,
    shrink_span: Span<ShrinkOrigin>,
) -> Grown·shrunk·span<Vec<Origin, Element>, Arena<ShrinkOrigin, Element>, Span<Origin>> {
    let grow_span = grow.add_arena_span(&mut shrink, shrink_span);
    Grown·shrunk·span {
        grown: grow,
        shrunk: shrink,
        span: grow_span,
    }
}
pub fn vec_add_vec_span<Origin, ShrinkOrigin, Element>(
    mut grow: Vec<Origin, Element>,
    mut shrink: Vec<ShrinkOrigin, Element>,
    shrink_span: Span<ShrinkOrigin>,
) -> Grown·shrunk·span<Vec<Origin, Element>, Vec<ShrinkOrigin, Element>, Span<Origin>> {
    let grow_span = grow.add_vec_span(&mut shrink, shrink_span);
    Grown·shrunk·span {
        grown: grow,
        shrunk: shrink,
        span: grow_span,
    }
}
pub fn vec_add_str<Origin>(
    New·vec {
        mut vec,
        new: new_str,
    }: New·vec<Str, Vec<Origin, Char>>,
) -> Span·vec<Opt<Span<Origin>>, Vec<Origin, Char>> {
    let grow_span = vec.add_str(new_str);
    Span·vec {
        vec: vec,
        span: grow_span,
    }
}
pub fn vec_replace<Origin, Element>(
    New·slot·vec {
        mut vec,
        mut slot,
        new: new_element,
    }: New·slot·vec<Element, Slot<Origin>, Vec<Origin, Element>>,
) -> Old·slot·vec<Element, Slot<Origin>, Vec<Origin, Element>> {
    let old_element = std::mem::replace(vec.element_mut(&mut slot), new_element);
    Old·slot·vec {
        vec: vec,
        old: old_element,
        slot: slot,
    }
}
/// Often used for copying (dup) a value out or altering the element
pub fn vec_update<Origin, Element, In, Out>(
    mut vec: Vec<Origin, Element>,
    slot: Slot<Origin>,
    in_: In,
    element_update: Fn<Element·in<Element, In>, Element·out<Element, Out>>,
) -> Out·slot·vec<Out, Slot<Origin>, Vec<Origin, Element>> {
    // this should just be an in-place edit at one index. rust does not yet have a primitive for this
    let index = slot.index as usize;
    let last_index = vec.elements.len() - 1;
    let element = vec.elements.swap_remove(index);
    let element_updated = element_update(Element·in {
        element: element,
        in_: in_,
    });
    vec.elements.push(element_updated.element);
    vec.elements.swap(index, last_index);
    Out·slot·vec {
        vec: vec,
        slot: slot,
        out: element_updated.out,
    }
}
pub fn vec_opt_span_reverse<Origin, Element>(
    Span·vec { mut vec, mut span }: Span·vec<Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    vec.opt_span_slice_mut(&mut span).reverse();
    Span·vec { vec: vec, span }
}
pub fn vec_span_reverse<Origin, Element>(
    Span·vec { mut vec, mut span }: Span·vec<Span<Origin>, Vec<Origin, Element>>,
) -> Span·vec<Span<Origin>, Vec<Origin, Element>> {
    vec.span_slice_mut(&mut span).reverse();
    Span·vec { vec: vec, span }
}

pub fn vec_span_empty<Origin, Element>(
    vec: Vec<Origin, Element>,
) -> Opt_span_build<Vec<Origin, Element>> {
    Opt_span_build {
        start: vec.elements.len() as u32,
        backing: vec,
    }
}
pub fn vec_opt_span_add<Origin, Element>(
    Build·new {
        build: mut span_build,
        new: new_element,
    }: Build·new<Opt_span_build<Vec<Origin, Element>>, Element>,
) -> Span_build<Vec<Origin, Element>> {
    span_build.backing.elements.push(new_element);
    Span_build {
        backing: span_build.backing,
        start: span_build.start,
    }
}
pub fn vec_span_add<Origin, Element>(
    Build·new {
        build: mut span_build,
        new: new_element,
    }: Build·new<Span_build<Vec<Origin, Element>>, Element>,
) -> Span_build<Vec<Origin, Element>> {
    span_build.backing.elements.push(new_element);
    span_build
}
pub fn vec_opt_span_add_str<Origin>(
    Build·new {
        build: mut span_build,
        new: new_str,
    }: Build·new<Opt_span_build<Vec<Origin, Char>>, Str>,
) -> Opt_span_build<Vec<Origin, Char>> {
    std::iter::Extend::extend(&mut span_build.backing.elements, new_str.chars());
    Opt_span_build {
        backing: span_build.backing,
        start: span_build.start,
    }
}
pub fn vec_span_add_str<Origin>(
    Build·new {
        build: mut span_build,
        new: new_str,
    }: Build·new<Span_build<Vec<Origin, Char>>, Str>,
) -> Span_build<Vec<Origin, Char>> {
    std::iter::Extend::extend(&mut span_build.backing.elements, new_str.chars());
    Span_build {
        backing: span_build.backing,
        start: span_build.start,
    }
}
pub fn vec_opt_span_ignoring_vacant<Origin, Element>(
    span_build: Opt_span_build<Vec<Origin, Element>>,
) -> Span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    Span·vec {
        span: match std::num::NonZeroU32::new(
            span_build.backing.elements.len() as u32 - span_build.start,
        ) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(span_length) => Opt::Present(Span {
                start: index_to_slot::<Origin>(span_build.start),
                length: span_length,
            }),
        },
        vec: span_build.backing,
    }
}
pub fn vec_span_ignoring_vacant<Origin, Element>(
    span_build: Span_build<Vec<Origin, Element>>,
) -> Span·vec<Span<Origin>, Vec<Origin, Element>> {
    Span·vec {
        span: Span {
            start: index_to_slot::<Origin>(span_build.start),
            // Span_build is only returned from functions adding >= 1 element
            // and span_build.start is `.elements.len() as u32` from before adding
            length: unsafe {
                std::num::NonZeroU32::new_unchecked(
                    span_build.backing.elements.len() as u32 - span_build.start,
                )
            },
        },
        vec: span_build.backing,
    }
}
fn vec_span_with_length<Origin, Element>(
    span_build: &mut Opt_span_build<Vec<Origin, Element>>,
    span_length: std::num::NonZeroU32,
) -> Span<Origin> {
    let earlier_start_to_occupy_from = span_build
        .backing
        .mark_length_filled_as_occupied(span_length);
    if let std::option::Option::Some(earlier_start_to_occupy_from) = earlier_start_to_occupy_from {
        let (before_opt_span_slice, opt_span_slice) = span_build
            .backing
            .elements
            .split_at_mut(span_build.start as usize);
        // vec_mark_length_filled_as_occupied_mut found an existing (vacated) opt_span with length >= span_length
        unsafe {
            before_opt_span_slice.get_unchecked_mut(
                (earlier_start_to_occupy_from as usize)
                    ..(earlier_start_to_occupy_from as usize + span_length.get() as usize),
            )
        }
        .swap_with_slice(opt_span_slice);
        span_build
            .backing
            .elements
            .truncate(span_build.backing.elements.len() - span_length.get() as usize);
    }
    Span {
        start: index_to_slot::<Origin>(span_build.start),
        length: span_length,
    }
}
pub fn vec_opt_span_build<Origin, Element>(
    mut span_build: Opt_span_build<Vec<Origin, Element>>,
) -> Span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    Span·vec {
        span: match std::num::NonZeroU32::new(
            span_build.backing.elements.len() as u32 - span_build.start,
        ) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(span_length) => {
                let potentially_moved_opt_span = vec_span_with_length(&mut span_build, span_length);
                Opt::Present(potentially_moved_opt_span)
            }
        },
        vec: span_build.backing,
    }
}
pub fn vec_span_build<Origin, Element>(
    span_build: Span_build<Vec<Origin, Element>>,
) -> Span·vec<Span<Origin>, Vec<Origin, Element>> {
    Span·vec {
        span: Span {
            start: index_to_slot::<Origin>(span_build.start),
            length: span_build.length(),
        },
        vec: span_build.backing,
    }
}
