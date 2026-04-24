Bounds-checking in general is not slow (typically only around 2% slower than unchecked access in effect).
The goal here is representing tree-like data structures without plain integers (along with the need to handle failure), segmented memory, "garbage collection", "weak/strong references" or other memory-unoptimal patterns,
instead offering a safe, infallible way to refer to values stored in flatter memory structures.


# concept: each value only exists once
Each value can only be used used/consumed at most once.
Matching a value? Consumes it. Passing a value as an argument? Consumes it.
Even e.g. variables holding plain numbers have to be explicitly cloned if used in multiple places.
This can feel annoying and clunky. Think e.g. `fn vec-occupied-count (vec : vec<...>) -> (& vec(...) occupied-count(u32))`.
Not ony is it clunky, it is also conceptually less correct than taking an immutable view (aka &Vec in rust) because `vec-occupied-count` could return a modified vec.
The _big_ advantage is that it is _more intuitive_ and _way faster and simpler to statically analyze_ than lifetimes or similar, so it may just be worth it


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
Important: `vec` ranges/slots need to be manually "dropped"/removed from the backing vec!

# concept: distinct origin of a value in your code
(The idea of "fresh, distinct type instances by code" seems to generally be called "path-dependent types". In rust I know of 2 crates that successfully implement this: https://docs.rs/compact_arena/0.5.0/compact_arena/index.html (safe, pragmatic, simple but bare-bones) and https://docs.rs/indexing/0.4.1/indexing/ (safe, cumbersome, complicated))

Since any created value has a correlated origin, it can't escape the scope of it's origin
(Unlike plain ownership in rust which can be "created" in a scope and then "move to the caller/parent".
Values that contain linear structures in slot-lang follow borrowing rules of an owned (non-Copy) rust value in combination with a stored reference to a local allocator):

```
fn some-arena -> arena<??origin cannot even be annotated?? u32> (
    origin arena-origin
    is(arena-empty<u32> arena-origin) arena
    is(arena-push-back arena (123 : u32)) (& arena slot(_))
    arena
)
# compiles
fn add-some-values<Origin>(arena : arena<Origin u32>) -> arena<Origin u32> (
    arena-push-back arena (123 : u32)
)
```

# examples
## pass an origin form the outside (rare)
```
fn arena-empty<Origin> (origin : origin<Origin>) -> arena(Origin) # external
```
## creating a new origin, slots and slices
`origin some-name` creates a new origin variable and a local unique type for the start offset of its scope
An `origin` does not have a `-dup` helper.
At the end of the underlying origin of the annotated origin type, deallocate the memory of the value with that origin.
```
# use a temporary value within a scope
fn use-arena -> u32 (
    origin arena-origin
  	is(arena-empty<u32> arena-origin) arena
  	is(arena-push-back arena (123 : u32)) (& arena(arena) slot(first-slot))
  	is(arena-element arena first-slot) first # 123 : u32
  	is(arena-start-range arena) range-after-first
  	is(arena-range-push-back range-after-first (456 : u32) range-after-first) range-after-first
  	is(arena-range-push-back range-after-first (789 : u32) range-after-first) range-after-first
    is(arena-end-range range-after-first) (& arena(arena) range(range-after-first))
  	first
)
# different branches, different scopes
fn use-opt (opt : opt<u32>) -> Blank (
    # this won't compile as their origins come from different branches
    is(
        is(opt)
        Absent<u32> (
            origin vec-origin
            arena-empty<u32> vec-origin
        )
        (Present number) (
            origin vec-origin
            arena-one vec-origin number
        )
    )
    vec
    # this will compile:
    origin vec-origin
    is(
        is(opt)
        Absent (arena-empty<u32> vec-origin)
        (Present number) (arena-one vec-origin number)
    )
    vec
    Blank
)

# recursive structure. One cool thing is that expression will turn every slot
# into an exclusive slot
choice expression<Expressions-origin Patterns-origin> (
    (Int<Expressions-origin Patterns-origin> int64)
    (String<Expressions-origin Patterns-origin> str)
    (Vec<Patterns-origin> range<Expressions-origin>)
    (Call<Patterns-origin> &
        function(slot<Expressions-origin>)
        argument0(slot<Expressions-origin>)
        argument1-up(range<Expressions-origin>)
    )
    (Lambda &
        parameter0(slot<Patterns-origin>)
        parameter1-up(range<Patterns-origin>)
        result(slot<Expressions-origin>)
    )
)

type state<expressions-origin> (&
    # ...patterns, strings etc
    expressions(vec<expressions-origin expression<expressions-origin>>)
    root-expression(expression<expressions-origin>)
)
fn initial-state (expressions-origin : origin(Expressions-origin)) -> state<Expressions-origin> (&
    expressions(vec-empty<expression<Expressions-origin, ...>> expressions-origin)
    root-expression(todo "do parsing")
)
fn state-to-interface
    (interfaces-origin : origin<Interfaces-origin>)
    (state : state<Expressions-origin>)
-> arena<Interfaces-origin interface<state<expressions-origin>>> (
    is(arena-one interfaces-origin (Console-log<never>"hello"))) (& index(_) arena(interfaces))
    interfaces
)
```

# on shadowing
since each variable can be used at most once, most introduced names that would traditionally be considered "shadowed" are aready out of scope in slong

# known limitations
- nested sub-ranges/slots in a vec cannot be easily de-allocated in bulk (so without walking the whole syntax tree and removing ranges and slots one by one, aka pointer chasing).
Preferably, expressions etc. would be stored in different ranges per module, each with their own origin for bulk de-allocation and new-allocation.
However, this means that slots and ranges within the AST are non-owning
- the pattern of removing, then re-inserting an element at a slot just to access it (potentially immutably) is not optimal. This can be mitigated somewhat by using `vec-update(fn(Element) -> Element)` or compiling to/asking for code that uses `arena-set(slot, new-element) -> & slot() old-element()` with a dummy element followed by `arena-set(modified-old-element)` ignoring the returned dummy new-element instead

# TODO
- allow last is case result to spill, allow lambda fn result to spill, allow the last record field value to spill, do not allow last call argument to spill (including in dot call)
- consider adding vec-counting which can re-issue a slot that is already in use, effectively Slab<Rc<_>> but flattened in memory and not supporting weak references. This would enable graph structures, child-parent relations etc. `fn vec-counting-add-slot-use(..., slot(Origin)) -> (& vec() slot(slot<Origin>))` and `fn vec-counting-remove-slot-use(..., slot(Origin))`
- add slot-to-unt, range-start-slot, range-end-slot, range-slots-fold etc
- remove need for comma in parameter and argument list, potentially drop the outer parens
- do actually merge arena and vec if arena is basically a vec with an empty unoccupied-list
- add tuples: (* a b c). I dislike them conceptually but operations like `u32-dup` (and positional arguments?) kind-of require them.

# not coherently formulated thoughts
in rust, collections tend to own their element data, so safely keeping references to inside is tough.
This relationship is flipped on it's head in slot-lang: All elements of collections are divided into slots and ranges
which are owned by the code that parked values there i the first place.

Honestly this idea seems to overwhelmingly useful that I'm surprised I can't find other languages that lean into it (I only know of rust which at least enables it in userland).

One way this helps is that nested collections aren't segmented: what is usually `Vec<Box<str>>` aka n separate memory pieces can be e.g. `vec<range<str-origin>>` + `str<str-origin>`
(in rust there is I think an oroborus crate for this)

# rejected ideas
- (leaning no) consider _requiring_ single-reference values to be used (this would imply e.g. introducing arena-free() and vec-free() and unnecessarily returning slots and ranges to the origin arena. Not very ergonomic)
- (leaning no) add dot-call syntax sugar: `construct-argument0.function(argument1-up)` as potential alternative to `is construct-argument0 argument0 function(argument0, )`.
  Issue is that in general single-return-continuation is rare in slot-lang
- (leaning no) consider requiring all (!) generic type parameters to be passed to calls and variants, e.g.
  ```
  choice Choice<Value> | Variant Value
  
  fn take-variant (Choice<u32>.Variant <u32>value) -> Blank (
    dup3<u32>(u32)
  )
  ```
  I feel like this is more "natural", easier to type-check but way more verbose / redundant
