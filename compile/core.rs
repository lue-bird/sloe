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
    clippy::needless_update,
    clippy::must_use_candidate
)]
extern crate std;

// Most module members are directly usable by sloe code to avoid name clashes with generated functions and types.
// The remaining few member names must be explicitly added to `sloe::name_to_uppercase_rust` and `name_to_lowercase_rust`

#[derive(Clone, Copy, Debug)]
pub struct Record·a·b<A, B> {
    pub a: A,
    pub b: B,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·a·b·carry<A, B, Carry> {
    pub a: A,
    pub b: B,
    pub carry: Carry,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·p·u<P, U> {
    pub p: P,
    pub u: U,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·by·n<By, N> {
    pub by: By,
    pub n: N,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·mode·n<Mode, N> {
    pub mode: Mode,
    pub n: N,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·max·min<Max, Min> {
    pub max: Max,
    pub min: Min,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·element·in<Element, In> {
    pub element: Element,
    pub in_: In,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·element·slot<Element, Slot> {
    pub element: Element,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·slot·state<Slot, State> {
    pub slot: Slot,
    pub state: State,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·char·state<Char, State> {
    pub char: Char,
    pub state: State,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·element·out<Element, Out> {
    pub element: Element,
    pub out: Out,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·after·start<After, Start> {
    pub after: After,
    pub start: Start,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·before·end<Before, End> {
    pub before: Before,
    pub end: End,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·apart·connected<Apart, Connected> {
    pub apart: Apart,
    pub connected: Connected,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·index·slot<Index, Slot> {
    pub index: Index,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·length·span<Length, Span> {
    pub length: Length,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·exit·remaining<Exit, Remaining> {
    pub exit: Exit,
    pub remaining: Remaining,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·length·vec<Length, Vec> {
    pub length: Length,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·slot·vec<Slot, Vec> {
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·new·vec<New, Vec> {
    pub new: New,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·new·slot·vec<New, Slot, Vec> {
    pub new: New,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·in_·slot·update·vec<In, Slot, Update, Vec> {
    pub in_: In,
    pub slot: Slot,
    pub update: Update,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·span·vec<Span, Vec> {
    pub span: Span,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·new·span·vec<New, Span, Vec> {
    pub new: New,
    pub span: Span,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·new·span<New, Span> {
    pub new: New,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·new·shrink·span·vec<New, Shrink, Span, Vec> {
    pub new: New,
    pub shrink: Shrink,
    pub span: Span,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·out·slot·vec<Out, Slot, Vec> {
    pub out: Out,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·out·span<Out, Span> {
    pub out: Out,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·grown·shrunk·span<Grown, Shrunk, Span> {
    pub grown: Grown,
    pub shrunk: Shrunk,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·element·vec<Element, Vec> {
    pub element: Element,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·element·slot·vec<Element, Slot, Vec> {
    pub element: Element,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·old·slot·vec<Old_element, Slot, Vec> {
    pub old: Old_element,
    pub slot: Slot,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·direction·state·step·str<Direction, State, Step, Str> {
    pub direction: Direction,
    pub state: State,
    pub step: Step,
    pub str: Str,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·direction·state·step·vec<Direction, State, Step, Vec> {
    pub direction: Direction,
    pub state: State,
    pub step: Step,
    pub vec: Vec,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·direction·span·state·step<Direction, Span, State, Step> {
    pub direction: Direction,
    pub span: Span,
    pub state: State,
    pub step: Step,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·carry·wrapped<Carry, Wrapped> {
    pub carry: Carry,
    pub wrapped: Wrapped,
}
#[derive(Clone, Copy, Debug)]
pub enum Choice·Absent·Present<Absent, Present> {
    Absent(Absent),
    Present(Present),
}
#[derive(Clone, Copy, Debug)]
pub enum Choice·Exit·Go_on<Exit, Go_on> {
    Exit(Exit),
    Go_on(Go_on),
}
#[derive(Clone, Copy, Debug)]
pub enum Choice·Down·Up<Down, Up> {
    Down(Down),
    Up(Up),
}
#[derive(Clone, Copy, Debug)]
pub enum Choice·Contained·Overflowed<Contained, Overflowed> {
    Contained(Contained),
    Overflowed(Overflowed),
}
#[derive(Clone, Copy, Debug)]
pub enum Choice·Away_from_0·Down·Nearest_else_away_from_0·Nearest_else_even·Toward_0·Up<
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

/// empty record, represented as unit
pub type Record = ();
/// empty choice
#[derive(Clone, Copy, Debug)]
pub enum Choice {}

pub type P32 = std::num::NonZeroU32;
pub type U32 = u32;
pub type I32 = i32;
pub type F32 = f32;
pub type Char = char;
pub type Str = &'static str;
pub type Fn<In, Out> = fn(In) -> Out;
pub type Opt<Present> = Choice·Absent·Present<Record, Present>;
pub type Round_mode =
    Choice·Away_from_0·Down·Nearest_else_away_from_0·Nearest_else_even·Toward_0·Up<
        Record,
        Record,
        Record,
        Record,
        Record,
        Record,
    >;

#[derive(Debug)]
pub struct Origin<LocalOrigin>(LocalOrigin);
#[derive(Debug)]
pub struct Vec<LocalOrigin, Element> {
    // invariants (in addition to the invariants of (Empty_)Slot/Span):
    // - no `Empty_span`s in `.vacant` are connected
    //   (and thus could be combined into one larger consecutive span)
    // - any index contained in any vacant `Empty_span` is less than elements.len()
    // - any index contained in any vacant `Empty_span` should be assumed uninitialized
    //   in `.elements`
    //
    // -------
    // `.elements` contains `std::mem::MaybeUninit<Element>` because
    // - functions like `vec.add_empty` explicitly require uninitialized memory.
    //   creating uninitialized memory of type `Element` out of thin air is UB
    // - it matches well semantically: access is inherently unsafe.
    //   vec::Vec<Element> makes it appear safe
    // - drawbacks (like the removal of niches) do not have an impact here
    // - it prevents drop from being called
    //   which could double-free on already vacated elements.
    //   Vec<_,_> originally implemented a custom Drop as
    //   `for e in self.elements.drain(..) { std::mem::forget(e); }`
    //   with the following documentation:
    //     At this point, all elements are either
    //     - handled (in sloe code this is always the case or you'll get an error)
    //     - unhandled (only possible from rust code when a `Slot`/`Span` is dropped)
    //     - empty (only possible from rust code when a `Empty_span`/`Empty_span` is dropped)
    //     - occupied (only possible from rust code).
    //
    //     If we used the regular Drop implementation, elements that were already vacated
    //     or temporarily extracted (where e.g. the resulting `Empty_slot` from `vec.element()` was dropped)
    //     could be freed twice (!).
    //     So the only thing that can realistically be done is to "leak" all remaining elements.
    //
    //     To recap, if some rust code kept some slots occupied,
    //     we _must_ prevent double-frees by leaking those elements.
    //     This is not as bad as you might think:
    //     - dropping a `Slot`/`Empty_slot` is always a leak
    //       but it cannot reasonably prevented in rust. It's the cost of doing business
    //     - in a `Vec<Origin, Element>`, the element type will realistically not be a type that
    //       directly points to the heap. In fact in sloe you cannot even put more than one vec inside of
    //       another vec as each vec has a different origin!
    elements: std::vec::Vec<std::mem::MaybeUninit<Element>>,
    // Performance assumption:
    // Neighboring elements are way more likely to be vacated together.
    // Think e.g. vec_span_add_vec_span but also
    // regular chunks of nested individual slots which were likely allocated close to their neighbors.
    //
    // It is also assumed that there won't be a large amount of these vacant spans
    // so e.g. HashSet loses despite having a faster "find out if this index is vacant".
    // If usage ends up suggesting otherwise, we should change accordingly
    vacant: std::vec::Vec<Empty_span<LocalOrigin>>,
}
pub type Slot<LocalOrigin> = Slot_with_occupancy<LocalOrigin, Occupied>;
pub type Empty_slot<LocalOrigin> = Slot_with_occupancy<LocalOrigin, Empty>;
#[non_exhaustive]
pub struct Slot_with_occupancy<LocalOrigin, Occupancy> {
    pub origin: std::marker::PhantomData<LocalOrigin>,
    pub occupancy: std::marker::PhantomData<Occupancy>,
    // consider switching to NonZeroU32 to create a niche for use with Option<Slot<>>
    pub index: u32,
}
pub type Span<LocalOrigin> = Span_with_occupancy<LocalOrigin, Occupied>;
pub type Empty_span<LocalOrigin> = Span_with_occupancy<LocalOrigin, Empty>;
#[non_exhaustive]
pub struct Span_with_occupancy<LocalOrigin, Occupancy> {
    pub start: Slot_with_occupancy<LocalOrigin, Occupancy>,
    // consider instead: end_index: NonZeroU32.
    // This makes combining 2 opt_spans and converting to ops::Range a bit faster,
    // at the cost of other operations like checking a vec's occupied count
    pub length: std::num::NonZeroU32,
}
pub struct Empty();
pub struct Occupied();

impl<Origin, Occupancy> std::fmt::Debug for Slot_with_occupancy<Origin, Occupancy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slot").field("index", &self.index).finish()
    }
}
impl<Origin, Occupancy> std::fmt::Debug for Span_with_occupancy<Origin, Occupancy> {
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
            std::option::Option::None => Opt::Absent(()),
            std::option::Option::Some(present) => Opt::Present(present),
        }
    }
    pub fn into_option(self) -> std::option::Option<A> {
        match self {
            Opt::Absent(()) => std::option::Option::None,
            Opt::Present(present) => std::option::Option::Some(present),
        }
    }
    pub fn as_ref(&self) -> Opt<&A> {
        match self {
            Opt::Absent(()) => Opt::Absent(()),
            Opt::Present(present) => Opt::Present(present),
        }
    }
    pub fn as_mut(&mut self) -> Opt<&mut A> {
        match self {
            Opt::Absent(()) => Opt::Absent(()),
            Opt::Present(present) => Opt::Present(present),
        }
    }
}

impl<Exit, GoOn> Choice·Exit·Go_on<Exit, GoOn> {
    pub fn from_control_flow(control_flow: std::ops::ControlFlow<Exit, GoOn>) -> Self {
        match control_flow {
            std::ops::ControlFlow::Break(exit) => Choice·Exit·Go_on::Exit(exit),
            std::ops::ControlFlow::Continue(go_on) => Choice·Exit·Go_on::Go_on(go_on),
        }
    }
    pub fn into_control_flow(self) -> std::ops::ControlFlow<Exit, GoOn> {
        match self {
            Choice·Exit·Go_on::Exit(exit) => std::ops::ControlFlow::Break(exit),
            Choice·Exit·Go_on::Go_on(go_on) => std::ops::ControlFlow::Continue(go_on),
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

impl<LocalOrigin> Origin<LocalOrigin> {
    /// # Safety
    /// This constructor is exposed because sadly macros (namely origin_new!) require it.
    /// It's _very strongly_ recommended to instead only construct new origins with `origin_new!`.
    /// Misusing this constructor can lead to UB like unchecked out of bounds access.
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

impl<Origin, Element> Vec<Origin, Element> {
    /// Especially when working with estimates or future insertions, you usually want pre_allocate_at_least
    pub fn pre_allocate(&mut self, pre_allocated_length: u32) {
        self.elements.reserve_exact(pre_allocated_length as usize);
    }
    pub fn pre_allocate_at_least(&mut self, min_pre_allocated_length: u32) {
        self.elements.reserve(min_pre_allocated_length as usize);
    }
    pub fn element_ref<'a>(&'a self, slot: &'a Slot<Origin>) -> &'a Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe {
            self.elements
                .get_unchecked(slot.index as usize)
                .assume_init_ref()
        }
    }
    pub fn element_mut<'a>(&'a mut self, slot: &'a mut Slot<Origin>) -> &'a mut Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe {
            self.elements
                .get_unchecked_mut(slot.index as usize)
                .assume_init_mut()
        }
    }
    pub fn opt_span_slice<'a>(&'a self, opt_span: Opt<&'a Span<Origin>>) -> &'a [Element] {
        match opt_span {
            Opt::Absent(()) => &[],
            Opt::Present(span) => self.span_slice(span),
        }
    }
    pub fn span_slice<'a>(&'a self, span: &'a Span<Origin>) -> &'a [Element] {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid range
        unsafe {
            self.elements
                .get_unchecked(span.to_range())
                .assume_init_ref()
        }
    }
    pub fn opt_span_slice_mut<'a>(
        &'a mut self,
        opt_span: &'a mut Opt<Span<Origin>>,
    ) -> &'a mut [Element] {
        match opt_span {
            Opt::Absent(()) => &mut [],
            Opt::Present(span) => self.span_slice_mut(span),
        }
    }
    pub fn span_slice_mut<'a>(&'a mut self, span: &'a mut Span<Origin>) -> &'a mut [Element] {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid range
        unsafe {
            self.elements
                .get_unchecked_mut(span.to_range())
                .assume_init_mut()
        }
    }
    // TODO recheck lifetime of the consumed iterator
    pub fn consume_span_iterator<Out>(
        &mut self,
        mut span: Span<Origin>,
        consume_iterator: impl for<'iterator> std::ops::FnOnce(
            OwnedSliceIterator<'iterator, Element>,
        ) -> Out,
    ) -> Record·out·span<Out, Empty_span<Origin>> {
        // elements in the opt_span are consumed and never accessed after. During this whole ordeal
        // the elements are "locked" behind a mut ref
        let munched = consume_iterator(unsafe {
            mut_slice_into_owned_iterator(self.span_slice_mut(&mut span))
        });
        Record·out·span {
            out: munched,
            span: Empty_span::<Origin> {
                start: Empty_slot::<Origin>::from_index(span.start.index),
                length: span.length,
            },
        }
    }
    pub fn take_consume_span_iterator<Out>(
        &mut self,
        shrink_span: Span<Origin>,
        consume_iterator: impl for<'iterator> std::ops::FnOnce(
            OwnedSliceIterator<'iterator, Element>,
        ) -> Out,
    ) -> Out {
        let munched = self.consume_span_iterator(shrink_span, consume_iterator);
        self.span_rid(munched.span);
        munched.out
    }
    pub fn take(&mut self, slot: Slot<Origin>) -> Element {
        // vacated opt_span elements are never accessed, not even while vacating them
        let element = self.element(slot);
        self.slot_rid(element.slot);
        element.element
    }
    pub fn element(
        &mut self,
        mut slot: Slot<Origin>,
    ) -> Record·element·slot<Element, Empty_slot<Origin>> {
        // its unique slot is consumed, so this element cannot be accessed after
        let element = unsafe {
            std::ptr::NonNull::read(std::ptr::NonNull::from_ref(self.element_mut(&mut slot)))
        };
        Record·element·slot {
            element: element,
            slot: Empty_slot::<Origin>::from_index(slot.index),
        }
    }
    pub fn set(&mut self, slot: Empty_slot<Origin>, element: Element) -> Slot<Origin> {
        // Empty_slot always references valid position and is inaccessible after this operation
        unsafe { self.elements.get_unchecked_mut(slot.index as usize) }.write(element);
        Slot::<Origin>::from_index(slot.index)
    }
    pub fn slot_rid(&mut self, slot_to_vacate: Empty_slot<Origin>) {
        // can maybe be optimized
        self.span_rid(slot_to_vacate.to_span());
    }
    pub fn span_rid(&mut self, span_to_vacate: Empty_span<Origin>) {
        let maybe_vacant_span_index_connecting_earlier: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_span| {
                std::cmp::PartialEq::<u32>::eq(
                    &(vacant_span.end_index() + 1),
                    &span_to_vacate.start.index,
                )
            });
        let maybe_vacant_span_index_connecting_later: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_span| {
                std::cmp::PartialEq::<u32>::eq(
                    &(span_to_vacate.end_index() + 1),
                    &vacant_span.start.index,
                )
            });
        match (
            maybe_vacant_span_index_connecting_earlier,
            maybe_vacant_span_index_connecting_later,
        ) {
            (std::option::Option::None, std::option::Option::None) => {
                if (span_to_vacate.start.index + span_to_vacate.length.get() + 1) as usize
                    == self.elements.len()
                {
                    self.elements
                        .truncate(self.elements.len() - span_to_vacate.length.get() as usize);
                } else {
                    self.vacant.push(span_to_vacate);
                }
            }
            (
                std::option::Option::Some(index_connecting_earlier),
                std::option::Option::Some(index_connecting_later),
            ) => {
                // if both spans start connecting now, combine them
                let (earlier_span_start, earlier_span_length) = {
                    let earlier_span = &self.vacant[index_connecting_earlier];
                    (earlier_span.start.index, earlier_span.length)
                };
                let later_span_to_extend = &mut self.vacant[index_connecting_later];
                *later_span_to_extend = Empty_span {
                    start: Empty_slot::<Origin>::from_index(earlier_span_start),
                    length: std::num::NonZeroU32::saturating_add(
                        std::num::NonZeroU32::saturating_add(
                            earlier_span_length,
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
                *later_opt_span_to_extend = Empty_span {
                    start: span_to_vacate.start,
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
        self.elements.push(std::mem::MaybeUninit::new(new_element));
        Slot::from_index(added_index as u32)
    }
    pub fn add_empty_ignoring_vacant(&mut self) -> Empty_slot<Origin> {
        let added_index = self.elements.len();
        self.elements.push(std::mem::MaybeUninit::uninit());
        Empty_slot::from_index(added_index as u32)
    }
    pub fn add(&mut self, new_element: Element) -> Slot<Origin> {
        let empty_slot = self.add_empty();
        self.set(empty_slot, new_element)
    }
    pub fn add_empty(&mut self) -> Empty_slot<Origin> {
        match self.vacant.pop() {
            std::option::Option::None => self.add_empty_ignoring_vacant(),
            std::option::Option::Some(vacant_opt_span_to_occupy) => {
                if let std::option::Option::Some(remaining_length) =
                    std::num::NonZeroU32::new(p32_predecessor(vacant_opt_span_to_occupy.length))
                {
                    self.vacant.push(Empty_span {
                        start: Empty_slot::<Origin>::from_index(
                            vacant_opt_span_to_occupy.start.index + 1,
                        ),
                        length: remaining_length,
                    });
                }
                vacant_opt_span_to_occupy.start
            }
        }
    }
    // potential improvement: return Empty_span
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
                let start_to_occupy_from = vacant_opt_span_to_occupy.start.index;
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
                self.elements.splice(
                    grow_span.to_range(),
                    new_elements.map(std::mem::MaybeUninit::new),
                );
                grow_span
            }
        }
    }
    pub fn add_iterator_ignoring_vacant(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
    ) -> Opt<Span<Origin>> {
        let length_without_new_elements = self.elements.len();
        std::iter::Extend::extend(
            &mut self.elements,
            new_elements.map(std::mem::MaybeUninit::new),
        );
        match std::num::NonZeroU32::new((self.elements.len() - length_without_new_elements) as u32)
        {
            std::option::Option::None => Opt::Absent(()),
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
        std::iter::Extend::extend(
            &mut self.elements,
            new_elements.map(std::mem::MaybeUninit::new),
        );
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
            std::option::Option::None => Opt::Absent(()),
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
            return Opt::Absent(());
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
        shrink.take_consume_span_iterator(shrink_span, |shrink_elements| {
            self.add_iterator_filled(shrink_elements, shrink_span_length)
        })
    }
    pub fn add_take_vec_span_ignoring_vacant<ShrinkOrigin>(
        &mut self,
        shrink: &mut Vec<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        let shrink_span_length = shrink_span.length;
        shrink.take_consume_span_iterator(shrink_span, |shrink_elements| {
            self.add_iterator_filled_ignoring_vacant(shrink_elements, shrink_span_length)
        })
    }
    pub fn opt_span_add_take_vec_span<ShrinkOrigin>(
        &mut self,
        span: Opt<Span<Origin>>,
        shrink_vec: &mut Vec<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        match span {
            Opt::Absent(()) => self.add_take_vec_span_ignoring_vacant(shrink_vec, shrink_span),
            Opt::Present(span) => self.span_add_take_vec_span(span, shrink_vec, shrink_span),
        }
    }
    pub fn span_add_take_vec_span<ShrinkOrigin>(
        &mut self,
        span: Span<Origin>,
        shrink_vec: &mut Vec<ShrinkOrigin, Element>,
        shrink_span: Span<ShrinkOrigin>,
    ) -> Span<Origin> {
        shrink_vec.take_consume_span_iterator(shrink_span, |elements| {
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
        std::iter::Extend::extend(
            &mut self.elements,
            new_elements.map(std::mem::MaybeUninit::new),
        );
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
        self.elements.push(std::mem::MaybeUninit::new(new_element));
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
        self.span_rid(Empty_span::<Origin> {
            start: Empty_slot::<Origin>::from_index(span.start.index),
            length: span.length,
        });
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
                    std::iter::Iterator::map(elements_to_move, std::mem::MaybeUninit::new),
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
    /// counts both occupied positions and temporarily empty ones referenced by `empty-slot`s
    pub fn not_vacant_count_usize(&self) -> usize {
        usize::saturating_sub(self.elements.len(), self.vacant_count_usize())
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
            Choice·Absent·Present::Absent(()) => self.add_str_ignoring_vacant(new_str),
            Choice·Absent·Present::Present(span) => {
                Opt::Present(self.span_add_str(span, new_str))
            }
        }
    }
    pub fn span_add_str(&mut self, span: Span<Origin>, new_str: Str) -> Span<Origin> {
        self.span_add_iterator(span, new_str.chars())
    }
}

impl<Origin, Occupancy> Slot_with_occupancy<Origin, Occupancy> {
    /// use with caution. duplicate use or out-of-bounds of the given index can lead to UB.
    /// consider making it unsafe and exposing it
    fn from_index(index: u32) -> Slot_with_occupancy<Origin, Occupancy> {
        Slot_with_occupancy {
            origin: std::marker::PhantomData::<Origin>,
            occupancy: std::marker::PhantomData::<Occupancy>,
            index: index,
        }
    }
    fn to_span(self) -> Span_with_occupancy<Origin, Occupancy> {
        Span_with_occupancy {
            start: self,
            length: std::num::NonZeroU32::MIN,
        }
    }
}

impl<Origin, Occupancy> Span_with_occupancy<Origin, Occupancy> {
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
    pub fn split_start(
        self,
    ) -> Record·after·start<
        Opt<Span_with_occupancy<Origin, Occupancy>>,
        Slot_with_occupancy<Origin, Occupancy>,
    > {
        Record·after·start {
            after: match std::num::NonZeroU32::new(p32_predecessor(self.length)) {
                std::option::Option::None => Opt::Absent(()),
                std::option::Option::Some(after_length) => Opt::Present(Span_with_occupancy {
                    start: Slot_with_occupancy::<Origin, Occupancy>::from_index(
                        self.start.index + 1,
                    ),
                    length: after_length,
                }),
            },
            start: self.start,
        }
    }
    pub fn split_end(
        self,
    ) -> Record·before·end<
        Opt<Span_with_occupancy<Origin, Occupancy>>,
        Slot_with_occupancy<Origin, Occupancy>,
    > {
        Record·before·end {
            end: Slot_with_occupancy::<Origin, Occupancy>::from_index(self.end_index()),
            before: match std::num::NonZeroU32::new(p32_predecessor(self.length)) {
                std::option::Option::None => Opt::Absent(()),
                std::option::Option::Some(before_length) => Opt::Present(Span_with_occupancy {
                    start: Slot_with_occupancy::<Origin, Occupancy>::from_index(
                        self.start.index - 1,
                    ),
                    length: before_length,
                }),
            },
        }
    }
    pub fn split_start_positive(
        self,
        start_length: std::num::NonZeroU32,
    ) -> Record·after·start<
        Opt<Span_with_occupancy<Origin, Occupancy>>,
        Span_with_occupancy<Origin, Occupancy>,
    > {
        Record·after·start {
            after: match std::num::NonZeroU32::new(u32::saturating_sub(
                self.length.get(),
                start_length.get(),
            )) {
                std::option::Option::None => Opt::Absent(()),
                std::option::Option::Some(after_length) => Opt::Present(Span_with_occupancy {
                    start: Slot_with_occupancy::<Origin, Occupancy>::from_index(
                        self.start.index + start_length.get(),
                    ),
                    length: after_length,
                }),
            },
            start: Span_with_occupancy {
                start: self.start,
                length: start_length,
            },
        }
    }
    pub fn connect_slot(
        self,
        slot_to_add: Slot_with_occupancy<Origin, Occupancy>,
    ) -> Record·apart·connected<
        Opt<Slot_with_occupancy<Origin, Occupancy>>,
        Span_with_occupancy<Origin, Occupancy>,
    > {
        if self.end_index() + 1 == slot_to_add.index {
            Record·apart·connected {
                connected: Span_with_occupancy {
                    start: self.start,
                    length: self.length.saturating_add(1),
                },
                apart: Opt::Absent(()),
            }
        } else if slot_to_add.index + 1 == self.start.index {
            Record·apart·connected {
                connected: Span_with_occupancy {
                    start: slot_to_add,
                    length: self.length.saturating_add(1),
                },
                apart: Opt::Absent(()),
            }
        } else {
            Record·apart·connected {
                connected: self,
                apart: Opt::Present(slot_to_add),
            }
        }
    }
    pub fn connect(
        self,
        span_to_add: Span_with_occupancy<Origin, Occupancy>,
    ) -> Record·apart·connected<
        Opt<Span_with_occupancy<Origin, Occupancy>>,
        Span_with_occupancy<Origin, Occupancy>,
    > {
        if self.end_index() + 1 == span_to_add.start.index {
            Record·apart·connected {
                connected: Span_with_occupancy {
                    start: self.start,
                    length: self.length.saturating_add(span_to_add.length.get()),
                },
                apart: Opt::Absent(()),
            }
        } else if span_to_add.end_index() + 1 == self.start.index {
            Record·apart·connected {
                connected: Span_with_occupancy {
                    start: span_to_add.start,
                    length: self.length.saturating_add(span_to_add.length.get()),
                },
                apart: Opt::Absent(()),
            }
        } else {
            Record·apart·connected {
                connected: self,
                apart: Opt::Present(span_to_add),
            }
        }
    }
}

impl<Origin, Occupancy> Opt<&Span_with_occupancy<Origin, Occupancy>> {
    pub fn to_range(self) -> std::ops::Range<usize> {
        match self {
            Opt::Absent(()) => <std::ops::Range<usize> as std::default::Default>::default(),
            Opt::Present(span) => span.to_range(),
        }
    }
    pub fn to_range_u32(self) -> std::ops::Range<u32> {
        match self {
            Opt::Absent(()) => <std::ops::Range<u32> as std::default::Default>::default(),
            Opt::Present(span) => span.to_range_u32(),
        }
    }
    pub fn length(self) -> u32 {
        match self {
            Opt::Absent(()) => 0,
            Opt::Present(span) => span.length.get(),
        }
    }
}

pub fn p32_dup(n: P32) -> Record·a·b<P32, P32> {
    Record·a·b { a: n, b: n }
}
pub fn p32_rid(_: P32) -> Record {
    ()
}
pub fn p32_predecessor(n: P32) -> U32 {
    n.get() - 1
}
pub fn p32_add_clamp(Record·p·u { p, u }: Record·p·u<P32, U32>) -> P32 {
    p.saturating_add(u)
}
pub fn p32_to_u32(n: P32) -> U32 {
    n.get()
}
pub fn u32_to_p32(n: U32) -> Opt<P32> {
    Opt::from_option(P32::new(n))
}
pub fn u32_rid(_: U32) -> Record {
    ()
}
pub fn u32_dup(n: U32) -> Record·a·b<U32, U32> {
    Record·a·b { a: n, b: n }
}
#[expect(clippy::cast_precision_loss)]
pub fn u32_to_f32(n: U32) -> F32 {
    n as F32
}
pub fn u32_add_clamp(Record·a·b { a, b }: Record·a·b<U32, U32>) -> U32 {
    a.saturating_add(b)
}
pub fn u32_add_carry(
    Record·a·b·carry { a, b, carry }: Record·a·b·carry<
        U32,
        U32,
        Choice·Contained·Overflowed<Record, Record>,
    >,
) -> Record·carry·wrapped<Choice·Contained·Overflowed<Record, Record>, U32> {
    let (sum, carry) = a.carrying_add(
        b,
        match carry {
            Choice·Contained·Overflowed::Overflowed(()) => true,
            Choice·Contained·Overflowed::Contained(()) => false,
        },
    );
    Record·carry·wrapped {
        carry: if carry {
            Choice·Contained·Overflowed::Overflowed(())
        } else {
            Choice·Contained·Overflowed::Contained(())
        },
        wrapped: sum,
    }
}
pub fn i32_dup(n: I32) -> Record·a·b<I32, I32> {
    Record·a·b { a: n, b: n }
}
pub fn i32_rid(_: I32) -> Record {
    ()
}
#[expect(clippy::cast_precision_loss)]
pub fn i32_to_f32(n: I32) -> F32 {
    n as F32
}
pub fn i32_to_u32(n: I32) -> Opt<U32> {
    match <U32 as std::convert::TryFrom<I32>>::try_from(n) {
        std::result::Result::Err(_) => Opt::Absent(()),
        std::result::Result::Ok(u) => Opt::Present(u),
    }
}
pub fn i32_to_p32(n: I32) -> Opt<P32> {
    match <U32 as std::convert::TryFrom<I32>>::try_from(n) {
        std::result::Result::Err(_) => Opt::Absent(()),
        std::result::Result::Ok(u) => u32_to_p32(u),
    }
}
pub fn i32_abs_u32(n: I32) -> U32 {
    n.unsigned_abs()
}
pub fn i32_negate(n: I32) -> I32 {
    -n
}
pub fn i32_add_clamp(Record·a·b { a, b }: Record·a·b<I32, I32>) -> I32 {
    a.saturating_add(b)
}
pub fn i32_add_carry(
    Record·a·b { a, b }: Record·a·b<I32, I32>,
) -> Record·carry·wrapped<Choice·Contained·Overflowed<Record, Record>, I32> {
    let (sum, carry) = a.overflowing_add(b);
    Record·carry·wrapped {
        carry: if carry {
            Choice·Contained·Overflowed::Overflowed(())
        } else {
            Choice·Contained·Overflowed::Contained(())
        },
        wrapped: sum,
    }
}
pub fn i32_mul_clamp(Record·a·b { a, b }: Record·a·b<I32, I32>) -> I32 {
    a.saturating_mul(b)
}
pub fn f32_dup(n: F32) -> Record·a·b<F32, F32> {
    Record·a·b { a: n, b: n }
}
pub fn f32_rid(_: F32) -> Record {
    ()
}
pub fn f32_add_clamp(Record·a·b { a, b }: Record·a·b<F32, F32>) -> F32 {
    (a + b).clamp(f32::MIN, f32::MAX)
}
pub fn f32_mul_clamp(Record·a·b { a, b }: Record·a·b<F32, F32>) -> F32 {
    (a * b).clamp(f32::MIN, f32::MAX)
}
pub fn f32_div_clamp(Record·by·n { n, by }: Record·by·n<F32, F32>) -> F32 {
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
pub fn f32_round(Record·mode·n { mode, n }: Record·mode·n<Round_mode, F32>) -> F32 {
    match mode {
        Round_mode::Up(()) => n.ceil(),
        Round_mode::Down(()) => n.floor(),
        Round_mode::Away_from_0(()) => {
            // I'm not convinced this is the fastest but since this is by far the
            // most common implementation I've seen I'm hoping this gets optimized at least
            n.abs().ceil() * n.signum()
        }
        Round_mode::Toward_0(()) => n.trunc(),
        Round_mode::Nearest_else_away_from_0(()) => n.round(),
        Round_mode::Nearest_else_even(()) => n.round_ties_even(),
    }
}
pub fn f32_to_i32_clamp(operation: Record·mode·n<Round_mode, F32>) -> I32 {
    f32_round(operation) as I32
}

pub fn fn_dup<In, Out>(fn_: Fn<In, Out>) -> Record·a·b<Fn<In, Out>, Fn<In, Out>> {
    Record·a·b { a: fn_, b: fn_ }
}
pub fn fn_rid<In, Out>(_: Fn<In, Out>) -> Record {
    ()
}

pub fn char_dup(char: Char) -> Record·a·b<Char, Char> {
    Record·a·b { a: char, b: char }
}
pub fn char_rid(_: Char) -> Record {}
pub fn char_to_code_point(char: Char) -> U32 {
    <u32 as std::convert::From<char>>::from(char)
}
pub fn u32_code_point_to_char(code_point: U32) -> Opt<Char> {
    Opt::from_option(char::from_u32(code_point))
}

pub fn str_dup(str: Str) -> Record·a·b<Str, Str> {
    Record·a·b { a: str, b: str }
}
pub fn str_rid(_: Str) -> Record {}
pub fn str_byte_count(str: Str) -> u32 {
    str.len() as u32
}
pub fn str_char_count(str: Str) -> u32 {
    std::iter::Iterator::count(str.chars()) as u32
}
pub fn str_start(str: Str) -> Opt<Record·after·start<Str, Char>> {
    let mut chars = str.chars();
    Opt::from_option(
        std::iter::Iterator::next(&mut chars).map(|c| Record·after·start {
            start: c,
            after: chars.as_str(),
        }),
    )
}
pub fn str_end(str: Str) -> Opt<Record·before·end<Str, Char>> {
    let mut chars = str.chars();
    Opt::from_option(
        std::iter::Iterator::next(&mut std::iter::Iterator::rev(&mut chars)).map(|c| {
            Record·before·end {
                end: c,
                before: chars.as_str(),
            }
        }),
    )
}
pub fn str_chars_fold<State>(
    Record·direction·state·step·str {
        direction,
        str,
        state: initial_state,
        step,
    }: Record·direction·state·step·str<
        Choice·Down·Up<Record, Record>,
        State,
        Fn<Record·char·state<Char, State>, State>,
        Str,
    >,
) -> State {
    iterator_fold_in_direction(str.chars(), direction, initial_state, |state, char| {
        step(Record·char·state {
            state: state,
            char: char,
        })
    })
}
fn iterator_fold_in_direction<Element, State>(
    mut iterator: impl std::iter::DoubleEndedIterator<Item = Element>,
    direction: Choice·Down·Up<Record, Record>,
    state: State,
    step: impl std::ops::Fn(State, Element) -> State,
) -> State {
    match direction {
        Choice·Down·Up::Up(()) => std::iter::Iterator::fold(&mut iterator, state, step),
        Choice·Down·Up::Down(()) => {
            std::iter::Iterator::fold(&mut std::iter::Iterator::rev(iterator), state, step)
        }
    }
}
fn iterator_try_fold_in_direction<Element, B, C>(
    mut iterator: impl std::iter::DoubleEndedIterator<Item = Element>,
    direction: Choice·Down·Up<Record, Record>,
    state: C,
    step: impl std::ops::Fn(C, Element) -> std::ops::ControlFlow<B, C>,
) -> std::ops::ControlFlow<B, C> {
    match direction {
        Choice·Down·Up::Up(()) => std::iter::Iterator::try_fold(&mut iterator, state, step),
        Choice·Down·Up::Down(()) => {
            std::iter::Iterator::try_fold(&mut std::iter::Iterator::rev(iterator), state, step)
        }
    }
}
pub fn str_chars_fold_while<Exit, GoOn>(
    Record·direction·state·step·str {
        direction,
        str,
        state: initial_state,
        step,
    }: Record·direction·state·step·str<
        Choice·Down·Up<Record, Record>,
        GoOn,
        Fn<Record·char·state<Char, GoOn>, Choice·Exit·Go_on<Exit, GoOn>>,
        Str,
    >,
) -> Choice·Exit·Go_on<Exit, GoOn> {
    Choice·Exit·Go_on::from_control_flow(iterator_try_fold_in_direction(
        str.chars(),
        direction,
        initial_state,
        |state, char| {
            Choice·Exit·Go_on::into_control_flow(step(Record·char·state { state, char }))
        },
    ))
}

pub fn opt_present<Present>(present: Present) -> Opt<Present> {
    Opt::Present(present)
}

pub fn slot_index<Origin>(slot: Slot<Origin>) -> Record·index·slot<u32, Slot<Origin>> {
    Record·index·slot {
        index: slot.index,
        slot: slot,
    }
}
pub fn slot_to_span<Origin>(slot: Slot<Origin>) -> Span<Origin> {
    slot.to_span()
}

pub fn empty_slot_to_span<Origin>(slot: Empty_slot<Origin>) -> Empty_span<Origin> {
    slot.to_span()
}
pub fn empty_slot_index<Origin>(
    slot: Empty_slot<Origin>,
) -> Record·index·slot<u32, Empty_slot<Origin>> {
    Record·index·slot {
        index: slot.index,
        slot: slot,
    }
}

pub fn span_start<Origin>(
    span: Span<Origin>,
) -> Record·after·start<Opt<Span<Origin>>, Slot<Origin>> {
    span.split_start()
}
pub fn span_end<Origin>(
    span: Span<Origin>,
) -> Record·before·end<Opt<Span<Origin>>, Slot<Origin>> {
    span.split_end()
}
pub fn opt_span_length<Origin>(
    span: Opt<Span<Origin>>,
) -> Record·length·span<u32, Opt<Span<Origin>>> {
    Record·length·span {
        length: span.as_ref().length(),
        span: span,
    }
}
pub fn opt_span_take_start<Origin>(
    Record·length·span {
        length: length_to_take,
        span,
    }: Record·length·span<U32, Opt<Span<Origin>>>,
) -> Record·after·start<Opt<Span<Origin>>, Opt<Span<Origin>>> {
    match std::num::NonZeroU32::new(length_to_take) {
        std::option::Option::None => Record·after·start {
            start: Opt::Absent(()),
            after: span,
        },
        std::option::Option::Some(positive_length_to_take) => match span {
            Opt::Absent(()) => Record·after·start {
                start: Opt::Absent(()),
                after: Opt::Absent(()),
            },
            Opt::Present(span) => {
                let Record·after·start { start, after } =
                    span.split_start_positive(positive_length_to_take);
                Record·after·start {
                    start: Opt::Present(start),
                    after: after,
                }
            }
        },
    }
}
pub fn span_take_start_positive<Origin>(
    Record·length·span {
        length: start_length,
        span,
    }: Record·length·span<P32, Span<Origin>>,
) -> Record·after·start<Opt<Span<Origin>>, Span<Origin>> {
    span.split_start_positive(start_length)
}
pub fn opt_span_fold<Origin, State>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Opt<Span<Origin>>,
        State,
        Fn<Record·slot·state<Slot<Origin>, State>, State>,
    >,
) -> State {
    iterator_fold_in_direction(
        span.as_ref().to_range_u32(),
        direction,
        initial_state,
        |state, index| {
            step(Record·slot·state {
                state,
                slot: Slot::<Origin>::from_index(index),
            })
        },
    )
}
pub fn opt_span_fold_while<Exit, GoOn, Origin>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Opt<Span<Origin>>,
        GoOn,
        Fn<Record·slot·state<Slot<Origin>, GoOn>, Choice·Exit·Go_on<Exit, GoOn>>,
    >,
) -> Choice·Exit·Go_on<Record·exit·remaining<Exit, Opt<Span<Origin>>>, GoOn> {
    let state_after_fold = iterator_try_fold_in_direction(
        span.as_ref().to_range_u32(),
        direction,
        initial_state,
        |state, index| {
            Choice·Exit·Go_on::into_control_flow(step(Record·slot·state {
                state: state,
                slot: Slot::<Origin>::from_index(index),
            }))
            .map_break(|exit| (index, exit))
        },
    );
    match state_after_fold {
        std::ops::ControlFlow::Continue(state) => Choice·Exit·Go_on::Go_on(state),
        std::ops::ControlFlow::Break((exit_index, exit_state)) => {
            let Record·after·start {
                start: _,
                after: not_folded_over_opt_span,
            } = opt_span_take_start(Record·length·span {
                span: span,
                length: exit_index + 1,
            });
            Choice·Exit·Go_on::Exit(Record·exit·remaining {
                exit: exit_state,
                remaining: not_folded_over_opt_span,
            })
        }
    }
}
pub fn span_connect_slot<Origin>(
    span: Span<Origin>,
    slot_to_add: Slot<Origin>,
) -> Record·apart·connected<Opt<Slot<Origin>>, Span<Origin>> {
    span.connect_slot(slot_to_add)
}
pub fn span_connect<Origin>(
    Record·new·span {
        span,
        new: span_to_add,
    }: Record·new·span<Span<Origin>, Span<Origin>>,
) -> Record·apart·connected<Opt<Span<Origin>>, Span<Origin>> {
    span.connect(span_to_add)
}

pub fn empty_span_start<Origin>(
    span: Empty_span<Origin>,
) -> Record·after·start<Opt<Empty_span<Origin>>, Empty_slot<Origin>> {
    span.split_start()
}
pub fn empty_span_end<Origin>(
    span: Empty_span<Origin>,
) -> Record·before·end<Opt<Empty_span<Origin>>, Empty_slot<Origin>> {
    span.split_end()
}
pub fn opt_empty_span_length<Origin>(
    span: Opt<Empty_span<Origin>>,
) -> Record·length·span<u32, Opt<Empty_span<Origin>>> {
    Record·length·span {
        length: span.as_ref().length(),
        span: span,
    }
}
pub fn opt_empty_span_fold<Origin, State>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Opt<Empty_span<Origin>>,
        State,
        Fn<Record·slot·state<Empty_slot<Origin>, State>, State>,
    >,
) -> State {
    iterator_fold_in_direction(
        span.as_ref().to_range_u32(),
        direction,
        initial_state,
        |state, index| {
            step(Record·slot·state {
                state,
                slot: Empty_slot::<Origin>::from_index(index),
            })
        },
    )
}

pub fn origin_rid<LocalOrigin>(_: Origin<LocalOrigin>) -> Record {
    ()
}

pub fn vec_empty<LocalOrigin, Element>(_: Origin<LocalOrigin>) -> Vec<LocalOrigin, Element> {
    Vec::<LocalOrigin, Element> {
        elements: std::vec::Vec::new(),
        vacant: std::vec::Vec::new(),
    }
}
pub fn vec_pre_allocate_at_least<Origin, Element>(
    Record·length·vec {
        vec: mut vec,
        length: min_pre_allocated_length,
    }: Record·length·vec<u32, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.pre_allocate_at_least(min_pre_allocated_length);
    vec
}
pub fn vec_take<Origin, Element>(
    Record·slot·vec { mut vec, slot }: Record·slot·vec<Slot<Origin>, Vec<Origin, Element>>,
) -> Record·element·vec<Element, Vec<Origin, Element>> {
    let element = vec.take(slot);
    Record·element·vec {
        element: element,
        vec: vec,
    }
}
pub fn vec_element<Origin, Element>(
    Record·slot·vec { mut vec, slot }: Record·slot·vec<Slot<Origin>, Vec<Origin, Element>>,
) -> Record·element·slot·vec<Element, Empty_slot<Origin>, Vec<Origin, Element>> {
    let element = vec.element(slot);
    Record·element·slot·vec {
        element: element.element,
        slot: element.slot,
        vec: vec,
    }
}
pub fn vec_set<Origin, Element>(
    Record·new·slot·vec {
        mut vec,
        slot,
        new: element,
    }: Record·new·slot·vec<Element, Empty_slot<Origin>, Vec<Origin, Element>>,
) -> Record·slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let empty_slot = vec.set(slot, element);
    Record·slot·vec {
        vec: vec,
        slot: empty_slot,
    }
}
pub fn vec_slot_rid<Origin, Element>(
    Record·slot·vec {
        slot: slot_to_vacate,
        mut vec,
    }: Record·slot·vec<Empty_slot<Origin>, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.slot_rid(slot_to_vacate);
    vec
}
pub fn vec_span_rid<Origin, Element>(
    Record·span·vec {
        span: span_to_vacate,
        mut vec,
    }: Record·span·vec<Empty_span<Origin>, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.span_rid(span_to_vacate);
    vec
}
pub fn vec_rid<Origin, Element>(_: Vec<Origin, Element>) -> Record {
    ()
}
pub fn vec_add<Origin, Element>(
    Record·new·vec {
        mut vec,
        new: new_element,
    }: Record·new·vec<Element, Vec<Origin, Element>>,
) -> Record·slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add(new_element);
    Record·slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add_ignoring_vacant<Origin, Element>(
    Record·new·vec {
        mut vec,
        new: new_element,
    }: Record·new·vec<Element, Vec<Origin, Element>>,
) -> Record·slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add_ignoring_vacant(new_element);
    Record·slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add_empty<Origin, Element>(
    mut vec: Vec<Origin, Element>,
) -> Record·slot·vec<Empty_slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add_empty();
    Record·slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add_empty_ignoring_vacant<Origin, Element>(
    mut vec: Vec<Origin, Element>,
) -> Record·slot·vec<Empty_slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add_empty_ignoring_vacant();
    Record·slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add_take_vec_span<Origin, ShrinkOrigin, Element>(
    mut grow: Vec<Origin, Element>,
    mut shrink: Vec<ShrinkOrigin, Element>,
    shrink_span: Span<ShrinkOrigin>,
) -> Record·grown·shrunk·span<Vec<Origin, Element>, Vec<ShrinkOrigin, Element>, Span<Origin>> {
    let grow_span = grow.add_vec_span(&mut shrink, shrink_span);
    Record·grown·shrunk·span {
        grown: grow,
        shrunk: shrink,
        span: grow_span,
    }
}
pub fn vec_add_str<Origin>(
    Record·new·vec {
        mut vec,
        new: new_str,
    }: Record·new·vec<Str, Vec<Origin, Char>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Char>> {
    let grow_span = vec.add_str(new_str);
    Record·span·vec {
        vec: vec,
        span: grow_span,
    }
}
pub fn vec_replace<Origin, Element>(
    Record·new·slot·vec {
        mut vec,
        mut slot,
        new: new_element,
    }: Record·new·slot·vec<Element, Slot<Origin>, Vec<Origin, Element>>,
) -> Record·old·slot·vec<Element, Slot<Origin>, Vec<Origin, Element>> {
    let old_element = std::mem::replace(vec.element_mut(&mut slot), new_element);
    Record·old·slot·vec {
        vec: vec,
        old: old_element,
        slot: slot,
    }
}
pub fn vec_opt_span_reverse<Origin, Element>(
    Record·span·vec { mut vec, mut span }: Record·span·vec<
        Opt<Span<Origin>>,
        Vec<Origin, Element>,
    >,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    vec.opt_span_slice_mut(&mut span).reverse();
    Record·span·vec { vec: vec, span }
}
pub fn vec_span_reverse<Origin, Element>(
    Record·span·vec { mut vec, mut span }: Record·span·vec<Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    vec.span_slice_mut(&mut span).reverse();
    Record·span·vec { vec: vec, span }
}

pub fn vec_opt_span_add<Origin, Element>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_element,
    }: Record·new·span·vec<Element, Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    match span {
        Choice·Absent·Present::Absent(()) => {
            let new_slot = vec.add(new_element);
            Record·span·vec {
                vec: vec,
                span: slot_to_span(new_slot),
            }
        }
        Choice·Absent·Present::Present(span) => vec_span_add(Record·new·span·vec {
            vec: vec,
            span: span,
            new: new_element,
        }),
    }
}
pub fn vec_span_add<Origin, Element>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_element,
    }: Record·new·span·vec<Element, Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let grown_span = vec.span_add(span, new_element);
    Record·span·vec {
        vec: vec,
        span: grown_span,
    }
}
pub fn vec_opt_span_add_str<Origin>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_str,
    }: Record·new·span·vec<Str, Opt<Span<Origin>>, Vec<Origin, Char>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Char>> {
    let grown_span = vec.opt_span_add_str(span, new_str);
    Record·span·vec {
        vec: vec,
        span: grown_span,
    }
}
pub fn vec_span_add_str<Origin>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_str,
    }: Record·new·span·vec<Str, Span<Origin>, Vec<Origin, Char>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    let grown_span = vec.span_add_str(span, new_str);
    Record·span·vec {
        vec: vec,
        span: grown_span,
    }
}
pub fn vec_opt_span_add_take_vec_opt_span<GrowOrigin, ShrinkOrigin, Element>(
    Record·new·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: Record·new·shrink·span·vec<
        Opt<Span<ShrinkOrigin>>,
        Vec<ShrinkOrigin, Element>,
        Opt<Span<GrowOrigin>>,
        Vec<GrowOrigin, Element>,
    >,
) -> Record·grown·shrunk·span<
    Vec<GrowOrigin, Element>,
    Vec<ShrinkOrigin, Element>,
    Opt<Span<GrowOrigin>>,
> {
    let maybe_grown_span = match shrink_span {
        Choice·Absent·Present::Absent(()) => span,
        Choice·Absent·Present::Present(shrink_span) => {
            Opt::Present(vec.opt_span_add_take_vec_span(span, &mut shrink_vec, shrink_span))
        }
    };
    Record·grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: maybe_grown_span,
    }
}
pub fn vec_span_add_take_vec_opt_span<GrowOrigin, ShrinkOrigin, Element>(
    Record·new·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: Record·new·shrink·span·vec<
        Opt<Span<ShrinkOrigin>>,
        Vec<ShrinkOrigin, Element>,
        Span<GrowOrigin>,
        Vec<GrowOrigin, Element>,
    >,
) -> Record·grown·shrunk·span<
    Vec<GrowOrigin, Element>,
    Vec<ShrinkOrigin, Element>,
    Span<GrowOrigin>,
> {
    let maybe_grown_span = match shrink_span {
        Choice·Absent·Present::Absent(()) => span,
        Choice·Absent·Present::Present(shrink_span) => {
            vec.span_add_take_vec_span(span, &mut shrink_vec, shrink_span)
        }
    };
    Record·grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: maybe_grown_span,
    }
}
pub fn vec_opt_span_add_take_vec_span<GrowOrigin, ShrinkOrigin, Element>(
    Record·new·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: Record·new·shrink·span·vec<
        Span<ShrinkOrigin>,
        Vec<ShrinkOrigin, Element>,
        Opt<Span<GrowOrigin>>,
        Vec<GrowOrigin, Element>,
    >,
) -> Record·grown·shrunk·span<
    Vec<GrowOrigin, Element>,
    Vec<ShrinkOrigin, Element>,
    Span<GrowOrigin>,
> {
    let grown_span = vec.opt_span_add_take_vec_span(span, &mut shrink_vec, shrink_span);
    Record·grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: grown_span,
    }
}
pub fn vec_span_add_take_vec_span<GrowOrigin, ShrinkOrigin, Element>(
    Record·new·shrink·span·vec {
        mut vec,
        span,
        shrink: mut shrink_vec,
        new: shrink_span,
    }: Record·new·shrink·span·vec<
        Span<ShrinkOrigin>,
        Vec<ShrinkOrigin, Element>,
        Span<GrowOrigin>,
        Vec<GrowOrigin, Element>,
    >,
) -> Record·grown·shrunk·span<
    Vec<GrowOrigin, Element>,
    Vec<ShrinkOrigin, Element>,
    Span<GrowOrigin>,
> {
    let grown_span = vec.span_add_take_vec_span(span, &mut shrink_vec, shrink_span);
    Record·grown·shrunk·span {
        grown: vec,
        shrunk: shrink_vec,
        span: grown_span,
    }
}

pub fn vec_move_opt_span_to_vacant<Origin, Element>(
    Record·span·vec { span, mut vec }: Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    match span {
        Opt::Absent(()) => Record·span·vec {
            span: Opt::Absent(()),
            vec: vec,
        },
        Opt::Present(span) => {
            let moved_span = vec.move_span_to_vacant(span);
            Record·span·vec {
                span: Opt::Present(moved_span),
                vec: vec,
            }
        }
    }
}
pub fn vec_move_span_to_vacant<Origin, Element>(
    Record·span·vec { span, mut vec }: Record·span·vec<Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let moved_span = vec.move_span_to_vacant(span);
    Record·span·vec {
        span: moved_span,
        vec: vec,
    }
}
