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
pub struct Record·buf·item_isolate<Buf, Item_isolate> {
    pub buf: Buf,
    pub item_isolate: Item_isolate,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·item_unerase·uneraser<Buf, Item_unerase, Uneraser> {
    pub buf: Buf,
    pub item_unerase: Item_unerase,
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
pub struct Record·item·in<Item, In> {
    pub item: Item,
    pub in_: In,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·item·uneraser<Item, Uneraser> {
    pub item: Item,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·item·slot<Item, Slot> {
    pub item: Item,
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
pub struct Record·in_·item<In, Item> {
    pub in_: In,
    pub item: Item,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·item·out<Item, Out> {
    pub item: Item,
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
pub struct Record·index·span<Index, Span> {
    pub index: Index,
    pub span: Span,
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
pub struct Record·done·rest<Done, Rest> {
    pub done: Done,
    pub rest: Rest,
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
pub struct Record·buf·slot_a·slot_b<Buf, Slot_a, Slot_b> {
    pub buf: Buf,
    pub slot_a: Slot_a,
    pub slot_b: Slot_b,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·in_·slot·step<Buf, In, Slot, Step> {
    pub buf: Buf,
    pub in_: In,
    pub slot: Slot,
    pub step: Step,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·item_rid·slot<Buf, Item_rid, Slot> {
    pub buf: Buf,
    pub item_rid: Item_rid,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·item_rid·span<Buf, Item_rid, Span> {
    pub buf: Buf,
    pub item_rid: Item_rid,
    pub span: Span,
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
pub struct Record·buf·source·span<Buf, Source, Span> {
    pub buf: Buf,
    pub source: Source,
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
pub struct Record·buf·item<Buf, Item> {
    pub buf: Buf,
    pub item: Item,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·uneraser<Buf, Uneraser> {
    pub buf: Buf,
    pub uneraser: Uneraser,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·item·slot<Buf, Item, Slot> {
    pub buf: Buf,
    pub item: Item,
    pub slot: Slot,
}
#[derive(Clone, Copy, Debug)]
pub struct Record·buf·old·slot<Buf, Old_item, Slot> {
    pub buf: Buf,
    pub old: Old_item,
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
pub enum Choice·Done·Going<Done, Going> {
    Done(Done),
    Going(Going),
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

pub struct Unset_slice<Item>(
    // invariant: .len() == 0
    std::vec::Vec<std::option::Option<Item>>,
);
#[derive(Debug)]
#[non_exhaustive]
pub struct Buf_origin_erased<Part, Item> {
    erased: Buf<Origin<Erased, Part>, Item>,
}
#[derive(Debug)]
pub struct Buf<Origin, Item> {
    // invariant: the last item in .items is Some(_)
    items: std::vec::Vec<std::option::Option<Item>>,
    // cached first Option::None item index in .items. Invariants:
    // - if first_none == u32::MAX: all elements in .items are Some(_)
    // - if first_none < items.len(): items[i] == None
    first_unset_index: u32,
    origin: std::marker::PhantomData<Origin>,
}
pub struct Slot<LocalOrigin> {
    pub origin: std::marker::PhantomData<LocalOrigin>,
    /// Invariant: < u32::MAX because a containing container can only hold <= u32::MAX items
    // consider switching to NonZeroU32 to create a niche for use with Option<Slot<>>
    index: u32,
}
pub struct Span<LocalOrigin> {
    // it would probably be better to flatten the Slot out
    start: Slot<LocalOrigin>,
    length: std::num::NonZeroU32,
}

pub struct Array<Item, Record> {
    pub record: Record,
    /// Warning: API might be changed in the future; do not rely on it if you can.
    ///
    /// It would be great if we could find a _safe_ way to as directly as possible iterate the array.
    /// Various helpers like getting the size and dup-ing would aso be nice
    /// but are to be avoided if they come at a memory cost.
    /// The problem is that
    /// - providing fold is impossible because for<State> fn is not allowed
    /// - providing for_each is impossible because fn(impl FnMut) is not allowed
    /// - there is no such thing as an "owned stack-allocated dynamic-size slice" in rust
    ///
    /// A solution would be using
    /// pub as_slice: fn(&mut Record) -> &mut [Item]
    /// but this relies (!) on both
    ///   - callers to use unsafe to extract owned items
    ///   - array creation to use unsafe (and rely on field order despite not using repr(C))
    ///
    /// Another "solution" would be to just give up and use `Box<[Item]>`
    /// or add a fn that returns Box<dyn Iterator> or similar
    /// and hope the optimizer converts heap into stack alloction. (naw man, that ain't it)
    ///
    /// We could even use somthing like SmallVec as e.g.
    /// `size: P32, on_stack: [Item;8], remaining: Option<Box<[Item]>>`
    /// quite thick, and probably no faster than full-on heap :(
    /// (using this when we actually know the exact size also feels bad)
    ///
    /// So for now, use-cases are "embarassingly hardcoded".
    /// This solution is restrictive and thus very unsatisfying but at least it works.
    /// Maybe there are nicer hardcoded primitives, though
    /// (e.g. writing into a given &mut [MaybeUninit]?,
    /// or fn at(index: u32, &mut Record) -> Option<&mut Item>
    /// which requires unsafe at call-site for ownership and is probably slower?)
    /// Help!
    ///
    /// Why this weird function signature?
    /// To enable operations like Vec::add_array to return a Span instead of an Opt<Span>.
    /// We could panic or uncheck that case but then buggy Array instances could blow everything up.
    /// Originally I split .record into .before and .last but it felt confusing
    /// in sloe code that the specified record had 1 field less than actual items
    pub split_last_and_extend_vec_with_before:
        fn(&mut std::vec::Vec<std::option::Option<Item>>, Record) -> Item,
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
impl<Item> std::fmt::Debug for Unset_slice<Item> {
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

impl<Done, Going> Choice·Done·Going<Done, Going> {
    pub fn from_control_flow(control_flow: std::ops::ControlFlow<Done, Going>) -> Self {
        match control_flow {
            std::ops::ControlFlow::Break(done) => Choice·Done·Going::Done(done),
            std::ops::ControlFlow::Continue(doing) => Choice·Done·Going::Going(doing),
        }
    }
    pub fn into_control_flow(self) -> std::ops::ControlFlow<Done, Going> {
        match self {
            Choice·Done·Going::Done(done) => std::ops::ControlFlow::Break(done),
            Choice·Done·Going::Going(doing) => std::ops::ControlFlow::Continue(doing),
        }
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
    /// Same as `new` but yoinking the type from a local origin value
    pub unsafe fn of(_: &LocalOrigin) -> Origin<LocalOrigin, Part> {
        Origin(std::marker::PhantomData::<(LocalOrigin, Part)>)
    }
}

/// To create multiple Origins with the same unique origin type
/// ```ignore
/// origin_new!(some, UniqueOrigin, a_variable, Record·part_a, b_variable, Record·part_b)
/// ```
/// (only works if there is actually such a record in the generated code.
/// Also note that this is different from sloe's ^ .a .b unique which only creates one variable)
///
/// If you don't, use the simpler
/// ```ignore
/// origin_new!(variable_name, LocalOriginName)
/// ```
///
/// Careful! The multi-origin form of `origin_new!`
/// wil crash **at runtime** if some field names overlap.
/// This is to prevent multiple origins with the same name type being created.
/// In theory, it should be possible to report a compile-error in that case,
/// however, there seems to neither exist const == on &str, nor const panic, nor ident concat etc.
/// I'm sorry :3
#[macro_export]
macro_rules! origin_new {
    // History: Originally the implementation only relied on a local type.
    // But it turns out, those types can freely escape their scope:
    // https://stackoverflow.com/a/76876800 Thanks rodrigo!
    // This means that using it in a loop or similar re-entering of the same expression
    // using origin_new! could have lead to two origins with the same unique local type :/.
    // One potential solution was only utilizing unique lifetimes there are many, many ugly
    // patches needed to make it sound, read e.g. <https://arhan.sh/blog/the-generativity-pattern-in-rust/>.
    // So in the end, I settled with a simpler hybrid approach inspired by
    // https://codeberg.org/binarycat/typetoken/src/branch/trunk/src/lib.rs
    ($variable_name:ident, $type_name:ident) => {
		let local = ();
		let mut disable_lifetime_covariance = &local;
        struct $type_name<'a>(#[expect(unused)] &'a mut &'a ());
        let $variable_name: $crate::core::Origin::<$type_name, $crate::core::Record> = unsafe {
            $crate::core::Origin::of(&$type_name(&mut disable_lifetime_covariance))
        };
    };
    ($type_name:ident, $part0_name:ident, $($parts:ident),+) => {
        if part_record_names_are_invalid!($($parts),+) {
            panic!("invalid part names. Each part needs a unique name and must start with Record·!");
        }
		let local = ();
		let mut disable_lifetime_covariance = &local;
        struct $type_name<'a>(#[expect(unused)] &'a mut &'a ());
        let local_origin = $type_name(&mut disable_lifetime_covariance);
        origin_new_variables!($type_name, unsafe { $crate::core::Origin::of(&local_origin) }, $part0_name, $($parts),+)
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

impl<Item> Unset_slice<Item> {
    pub fn allocate_length(length: u32) -> Self {
        Unset_slice(std::vec::Vec::with_capacity(length as usize))
    }
    pub fn from_vec_option(mut vec_option: std::vec::Vec<std::option::Option<Item>>) -> Self {
        vec_option.clear();
        Unset_slice(vec_option)
    }
    pub fn as_slice(&self) -> &[std::option::Option<Item>] {
        &self.0
    }
    pub fn cast_or_rid_and_allocate<NewItem>(self) -> Unset_slice<NewItem> {
        Unset_slice(self.into_vec())
    }
    /// almost always you should prefer .into_vec
    /// The only reason to use .into_vec_option is to speed up debug builds and to be sure no allocation is happening
    pub fn into_vec_option(self) -> std::vec::Vec<std::option::Option<Item>> {
        self.0
    }
    pub fn into_vec<NewItem>(self) -> std::vec::Vec<NewItem> {
        // below will reinterpret the allocation in release mode without doing any work
        std::assert!(self.0.is_empty());
        std::iter::Iterator::collect(std::iter::Iterator::map(
            std::iter::IntoIterator::into_iter(self.0),
            |_| unsafe { std::hint::unreachable_unchecked() },
        ))
    }
}

impl<Item, LocalOrigin> Buf<LocalOrigin, Item> {
    /// See also `set_count` and `unset_count`
    pub fn length_including_unset(&self) -> u32 {
        self.items.len() as u32
    }
    /// Especially when working with estimates or future insertions, you usually want pre_allocate_at_least
    pub fn pre_allocate(&mut self, pre_allocated_length: u32) {
        self.items.reserve_exact(pre_allocated_length as usize);
    }
    pub fn pre_allocate_at_least_usize(&mut self, min_pre_allocated_length: usize) {
        self.items.reserve(min_pre_allocated_length);
    }
    pub fn pre_allocate_at_least(&mut self, min_pre_allocated_length: u32) {
        self.pre_allocate_at_least_usize(min_pre_allocated_length as usize);
    }
    pub fn pre_allocation_rid(&mut self) {
        self.items.shrink_to_fit();
    }
    pub fn as_slice<'a>(&'a self) -> &'a [std::option::Option<Item>] {
        &self.items
    }
    fn item_option<'a>(&'a self, slot: &'a Slot<LocalOrigin>) -> &'a std::option::Option<Item> {
        // new slots are bound to this collection origin and contain a known valid index
        unsafe { self.items.get_unchecked(slot.index as usize) }
    }
    pub fn item<'a>(&'a self, slot: &'a Slot<LocalOrigin>) -> &'a Item {
        unsafe { self.item_option(slot).as_ref().unwrap_unchecked() }
    }
    fn item_option_mut<'a>(
        &'a mut self,
        slot: &'a mut Slot<LocalOrigin>,
    ) -> &'a mut std::option::Option<Item> {
        // new slots are bound to this collection origin and contain a known valid index
        unsafe { self.items.get_unchecked_mut(slot.index as usize) }
    }
    pub fn item_mut<'a>(&'a mut self, slot: &'a mut Slot<LocalOrigin>) -> &'a mut Item {
        unsafe { self.item_option_mut(slot).as_mut().unwrap_unchecked() }
    }
    pub fn opt_span_slice_option<'a>(
        &'a self,
        opt_span: Opt<&'a Span<LocalOrigin>>,
    ) -> &'a [std::option::Option<Item>] {
        match opt_span {
            Opt::No(()) => &[],
            Opt::Yes(span) => self.span_slice_option(span),
        }
    }
    pub fn span_slice_option<'a>(
        &'a self,
        span: &'a Span<LocalOrigin>,
    ) -> &'a [std::option::Option<Item>] {
        // new slots are bound to this collection origin and contain a known valid range
        unsafe { self.items.get_unchecked(span.to_range()) }
    }
    pub fn opt_span_iter<'a>(
        &'a self,
        opt_span: Opt<&'a Span<LocalOrigin>>,
    ) -> impl std::iter::DoubleEndedIterator<Item = &'a Item> {
        match opt_span {
            Opt::No(()) => std::iter::Iterator::flatten(std::iter::IntoIterator::into_iter(
                std::option::Option::None,
            )),
            Opt::Yes(span) => std::iter::Iterator::flatten(std::iter::IntoIterator::into_iter(
                std::option::Option::Some(self.span_iter(span)),
            )),
        }
    }
    pub fn span_iter<'a>(
        &'a self,
        span: &'a Span<LocalOrigin>,
    ) -> impl std::iter::DoubleEndedIterator<Item = &'a Item> {
        // new slots are bound to this collection origin and contain a known valid range
        std::iter::Iterator::map(self.span_slice_option(span).iter(), |item| unsafe {
            item.as_ref().unwrap_unchecked()
        })
    }
    fn opt_span_slice_option_mut<'a>(
        &'a mut self,
        opt_span: &'a mut Opt<Span<LocalOrigin>>,
    ) -> &'a mut [std::option::Option<Item>] {
        match opt_span {
            Opt::No(()) => &mut [],
            Opt::Yes(span) => self.span_slice_option_mut(span),
        }
    }
    pub fn opt_span_slice_mut<'a>(
        &'a mut self,
        opt_span: &'a mut Opt<Span<LocalOrigin>>,
    ) -> impl std::iter::DoubleEndedIterator<Item = &'a mut Item> {
        match opt_span {
            Opt::No(()) => std::iter::Iterator::flatten(std::iter::IntoIterator::into_iter(
                std::option::Option::None,
            )),
            Opt::Yes(span) => std::iter::Iterator::flatten(std::iter::IntoIterator::into_iter(
                std::option::Option::Some(self.span_iter_mut(span)),
            )),
        }
    }
    pub fn span_iter_mut<'a>(
        &'a mut self,
        span: &'a mut Span<LocalOrigin>,
    ) -> impl std::iter::DoubleEndedIterator<Item = &'a mut Item> {
        std::iter::Iterator::map(self.span_slice_option_mut(span).iter_mut(), |item| unsafe {
            item.as_mut().unwrap_unchecked()
        })
    }
    fn span_slice_option_mut<'a>(
        &'a mut self,
        span: &'a mut Span<LocalOrigin>,
    ) -> &'a mut [std::option::Option<Item>] {
        // new slots are bound to this collection origin and contain a known valid range
        unsafe { self.items.get_unchecked_mut(span.to_range()) }
    }
    /// this APIs is a bit wonky and compromises on performance and useability
    fn span_into_iter<'a, Out>(
        &'a mut self,
        span: Span<LocalOrigin>,
        consume_iterator: impl std::ops::FnOnce(
            &mut dyn std::iter::DoubleEndedIterator<Item = Item>,
        ) -> Out,
    ) -> Out {
        if self.span_is_at_the_end_of_items(&span) {
            let out = consume_iterator(&mut std::iter::Iterator::map(
                self.items.drain((span.start.index as usize)..),
                |item| unsafe { item.unwrap_unchecked() },
            ));
            self.rid_trailing_unset();
            out
        } else {
            let out = consume_iterator(&mut
                // items in the span are consumed and never accessed after
                unsafe {
                    std::iter::Iterator::map(
                        self.items.get_unchecked_mut(span.to_range()).iter_mut(),
                        |item| item.take().unwrap_unchecked(),
                    )
                });
            self.first_unset_index = std::cmp::min(self.first_unset_index, span.start.index);
            out
        }
    }
    pub fn remove(&mut self, mut slot: Slot<LocalOrigin>) -> Item {
        if slot.index + 1 < self.length_including_unset() {
            let item = unsafe { self.item_option_mut(&mut slot).take().unwrap_unchecked() };
            self.first_unset_index = std::cmp::min(self.first_unset_index, slot.index);
            item
        } else {
            let item = unsafe { self.items.pop().unwrap_unchecked().unwrap_unchecked() };
            self.rid_trailing_unset();
            item
        }
    }
    /// After this, slot0 will reference the same item that slot1 did originally
    /// and slot1 will reference the same item that slot0 did originally
    fn swap(&mut self, slot0: &mut Slot<LocalOrigin>, slot1: &mut Slot<LocalOrigin>) {
        // can probably be opimized by asserting slot0.index == slot1.index is unreachable
        self.items.swap(slot0.index as usize, slot1.index as usize);
    }
    /// returns the old item
    fn replace(&mut self, slot: &mut Slot<LocalOrigin>, new_item: Item) -> Item {
        let item_option_ref_mut = self.item_option_mut(slot);
        let old_item = unsafe { item_option_ref_mut.take().unwrap_unchecked() };
        _ = item_option_ref_mut.insert(new_item);
        old_item
    }
    pub fn item_step<'a, Out>(
        &'a mut self,
        slot: &'a mut Slot<LocalOrigin>,
        step: impl std::ops::FnOnce(Item) -> (Item, Out),
    ) -> Out {
        let item_option = self.item_option_mut(slot);
        let item = unsafe { item_option.take().unwrap_unchecked() };
        let (new_item, out) = step(item);
        _ = item_option.insert(new_item);
        out
    }
    pub fn opt_span_rid(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        item_rid: impl std::ops::Fn(Item),
    ) {
        if let Opt::Yes(span) = span {
            self.span_rid(span, item_rid);
        }
    }
    pub fn span_rid(&mut self, span: Span<LocalOrigin>, item_rid: impl std::ops::Fn(Item)) {
        self.span_into_iter(span, |items| {
            for item in items {
                item_rid(item)
            }
        });
    }
    fn rid_trailing_unset(&mut self) {
        // this feels unoptimal somehow
        while let std::option::Option::Some(std::option::Option::None) = self.items.last() {
            self.items.pop();
        }
        if self.first_unset_index == self.length_including_unset() {
            self.first_unset_index = u32::MAX;
        }
    }
    pub fn add(&mut self, new_item: Item) -> Slot<LocalOrigin> {
        self.items.push(std::option::Option::Some(new_item));
        Slot::from_index(std::convert::TryInto::<u32>::try_into(self.items.len()).unwrap() - 1)
    }
    pub fn insert(&mut self, new_item: Item) -> Slot<LocalOrigin> {
        // can maybe be optimized? first_unset_index is always valid if it's not ::MAX
        match self.items.get_mut(self.first_unset_index as usize) {
            std::option::Option::Some(item_option_to_set) => {
                _ = item_option_to_set.insert(new_item);
                let set_index = self.first_unset_index;
                self.first_unset_index = std::iter::Iterator::find_map(
                    &mut std::iter::Iterator::skip(
                        std::iter::Iterator::enumerate(self.items.iter()),
                        (self.first_unset_index + 1) as usize,
                    ),
                    |(i, item)| match item {
                        std::option::Option::None => std::option::Option::Some(i as u32),
                        std::option::Option::Some(_) => std::option::Option::None,
                    },
                )
                .unwrap_or_else(|| u32::MAX);
                Slot::<LocalOrigin>::from_index(set_index)
            }
            std::option::Option::None => self.add(new_item),
        }
    }
    fn find_unset_length_positive(
        &mut self,
        unset_length_to_find: std::num::NonZeroU32,
    ) -> std::option::Option<u32> {
        // can maybe be optimized by skipping unset_length_to_find - 1 ahead when encountering a None
        // and going back to check that all are None only if the end is itself None
        std::iter::Iterator::try_fold(
            &mut std::iter::Iterator::skip(
                std::iter::Iterator::enumerate(self.items.iter()),
                self.first_unset_index as usize,
            ),
            0,
            |length_so_far, (index, item)| match item {
                std::option::Option::None => std::ops::ControlFlow::Continue(0),
                std::option::Option::Some(_) => {
                    if length_so_far + 1 < unset_length_to_find.get() {
                        std::ops::ControlFlow::Continue(length_so_far + 1)
                    } else {
                        std::ops::ControlFlow::Break((index, length_so_far + 1))
                    }
                }
            },
        )
        .break_value()
        .map(|(index, _)| index as u32)
    }
    pub fn add_iterator(
        &mut self,
        new_items: impl std::iter::Iterator<Item = Item>,
    ) -> Opt<Span<LocalOrigin>> {
        let length_without_new_items = self.length_including_unset();
        std::iter::Extend::extend(&mut self.items, new_items.map(std::option::Option::Some));
        let length_with_new_items =
            std::convert::TryInto::<u32>::try_into(self.items.len()).unwrap();
        match std::num::NonZeroU32::new(length_with_new_items - length_without_new_items) {
            std::option::Option::None => Opt::No(()),
            std::option::Option::Some(new_length) => Opt::Yes(Span {
                start: Slot::from_index(length_without_new_items),
                length: new_length,
            }),
        }
    }
    pub fn add_one_then_iterator(
        &mut self,
        new_start: Item,
        new_after: impl std::iter::Iterator<Item = Item>,
    ) -> Span<LocalOrigin> {
        let length_without_new_items = self.length_including_unset();
        self.items.push(std::option::Option::Some(new_start));
        std::iter::Extend::extend(&mut self.items, new_after.map(std::option::Option::Some));
        let length_with_new_items =
            std::convert::TryInto::<u32>::try_into(self.items.len()).unwrap();
        Span {
            start: Slot::from_index(length_without_new_items),
            length: std::num::NonZeroU32::MIN
                .saturating_add(length_with_new_items - length_without_new_items - 1),
        }
    }
    /// Important invariant! new_item_count must equal new_items.count()
    /// The recommended, safe alternative is add_one_then_iterator
    fn add_iterator_filled(
        &mut self,
        new_items: impl std::iter::Iterator<Item = Item>,
        new_item_count: std::num::NonZeroU32,
    ) -> Span<LocalOrigin> {
        let length_without_new_items = self.length_including_unset();
        std::iter::Extend::extend(&mut self.items, new_items.map(std::option::Option::Some));
        _ = std::convert::TryInto::<u32>::try_into(self.items.len()).unwrap();
        Span {
            start: Slot::from_index(length_without_new_items),
            length: new_item_count,
        }
    }
    /// assumes that the length of the iterator is <= u32::MAX
    pub fn insert_iterator(
        &mut self,
        new_items: impl std::iter::ExactSizeIterator<Item = Item>,
    ) -> Opt<Span<LocalOrigin>> {
        match std::num::NonZeroU32::new(new_items.len() as u32) {
            std::option::Option::None => Opt::No(()),
            std::option::Option::Some(new_item_count) => {
                Opt::Yes(self.insert_iterator_filled(new_items, new_item_count))
            }
        }
    }
    /// Important invariant! new_item_count must equal new_items.count()
    fn insert_iterator_filled(
        &mut self,
        new_items: impl std::iter::Iterator<Item = Item>,
        new_item_count: std::num::NonZeroU32,
    ) -> Span<LocalOrigin> {
        match self.find_unset_length_positive(new_item_count) {
            std::option::Option::None => self.add_iterator_filled(new_items, new_item_count),
            std::option::Option::Some(index_to_populate_from) => {
                let new_span = Span {
                    start: Slot::from_index(index_to_populate_from),
                    length: new_item_count,
                };
                self.items.splice(
                    new_span.to_range(),
                    new_items.map(std::option::Option::Some),
                );
                new_span
            }
        }
    }
    /// This will clone the iterator. Prefer add_iterator whenever possible
    pub fn insert_iterator_without_known_size(
        &mut self,
        new_items: impl std::iter::Iterator<Item = Item> + std::clone::Clone,
    ) -> Opt<Span<LocalOrigin>> {
        // can be optimized to only clone if there is actually existing unset space to occupy.
        // Might make sense to also benchmark with simply writing to the end, then relocating
        let std::option::Option::Some(new_length) =
            std::num::NonZeroU32::new(std::iter::Iterator::count(new_items.clone()) as u32)
        else {
            return Opt::No(());
        };
        let new_span = self.insert_iterator_filled(new_items, new_length);
        Opt::Yes(new_span)
    }
    pub fn add_array<Record>(&mut self, new_items: Array<Item, Record>) -> Span<LocalOrigin> {
        let length_without_new_items = self.length_including_unset();
        let new_last =
            (new_items.split_last_and_extend_vec_with_before)(&mut self.items, new_items.record);
        let length_with_new_items_before_last = self.add(new_last).index;
        Span {
            start: Slot::from_index(length_without_new_items),
            length: std::num::NonZeroU32::MIN
                .saturating_add(length_with_new_items_before_last - length_without_new_items),
        }
    }
    pub fn insert_buf_span<SourceOrigin>(
        &mut self,
        source: &mut Buf<SourceOrigin, Item>,
        source_span: Span<SourceOrigin>,
    ) -> Span<LocalOrigin> {
        let source_span_length = source_span.length;
        let new_span = source.span_into_iter(source_span, |source_items| {
            self.insert_iterator_filled(source_items, source_span_length)
        });
        new_span
    }
    pub fn add_buf_span<SourceOrigin>(
        &mut self,
        source: &mut Buf<SourceOrigin, Item>,
        source_span: Span<SourceOrigin>,
    ) -> Span<LocalOrigin> {
        let source_span_length = source_span.length;
        let new_span = source.span_into_iter(source_span, |source_items| {
            self.add_iterator_filled(source_items, source_span_length)
        });
        new_span
    }
    pub fn span_add_buf_span<SourceOrigin>(
        &mut self,
        span: Span<LocalOrigin>,
        source: &mut Buf<SourceOrigin, Item>,
        source_span: Span<SourceOrigin>,
    ) -> Span<LocalOrigin> {
        let new_span = source.span_into_iter(source_span, |source_items| {
            self.span_add_iterator(span, source_items)
        });
        new_span
    }
    pub fn span_add_buf_opt_span<SourceOrigin>(
        &mut self,
        span: Span<LocalOrigin>,
        source: &mut Buf<SourceOrigin, Item>,
        source_span: Opt<Span<SourceOrigin>>,
    ) -> Span<LocalOrigin> {
        match source_span {
            Opt::No(()) => span,
            Opt::Yes(source_span) => {
                let combined_span = self.span_add_buf_span(span, source, source_span);
                combined_span
            }
        }
    }
    pub fn opt_span_add_buf_span<SourceOrigin>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        source: &mut Buf<SourceOrigin, Item>,
        source_span: Span<SourceOrigin>,
    ) -> Span<LocalOrigin> {
        match span {
            Opt::No(()) => self.add_buf_span(source, source_span),
            Opt::Yes(span) => self.span_add_buf_span(span, source, source_span),
        }
    }
    pub fn opt_span_add_buf_opt_span<SourceOrigin>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        source: &mut Buf<SourceOrigin, Item>,
        source_span: Opt<Span<SourceOrigin>>,
    ) -> Opt<Span<LocalOrigin>> {
        match source_span {
            Opt::No(()) => span,
            Opt::Yes(source_span) => {
                let combined_span = self.opt_span_add_buf_span(span, source, source_span);
                Opt::Yes(combined_span)
            }
        }
    }
    pub fn span_add_iterator(
        &mut self,
        span: Span<LocalOrigin>,
        new_items: impl std::iter::Iterator<Item = Item>,
    ) -> Span<LocalOrigin> {
        let moved_span = self.span_move_to_end(span);
        let new_items_span = self.add_iterator(new_items);
        Span {
            start: moved_span.start,
            length: moved_span
                .length
                .saturating_add(new_items_span.as_ref().length()),
        }
    }
    pub fn opt_span_add_iterator(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        new_items: impl std::iter::Iterator<Item = Item>,
    ) -> Opt<Span<LocalOrigin>> {
        match span {
            Opt::No(()) => self.add_iterator(new_items),
            Opt::Yes(span) => Opt::Yes(self.span_add_iterator(span, new_items)),
        }
    }
    pub fn span_add_array<Record>(
        &mut self,
        span: Span<LocalOrigin>,
        new_items: Array<Item, Record>,
    ) -> Span<LocalOrigin> {
        let moved_span = self.span_move_to_end(span);
        let new_items_span = self.add_array(new_items);
        Span {
            start: moved_span.start,
            length: moved_span
                .length
                .saturating_add(new_items_span.length.get()),
        }
    }
    pub fn opt_span_add_array<Record>(
        &mut self,
        span: Opt<Span<LocalOrigin>>,
        new_items: Array<Item, Record>,
    ) -> Span<LocalOrigin> {
        match span {
            Opt::No(()) => self.add_array(new_items),
            Opt::Yes(span) => self.span_add_array(span, new_items),
        }
    }
    pub fn span_add(&mut self, span: Span<LocalOrigin>, new_item: Item) -> Span<LocalOrigin> {
        let moved_span = self.span_move_to_end(span);
        self.add(new_item);
        Span {
            start: moved_span.start,
            length: moved_span.length.saturating_add(1),
        }
    }
    pub fn span_move_to_end(&mut self, span: Span<LocalOrigin>) -> Span<LocalOrigin> {
        if self.span_is_at_the_end_of_items(&span) {
            return span;
        }
        // span is not at the end of iitems already

        let move_destination_start = self.items.len();
        std::iter::Extend::extend(
            &mut self.items,
            std::iter::Iterator::take(
                std::iter::repeat_with(|| std::option::Option::None),
                span.length.get() as usize,
            ),
        );
        _ = std::convert::TryInto::<u32>::try_into(self.items.len());
        let (before_move_destination, from_move_destination) =
            unsafe { self.items.split_at_mut_unchecked(move_destination_start) };
        unsafe { before_move_destination.get_unchecked_mut(span.to_range()) }
            .swap_with_slice(from_move_destination);
        self.first_unset_index = std::cmp::min(self.first_unset_index, span.start.index);
        Span {
            start: Slot::<LocalOrigin>::from_index(move_destination_start as u32),
            length: span.length,
        }
    }
    pub fn span_is_at_the_end_of_items(&self, span: &Span<LocalOrigin>) -> bool {
        span.start.index + span.length.get() < self.length_including_unset()
    }
    pub fn span_move_to_unset(&mut self, span: Span<LocalOrigin>) -> Span<LocalOrigin> {
        if !self.span_is_at_the_end_of_items(&span) {
            // moving this span would not reduce the amount of unset space
            return span;
        }
        // span is at the end of items

        let earlier_start_to_occupy_from = self.find_unset_length_positive(span.length);
        match earlier_start_to_occupy_from {
            std::option::Option::None => span,
            std::option::Option::Some(earlier_start_to_occupy_from) => {
                let (items_before_span, items_from_span) =
                    self.items.split_at_mut(span.start.index as usize);
                items_from_span[0..span.length.get() as usize].swap_with_slice(
                    &mut items_before_span[(earlier_start_to_occupy_from as usize)
                        ..(earlier_start_to_occupy_from + span.length.get()) as usize],
                );
                // we could alternatively have used splice. Not sure what's faster
                self.items
                    .truncate(self.items.len() - span.length.get() as usize);
                if self.first_unset_index == earlier_start_to_occupy_from {
                    self.first_unset_index = std::iter::Iterator::find_map(
                        &mut std::iter::Iterator::skip(
                            std::iter::Iterator::enumerate(self.items.iter()),
                            (earlier_start_to_occupy_from + span.length.get()) as usize,
                        ),
                        |(i, item)| match item {
                            std::option::Option::None => std::option::Option::Some(i as u32),
                            std::option::Option::Some(_) => std::option::Option::None,
                        },
                    )
                    .unwrap_or_else(|| u32::MAX);
                } else {
                    // self.first_unset_index is either u32::MAX or at an earlier index
                }
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
    pub fn unset_count(&self) -> u32 {
        self.unset_count_usize() as u32
    }
    pub fn unset_count_usize(&self) -> usize {
        std::iter::Iterator::count(std::iter::Iterator::filter(
            std::iter::Iterator::skip(self.items.iter(), self.first_unset_index as usize),
            |option| option.is_none(),
        ))
    }
    pub fn set_count(&self) -> u32 {
        self.set_count_usize() as u32
    }
    pub fn set_count_usize(&self) -> usize {
        std::iter::Iterator::count(std::iter::Iterator::filter(self.items.iter(), |option| {
            option.is_some()
        }))
    }
    /// The raw allocation. Can be used to create new Vecs or even
    /// to drop the memory in a separate thread
    pub fn into_unset_slice(self) -> Unset_slice<Item> {
        Unset_slice::from_vec_option(self.items)
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
impl<Item, LocalOrigin, Part> Buf<Origin<LocalOrigin, Part>, Item> {
    pub fn new(_: Origin<LocalOrigin, Part>) -> Self {
        Buf::<Origin<LocalOrigin, Part>, Item> {
            origin: std::marker::PhantomData::<Origin<LocalOrigin, Part>>,
            items: std::vec::Vec::new(),
            first_unset_index: u32::MAX,
        }
    }
    pub fn reuse(_: Origin<LocalOrigin, Part>, allocation: Unset_slice<Item>) -> Self {
        Buf::<Origin<LocalOrigin, Part>, Item> {
            origin: std::marker::PhantomData::<Origin<LocalOrigin, Part>>,
            first_unset_index: u32::MAX,
            items: allocation.into_vec_option(),
        }
    }
    pub fn origin_isolate<ItemErased>(
        self,
        item_erase: impl std::ops::Fn(Item) -> Origin_isolated<LocalOrigin, ItemErased>,
    ) -> Origin_isolated<LocalOrigin, Buf_origin_erased<Part, ItemErased>> {
        Origin_isolated {
            origin: std::marker::PhantomData::<LocalOrigin>,
            value_erased: Buf_origin_erased {
                erased: Buf {
                    origin: std::marker::PhantomData::<Origin<Erased, Part>>,
                    // the optimizer should be able to figure out that the atual memory does not change here
                    // when the `item_erase` really justs erases origins
                    items: std::iter::Iterator::collect(std::iter::Iterator::map(
                        std::iter::IntoIterator::into_iter(self.items),
                        |item| match item {
                            std::option::Option::None => std::option::Option::None,
                            std::option::Option::Some(item) => {
                                std::option::Option::Some(item_erase(item).value_erased)
                            }
                        },
                    )),
                    first_unset_index: self.first_unset_index,
                },
            },
        }
    }
}
impl<Item, Part> Buf<Origin<Erased, Part>, Item> {
    pub fn origin_unerase_keep_items<LocalOrigin>(
        self,
        _: &Origin_uneraser<LocalOrigin>,
    ) -> Buf<Origin<LocalOrigin, Part>, Item> {
        Buf {
            origin: std::marker::PhantomData::<Origin<LocalOrigin, Part>>,
            items: self.items,
            first_unset_index: self.first_unset_index,
        }
    }
    pub fn origin_unerase<LocalOrigin, ItemUnerased>(
        self,
        uneraser: &Origin_uneraser<LocalOrigin>,
        item_unerase: impl std::ops::Fn(
            Item,
            Origin_uneraser<LocalOrigin>,
        ) -> (ItemUnerased, Origin_uneraser<LocalOrigin>),
    ) -> Buf<Origin<LocalOrigin, Part>, ItemUnerased> {
        Buf {
            origin: std::marker::PhantomData::<Origin<LocalOrigin, Part>>,
            // the optimizer should be able to figure out that the atual memory does not change here
            // when the `item_unerase` really justs unerases origins
            items: std::iter::Iterator::collect(std::iter::Iterator::map(
                std::iter::IntoIterator::into_iter(self.items),
                |item| match item {
                    std::option::Option::None => std::option::Option::None,
                    std::option::Option::Some(item) => {
                        std::option::Option::Some(item_unerase(item, Origin_uneraser(uneraser.0)).0)
                    }
                },
            )),
            first_unset_index: self.first_unset_index,
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
    pub fn to_span(self) -> Span<Origin> {
        Span {
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
            index: self.index,
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
    pub fn split_start(self) -> Record·after·start<Opt<Span<Origin>>, Slot<Origin>> {
        Record·after·start {
            after: match std::num::NonZeroU32::new(p32_predecessor(self.length)) {
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(after_length) => Opt::Yes(Span {
                    start: Slot::<Origin>::from_index(self.start.index + 1),
                    length: after_length,
                }),
            },
            start: self.start,
        }
    }
    pub fn split_end(self) -> Record·before·end<Opt<Span<Origin>>, Slot<Origin>> {
        Record·before·end {
            end: Slot::<Origin>::from_index(self.end_index()),
            before: match std::num::NonZeroU32::new(p32_predecessor(self.length)) {
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(before_length) => Opt::Yes(Span {
                    start: Slot::<Origin>::from_index(self.start.index - 1),
                    length: before_length,
                }),
            },
        }
    }
    pub fn split_after_length_positive(
        self,
        start_length_or_greater: std::num::NonZeroU32,
    ) -> Record·after·start<Opt<Span<Origin>>, Span<Origin>> {
        let start_length =
            <std::num::NonZeroU32 as std::cmp::Ord>::min(start_length_or_greater, self.length);
        Record·after·start {
            after: match std::num::NonZeroU32::new(self.length.get() - start_length.get()) {
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(after_length) => Opt::Yes(Span::<Origin> {
                    start: Slot::<Origin>::from_index(self.start.index + start_length.get()),
                    length: after_length,
                }),
            },
            start: Span::<Origin> {
                start: self.start,
                length: start_length,
            },
        }
    }
    pub fn split_before_end_length_positive(
        self,
        end_length_or_greater: std::num::NonZeroU32,
    ) -> Record·before·end<Opt<Span<Origin>>, Span<Origin>> {
        let end_length =
            <std::num::NonZeroU32 as std::cmp::Ord>::min(end_length_or_greater, self.length);
        let before_length = self.length.get() - end_length.get();
        Record·before·end {
            end: Span::<Origin> {
                start: Slot::<Origin>::from_index(self.start.index + before_length),
                length: end_length,
            },
            before: match std::num::NonZeroU32::new(before_length) {
                std::option::Option::None => Opt::No(()),
                std::option::Option::Some(before_length) => Opt::Yes(Span::<Origin> {
                    start: self.start,
                    length: before_length,
                }),
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

impl<Origin> Opt<&Span<Origin>> {
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
pub fn u32_mul_wrap(Record·a·b { a, b }: Record·a·b<U32, U32>) -> U32 {
    a.wrapping_mul(b)
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
pub fn f32_square_root(n: F32) -> Opt<F32> {
    if n < 0.0 {
        Opt::No(())
    } else {
        Opt::Yes(n.sqrt())
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
pub fn str_chars_step<State>(
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
fn iterator_fold_in_direction<Item, State>(
    mut iterator: impl std::iter::DoubleEndedIterator<Item = Item>,
    direction: Choice·Down·Up<Record, Record>,
    state: State,
    step: impl std::ops::Fn(State, Item) -> State,
) -> State {
    match direction {
        Choice·Down·Up::Up(()) => std::iter::Iterator::fold(&mut iterator, state, step),
        Choice·Down·Up::Down(()) => {
            std::iter::Iterator::fold(&mut std::iter::Iterator::rev(iterator), state, step)
        }
    }
}
fn iterator_try_fold_in_direction<Item, B, C>(
    mut iterator: impl std::iter::DoubleEndedIterator<Item = Item>,
    direction: Choice·Down·Up<Record, Record>,
    state: C,
    step: impl std::ops::Fn(C, Item) -> std::ops::ControlFlow<B, C>,
) -> std::ops::ControlFlow<B, C> {
    match direction {
        Choice·Down·Up::Up(()) => std::iter::Iterator::try_fold(&mut iterator, state, step),
        Choice·Down·Up::Down(()) => {
            std::iter::Iterator::try_fold(&mut std::iter::Iterator::rev(iterator), state, step)
        }
    }
}
pub fn str_chars_step_while<Done, Going>(
    Record·direction·state·step·str {
        direction,
        str,
        state: initial_state,
        step,
    }: Record·direction·state·step·str<
        Choice·Down·Up<Record, Record>,
        Going,
        Fn<Record·char·state<Char, Going>, Choice·Done·Going<Done, Going>>,
        Str,
    >,
) -> Choice·Done·Going<Done, Going> {
    Choice·Done·Going::from_control_flow(iterator_try_fold_in_direction(
        str.str.chars(),
        direction,
        initial_state,
        |state, char| {
            Choice·Done·Going::into_control_flow(step(Record·char·state { state, char }))
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

pub fn span_start_index<Origin>(span: Span<Origin>) -> Record·index·span<U32, Span<Origin>> {
    Record·index·span {
        index: span.start.index,
        span: span,
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
pub fn opt_span_step<Origin, State>(
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
    match span {
        Opt::No(()) => initial_state,
        Opt::Yes(span) => span_step(Record·direction·span·state·step {
            direction: direction,
            span: span,
            state: initial_state,
            step: step,
        }),
    }
}
pub fn span_step<Origin, State>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Span<Origin>,
        State,
        Fn<Record·slot·state<Slot<Origin>, State>, State>,
    >,
) -> State {
    iterator_fold_in_direction(
        span.to_range_u32(),
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
pub fn done<Done, Going>(done: Done) -> Choice·Done·Going<Done, Going> {
    Choice·Done·Going::Done(done)
}
pub fn going<Done, Going>(going: Going) -> Choice·Done·Going<Done, Going> {
    Choice·Done·Going::Going(going)
}
pub fn opt_span_step_while<Done, Going, Origin>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Opt<Span<Origin>>,
        Going,
        Fn<Record·slot·state<Slot<Origin>, Going>, Choice·Done·Going<Done, Going>>,
    >,
) -> Choice·Done·Going<Record·done·rest<Done, Opt<Span<Origin>>>, Going> {
    match span {
        Opt::No(()) => Choice·Done·Going::Going(initial_state),
        Opt::Yes(span) => span_step_while(Record·direction·span·state·step {
            direction: direction,
            span: span,
            state: initial_state,
            step: step,
        }),
    }
}
pub fn span_step_while<Done, Going, Origin>(
    Record·direction·span·state·step {
        direction,
        span,
        state: initial_state,
        step,
    }: Record·direction·span·state·step<
        Choice·Down·Up<Record, Record>,
        Span<Origin>,
        Going,
        Fn<Record·slot·state<Slot<Origin>, Going>, Choice·Done·Going<Done, Going>>,
    >,
) -> Choice·Done·Going<Record·done·rest<Done, Opt<Span<Origin>>>, Going> {
    let state_after_fold = iterator_try_fold_in_direction(
        span.to_range_u32(),
        direction,
        initial_state,
        |state, index| {
            Choice·Done·Going::into_control_flow(step(Record·slot·state {
                state: state,
                slot: Slot::<Origin>::from_index(index),
            }))
            .map_break(|exit| (index, exit))
        },
    );
    match state_after_fold {
        std::ops::ControlFlow::Continue(state) => Choice·Done·Going::Going(state),
        std::ops::ControlFlow::Break((exit_index, exit_state)) => {
            let Record·after·start {
                start: _,
                after: not_folded_over_opt_span,
            } = span.split_after_length_positive(P32::MIN.saturating_add(exit_index));
            Choice·Done·Going::Done(Record·done·rest {
                done: exit_state,
                rest: not_folded_over_opt_span,
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
pub fn origin_unerase<LocalOrigin, Value, ValueErased>(
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

pub fn buf_empty<Item, LocalOrigin, Part>(
    origin: Origin<LocalOrigin, Part>,
) -> Buf<Origin<LocalOrigin, Part>, Item> {
    Buf::<Origin<LocalOrigin, Part>, Item>::new(origin)
}
pub fn buf_pre_allocate_at_least<Item, Origin>(
    Record·buf·length {
        mut buf,
        length: min_pre_allocated_length,
    }: Record·buf·length<Buf<Origin, Item>, u32>,
) -> Buf<Origin, Item> {
    buf.pre_allocate_at_least(min_pre_allocated_length);
    buf
}
pub fn buf_pre_allocation_rid<Item, Origin>(mut buf: Buf<Origin, Item>) -> Buf<Origin, Item> {
    buf.pre_allocation_rid();
    buf
}
pub fn buf_remove<Item, Origin>(
    Record·buf·slot { mut buf, slot }: Record·buf·slot<Buf<Origin, Item>, Slot<Origin>>,
) -> Record·buf·item<Buf<Origin, Item>, Item> {
    let item = buf.remove(slot);
    Record·buf·item {
        buf: buf,
        item: item,
    }
}
pub fn buf_replace<Item, Origin>(
    Record·buf·new·slot {
        mut buf,
        mut slot,
        new,
    }: Record·buf·new·slot<Buf<Origin, Item>, Item, Slot<Origin>>,
) -> Record·buf·item·slot<Buf<Origin, Item>, Item, Slot<Origin>> {
    let old_item = buf.replace(&mut slot, new);
    Record·buf·item·slot {
        buf: buf,
        slot: slot,
        item: old_item,
    }
}
pub fn buf_item_step<In, Item, Origin, Out>(
    Record·buf·in_·slot·step {
        mut buf,
        in_,
        mut slot,
        step,
    }: Record·buf·in_·slot·step<
        Buf<Origin, Item>,
        In,
        Slot<Origin>,
        Fn<Record·in_·item<In, Item>, Record·item·out<Item, Out>>,
    >,
) -> Record·buf·out·slot<Buf<Origin, Item>, Out, Slot<Origin>> {
    let out = buf.item_step(&mut slot, move |item| {
        let stepped = step(Record·in_·item {
            in_: in_,
            item: item,
        });
        (stepped.item, stepped.out)
    });
    Record·buf·out·slot {
        buf: buf,
        slot: slot,
        out: out,
    }
}
pub fn buf_swap<Item, Origin>(
    Record·buf·slot_a·slot_b {
        mut buf,
        mut slot_a,
        mut slot_b,
    }: Record·buf·slot_a·slot_b<Buf<Origin, Item>, Slot<Origin>, Slot<Origin>>,
) -> Record·buf·slot_a·slot_b<Buf<Origin, Item>, Slot<Origin>, Slot<Origin>> {
    buf.swap(&mut slot_a, &mut slot_b);
    Record·buf·slot_a·slot_b {
        buf: buf,
        slot_a: slot_b,
        slot_b: slot_a,
    }
}
pub fn buf_span_rid<Item, Origin>(
    Record·buf·item_rid·span {
        mut buf,
        item_rid,
        span,
    }: Record·buf·item_rid·span<Buf<Origin, Item>, Fn<Item, Record>, Span<Origin>>,
) -> Buf<Origin, Item> {
    buf.span_rid(span, item_rid);
    buf
}
pub fn buf_opt_span_rid<Item, Origin>(
    Record·buf·item_rid·span {
        mut buf,
        item_rid,
        span,
    }: Record·buf·item_rid·span<Buf<Origin, Item>, Fn<Item, Record>, Opt<Span<Origin>>>,
) -> Buf<Origin, Item> {
    buf.opt_span_rid(span, item_rid);
    buf
}
pub fn buf_rid<Item, Origin>(_: Buf<Origin, Item>) -> Record {}
pub fn buf_insert<Item, Origin>(
    Record·buf·new {
        mut buf,
        new: new_item,
    }: Record·buf·new<Buf<Origin, Item>, Item>,
) -> Record·buf·slot<Buf<Origin, Item>, Slot<Origin>> {
    let slot = buf.insert(new_item);
    Record·buf·slot {
        buf: buf,
        slot: slot,
    }
}
pub fn buf_add<Item, Origin>(
    Record·buf·new {
        mut buf,
        new: new_item,
    }: Record·buf·new<Buf<Origin, Item>, Item>,
) -> Record·buf·slot<Buf<Origin, Item>, Slot<Origin>> {
    let slot = buf.add(new_item);
    Record·buf·slot {
        buf: buf,
        slot: slot,
    }
}
pub fn buf_add_array<Item, Origin, Record>(
    Record·buf·new { mut buf, new }: Record·buf·new<Buf<Origin, Item>, Array<Item, Record>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let span = buf.add_array(new);
    Record·buf·span {
        buf: buf,
        span: span,
    }
}
pub fn buf_add_str_chars<Origin>(
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
pub fn buf_opt_span_reverse<Item, Origin>(
    Record·buf·span { mut buf, mut span }: Record·buf·span<Buf<Origin, Item>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Item>, Opt<Span<Origin>>> {
    buf.opt_span_slice_option_mut(&mut span).reverse();
    Record·buf·span {
        buf: buf,
        span: span,
    }
}
pub fn buf_span_reverse<Item, Origin>(
    Record·buf·span { mut buf, mut span }: Record·buf·span<Buf<Origin, Item>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    buf.span_slice_option_mut(&mut span).reverse();
    Record·buf·span {
        buf: buf,
        span: span,
    }
}

pub fn buf_opt_span_add<Item, Origin>(
    Record·buf·new·span {
        mut buf,
        span,
        new: new_item,
    }: Record·buf·new·span<Buf<Origin, Item>, Item, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    match span {
        Opt::No(()) => {
            let new_slot = buf.insert(new_item);
            Record·buf·span {
                buf: buf,
                span: slot_to_span(new_slot),
            }
        }
        Opt::Yes(span) => buf_span_add(Record·buf·new·span {
            buf: buf,
            span: span,
            new: new_item,
        }),
    }
}
pub fn buf_span_add<Item, Origin>(
    Record·buf·new·span {
        mut buf,
        span,
        new: new_item,
    }: Record·buf·new·span<Buf<Origin, Item>, Item, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let combined_span = buf.span_add(span, new_item);
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_span_add_array<Item, Origin, Record>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Item>,
        Array<Item, Record>,
        Span<Origin>,
    >,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let combined_span = buf.span_add_array(span, new);
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_opt_span_add_array<Item, Origin, Record>(
    Record·buf·new·span { mut buf, span, new }: Record·buf·new·span<
        Buf<Origin, Item>,
        Array<Item, Record>,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let combined_span = buf.opt_span_add_array(span, new);
    Record·buf·span {
        buf: buf,
        span: combined_span,
    }
}
pub fn buf_opt_span_add_str_chars<Origin>(
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
pub fn buf_span_add_str_chars<Origin>(
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
pub fn buf_span_add_u32_chars<Origin>(
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
pub fn buf_opt_span_add_u32_chars<Origin>(
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
pub fn buf_span_add_i32_chars<Origin>(
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
pub fn buf_opt_span_add_i32_chars<Origin>(
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
pub fn buf_span_add_f32_chars<Origin>(
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
pub fn buf_opt_span_add_f32_chars<Origin>(
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
pub fn buf_opt_span_add_buf_opt_span<Origin, SourceOrigin, Item>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Item>,
        Buf<SourceOrigin, Item>,
        Opt<Span<SourceOrigin>>,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·source·span<Buf<Origin, Item>, Buf<SourceOrigin, Item>, Opt<Span<Origin>>> {
    let combined_span = buf.opt_span_add_buf_opt_span(span, &mut source, source_span);
    Record·buf·source·span {
        source: source,
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_span_add_buf_opt_span<Origin, SourceOrigin, Item>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Item>,
        Buf<SourceOrigin, Item>,
        Opt<Span<SourceOrigin>>,
        Span<Origin>,
    >,
) -> Record·buf·source·span<Buf<Origin, Item>, Buf<SourceOrigin, Item>, Span<Origin>> {
    let combined_span = buf.span_add_buf_opt_span(span, &mut source, source_span);
    Record·buf·source·span {
        source: source,
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_span_add_buf_span<Origin, SourceOrigin, Item>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Item>,
        Buf<SourceOrigin, Item>,
        Span<SourceOrigin>,
        Opt<Span<Origin>>,
    >,
) -> Record·buf·source·span<Buf<Origin, Item>, Buf<SourceOrigin, Item>, Span<Origin>> {
    let combined_span = buf.opt_span_add_buf_span(span, &mut source, source_span);
    Record·buf·source·span {
        source: source,
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_span_add_buf_span<Origin, SourceOrigin, Item>(
    Record·buf·source·source_span·span {
        mut source,
        source_span,
        span,
        mut buf,
    }: Record·buf·source·source_span·span<
        Buf<Origin, Item>,
        Buf<SourceOrigin, Item>,
        Span<SourceOrigin>,
        Span<Origin>,
    >,
) -> Record·buf·source·span<Buf<Origin, Item>, Buf<SourceOrigin, Item>, Span<Origin>> {
    let combined_span = buf.span_add_buf_span(span, &mut source, source_span);
    Record·buf·source·span {
        source: source,
        span: combined_span,
        buf: buf,
    }
}

pub fn buf_span_add_own_span<Item, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Item>, Span<Origin>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let combined_span = buf.span_add_own_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_span_add_own_opt_span<Item, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Item>, Opt<Span<Origin>>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let combined_span = buf.span_add_own_opt_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_span_add_own_span<Item, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Item>, Span<Origin>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let combined_span = buf.opt_span_add_own_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_opt_span_add_own_opt_span<Item, Origin>(
    Record·buf·end·start {
        end,
        start,
        mut buf,
    }: Record·buf·end·start<Buf<Origin, Item>, Opt<Span<Origin>>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Item>, Opt<Span<Origin>>> {
    let combined_span = buf.opt_span_add_own_opt_span(start, end);
    Record·buf·span {
        span: combined_span,
        buf: buf,
    }
}
pub fn buf_span_move_to_unset<Item, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Item>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let moved_span = buf.span_move_to_unset(span);
    Record·buf·span {
        span: moved_span,
        buf: buf,
    }
}
pub fn buf_opt_span_move_to_unset<Item, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Item>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Item>, Opt<Span<Origin>>> {
    match span {
        Opt::No(()) => Record·buf·span {
            span: Opt::No(()),
            buf: buf,
        },
        Opt::Yes(span) => {
            let moved_span = buf.span_move_to_unset(span);
            Record·buf·span {
                span: Opt::Yes(moved_span),
                buf: buf,
            }
        }
    }
}
pub fn buf_span_move_to_end<Item, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Item>, Span<Origin>>,
) -> Record·buf·span<Buf<Origin, Item>, Span<Origin>> {
    let moved_span = buf.span_move_to_end(span);
    Record·buf·span {
        span: moved_span,
        buf: buf,
    }
}
pub fn buf_opt_span_move_to_end<Item, Origin>(
    Record·buf·span { span, mut buf }: Record·buf·span<Buf<Origin, Item>, Opt<Span<Origin>>>,
) -> Record·buf·span<Buf<Origin, Item>, Opt<Span<Origin>>> {
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
pub fn buf_to_unset<Item, Origin>(buf: Buf<Origin, Item>) -> Unset_slice<Item> {
    buf.into_unset_slice()
}
pub fn buf_reuse<LocalOrigin, Part, Item>(
    Record·origin·slice { origin, slice }: Record·origin·slice<
        Origin<LocalOrigin, Part>,
        Unset_slice<Item>,
    >,
) -> Buf<Origin<LocalOrigin, Part>, Item> {
    Buf::reuse(origin, slice)
}
fn buf_origin_isolate<Item, ItemErased, LocalOrigin, Part>(
    Record·buf·item_isolate { buf, item_isolate }: Record·buf·item_isolate<
        Buf<Origin<LocalOrigin, Part>, Item>,
        Fn<Item, Origin_isolated<LocalOrigin, ItemErased>>,
    >,
) -> Origin_isolated<LocalOrigin, Buf_origin_erased<Part, ItemErased>> {
    buf.origin_isolate(item_isolate)
}
pub fn buf_origin_unerase_keep_items<Item, LocalOrigin, Part>(
    Record·buf·uneraser { buf, uneraser }: Record·buf·uneraser<
        Buf_origin_erased<Part, Item>,
        Origin_uneraser<LocalOrigin>,
    >,
) -> Record·buf·uneraser<Buf<Origin<LocalOrigin, Part>, Item>, Origin_uneraser<LocalOrigin>> {
    let buf_unerased = buf.erased.origin_unerase_keep_items(&uneraser);
    Record·buf·uneraser {
        buf: buf_unerased,
        uneraser: uneraser,
    }
}
pub fn buf_origin_unerase<Item, ItemErased, LocalOrigin, Part>(
    Record·buf·item_unerase·uneraser {
        buf,
        item_unerase,
        uneraser,
    }: Record·buf·item_unerase·uneraser<
        Buf_origin_erased<Part, ItemErased>,
        Fn<
            Record·item·uneraser<ItemErased, Origin_uneraser<LocalOrigin>>,
            Record·item·uneraser<Item, Origin_uneraser<LocalOrigin>>,
        >,
        Origin_uneraser<LocalOrigin>,
    >,
) -> Record·buf·uneraser<Buf<Origin<LocalOrigin, Part>, Item>, Origin_uneraser<LocalOrigin>> {
    let buf_unerased = buf.erased.origin_unerase(&uneraser, |item, uneraser| {
        let Record·item·uneraser {
            item: item_erased,
            uneraser: eraser,
        } = item_unerase(Record·item·uneraser {
            item: item,
            uneraser: uneraser,
        });
        (item_erased, eraser)
    });
    Record·buf·uneraser {
        buf: buf_unerased,
        uneraser: uneraser,
    }
}

pub fn unset_slice_rid<Item>(_: Unset_slice<Item>) -> Record {}
pub fn unset_slice_allocate_length<Item>(length: U32) -> Unset_slice<Item> {
    Unset_slice::allocate_length(length)
}
pub fn unset_slice_cast_or_rid_and_allocate<Item, NewItem>(
    unset_slice: Unset_slice<Item>,
) -> Unset_slice<NewItem> {
    unset_slice.cast_or_rid_and_allocate::<NewItem>()
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
        std::assert_eq!(buf.as_slice().len(), 0);
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
