#![no_implicit_prelude]
#![allow(
    dead_code,
    unused_imports,
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

// Most module members are directly usable by sloe code to avoid name clashes with generated functions and types.
// The remaining few member names must be explicitly added to `sloe::name_to_uppercase_rust` and `name_to_lowercase_rust`

#[derive(Clone, Copy, Debug)]
pub struct A·b<A, B> {
    pub a: A,
    pub b: B,
}
#[derive(Clone, Copy, Debug)]
pub struct A·b·carry<A, B, Carry> {
    pub a: A,
    pub b: B,
    pub carry: Carry,
}
#[derive(Clone, Copy, Debug)]
pub struct P·u<P, U> {
    pub p: P,
    pub u: U,
}
#[derive(Clone, Copy, Debug)]
pub struct By·n<By, N> {
    pub by: By,
    pub n: N,
}
#[derive(Clone, Copy, Debug)]
pub struct Mode·n<Mode, N> {
    pub mode: Mode,
    pub n: N,
}
#[derive(Clone, Copy, Debug)]
pub struct Max·min<Max, Min> {
    pub max: Max,
    pub min: Min,
}
#[derive(Clone, Copy, Debug)]
pub struct Element·in<Element, In> {
    pub element: Element,
    pub in_: In,
}
#[derive(Clone, Copy, Debug)]
pub struct Element·state<Element, State> {
    pub element: Element,
    pub state: State,
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
pub struct In_·slot·update·vec<In, Slot, Update, Vec> {
    pub in_: In,
    pub slot: Slot,
    pub update: Update,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Span·vec<Span, Vec> {
    pub span: Span,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct New·span·vec<New, Span, Vec> {
    pub new: New,
    pub span: Span,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct New·shrink·span·vec<New, Shrink, Span, Vec> {
    pub new: New,
    pub shrink: Shrink,
    pub span: Span,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Out·slot·vec<Out, Slot, Vec> {
    pub out: Out,
    pub slot: Slot,
    pub vec: Vec,
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
pub struct Old·slot·vec<Old_element, Slot, Vec> {
    pub old: Old_element,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Direction·state·step·str<Direction, State, Step, Str> {
    pub direction: Direction,
    pub state: State,
    pub step: Step,
    pub str: Str,
}
#[derive(Clone, Copy, Debug)]
pub struct Direction·state·step·vec<Direction, State, Step, Vec> {
    pub direction: Direction,
    pub state: State,
    pub step: Step,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Direction·span·state·step<Direction, Span, State, Step> {
    pub direction: Direction,
    pub span: Span,
    pub state: State,
    pub step: Step,
}
#[derive(Clone, Copy, Debug)]
pub struct Element·origin_rid·state<Element, Origin_rid, State> {
    pub element: Element,
    pub origin_rid: Origin_rid,
    pub state: State,
}
#[derive(Clone, Copy, Debug)]
pub struct Origin_rid·state<Origin_rid, State> {
    pub origin_rid: Origin_rid,
    pub state: State,
}
#[derive(Clone, Copy, Debug)]
pub struct Origin_rid·slot<Origin_rid, Slot> {
    pub origin_rid: Origin_rid,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Origin_rid·span<Origin_rid, Span> {
    pub origin_rid: Origin_rid,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Occupied_count·vec<Occupied_count, Vec> {
    pub occupied_count: Occupied_count,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Carry·wrapped<Carry, Wrapped> {
    pub carry: Carry,
    pub wrapped: Wrapped,
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
pub enum Down·Up<Down, Up> {
    Down(Down),
    Up(Up),
}
#[derive(Clone, Copy, Debug)]
pub enum Contained·Overflowed<Contained, Overflowed> {
    Contained(Contained),
    Overflowed(Overflowed),
}
#[derive(Clone, Copy, Debug)]
pub enum Away_from_0·Down·Nearest_else_away_from_0·Nearest_else_even·Toward_0·Up<
    Away_from_0,
    Down,
    Nearest_else_away_from_0,
    Nearest_else_even,
    Toward_0,
    Up,
> {
    Away_from_0(Away_from_0),
    Down(Down),
    Nearest_else_away_from_0(Nearest_else_away_from_0),
    Nearest_else_even(Nearest_else_even),
    Toward_0(Toward_0),
    Up(Up),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type Round_mode = Away_from_0·Down·Nearest_else_away_from_0·Nearest_else_even·Toward_0·Up<
    Blank,
    Blank,
    Blank,
    Blank,
    Blank,
    Blank,
>;

#[derive(Debug)]
pub struct Origin<LocalOrigin>(LocalOrigin);
#[derive(Debug)]
pub struct Origin_rid<Origin>(std::marker::PhantomData<Origin>);
#[derive(Debug)]
pub struct Vec<LocalOrigin, Element> {
    // invariants:
    // - no SpanRaws in vacant are connected
    //   (and thus could be combined into one larger consecutive SpanRaw)
    // - any index contained in any vacant SpanRaw is less than elements.len()
    //   (and therefore no index within a vacant SpanRaw indexes uninitialized memory)
    // - any index contained in any vacant SpanRaw is an index in elements that should
    //   not be accessed again
    pub origin: Origin<LocalOrigin>,
    elements: std::vec::Vec<Element>,
    // Performance assumption:
    // Neighboring elements are way more likely to be vacated together.
    // Think e.g. vec_span_add_vec_span but also
    // regular chunks of nested individual slots which were likely allocated close to their neighbors.
    //
    // It is also assumed that there won't be a large amount of these vacant spans
    // so e.g. HashSet loses despite having a faster "find out if this index is vacant".
    // If usage ends up suggesting otherwise, we should change accordingly
    pub vacant: std::vec::Vec<SpanRaw>,
}
#[derive(Debug, Clone, Copy)]
pub struct SpanRaw {
    pub start: u32,
    pub length: std::num::NonZeroU32,
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
    // This makes combining 2 opt_spans and converting to ops::Range a bit faster,
    // at the cost of other operations like checking a vec's occupied count
    pub length: std::num::NonZeroU32,
}

impl<Origin> std::marker::Copy for Origin_rid<Origin> {}
impl<Origin> std::clone::Clone for Origin_rid<Origin> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
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
    pub fn as_mut(&mut self) -> Opt<&mut A> {
        match self {
            Opt::Absent(Blank {}) => Opt::Absent(Blank {}),
            Opt::Present(present) => Opt::Present(present),
        }
    }
}

impl<Exit, GoOn> Exit·Go_on<Exit, GoOn> {
    pub fn from_control_flow(control_flow: std::ops::ControlFlow<Exit, GoOn>) -> Self {
        match control_flow {
            std::ops::ControlFlow::Break(exit) => Exit·Go_on::Exit(exit),
            std::ops::ControlFlow::Continue(go_on) => Exit·Go_on::Go_on(go_on),
        }
    }
    pub fn into_control_flow(self) -> std::ops::ControlFlow<Exit, GoOn> {
        match self {
            Exit·Go_on::Exit(exit) => std::ops::ControlFlow::Break(exit),
            Exit·Go_on::Go_on(go_on) => std::ops::ControlFlow::Continue(go_on),
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
        self.ref_mut_iterator.next().map(|element_ref| unsafe {
            std::ptr::NonNull::read(std::ptr::NonNull::from_ref(element_ref))
        })
    }
}

/// This constructor is exposed because sadly macros (namely origin_new!) require it.
/// It's _very strongly_ recommended to instead only construct new origins with `origin_new!`.
/// Misusing this constructor can lead to UB like unchecked out of bounds access.
impl<LocalOrigin> Origin<LocalOrigin> {
    pub unsafe fn new_use_macro_instead(local_type_instance: LocalOrigin) -> Origin<LocalOrigin> {
        Origin(local_type_instance)
    }
}
#[macro_export]
macro_rules! origin_new {
    ($variable_name:ident, $type_name:ident) => {
        struct $type_name();
        let $variable_name = unsafe { $crate::core::Origin::new_use_macro_instead($type_name()) };
    };
}
pub use origin_new;

impl SpanRaw {
    fn end_index(self) -> u32 {
        self.start + p32_predecessor(self.length)
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
        self.vacate_span(shrink_span);
        munched
    }
    pub fn take(&mut self, mut slot: Slot<Origin>) -> Element {
        // vacated opt_span elements are never accessed, not even while vacating them
        let element = unsafe {
            std::ptr::NonNull::read(std::ptr::NonNull::from_ref(self.element_mut(&mut slot)))
        };
        // can maybe be optimized
        self.vacate_span(slot_to_span(slot));
        element
    }
    /// only use when the element values are safe to not handle or are handled unsafely immediately after
    fn vacate_span(&mut self, span_to_vacate: Span<Origin>) {
        let maybe_vacant_span_index_connecting_earlier: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_span| {
                std::cmp::PartialEq::<u32>::eq(
                    &(vacant_span.end_index() + 1),
                    &span_to_vacate.start.index,
                )
            });
        let maybe_vacant_span_inde_connecting_later: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_span| {
                std::cmp::PartialEq::<u32>::eq(
                    &(span_to_vacate.end_index() + 1),
                    &vacant_span.start,
                )
            });
        match (
            maybe_vacant_span_index_connecting_earlier,
            maybe_vacant_span_inde_connecting_later,
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
                // if both spans start connecting now, combine them
                let earlier_span = self.vacant[index_connecting_earlier];
                let later_span_to_extend = &mut self.vacant[index_connecting_later];
                *later_span_to_extend = SpanRaw {
                    start: earlier_span.start,
                    length: std::num::NonZeroU32::saturating_add(
                        std::num::NonZeroU32::saturating_add(
                            earlier_span.length,
                            later_span_to_extend.length.get(),
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
    pub fn add_ignoring_vacant(&mut self, new_element: Element) -> Slot<Origin> {
        let added_index = self.elements.len();
        self.elements.push(new_element);
        Slot::from_index(added_index as u32)
    }
    pub fn add(&mut self, new_element: Element) -> Slot<Origin> {
        match self.vacant.pop() {
            std::option::Option::None => self.add_ignoring_vacant(new_element),
            std::option::Option::Some(vacant_opt_span_to_occupy) => {
                // each vacant span only contains indexes present in .elements.
                // This is still true when the vec's capacity has been shrunk as vacant spans
                // for memory after the last element index are never created
                unsafe {
                    *self
                        .elements
                        .get_unchecked_mut(vacant_opt_span_to_occupy.start as usize) = new_element;
                }
                if let std::option::Option::Some(remaining_length) =
                    std::num::NonZeroU32::new(p32_predecessor(vacant_opt_span_to_occupy.length))
                {
                    self.vacant.push(SpanRaw {
                        start: vacant_opt_span_to_occupy.start + 1,
                        length: remaining_length,
                    });
                }
                Slot::from_index(vacant_opt_span_to_occupy.start)
            }
        }
    }
    fn mark_length_positive_as_occupied(
        &mut self,
        length_to_occupy: std::num::NonZeroU32,
    ) -> std::option::Option<u32> {
        let vacant_opt_span_to_reuse_index =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_span| {
                std::cmp::PartialOrd::ge(&vacant_span.length, &length_to_occupy)
            });
        match vacant_opt_span_to_reuse_index {
            std::option::Option::None => std::option::Option::None,
            std::option::Option::Some(vacant_opt_span_to_reuse_index) => {
                let vacant_opt_span_to_occupy = &mut self.vacant[vacant_opt_span_to_reuse_index];
                let start_to_occupy_from = vacant_opt_span_to_occupy.start;
                match std::num::NonZeroU32::new(
                    vacant_opt_span_to_occupy.length.get() - length_to_occupy.get(),
                ) {
                    std::option::Option::None => {
                        // vacant_opt_span_to_occupy.length == length_to_occupy
                        self.vacant.swap_remove(vacant_opt_span_to_reuse_index);
                    }
                    std::option::Option::Some(remaining_vacant_length) => {
                        vacant_opt_span_to_occupy.length = remaining_vacant_length;
                    }
                }
                std::option::Option::Some(start_to_occupy_from)
            }
        }
    }
    // invariant! new_element_count must equal new_elements.count()
    fn add_iterator_filled(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
        new_element_count: std::num::NonZeroU32,
    ) -> Span<Origin> {
        match self.mark_length_positive_as_occupied(new_element_count) {
            std::option::Option::None => {
                self.add_iterator_filled_ignoring_vacant(new_elements, new_element_count)
            }
            std::option::Option::Some(index_to_populate_from) => {
                let grow_span = Span {
                    start: Slot::from_index(index_to_populate_from),
                    length: new_element_count,
                };
                self.elements.splice(grow_span.to_range(), new_elements);
                grow_span
            }
        }
    }
    pub fn add_iterator_ignoring_vacant(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
    ) -> Opt<Span<Origin>> {
        let length_without_new_elements = self.elements.len();
        std::iter::Extend::extend(&mut self.elements, new_elements);
        match std::num::NonZeroU32::new((self.elements.len() - length_without_new_elements) as u32)
        {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(new_length) => Opt::Present(Span {
                start: Slot::from_index(length_without_new_elements as u32),
                length: new_length,
            }),
        }
    }
    // invariant! new_element_count must equal new_elements.count()
    fn add_iterator_filled_ignoring_vacant(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
        new_element_count: std::num::NonZeroU32,
    ) -> Span<Origin> {
        let length_without_new_elements = self.elements.len() as u32;
        std::iter::Extend::extend(&mut self.elements, new_elements);
        Span {
            start: Slot::from_index(length_without_new_elements),
            length: new_element_count,
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
    // This will clone the iterator. Prefer add_iterator whenever possible
    pub fn add_iterator_without_known_size(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element> + std::clone::Clone,
    ) -> Opt<Span<Origin>> {
        // can be optimized to only clone if there is actually existing vacant space to occupy.
        // Might make sense to also benchmark with simply writing to the end, then relocating
        let std::option::Option::Some(grow_length) =
            std::num::NonZeroU32::new(std::iter::Iterator::count(new_elements.clone()) as u32)
        else {
            return Opt::Absent(Blank {});
        };
        let grow_span = self.add_iterator_filled(new_elements, grow_length);
        Opt::Present(grow_span)
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
    pub fn snatch_vec_span_ignoring_vacant<ShrinkOrigin>(
        &mut self,
        shrink: &mut Vec<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        let shrink_span_length = shrink_span.length;
        let grow_span = shrink.vacate_and_consume_span_iterator(shrink_span, |shrink_elements| {
            self.add_iterator_filled_ignoring_vacant(shrink_elements, shrink_span_length)
        });
        grow_span
    }
    pub fn opt_span_snatch_vec_span<ShrinkOrigin>(
        &mut self,
        span: Opt<Span<Origin>>,
        shrink_vec: &mut Vec<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        match span {
            Opt::Absent(Blank {}) => self.snatch_vec_span_ignoring_vacant(shrink_vec, shrink_span),
            Opt::Present(span) => self.span_snatch_vec_span(span, shrink_vec, shrink_span),
        }
    }
    pub fn span_snatch_vec_span<ShrinkOrigin>(
        &mut self,
        span: Span<Origin>,
        shrink_vec: &mut Vec<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        shrink_vec.vacate_and_consume_span_iterator(shrink_span, |elements| {
            self.span_add_iterator(span, elements)
        })
    }
    pub fn span_add_iterator(
        &mut self,
        span: Span<Origin>,
        new_elements: impl std::iter::Iterator<Item = Element>,
    ) -> Span<Origin> {
        // does not check for vacant space after because that will be rare
        let moved_span = self.move_span_to_end(span);
        let length_before_extend = self.elements.len();
        std::iter::Extend::extend(&mut self.elements, new_elements);
        Span {
            start: moved_span.start,
            length: moved_span
                .length
                .saturating_add((self.elements.len() - length_before_extend) as u32),
        }
    }
    pub fn span_add(&mut self, span: Span<Origin>, new_element: Element) -> Span<Origin> {
        // does not check for vacant space after because that will be rare
        let moved_span = self.move_span_to_end(span);
        self.elements.push(new_element);
        Span {
            start: moved_span.start,
            length: moved_span.length.saturating_add(1),
        }
    }
    pub fn move_span_to_end(&mut self, mut span: Span<Origin>) -> Span<Origin> {
        if span.end_index_usize() + 1 == self.elements.len() {
            return span;
        }
        // span is not at the end already

        // elements in the span are moved and never accessed in the original slice after as they are vacated.
        let elements_to_move = unsafe {
            mut_slice_into_owned_iterator(
                // we give this &mut slice a new lifetime to allow extending
                // the original Vec at the same time.
                // This is okay because earlier elements in that slice
                // do not get mutated or removed during an extend.
                std::ptr::NonNull::from_mut(self.span_slice_mut(&mut span)).as_mut(),
            )
        };
        let moved_span = self.add_iterator_filled_ignoring_vacant(elements_to_move, span.length);
        self.vacate_span(span);
        moved_span
    }
    pub fn move_span_to_vacant(&mut self, mut span: Span<Origin>) -> Span<Origin> {
        if span.end_index() as usize + 1 < self.elements.len() {
            // moving this span would not reduce the amount of vacant space
            return span;
        }
        // span is at the end of elements

        let earlier_start_to_occupy_from = self.mark_length_positive_as_occupied(span.length);
        match earlier_start_to_occupy_from {
            std::option::Option::None => span,
            std::option::Option::Some(earlier_start_to_occupy_from) => {
                // the range of the span will be truncated next.
                // the &mut lifetime is ignored because the edited and read ranges do not overlap
                let elements_to_move = unsafe {
                    mut_slice_into_owned_iterator(
                        std::ptr::NonNull::from_mut(self.span_slice_mut(&mut span)).as_mut(),
                    )
                };
                self.elements.splice(
                    (earlier_start_to_occupy_from as usize)
                        ..(earlier_start_to_occupy_from as usize + span.length.get() as usize),
                    elements_to_move,
                );
                // we could alternatively have swapped the non-overlapping slices. Not sure what is faster
                self.elements
                    .truncate(self.elements.len() - span.length.get() as usize);
                Span {
                    start: Slot::<Origin>::from_index(earlier_start_to_occupy_from),
                    length: span.length,
                }
            }
        }
    }
    pub fn vacant_count_usize(&self) -> usize {
        std::iter::Iterator::sum(std::iter::Iterator::map(self.vacant.iter(), |r| {
            r.length.get() as usize
        }))
    }
    pub fn vacant_count_u32(&self) -> u32 {
        std::iter::Iterator::sum(std::iter::Iterator::map(self.vacant.iter(), |r| {
            r.length.get()
        }))
    }
    pub fn occupied_count_usize(&self) -> usize {
        usize::saturating_sub(self.elements.len(), self.vacant_count_usize())
    }
    pub fn occupied_count_u32(&self) -> u32 {
        u32::saturating_sub(self.elements.len() as u32, self.vacant_count_u32())
    }
    pub fn into_occupied_elements(mut self) -> VecIter<Element> {
        let maybe_occupied_elements =
            std::iter::Iterator::enumerate(std::iter::IntoIterator::into_iter(self.elements));
        self.vacant.sort_unstable_by_key(|span| span.start);
        let mut vacant_ascending = std::iter::IntoIterator::into_iter(self.vacant);
        let vacant_start = std::iter::Iterator::next(&mut vacant_ascending);
        VecIter {
            vacant_start: vacant_start,
            vacant_after_ascending: vacant_ascending,
            maybe_occupied_elements: maybe_occupied_elements,
        }
    }
    pub fn into_occupied_elements_rev(mut self) -> VecIterRev<Element> {
        let maybe_occupied_elements_rev = std::iter::Iterator::rev(std::iter::Iterator::enumerate(
            std::iter::IntoIterator::into_iter(self.elements),
        ));
        self.vacant.sort_unstable_by(|a_span, b_span| {
            <u32 as std::cmp::Ord>::cmp(&a_span.start, &b_span.start).reverse()
        });
        let mut vacant_descending = std::iter::IntoIterator::into_iter(self.vacant);
        let vacant_end = std::iter::Iterator::next(&mut vacant_descending);
        VecIterRev {
            vacant_end: vacant_end,
            vacant_before_descending: vacant_descending,
            maybe_occupied_elements_rev: maybe_occupied_elements_rev,
        }
    }
}
impl<Origin> Vec<Origin, Char> {
    pub fn add_str(&mut self, new_str: Str) -> Opt<Span<Origin>> {
        self.add_iterator_without_known_size(new_str.chars())
    }
    pub fn add_str_ignoring_vacant(&mut self, new_str: Str) -> Opt<Span<Origin>> {
        self.add_iterator_ignoring_vacant(new_str.chars())
    }
    pub fn opt_span_add_str(&mut self, span: Opt<Span<Origin>>, new_str: Str) -> Opt<Span<Origin>> {
        match span {
            Absent·Present::Absent(Blank {}) => self.add_str_ignoring_vacant(new_str),
            Absent·Present::Present(span) => Opt::Present(self.span_add_str(span, new_str)),
        }
    }
    pub fn span_add_str(&mut self, span: Span<Origin>, new_str: Str) -> Span<Origin> {
        self.span_add_iterator(span, new_str.chars())
    }
}
pub struct VecIter<Element> {
    pub vacant_start: std::option::Option<SpanRaw>,
    pub vacant_after_ascending: std::vec::IntoIter<SpanRaw>,
    maybe_occupied_elements: std::iter::Enumerate<std::vec::IntoIter<Element>>,
}
impl<Element> std::iter::Iterator for VecIter<Element> {
    type Item = Element;
    fn next(&mut self) -> std::option::Option<Element> {
        match std::iter::Iterator::next(&mut self.maybe_occupied_elements) {
            std::option::Option::None => std::option::Option::None,
            std::option::Option::Some((index, maybe_occupied_element)) => {
                match &self.vacant_start {
                    std::option::Option::None => std::option::Option::Some(maybe_occupied_element),
                    std::option::Option::Some(vacant_start) => {
                        if index as u32 == vacant_start.start {
                            let next_after_vacant = std::iter::Iterator::nth(
                                &mut self.maybe_occupied_elements,
                                p32_predecessor(vacant_start.length) as usize,
                            );
                            self.vacant_start =
                                std::iter::Iterator::next(&mut self.vacant_after_ascending);
                            // as vacant SpanRaws are always disconnected, this one is known to be occupied
                            next_after_vacant.map(|(_, element)| element)
                        } else {
                            std::option::Option::Some(maybe_occupied_element)
                        }
                    }
                }
            }
        }
    }
}
pub struct VecIterRev<Element> {
    pub vacant_end: std::option::Option<SpanRaw>,
    pub vacant_before_descending: std::vec::IntoIter<SpanRaw>,
    maybe_occupied_elements_rev: std::iter::Rev<std::iter::Enumerate<std::vec::IntoIter<Element>>>,
}
impl<Element> std::iter::Iterator for VecIterRev<Element> {
    type Item = Element;
    fn next(&mut self) -> std::option::Option<Element> {
        match std::iter::Iterator::next(&mut self.maybe_occupied_elements_rev) {
            std::option::Option::None => std::option::Option::None,
            std::option::Option::Some((index, maybe_occupied_element)) => {
                match &self.vacant_end {
                    std::option::Option::None => std::option::Option::Some(maybe_occupied_element),
                    std::option::Option::Some(vacant_end) => {
                        if index as u32 == vacant_end.end_index() {
                            let next_before_vacant = std::iter::Iterator::nth(
                                &mut self.maybe_occupied_elements_rev,
                                p32_predecessor(vacant_end.length) as usize,
                            );
                            self.vacant_end =
                                std::iter::Iterator::next(&mut self.vacant_before_descending);
                            // as vacant SpanRaws are always disconnected, this one is known to be occupied
                            next_before_vacant.map(|(_, element)| element)
                        } else {
                            std::option::Option::Some(maybe_occupied_element)
                        }
                    }
                }
            }
        }
    }
}

impl<Origin> Slot<Origin> {
    /// use with caution. duplicate use or out-of-bounds of the given index can lead to UB.
    /// consider making it unsafe and exposing it
    fn from_index(index: u32) -> Slot<Origin> {
        Slot {
            origin: std::marker::PhantomData::<Origin>,
            index: index,
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
    pub fn end_index(&self) -> u32 {
        self.start.index + p32_predecessor(self.length)
    }
    pub fn end_index_usize(&self) -> usize {
        self.start.index as usize + p32_predecessor(self.length) as usize
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

pub fn p32_dup(n: P32) -> A·b<P32, P32> {
    A·b { a: n, b: n }
}
pub fn p32_rid(_: P32) -> Blank {
    Blank {}
}
pub fn p32_predecessor(n: P32) -> U32 {
    n.get() - 1
}
pub fn p32_add_clamp(P·u { p, u }: P·u<P32, U32>) -> P32 {
    p.saturating_add(u)
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
pub fn u32_add_clamp(A·b { a, b }: A·b<U32, U32>) -> U32 {
    a.saturating_add(b)
}
pub fn u32_add_carry(
    A·b·carry { a, b, carry }: A·b·carry<U32, U32, Contained·Overflowed<Blank, Blank>>,
) -> Carry·wrapped<Contained·Overflowed<Blank, Blank>, U32> {
    let (sum, carry) = a.carrying_add(
        b,
        match carry {
            Contained·Overflowed::Overflowed(Blank {}) => true,
            Contained·Overflowed::Contained(Blank {}) => false,
        },
    );
    Carry·wrapped {
        carry: if carry {
            Contained·Overflowed::Overflowed(Blank {})
        } else {
            Contained·Overflowed::Contained(Blank {})
        },
        wrapped: sum,
    }
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
pub fn i32_add_clamp(A·b { a, b }: A·b<I32, I32>) -> I32 {
    a.saturating_add(b)
}
pub fn i32_add_carry(
    A·b { a, b }: A·b<I32, I32>,
) -> Carry·wrapped<Contained·Overflowed<Blank, Blank>, I32> {
    let (sum, carry) = a.overflowing_add(b);
    Carry·wrapped {
        carry: if carry {
            Contained·Overflowed::Overflowed(Blank {})
        } else {
            Contained·Overflowed::Contained(Blank {})
        },
        wrapped: sum,
    }
}
pub fn i32_mul_clamp(A·b { a, b }: A·b<I32, I32>) -> I32 {
    a.saturating_mul(b)
}
pub fn f32_dup(n: F32) -> A·b<F32, F32> {
    A·b { a: n, b: n }
}
pub fn f32_rid(_: F32) -> Blank {
    Blank {}
}
pub fn f32_add_clamp(A·b { a, b }: A·b<F32, F32>) -> F32 {
    (a + b).clamp(f32::MIN, f32::MAX)
}
pub fn f32_mul_clamp(A·b { a, b }: A·b<F32, F32>) -> F32 {
    (a * b).clamp(f32::MIN, f32::MAX)
}
pub fn f32_div_clamp(By·n { n, by }: By·n<F32, F32>) -> F32 {
    if by == 0_f32 {
        0_f32
    } else {
        (n / by).clamp(f32::MIN, f32::MAX)
    }
}
pub fn f32_abs(n: F32) -> F32 {
    n.abs()
}
pub fn f32_negate(n: F32) -> F32 {
    -n
}
pub fn f32_round(Mode·n { mode, n }: Mode·n<Round_mode, F32>) -> F32 {
    match mode {
        Round_mode::Up(Blank {}) => n.ceil(),
        Round_mode::Down(Blank {}) => n.floor(),
        Round_mode::Away_from_0(Blank {}) => {
            // I'm not convinced this is the fastest but since this is by far the
            // most common implementation I've seen I'm hoping this gets optimized at least
            n.abs().ceil() * n.signum()
        }
        Round_mode::Toward_0(Blank {}) => n.trunc(),
        Round_mode::Nearest_else_away_from_0(Blank {}) => n.round(),
        Round_mode::Nearest_else_even(Blank {}) => n.round_ties_even(),
    }
}
pub fn f32_to_i32_clamp(operation: Mode·n<Round_mode, F32>) -> I32 {
    f32_round(operation) as I32
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
pub fn char_to_code_point(char: Char) -> U32 {
    <u32 as std::convert::From<char>>::from(char)
}
pub fn u32_code_point_to_char(code_point: U32) -> Opt<Char> {
    Opt::from_option(char::from_u32(code_point))
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
    Direction·state·step·str {
        direction,
        str,
        state,
        step,
    }: Direction·state·step·str<
        Down·Up<Blank, Blank>,
        State,
        Fn<Char·state<Char, State>, State>,
        Str,
    >,
) -> State {
    let reduce = |state, char| {
        step(Char·state {
            state: state,
            char: char,
        })
    };
    match direction {
        Down·Up::Up(Blank {}) => std::iter::Iterator::fold(&mut str.chars(), state, reduce),
        Down·Up::Down(Blank {}) => {
            std::iter::Iterator::fold(&mut std::iter::Iterator::rev(str.chars()), state, reduce)
        }
    }
}
pub fn str_chars_fold_while<Exit, GoOn>(
    Direction·state·step·str {
        direction,
        str,
        state,
        step,
    }: Direction·state·step·str<
        Down·Up<Blank, Blank>,
        GoOn,
        Fn<Char·state<Char, GoOn>, Exit·Go_on<Exit, GoOn>>,
        Str,
    >,
) -> Exit·Go_on<Exit, GoOn> {
    let reduce = |state, char| Exit·Go_on::into_control_flow(step(Char·state { state, char }));
    Exit·Go_on::from_control_flow(match direction {
        Down·Up::Up(Blank {}) => std::iter::Iterator::try_fold(&mut str.chars(), state, reduce),
        Down·Up::Down(Blank {}) => {
            std::iter::Iterator::try_fold(&mut std::iter::Iterator::rev(str.chars()), state, reduce)
        }
    })
}

pub fn opt_present<Present>(present: Present) -> Opt<Present> {
    Opt::Present(present)
}

pub fn slot_rid<Origin>(_: Origin_rid·slot<Origin_rid<Origin>, Slot<Origin>>) -> Blank {
    Blank {}
}
pub fn slot_index<Origin>(slot: Slot<Origin>) -> Index·slot<u32, Slot<Origin>> {
    Index·slot {
        index: slot.index,
        slot: slot,
    }
}
pub fn slot_to_span<Origin>(slot: Slot<Origin>) -> Span<Origin> {
    Span {
        start: slot,
        length: std::num::NonZeroU32::MIN,
    }
}

pub fn span_rid<Origin>(_: Origin_rid·span<Origin_rid<Origin>, Span<Origin>>) -> Blank {
    Blank {}
}
pub fn opt_span_rid<Origin>(_: Origin_rid·span<Origin_rid<Origin>, Opt<Span<Origin>>>) -> Blank {
    Blank {}
}
pub fn span_start<Origin>(span: Span<Origin>) -> After·start<Opt<Span<Origin>>, Slot<Origin>> {
    After·start {
        after: match std::num::NonZeroU32::new(p32_predecessor(span.length)) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(after_length) => Opt::Present(Span {
                start: Slot::<Origin>::from_index(span.start.index + 1),
                length: after_length,
            }),
        },
        start: span.start,
    }
}
pub fn span_end<Origin>(span: Span<Origin>) -> Before·end<Opt<Span<Origin>>, Slot<Origin>> {
    Before·end {
        end: Slot::<Origin>::from_index(span.end_index()),
        before: match std::num::NonZeroU32::new(p32_predecessor(span.length)) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(before_length) => Opt::Present(Span {
                start: Slot::<Origin>::from_index(span.start.index - 1),
                length: before_length,
            }),
        },
    }
}
pub fn opt_span_length<Origin>(span: Opt<Span<Origin>>) -> Length·span<u32, Opt<Span<Origin>>> {
    Length·span {
        length: span.as_ref().length(),
        span: span,
    }
}
pub fn opt_span_take_start<Origin>(
    Length·span {
        length: length_to_take,
        span,
    }: Length·span<U32, Opt<Span<Origin>>>,
) -> After·start<Opt<Span<Origin>>, Opt<Span<Origin>>> {
    match std::num::NonZeroU32::new(length_to_take) {
        std::option::Option::None => After·start {
            start: Opt::Absent(Blank {}),
            after: span,
        },
        std::option::Option::Some(positive_length_to_take) => match span {
            Opt::Absent(Blank {}) => After·start {
                start: Opt::Absent(Blank {}),
                after: Opt::Absent(Blank {}),
            },
            Opt::Present(span) => {
                let After·start { start, after } = span_take_start_positive(Length·span {
                    span: span,
                    length: positive_length_to_take,
                });
                After·start {
                    start: Opt::Present(start),
                    after: after,
                }
            }
        },
    }
}
pub fn span_take_start_positive<Origin>(
    Length·span {
        length: length_to_take,
        span,
    }: Length·span<P32, Span<Origin>>,
) -> After·start<Opt<Span<Origin>>, Span<Origin>> {
    After·start {
        after: match std::num::NonZeroU32::new(u32::saturating_sub(
            span.length.get(),
            length_to_take.get(),
        )) {
            std::option::Option::None => Opt::Absent(Blank {}),
            std::option::Option::Some(after_length) => Opt::Present(Span {
                start: Slot::from_index(span.start.index + length_to_take.get()),
                length: after_length,
            }),
        },
        start: Span {
            start: span.start,
            length: length_to_take,
        },
    }
}
pub fn opt_span_fold<Origin, State>(
    Direction·span·state·step {
        direction,
        span,
        state,
        step,
    }: Direction·span·state·step<
        Down·Up<Blank, Blank>,
        Opt<Span<Origin>>,
        State,
        Fn<Slot·state<Slot<Origin>, State>, State>,
    >,
) -> State {
    let reduce = |state, index| {
        step(Slot·state {
            state,
            slot: Slot::from_index(index),
        })
    };
    match direction {
        Down·Up::Up(Blank {}) => {
            std::iter::Iterator::fold(&mut span.as_ref().to_range_u32(), state, reduce)
        }
        Down·Up::Down(Blank {}) => std::iter::Iterator::fold(
            &mut std::iter::Iterator::rev(span.as_ref().to_range_u32()),
            state,
            reduce,
        ),
    }
}
pub fn opt_span_fold_while<Origin, Exit, GoOn>(
    Direction·span·state·step {
        direction,
        span,
        state,
        step,
    }: Direction·span·state·step<
        Down·Up<Blank, Blank>,
        Opt<Span<Origin>>,
        GoOn,
        Fn<Slot·state<Slot<Origin>, GoOn>, Exit·Go_on<Exit, GoOn>>,
    >,
) -> Exit·Go_on<Exit·remaining<Exit, Opt<Span<Origin>>>, GoOn> {
    let reduce = |state, index| {
        Exit·Go_on::into_control_flow(step(Slot·state {
            state: state,
            slot: Slot::from_index(index),
        }))
        .map_break(|exit| (index, exit))
    };
    let state_after_fold = match direction {
        Down·Up::Down(Blank {}) => std::iter::Iterator::try_fold(
            &mut std::iter::Iterator::rev(span.as_ref().to_range_u32()),
            state,
            reduce,
        ),
        Down·Up::Up(Blank {}) => {
            std::iter::Iterator::try_fold(&mut span.as_ref().to_range_u32(), state, reduce)
        }
    };
    match state_after_fold {
        std::ops::ControlFlow::Continue(state) => Exit·Go_on::Go_on(state),
        std::ops::ControlFlow::Break((exit_index, exit_state)) => {
            let After·start {
                start: _,
                after: not_folded_over_opt_span,
            } = opt_span_take_start(Length·span {
                span: span,
                length: exit_index + 1,
            });
            Exit·Go_on::Exit(Exit·remaining {
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
    if span.end_index() + 1 == slot_to_add.index {
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
    if span.end_index() + 1 == span_to_add.start.index {
        Apart·connected {
            connected: Span {
                start: span.start,
                length: span.length.saturating_add(span_to_add.length.get()),
            },
            apart: Opt::Absent(Blank {}),
        }
    } else if span_to_add.end_index() + 1 == span.start.index {
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

pub fn origin_rid<LocalOrigin>(_: Origin<LocalOrigin>) -> Blank {
    Blank {}
}

pub fn origin_rid_rid<Origin>(_: Origin_rid<Origin>) -> Blank {
    Blank {}
}
pub fn origin_rid_dup<Origin>(
    origin_rid: Origin_rid<Origin>,
) -> A·b<Origin_rid<Origin>, Origin_rid<Origin>> {
    A·b {
        a: origin_rid,
        b: origin_rid,
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
pub fn vec_take<Origin, Element>(
    Slot·vec { mut vec, slot }: Slot·vec<Slot<Origin>, Vec<Origin, Element>>,
) -> Element·vec<Element, Vec<Origin, Element>> {
    let element = vec.take(slot);
    Element·vec {
        element: element,
        vec: vec,
    }
}
pub fn vec_opt_span_fold<Origin, Element, State>(
    vec: Vec<Origin, Element>,
    span: Opt<Span<Origin>>,
    state: State,
    step: Fn<Element·state<Element, State>, State>,
) -> Vec<Origin, Element> {
    match span {
        Opt::Absent(Blank {}) => vec,
        Opt::Present(shrink_span) => vec_span_fold(vec, shrink_span, state, step),
    }
}
pub fn vec_span_fold<Origin, Element, State>(
    mut vec: Vec<Origin, Element>,
    shrink_span: Span<Origin>,
    state: State,
    step: Fn<Element·state<Element, State>, State>,
) -> Vec<Origin, Element> {
    vec.vacate_and_consume_span_iterator(shrink_span, |mut elements| {
        std::iter::Iterator::fold(&mut elements, state, |state, element| {
            step(Element·state {
                element: element,
                state,
            })
        })
    });
    vec
}
pub fn vec_occupied_count<Origin, Element, State>(
    vec: Vec<Origin, Element>,
) -> Occupied_count·vec<u32, Vec<Origin, Element>> {
    Occupied_count·vec {
        occupied_count: vec.occupied_count_u32(),
        vec: vec,
    }
}
pub fn vec_rid<Origin, Element>(_: Vec<Origin, Element>) -> Blank {
    Blank {}
}
pub fn vec_fold<Origin, Element, State>(
    Direction·state·step·vec {
        direction,
        state,
        step,
        vec,
    }: Direction·state·step·vec<
        Down·Up<Blank, Blank>,
        State,
        Fn<Element·state<Element, State>, State>,
        Vec<Origin, Element>,
    >,
) -> State {
    match direction {
        Down·Up::Up(Blank {}) => std::iter::Iterator::fold(
            &mut vec.into_occupied_elements(),
            state,
            |state, element| {
                step(Element·state {
                    element: element,
                    state: state,
                })
            },
        ),
        Down·Up::Down(Blank {}) => std::iter::Iterator::fold(
            &mut vec.into_occupied_elements_rev(),
            state,
            |state, element| {
                step(Element·state {
                    element: element,
                    state: state,
                })
            },
        ),
    }
}
pub fn vec_fold_with_origin_rid<Origin, Element, State>(
    Direction·state·step·vec {
        direction,
        state,
        step,
        vec,
    }: Direction·state·step·vec<
        Down·Up<Blank, Blank>,
        State,
        Fn<Element·origin_rid·state<Element, Origin_rid<Origin>, State>, State>,
        Vec<Origin, Element>,
    >,
) -> Origin_rid·state<Origin_rid<Origin>, State> {
    let origin_rid = Origin_rid(std::marker::PhantomData::<Origin>);
    Origin_rid·state {
        origin_rid: origin_rid,
        state: match direction {
            Down·Up::Up(Blank {}) => std::iter::Iterator::fold(
                &mut vec.into_occupied_elements(),
                state,
                |state, element| {
                    step(Element·origin_rid·state {
                        element: element,
                        state: state,
                        origin_rid: origin_rid,
                    })
                },
            ),
            Down·Up::Down(Blank {}) => std::iter::Iterator::fold(
                &mut vec.into_occupied_elements_rev(),
                state,
                |state, element| {
                    step(Element·origin_rid·state {
                        element: element,
                        state: state,
                        origin_rid: origin_rid,
                    })
                },
            ),
        },
    }
}
pub fn vec_add_ignoring_vacant<Origin, Element>(
    New·vec {
        mut vec,
        new: new_element,
    }: New·vec<Element, Vec<Origin, Element>>,
) -> Slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add_ignoring_vacant(new_element);
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
    In_·slot·update·vec {
        mut vec,
        slot,
        in_,
        update: element_update,
    }: In_·slot·update·vec<
        In,
        Slot<Origin>,
        Fn<Element·in<Element, In>, Element·out<Element, Out>>,
        Vec<Origin, Element>,
    >,
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

pub fn vec_opt_span_add<Origin, Element>(
    New·span·vec {
        mut vec,
        span,
        new: new_element,
    }: New·span·vec<Element, Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Span·vec<Span<Origin>, Vec<Origin, Element>> {
    match span {
        Absent·Present::Absent(Blank {}) => {
            let new_slot = vec.add(new_element);
            Span·vec {
                vec: vec,
                span: slot_to_span(new_slot),
            }
        }
        Absent·Present::Present(span) => vec_span_add(New·span·vec {
            vec: vec,
            span: span,
            new: new_element,
        }),
    }
}
pub fn vec_span_add<Origin, Element>(
    New·span·vec {
        mut vec,
        span,
        new: new_element,
    }: New·span·vec<Element, Span<Origin>, Vec<Origin, Element>>,
) -> Span·vec<Span<Origin>, Vec<Origin, Element>> {
    let grown_span = vec.span_add(span, new_element);
    Span·vec {
        vec: vec,
        span: grown_span,
    }
}
pub fn vec_opt_span_add_str<Origin>(
    New·span·vec {
        mut vec,
        span,
        new: new_str,
    }: New·span·vec<Str, Opt<Span<Origin>>, Vec<Origin, Char>>,
) -> Span·vec<Opt<Span<Origin>>, Vec<Origin, Char>> {
    let grown_span = vec.opt_span_add_str(span, new_str);
    Span·vec {
        vec: vec,
        span: grown_span,
    }
}
pub fn vec_span_add_str<Origin>(
    New·span·vec {
        mut vec,
        span,
        new: new_str,
    }: New·span·vec<Str, Span<Origin>, Vec<Origin, Char>>,
) -> Span·vec<Span<Origin>, Vec<Origin, Char>> {
    let grown_span = vec.span_add_str(span, new_str);
    Span·vec {
        vec: vec,
        span: grown_span,
    }
}
pub fn vec_opt_span_snatch_vec_opt_span<GrowOrigin, ShrinkOrigin, Element>(
    New·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: New·shrink·span·vec<
        Opt<Span<ShrinkOrigin>>,
        Vec<ShrinkOrigin, Element>,
        Opt<Span<GrowOrigin>>,
        Vec<GrowOrigin, Element>,
    >,
) -> Grown·shrunk·span<Vec<GrowOrigin, Element>, Vec<ShrinkOrigin, Element>, Opt<Span<GrowOrigin>>>
{
    let maybe_grown_span = match shrink_span {
        Absent·Present::Absent(Blank {}) => span,
        Absent·Present::Present(shrink_span) => {
            Opt::Present(vec.opt_span_snatch_vec_span(span, &mut shrink_vec, shrink_span))
        }
    };
    Grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: maybe_grown_span,
    }
}
pub fn vec_span_snatch_vec_opt_span<GrowOrigin, ShrinkOrigin, Element>(
    New·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: New·shrink·span·vec<
        Opt<Span<ShrinkOrigin>>,
        Vec<ShrinkOrigin, Element>,
        Span<GrowOrigin>,
        Vec<GrowOrigin, Element>,
    >,
) -> Grown·shrunk·span<Vec<GrowOrigin, Element>, Vec<ShrinkOrigin, Element>, Span<GrowOrigin>> {
    let maybe_grown_span = match shrink_span {
        Absent·Present::Absent(Blank {}) => span,
        Absent·Present::Present(shrink_span) => {
            vec.span_snatch_vec_span(span, &mut shrink_vec, shrink_span)
        }
    };
    Grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: maybe_grown_span,
    }
}
pub fn vec_opt_span_snatch_vec_span<GrowOrigin, ShrinkOrigin, Element>(
    New·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: New·shrink·span·vec<
        Span<ShrinkOrigin>,
        Vec<ShrinkOrigin, Element>,
        Opt<Span<GrowOrigin>>,
        Vec<GrowOrigin, Element>,
    >,
) -> Grown·shrunk·span<Vec<GrowOrigin, Element>, Vec<ShrinkOrigin, Element>, Span<GrowOrigin>> {
    let grown_span = vec.opt_span_snatch_vec_span(span, &mut shrink_vec, shrink_span);
    Grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: grown_span,
    }
}
pub fn vec_span_snatch_vec_span<GrowOrigin, ShrinkOrigin, Element>(
    New·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: New·shrink·span·vec<
        Span<ShrinkOrigin>,
        Vec<ShrinkOrigin, Element>,
        Span<GrowOrigin>,
        Vec<GrowOrigin, Element>,
    >,
) -> Grown·shrunk·span<Vec<GrowOrigin, Element>, Vec<ShrinkOrigin, Element>, Span<GrowOrigin>> {
    let grown_span = vec.span_snatch_vec_span(span, &mut shrink_vec, shrink_span);
    Grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: grown_span,
    }
}

pub fn vec_move_opt_span_to_vacant<Origin, Element>(
    Span·vec { span, mut vec }: Span·vec<Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    match span {
        Opt::Absent(Blank {}) => Span·vec {
            span: Opt::Absent(Blank {}),
            vec: vec,
        },
        Opt::Present(span) => {
            let moved_span = vec.move_span_to_vacant(span);
            Span·vec {
                span: Opt::Present(moved_span),
                vec: vec,
            }
        }
    }
}
pub fn vec_move_span_to_vacant<Origin, Element>(
    Span·vec { span, mut vec }: Span·vec<Span<Origin>, Vec<Origin, Element>>,
) -> Span·vec<Span<Origin>, Vec<Origin, Element>> {
    let moved_span = vec.move_span_to_vacant(span);
    Span·vec {
        span: moved_span,
        vec: vec,
    }
}
