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
extern crate core;
extern crate std;

// Most module members are directly usable by sloe code to avoid name clashes with generated functions and types.
// The remaining few member names must be explicitly added to `sloe::name_to_uppercase_rust` and `name_to_lowercase_rust`

#[derive(Clone, Copy, Debug)]
pub struct Record·a·b<A, B> {
    pub a: A,
    pub b: B,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·left·right<Left, Right> {
    pub left: Left,
    pub right: Right,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·i·u<I, U> {
    pub i: I,
    pub u: U,
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
pub struct Record·base·exponent<Base, Exponent> {
    pub base: Base,
    pub exponent: Exponent,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·fnø·inø<Fn, In> {
    pub fnø: Fn,
    pub inø: In,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·max·min<Max, Min> {
    pub max: Max,
    pub min: Min,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·part·rest<Part, Rest> {
    pub part: Part,
    pub rest: Rest,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·change·isolated<Change, Isolated> {
    pub change: Change,
    pub isolated: Isolated,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·element_isolate<Buf, Element_isolate> {
    pub buf: Buf,
    pub element_isolate: Element_isolate,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·element_unerase·uneraser<Buf, Element_unerase, Uneraser> {
    pub buf: Buf,
    pub element_unerase: Element_unerase,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·erase·value<Erase, Value> {
    pub erase: Erase,
    pub value: Value,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·erased·origin·unerase<Erased, Origin, Unerase> {
    pub erased: Erased,
    pub origin: Origin,
    pub unerase: Unerase,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·erased·rid<Erased, Rid> {
    pub erased: Erased,
    pub rid: Rid,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·erased·uneraser<Erased, Uneraser> {
    pub erased: Erased,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·unerased·uneraser<Unerased, Uneraser> {
    pub unerased: Unerased,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·slot·uneraser<Slot, Uneraser> {
    pub slot: Slot,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·span·uneraser<Span, Uneraser> {
    pub span: Span,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·element·in<Element, In> {
    pub element: Element,
    pub in_: In,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·element·uneraser<Element, Uneraser> {
    pub element: Element,
    pub uneraser: Uneraser,
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
pub struct Record·buf·length<Buf, Length> {
    pub buf: Buf,
    pub length: Length,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·slot<Buf, Slot> {
    pub buf: Buf,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·new<Buf, New> {
    pub buf: Buf,
    pub new: New,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·new·slot<Buf, New, Slot> {
    pub buf: Buf,
    pub new: New,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·span<Buf, Span> {
    pub buf: Buf,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·new·span<Buf, New, Span> {
    pub buf: Buf,
    pub new: New,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·new·span<New, Span> {
    pub new: New,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·source·source_span·span<Buf, Source, Source_span, Span> {
    pub buf: Buf,
    pub source: Source,
    pub source_span: Source_span,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·out·slot<Buf, Out, Slot> {
    pub buf: Buf,
    pub out: Out,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·out·span<Out, Span> {
    pub out: Out,
    pub span: Span,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·end·start<Buf, End, Start> {
    pub buf: Buf,
    pub end: End,
    pub start: Start,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·element<Buf, Element> {
    pub buf: Buf,
    pub element: Element,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·uneraser<Buf, Uneraser> {
    pub buf: Buf,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·element·slot<Buf, Element, Slot> {
    pub buf: Buf,
    pub element: Element,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·old·slot<Buf, Old_element, Slot> {
    pub buf: Buf,
    pub old: Old_element,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·direction·state·step·str<Direction, State, Step, Str> {
    pub direction: Direction,
    pub state: State,
    pub step: Step,
    pub str: Str,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·direction·state·step<Buf, Direction, State, Step> {
    pub buf: Buf,
    pub direction: Direction,
    pub state: State,
    pub step: Step,
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
pub enum Choice·No·Yes<No, Yes> {
    No(No),
    Yes(Yes),
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
pub enum Choice·Equal·Greater·Less<Equal, Greater, Less> {
    Equal(Equal),
    Greater(Greater),
    Less(Less),
}

/// empty record, represented as unit
pub type Record = ();
/// empty choice. Should be changed to the never type once stabilized
#[derive(Clone, Copy, Debug)]
pub enum Choice {}

pub type P32 = std::num::NonZeroU32;
pub type U32 = u32;
pub type I32 = i32;
pub type F32 = f32;
pub type Char = char;
#[derive(
    Debug,
    Clone,
    Copy,
    std::cmp::PartialEq,
    std::cmp::Eq,
    std::cmp::PartialOrd,
    std::cmp::Ord,
    std::hash::Hash,
)]
#[non_exhaustive]
pub struct Str {
    /// known to contain at least 1 char and at most u32::MAX bytes
    // I already tried to split it into start:char, after:&str
    // but then it can't be passed to functions, Cow, etc. that expect a single consecutive &str
    str: &'static str,
}
pub type Fn<In, Out> = fn(In) -> Out;
pub type Order = Choice·Equal·Greater·Less<Record, Record, Record>;
pub type Opt<Yes> = Choice·No·Yes<Record, Yes>;
pub type Part_rest<Part, Rest> = Record·part·rest<Part, Rest>;

#[derive(Debug)]
#[non_exhaustive]
pub struct Origin<LocalOrigin, Part>(std::marker::PhantomData<(LocalOrigin, Part)>);

#[derive(Debug)]
pub enum Erased {}
#[derive(Debug)]
#[non_exhaustive]
pub struct Origin_erased<Value_erased> {
    value_erased: Value_erased,
}
#[derive(Debug)]
#[non_exhaustive]
pub struct Origin_isolated<Origin, Value_erased> {
    pub origin: std::marker::PhantomData<Origin>,
    value_erased: Value_erased,
}
#[derive(Debug)]
#[non_exhaustive]
pub struct Origin_uneraser<Origin>(std::marker::PhantomData<Origin>);

pub struct Unset_slice<Element>(std::boxed::Box<[std::mem::MaybeUninit<Element>]>);
#[derive(Debug)]
#[non_exhaustive]
pub struct Buf_origin_erased<Part, Element> {
    erased: Buf<Origin<Erased, Part>, Element>,
}
#[derive(Debug)]
pub struct Buf<LocalOrigin, Element> {
    // invariants (in addition to the invariants of (Unset_)slot/span):
    // - no `Unset_span`s in `.vacant` are connected
    //   (and thus could be combined into one larger consecutive span)
    // - any index contained in any vacant `Unset_span` is less than elements.len()
    // - any index contained in any vacant `Unset_span` should be assumed uninitialized
    //   in `.elements`
    //
    // -------
    // `.elements` contains `std::mem::MaybeUninit<Element>` because
    // - functions like `buf.add_unset` explicitly require uninitialized memory.
    //   creating uninitialized memory of type `Element` out of thin air is UB
    // - it matches well semantically: access is inherently unsafe.
    //   vec::Vec<Element> makes it appear safe
    // - drawbacks (like the removal of niches) do not have an impact here
    // - it prevents drop from being called on elements
    //   which could double-free on already vacated elements.
    //   Buf originally implemented a custom Drop as
    //   `for e in self.elements.drain(..) { std::mem::forget(e); }`
    //   with the following documentation:
    //     At this point, all elements are either
    //     - handled (in sloe code this is always the case or you'll get an error)
    //     - unhandled (only possible from rust code when a `Slot`/`Span` is dropped)
    //     - empty (only possible from rust code when a `Unset_span`/`Unset_span` is dropped)
    //     - occupied (only possible from rust code).
    //
    //     If we used the regular Drop implementation, elements that were already vacated
    //     or temporarily extracted (where e.g. the resulting `Unset_slot` from `buf.unset()` was dropped)
    //     could be freed twice (!).
    //     So the only thing that can realistically be done is to "leak" all remaining elements.
    //
    //     To recap, if some rust code kept some slots occupied,
    //     we _must_ prevent double-frees by leaking those elements.
    //     This is not as bad as you might think:
    //     - dropping a `Slot`/`Unset_slot` is always a leak
    //       but it cannot reasonably prevented in rust. It's the cost of doing business
    //     - in a `Buf<Origin, Element>`, the element type will realistically not be a type that
    //       directly points to the heap. In fact in sloe you cannot even put more than one buf inside of
    //       another buf as each buf has a different origin!
    //
    //   However, just overwriting the Drop implementation is far from enough
    //   as many Buf functions somewhat willy-nilly drop elements if you're not careful.
    //   An example is `truncate` which is used in `unset_span_rid`.
    elements: std::vec::Vec<std::mem::MaybeUninit<Element>>,
    // Performance assumption:
    // Neighboring elements are way more likely to be vacated together.
    // Think e.g. buf_span_add_buf_span but also
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

impl Order {
    pub fn from_ordering(order: std::cmp::Ordering) -> Order {
        match order {
            std::cmp::Ordering::Less => Order::Less(()),
            std::cmp::Ordering::Equal => Order::Equal(()),
            std::cmp::Ordering::Greater => Order::Greater(()),
        }
    }
    pub fn to_ordering(self) -> std::cmp::Ordering {
        match self {
            Order::Less(()) => std::cmp::Ordering::Less,
            Order::Equal(()) => std::cmp::Ordering::Equal,
            Order::Greater(()) => std::cmp::Ordering::Greater,
        }
    }
}

impl<Yes> Opt<Yes> {
    pub fn from_option(option: std::option::Option<Yes>) -> Self {
        match option {
            std::option::Option::None => Opt::No(()),
            std::option::Option::Some(yes) => Opt::Yes(yes),
        }
    }
    pub fn into_option(self) -> std::option::Option<Yes> {
        match self {
            Opt::No(()) => std::option::Option::None,
            Opt::Yes(yes) => std::option::Option::Some(yes),
        }
    }
    pub fn as_ref(&self) -> Opt<&Yes> {
        match self {
            Opt::No(()) => Opt::No(()),
            Opt::Yes(yes) => Opt::Yes(yes),
        }
    }
    pub fn as_mut(&mut self) -> Opt<&mut Yes> {
        match self {
            Opt::No(()) => Opt::No(()),
            Opt::Yes(yes) => Opt::Yes(yes),
        }
    }
    pub fn map<NewYes>(self, yes_change: impl std::ops::Fn(Yes) -> NewYes) -> Opt<NewYes> {
        match self {
            Opt::No(()) => Opt::No(()),
            Opt::Yes(yes) => Opt::Yes(yes_change(yes)),
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

impl Str {
    /// Using .unwrap() if given a non-empty string literal that is okay
    pub const fn from_str(static_str: &'static str) -> std::option::Option<Str> {
        if static_str.is_empty()
            || // <u32 as std::convert::TryFrom<usize>>::try_from(static_str.len()).is_err() isn't const, yet
                static_str.len() > U32::MAX as usize
        {
            std::option::Option::None
        } else {
            std::option::Option::Some(Str { str: static_str })
        }
    }
    pub const fn as_str(self) -> &'static str {
        self.str
    }
    pub fn byte_len(self) -> std::num::NonZeroUsize {
        unsafe { std::num::NonZeroUsize::new_unchecked(self.str.len()) }
    }
    pub fn char_count(self) -> std::num::NonZeroUsize {
        unsafe {
            std::num::NonZeroUsize::new_unchecked(std::iter::Iterator::count(self.str.chars()))
        }
    }
    pub fn split_start(self) -> (char, &'static str) {
        let mut chars = self.str.chars();
        (
            unsafe { std::iter::Iterator::next(&mut chars).unwrap_unchecked() },
            chars.as_str(),
        )
    }
    pub fn split_end(self) -> (char, &'static str) {
        let mut chars = self.str.chars();
        (
            unsafe { std::iter::DoubleEndedIterator::next_back(&mut chars).unwrap_unchecked() },
            chars.as_str(),
        )
    }
}

impl<LocalOrigin, Part> Origin<LocalOrigin, Part> {
    /// Safe if no other origin exists with the same LocalOrigin and Part.
    /// LocalOrigin is usually a local type without values and Part a global type
    ///
    /// This constructor is exposed because sadly macros (namely origin_new!) require it.
    /// It's _strongly_ recommended to only construct new origins with `origin_new!`.
    /// Misusing this constructor can lead to UB like unchecked out of bounds access.
    pub const unsafe fn new() -> Origin<LocalOrigin, Part> {
        Origin(std::marker::PhantomData::<(LocalOrigin, Part)>)
    }
}
/// To create multiple Origins with the same unique origin type
/// ```ignore
/// origin_new!(some, UniqueOrigin, Record·part_a, Record·part_b)
/// ```
/// (only works if there is actually such a record in the generated code)
///
/// If you don't, use the simpler
/// ```ignore
/// origin_new!(variable_name, LocalOriginName)
/// ```
///
/// Careful!
/// ```ignore
/// origin_new!(some, Origin, Record·not_origin)
/// ```
/// wil crash **at runtime** when some field names overlap.
/// This is to prevent multiple origins with the same name type being created.
/// In theory, it should be possible to report a compile-error in that case,
/// however, there seems to neither exist const == on &str, nor const panic, nor ident concat etc.
/// I'm sorry :3
#[macro_export]
macro_rules! origin_new {
    ($variable_name:ident, $type_name:ident) => {
        struct $type_name;
        let $variable_name: $crate::core::Origin::<$type_name, $crate::core::Record> = unsafe {
            $crate::core::Origin::new()
        };
    };
    ($type_name:ident, $part0_name:ident, $($parts:ident),+) => {
        if part_record_names_are_invalid!($($parts),+) {
            panic!("invalid part names. Each part needs a unique name and must start with Record·!");
        }
        struct $type_name;
        origin_new_variables!($type_name, unsafe { $crate::core::Origin::new() }, $part0_name, $($parts),+)
    };
}
pub use origin_new;
/// only for use inside other macro_rules!
#[macro_export]
macro_rules! origin_new_variables {
    ($type_name:ident, $assigned_value:expr) => {};
    ($type_name:ident, $assigned_value:expr, $part0_variable:ident, $part0:ident) => {
        let $part0_variable: $crate::core::Origin::<$type_name, $crate::core::$part0<$crate::core::Record>> = $assigned_value;
    };
    ($type_name:ident, $assigned_value:expr, $part0_variable:ident, $part0:ident, $($part1_up:ident),*) => {
        let $part0_variable: $crate::core::Origin::<$type_name, $crate::core::$part0<$crate::core::Record>> = $assigned_value;
        origin_new_variables!($type_name, $assigned_value, $($part1_up),*);
    };
}
pub use origin_new_variables;
/// only for use inside other macro_rules!
#[macro_export]
macro_rules! part_record_names_are_invalid {
    () => { false };
    ($i:ident) => { !stringify!($a).starts_with("Record·") };
    ($a:ident, $_:ident, $b:ident) => {
        stringify!($a) == stringify!($b)
            || !stringify!($a).starts_with("Record·")
            || !stringify!($b).starts_with("Record·")
    };
    ($a:ident, $_:ident, $b:ident, $($rest:ident),*) => {
        stringify!($a) == stringify!($b)
            || !stringify!($a).starts_with("Record·")
            || !stringify!($b).starts_with("Record·")
            || part_record_names_are_invalid!($a, $($rest),*)
            || part_record_names_are_invalid!($b, $($rest),*)
    };
}

impl<Element> Unset_slice<Element> {
    pub fn allocate_length(length: u32) -> Self {
        Unset_slice(std::boxed::Box::new_uninit_slice(length as usize))
    }
    pub fn from_buf_maybe_uninit(
        mut maybe_uninit_buf: std::vec::Vec<std::mem::MaybeUninit<Element>>,
    ) -> Self {
        // This is the closest approximation for `vec.ptr[..vec.capacity]` I could find in safe rust.
        // The first part should optimize to maybe_uninit_buf.set_len(maybe_uninit_buf.capacity())
        // If it doesn't, change to that unsafe operation.
        // Preferably there would be something like `vec.clear(); vec.into_spare_capacity()`
        let spare_capacity = maybe_uninit_buf.spare_capacity_mut().len();
        std::iter::Extend::extend(
            &mut maybe_uninit_buf,
            std::iter::Iterator::take(
                std::iter::repeat_with(|| std::mem::MaybeUninit::uninit()),
                spare_capacity,
            ),
        );
        Unset_slice(maybe_uninit_buf.into_boxed_slice())
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
        let (buf_ptr, buf_length, buf_capacity) = vec.into_raw_parts();
        unsafe {
            std::vec::Vec::from_raw_parts(buf_ptr.cast::<Element>(), buf_length, buf_capacity)
        }
    }
    pub fn into_buf_maybe_uninit(self) -> std::vec::Vec<std::mem::MaybeUninit<Element>> {
        let mut vec: std::vec::Vec<std::mem::MaybeUninit<Element>> =
            self.into_boxed_slice().into_vec();
        vec.clear();
        vec
    }
    pub fn leak<'a>(self) -> &'a mut [std::mem::MaybeUninit<Element>] {
        std::boxed::Box::leak(self.into_boxed_slice())
    }
}

impl<Element, LocalOrigin> Buf<LocalOrigin, Element> {
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
            Opt::No(()) => &[],
            Opt::Yes(span) => self.span_slice(span),
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
            Opt::No(()) => &mut [],
            Opt::Yes(span) => self.span_slice_mut(span),
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
        self.unset_slot_rid(element.slot);
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
    pub fn unset_slot_rid(&mut self, slot_to_vacate: Unset_slot<LocalOrigin>) {
        // can maybe be optimized
        self.unset_span_rid(slot_to_vacate.to_span());
    }
    pub fn opt_unset_span_rid(&mut self, span_to_vacate: Opt<Unset_span<LocalOrigin>>) {
        if let Opt::Yes(span_to_vacate) = span_to_vacate {
            self.unset_span_rid(span_to_vacate);
        }
    }
    pub fn unset_span_rid(&mut self, span_to_vacate: Unset_span<LocalOrigin>) {
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
            std::option::Option::None => Opt::No(()),
            std::option::Option::Some(length) => {
                let span = self.add_unset_length_positive(length);
                Opt::Yes(span)
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
            std::option::Option::None => Opt::No(()),
            std::option::Option::Some(new_length) => Opt::Yes(Span {
                start: Slot::from_index(length_without_new_elements as u32),
                length: new_length,
            }),
        }
    }
    pub fn add_one_then_iterator(
        &mut self,
        new_start: Element,
        new_after: impl std::iter::Iterator<Item = Element>,
    ) -> Span<LocalOrigin> {
        let length_before_new = self.elements.len();
        self.elements.push(std::mem::MaybeUninit::new(new_start));
        std::iter::Extend::extend(
            &mut self.elements,
            new_after.map(std::mem::MaybeUninit::new),
        );
        Span {
            start: Slot::from_index(length_before_new as u32),
            length: std::num::NonZeroU32::MIN
                .saturating_add((self.elements.len() - (length_before_new + 1)) as u32),
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
            std::option::Option::None => Opt::No(()),
            std::option::Option::Some(new_element_count) => {
                Opt::Yes(self.insert_iterator_filled(new_elements, new_element_count))
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
            return Opt::No(());
        };
        let new_span = self.insert_iterator_filled(new_elements, new_length);
        Opt::Yes(new_span)
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
    pub fn insert_buf_span<SourceOrigin>(
        &mut self,
        source: &mut Buf<SourceOrigin, Element>,
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
    pub fn add_buf_span<SourceOrigin>(
        &mut self,
        source: &mut Buf<SourceOrigin, Element>,
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
    pub fn span_add_buf_span<SourceOrigin>(
        &mut self,
        span: Span<LocalOrigin>,
        source: &mut Buf<SourceOrigin, Element>,
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
    pub fn span_add_buf_opt_span<SourceOrigin>(
        &mut self,
        span: Span<LocalOrigin>,
        source: &mut Buf<SourceOrigin, Element>,
        source_span: Opt<Span<SourceOrigin>>,
    ) -> (Opt<Unset_span<SourceOrigin>>, Span<LocalOrigin>) {
        match source_span {
            Opt::No(()) => (Opt::No(()), span),
            Opt::Yes(source_span) => {
                let (source_span, combined_span) =
                    self.span_add_buf_span(span, source, source_span);
                (Opt::Yes(source_span), combined_span)
            }
        }
    }
    pub fn opt_span_add_buf_span<SourceOrigin>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        source: &mut Buf<SourceOrigin, Element>,
        source_span: Span<SourceOrigin>,
    ) -> (Unset_span<SourceOrigin>, Span<LocalOrigin>) {
        match span {
            Opt::No(()) => self.add_buf_span(source, source_span),
            Opt::Yes(span) => self.span_add_buf_span(span, source, source_span),
        }
    }
    pub fn opt_span_add_buf_opt_span<SourceOrigin>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        source: &mut Buf<SourceOrigin, Element>,
        source_span: Opt<Span<SourceOrigin>>,
    ) -> (Opt<Unset_span<SourceOrigin>>, Opt<Span<LocalOrigin>>) {
        match source_span {
            Opt::No(()) => (Opt::No(()), span),
            Opt::Yes(source_span) => {
                let (source_span, combined_span) =
                    self.opt_span_add_buf_span(span, source, source_span);
                (Opt::Yes(source_span), Opt::Yes(combined_span))
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
    pub fn opt_span_add_iterator(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        new_elements: impl std::iter::Iterator<Item = Element>,
    ) -> Opt<Span<LocalOrigin>> {
        match span {
            Opt::No(()) => self.add_iterator(new_elements),
            Opt::Yes(span) => Opt::Yes(self.span_add_iterator(span, new_elements)),
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
            Opt::No(()) => self.add_array(new_elements),
            Opt::Yes(span) => self.span_add_array(span, new_elements),
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
        self.unset_span_rid(Unset_span {
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
            Opt::No(()) => start,
            Opt::Yes(end) => self.span_add_own_span(start, end),
        }
    }
    pub fn opt_span_add_own_span(
        &mut self,
        start: Opt<Span<LocalOrigin>>,
        end: Span<LocalOrigin>,
    ) -> Span<LocalOrigin> {
        match start {
            Opt::No(()) => end,
            Opt::Yes(start) => self.span_add_own_span(start, end),
        }
    }
    pub fn opt_span_add_own_opt_span(
        &mut self,
        start: Opt<Span<LocalOrigin>>,
        end: Opt<Span<LocalOrigin>>,
    ) -> Opt<Span<LocalOrigin>> {
        match start {
            Opt::No(()) => end,
            Opt::Yes(start) => Opt::Yes(self.span_add_own_opt_span(start, end)),
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
            Opt::No(()) => start,
            Opt::Yes(end) => self.unset_span_add_own_span(start, end),
        }
    }
    pub fn opt_unset_span_add_own_span(
        &mut self,
        start: Opt<Unset_span<LocalOrigin>>,
        end: Unset_span<LocalOrigin>,
    ) -> Unset_span<LocalOrigin> {
        match start {
            Opt::No(()) => end,
            Opt::Yes(start) => self.unset_span_add_own_span(start, end),
        }
    }
    pub fn opt_unset_span_add_own_opt_span(
        &mut self,
        start: Opt<Unset_span<LocalOrigin>>,
        end: Opt<Unset_span<LocalOrigin>>,
    ) -> Opt<Unset_span<LocalOrigin>> {
        match start {
            Opt::No(()) => end,
            Opt::Yes(start) => Opt::Yes(self.unset_span_add_own_opt_span(start, end)),
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
        Unset_slice::from_buf_maybe_uninit(self.elements)
    }
}
impl<Origin> Buf<Origin, Char> {
    pub fn add_str(&mut self, new_str: Str) -> Span<Origin> {
        let (new_start, new_after) = new_str.split_start();
        self.add_one_then_iterator(new_start, new_after.chars())
    }
    pub fn opt_span_add_str(&mut self, span: Opt<Span<Origin>>, new_str: Str) -> Span<Origin> {
        match span {
            Opt::No(()) => self.add_str(new_str),
            Opt::Yes(span) => self.span_add_iterator(span, new_str.str.chars()),
        }
    }
}
impl<Element, LocalOrigin, Part> Buf<Origin<LocalOrigin, Part>, Element> {
    pub fn new(_: Origin<LocalOrigin, Part>) -> Self {
        Buf::<Origin<LocalOrigin, Part>, Element> {
            elements: std::vec::Vec::new(),
            vacant: std::vec::Vec::new(),
        }
    }
    pub fn reuse(_: Origin<LocalOrigin, Part>, allocation: Unset_slice<Element>) -> Self {
        Buf::<Origin<LocalOrigin, Part>, Element> {
            elements: allocation.into_buf_maybe_uninit(),
            vacant: std::vec::Vec::new(),
        }
    }
    /// safe if no Unset_slot or Unset_span into the Buf with the same origin exists
    /// at this point in time
    pub unsafe fn origin_isolate_assume_no_unset<ElementErased>(
        self,
        element_erase: impl std::ops::Fn(Element) -> Origin_isolated<LocalOrigin, ElementErased>,
    ) -> Origin_isolated<LocalOrigin, Buf_origin_erased<Part, ElementErased>> {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: Buf_origin_erased {
                erased: Buf {
                    // the optimizer should be able to figure out that the atual memory does not change here
                    // when the `element_erase` really justs erases origins.
                    // If it can't, look into branching on if ElementErased has the same size and alignment
                    // and transmute the vacant elements instead of ::uninit()
                    elements: std::iter::Iterator::collect(std::iter::Iterator::map(
                        std::iter::Iterator::enumerate(std::iter::IntoIterator::into_iter(
                            self.elements,
                        )),
                        |(element_index, element)| {
                            if std::iter::Iterator::any(&mut self.vacant.iter(), |vacant_range| {
                                vacant_range.to_range().contains(&element_index)
                            }) {
                                std::mem::MaybeUninit::uninit()
                            } else {
                                std::mem::MaybeUninit::new(
                                    element_erase(unsafe { element.assume_init() }).value_erased,
                                )
                            }
                        },
                    )),
                    vacant: std::iter::Iterator::collect(std::iter::Iterator::map(
                        std::iter::IntoIterator::into_iter(self.vacant),
                        |vacant_span| Unset_span {
                            start: Unset_slot::<Origin<Erased, Part>>::from_index(
                                vacant_span.start.index,
                            ),
                            length: vacant_span.length,
                        },
                    )),
                },
            },
        }
    }
}
impl<Element, Part> Buf<Origin<Erased, Part>, Element> {
    pub fn origin_unerase<LocalOrigin, ElementUnerased>(
        self,
        uneraser: &Origin_uneraser<LocalOrigin>,
        element_unerase: impl std::ops::Fn(
            Element,
            Origin_uneraser<LocalOrigin>,
        ) -> (ElementUnerased, Origin_uneraser<LocalOrigin>),
    ) -> Buf<Origin<LocalOrigin, Part>, ElementUnerased> {
        Buf {
            // the optimizer should be able to figure out that the atual memory does not change here
            // when the `element_unerase` really justs unerases origins.
            // If it can't, look into branching on if ElementUnerased has the same size and alignment
            // and transmute the vacant elements instead of ::uninit()
            elements: std::iter::Iterator::collect(std::iter::Iterator::map(
                std::iter::Iterator::enumerate(std::iter::IntoIterator::into_iter(self.elements)),
                |(element_index, element)| {
                    if std::iter::Iterator::any(&mut self.vacant.iter(), |vacant_range| {
                        vacant_range.to_range().contains(&element_index)
                    }) {
                        std::mem::MaybeUninit::uninit()
                    } else {
                        std::mem::MaybeUninit::new(
                            element_unerase(
                                // Unset_slot<Erased> and Unset_span<Erased> cannot be created
                                // outside of this module
                                unsafe { element.assume_init() },
                                Origin_uneraser(uneraser.0),
                            )
                            .0,
                        )
                    }
                },
            )),
            vacant: std::iter::Iterator::collect(std::iter::Iterator::map(
                std::iter::IntoIterator::into_iter(self.vacant),
                |vacant_span| Unset_span {
                    start: Unset_slot::<Origin<LocalOrigin, Part>>::from_index(
                        vacant_span.start.index,
                    ),
                    length: vacant_span.length,
                },
            )),
        }
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
    pub fn to_span(self) -> Span_with_occupancy<Origin, Occupancy> {
        Span_with_occupancy {
            start: self,
            length: std::num::NonZeroU32::MIN,
        }
    }
}
impl<LocalOrigin, Part> Slot<Origin<LocalOrigin, Part>> {
    pub fn origin_isolate(self) -> Origin_isolated<LocalOrigin, Slot<Origin<Erased, Part>>> {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: Slot::<Origin<Erased, Part>>::from_index(self.index),
        }
    }
}
impl<Part> Slot<Origin<Erased, Part>> {
    pub fn origin_unerase<LocalOrigin>(
        self,
        _: &Origin_uneraser<LocalOrigin>,
    ) -> Slot<Origin<LocalOrigin, Part>> {
        Slot {
            origin: std::marker::PhantomData::<Origin<LocalOrigin, Part>>,
            occupancy: self.occupancy,
            index: self.index,
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
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(after_length) => Opt::Yes(Span_with_occupancy {
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
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(before_length) => Opt::Yes(Span_with_occupancy {
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
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(after_length) => {
                    Opt::Yes(Span_with_occupancy::<Origin, Occupancy> {
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
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(before_length) => {
                    Opt::Yes(Span_with_occupancy::<Origin, Occupancy> {
                        start: self.start,
                        length: before_length,
                    })
                }
            },
        }
    }
}
impl<LocalOrigin, Part> Span<Origin<LocalOrigin, Part>> {
    pub fn origin_isolate(self) -> Origin_isolated<LocalOrigin, Span<Origin<Erased, Part>>> {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: Span {
                start: self.start.origin_isolate().value_erased,
                length: self.length,
            },
        }
    }
}
impl<Part> Span<Origin<Erased, Part>> {
    pub fn origin_unerase<LocalOrigin>(
        self,
        uneraser: &Origin_uneraser<LocalOrigin>,
    ) -> Span<Origin<LocalOrigin, Part>> {
        Span {
            start: self.start.origin_unerase(uneraser),
            length: self.length,
        }
    }
}

impl<Origin, Occupancy> Opt<&Span_with_occupancy<Origin, Occupancy>> {
    pub fn to_range(self) -> std::ops::Range<usize> {
        match self {
            Opt::No(()) => <std::ops::Range<usize> as std::default::Default>::default(),
            Opt::Yes(span) => span.to_range(),
        }
    }
    pub fn to_range_u32(self) -> std::ops::Range<u32> {
        match self {
            Opt::No(()) => <std::ops::Range<u32> as std::default::Default>::default(),
            Opt::Yes(span) => span.to_range_u32(),
        }
    }
    pub fn length(self) -> u32 {
        match self {
            Opt::No(()) => 0,
            Opt::Yes(span) => span.length.get(),
        }
    }
}
impl<LocalOrigin, Part> Opt<Span<Origin<LocalOrigin, Part>>> {
    pub fn origin_isolate(self) -> Origin_isolated<LocalOrigin, Opt<Span<Origin<Erased, Part>>>> {
        match self {
            Opt::No(()) => Origin_isolated::constant(Opt::No),
            Opt::Yes(span) => span.origin_isolate().map(Opt::Yes),
        }
    }
}
impl<Part> Opt<Span<Origin<Erased, Part>>> {
    pub fn origin_unerase<LocalOrigin>(
        self,
        uneraser: &Origin_uneraser<LocalOrigin>,
    ) -> Opt<Span<Origin<LocalOrigin, Part>>> {
        match self {
            Opt::No(()) => Opt::No(()),
            Opt::Yes(span) => Opt::Yes(span.origin_unerase(uneraser)),
        }
    }
}

impl<LocalOrigin, ValueErased> Origin_isolated<LocalOrigin, ValueErased> {
    pub fn constant(value: fn(()) -> ValueErased) -> Self {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: value(()),
        }
    }
    pub fn map<NewValueErased>(
        self,
        change: impl std::ops::Fn(ValueErased) -> NewValueErased,
    ) -> Origin_isolated<LocalOrigin, NewValueErased> {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: change(self.value_erased),
        }
    }
    pub fn merge<OtherValueErased>(
        self,
        other: Origin_isolated<LocalOrigin, OtherValueErased>,
    ) -> Origin_isolated<LocalOrigin, Record·a·b<ValueErased, OtherValueErased>> {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: Record·a·b {
                a: self.value_erased,
                b: other.value_erased,
            },
        }
    }
    pub fn erase(self) -> Origin_erased<ValueErased> {
        Origin_erased {
            value_erased: self.value_erased,
        }
    }
    pub fn unisolate<Value>(
        self,
        unisolate: impl std::ops::Fn(
            ValueErased,
            Origin_uneraser<LocalOrigin>,
        ) -> (Value, Origin_uneraser<LocalOrigin>),
    ) -> Value {
        unisolate(
            self.value_erased,
            Origin_uneraser(std::marker::PhantomData::<LocalOrigin>),
        )
        .0
    }
}
impl<ValueErased> Origin_erased<ValueErased> {
    pub fn unerase<LocalOrigin>(
        self,
        _: Origin<LocalOrigin, Record>,
    ) -> Origin_isolated<LocalOrigin, ValueErased> {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: self.value_erased,
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
pub fn p32_mul_clamp(Record·a·b { a, b }: Record·a·b<P32, P32>) -> P32 {
    a.saturating_mul(b)
}
pub fn p32_to_u32(n: P32) -> U32 {
    n.get()
}
pub fn p32_order(Record·left·right { left, right }: Record·left·right<P32, P32>) -> Order {
    Order::from_ordering(std::cmp::Ord::cmp(&left, &right))
}
pub fn p32_origin_isolate<Origin>(n: P32) -> Origin_isolated<Origin, P32> {
    Origin_isolated {
        origin: std::marker::PhantomData::<Origin>,
        value_erased: n,
    }
}
pub fn u32_to_p32(n: U32) -> Opt<P32> {
    Opt::from_option(P32::new(n))
}
pub fn u32_rid(_: U32) -> Record {}
pub fn u32_dup(n: U32) -> Record·a·b<U32, U32> {
    Record·a·b { a: n, b: n }
}
#[expect(clippy::cast_precision_loss)]
pub fn u32_round_to_nearest_f32_else_even(n: U32) -> F32 {
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
pub fn u32_add_i32_clamp(Record·i·u { i, u }: Record·i·u<I32, U32>) -> U32 {
    u.saturating_add_signed(i)
}
pub fn u32_mul_clamp(Record·a·b { a, b }: Record·a·b<U32, U32>) -> U32 {
    a.saturating_mul(b)
}
pub fn u32_pow_clamp(
    Record·base·exponent { base, exponent }: Record·base·exponent<U32, P32>,
) -> U32 {
    base.saturating_pow(exponent.get())
}
pub fn u32_successor_clamp(n: U32) -> P32 {
    P32::MIN.saturating_add(n)
}
pub fn u32_to_i32_clamp(n: U32) -> I32 {
    <I32 as std::convert::TryFrom<U32>>::try_from(n).unwrap_or(I32::MAX)
}
pub fn u32_order(Record·left·right { left, right }: Record·left·right<U32, U32>) -> Order {
    Order::from_ordering(std::cmp::Ord::cmp(&left, &right))
}
pub fn u32_origin_isolate<Origin>(n: U32) -> Origin_isolated<Origin, U32> {
    Origin_isolated {
        origin: std::marker::PhantomData::<Origin>,
        value_erased: n,
    }
}
pub fn i32_dup(n: I32) -> Record·a·b<I32, I32> {
    Record·a·b { a: n, b: n }
}
pub fn i32_rid(_: I32) -> Record {}
#[expect(clippy::cast_precision_loss)]
pub fn i32_round_to_nearest_f32_else_even(n: I32) -> F32 {
    n as F32
}
pub fn i32_to_u32(n: I32) -> Opt<U32> {
    match <U32 as std::convert::TryFrom<I32>>::try_from(n) {
        std::result::Result::Err(_) => Opt::No(()),
        std::result::Result::Ok(u) => Opt::Yes(u),
    }
}
pub fn i32_abs_to_u32(n: I32) -> U32 {
    n.unsigned_abs()
}
pub fn i32_negate_clamp(n: I32) -> I32 {
    n.saturating_neg()
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
pub fn i32_pow_clamp(
    Record·base·exponent { base, exponent }: Record·base·exponent<I32, P32>,
) -> I32 {
    base.saturating_pow(exponent.get())
}
pub fn i32_order(Record·left·right { left, right }: Record·left·right<I32, I32>) -> Order {
    Order::from_ordering(std::cmp::Ord::cmp(&left, &right))
}
pub fn i32_origin_isolate<Origin>(n: I32) -> Origin_isolated<Origin, I32> {
    Origin_isolated {
        origin: std::marker::PhantomData::<Origin>,
        value_erased: n,
    }
}
pub fn f32_dup(n: F32) -> Record·a·b<F32, F32> {
    Record·a·b { a: n, b: n }
}
pub fn f32_rid(_: F32) -> Record {}
pub fn f32_pi((): Record) -> F32 {
    return std::f32::consts::PI;
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
pub fn f32_pow_i32(
    Record·base·exponent { base, exponent }: Record·base·exponent<F32, I32>,
) -> Opt<F32> {
    let power = base.powi(exponent);
    if power.is_finite() {
        Opt::Yes(power)
    } else {
        Opt::No(())
    }
}
pub fn f32_pow(
    Record·base·exponent { base, exponent }: Record·base·exponent<F32, F32>,
) -> Opt<F32> {
    let power = base.powf(exponent);
    if power.is_finite() {
        Opt::Yes(power)
    } else {
        Opt::No(())
    }
}
pub fn f32_abs(n: F32) -> F32 {
    n.abs()
}
pub fn f32_negate(n: F32) -> F32 {
    -n
}
pub fn f32_ln(n: F32) -> Opt<F32> {
    if n <= 0.0 {
        Opt::No(())
    } else if let ln_result = n.ln()
        && ln_result.is_finite()
    {
        Opt::Yes(ln_result)
    } else {
        Opt::No(())
    }
}
pub fn f32_exp(n: F32) -> F32 {
    n.exp().min(F32::MAX)
}
pub fn f32_sin(radians: F32) -> F32 {
    radians.sin()
}
pub fn f32_cos(radians: F32) -> F32 {
    radians.cos()
}
pub fn f32_tan(radians: F32) -> F32 {
    radians.tan()
}
pub fn f32_atan(radians: F32) -> F32 {
    radians.atan()
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
pub fn f32_order(Record·left·right { left, right }: Record·left·right<F32, F32>) -> Order {
    Order::from_ordering(
        // should always succeed because F32 is assumed to be finite
        <F32 as std::cmp::PartialOrd>::partial_cmp(&left, &right)
            .unwrap_or(std::cmp::Ordering::Greater),
    )
}
pub fn f32_origin_isolate<Origin>(n: F32) -> Origin_isolated<Origin, F32> {
    Origin_isolated {
        origin: std::marker::PhantomData::<Origin>,
        value_erased: n,
    }
}

pub fn fn_dup<In, Out>(fn_: Fn<In, Out>) -> Record·a·b<Fn<In, Out>, Fn<In, Out>> {
    Record·a·b { a: fn_, b: fn_ }
}
pub fn fn_rid<In, Out>(_: Fn<In, Out>) -> Record {}
#[inline]
pub fn call<In, Out>(to_call: Record·fnø·inø<Fn<In, Out>, In>) -> Out {
    (to_call.fnø)(to_call.inø)
}
pub fn fn_origin_isolate<In, Origin, Out>(
    function: Fn<In, Out>,
) -> Origin_isolated<Origin, Fn<In, Out>> {
    Origin_isolated {
        origin: std::marker::PhantomData::<Origin>,
        value_erased: function,
    }
}

pub fn char_dup(char: Char) -> Record·a·b<Char, Char> {
    Record·a·b { a: char, b: char }
}
pub fn char_rid(_: Char) -> Record {}
pub fn char_to_u32(char: Char) -> U32 {
    <u32 as std::convert::From<char>>::from(char)
}
pub fn u32_code_point_to_char(code_point: U32) -> Opt<Char> {
    Opt::from_option(char::from_u32(code_point))
}
pub fn char_origin_isolate<Origin>(c: Char) -> Origin_isolated<Origin, Char> {
    Origin_isolated {
        origin: std::marker::PhantomData::<Origin>,
        value_erased: c,
    }
}

pub fn str_dup(str: Str) -> Record·a·b<Str, Str> {
    Record·a·b { a: str, b: str }
}
pub fn str_rid(_: Str) -> Record {}
pub fn str_utf8_length(str: Str) -> P32 {
    <P32 as std::convert::TryFrom<std::num::NonZeroUsize>>::try_from(str.byte_len()).unwrap()
}
pub fn str_char_count(str: Str) -> P32 {
    <P32 as std::convert::TryFrom<std::num::NonZeroUsize>>::try_from(str.char_count()).unwrap()
}
pub fn str_start(str: Str) -> Record·after·start<Opt<Str>, Char> {
    let (start, after) = str.split_start();
    Record·after·start {
        start: start,
        after: Opt::from_option(Str::from_str(after)),
    }
}
pub fn str_end(str: Str) -> Record·before·end<Opt<Str>, Char> {
    let (end, before) = str.split_end();
    Record·before·end {
        end: end,
        before: Opt::from_option(Str::from_str(before)),
    }
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
    iterator_fold_in_direction(str.str.chars(), direction, initial_state, |state, char| {
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
        str.str.chars(),
        direction,
        initial_state,
        |state, char| {
            Choice·Exit·Go_on::into_control_flow(step(Record·char·state { state, char }))
        },
    ))
}
pub fn str_origin_isolate<Origin>(s: Str) -> Origin_isolated<Origin, Str> {
    Origin_isolated {
        origin: std::marker::PhantomData::<Origin>,
        value_erased: s,
    }
}

pub fn opt_yes<Yes>(yes: Yes) -> Opt<Yes> {
    Opt::Yes(yes)
}

pub fn choice_empty_to<Result>(choice_empty: Choice) -> Result {
    match choice_empty {}
}

pub fn slot_index<Origin>(slot: Slot<Origin>) -> Record·index·slot<u32, Slot<Origin>> {
    Record·index·slot {
        index: slot.index,
        slot: slot,
    }
}
pub fn slot_origin_isolate<LocalOrigin, Part>(
    slot: Slot<Origin<LocalOrigin, Part>>,
) -> Origin_isolated<LocalOrigin, Slot<Origin<Erased, Part>>> {
    slot.origin_isolate()
}
pub fn slot_origin_unerase<LocalOrigin, Part>(
    Record·slot·uneraser { slot, uneraser }: Record·slot·uneraser<
        Slot<Origin<Erased, Part>>,
        Origin_uneraser<LocalOrigin>,
    >,
) -> Record·slot·uneraser<Slot<Origin<LocalOrigin, Part>>, Origin_uneraser<LocalOrigin>> {
    let slot = slot.origin_unerase(&uneraser);
    Record·slot·uneraser {
        slot: slot,
        uneraser: uneraser,
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
        Opt::No(()) => Choice·Exit·Go_on::Go_on(initial_state),
        Opt::Yes(span) => span_fold_while(Record·direction·span·state·step {
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
pub fn span_origin_isolate<LocalOrigin, Part>(
    span: Span<Origin<LocalOrigin, Part>>,
) -> Origin_isolated<LocalOrigin, Span<Origin<Erased, Part>>> {
    Origin_isolated {
        origin: std::marker::PhantomData::<LocalOrigin>,
        value_erased: Span {
            start: slot_origin_isolate(span.start).value_erased,
            length: span.length,
        },
    }
}
pub fn span_origin_unerase<LocalOrigin, Part>(
    Record·span·uneraser { span, uneraser }: Record·span·uneraser<
        Span<Origin<Erased, Part>>,
        Origin_uneraser<LocalOrigin>,
    >,
) -> Record·span·uneraser<Span<Origin<LocalOrigin, Part>>, Origin_uneraser<LocalOrigin>> {
    let span = span.origin_unerase(&uneraser);
    Record·span·uneraser {
        span: span,
        uneraser: uneraser,
    }
}
pub fn opt_span_origin_isolate<LocalOrigin, Part>(
    span: Opt<Span<Origin<LocalOrigin, Part>>>,
) -> Origin_isolated<LocalOrigin, Opt<Span<Origin<Erased, Part>>>> {
    span.origin_isolate()
}
pub fn opt_span_origin_unerase<LocalOrigin, Part>(
    Record·span·uneraser { span, uneraser }: Record·span·uneraser<
        Opt<Span<Origin<Erased, Part>>>,
        Origin_uneraser<LocalOrigin>,
    >,
) -> Record·span·uneraser<Opt<Span<Origin<LocalOrigin, Part>>>, Origin_uneraser<LocalOrigin>> {
    let span = span.origin_unerase(&uneraser);
    Record·span·uneraser {
        span: span,
        uneraser: uneraser,
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

pub fn origin_rid<LocalOrigin, Part>(_: Origin<LocalOrigin, Part>) -> Record {}

pub fn origin_isolate_constant<LocalOrigin, ValueErased>(
    value: Fn<Record, ValueErased>,
) -> Origin_isolated<LocalOrigin, ValueErased> {
    Origin_isolated::constant(value)
}
pub fn origin_isolated_map<Erased, NewErased, LocalOrigin>(
    Record·change·isolated { change, isolated }: Record·change·isolated<
        Fn<Erased, NewErased>,
        Origin_isolated<LocalOrigin, Erased>,
    >,
) -> Origin_isolated<LocalOrigin, NewErased> {
    isolated.map(change)
}
pub fn origin_isolated_merge<A, B, LocalOrigin>(
    Record·a·b { a, b }: Record·a·b<
        Origin_isolated<LocalOrigin, A>,
        Origin_isolated<LocalOrigin, B>,
    >,
) -> Origin_isolated<LocalOrigin, Record·a·b<A, B>> {
    a.merge(b)
}
pub fn origin_erase<Origin, ValueErased>(
    isolated: Origin_isolated<Origin, ValueErased>,
) -> Origin_erased<ValueErased> {
    isolated.erase()
}
pub fn origin_erased_rid<ValueErased>(
    Record·erased·rid { erased, rid }: Record·erased·rid<
        Origin_erased<ValueErased>,
        Fn<ValueErased, Record>,
    >,
) -> Record {
    rid(erased.value_erased)
}
fn origin_unerase<LocalOrigin, Value, ValueErased>(
    Record·erased·origin·unerase {
        erased,
        origin,
        unerase,
    }: Record·erased·origin·unerase<
        Origin_erased<ValueErased>,
        Origin<LocalOrigin, Record>,
        Fn<
            Record·erased·uneraser<ValueErased, Origin_uneraser<LocalOrigin>>,
            Record·unerased·uneraser<Value, Origin_uneraser<LocalOrigin>>,
        >,
    >,
) -> Value {
    erased.unerase(origin).unisolate(|erased, uneraser| {
        let Record·unerased·uneraser { unerased, uneraser } = unerase(Record·erased·uneraser {
            erased: erased,
            uneraser: uneraser,
        });
        (unerased, uneraser)
    })
}

pub fn buf_empty<Element, LocalOrigin, Part>(
    origin: Origin<LocalOrigin, Part>,
) -> Buf<Origin<LocalOrigin, Part>, Element> {
    Buf::<Origin<LocalOrigin, Part>, Element>::new(origin)
}
pub fn buf_pre_allocate_at_least<Element, Origin>(
    Record·buf·length {
        mut buf,
        length: min_pre_allocated_length,
    }: Record·buf·length<Buf<Origin, Element>, u32>,
) -> Buf<Origin, Element> {
    buf.pre_allocate_at_least(min_pre_allocated_length);
    buf
}
pub fn buf_pre_allocation_rid<Element, Origin>(
    mut buf: Buf<Origin, Element>,
) -> Buf<Origin, Element> {
    buf.pre_allocation_rid();
    buf
}
pub fn buf_remove<Element, Origin>(
    Record·buf·slot { mut buf, slot }: Record·buf·slot<Buf<Origin, Element>, Slot<Origin>>,
) -> Record·buf·element<Buf<Origin, Element>, Element> {
    let element = buf.remove(slot);
    Record·buf·element {
        buf: buf,
        element: element,
    }
}
pub fn buf_unset<Element, Origin>(
    Record·buf·slot { mut buf, slot }: Record·buf·slot<Buf<Origin, Element>, Slot<Origin>>,
) -> Record·buf·element·slot<Buf<Origin, Element>, Element, Unset_slot<Origin>> {
    let element = buf.unset(slot);
    Record·buf·element·slot {
        buf: buf,
        element: element.element,
        slot: element.slot,
    }
}
pub fn buf_set<Element, Origin>(
    Record·buf·new·slot {
        mut buf,
        slot,
        new: element,
    }: Record·buf·new·slot<Buf<Origin, Element>, Element, Unset_slot<Origin>>,
) -> Record·buf·slot<Buf<Origin, Element>, Slot<Origin>> {
    let set_slot = buf.set(slot, element);
    Record·buf·slot {
        buf: buf,
        slot: set_slot,
    }
}
pub fn buf_unset_slot_rid<Element, Origin>(
    Record·buf·slot {
        mut buf,
        slot: slot_to_vacate,
    }: Record·buf·slot<Buf<Origin, Element>, Unset_slot<Origin>>,
) -> Buf<Origin, Element> {
    buf.unset_slot_rid(slot_to_vacate);
    buf
}
pub fn buf_unset_span_rid<Element, Origin>(
    Record·buf·span {
        span: span_to_vacate,
        mut buf,
    }: Record·buf·span<Buf<Origin, Element>, Unset_span<Origin>>,
) -> Buf<Origin, Element> {
    buf.unset_span_rid(span_to_vacate);
    buf
}
pub fn buf_opt_unset_span_rid<Element, Origin>(
    Record·buf·span {
        span: span_to_vacate,
        mut buf,
    }: Record·buf·span<Buf<Origin, Element>, Opt<Unset_span<Origin>>>,
) -> Buf<Origin, Element> {
    buf.opt_unset_span_rid(span_to_vacate);
    buf
}
pub fn buf_rid<Element, Origin>(_: Buf<Origin, Element>) -> Record {}
pub fn buf_insert<Element, Origin>(
    Record·buf·new {
        mut buf,
        new: new_element,
    }: Record·buf·new<Buf<Origin, Element>, Element>,
) -> Record·buf·slot<Buf<Origin, Element>, Slot<Origin>> {
    let slot = buf.insert(new_element);
    Record·buf·slot {
        buf: buf,
        slot: slot,
    }
}
pub fn buf_add<Element, Origin>(
    Record·buf·new {
        mut buf,
        new: new_element,
    }: Record·buf·new<Buf<Origin, Element>, Element>,
) -> Record·buf·slot<Buf<Origin, Element>, Slot<Origin>> {
    let slot = buf.add(new_element);
    Record·buf·slot {
        buf: buf,
        slot: slot,
    }
}
pub fn buf_insert_unset<Element, Origin>(
    mut buf: Buf<Origin, Element>,
) -> Record·buf·slot<Buf<Origin, Element>, Unset_slot<Origin>> {
    let slot = buf.insert_unset();
    Record·buf·slot {
        buf: buf,
        slot: slot,
    }
}
pub fn buf_add_unset<Element, Origin>(
    mut buf: Buf<Origin, Element>,
) -> Record·buf·slot<Buf<Origin, Element>, Unset_slot<Origin>> {
    let slot = buf.add_unset();
    Record·buf·slot {
        buf: buf,
        slot: slot,
    }
}
pub fn buf_add_unset_length<Element, Origin>(
    Record·buf·length { length, mut buf }: Record·buf·length<Buf<Origin, Element>, U32>,
) -> Record·buf·span<Buf<Origin, Element>, Opt<Unset_span<Origin>>> {
    let span = buf.add_unset_length(length);
    Record·buf·span {
        buf: buf,
        span: span,
    }
}
pub fn buf_add_unset_length_positive<Element, Origin>(
    Record·buf·length { length, mut buf }: Record·buf·length<Buf<Origin, Element>, P32>,
) -> Record·buf·span<Buf<Origin, Element>, Unset_span<Origin>> {
    let span = buf.add_unset_length_positive(length);
    Record·buf·span {
        buf: buf,
        span: span,
    }
}
pub fn buf_add_array<Element, Origin, Record>(
    Record·buf·new { mut buf, new }: Record·buf·new<Buf<Origin, Element>, Array<Element, Record>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let span = buf.add_array(new);
    Record·buf·span {
        buf: buf,
        span: span,
    }
}
pub fn buf_char_add_str<Origin>(
    Record·buf·new {
        mut buf,
        new: new_str,
    }: Record·buf·new<Buf<Origin, Char>, Str>,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    let new_span = buf.add_str(new_str);
    Record·buf·span {
        buf: buf,
        span: new_span,
    }
}
pub fn buf_opt_span_reverse<Element, Origin>(
    Record·buf·span { mut buf, mut span }: Record·buf·span<
        Buf<Origin, Element>,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Element>, Opt<Span<Origin>>> {
    buf.opt_span_slice_mut(&mut span).reverse();
    Record·buf·span {
        buf: buf,
        span: span,
    }
}
pub fn buf_span_reverse<Element, Origin>(
    Record·buf·span { mut buf, mut span }: Record·buf·span<Buf<Origin, Element>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    buf.span_slice_mut(&mut span).reverse();
    Record·buf·span {
        buf: buf,
        span: span,
    }
}

pub fn buf_opt_span_add<Element, Origin>(
    Record·buf·new·span {
        mut buf,
        span,
        new: new_element,
    }: Record·buf·new·span<Buf<Origin, Element>, Element, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    match span {
        Opt::No(()) => {
            let new_slot = buf.insert(new_element);
            Record·buf·span {
                buf: buf,
                span: slot_to_span(new_slot),
            }
        }
        Opt::Yes(span) => buf_span_add(Record·buf·new·span {
            buf: buf,
            span: span,
            new: new_element,
        }),
    }
}
pub fn buf_span_add<Element, Origin>(
    Record·buf·new·span {
        mut buf,
        span,
        new: new_element,
    }: Record·buf·new·span<Buf<Origin, Element>, Element, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let combined_span = buf.span_add(span, new_element);
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_span_add_array<Element, Origin, Record>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Element>,
        Array<Element, Record>,
        Span<Origin>,
    >,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let combined_span = buf.span_add_array(span, new);
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_opt_span_add_array<Element, Origin, Record>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Element>,
        Array<Element, Record>,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let combined_span = buf.opt_span_add_array(span, new);
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_char_opt_span_add_str<Origin>(
    Record·buf·new·span {
        mut buf,
        span,
        new: new_str,
    }: Record·buf·new·span<Buf<Origin, Char>, Str, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    let combined_span = buf.opt_span_add_str(span, new_str);
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_char_span_add_str<Origin>(
    Record·buf·new·span {
        mut buf,
        span,
        new: new_str,
    }: Record·buf·new·span<Buf<Origin, Char>, Str, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    let combined_span = buf.span_add_iterator(span, new_str.str.chars());
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_char_span_add_u32<Origin>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Char>,
        U32,
        Span<Origin>,
    >,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    let combined_span = buf.span_add_iterator(
        span,
        new.format_into(&mut core::fmt::NumBuffer::new()).chars(),
    );
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_char_opt_span_add_u32<Origin>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Char>,
        U32,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    let combined_span = buf.opt_span_add_iterator(
        span,
        new.format_into(&mut core::fmt::NumBuffer::new()).chars(),
    );
    Record·buf·span {
        buf: buf,
        span: {
            // .chars() has .len() >= 1 because a formatted number is never ""
            unsafe { combined_span.into_option().unwrap_unchecked() }
        },
    }
}
pub fn buf_char_span_add_i32<Origin>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Char>,
        U32,
        Span<Origin>,
    >,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    let combined_span = buf.span_add_iterator(
        span,
        new.format_into(&mut core::fmt::NumBuffer::new()).chars(),
    );
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_char_opt_span_add_i32<Origin>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Char>,
        I32,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    let combined_span = buf.opt_span_add_iterator(
        span,
        new.format_into(&mut core::fmt::NumBuffer::new()).chars(),
    );
    Record·buf·span {
        buf: buf,
        span: {
            // .chars() has .len() >= 1 because a formatted number is never ""
            unsafe { combined_span.into_option().unwrap_unchecked() }
        },
    }
}
pub fn buf_char_span_add_f32<Origin>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Char>,
        F32,
        Span<Origin>,
    >,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    // can be optimized if NumBuffer gets expanded to cover f32
    let new_as_string = std::format!("{:.}", new);
    let combined_span = buf.span_add_iterator(span, new_as_string.chars());
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_char_opt_span_add_f32<Origin>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Char>,
        F32,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Char>, Span<Origin>> {
    // can be optimized if NumBuffer gets expanded to cover f32
    let new_as_string = std::format!("{:.}", new);
    let combined_span = buf.opt_span_add_iterator(span, new_as_string.chars());
    Record·buf·span {
        buf: buf,
        span: {
            // new_as_string has .len() >= 1 because a formatted number is never ""
            unsafe { combined_span.into_option().unwrap_unchecked() }
        },
    }
}
pub fn buf_opt_span_add_buf_opt_span<Origin, SourceOrigin, Element>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Element>,
        Buf<SourceOrigin, Element>,
        Opt<Span<SourceOrigin>>,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·source·source_span·span<
    Buf<Origin, Element>,
    Buf<SourceOrigin, Element>,
    Opt<Unset_span<SourceOrigin>>,
    Opt<Span<Origin>>,
> {
    let (source_span, combined_span) =
        buf.opt_span_add_buf_opt_span(span, &mut source, source_span);
    Record·buf·source·source_span·span {
        source: source,
        source_span: source_span,
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_span_add_buf_opt_span<Origin, SourceOrigin, Element>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Element>,
        Buf<SourceOrigin, Element>,
        Opt<Span<SourceOrigin>>,
        Span<Origin>,
    >,
) -> Record·buf·source·source_span·span<
    Buf<Origin, Element>,
    Buf<SourceOrigin, Element>,
    Opt<Unset_span<SourceOrigin>>,
    Span<Origin>,
> {
    let (source_span, combined_span) = buf.span_add_buf_opt_span(span, &mut source, source_span);
    Record·buf·source·source_span·span {
        source: source,
        source_span: source_span,
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_span_add_buf_span<Origin, SourceOrigin, Element>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Element>,
        Buf<SourceOrigin, Element>,
        Span<SourceOrigin>,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·source·source_span·span<
    Buf<Origin, Element>,
    Buf<SourceOrigin, Element>,
    Unset_span<SourceOrigin>,
    Span<Origin>,
> {
    let (source_span, combined_span) = buf.opt_span_add_buf_span(span, &mut source, source_span);
    Record·buf·source·source_span·span {
        source: source,
        source_span: source_span,
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_span_add_buf_span<Origin, SourceOrigin, Element>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Element>,
        Buf<SourceOrigin, Element>,
        Span<SourceOrigin>,
        Span<Origin>,
    >,
) -> Record·buf·source·source_span·span<
    Buf<Origin, Element>,
    Buf<SourceOrigin, Element>,
    Unset_span<SourceOrigin>,
    Span<Origin>,
> {
    let (source_span, combined_span) = buf.span_add_buf_span(span, &mut source, source_span);
    Record·buf·source·source_span·span {
        source: source,
        source_span: source_span,
        span: combined_span,
        buf: buf,
    }
}

pub fn buf_span_add_own_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Element>, Span<Origin>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let combined_span = buf.span_add_own_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_span_add_own_opt_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Element>, Opt<Span<Origin>>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let combined_span = buf.span_add_own_opt_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_span_add_own_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Element>, Span<Origin>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let combined_span = buf.opt_span_add_own_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_span_add_own_opt_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Element>, Opt<Span<Origin>>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Element>, Opt<Span<Origin>>> {
    let combined_span = buf.opt_span_add_own_opt_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_unset_span_add_own_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Element>, Unset_span<Origin>, Unset_span<Origin>>,
) -> Record·buf·span<Buf<Origin, Element>, Unset_span<Origin>> {
    let combined_span = buf.unset_span_add_own_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_unset_span_add_own_opt_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<
        Buf<Origin, Element>,
        Opt<Unset_span<Origin>>,
        Unset_span<Origin>,
    >,
) -> Record·buf·span<Buf<Origin, Element>, Unset_span<Origin>> {
    let combined_span = buf.unset_span_add_own_opt_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_unset_span_add_own_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<
        Buf<Origin, Element>,
        Unset_span<Origin>,
        Opt<Unset_span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Element>, Unset_span<Origin>> {
    let combined_span = buf.opt_unset_span_add_own_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_unset_span_add_own_opt_span<Element, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<
        Buf<Origin, Element>,
        Opt<Unset_span<Origin>>,
        Opt<Unset_span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Element>, Opt<Unset_span<Origin>>> {
    let combined_span = buf.opt_unset_span_add_own_opt_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}

pub fn buf_span_move_to_vacant<Element, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Element>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let moved_span = buf.span_move_to_vacant(span);
    Record·buf·span {
        span: moved_span,
        buf: buf,
    }
}
pub fn buf_opt_span_move_to_vacant<Element, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Element>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Element>, Opt<Span<Origin>>> {
    match span {
        Opt::No(()) => Record·buf·span {
            span: Opt::No(()),
            buf: buf,
        },
        Opt::Yes(span) => {
            let moved_span = buf.span_move_to_vacant(span);
            Record·buf·span {
                span: Opt::Yes(moved_span),
                buf: buf,
            }
        }
    }
}
pub fn buf_span_move_to_end<Element, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Element>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Element>, Span<Origin>> {
    let moved_span = buf.span_move_to_end(span);
    Record·buf·span {
        span: moved_span,
        buf: buf,
    }
}
pub fn buf_opt_span_move_to_end<Element, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Element>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Element>, Opt<Span<Origin>>> {
    match span {
        Opt::No(()) => Record·buf·span {
            span: Opt::No(()),
            buf: buf,
        },
        Opt::Yes(span) => {
            let moved_span = buf.span_move_to_end(span);
            Record·buf·span {
                span: Opt::Yes(moved_span),
                buf: buf,
            }
        }
    }
}
pub fn buf_to_unset<Element, Origin>(buf: Buf<Origin, Element>) -> Unset_slice<Element> {
    buf.into_unset_slice()
}
pub fn buf_reuse<LocalOrigin, Part, Element>(
    Record·origin·slice { origin, slice }: Record·origin·slice<
        Origin<LocalOrigin, Part>,
        Unset_slice<Element>,
    >,
) -> Buf<Origin<LocalOrigin, Part>, Element> {
    Buf::reuse(origin, slice)
}
fn buf_origin_isolate<Element, ElementErased, LocalOrigin, Part>(
    Record·buf·element_isolate {
        buf,
        element_isolate,
    }: Record·buf·element_isolate<
        Buf<Origin<LocalOrigin, Part>, Element>,
        Fn<Element, Origin_isolated<LocalOrigin, ElementErased>>,
    >,
) -> Origin_isolated<LocalOrigin, Buf_origin_erased<Part, ElementErased>> {
    // safe because origin_unerase is not public
    // and called only from sloe which follows stricter rules (linear types)
    // which prevent unset slots and spans to be scrapped (they cannot be origin-isolated)
    unsafe { buf.origin_isolate_assume_no_unset(element_isolate) }
}
pub fn buf_origin_unerase<Element, ElementErased, LocalOrigin, Part>(
    Record·buf·element_unerase·uneraser {
        buf,
        element_unerase,
        uneraser,
    }: Record·buf·element_unerase·uneraser<
        Buf_origin_erased<Part, ElementErased>,
        Fn<
            Record·element·uneraser<ElementErased, Origin_uneraser<LocalOrigin>>,
            Record·element·uneraser<Element, Origin_uneraser<LocalOrigin>>,
        >,
        Origin_uneraser<LocalOrigin>,
    >,
) -> Record·buf·uneraser<Buf<Origin<LocalOrigin, Part>, Element>, Origin_uneraser<LocalOrigin>> {
    let buf_unerased = buf.erased.origin_unerase(&uneraser, |element, uneraser| {
        let Record·element·uneraser {
            element: element_erased,
            uneraser: eraser,
        } = element_unerase(Record·element·uneraser {
            element: element,
            uneraser: uneraser,
        });
        (element_erased, eraser)
    });
    Record·buf·uneraser {
        buf: buf_unerased,
        uneraser: uneraser,
    }
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
        let mut buf = crate::core::Buf::new(origin);
        let mut slots = std::iter::Iterator::collect::<std::vec::Vec<_>>(std::iter::Iterator::map(
            0..100,
            |i| buf.add(i),
        ));
        // a bit of fake dumb noise.
        // once rust gains std:: fuzzing/randomness we should use that
        slots.as_mut_slice()[25..50].reverse();
        slots.swap(2, 94);
        slots.swap(12, 88);
        slots.swap(34, 39);
        for slot in slots {
            buf.remove(slot);
        }
        std::assert_eq!(buf.vacant_spans().len(), 0);
        std::assert_eq!(buf.maybe_uninit_elements().len(), 0);
        crate::core::buf_rid(buf);
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
        let buf = crate::core::Buf::reuse(origin, unset_slice_i64);
        crate::core::buf_rid(buf);
    }
    #[test]
    fn unset_slice_cast_or_rid_and_allocate_u64_to_tuple_u32_u32() {
        let unset_slice_u64 = crate::core::Unset_slice::<u64>::allocate_length(20);
        let unset_slice_tuple_u32_u32 = unset_slice_u64.cast_or_rid_and_allocate::<(u32, u32)>();
        crate::core::origin_new!(origin, Origin);
        let buf = crate::core::Buf::reuse(origin, unset_slice_tuple_u32_u32);
        crate::core::buf_rid(buf);
    }
}
