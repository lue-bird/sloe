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
pub struct Record·length·slice<Length, Slice> {
    pub length: Length,
    pub slice: Slice,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·origin·slice<Origin, Slice> {
    pub origin: Origin,
    pub slice: Slice,
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
pub struct Record·source·source_span·span·vec<Source, Source_span, Span, Vec> {
    pub source: Source,
    pub source_span: Source_span,
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
pub struct Record·end·start·vec<End, Start, Vec> {
    pub end: End,
    pub start: Start,
    pub vec: Vec,
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

#[derive(Debug)]
pub struct Origin<LocalOrigin>(LocalOrigin);
pub struct Unset_slice<Element>(std::boxed::Box<[std::mem::MaybeUninit<Element>]>);
#[derive(Debug)]
pub struct Vec<LocalOrigin, Element> {
    // invariants (in addition to the invariants of (Unset_)slot/span):
    // - no `Unset_span`s in `.vacant` are connected
    //   (and thus could be combined into one larger consecutive span)
    // - any index contained in any vacant `Unset_span` is less than elements.len()
    // - any index contained in any vacant `Unset_span` should be assumed uninitialized
    //   in `.elements`
    //
    // -------
    // `.elements` contains `std::mem::MaybeUninit<Element>` because
    // - functions like `vec.add_unset` explicitly require uninitialized memory.
    //   creating uninitialized memory of type `Element` out of thin air is UB
    // - it matches well semantically: access is inherently unsafe.
    //   vec::Vec<Element> makes it appear safe
    // - drawbacks (like the removal of niches) do not have an impact here
    // - it prevents drop from being called on elements
    //   which could double-free on already vacated elements.
    //   Vec<_,_> originally implemented a custom Drop as
    //   `for e in self.elements.drain(..) { std::mem::forget(e); }`
    //   with the following documentation:
    //     At this point, all elements are either
    //     - handled (in sloe code this is always the case or you'll get an error)
    //     - unhandled (only possible from rust code when a `Slot`/`Span` is dropped)
    //     - empty (only possible from rust code when a `Unset_span`/`Unset_span` is dropped)
    //     - occupied (only possible from rust code).
    //
    //     If we used the regular Drop implementation, elements that were already vacated
    //     or temporarily extracted (where e.g. the resulting `Unset_slot` from `vec.element()` was dropped)
    //     could be freed twice (!).
    //     So the only thing that can realistically be done is to "leak" all remaining elements.
    //
    //     To recap, if some rust code kept some slots occupied,
    //     we _must_ prevent double-frees by leaking those elements.
    //     This is not as bad as you might think:
    //     - dropping a `Slot`/`Unset_slot` is always a leak
    //       but it cannot reasonably prevented in rust. It's the cost of doing business
    //     - in a `Vec<Origin, Element>`, the element type will realistically not be a type that
    //       directly points to the heap. In fact in sloe you cannot even put more than one vec inside of
    //       another vec as each vec has a different origin!
    //
    //   However, just overwriting the Drop implementation is far from enough
    //   as many Vec functions somewhat willy-nilly drop elements if you're not careful.
    //   An example is `truncate` which is used in `span_rid`.
    elements: std::vec::Vec<std::mem::MaybeUninit<Element>>,
    // Performance assumption:
    // Neighboring elements are way more likely to be vacated together.
    // Think e.g. vec_span_add_vec_span but also
    // regular chunks of nested individual slots which were likely allocated close to their neighbors.
    //
    // It is also assumed that there won't be a large amount of these vacant spans
    // so e.g. HashSet loses despite having a faster "find out if this index is vacant".
    // If usage ends up suggesting otherwise, we should change accordingly
    vacant: std::vec::Vec<Unset_span<LocalOrigin>>,
}
pub type Slot<LocalOrigin> = Slot_with_occupancy<LocalOrigin, OccupancySet>;
pub type Unset_slot<LocalOrigin> = Slot_with_occupancy<LocalOrigin, UccupancyUnset>;
#[non_exhaustive]
pub struct Slot_with_occupancy<LocalOrigin, Occupancy> {
    pub origin: std::marker::PhantomData<LocalOrigin>,
    pub occupancy: std::marker::PhantomData<Occupancy>,
    // consider switching to NonZeroU32 to create a niche for use with Option<Slot<>>
    pub index: u32,
}
pub type Span<LocalOrigin> = Span_with_occupancy<LocalOrigin, OccupancySet>;
pub type Unset_span<LocalOrigin> = Span_with_occupancy<LocalOrigin, UccupancyUnset>;
#[non_exhaustive]
pub struct Span_with_occupancy<LocalOrigin, Occupancy> {
    pub start: Slot_with_occupancy<LocalOrigin, Occupancy>,
    // consider instead: end_index: NonZeroU32.
    // This makes combining 2 opt_spans and converting to ops::Range a bit faster,
    // at the cost of other operations like checking a vec's occupied count
    pub length: std::num::NonZeroU32,
}
pub enum UccupancyUnset {}
pub enum OccupancySet {}

pub struct Array<Element, Record> {
    pub record: Record,
    // It would be great if we could find a _safe_ way to as directly as possible iterate the array.
    // Various helpers like getting the size and dup-ing would aso be nice
    // but are to be avoided if they come at a memory cost.
    // The problem is that
    // - providing fold is impossible because for<State> fn is not allowed
    // - providing for_each is impossible because fn(impl FnMut) is not allowed
    // - there is no such thing as an "owned stack-allocated dynamic-size slice" in rust
    //
    // A solution would be using
    // pub as_slice: fn(&mut Record) -> &mut [Element]
    // but this relies (!) on both
    //   - callers to use unsafe to extract owned elements
    //   - array creation to use unsafe (and rely on field order despite not using repr(C))
    //
    // Another "solution" would be to just give up and use `Box<[Element]>`
    // or add a fn that returns Box<dyn Iterator> or similar
    // and hope the optimizer converts heap into stack alloction. (naw man, that ain't it)
    //
    // We could even use somthing like SmallVec as e.g.
    // `size: P32, on_stack: [Element;8], remaining: Option<Box<[Element]>>`
    // quite thick, and probably no faster than full-on heap :(
    // (using this when we actually know the exact size also feels bad)
    //
    // So for now, use-cases are "embarassingly hardcoded".
    // This solution is restrictive and thus very unsatisfying but at least it works.
    // Maybe there are nicer hardcoded primitives, though
    // (e.g. writing into a given &mut [MaybeUninit]?,
    // or fn at(index: u32, &mut Record) -> Option<&mut Element>
    // which requires unsafe at call-site for ownership and is probably slower?)
    // Help!
    //
    // Why this weird function signature?
    // To enable operations like Vec::add_array to return a Span instead of an Opt<Span>.
    // We could panic or uncheck that case but then buggy Array instances could blow everything up.
    // Originally I split .record into .before and .last but it felt confusing
    // in sloe code that the specified record had 1 field less than actual elements
    pub split_last_and_extend_vec_with_before:
        fn(&mut std::vec::Vec<std::mem::MaybeUninit<Element>>, Record) -> Element,
}

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
impl<Element> std::fmt::Debug for Unset_slice<Element> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Unset_slice")
            .field("length", &self.0.len())
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
impl<'a, Element> std::iter::DoubleEndedIterator for OwnedSliceIterator<'a, Element> {
    fn next_back(&mut self) -> std::option::Option<Self::Item> {
        // usage is safe when constructor is safe, see mut_slice_into_owned_iterator
        self.ref_mut_iterator.next_back().map(|element_ref| unsafe {
            std::ptr::NonNull::read(std::ptr::NonNull::from_ref(element_ref))
        })
    }
}

impl<LocalOrigin> Origin<LocalOrigin> {
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

impl<Element> Unset_slice<Element> {
    pub fn allocate_length(length: u32) -> Self {
        Unset_slice(std::boxed::Box::new_uninit_slice(length as usize))
    }
    pub fn from_vec_maybe_uninit(
        mut maybe_uninit_vec: std::vec::Vec<std::mem::MaybeUninit<Element>>,
    ) -> Self {
        // This is the closest approximation for `vec.ptr[..vec.capacity]` I could find in safe rust.
        // The first part should optimize to maybe_uninit_vec.set_len(maybe_uninit_vec.capacity())
        // If it doesn't, change to that unsafe operation.
        // Preferably there would be something like `vec.clear(); vec.into_spare_capacity()`
        let spare_capacity = maybe_uninit_vec.spare_capacity_mut().len();
        std::iter::Extend::extend(
            &mut maybe_uninit_vec,
            std::iter::Iterator::take(
                std::iter::repeat_with(|| std::mem::MaybeUninit::uninit()),
                spare_capacity,
            ),
        );
        Unset_slice(maybe_uninit_vec.into_boxed_slice())
    }
    pub fn as_slice(&self) -> &[std::mem::MaybeUninit<Element>] {
        &self.0
    }
    pub fn length_usize(&self) -> usize {
        self.as_slice().len()
    }
    pub fn length(&self) -> u32 {
        self.as_slice().len() as u32
    }
    pub fn cast_or_rid_and_allocate<NewElement>(self) -> Unset_slice<NewElement> {
        const fn mem_stride_of<Element>() -> usize {
            // at the time of writing, this is the same as size
            // is there a nicer way?
            std::mem::size_of::<Element>()
        }
        // safe alternative
        // ```rust
        // self.into_boxed_slice().into_iter().collect().into_boxed_slice()
        // ```
        // which should automatically reuse the memory if layouts are equal (in release mode)
        if const {
            mem_stride_of::<NewElement>() == mem_stride_of::<Element>()
                && std::mem::align_of::<NewElement>() == std::mem::align_of::<Element>()
        } {
            // safe because all contained memory is uninitialized
            Unset_slice(unsafe {
                std::boxed::Box::from_raw(std::boxed::Box::into_raw(self.into_boxed_slice())
                    as *mut [std::mem::MaybeUninit<NewElement>])
            })
        } else {
            Unset_slice::<NewElement>::allocate_length(self.length())
        }
    }
    pub fn into_boxed_slice(self) -> std::boxed::Box<[std::mem::MaybeUninit<Element>]> {
        self.0
    }
    pub fn into_vec(self) -> std::vec::Vec<Element> {
        let mut vec: std::vec::Vec<std::mem::MaybeUninit<Element>> =
            self.into_boxed_slice().into_vec();
        vec.clear();
        // only safe because there are no more safely accessible items in the Vec anymore
        // and the spare_capacity is assumed to never be accessed via assume_init.
        // IMO there should be a safe operation in std::vec::Vec for this.
        //
        // Safe alternative:
        // ```rust
        // vec.into_iter().map(|impossible| impossible.assume_init()).collect()
        // // or
        // vec.into_iter().map(|_| unsafe { std::hint::unreachable_unchecked() }).collect()
        // ```
        // combined with asserting equal size to reuse memory (in release mode)
        let (vec_ptr, vec_length, vec_capacity) = vec.into_raw_parts();
        unsafe {
            std::vec::Vec::from_raw_parts(vec_ptr.cast::<Element>(), vec_length, vec_capacity)
        }
    }
    pub fn into_vec_maybe_uninit(self) -> std::vec::Vec<std::mem::MaybeUninit<Element>> {
        let mut vec: std::vec::Vec<std::mem::MaybeUninit<Element>> =
            self.into_boxed_slice().into_vec();
        vec.clear();
        vec
    }
    pub fn leak<'a>(self) -> &'a mut [std::mem::MaybeUninit<Element>] {
        std::boxed::Box::leak(self.into_boxed_slice())
    }
}

impl<LocalOrigin, Element> Vec<LocalOrigin, Element> {
    pub fn new(_: Origin<LocalOrigin>) -> Self {
        Vec::<LocalOrigin, Element> {
            elements: std::vec::Vec::new(),
            vacant: std::vec::Vec::new(),
        }
    }
    pub fn reuse(_: Origin<LocalOrigin>, allocation: Unset_slice<Element>) -> Self {
        Vec::<LocalOrigin, Element> {
            elements: allocation.into_vec_maybe_uninit(),
            vacant: std::vec::Vec::new(),
        }
    }
    /// Especially when working with estimates or future insertions, you usually want pre_allocate_at_least
    pub fn pre_allocate(&mut self, pre_allocated_length: u32) {
        self.elements.reserve_exact(pre_allocated_length as usize);
    }
    pub fn pre_allocate_at_least_usize(&mut self, min_pre_allocated_length: usize) {
        self.elements.reserve(min_pre_allocated_length);
    }
    pub fn pre_allocate_at_least(&mut self, min_pre_allocated_length: u32) {
        self.pre_allocate_at_least_usize(min_pre_allocated_length as usize);
    }
    pub fn pre_allocation_rid(&mut self) {
        self.elements.shrink_to_fit();
    }
    pub fn element<'a>(&'a self, slot: &'a Slot<LocalOrigin>) -> &'a Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe {
            self.elements
                .get_unchecked(slot.index as usize)
                .assume_init_ref()
        }
    }
    pub fn element_mut<'a>(&'a mut self, slot: &'a mut Slot<LocalOrigin>) -> &'a mut Element {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid index
        unsafe {
            self.elements
                .get_unchecked_mut(slot.index as usize)
                .assume_init_mut()
        }
    }
    pub fn opt_span_slice<'a>(&'a self, opt_span: Opt<&'a Span<LocalOrigin>>) -> &'a [Element] {
        match opt_span {
            Opt::Absent(()) => &[],
            Opt::Present(span) => self.span_slice(span),
        }
    }
    pub fn span_slice<'a>(&'a self, span: &'a Span<LocalOrigin>) -> &'a [Element] {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid range
        unsafe {
            self.elements
                .get_unchecked(span.to_range())
                .assume_init_ref()
        }
    }
    pub fn opt_span_slice_mut<'a>(
        &'a mut self,
        opt_span: &'a mut Opt<Span<LocalOrigin>>,
    ) -> &'a mut [Element] {
        match opt_span {
            Opt::Absent(()) => &mut [],
            Opt::Present(span) => self.span_slice_mut(span),
        }
    }
    pub fn span_slice_mut<'a>(&'a mut self, span: &'a mut Span<LocalOrigin>) -> &'a mut [Element] {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid range
        unsafe { self.maybe_uninit_span_slice_mut(span).assume_init_mut() }
    }
    pub fn maybe_uninit_span_slice_mut<'a, Occupancy>(
        &'a mut self,
        span: &'a mut Span_with_occupancy<LocalOrigin, Occupancy>,
    ) -> &'a mut [std::mem::MaybeUninit<Element>] {
        // the .elements are never shortened and new slots are bound to this collection origin and contain a known valid range
        unsafe { self.elements.get_unchecked_mut(span.to_range()) }
    }
    pub fn span_into_iterator<'a>(
        &'a mut self,
        span: Span<LocalOrigin>,
    ) -> OwnedSliceIterator<'a, Element> {
        // elements in the opt_span are consumed and never accessed after. During this whole ordeal
        // the elements are "locked" behind a mut ref with the same lifetime as the iterator
        unsafe {
            mut_slice_into_owned_iterator(
                self.elements
                    .get_unchecked_mut(span.to_range())
                    .assume_init_mut(),
            )
        }
    }
    pub fn remove(&mut self, slot: Slot<LocalOrigin>) -> Element {
        // vacated opt_span elements are never accessed, not even while vacating them
        let element = self.unset(slot);
        self.slot_rid(element.slot);
        element.element
    }
    pub fn unset(
        &mut self,
        mut slot: Slot<LocalOrigin>,
    ) -> Record·element·slot<Element, Unset_slot<LocalOrigin>> {
        // its unique slot is consumed, so this element cannot be accessed after
        let element = unsafe {
            std::ptr::NonNull::read(std::ptr::NonNull::from_ref(self.element_mut(&mut slot)))
        };
        Record·element·slot {
            element: element,
            slot: Unset_slot::<LocalOrigin>::from_index(slot.index),
        }
    }
    pub fn set(&mut self, slot: Unset_slot<LocalOrigin>, element: Element) -> Slot<LocalOrigin> {
        // Unset_slot always references valid position and is inaccessible after this operation
        unsafe { self.elements.get_unchecked_mut(slot.index as usize) }.write(element);
        Slot::<LocalOrigin>::from_index(slot.index)
    }
    pub fn slot_rid(&mut self, slot_to_vacate: Unset_slot<LocalOrigin>) {
        // can maybe be optimized
        self.span_rid(slot_to_vacate.to_span());
    }
    pub fn opt_span_rid(&mut self, span_to_vacate: Opt<Unset_span<LocalOrigin>>) {
        if let Opt::Present(span_to_vacate) = span_to_vacate {
            self.span_rid(span_to_vacate);
        }
    }
    pub fn span_rid(&mut self, span_to_vacate: Unset_span<LocalOrigin>) {
        let maybe_vacant_span_index_connecting_earlier: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_span| {
                std::cmp::PartialEq::<usize>::eq(
                    &(vacant_span.start.index as usize + vacant_span.length.get() as usize),
                    &(span_to_vacate.start.index as usize),
                )
            });
        let maybe_vacant_span_index_connecting_later: std::option::Option<usize> =
            std::iter::Iterator::rposition(&mut self.vacant.iter(), |vacant_span| {
                std::cmp::PartialEq::<usize>::eq(
                    &(span_to_vacate.start.index as usize + span_to_vacate.length.get() as usize),
                    &(vacant_span.start.index as usize),
                )
            });
        match (
            maybe_vacant_span_index_connecting_earlier,
            maybe_vacant_span_index_connecting_later,
        ) {
            (std::option::Option::None, std::option::Option::None) => {
                if span_to_vacate.start.index as usize + span_to_vacate.length.get() as usize
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
                *later_span_to_extend = Unset_span {
                    start: Unset_slot::<LocalOrigin>::from_index(earlier_span_start),
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
                if span_to_vacate.start.index as usize + span_to_vacate.length.get() as usize
                    == self.elements.len()
                {
                    self.elements.truncate(
                        self.elements.len()
                            - span_to_vacate.length.get() as usize
                            - earlier_opt_span_to_extend.length.get() as usize,
                    );
                    let _ = self.vacant.swap_remove(index_connecting_earlier);
                } else {
                    earlier_opt_span_to_extend.length = std::num::NonZeroU32::saturating_add(
                        span_to_vacate.length,
                        earlier_opt_span_to_extend.length.get(),
                    );
                }
            }
            (std::option::Option::None, std::option::Option::Some(index_connecting_after)) => {
                let later_opt_span_to_extend = &mut self.vacant[index_connecting_after];
                *later_opt_span_to_extend = Unset_span {
                    start: span_to_vacate.start,
                    length: std::num::NonZeroU32::saturating_add(
                        span_to_vacate.length,
                        later_opt_span_to_extend.length.get(),
                    ),
                };
            }
        }
    }
    pub fn add(&mut self, new_element: Element) -> Slot<LocalOrigin> {
        let added_index = self.elements.len();
        self.elements.push(std::mem::MaybeUninit::new(new_element));
        Slot::from_index(added_index as u32)
    }
    pub fn add_unset(&mut self) -> Unset_slot<LocalOrigin> {
        let added_index = self.elements.len();
        self.elements.push(std::mem::MaybeUninit::uninit());
        Unset_slot::from_index(added_index as u32)
    }
    pub fn insert(&mut self, new_element: Element) -> Slot<LocalOrigin> {
        let unset_slot = self.insert_unset();
        self.set(unset_slot, new_element)
    }
    pub fn insert_unset(&mut self) -> Unset_slot<LocalOrigin> {
        match self.vacant.pop() {
            std::option::Option::None => self.add_unset(),
            std::option::Option::Some(vacant_opt_span_to_occupy) => {
                if let std::option::Option::Some(remaining_length) =
                    std::num::NonZeroU32::new(p32_predecessor(vacant_opt_span_to_occupy.length))
                {
                    self.vacant.push(Unset_span {
                        start: Unset_slot::<LocalOrigin>::from_index(
                            vacant_opt_span_to_occupy.start.index + 1,
                        ),
                        length: remaining_length,
                    });
                }
                vacant_opt_span_to_occupy.start
            }
        }
    }
    pub fn add_unset_length(&mut self, length: u32) -> Opt<Unset_span<LocalOrigin>> {
        match P32::new(length) {
            std::option::Option::None => Opt::Absent(()),
            std::option::Option::Some(length) => {
                let span = self.add_unset_length_positive(length);
                Opt::Present(span)
            }
        }
    }
    pub fn add_unset_length_positive(
        &mut self,
        length: std::num::NonZeroU32,
    ) -> Unset_span<LocalOrigin> {
        let unset_start_index = self.elements.len();
        // If below doesn't get optimized, an unsafe but maybe faster alternative would be
        // using reserve + set_len(.len + length)
        std::iter::Extend::extend(
            &mut self.elements,
            std::iter::Iterator::take(
                std::iter::repeat_with(|| std::mem::MaybeUninit::uninit()),
                length.get() as usize,
            ),
        );
        Unset_span {
            start: Unset_slot::<LocalOrigin>::from_index(unset_start_index as u32),
            length: length,
        }
    }
    // potential improvement: return Unset_span
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
    fn insert_iterator_filled(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
        new_element_count: std::num::NonZeroU32,
    ) -> Span<LocalOrigin> {
        match self.mark_length_positive_as_occupied(new_element_count) {
            std::option::Option::None => self.add_iterator_filled(new_elements, new_element_count),
            std::option::Option::Some(index_to_populate_from) => {
                let new_span = Span {
                    start: Slot::from_index(index_to_populate_from),
                    length: new_element_count,
                };
                self.elements.splice(
                    new_span.to_range(),
                    new_elements.map(std::mem::MaybeUninit::new),
                );
                new_span
            }
        }
    }
    pub fn add_iterator(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
    ) -> Opt<Span<LocalOrigin>> {
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
    fn add_iterator_filled(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element>,
        new_element_count: std::num::NonZeroU32,
    ) -> Span<LocalOrigin> {
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
    pub fn insert_iterator(
        &mut self,
        new_elements: impl std::iter::ExactSizeIterator<Item = Element>,
    ) -> Opt<Span<LocalOrigin>> {
        match std::num::NonZeroU32::new(new_elements.len() as u32) {
            std::option::Option::None => Opt::Absent(()),
            std::option::Option::Some(new_element_count) => {
                Opt::Present(self.insert_iterator_filled(new_elements, new_element_count))
            }
        }
    }
    // This will clone the iterator. Prefer add_iterator whenever possible
    pub fn insert_iterator_without_known_size(
        &mut self,
        new_elements: impl std::iter::Iterator<Item = Element> + std::clone::Clone,
    ) -> Opt<Span<LocalOrigin>> {
        // can be optimized to only clone if there is actually existing vacant space to occupy.
        // Might make sense to also benchmark with simply writing to the end, then relocating
        let std::option::Option::Some(new_length) =
            std::num::NonZeroU32::new(std::iter::Iterator::count(new_elements.clone()) as u32)
        else {
            return Opt::Absent(());
        };
        let new_span = self.insert_iterator_filled(new_elements, new_length);
        Opt::Present(new_span)
    }
    pub fn add_array<Record>(&mut self, new_elements: Array<Element, Record>) -> Span<LocalOrigin> {
        let length_without_new_elements = self.elements.len();
        let new_last = (new_elements.split_last_and_extend_vec_with_before)(
            &mut self.elements,
            new_elements.record,
        );
        let length_with_new_elements_before_last = self.elements.len();
        self.elements.push(std::mem::MaybeUninit::new(new_last));
        Span {
            start: Slot::from_index(length_without_new_elements as u32),
            length: std::num::NonZeroU32::MIN.saturating_add(
                (length_with_new_elements_before_last - length_without_new_elements) as u32,
            ),
        }
    }
    pub fn insert_vec_span<SourceOrigin>(
        &mut self,
        source: &mut Vec<SourceOrigin, Element>,
        source_span: Span<SourceOrigin>,
    ) -> (Unset_span<SourceOrigin>, Span<LocalOrigin>) {
        let (source_span_start_index, source_span_length) =
            (source_span.start.index, source_span.length);
        let source_elements = source.span_into_iterator(source_span);
        let new_span = self.insert_iterator_filled(source_elements, source_span_length);
        (
            Unset_span::<SourceOrigin> {
                start: Unset_slot::<SourceOrigin>::from_index(source_span_start_index),
                length: source_span_length,
            },
            new_span,
        )
    }
    pub fn add_vec_span<SourceOrigin>(
        &mut self,
        source: &mut Vec<SourceOrigin, Element>,
        source_span: Span<SourceOrigin>,
    ) -> (Unset_span<SourceOrigin>, Span<LocalOrigin>) {
        let (source_span_start_index, source_span_length) =
            (source_span.start.index, source_span.length);
        let source_elements = source.span_into_iterator(source_span);
        let new_span = self.add_iterator_filled(source_elements, source_span_length);
        (
            Unset_span::<SourceOrigin> {
                start: Unset_slot::<SourceOrigin>::from_index(source_span_start_index),
                length: source_span_length,
            },
            new_span,
        )
    }
    pub fn span_add_vec_span<SourceOrigin>(
        &mut self,
        span: Span<LocalOrigin>,
        source: &mut Vec<SourceOrigin, Element>,
        source_span: Span<SourceOrigin>,
    ) -> (Unset_span<SourceOrigin>, Span<LocalOrigin>) {
        let (source_span_start_index, source_span_length) =
            (source_span.start.index, source_span.length);
        let source_elements = source.span_into_iterator(source_span);
        let new_span = self.span_add_iterator(span, source_elements);
        (
            Unset_span::<SourceOrigin> {
                start: Unset_slot::<SourceOrigin>::from_index(source_span_start_index),
                length: source_span_length,
            },
            new_span,
        )
    }
    pub fn span_add_vec_opt_span<SourceOrigin>(
        &mut self,
        span: Span<LocalOrigin>,
        source: &mut Vec<SourceOrigin, Element>,
        source_span: Opt<Span<SourceOrigin>>,
    ) -> (Opt<Unset_span<SourceOrigin>>, Span<LocalOrigin>) {
        match source_span {
            Opt::Absent(()) => (Opt::Absent(()), span),
            Opt::Present(source_span) => {
                let (source_span, combined_span) =
                    self.span_add_vec_span(span, source, source_span);
                (Opt::Present(source_span), combined_span)
            }
        }
    }
    pub fn opt_span_add_vec_span<SourceOrigin>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        source: &mut Vec<SourceOrigin, Element>,
        source_span: Span<SourceOrigin>,
    ) -> (Unset_span<SourceOrigin>, Span<LocalOrigin>) {
        match span {
            Opt::Absent(()) => self.add_vec_span(source, source_span),
            Opt::Present(span) => self.span_add_vec_span(span, source, source_span),
        }
    }
    pub fn opt_span_add_vec_opt_span<SourceOrigin>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        source: &mut Vec<SourceOrigin, Element>,
        source_span: Opt<Span<SourceOrigin>>,
    ) -> (Opt<Unset_span<SourceOrigin>>, Opt<Span<LocalOrigin>>) {
        match source_span {
            Opt::Absent(()) => (Opt::Absent(()), span),
            Opt::Present(source_span) => {
                let (source_span, combined_span) =
                    self.opt_span_add_vec_span(span, source, source_span);
                (Opt::Present(source_span), Opt::Present(combined_span))
            }
        }
    }
    pub fn span_add_iterator(
        &mut self,
        span: Span<LocalOrigin>,
        new_elements: impl std::iter::Iterator<Item = Element>,
    ) -> Span<LocalOrigin> {
        let moved_span = self.span_move_to_end(span);
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
    pub fn span_add_array<Record>(
        &mut self,
        span: Span<LocalOrigin>,
        new_elements: Array<Element, Record>,
    ) -> Span<LocalOrigin> {
        let moved_span = self.span_move_to_end(span);
        let length_before_extend = self.elements.len();
        let new_last = (new_elements.split_last_and_extend_vec_with_before)(
            &mut self.elements,
            new_elements.record,
        );
        self.elements.push(std::mem::MaybeUninit::new(new_last));
        Span {
            start: moved_span.start,
            length: moved_span
                .length
                .saturating_add((self.elements.len() - length_before_extend) as u32),
        }
    }
    pub fn opt_span_add_array<Record>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        new_elements: Array<Element, Record>,
    ) -> Span<LocalOrigin> {
        match span {
            Opt::Absent(()) => self.add_array(new_elements),
            Opt::Present(span) => self.span_add_array(span, new_elements),
        }
    }
    pub fn span_add(&mut self, span: Span<LocalOrigin>, new_element: Element) -> Span<LocalOrigin> {
        let moved_span = self.span_move_to_end(span);
        self.elements.push(std::mem::MaybeUninit::new(new_element));
        Span {
            start: moved_span.start,
            length: moved_span.length.saturating_add(1),
        }
    }
    pub fn span_move_to_end(&mut self, span: Span<LocalOrigin>) -> Span<LocalOrigin> {
        if span.start.index as usize + span.length.get() as usize == self.elements.len() {
            return span;
        }
        // span is not at the end already

        let move_destination_span = self.add_unset_length_positive(span.length);
        {
            let (before_move_destination, from_move_destination) = unsafe {
                self.elements
                    .split_at_mut_unchecked(move_destination_span.start.index as usize)
            };
            // technically we just want to write the destination, not swap. Something like
            // read_from_slice(&mut [MaybeUninit<A>], &[MaybeUninit<A>])
            // I know there is std::ptr::copy_nonoverlapping(src, dst, count) but I'd prefer something higher-level
            // If you know how to do this (nicely), open an issue please
            unsafe { before_move_destination.get_unchecked_mut(span.to_range()) }
                .swap_with_slice(from_move_destination);
        }
        self.span_rid(Unset_span {
            start: Unset_slot::<LocalOrigin>::from_index(span.start.index),
            length: span.length,
        });
        Span {
            start: Slot::<LocalOrigin>::from_index(move_destination_span.start.index),
            length: move_destination_span.length,
        }
    }
    pub fn span_is_at_the_end<Occupancy>(
        &self,
        span: &Span_with_occupancy<LocalOrigin, Occupancy>,
    ) -> bool {
        (span.start.index as usize + span.length.get() as usize) < self.elements.len()
    }
    pub fn span_move_to_vacant(&mut self, mut span: Span<LocalOrigin>) -> Span<LocalOrigin> {
        if self.span_is_at_the_end(&span) {
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
                    start: Slot::<LocalOrigin>::from_index(earlier_start_to_occupy_from),
                    length: span.length,
                }
            }
        }
    }
    pub fn span_add_own_span(
        &mut self,
        start: Span<LocalOrigin>,
        end: Span<LocalOrigin>,
    ) -> Span<LocalOrigin> {
        let combined_length = start.length.saturating_add(end.length.get());
        if start.start.index + start.length.get() == end.start.index {
            Span {
                start: start.start,
                length: combined_length,
            }
        } else {
            let start_moved = self.span_move_to_end(start);
            let _ = self.span_move_to_end(end);
            Span {
                start: start_moved.start,
                length: combined_length,
            }
        }
    }
    pub fn span_add_own_opt_span(
        &mut self,
        start: Span<LocalOrigin>,
        end: Opt<Span<LocalOrigin>>,
    ) -> Span<LocalOrigin> {
        match end {
            Opt::Absent(()) => start,
            Opt::Present(end) => self.span_add_own_span(start, end),
        }
    }
    pub fn opt_span_add_own_span(
        &mut self,
        start: Opt<Span<LocalOrigin>>,
        end: Span<LocalOrigin>,
    ) -> Span<LocalOrigin> {
        match start {
            Opt::Absent(()) => end,
            Opt::Present(start) => self.span_add_own_span(start, end),
        }
    }
    pub fn opt_span_add_own_opt_span(
        &mut self,
        start: Opt<Span<LocalOrigin>>,
        end: Opt<Span<LocalOrigin>>,
    ) -> Opt<Span<LocalOrigin>> {
        match start {
            Opt::Absent(()) => end,
            Opt::Present(start) => Opt::Present(self.span_add_own_opt_span(start, end)),
        }
    }
    pub fn unset_span_add_own_span(
        &mut self,
        start: Unset_span<LocalOrigin>,
        end: Unset_span<LocalOrigin>,
    ) -> Unset_span<LocalOrigin> {
        let combined_length = start.length.saturating_add(end.length.get());
        if start.start.index as usize + start.length.get() as usize == end.start.index as usize {
            Unset_span {
                start: start.start,
                length: combined_length,
            }
        } else {
            self.add_unset_length_positive(combined_length)
        }
    }
    pub fn unset_span_add_own_opt_span(
        &mut self,
        start: Unset_span<LocalOrigin>,
        end: Opt<Unset_span<LocalOrigin>>,
    ) -> Unset_span<LocalOrigin> {
        match end {
            Opt::Absent(()) => start,
            Opt::Present(end) => self.unset_span_add_own_span(start, end),
        }
    }
    pub fn opt_unset_span_add_own_span(
        &mut self,
        start: Opt<Unset_span<LocalOrigin>>,
        end: Unset_span<LocalOrigin>,
    ) -> Unset_span<LocalOrigin> {
        match start {
            Opt::Absent(()) => end,
            Opt::Present(start) => self.unset_span_add_own_span(start, end),
        }
    }
    pub fn opt_unset_span_add_own_opt_span(
        &mut self,
        start: Opt<Unset_span<LocalOrigin>>,
        end: Opt<Unset_span<LocalOrigin>>,
    ) -> Opt<Unset_span<LocalOrigin>> {
        match start {
            Opt::Absent(()) => end,
            Opt::Present(start) => Opt::Present(self.unset_span_add_own_opt_span(start, end)),
        }
    }
    pub fn vacant_spans<'a>(&'a self) -> &'a std::vec::Vec<Unset_span<LocalOrigin>> {
        &self.vacant
    }
    pub fn maybe_uninit_elements<'a>(
        &'a self,
    ) -> &'a std::vec::Vec<std::mem::MaybeUninit<Element>> {
        &self.elements
    }
    pub fn length_vacated_or_not(&self) -> usize {
        self.elements.len()
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
        usize::saturating_sub(self.length_vacated_or_not(), self.vacant_count_usize())
    }
    /// The raw allocation. Can be used to create new Vecs or even
    /// to drop the memory in a separate thread
    pub fn into_unset_slice(self) -> Unset_slice<Element> {
        Unset_slice::from_vec_maybe_uninit(self.elements)
    }
}
impl<Origin> Vec<Origin, Char> {
    /// try to avoid using
    pub fn insert_str(&mut self, new_str: &str) -> Opt<Span<Origin>> {
        self.insert_iterator_without_known_size(new_str.chars())
    }
    pub fn add_str(&mut self, new_str: &str) -> Opt<Span<Origin>> {
        self.add_iterator(new_str.chars())
    }
    pub fn opt_span_add_str(
        &mut self,
        span: Opt<Span<Origin>>,
        new_str: &str,
    ) -> Opt<Span<Origin>> {
        match span {
            Opt::Absent(()) => self.add_str(new_str),
            Opt::Present(span) => Opt::Present(self.span_add_str(span, new_str)),
        }
    }
    pub fn span_add_str(&mut self, span: Span<Origin>, new_str: &str) -> Span<Origin> {
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
    pub fn split_after_length_positive(
        self,
        start_length_or_greater: std::num::NonZeroU32,
    ) -> Record·after·start<
        Opt<Span_with_occupancy<Origin, Occupancy>>,
        Span_with_occupancy<Origin, Occupancy>,
    > {
        let start_length =
            <std::num::NonZeroU32 as std::cmp::Ord>::min(start_length_or_greater, self.length);
        Record·after·start {
            after: match std::num::NonZeroU32::new(self.length.get() - start_length.get()) {
                std::option::Option::None => Opt::Absent(()),
                std::option::Option::Some(after_length) => {
                    Opt::Present(Span_with_occupancy::<Origin, Occupancy> {
                        start: Slot_with_occupancy::<Origin, Occupancy>::from_index(
                            self.start.index + start_length.get(),
                        ),
                        length: after_length,
                    })
                }
            },
            start: Span_with_occupancy::<Origin, Occupancy> {
                start: self.start,
                length: start_length,
            },
        }
    }
    pub fn split_before_end_length_positive(
        self,
        end_length_or_greater: std::num::NonZeroU32,
    ) -> Record·before·end<
        Opt<Span_with_occupancy<Origin, Occupancy>>,
        Span_with_occupancy<Origin, Occupancy>,
    > {
        let end_length =
            <std::num::NonZeroU32 as std::cmp::Ord>::min(end_length_or_greater, self.length);
        let before_length = self.length.get() - end_length.get();
        Record·before·end {
            end: Span_with_occupancy::<Origin, Occupancy> {
                start: Slot_with_occupancy::<Origin, Occupancy>::from_index(
                    self.start.index + before_length,
                ),
                length: end_length,
            },
            before: match std::num::NonZeroU32::new(before_length) {
                std::option::Option::None => Opt::Absent(()),
                std::option::Option::Some(before_length) => {
                    Opt::Present(Span_with_occupancy::<Origin, Occupancy> {
                        start: self.start,
                        length: before_length,
                    })
                }
            },
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
pub fn p32_rid(_: P32) -> Record {}
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
pub fn u32_rid(_: U32) -> Record {}
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
pub fn i32_rid(_: I32) -> Record {}
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
pub fn f32_rid(_: F32) -> Record {}
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
pub fn f32_round_up(n: F32) -> F32 {
    n.ceil()
}
pub fn f32_round_down(n: F32) -> F32 {
    n.floor()
}
pub fn f32_round_toward_0(n: F32) -> F32 {
    n.trunc()
}
pub fn f32_round_away_from_0(n: F32) -> F32 {
    // I'm not convinced this is the fastest but since this is by far the
    // most common implementation I've seen I'm hoping this gets optimized at least
    n.abs().ceil() * n.signum()
}
pub fn f32_round_nearest_else_even(n: F32) -> F32 {
    n.round_ties_even()
}
pub fn f32_round_nearest_else_away_from_0(n: F32) -> F32 {
    n.round()
}
pub fn f32_round_up_to_i32_clamp(n: F32) -> I32 {
    f32_round_up(n) as I32
}
pub fn f32_round_down_to_i32_clamp(n: F32) -> I32 {
    f32_round_down(n) as I32
}
pub fn f32_round_toward_0_to_i32_clamp(n: F32) -> I32 {
    n as I32
}
pub fn f32_round_away_from_0_to_i32_clamp(n: F32) -> I32 {
    f32_round_away_from_0(n) as I32
}
pub fn f32_round_nearest_else_even_to_i32_clamp(n: F32) -> I32 {
    f32_round_nearest_else_even(n) as I32
}
pub fn f32_round_nearest_else_away_from_0_to_i32_clamp(n: F32) -> I32 {
    f32_round_nearest_else_away_from_0(n) as I32
}

pub fn fn_dup<In, Out>(fn_: Fn<In, Out>) -> Record·a·b<Fn<In, Out>, Fn<In, Out>> {
    Record·a·b { a: fn_, b: fn_ }
}
pub fn fn_rid<In, Out>(_: Fn<In, Out>) -> Record {}

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

pub fn unset_slot_to_span<Origin>(slot: Unset_slot<Origin>) -> Unset_span<Origin> {
    slot.to_span()
}
pub fn unset_slot_index<Origin>(
    slot: Unset_slot<Origin>,
) -> Record·index·slot<u32, Unset_slot<Origin>> {
    Record·index·slot {
        index: slot.index,
        slot: slot,
    }
}

pub fn span_length<Origin>(span: Span<Origin>) -> Record·length·span<P32, Span<Origin>> {
    Record·length·span {
        length: span.length,
        span: span,
    }
}
pub fn opt_span_length<Origin>(
    span: Opt<Span<Origin>>,
) -> Record·length·span<u32, Opt<Span<Origin>>> {
    Record·length·span {
        length: span.as_ref().length(),
        span: span,
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
pub fn span_start_of_length_positive<Origin>(
    Record·length·span {
        length: start_length,
        span,
    }: Record·length·span<P32, Span<Origin>>,
) -> Record·after·start<Opt<Span<Origin>>, Span<Origin>> {
    span.split_after_length_positive(start_length)
}
pub fn span_end_of_length_positive<Origin>(
    Record·length·span {
        length: start_length,
        span,
    }: Record·length·span<P32, Span<Origin>>,
) -> Record·before·end<Opt<Span<Origin>>, Span<Origin>> {
    span.split_before_end_length_positive(start_length)
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
    match span {
        Opt::Absent(()) => Choice·Exit·Go_on::Go_on(initial_state),
        Opt::Present(span) => span_fold_while(Record·direction·span·state·step {
            direction: direction,
            span: span,
            state: initial_state,
            step: step,
        }),
    }
}
pub fn span_fold_while<Exit, GoOn, Origin>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Span<Origin>,
        GoOn,
        Fn<Record·slot·state<Slot<Origin>, GoOn>, Choice·Exit·Go_on<Exit, GoOn>>,
    >,
) -> Choice·Exit·Go_on<Record·exit·remaining<Exit, Opt<Span<Origin>>>, GoOn> {
    let state_after_fold = iterator_try_fold_in_direction(
        span.to_range_u32(),
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
            } = span.split_after_length_positive(P32::MIN.saturating_add(exit_index));
            Choice·Exit·Go_on::Exit(Record·exit·remaining {
                exit: exit_state,
                remaining: not_folded_over_opt_span,
            })
        }
    }
}

pub fn unset_span_length<Origin>(
    span: Unset_span<Origin>,
) -> Record·length·span<P32, Unset_span<Origin>> {
    Record·length·span {
        length: span.length,
        span: span,
    }
}
pub fn opt_unset_span_length<Origin>(
    span: Opt<Unset_span<Origin>>,
) -> Record·length·span<U32, Opt<Unset_span<Origin>>> {
    Record·length·span {
        length: span.as_ref().length(),
        span: span,
    }
}
pub fn unset_span_start<Origin>(
    span: Unset_span<Origin>,
) -> Record·after·start<Opt<Unset_span<Origin>>, Unset_slot<Origin>> {
    span.split_start()
}
pub fn unset_span_end<Origin>(
    span: Unset_span<Origin>,
) -> Record·before·end<Opt<Unset_span<Origin>>, Unset_slot<Origin>> {
    span.split_end()
}
pub fn unset_span_start_of_length_positive<Origin>(
    Record·length·span {
        length: start_length,
        span,
    }: Record·length·span<P32, Unset_span<Origin>>,
) -> Record·after·start<Opt<Unset_span<Origin>>, Unset_span<Origin>> {
    span.split_after_length_positive(start_length)
}
pub fn unset_span_end_of_length_positive<Origin>(
    Record·length·span {
        length: start_length,
        span,
    }: Record·length·span<P32, Unset_span<Origin>>,
) -> Record·before·end<Opt<Unset_span<Origin>>, Unset_span<Origin>> {
    span.split_before_end_length_positive(start_length)
}
pub fn opt_unset_span_fold<Origin, State>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Opt<Unset_span<Origin>>,
        State,
        Fn<Record·slot·state<Unset_slot<Origin>, State>, State>,
    >,
) -> State {
    iterator_fold_in_direction(
        span.as_ref().to_range_u32(),
        direction,
        initial_state,
        |state, index| {
            step(Record·slot·state {
                state,
                slot: Unset_slot::<Origin>::from_index(index),
            })
        },
    )
}

pub fn origin_rid<LocalOrigin>(_: Origin<LocalOrigin>) -> Record {}

pub fn vec_empty<LocalOrigin, Element>(origin: Origin<LocalOrigin>) -> Vec<LocalOrigin, Element> {
    Vec::<LocalOrigin, Element>::new(origin)
}
pub fn vec_pre_allocate_at_least<Element, Origin>(
    Record·length·vec {
        vec: mut vec,
        length: min_pre_allocated_length,
    }: Record·length·vec<u32, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.pre_allocate_at_least(min_pre_allocated_length);
    vec
}
pub fn vec_pre_allocation_rid<Element, Origin>(
    mut vec: Vec<Origin, Element>,
) -> Vec<Origin, Element> {
    vec.pre_allocation_rid();
    vec
}
pub fn vec_remove<Element, Origin>(
    Record·slot·vec { mut vec, slot }: Record·slot·vec<Slot<Origin>, Vec<Origin, Element>>,
) -> Record·element·vec<Element, Vec<Origin, Element>> {
    let element = vec.remove(slot);
    Record·element·vec {
        element: element,
        vec: vec,
    }
}
pub fn vec_unset<Element, Origin>(
    Record·slot·vec { mut vec, slot }: Record·slot·vec<Slot<Origin>, Vec<Origin, Element>>,
) -> Record·element·slot·vec<Element, Unset_slot<Origin>, Vec<Origin, Element>> {
    let element = vec.unset(slot);
    Record·element·slot·vec {
        element: element.element,
        slot: element.slot,
        vec: vec,
    }
}
pub fn vec_set<Element, Origin>(
    Record·new·slot·vec {
        mut vec,
        slot,
        new: element,
    }: Record·new·slot·vec<Element, Unset_slot<Origin>, Vec<Origin, Element>>,
) -> Record·slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let set_slot = vec.set(slot, element);
    Record·slot·vec {
        vec: vec,
        slot: set_slot,
    }
}
pub fn vec_slot_rid<Element, Origin>(
    Record·slot·vec {
        slot: slot_to_vacate,
        mut vec,
    }: Record·slot·vec<Unset_slot<Origin>, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.slot_rid(slot_to_vacate);
    vec
}
pub fn vec_span_rid<Element, Origin>(
    Record·span·vec {
        span: span_to_vacate,
        mut vec,
    }: Record·span·vec<Unset_span<Origin>, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.span_rid(span_to_vacate);
    vec
}
pub fn vec_opt_span_rid<Element, Origin>(
    Record·span·vec {
        span: span_to_vacate,
        mut vec,
    }: Record·span·vec<Opt<Unset_span<Origin>>, Vec<Origin, Element>>,
) -> Vec<Origin, Element> {
    vec.opt_span_rid(span_to_vacate);
    vec
}
pub fn vec_rid<Element, Origin>(_: Vec<Origin, Element>) -> Record {}
pub fn vec_insert<Element, Origin>(
    Record·new·vec {
        mut vec,
        new: new_element,
    }: Record·new·vec<Element, Vec<Origin, Element>>,
) -> Record·slot·vec<Slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.insert(new_element);
    Record·slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add<Element, Origin>(
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
pub fn vec_insert_unset<Element, Origin>(
    mut vec: Vec<Origin, Element>,
) -> Record·slot·vec<Unset_slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.insert_unset();
    Record·slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add_unset<Element, Origin>(
    mut vec: Vec<Origin, Element>,
) -> Record·slot·vec<Unset_slot<Origin>, Vec<Origin, Element>> {
    let slot = vec.add_unset();
    Record·slot·vec {
        vec: vec,
        slot: slot,
    }
}
pub fn vec_add_unset_length<Element, Origin>(
    Record·length·vec { length, mut vec }: Record·length·vec<U32, Vec<Origin, Element>>,
) -> Record·span·vec<Opt<Unset_span<Origin>>, Vec<Origin, Element>> {
    let span = vec.add_unset_length(length);
    Record·span·vec {
        vec: vec,
        span: span,
    }
}
pub fn vec_add_unset_length_positive<Element, Origin>(
    Record·length·vec { length, mut vec }: Record·length·vec<P32, Vec<Origin, Element>>,
) -> Record·span·vec<Unset_span<Origin>, Vec<Origin, Element>> {
    let span = vec.add_unset_length_positive(length);
    Record·span·vec {
        vec: vec,
        span: span,
    }
}
pub fn vec_insert_vec_span<Origin, SourceOrigin, Element>(
    mut vec: Vec<Origin, Element>,
    mut source: Vec<SourceOrigin, Element>,
    source_span: Span<SourceOrigin>,
) -> Record·source·source_span·span·vec<
    Vec<SourceOrigin, Element>,
    Unset_span<SourceOrigin>,
    Span<Origin>,
    Vec<Origin, Element>,
> {
    let (source_span, new_span) = vec.insert_vec_span(&mut source, source_span);
    Record·source·source_span·span·vec {
        vec: vec,
        source: source,
        source_span: source_span,
        span: new_span,
    }
}
pub fn vec_insert_str<Origin>(
    Record·new·vec {
        mut vec,
        new: new_str,
    }: Record·new·vec<Str, Vec<Origin, Char>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Char>> {
    let new_span = vec.insert_str(new_str);
    Record·span·vec {
        vec: vec,
        span: new_span,
    }
}
pub fn vec_char_add_str<Origin>(
    Record·new·vec {
        mut vec,
        new: new_str,
    }: Record·new·vec<Str, Vec<Origin, Char>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Char>> {
    let new_span = vec.add_str(new_str);
    Record·span·vec {
        vec: vec,
        span: new_span,
    }
}
pub fn vec_replace<Element, Origin>(
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
pub fn vec_opt_span_reverse<Element, Origin>(
    Record·span·vec { mut vec, mut span }: Record·span·vec<
        Opt<Span<Origin>>,
        Vec<Origin, Element>,
    >,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    vec.opt_span_slice_mut(&mut span).reverse();
    Record·span·vec { vec: vec, span }
}
pub fn vec_span_reverse<Element, Origin>(
    Record·span·vec { mut vec, mut span }: Record·span·vec<Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    vec.span_slice_mut(&mut span).reverse();
    Record·span·vec { vec: vec, span }
}

pub fn vec_opt_span_add<Element, Origin>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_element,
    }: Record·new·span·vec<Element, Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    match span {
        Opt::Absent(()) => {
            let new_slot = vec.insert(new_element);
            Record·span·vec {
                vec: vec,
                span: slot_to_span(new_slot),
            }
        }
        Opt::Present(span) => vec_span_add(Record·new·span·vec {
            vec: vec,
            span: span,
            new: new_element,
        }),
    }
}
pub fn vec_span_add<Element, Origin>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_element,
    }: Record·new·span·vec<Element, Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.span_add(span, new_element);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_span_add_array<Element, Origin, Record>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        Array<Element, Record>,
        Span<Origin>,
        Vec<Origin, Element>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.span_add_array(span, new);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_opt_span_add_array<Element, Origin, Record>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        Array<Element, Record>,
        Opt<Span<Origin>>,
        Vec<Origin, Element>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.opt_span_add_array(span, new);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_char_opt_span_add_str<Origin>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_str,
    }: Record·new·span·vec<Str, Opt<Span<Origin>>, Vec<Origin, Char>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Char>> {
    let combined_span = vec.opt_span_add_str(span, new_str);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_char_span_add_str<Origin>(
    Record·new·span·vec {
        mut vec,
        span,
        new: new_str,
    }: Record·new·span·vec<Str, Span<Origin>, Vec<Origin, Char>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    let combined_span = vec.span_add_str(span, new_str);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_char_span_add_u32<Origin>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        U32,
        Span<Origin>,
        Vec<Origin, Char>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    // can be optimized once https://github.com/rust-lang/rust/issues/138215 lands
    let new_as_string = std::format!("{}", new);
    let combined_span = vec.span_add_str(span, &new_as_string);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_char_opt_span_add_u32<Origin>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        U32,
        Opt<Span<Origin>>,
        Vec<Origin, Char>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    // can be optimized once https://github.com/rust-lang/rust/issues/138215 lands
    let new_as_string = std::format!("{}", new);
    let combined_span = vec.opt_span_add_str(span, &new_as_string);
    Record·span·vec {
        vec: vec,
        span: {
            // new_as_string has .len() >= 1 because a formatted number is never ""
            unsafe { combined_span.into_option().unwrap_unchecked() }
        },
    }
}
pub fn vec_char_span_add_i32<Origin>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        I32,
        Span<Origin>,
        Vec<Origin, Char>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    // can be optimized once https://github.com/rust-lang/rust/issues/138215 lands
    let new_as_string = std::format!("{}", new);
    let combined_span = vec.span_add_str(span, &new_as_string);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_char_opt_span_add_i32<Origin>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        I32,
        Opt<Span<Origin>>,
        Vec<Origin, Char>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    // can be optimized once https://github.com/rust-lang/rust/issues/138215 lands
    let new_as_string = std::format!("{}", new);
    let combined_span = vec.opt_span_add_str(span, &new_as_string);
    Record·span·vec {
        vec: vec,
        span: {
            // new_as_string has .len() >= 1 because a formatted number is never ""
            unsafe { combined_span.into_option().unwrap_unchecked() }
        },
    }
}
pub fn vec_char_span_add_f32<Origin>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        F32,
        Span<Origin>,
        Vec<Origin, Char>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    // can be optimized once https://github.com/rust-lang/rust/issues/138215 lands
    let new_as_string = std::format!("{:.}", new);
    let combined_span = vec.span_add_str(span, &new_as_string);
    Record·span·vec {
        vec: vec,
        span: combined_span,
    }
}
pub fn vec_char_opt_span_add_f32<Origin>(
    Record·new·span·vec { mut vec, span, new }: Record·new·span·vec<
        F32,
        Opt<Span<Origin>>,
        Vec<Origin, Char>,
    >,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Char>> {
    // can be optimized once https://github.com/rust-lang/rust/issues/138215 lands
    let new_as_string = std::format!("{:.}", new);
    let combined_span = vec.opt_span_add_str(span, &new_as_string);
    Record·span·vec {
        vec: vec,
        span: {
            // new_as_string has .len() >= 1 because a formatted number is never ""
            unsafe { combined_span.into_option().unwrap_unchecked() }
        },
    }
}
pub fn vec_opt_span_add_vec_opt_span<Origin, SourceOrigin, Element>(
    Record·source·source_span·span·vec {
        mut source,
        source_span,
        span,
        mut vec,
    }: Record·source·source_span·span·vec<
        Vec<SourceOrigin, Element>,
        Opt<Span<SourceOrigin>>,
        Opt<Span<Origin>>,
        Vec<Origin, Element>,
    >,
) -> Record·source·source_span·span·vec<
    Vec<SourceOrigin, Element>,
    Opt<Unset_span<SourceOrigin>>,
    Opt<Span<Origin>>,
    Vec<Origin, Element>,
> {
    let (source_span, combined_span) =
        vec.opt_span_add_vec_opt_span(span, &mut source, source_span);
    Record·source·source_span·span·vec {
        source: source,
        source_span,
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_span_add_vec_opt_span<Origin, SourceOrigin, Element>(
    Record·source·source_span·span·vec {
        mut source,
        source_span,
        span,
        mut vec,
    }: Record·source·source_span·span·vec<
        Vec<SourceOrigin, Element>,
        Opt<Span<SourceOrigin>>,
        Span<Origin>,
        Vec<Origin, Element>,
    >,
) -> Record·source·source_span·span·vec<
    Vec<SourceOrigin, Element>,
    Opt<Unset_span<SourceOrigin>>,
    Span<Origin>,
    Vec<Origin, Element>,
> {
    let (source_span, combined_span) = vec.span_add_vec_opt_span(span, &mut source, source_span);
    Record·source·source_span·span·vec {
        source: source,
        source_span,
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_opt_span_add_vec_span<Origin, SourceOrigin, Element>(
    Record·source·source_span·span·vec {
        mut source,
        source_span,
        span,
        mut vec,
    }: Record·source·source_span·span·vec<
        Vec<SourceOrigin, Element>,
        Span<SourceOrigin>,
        Opt<Span<Origin>>,
        Vec<Origin, Element>,
    >,
) -> Record·source·source_span·span·vec<
    Vec<SourceOrigin, Element>,
    Unset_span<SourceOrigin>,
    Span<Origin>,
    Vec<Origin, Element>,
> {
    let (source_span, combined_span) = vec.opt_span_add_vec_span(span, &mut source, source_span);
    Record·source·source_span·span·vec {
        source: source,
        source_span,
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_span_add_vec_span<Origin, SourceOrigin, Element>(
    Record·source·source_span·span·vec {
        mut source,
        source_span,
        span,
        mut vec,
    }: Record·source·source_span·span·vec<
        Vec<SourceOrigin, Element>,
        Span<SourceOrigin>,
        Span<Origin>,
        Vec<Origin, Element>,
    >,
) -> Record·source·source_span·span·vec<
    Vec<SourceOrigin, Element>,
    Unset_span<SourceOrigin>,
    Span<Origin>,
    Vec<Origin, Element>,
> {
    let (source_span, combined_span) = vec.span_add_vec_span(span, &mut source, source_span);
    Record·source·source_span·span·vec {
        source: source,
        source_span: source_span,
        span: combined_span,
        vec: vec,
    }
}

pub fn vec_span_add_own_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<Span<Origin>, Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.span_add_own_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_span_add_own_opt_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<Opt<Span<Origin>>, Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.span_add_own_opt_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_opt_span_add_own_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<Span<Origin>, Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.opt_span_add_own_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_opt_span_add_own_opt_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<Opt<Span<Origin>>, Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    let combined_span = vec.opt_span_add_own_opt_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_unset_span_add_own_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<Unset_span<Origin>, Unset_span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Unset_span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.unset_span_add_own_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_unset_span_add_own_opt_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<
        Opt<Unset_span<Origin>>,
        Unset_span<Origin>,
        Vec<Origin, Element>,
    >,
) -> Record·span·vec<Unset_span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.unset_span_add_own_opt_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_opt_unset_span_add_own_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<
        Unset_span<Origin>,
        Opt<Unset_span<Origin>>,
        Vec<Origin, Element>,
    >,
) -> Record·span·vec<Unset_span<Origin>, Vec<Origin, Element>> {
    let combined_span = vec.opt_unset_span_add_own_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}
pub fn vec_opt_unset_span_add_own_opt_span<Element, Origin>(
    Record·end·start·vec {
        end,
        start,
        mut vec,
    }: Record·end·start·vec<
        Opt<Unset_span<Origin>>,
        Opt<Unset_span<Origin>>,
        Vec<Origin, Element>,
    >,
) -> Record·span·vec<Opt<Unset_span<Origin>>, Vec<Origin, Element>> {
    let combined_span = vec.opt_unset_span_add_own_opt_span(start, end);
    Record·span·vec {
        span: combined_span,
        vec: vec,
    }
}

pub fn vec_span_move_to_vacant<Element, Origin>(
    Record·span·vec { span, mut vec }: Record·span·vec<Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let moved_span = vec.span_move_to_vacant(span);
    Record·span·vec {
        span: moved_span,
        vec: vec,
    }
}
pub fn vec_opt_span_move_to_vacant<Element, Origin>(
    Record·span·vec { span, mut vec }: Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    match span {
        Opt::Absent(()) => Record·span·vec {
            span: Opt::Absent(()),
            vec: vec,
        },
        Opt::Present(span) => {
            let moved_span = vec.span_move_to_vacant(span);
            Record·span·vec {
                span: Opt::Present(moved_span),
                vec: vec,
            }
        }
    }
}
pub fn vec_span_move_to_end<Element, Origin>(
    Record·span·vec { span, mut vec }: Record·span·vec<Span<Origin>, Vec<Origin, Element>>,
) -> Record·span·vec<Span<Origin>, Vec<Origin, Element>> {
    let moved_span = vec.span_move_to_end(span);
    Record·span·vec {
        span: moved_span,
        vec: vec,
    }
}
pub fn vec_opt_span_move_to_end<Element, Origin>(
    Record·span·vec { span, mut vec }: Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>>,
) -> Record·span·vec<Opt<Span<Origin>>, Vec<Origin, Element>> {
    match span {
        Opt::Absent(()) => Record·span·vec {
            span: Opt::Absent(()),
            vec: vec,
        },
        Opt::Present(span) => {
            let moved_span = vec.span_move_to_end(span);
            Record·span·vec {
                span: Opt::Present(moved_span),
                vec: vec,
            }
        }
    }
}
pub fn vec_to_unset<Element, Origin>(vec: Vec<Origin, Element>) -> Unset_slice<Element> {
    vec.into_unset_slice()
}
pub fn vec_reuse<LocalOrigin, Element>(
    Record·origin·slice { origin, slice }: Record·origin·slice<
        Origin<LocalOrigin>,
        Unset_slice<Element>,
    >,
) -> Vec<LocalOrigin, Element> {
    Vec::<LocalOrigin, Element>::reuse(origin, slice)
}

pub fn unset_slice_rid<Element>(_: Unset_slice<Element>) -> Record {}
pub fn unset_slice_length<Element>(
    unset_slice: Unset_slice<Element>,
) -> Record·length·slice<U32, Unset_slice<Element>> {
    Record·length·slice {
        length: unset_slice.length(),
        slice: unset_slice,
    }
}
pub fn unset_slice_allocate_length<Element>(length: U32) -> Unset_slice<Element> {
    Unset_slice::allocate_length(length)
}
pub fn unset_slice_cast_or_rid_and_allocate<Element, NewElement>(
    unset_slice: Unset_slice<Element>,
) -> Unset_slice<NewElement> {
    unset_slice.cast_or_rid_and_allocate::<NewElement>()
}

#[cfg(test)]
mod core_test {
    extern crate std;
    use std::prelude;
    #[test]
    fn add_remove_stress_test() {
        origin_new!(origin, Origin);
        let mut vec = crate::core::Vec::new(origin);
        let mut slots = std::iter::Iterator::collect::<std::vec::Vec<_>>(std::iter::Iterator::map(
            0..100,
            |i| vec.add(i),
        ));
        // a bit of fake dumb noise.
        // once rust gains std:: fuzzing/randomness we should use that
        slots.as_mut_slice()[25..50].reverse();
        slots.swap(2, 94);
        slots.swap(12, 88);
        slots.swap(34, 39);
        for slot in slots {
            vec.remove(slot);
        }
        std::assert_eq!(vec.vacant_spans().len(), 0);
        std::assert_eq!(vec.maybe_uninit_elements().len(), 0);
        crate::core::vec_rid(vec);
    }
    #[test]
    fn unset_slice_cast_or_rid_and_allocate_u64_to_i64() {
        let unset_slice_u64 = crate::core::Unset_slice::<u64>::allocate_length(20);
        let unset_slice_u64_address = unset_slice_u64.0.iter().as_slice().as_ptr().addr();
        let unset_slice_i64 = unset_slice_u64.cast_or_rid_and_allocate::<i64>();
        // memory is reused, not re-allocated
        std::assert_eq!(
            unset_slice_i64.0.iter().as_slice().as_ptr().addr(),
            unset_slice_u64_address
        );
        crate::core::origin_new!(origin, Origin);
        let vec = crate::core::Vec::reuse(origin, unset_slice_i64);
        crate::core::vec_rid(vec);
    }
    #[test]
    fn unset_slice_cast_or_rid_and_allocate_u64_to_tuple_u32_u32() {
        let unset_slice_u64 = crate::core::Unset_slice::<u64>::allocate_length(20);
        let unset_slice_tuple_u32_u32 = unset_slice_u64.cast_or_rid_and_allocate::<(u32, u32)>();
        crate::core::origin_new!(origin, Origin);
        let vec = crate::core::Vec::reuse(origin, unset_slice_tuple_u32_u32);
        crate::core::vec_rid(vec);
    }
}
