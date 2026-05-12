Small, fast pure functional programming language where indexes are valid and values can't be shared.

The goal is representing tree-like data structures without segmented memory or plain index integers (along with the need to handle failure and generations),
instead offering a safe, infallible way to refer to values and spans stored in flat memory structures.

[skip to examples](#examples)

Note that while as a side effect this avoids any bounds checks,
bounds-checking in general is not slow (typically only around 2% slower than unchecked access in practice).

# install
> This language is just a design for now. There is no tooling implementation to install. Once there is something:

```bash
cargo install --git https://github.com/lue-bird/sloe sloe
```

# concept: each value can only be used used/consumed at most once
Matching a value? Consumes it. Passing a value as an argument? Consumes it.
Even e.g. variables holding plain numbers have to be explicitly duplicated to use them in multiple places.

This allows
- values know when they aren't used anymore at compile time. Their memory can be reclaimed without garbage collection or similar
- values can be mutated internally without mutation being detectable
- representing things that can only be consumed once, like thread join handles

This can feel annoying and clunky. Think e.g. `fn vec-occupied-count (vec vec ...) -> (& (vec ...) (occupied-count u32))`.
Not ony is it clunky, it is also conceptually less constrained than taking an immutable view (like &Vec in rust) because `vec-occupied-count` could return a modified vec.

The _big_ advantage is that it is easy to understand and _way simpler and faster to statically analyze_ than lifetimes or similar.

Further reading if interested: "affine types", rust owned values.


# concept: flat memory collections
## `arena`
temporary, append-only arena, bumping + bulk de-allocation: just a plain vec without the ability to remove, could alternatively be implemented using SmallArena https://docs.rs/compact_arena/0.5.0/compact_arena/struct.SmallArena.html or ExternalStableVec https://github.com/LukasKalbertodt/stable-vec
Use for things like building a formatted string, then writing it into a file. After that, the string can be cleared.
Choosing `arena` for deletion-heavy state of long-ish-running programs will be a memory leak.

## `vec`
Only bulk-de-allocating an `arena` that is introduced in the main loop (persistent application state) once it goes out of scope (aka the program exits)
would be a (safe but bad) memory leak.

A better solution: Introduce a collection which can mark some parts of itself as onuccupied.
This can be used to "return" memory which has become invalid with `vec-vacate vec slot` and `vec-vacate-span vec span`

This concept is often called slot map, reusing memory.
Important: `vec` spans/slots need to be manually "dropped"/removed from the backing vec if that backing vec is persistent. In rust, a prominent example is [slab](https://docs.rs/crate/slab/latest).
Various kinds of rust collections are compared here: https://donsz.nl/blog/arenas/


# concept: distinct origin of a value in your code
Every created collection has a correlated origin.
Since also explicit function result types are required, a value whose type contains an origin can't escape the function scope of it's origin
```
fn some-arena -> (arena ??origin cannot even be annotated?? u32) (
    origin arena-origin
    :(arena-empty<u32> arena-origin) arena
    :(arena-push arena (123 u32)) (& (arena arena) (slot _))
    arena
)
# compiles
fn add-some-values<Origin> (arena arena Origin u32) -> (arena Origin u32) (
    :(arena-push arena (123 u32)) (& (arena arena) (slot _))
    arena
)
```

Further reading if interested: In effect, collections in sloe follow rust borrowing rules similar to an owned (non-Copy) rust value in combination with a stored reference to a distinct local allocator. The idea of "fresh, distinct type instances by code" seems to generally be called "path-dependent types". In rust I know of 2 crates that successfully implement this: https://docs.rs/compact_arena/0.5.0/compact_arena/index.html (safe, pragmatic, simple but bare-bones) and https://docs.rs/indexing/0.4.1/indexing/ (safe, cumbersome, complicated).
The same idea but with runtime checking instead of compile-time checking can quite easily be implemented by storing an ID in each collection and the same id in each contained slot, and incrementing a global variable for the next available ID: https://github.com/thomcc/handy/blob/master/src/lib.rs#L111-L126
(apart from security I'm not sure this is ever worth it for regular users, considering it is also slower).

# examples
## pass in an origin from the outside (rare)
```
fn arena-empty<Element> (origin Origin) -> (arena Origin Element) # external
```
shift the responsibility for cleanup to the caller.
This is done for most initializer functions, e.g. for the initial persistent application state.

## creating a new origin, slots and spans
`origin some-name` creates a new origin variable and a local unique type for the start offset of its scope.
An origin type does not have a `-dup` helper and thus can only be used for one collection.
At the end of the underlying origin of the annotated origin type, the memory of the value with that origin will be deallocated.
```
# use a temporary value within a scope
fn use-arena -> u32 (
    origin arena-origin
  	:(arena-empty<u32> arena-origin) arena
  	:(arena-push arena (123 u32)) (& (arena arena) (slot first-slot))
  	:(arena-element arena first-slot) (& (arena arena) (element first)) # 123 u32
  	:(arena-start-span arena) span-after-first
  	:(arena-span-push span-after-first (456 u32) span-after-first) span-after-first
  	:(arena-span-push span-after-first (789 u32) span-after-first) span-after-first
    :(arena-end-span span-after-first) (& (arena arena) (span span-after-first))
  	first
)
# different branches, different scopes
fn use-opt (opt opt u32) -> Blank (
    # this won't compile as their origins come from different branches
    :(
        :opt
        (Absent
            origin vec-origin
            arena-empty<u32> vec-origin
        )
        ((Present number)
            origin vec-origin
            arena-one vec-origin number
        )
     )
    vec
    # this will compile:
    origin vec-origin
    :(
        :(opt)
        (Absent arena-empty<u32> vec-origin)
        ((Present number) arena-one vec-origin number)
    )
    vec
    Blank
)

# recursive structure. One cool thing is that expression will turn every slot
# into an exclusive slot
choice expression Expressions-origin Patterns-origin Str-origin (
    (Int<Expressions-origin Patterns-origin Str-origin> int64)
    (String<Expressions-origin Patterns-origin> span Str-origin)
    (Vec<Patterns-origin Str-origin> span<Expressions-origin>)
    (Call<Patterns-origin Str-origin> &
        (function slot Expressions-origin)
        (argument0 slot Expressions-origin)
        (argument1-up span Expressions-origin)
    )
    (Lambda<Str-origin> &
        (parameter0 slot Patterns-origin)
        (parameter1-up span Patterns-origin)
        (result slot Expressions-origin)
    )
)

choice state Expressions-origin (
    State &
        # ...patterns, strings etc
        (expressions vec Expressions-origin (expression Expressions-origin))
        (root-expression expression Expressions-origin)
)
fn initial-state (expressions-origin origin Expressions-origin) -> state Expressions-origin (
    State &
    (expressions vec-empty<expression Expressions-origin ...> expressions-origin)
    (root-expression todo "do parsing")
)
fn state-to-interfaces-into
    (interfaces arena Interfaces-origin)
    (state state Expressions-origin)
-> (arena Interfaces-origin (interface state Expressions-origin)) (
    :(arena-one interfaces-origin (Console-log<never> "hello"))) (& (slot _) (arena interfaces))
    interfaces
)
```

# on shadowing
since each variable can be used at most once, most introduced names that would traditionally be considered "shadowed" are aready out of scope in sloe

# known limitations
- nested sub-spans/slots in a persistent vec cannot be easily de-allocated in bulk (so without walking the whole syntax tree and removing spans and slots one by one, aka pointer chasing).
Preferably, expressions etc. would be stored in different spans per module, each with their own origin for bulk de-allocation and new-allocation.
However, this means that slots and spans within the AST are non-owning
- the pattern of removing, then re-inserting an element at a slot just to access it (potentially immutably) is not optimal. This can be mitigated somewhat by using `vec-update vec (fn(Element) -> Element)` or compiling to/asking for code that uses `arena-set slot new-element -> & slot() old-element()` with a dummy element followed by `arena-replace vec modified-old-element` ignoring the returned dummy new-element instead

# syntax
Syntax is secondary but I tried to make it coherent and practical, avoiding parens and indentation when possible, especially for trailing syntax.
```
# line comment

# number type, so for example
3.2 f32 # number types are u8, u16, u32, u64, i8, i16, i32, i64, f32, f64

# str
"hello"

# char
'a'

# most identifiers
some-function-or-variable-or-field-or-type-name

# other identifiers
Some-variant-or-type-variable-name

# function call. Requires type arguments in <...> for certain functions
# function can be of type `fn` or `fn-once`
some-function<Type Arguments> first (inner-call-as-the-second-argument inner-first)

# record
& (first-field first-value) (second-field second-value)

# local fn of type fn.
# can **not** use local variables from the outer scope. see the fn-once core type for these
fn first-parameter-pattern second-parameter-pattern -> required-result-type result

# pattern variable
# appending a type is required in function parameters. this can look confusing at first but is more consistent with fields, making the switch from positional to named arguments easy
some-variable some-type

# pattern (temporary) leak.
# Conveniently skip handling a value and let it leak until the structure that contains it goes out of scope
# appending a type is required in function parameters
_ some-type

# pattern match. The last case does not need to be parenthesized. Cases are checked for exhaustiveness
is value (first-case-pattern first-result) (second-case-pattern second-result)

# introduce a new origin
origin new-origin-name

# project function declaration
fn function-name<Potential Type-Arguments Only-Used-In-The-Result> first-argument-pattern second-argument-pattern
    -> result-type
    result-expression-usually-wrapped-in-parens

# note that there are no "project value declarations"
# and that functions without arguments are automatically applied when their name is used.
f32-pi # f32, not fn -> f32

# to actually use it as a lazy function, explicitly wrap it in a local fn
fn -> f32 f32-pi

# project type that can come in different shapes ("variants")
# which each have a unique uppercase name and 0 or 1 associated value.
# If a variant doesn't use all type variables of the type, they need to be specified within <>
choice type-name Potential Type-Parameters (
    First-Option<Potential Type-Parameters>
    (Second-option<Type-Parameters> vec Potential u32)
    (Third-option type-name-alias Potential Type-Parameters)
)
```
(This list is incomplete, examples may show more)

# TODO
- what should actually be done about element access into temporary collections (especially into arena)? Currently a collection deallocates its content after the lexical scope of its origin is exited.
  If we aren't careful (copying their element content instead of referencing it!)
- rename element_span to elements
- consider adding `vec-generational`, `slot-strong` and `slot-weak` which can reference a slot that is already in use, without any guarantee that it still points to an occupied slot: `slot-dup-weak (slot-strong slot Origin) -> & (slot slot-strong Origin) (weak slot-weak Origin)` + `slot-weak-dup`. This would enable graph structures, child-parent relations, doubly-linked lists etc.
  **important**: This requires generational slots.
  Can be implemented using e.g. [slotmap](https://docs.rs/crate/slotmap/latest), [thunderdome](https://docs.rs/crate/thunderdome/latest), [riddance](https://docs.rs/riddance/latest/riddance/) or on top of `vec` with an added generation counter in slot and collection. TODO what about spans?
- add `slot-to-span`, `fn span-pop-front/back (span Origin) -> (span span Origin) (slot slot Origin)`, `span-slots-fold` or `arena/vec-vacate-span-fold` etc
- add tuples: (* a b c). I dislike them conceptually but operations like `u32-dup` are much nicer with them.
  This would also make "positional arguments" not something special:
  `fn name* first second third` used as `vec-add* some-vec some-element` (as opposed to e.g. `vec-add& (vec vec) (element element)`).
  This is more verbose though.
- consider adding special syntax `fn-once` that automatically assembles the environment from the used local variables
- verify this is corrct for all kinds of recursion! e.g. this one seems on the edge of correct:
  _different vecs have the same origin_ but their slots can't intermix.
  ```
  fn recurse (consume-origin Consume-origin) (result-origin Result-origin) -> (vec Result-origin u32) (
      origin local-origin
      :(vec-empty<u32> consume-origin) temporary
      :(recurse local-origin result-origin) result
      :(vec-add temporary (1 u32)) (& (slot _) (vec _))
      result
  )
  ```
  If we find a problem, creating a new `origin` should be disallowed in (mutually) recursive calls.
  This is a bit restrictive but alright I believe.
  If feeling motived, look into proof languages and make sure this is rock solid
- figure out strings. Definitely "abc" is of type str and slicing that should give str as well. I think for dynamic strings we'll use arena<char> and vec<char> for now (memory inefficient), with potential improvements to array-of-tagged-union (choice (ascii u8) (unicode u32)) or something
- add `set Origin Element` with a initialization function like `set-empty (origin ...) (hash fn Element -> Hash) -> set Origin Element`
- add something like `map Origin Key Value` which still gives out `slot Origin`s for each entry but can be queried using e.g. `map-contains-key (map ...) (key Key) (value-dup ...) -> & (map ...) (contains-key bool)`. `map-empty` will require providing a `fn Key Key -> order`.
  Alternatively, check if implementing in userland via e.g. AVL or red-black tree backed by a regular `vec`/`arena` is fast enough

# potential improvements in the far future
I think in theory there should be all the bits and pieces present to allow for struct-of-arrays and arrays-of-variant-values (made up name). E.g. internally compiling
- `vec Origin (& (a A) (b B))` to `A·B<Vec<A>, Vec<B>>`
- for `choice A-or-b A B ((A A) (B B))`: `vec Origin A-or-b` to either 
    - `Tag·ValueIndex·A·B<Vec<A_or_B_Tag>, Vec<u32>, Vec<A>, Vec<B>>` (which also has ~2 hops but makes sense when sizes of A and B are different enough)
    - `A·B<Vec<A>, Vec<B>>` (which requires the index to hold both the tag and the value index, aka 64 bit instead of 32, which somewhat defeats the point of reducing padding of the variant when values get bigger. Potentially there could however be struct-of-arrays for individual variant values making this worth it: https://github.com/dist1ll/osmium & https://alic.dev/blog/dense-enums)
    - `A·B<Vec<A>, Map<u32, B>>` (which is inefficient, and wasteful if `B` is common, and also doesn't scale with more than 2 variants)
- TODO look into soa_derive for rust, maybe this already does most of the useful work.
- consider instead leaning heavily into making variant values themselves small, e.g. using `NonZeroU32` (aka maybe `p32`?)
- try adding a compiler output to zig or similar which I think fits well (few free(), no need for lifetimes, fast compilation, anonymous structs, allocators, MultiArrayList. Downsides: pattern matching is less developed I think? ecosystem, no language-level ownership, making sloe values prone to mistakes south of the the ffi border) and the overall philosophy (explicitness, data oriented)
- imagine what a logic programming language with this concept would look like. I imagine it wouldn't look much different (!) though with some different tradeoffs (e.g. more complex stdlib and compiler output, potentially a different typing and exhaustivess system)

# not coherently formulated thoughts
in rust, collections tend to own their element data, so safely keeping references to inside is tough.
This relationship is flipped on it's head in sloe: All elements of collections are divided into slots and spans
which are owned by the code that parked values there i the first place.

Honestly this idea seems to overwhelmingly useful that I'm surprised I can't find other languages that lean into it (I only know of rust which at least enables it in userland).

One way this helps is that nested collections aren't segmented: what is usually `Vec<Box<str>>` aka n separate memory pieces can be e.g. `vec (span str-origin)` + `str str-origin`
(in rust there is I think an oroborus crate for this)

# rejected ideas
- add type alias declaration syntax
  ```
  # project type alias to give a short name for a more elaborate type to shorten annotations
  type type-name-alias Potential Type-Parameters (&
      (u32s vec Potential u32)
      (f32s vec Type-Parameters u32)
  )
  ```
- add slot-to-u32. Is there a use for that?
- convert values from "affine" (<= 1 use) to "linear" (exactly 1 use) to avoid potential leaks (https://smallcultfollowing.com/babysteps/blog/2023/03/16/must-move-types/). I think this would work great but leads to a bunch of unreasonable cleanup for arena members (which most likely would get optimized away though): It would imply e.g. introducing arena-free and vec-free and unnecessarily returning slots and spans to the origin arena. Not very ergonomic
- (leaning no) add dot-call syntax sugar: `construct-argument0.function(argument1-up)` as potential alternative to `is construct-argument0 argument0 function(argument0, )`.
  Issue is that in general single-return-continuation is rare in sloe
- (leaning no) consider requiring all (!) generic type parameters to be passed to calls and variants, e.g.
  ```
  choice Choice Value (
      Variant Value
  )
  
  fn take-variant (Choice<u32>.Variant (value u32)) -> Blank (
      :(dup3 u32-dup value) _
      Blank
  )

  fn dup3
      (dup fn Value -> & (old Value) (new Value))
      (value Value)
      -> (& (a Value) (b Value) (c Value))
      (
      :(fn-dup dup) & ((old dup0) (new dup1))
      :(dup0 value) (& (old a) (new temp))
      :(dup1 temp) (& (old b) (new c))
      & (a a) (b b) (c c)
  )
  ```
  I feel like this is more "natural", easier to type-check but way more verbose / redundant
