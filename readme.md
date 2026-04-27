Small, fast pure functional programming language where indexes are valid and values can't be shared.

The goal is representing tree-like data structures without segmented memory or plain index integers (along with the need to handle failure and generations),
instead offering a safe, infallible way to refer to values and ranges stored in flat memory structures.

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
- values can mutated without mutation being detectable in another place

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
This can be used to "return" memory which has become invalid with `vec-remove vec slot` and `vec-remove-range vec range`

This concept is often called slot map, reusing memory.
Important: `vec` ranges/slots need to be manually "dropped"/removed from the backing vec if that backing vec is persistent.


# concept: distinct origin of a value in your code
(The idea of "fresh, distinct type instances by code" seems to generally be called "path-dependent types". In rust I know of 2 crates that successfully implement this: https://docs.rs/compact_arena/0.5.0/compact_arena/index.html (safe, pragmatic, simple but bare-bones) and https://docs.rs/indexing/0.4.1/indexing/ (safe, cumbersome, complicated))

Since any created value has a correlated origin and explicit function result types are required, a value whose type contains an origin can't escape the function scope of it's origin
(Unlike plain ownership in rust which can be "created" in a scope and then "move to the caller/parent".
Values that contain linear structures in sloe follow borrowing rules of an owned (non-Copy) rust value in combination with a stored reference to a local allocator):
```
fn some-arena -> arena ??origin cannot even be annotated?? u32 (
    origin arena-origin
    is(arena-empty<u32> arena-origin) arena
    is(arena-push arena (123 u32)) (& (arena arena) (slot _))
    arena
)
# compiles
fn add-some-values<Origin> (arena arena Origin u32) -> (arena Origin u32) (
    is(arena-push arena (123 u32)) (& (arena arena) (slot _))
    arena
)
```

# examples
## pass in an origin from the outside (rare)
```
fn arena-empty<Element> (origin Origin) -> arena Origin Element # external
```
shift the responsibility for cleanup to the caller.
This is done for most initializer functions, e.g. for the initial persistent application state.

## creating a new origin, slots and ranges
`origin some-name` creates a new origin variable and a local unique type for the start offset of its scope
Each origin does not have a `-dup` helper.
At the end of the underlying origin of the annotated origin type, deallocate the memory of the value with that origin.
```
# use a temporary value within a scope
fn use-arena -> u32 (
    origin arena-origin
  	is(arena-empty<u32> arena-origin) arena
  	is(arena-push arena (123 u32)) (& (arena arena) (slot first-slot))
  	is(arena-element arena first-slot) first # 123 u32
  	is(arena-start-range arena) range-after-first
  	is(arena-range-push range-after-first (456 u32) range-after-first) range-after-first
  	is(arena-range-push range-after-first (789 u32) range-after-first) range-after-first
    is(arena-end-range range-after-first) (& (arena arena) (range range-after-first))
  	first
)
# different branches, different scopes
fn use-opt (opt opt u32) -> Blank (
    # this won't compile as their origins come from different branches
    is(
        is(opt)
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
    is(
        is(opt)
        (Absent arena-empty<u32> vec-origin)
        ((Present number) arena-one vec-origin number)
    )
    vec
    Blank
)

# recursive structure. One cool thing is that expression will turn every slot
# into an exclusive slot
choice expression Expressions-origin Patterns-origin Str-origin (
    (Int<Expressions-origin Patterns-origin> int64)
    (String<Expressions-origin Patterns-origin> range Str-origin)
    (Vec<Patterns-origin> range<Expressions-origin>)
    (Call<Patterns-origin> &
        (function slot Expressions-origin)
        (argument0 slot Expressions-origin)
        (argument1-up range Expressions-origin)
    )
    (Lambda &
        (parameter0 slot Patterns-origin)
        (parameter1-up range Patterns-origin)
        (result slot Expressions-origin)
    )
)

type state Expressions-origin (&
    # ...patterns, strings etc
    (expressions vec Expressions-origin (expression Expressions-origin))
    (root-expression expression Expressions-origin)
)
fn initial-state (expressions-origin origin Expressions-origin) -> state Expressions-origin (&
    (expressions vec-empty<expression Expressions-origin ...> expressions-origin)
    (root-expression todo "do parsing")
)
fn state-to-interfaces-into
    (interfaces arena Interfaces-origin)
    (state state Expressions-origin)
-> (arena Interfaces-origin (interface state Expressions-origin)) (
    is(arena-one interfaces-origin (Console-log<never> "hello"))) (& (index _) (arena interfaces))
    interfaces
)
```

# on shadowing
since each variable can be used at most once, most introduced names that would traditionally be considered "shadowed" are aready out of scope in sloe

# known limitations
- nested sub-ranges/slots in a persistent vec cannot be easily de-allocated in bulk (so without walking the whole syntax tree and removing ranges and slots one by one, aka pointer chasing).
Preferably, expressions etc. would be stored in different ranges per module, each with their own origin for bulk de-allocation and new-allocation.
However, this means that slots and ranges within the AST are non-owning
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

# project type alias to give a short name for a more elaborate type to shorten annotations
type type-name-alias Potential Type-Parameters (&
    (u32s vec Potential u32)
    (f32s vec Type-Parameters u32)
)

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
- merge arena and vec types if arena is basically a vec with an empty unoccupied-list and make `arena` basically just an initialization option.
  This makes it easy to reuse types containing vec for arena and vice versa
- consider adding `slot-weak` which can reference a slot that is already in use, without any guarantee that it still points to an occupied slot: `slot-dup-weak (slot slot Origin) -> & (slot slot Origin) (weak slot-weak Origin)`. This would enable graph structures, child-parent relations etc.
  **important**: This requires generational slots, so most likely a different vec and slot type
- add slot-to-range, `fn range-pop-front/back (range Origin) -> (range range Origin) (slot slot Origin)`, `range-slots-fold` or `arena/vec-vacate-range-fold` etc
- add tuples: (* a b c). I dislike them conceptually but operations like `u32-dup` are much nicer with them.
  This would also make "positional arguments" not something special:
  `fn name* first second third` used as `vec-push* some-vec some-element` (as opposed to e.g. `vec-push& (vec vec) (element element)`).
  This is more verbose though.
- change `is()` to `..()`
- consider adding special syntax `fn-once` that automatically assembles the environment from the used local variables
- verify this is corrct for all kinds of recursion! e.g. this one seems on the edge of correct:
  _different vecs have the same origin_ but their slots can't intermix.
  ```
  fn recurse (consume-origin Consume-origin) (result-origin Result-origin) -> (vec Result-origin u32) (
      origin local-origin
      is(vec-empty<u32> consume-origin) temporary
      is(recurse local-origin result-origin) result
      is(vec-push temporary (1 u32)) (& (slot _) (vec _))
      result
  )
  ```
  If we find a problem, creating a new `origin` should be disallowed in (mutually) recursive calls.
  This is a bit restrictive but alright I believe.
  If feeling motived, look into proof languages and make sure this is rock solid

# not coherently formulated thoughts
in rust, collections tend to own their element data, so safely keeping references to inside is tough.
This relationship is flipped on it's head in sloe: All elements of collections are divided into slots and ranges
which are owned by the code that parked values there i the first place.

Honestly this idea seems to overwhelmingly useful that I'm surprised I can't find other languages that lean into it (I only know of rust which at least enables it in userland).

One way this helps is that nested collections aren't segmented: what is usually `Vec<Box<str>>` aka n separate memory pieces can be e.g. `vec<range<str-origin>>` + `str<str-origin>`
(in rust there is I think an oroborus crate for this)

# rejected ideas
- add slot-to-u32. Is there a use for that?
- convert values from "affine" (<= 1 use) to "linear" (exactly 1 use) to avoid potential leaks (https://smallcultfollowing.com/babysteps/blog/2023/03/16/must-move-types/). I think this would work great but leads to a bunch of unreasonable cleanup for arena members (which most likely would get optimized away though)
- (leaning no) consider _requiring_ single-reference values to be used (this would imply e.g. introducing arena-free() and vec-free() and unnecessarily returning slots and ranges to the origin arena. Not very ergonomic)
- (leaning no) add dot-call syntax sugar: `construct-argument0.function(argument1-up)` as potential alternative to `is construct-argument0 argument0 function(argument0, )`.
  Issue is that in general single-return-continuation is rare in sloe
- (leaning no) consider requiring all (!) generic type parameters to be passed to calls and variants, e.g.
  ```
  choice Choice<Value> ( (Variant Value) )
  
  fn take-variant (Choice<u32>.Variant <u32>value) -> Blank (
      dup3<u32> u32-dup value
  )
  fn dup3<Value> (dup : fn Value -> & (a Value) (b Value)) (value : Value) -> (& (a Value) (b Value) (c Value)) (
      is(dup value) (& (a a) (b temp))
      is(dup temp) (& (a b) (b c))
      & (a a) (b b) (c c)
  )
  ```
  I feel like this is more "natural", easier to type-check but way more verbose / redundant
