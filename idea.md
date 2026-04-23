Bounds-checking in general is not slow (typically only around 2% slower than unchecked access in effect).
The goal here is representing tree-like data structures without plain integers (along with the need to handle failure), segmented memory, "garbage collection", "weak/strong references" or other memory-unoptimal patterns,
instead offering a safe, infallible way to refer to values stored in flatter memory structures.


# concept: how many are referring to this value?
! or single-reference value can only be used at most once.

Since any created !value has a correlated origin, it can't escape the scope of it's origin
(unlike plain ownership in rust which can `move`
but ownership in combination with a stored reference to a local allocator):

```
fn some-arena() -> !arena<??origin cannot even be annotated?? unt32> (
    origin arena-origin
    let arena (arena-empty<unt32>(arena-origin))
    let { arena slot(_) } (arena-push-back(arena, <unt32>123))
    arena
)
# compiles
fn add-some-values<Origin>(<!arena<Origin unt32>>arena) -> !arena<Origin unt32> (
    arena-push-back(arena, <unt32>123)
)
```

shared (immutable, many-references or by-memcopy value, without !) values of single-reference values cannot escape their scope:
```
fn vec-sum<Origin>(<vec<Origin unt32>>vec) -> unt32 (
    vec-fold-first-to-last-from(vec, <unt32>0, unt32-add)
)
```

# concept: distinct origin of a value in your code
(The idea of "fresh, distinct type instances by code" seems to generally be called "path-dependent types")
```
# fn arena-empty<Origin>(origin : !origin<Origin>) -> !arena(Origin);
# origin some-name creates a new origin variable and a local unique type for the start offset of its scope
# An origin does not have a clone helper.
# At the end of the underlying origin of the annotated origin type, deallocate the memory of the value with that origin

# it is impossible to annotate a function that tries to return a value whose memory was origind to inside that function
fn some-arena-one() -> !arena(?? unt32) (
    origin arena-origin
  	let arena (arena-empty<unt32>(arena-origin))
  	let { arena slot(_) } (arena-push-back(arena, <unt32>123))
  	arena
)
# it is however possible to use a temporary value within a scope
fn use-arena() -> unt32 (
    origin arena-origin
  	let arena (arena-empty<unt32>(arena-origin))
  	let { vec(arena) slot(first-slot) } (arena-push-back(arena, <unt32>123))
  	let first (arena-element(arena, first-slot)) # <unt32>123
  	let slice-after-first (slice-empty<arena-origin>())
  	let { vec(arena) slice(slice-after-first) } (arena-push-back-into-slice(arena, <unt32>456, slice-after-first))
  	let { vec(arena) slice(slice-after-first) } (arena-push-back-into-slice(arena, <unt32>789, slice-after-first))
  	first
)
# different branches, different scopes
fn use-opt(<opt<unt32>>opt) -> {} (
    # this won't compile as their origins come from different branches
    let vec (
        match(opt)
        Absent (origin vec-origin arena-empty(vec-origin))
        Present(number) (origin vec-origin arena-one(vec-origin, number))
    )
    # this will compile:
    origin vec-origin
    let vec (
        match(opt)
        Absent (arena-empty(vec-origin))
        Present(number) (arena-one(vec-origin, number))
    )
    {}
)
# possible alternative way to create an immutable slice
fn slice(<slot<Origin>>start, <slot<Origin>>end) -> slice<Origin> (
    # ...
)

# recursive structure. One cool thing is that !expression will turn every slot
# into an exclusive slot
choice expression<Expressions-origin Patterns-origin> (
    Int<Expressions-origin Patterns-origin>(int64)
    String<Expressions-origin Patterns-origin>(string)
    Vec<Patterns-origin>(slice<Expressions-origin>)
    Call<Patterns-origin>({
        function(slot<Expressions-origin>)
        argument0(slot<Expressions-origin>)
        argument1-up(slice<Expressions-origin>)
    })
    Lambda({
        parameter0(slot<Patterns-origin>)
        parameter1-up(slice<Patterns-origin>)
        result(slot<Expressions-origin>)
    })
)
```

Only bulk-de-allocating owned values once they go out of scope (aka arena-allocation)
would be a (safe but bad) memory leak for main-scoped values like the application state.

A better solution: Allowing all values to de-allocate some part of themselves
and, more importantly, freeing owned values in branches they are not used in!
rust does that automatically which is awesome
```
type state<expressions-origin> ({
    // ...patterns, strings etc
    expressions(vec<expressions-origin expression<expressions-origin>>)
    root-expression(expression<expressions-origin>)
})
fn initial-state(<!origin(Expressions-origin)>expressions-origin) -> state<Expressions-origin> (
    {
        expressions(vec-empty(expressions-origin))
        root-expression(todo("do parsing"))
    }
)
fn state-to-interface(
    <!origin<Interfaces-origin>>interfaces-origin,
    <state<Expressions-origin>>state
) -> arena<Interfaces-origin state<expressions-origin>> (
    let { index(_) arena(interfaces) } arena-one(interfaces-origin, Console-log<never>"hello"))
    interfaces
)
```

# known limitations
nested sub-slices/slots cannot be easily de-allocated in bulk (so without walking the whole syntax tree and removing slices and slots one by one.
Preferably, expressions etc. would be stored in different slices per module, each with their own origin for bulk de-allocation and new-allocation. The closest solution would be to store a !slice() into all expressions etc for each module.
However, this means that slots and slices within the AST are non-owning

# TODO
- make sure that types with ! cannot be passed as type variables without !
- figure out how to make "immutable references" of !values possible and ergonomic.
  possibly disallow any further mutable referencing after any immutable reference is created - check if that's too limiting.
- allow last match case result to spill, allow lambda fn result to spill, change record syntax to something like rec x() y() z() and allow the last field value to spill, do not allow last call argument to spill (including in dot call)
- make drop explicit. That would make it very clear how things are deleted and at what cost. (and most importantly, when e.g. slices or slots are dropped they need a reference to the backing vec anyway)
- separate vec into vec (slot map, reusing memory) and arena (temporary arena, bumping + bulk de-allocation: just a plain vec whose elements simply get ignored after "remove" should be enough, alternatively ExternalStableVec https://github.com/LukasKalbertodt/stable-vec).
  choosing vec for deletion-heavy state will be a memory leak. This should be documented thoroughly!
  Potential quality of life: separate slot/slice types and
    !arena-slot/slice should not require explicit drop while !vec-slot/slice should
- consider adding vec-counting which can re-issue an index that is already in use, effectively Slab<Rc<_>> but flattened in memory and not supporting weak references. This would enable graph structures, child-parent relations etc. `fn vec-counting-add-slot-use(..., slot(Origin)) -> !{ vec() slot(!slot<Origin>) }` and `fn vec-counting-remove-slot-use(..., slot(Origin))`
- add index-to-unt, slice-start-index, slice-end-index, slice-indexes-fold etc
- remove the idea of an "immutable view over !" and just ask functions taking single-reference types to also return single-reference types. then ! can be removed and is basically baked into types. Open question: This means type variables are considered single-reference and require a clone to be passed if necessary
- remove need for comma in parameter and argument list, potentially drop the outer parens
