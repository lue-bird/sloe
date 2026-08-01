Small, fast programming language where indexes are valid and values can't be shared.

The goal is representing tree-like data structures without segmented, non-pre-allocatable memory or plain integer indexes (along with the need to handle failure and generations).
Sloe offers a safe, infallible way to refer to elements and slices stored in consecutive memory.

[skip to examples](#examples)

Note that while as a side effect this avoids any bounds checks,
bounds-checking in general is not slow.

Install with

```bash
cargo install --git https://github.com/lue-bird/sloe sloe
```

# concept: each value must be used used exctly once
Matching a value? Consumes it. Passing a value as an argument? Consumes it.
Even variables holding plain numbers for example have to be explicitly duplicated to use them in multiple places.

This allows
- values know when they aren't used anymore at compile time. Their memory is always explicitly reclaimed without garbage collection or similar
- values can be mutated internally without mutation being detectable
- representing things that should only be consumed once, like thread join handles
- representing things that should be cleaned up in a specific way, like memory that should be freed from a specific origin
- guaranteeing non-overlapping pointed memory regions can enable more optimizations, e.g. through [llvm's `noalias`](https://llvm.org/docs/LangRef.html#parameter-attributes) (though I think currently the languages sloe compiles to [don't entirely exploit this fact](https://github.com/rust-lang/rust/issues/16515))

This can feel annoying and clunky. Think e.g. `Span-length` which takes a span and gives back its size and the given span.
Not ony is it clunky, it is also often conceptually less constrained than taking an immutable view (like &Vec in rust) because `Buf-allocated-length` could return a changed `Buf` (this can also be an advantage but it usually isn't). If you wanted to track where a value changed, this makes things harder.

The big advantage of this rule is how easy it is to understand and how much simpler and faster it is to statically analyze compared to lifetimes or similar.

Further reading if interested: "linear types", [article "must move types"](https://smallcultfollowing.com/babysteps/blog/2023/03/16/must-move-types/), [nice short explainer in the austral language docs](https://austral-lang.org/linear-types), ["mutable value semantics"](https://www.jot.fm/issues/issue_2022_02/article2.pdf).
Sloe once allowed values to be ignored ("leaked"/forgotten) making them "affine types", like rust owned values. This was changed as it was too easy to for example accidentally forget to handle a value in one query case but not the others. Better be safe and explicit.

# concept: consecutive memory collection `Buf`
A collection which can mark some ranges within itself as vacant without moving existing elemnts around.
This can be used to "return" memory which has become outdated or useless, for example with `Buf-remove`, `Buf-slot-rid` and `Buf-span-rid`.
Note that this functionality is entirely optional and you can at no cost just use it for temporary builders etc. which never vacate anything before they are scrapped.

Further reading if interested: This concept is often called slot map, reusing memory.
In rust, a prominent example is [slab](https://docs.rs/crate/slab/latest).
[Comparison of various kinds of similar rust collections](https://donsz.nl/blog/arenas/).
There are even fast general purpose allocators based on this concept, for example [zig's SmpAllocator](https://codeberg.org/ziglang/zig/src/commit/a85cb728775375825afe4ebd62c60ae0b361d1e9/lib/std/heap/SmpAllocator.zig) or [the rust crate "smmalloc"](https://crates.io/crates/smmalloc)

# concept: collections do not handle their elements
Similar to allocators, you cannot access, alter or iterate their contained values.
Collections are seen as storage into which you can add elements, build slices etc.
Whenever you do so, you'll get `(Unset-)slot`s and `(Unset-)span`s that assert your right to access and alter the referenced elements as well as your responsibility to announce their release at some point.

The alternative to this would be to make tiny allocations for every slot and small span and to allow recursive types.
This is not uncommon in languages like rust.
However, sloe's goal is to do better here and to not bind storage to ownership over its elements. Instead, store a big array of each kind and point into it.

# concept: distinct origin of a collection in your code
Every created collection has a correlated origin.
A value whose type contains an origin can't escape the scope of it's origin.
This is checked at compile-time for the expression following origin creation but you'll likely realize it before then:
```sloe
fn Some-buf . : Buf ??origin cannot even be annotated??, u32 =
    origin buf-origin
    ? Buf-empty<u32> buf-origin [buf]
    ? Buf-add .buf buf .new 123 u32 [.buf buf .slot slot]
    ...
    buf

# compiles
fn Add-some-values buf Buf _origin, u32 : Buf _origin, u32 =
    ? Buf-add .buf buf .new 123 u32 [.buf buf .slot slot]
    ...
    buf
```

Further reading if interested: The insight "marking origin-specific types specific to code unique paths" has been described similarly in ["The Unreasonable Effectiveness of Naming Integers"](https://ziglang.org/devlog/2024/#2024-11-04).
With the small difference that in sloe's case the unique origin types
only exist at compile-time and can thus mark spans, slots, unset spans, unset slots, bufs etc. generically. Additionally it is _checked_ that actually only one collection and its indexes are marked that way.

The idea of "fresh, distinct type instances by code" seems to generally be called "path-dependent types". In rust I know of 2 crates that successfully implement this: https://docs.rs/compact_arena/0.5.0/compact_arena/index.html (safe, pragmatic, simple but bare-bones) and https://docs.rs/indexing/0.4.1/indexing/ (safe, cumbersome, complicated).
The same idea but with runtime checking instead of compile-time checking can quite easily be implemented by storing an ID in each collection and the same id in each contained slot, and incrementing a global variable (or similar) for the next available ID: https://github.com/thomcc/handy/blob/master/src/lib.rs#L111-L126
(apart from security this is hardly ever worth it for regular users, considering it is also slower).

I find it interesting that "storage" and "ownership over said storage" are decoupled.
I've heard this being called ["call-site dependency injection"](https://matklad.github.io/2020/12/28/csdi.html) which also perfectly applies to the idea of passing allocator, interner, concurrency runtime etc. around.
I really like this idea but understand that it cannot be implemented in e.g. rust which needs to store its allocator in it's value body to guarantee its content isn't splattered across different inaccessible allocator memories (and to satisfy `Drop` and to keep most of the existing function interfaces as well as convenience). Sloe solves this dilemma by assigning this unique origin at the high cost of user convenience.
In my opinion this isn't quite a solved problem and if you have other ideas, I warmly encourage you to explore and share them.

# examples
## creating new origins, slots and spans
`origin some-name` creates a new variable of type `origin` and a unique local type.
Like every other sloe value, an origin type can only be used once, so only for one collection.
```sloe
# use a temporary collection contained within a scope
fn Use-buf . : u32 =
    origin buf-origin
  	? Buf-empty<u32> buf-origin [buf]
  	? Buf-add .buf buf .new 123 u32 [.buf buf .slot first-slot]
  	? Buf-remove .buf buf .slot first-slot [.buf buf .element first]
  	? Buf-add-array .buf buf .new ; 456 u32 ; 789 u32 [.buf buf .span after-first]
    ...
  	first # = 123 u32

# different branches, different scopes
fn Use-opt opt Opt u32 : ... =
    # this won't compile as their origins come from different branches
    ? (
        ? opt
        [|no .]
            origin buf-origin
            Buf-empty<u32> buf-origin
        [|yes number] (
            origin buf-origin
            ? Buf-one .origin buf-origin .element number [.buf buf .slot slot]
            ...
            buf
            )
        )
    [buf]
    # this will compile:
    origin buf-origin
    ? (
        :opt
        [|no .]
            Buf-empty<u32> buf-origin
        [|yes number] (
            ? Buf-one .origin buf-origin .element number [.buf buf .slot slot]
            ...
            buf
            )
        )
    [buf]
    ...

# tree structure. every slot and span exclusively belongs to that expression
ty Expression _expressions-origin, _patterns-origin, Chars-origin
    |int i32
    |string Opt Span Chars-origin
    |buf Opt Span _expressions-origin
    |call
        .function Slot _expressions-origin
        .arguments Span _expressions-origin
    |lambda
        .parameters Span _patterns-origin
        .result Slot _expressions-origin

ty State _expressions-origin
    # ...patterns, strings, positions etc
    .expressions Buf _expressions-origin, Expression _expressions-origin
    .root-expression Expression _expressions-origin

fn Initial-state
    .expressions-origin expressions-origin Origin _expressions-origin
    : State _expressions-origin =
    .expressions Buf-empty<Expression _expressions-origin, ...> expressions-origin
    .root-expression (..do parsing..)

fn State-to-interfaces-into
    .interfaces interfaces Buf _interfaces-origin, Interface State _expressions-origin
    .state state State _expressions-origin
    : Buf Interfaces-origin, _interface State _expressions-origin =
    ? (
        Buf-one
        .origin interfaces-origin
        .element |console-log<Interface State _expressions-origin> "hello"
        )
    [.slot slot .buf interfaces]
    ...
    interfaces
```

## pass in origins or collections from the outside

```sloe
fn Buf-empty<_element> origin Origin _origin : Buf _origin, _element
```
Used by most initializer functions which return new collections from nothing, e.g. for the initial persistent application state.
For most other functions, it's more common to pass in an existing collection that you want to edit.

# syntax
Syntax is secondary but I tried to make it coherent, practical and compact, avoiding parens and indentation when possible, especially for trailing syntax.
Sloe is a very explicit language, so any extra verbosity is not tolerable.
```sloe
# line comment

# number type, so for example
3.2 f32 # number types are p32, u32, i32, f32

# str
"hello"

# char
'a'

# most identifiers
some-variable-or-field-or-type-without-parameters-or-variant-name-2012

# constructor name
Some-type-with-parameters-or-function-name

# function call.
# Rarely functions may require appended comma-separated type arguments: <...>.
# Any function is of type `fn` and always requires an argument (which does not need to be parenthesized)
Some-function<type, arguments> Inner-call-as-the-argument inner-inner-argument

# record. if values are open-ended they need to be parenthesized.
# The last field value can end in a record without needing to be parenthesized
.first-field first-value .second-field second-value

# "empty record", like void/unit.
# commonly used for variants "without a value", empty state
# or as the result of functions like u32-rid
.

# ..spread a record into other fields
# Can be placed anywhere and multiple are allowed
.field-1st value-1st .. one-existing-record .. another .field-2nd value-2nd

# temporary array
; first-element ; second-element ; third-element

# local function of type fn.
# the pattern must add a type to all variables
# can **not** use variables from the outer scope.
[parameter-pattern] result

# pattern variable
# appending a type is only necessary and allowed in function parameters
some-variable some-type

# pattern match, checked for exhaustiveness. expressions must be parenthesized if they themselves end in a query.
# The last case result does not need to be parenthesized
? value [first-case-pattern] first-result [second-case-pattern] second-result

# introduce a new origin. The given name can be used as a variable and type
origin new-origin-name expression-that uses new-origin-name

# project function declaration.
# For type variables in the result that aren't used in the input,
# functions require appended space-separated type parameters: <...>
fn Function-name<_potential, _type-arguments, _only-used-in-the-result>
    parameter-pattern-with-types
    : result-type =
    # optional documentation
    # comment
    result-expression

# type name without arguments. lowercase
u32

# type with multiple arguments. Uppercase name.
# Arguments before the last must be parenthesized if they end in a type with arguments
Span origin
My-function-type env, input, output

# project type which is an alias for an existing type.
# Here a "choice type" that can come in different shapes ("variants")
# which each have a unique name and one associated value.
ty point .x i32 .y i32

# project type with parameters
ty Type-name _potential, _type-Parameters
    |first-option .
    |second-option Buf _potential, u32
    |third-option Type-name-alias _potential, _type-Arguments

# creating a variant. Note that the type could refer to a type alias
# or a choice type directly <|... ...>
|some-variant<its-choice-type> its value

# variant pattern
|some-variant its value
```

> As a user of sloe you can stop reading here. The rest is mostly for developers and those interested in language design

# known limitations & design weaknesses
What I'm unhappy with in the current design.
Writing these down has already helped a lot in coming up with fixes (e.g. `Unset-slot`, `Buf-span-add-own-span` etc. did not exist at one point but were created in response to a now deleted list items).
And even if I'm unable to fix them, other people/teams might (in other projects)!

- it seems quite natural to represent a span of structs as e.g. `.field-names Span Field-names .field-values Span Values`. This pattern can avoid "type parameter spam" for any record (plus it is more memory efficient).
  The biggest missing convenience to make this attractive might be helpers to fold over many spans simultaneously.
  Honestly, the current "fold over one span and step through the rest with `Span-start`"
  is annoying. I do not particularly like it as there is always "overspill" that needs to be handled.
  Additionally, this wastes memory for the duplicated memory and wastes computation for unnecessarily handling 
  
  Zig "fixes" this by both
    - introducing special syntax and crashing at runtime if lengths differ
    - only storing the length in one of multiple slices and documenting the expected length for the other start pointers
  
  I think introducing `Span2 FirstOrigin, SecondOrigin` for 2 up to maybe 5 makes a bunch of sense. You'd be able to fold, access etc. them together and even split those up into separate spans whenever desired (but not join them back!).
  The sad thing is that this is positional and individual span slots then do not have an associated name.
  Also, how would this work with existing buf APIs? Something like `Buf2-opt-span-add`
- scattered sub-spans/slots in a persistent buf cannot be easily de-allocated/iterated in bulk (so without walking the whole tree and removing spans and slots one by one, aka pointer chasing).
  For example, preferably expressions etc. would be stored in different spans per module, each with their own origin for bulk de-allocation and new allocation.
  However, this would mean that slots and spans within the AST would not be owning.
  One quasi-solution would be storing `Buf<origin Buf-originless<...>>` and introducing `-originless` versions of slot/span/buf. These would need to be checked at runtime, with the branch with unequal origins being in the cold path.
  While this technically does solve the problem quite nicely, it's purely at runtime. Mistakes won't be caught early, runtime costs may add up, complexity increases, unnecessary error handling gets introduced.
  → More complex type systems could solve this (e.g. :hand-wave: add type `Origin-erase` that takes a function taking an origin and returning an element type, thus eliding a concrete origin from the buf type. Then you could call its wrapper to hide the origin and its unwrapper with a fresh origin whenever you need to access or modify the inards) but I'd like to stay simple
  → I need to investigate how other languages do this. E.g. [carbon's outer-self-other-field-place-referencing feature](https://chandlerc.blog/slides/2026-memory-safety-deep-3/#/51) may solve this (I'm not sure, but it's also not that simple).
  (For temporary bufs, a possible solution could be `Buf-slot-rid-without-vacating`, `Buf-span-rid-without-vacating` and `Buf-opt-span-rid-without-vacating` which would temporarily leak these slots and spans. This can be misused for persistent bufs but more importantly it does not solve the issue for persistent bufs)
- sometimes, you really own all the elements of a buf in one place (especially when the buf elements can be trivially copied).
  Splitting it into `opt span`+`Buf` is annoying and wastes a bit of space (length is carried twice and start is always 0).
  Due to "can't easily recombine spans" it is also really annoying to access any elements in the owning buf.
- by default, most passed arguments are quite fat on the stack (e.g. `Buf` is 6 usize-wide and you may pass a bunch of them).
  Pointers are much thinner. This can in some parts be optimized by the target language compiler
- currently syntax is not full-word-search friendly. Think `_construct argument` and `minus-dash-hyphen`
- the language is very sequential by design which disqualifies it from running fast on much of parallel computing e.g. GPUs, threads that share memory etc.
  Sloe is most likely not the right vehicle to explore this space,
  still it seems like a warning sign for a supposed "general-purpose language"
- number types, buftor/array types etc. are very underbaked in sloe.
  I need more real-world experience for their uses.
  Granted, sloe support for them is only realistic if rust (and zig) improve their support as well

# potential improvements in the future
- add field and variant rename and references
- add "add remaining query cases" code action
- suggest full parameter field patterns of existing project fns (just as rust does). This is super convenient, especially because stuff like `expressions Buf _expressions, Expression _expressions _patterns _types` doesn't exactly roll easily over one's keyboard
- add `Set _origin, _element` along with add something like `Map _origin, _key, _value` (or just `Map _origin, _element` where key is derived from element) which still gives out `Slot Origin`s for each entry but can be queried by key or similar. `Map-empty` will require providing an `.order (Fn .a _key .b _key, .a _key .b _key .order order) .dup (Fn _key, .a _key .b _key)` or similar.
  Alternatively, check if implementing in userland via e.g. index map, AVL or red-black tree backed by a regular `Buf` is fast enough
- consider adding `Buf-counting` and `slot` which can reference a slot that is already in use:
  ```Sloe
  fn Buf-counting-slot-dup
      .buf Buf-counting _origin, _element
      .slot Slot _origin
      :
      .buf Buf-counting _origin, _element
      .a Slot _origin
      .b Slot _origin
      =
  # what about spans?
  ```
  In theory, this would enable graph structures, child-parent relations, doubly-linked lists, inlined string storage (although that would need e.g. `Set-counting`) etc.
  Things I dislike with this design:
    - access via `Buf-counting-unset` which does not guarantee seems maybe too difficult (first un-occupy all known slots and even then there is no guarantee). `Buf-counting-update` should work nicely, especially for copiable types at the cost of: cannot access the buf at the same time and spooky action at a distance
    - _every_ element is reference-counted. A slot to a known single-reference element cannot be represented. This is not a biggie because I don't know if there is a use for this
    - maybe also -counting versions of map/set etc.
  
  The alternative is of course to do `Slot-weak` and generational indexes. However, this is un-usable for e.g. inlined string storage and also comes with overhead and even less guarantees.
  
  Open question of representation:
    - `Buf<{ count: u32/16, element: Element }>`:
      Finding vacant slots takes linear time. Generally fast.
      Takes the least space on the stack
    - `{ elements: Buf<Element>, counts: Buf<u32/16> }`:
      Finding vacant slots takes linear time. Generally fastest.
      A bit more error-prone than single buf
    - `{ elements: Buf<Element>, vacant: Buf<u32>, occoupied_counts: Buf<NonZeroU32/16> }`:
      Finding vacant slots takes constant time but doesn't feel deterministic.
      Generally fastest but vacating is more expensive.
      More error-prone than single or double-buf.
      Takes most space on the stack
    - `{ elements: Buf<Element>, counts: Buf<{ range: Range, count: u32/16 }>`:
      Tough to handle and error-prone.
      Inefficient for cases where slots are handled one by one (no spans exist).
      Efficient for things like inline storage where spans are clearly defined.
    - the above but with vacant ranges and occupied counts split

  I think I prefer not storing counts in ranges, as for example for string interning, you could store counted spans in separate collections:
  ```sloe
  chars # of type Buf _chars, char
  names # of type Buf-counting _names, Span _chars
  ..other bufs pointing into chars, e.g. for number literals..
  ```
  This is likely the better option anyway (even though it "hops twice")
  as it makes searching for the right span possible (and reasonably fast)
- introduce `ascii` (in rust backed by `std::ascii::Asci` which is currently experimental, in zig backed by `u7`), require char literals to be suffixed with a type, (optionally provide `ascii` as a choice type like [`std::ascii::Char`](https://doc.rust-lang.org/std/ascii/enum.Char.html)). Change `str` to `chars` and `ascii` to `asciis`. Preferably rust would support this directly, otherwise do transmutions or similar at some point. Also introduce `ascii-to-char`, `asciis-to-chars` and the inverse operations which return `opt`.
  remove `'c'` syntax in favor of `"c" char/ascii`
- combine scc stuff into the parser state to avoid walking the whole AST for info we could already have collected. Comes at the cost of a thicker ParseState, probably still worth
- (probably not that good of an idea) to the above effect, it could be nicer to add ultra-basic macro support, so e.g.
  `!u32 "3"` where `u32` is of type `_fn str, |success u32 |failure str` (instead of `3 u32`) which would evaluate the given function (which should return `|error str (?) |ok Value`).
  This would allow userland to create e.g. hex parsing functions, arabic number systems, string raw bytes stuff etc.
- (probably not that good of an idea) consider not counting function calls as using up a function variable.
  The disadvantage is that "overplacing" a variable step-wise doesn't work anymore
  if not wrapped somehow. Maybe a small price to pay
  And where no `Length` field can be instantiated
- consider replacing kebab-case with camelCase/PascalCase.
  while I do much prefer the typing experience of kebab-case,
  camelCase is shorter (!!), think
  `BufCharOptSpanAddStr` compared to
  `Buf-char-opt-span-add-str` (5 chars less, 20%!)
  and potentially more readable (?) due to clearer distinction to _ and 
  (this won't matter as much if call and construct syntax does not involve _).
  Take a bigger example, convert the case and see how it feels
- (not fully sure) Add explicit field punning syntax:
  Add pattern syntax `_` (untyped) / `_ value-type` (typed) (and maybe expression syntax `_`) where `_` behaves like a variable with the name of the parent.
  So e.g. `.field (_ value-type)` would introduce a variable named `field`.
  and pattern `|variant _` would introduce a variable named `variant`.
  Likewise, `linked-list-cons .nodes _ .linked-list numbers .new 3 u32`
  would work if a variable named `nodes` exists.
  If no parent name exists, an error is thrown.
  The only goal here is making record patterns more convenient to work with (similar to swifts named parameters). The biggest worry I have is name clashes. Things like rename might also become a little more complex.
  I would normally not consider this as a feature, but since sloe is so painfully
  explicit, I feel users deserve some sugar for their effort.
- add field spread syntax for types where overlapping field names is okay as long as their value types are equal
- add variant spread syntax `||existing-choice-type |other-variants-before-and-or-after` (only in types) analogue to the field spread syntax
- when checking, avoid shortcutting early when possible, still traversing sub-elements even when a clear error has been found
- add typescript backend or similar web support
- verify that origin creation is correct for all kinds of recursion! e.g. this one seems on the edge of correct:
  _different bufs have the same origin_ but their slots can't intermix.
  ```sloe
  fn Recurse
      .consume-origin consume-origin _consume-origin
      .result-origin result-origin _result-origin
      : Buf _result-origin, u32 =
      origin local-origin
      ? Buf-empty<u32> consume-origin [temporary]
      ? Recurse local-origin result-origin [result]
      ? Buf-add .buf temporary .new 1 u32 [.slot slot .buf temporary]
      ...
      result
  ```
  If we find a problem, creating a new `origin` should be disallowed in (mutually) recursive calls.
  This is a bit restrictive but alright I believe.
  If feeling motived, look into proof languages and make sure this is rock solid
- improve memory efficiency of string operations (currently buf of char).
  This is probably inefficient because:
    - more work on program boundaries. E.g. instead of validating data, then reusing the bytes, we need to re-allocate them and then finally un-convert them into utf-8 anyway
    - most bytes are 3/4th 0s because ascii is so common. wasted space is bad for the cache and memory usage
  If these somehow turn out to be nonconcerns (e.g. through array-of-union(enum) optimizations) that would be cool as well since `Buf _, char` is a much nicer API to work with
- zig-only: store an allocator within an origin (but! what about unset_slice? That one should probably store an allocator, too, and re-allocate if the buf origin allocator reference differs. I think this can be slightly unintuitive for sloe users but should in practice be okay). This achieves that origins created from within sloe code are arena-allocated and origins from user code are (usually) not, choosing e.g. MemoryPool.Aligned (does that actually work even?)
- (once there is an easy way to check if a pointer is aligned in rust) change `cast_or_rid_and_allocate` to recover alignment differences if the address happens to align
- (once allocator API is stabilized) allocate all collections with an origin that was declared in sloe using a locally-passed `impl Allocator<>`
- I think in theory there should be all the bits and pieces present to allow for struct-of-arrays and arrays-of-variant-values (made up name). E.g. internally compiling
    - `Buf Origin, .a A .b B` to `A·B<Buf<A>, Buf<B>>`
    - for `Buf Origin, |a A |b B` to either 
        - `Tag·ValueIndex·A·B<Buf<A_or_B_Tag>, Buf<u32>, Buf<A>, Buf<B>>` (which also has ~2 hops but makes sense when sizes of A and B are different enough)
        - `A·B<Buf<A>, Buf<B>>` (which requires the index to hold both the tag and the value index, aka 64 bit instead of 32, which somewhat defeats the point of reducing padding of the variant when values get bigger. Potentially there could however be struct-of-arrays for individual variant values making this worth it: https://github.com/dist1ll/osmium & https://alic.dev/blog/dense-enums)
        - `A·B<Buf<A>, Map<u32, B>>` (which is inefficient, and wasteful if `B` is common, and also doesn't scale with more than 2 variants)
- look into `soa_derive` for rust, maybe this already does most of the useful work
- (very out of scope but thinking never hurts) imagine what a logic programming language with this concept would look like. I imagine it wouldn't look much different (!) though with some different tradeoffs (e.g. more complex stdlib and compiler output, potentially a different typing and exhaustivess system)

# rejected ideas
As a hobby language that deliberately cannot by itself interface with the operating system, C etc. we can afford to skip many complex features. First some smaller-scale rejected ideas

- allow expressions whose type is known (basically anything except inputs to queries) to omit extra type info (namely number, variant<> and project-fn<>). I'm a little torn because this makes construction inconsistent and increases the distance between the known type and expression. On the other hand this is already the case for patterns (deliberately so) but has a much higher convenience gain there
- add special syntax `fn-once` that automatically assembles the environment from the used local variables.
  Rejected in favor of more explicit construction with contextual names and potentially multiple fns.
  More info in "not coherently formulated thoughts"
- add tuples: (* a * b * c). I dislike them conceptually but operations like `U32-add` or `U32-dup` are nicer with them. The field names `.a .b ` are just noise. Adding tuples might make for a nicer user interface when calling from rust
- consider allowing `origin name` at the project scope. This allows reducing the number of type parameters flying around in things like `Expression _expressions, _patterns, _types, _source, _cases, ...` if desired.
It also makes initial_state much easier to call from the rust side (though we need to be careful how...).
  Rejected because this makes it more or less impossible to run multiple sloe instances from a single rust program
  Issue is that in general single-return-continuation is rare in sloe
- requiring all (!) generic type parameters to be passed to calls
  I feel like this is more "natural", easier to type-check but way more verbose / redundant.
  And especially because having _many_ origin type variables is common, this sadly won't fly
- adding function call syntax sugar similar to piping.
  While this is bloody wonderful (succinct, intuitive-ish, great for builders), it doesn't quite have much of a purpose which pattern matching doesn't fill well already. But more importantly it is quite limiting (requires positional arguments, requires them in the right order, doesn't apply to variants and similar). It also introduces "yet another way of writing the same code" which is dislike
- (rejected, but interesting in theory) making `Buf` etc store multiple kinds of data (heterogenous) and letting them give out `Slot origin, data-type` and `Span origin, data-type`. This means that usually only one `origin` needs to be passed to things like `expression` and slots/spans actually tell you what data they point to. Similarly, only one buf needs to be passed around.
  This makes the porpose of `Buf` being allocator-ish spaces rather that collections to query and edit more clear and makes passing them around to operations very simple, e.g. `expression-end .expression Expression _origin .data Buf _origin, ... : .buf Buf _origin ... .end text-position`.
  This would also in theory enable a crazy representation of tagged unions as:
  ```sloe
  ty Expression-slot _origin
      |int Slot _origin, i32
      |plus Slot _origin, .left Expression-slot _origin .right Expression-slot _origin
      ...
  ```
  This also means slices etc need to be stored separately in the origin buf.
  The issue currently is that it feels hard to optimally construct/query such a heterogenous structure.
  Its structure _must_ be created at compile-time. Dynamically this doesn't fly: `Buf origin = { bucket: Map<for type_byte_size: { key: type_byte_size, value: Buf<type_byte_size> }> }`.
  However, really providing this in sloe would require sloe to add _some_ kind of "type variable must be record" constraint:
  ```sloe
  origin buf-origin
  ? Buf-empty<.expression expression .pattern pattern buf-origin> buf-origin [buf]
  ? Buf-add .buf buf .new some-expression [.buf buf slot some-expression-slot]
  ? Buf-add .buf buf .new some-pattern  [.buf buf .slot some-pattern-slot]
  ...
  ```
  This is probably doable in zig but hardly in rust without significant macro magic. Any ideas welcome!
- allowing `.. (|variant ...)` with a single variant and untyped variant expressions. No, should consistently use single-field record
- field and variants are changed so field names and variant names are uppercase
  and `.` is spread (same for `|`), e.g.
  ```sloe
  ty event
      |Counter-clicked
      |Mouse-moved .X u32 .Y u32
  ```
  The benefit is that the question above is answered (single field = single variant).
  Overall this is "more correct" than the current solution.
  Rejected because this is harder to type (and would require a change of type variable syntax)
- (rejection not final for all eternity. If you have a good use case, I'll support it) allow field and variant names to start with digit, upper-case and -, like `fn Char-dup char char : .0 char .1 char`.
  One nice thing is that this matches what most language use as field names for tuples.
  This is also a little bit confusing but you don't have to use it.
  Use cases are e.g. `ty bit |0 . |1 .`, `type board-pin |0 . |1 . |3 . |10 .` and nicer array records.
  Not included currently for consistency and simplicity.

## why no `&mut`/`inout`
While seemingly convenient and magnitudes better than regular mutable pointers,
- it's less obvious than passing values through
- there's no easy way to change the name of a resulting value that represents something different now
- there's no way to "reconstruct" a different out value. Especially for non-trivial edits the &mut approach can get messy or it's straight up impossible and parts will need to get cloned unnecessarily
- there's no way to change the type (e.g. from `Opt Span` to `Span`)
- there's no there's two ways to specify most conversions, with usually no clear method of converting one to the other
- it's surprisingly common that one path consumes an argument, the other path keeps it in tact (e.g. when searching a tree with intermediate information. Either we find something, consuming the context or we come up empty-handed with the original context, like `fn .context context ... : |exit found |go-on context` where found contains some parts of the context). This isn't modelled well with `&mut`
- `&mut` means the resulting changed collection is not returned, making use as the input to another function impossible. This almost necessarily results in the classic procedural-style statement form as opposed to the functional-style expression form. Minor gripe: especially in languages that don't allow local scopes with local returns (far, far too many) this basically makes it impossible to locally introduce a value, change it and implant it somewhere; instead you have to move the variable up to the top level.
- returning `.` (like returning `Unit` in gleam) feels super awkward to my brain. Most often, languages then automatically return void/... in the absence of a return and introduce all kinds of constructs like re-assignable variables, additional constructs for looping and branching that all can only return void/... . To my brain, this just confuses matters; it loves simple to follow flow of state!
- &mut usually comes with the need to check for non-overlapping references to the same parts of data. This isn't possible with owned data passing in the first place
- &mut usually necessitates the need for offering the same APIs in two shapes, e.g. `make_uppercase(&mut self)` vs `to_uppercase(self)->Self` on rust's char/str types, `take()`/`take_mut()`, `std::mem::swap` etc. This to me just feels wrong

rusts immutable references `&` have some similar trade-offs but seem kind of unavoidable at least for languages like rust.

## why no closures that capture environment variables automatically like in rust
- its type cannot be specified. as such, it cannot generally be stored as part of a type
- no clear unified interface. There could be multiple functions, there could be an output that additionally returns the captured values (allowing it to be called again), there could be an output that only sometimes returns the captured variables, etc.
- I personally never had a need for this. Usually you can just make the environment a type variable and you're golden

I'm strangely really convinced that this is the obvious, correct design decision (for most programming languages at that!).
Note that the current design does not natively have a `dyn Fn`; it needs to be manually emulated via an explicit `|` choice type.

## why no traits / type classes / (duck) (static) dispatch 
- traits introduce a crazy amount of complexity
- If really necessary, traits can be represented using arguments. I have yet to hit any complexities with this.
- attaching a set of functions to one "subject" seems super strange to me. Operations usually take different objects and create something new
- traits create a "one-fits-all" interface. Thinking to e.g. rusts `clone(&)`, `drop(&mut)` or `to_string(&)`, they seem super sensible in concept but fall short when trying to clone into a different allocator, trying to use a different string representation, or trying to mutate some backing storage on drop.
  These restrictions can sometimes kind of be circumvented in annoying ways.
  It all just seems so arbitrary for no apparent reason.
- traits are usually touted as a sensible solution for operator overloading. sloe does not have operators
- traits push languages in the direction of nominal record and choice types. This isn't wrong per se but to me they usually start to feel clunky to use and I then use them less often then would be helpful (e.g. for long parameter lists or multiple outputs)

Because traits cover a vast theoretical area of use, they tend to be used a bunch. I've never found them particularily pleasant to use. Libraries often only expose some functionality through these, without proper documentation. Incidentally, I've also found editor tooling to be lacking in these areas, not knowing if you want to look at the general or specific function.

## why no (mathematical) operators
- operators introduce a good amount of complexity: infix (and prefix) notation, associativity, precedence, most likely a way to overload based on context
- edge-case behavior (e.g. saturating vs overflowing vs checked vs carry vs ...) should be easier to control
- in general, operators are concise but as a result quite ambiguous. For example, changing a boolean to an integer may silently not generate a compiler error when `!` is binary not, or when changing a list to a string with `++`
- while numbers, bool and bit operations are not that uncommon, there are features that would deserve these symbols more, even in typical imparative languages (think `return`, `switch { case }`, `structure`, `import`, `public`, `static`, `void`, `null`, ...)
- allowing infix `-` and prefix `-` leads can lead to very confusing situations like `call-1` but more importantly using `-` as an operator pretty much prevents languages from using the superior (easier to type) kebab-style for identifiers

Somehow despite it's issues (math syntax kind of sucks, even the tiny subset), operators are one of the most prevalent features in programming languages, even hobby and experimental ones (0th class citizen). I do not quite understand this (well I guess not adding operators adds to the weirdness budget) .

## why no single-field access
a.k.a `record.field`. Quick and easy answer: Because this makes it embarassingly easy to forget handling a field (now or in the future). I've identified this as the second most common source of bugs in my own code. And in sloe, not handling a field could mean leaking some memory, so it would be even worse potentially!

## why no positional function arguments
- with positional arguments it isn't really possible to make the last argument open ended (at least with keeping the current syntax)
- it's tough (usually) to annotate a function whose arguments and argument types are unknown. E.g. what would `fn-dup`'s type be?
- positional arguments (usually) means no passing in bulk
  ```sloe
  fn U32-square-clamp natural u32 : u32 =
      U32-add-clamp U32-dup natural
  ```

"Positionality" in general is pretty much absent in sloe. E.g. positional arguments are super convenient, so they tend to be used for everything, even arguments that would benefit from a clear description.
Sloe had positional arguments once, largely because the rust-sloe interface is simpler in rust with positional arguments.

## unnecessary features in sloe
Features I've added which are fully replacible by other existing features.
If you're looking to learn from sloe, maybe do not learn from these:

- record spread. It provides an alternative syntax sugar for something that could already be expressed. I originally introduced it to make builders like string builers less jarring
  but I'm not so sure this worked.
  I'm on the fence; if you have complaints I'll remove this feature

- nested pattern matching.
  It's existence makes compilation, exhaustiveness-checking, error messages and flow-typing-like matching (e.g. matching |a in <|a|b|c> leaving |b|c) harder.
  It also creates a "two modes of matching" problem: You e.g. can't match on numbers, chars, strings, span start and lengths etc. And so you sometimes need an extra step, leading to nested matches anyway (does not feel consistent).
  It also "takes control from the user int othe magic hands of the compiler" and thus it may run checks etc. in a different order than you have.
  I originally introduced it to make e.g. matching on multiple `Opt`s easier.
  It helps keep context clear and visible like "if the left sub is empty and the right sub is a branch with an empty left side, do this".
  Honestly I should not have been so hasty to add this feature

- stack-allocated array syntax. It provides an alternative syntax sugar for something that could already be expressed as repeated queried function calls.
  I'm convinced that a feature like this would be very asked for if it didn't exist.
  Adding a bulk of elements to a buf seems very useful on first sight because
    - all kinds of examples and tests start with manually adding elements. Doing this one by one seems like cringe busywork.
    - building uis or any kind of trees programmatically,
      you more than often end up needing to specify sub-nodes of a parent.
      Adding this bulk of nodes as an array is only natural and avoids so much noise.
  
  Well, what are the alternatives, then?
    - simply provide `Buf-opt-span-add2/3/4/5/6/7/8` etc. While it really doesn't feel good, it's not very far from solutions of production languages, see e.g. java's `list.of2/3/4/...`. (Though adding or removing elements means adjusting the number which is annoying, especially for the argument field names)
    - provide and suggest better primitives and helpers. For example, instead of providing a list of modifiers, it may just make sense to e.g. provide a record of options or use builder-style helpers for the individual properties
    - introduce syntactical diabetis or macro-esque bullshit for repeated function calls
      ```sloe
      ...
      _@0 stack-cons .. example-stack .new 39 u32
      & _@0 ..@ .new 3 u32
      & _@0 ..@ .new 6 u32
      & _@0 ..@ .new 9 u32
      ```
      (the above is obviously insane in a bad way, but there may be a middle-ground)
  
  So yeah, these aren't amazing either.


# general quetions you might have

## does sloe fill any niche well enough to be worth it?
I'd say domains where languages like safe rust, C#, swift, go, nim stand today:
  - not extensive enough to have a place in bare systems programming,
    but comfortably sitting on top of a somewhat thin platform layer.
  - not as easy to use as scripting languages like python, gleam, lua, elm, prolog, etc.
  - mainly used for applications or similar where maintainability and being easy to reason about is important

Don't be afraid to program in a language sloe compiles to for tasks sloe feels annoying to use for.
E.g. I imagine writing a recursive file watcher in sloe is not fun, so just "outsource" it :)

## why put work into transpiling to existing languages
The best user experience interfacing with sloe code from existing system-level languages
is directly generating code in that language. Just sharing type names, structs, tagged unions, function signatures etc without any work by you is tasty enough.
And if you end up outgrowing sloe, you have all the code right there (that's the hope anyway but output readability is likely wose than as if it was hand-written).
Being easy to transpile is an explicit goal of sloe, enabled by its very limited set of features.

## why write the compiler and tooling in rust?
It did that before and it does it's job.
I imagine the current style leaves some performance on the table but I'd be surprised if it was too slow for its only potential user, the human reading this (<3). 

# dev setup
to re-compile
```bash
cargo install --offline --debug --path . sloe
```

# TODO

- (not fully sure) add `Buf-opt-unset-span-add-length-positive`, `Buf-opt-unset-span-add-length`, `Buf-unset-span-add-length`, `Buf-unset-span-add-own-opt-span`

- (not fully sure) add `Buf-opt-span-add-repeat`, `Buf-span-add-repeat`, `Buf-opt-span-add-repeat-for-length-positive`, maybe even unfold

- add more math and maybe bit operations

- for simplicity, change `Function<..., ..., ...>` to `Function<...><...><...>` at call and project fn sites

- find a symbol to replace the `origin` keyword. Maybe ^ as a visual "place anchor"

- consider changing `|variant-name<type> value` to `|<type>variant-name value`. This allows better autocomplete
  Example: before
  ```sloe
  |text-dynamic<Html _htmls, _modifiers, _chars> Opt-yes text-span
  ```
  after
  ```sloe
  |<Html _htmls, _modifiers, _chars>text-dynamic Opt-yes text-span
  ```

- strongly consider replacing `<>` by `{}` because it is more easily recognized as parens

- implement conversion to zig. current annoyances (non-blockers, though):
    - zig plans to add an `infer` syntax to replace the current `anytype`. This will (I think) enable us to not store any information about checked function call type variable replacements
    - zig actually doesn't have the concept of anonymous structs and union(enum)s anymore. This can be worked around but I want to ask others for ideas that are more ergonomic.
      Let it be said that I'm legit sad that zig removed support like most other languages.
    - pattern matching. Probably easiest to start with
      ```zig
      if (some_magic(case0_pattern, value)) |case0_value| ...
      else if (some_magic(case1_pattern, value)) |case1_value| ...
      else unreachable
      ```
      where `some_magic(pattern, value)` is some expression like
      ```zig
      block_012000120012: {
          const @"%matched_01022013:023130" = ..value..;
          const @"intermediate_03020:2340" = switch @"%matched_01022013:023130".field0 {
              .variant => |@"intermediate_03020:2340"| @"intermediate_03020:2340",
              else => break :block_012000120012 null
          };
          const @"intermediate_03020:2345" = switch @"%matched_01022013:023130".field0 {
              .variant => |@"intermediate_03020:2345"| @"intermediate_03020:2345",
              else => break :block_012000120012 null
          }
          break :block_012000120012 .{
              .pattern_variable0 = @"%intermediate_03020:2340",
              .pattern_variable1 = @"%intermediate_03020:2345"
          };
      }
      ```
      (basically nested switches on the original value as a variable and temporaries, both field-accessed if necessary. Finally returning all pattern variables in an anonymous struct)
      and then consider switching to manually generated decision-tree-like code with nested switches if the former doesn't optimize well (it kinda should, though)

- (hmm...) think of a way to "split an origin":
    - creating initial state in sloe code, without needing to pass
      an unknown amount of origins in from the outside.
    - type aliases may only need to take a single origin type parameter
      for spans/slots with connected lifetimes, e.g.
      ```sloe
      ty Expression _origin
          |int i32
          |string (Opt Span .str _origin)
          |buf (Opt Span .expression _origin)
          |call
              .function (Slot .expression _origin)
              .arguments (Span .expression _origin)
          |lambda
              .parameters (Span .pattern _origin)
              .result (Slot .expression _origin)
      ```
      I think baking origin deriving syntax into the language is the easiest solution:
      ```sloe
      origin .derived-origin-0 .derived-origin-1 original-origin
      # derived-origin-0 is of type _origin (.derived-origin-0 original-origin)
      # derived-origin-1 is of type _origin (.derived-origin-1 original-origin)
      # the original-origin variable is consumed
      result-expression
      ```
      Syntax to be decided.
      
      In theory, the same effect could be achieved without syntax additions:
      ```sloe
      fn Origin-dup origin Origin _local : .a Origin (.a _local) .b Origin (.b _local)
      ty Str-origin _origin .a _origin
      ty Expression-origin _origin .b .a _origin
      ty Pattern-origin _origin .b .b _origin
      ```
      However, notice that annotating an origin like that is very undescriptive.
      The problem is that these `*-origin` type aliases are very brittle and could be applied to any origin, even one which does not have this specific derived origin.

- fix bugs and TODOs


# not coherently formulated thoughts
In rust, collections tend to own their element data, so safely keeping references reaching inside is tough.
Alternatively, we could reach for `Range<usize>` and `usize` but we've lost ties to the origin structure and rust does not (yet?) have a mechanism for temporarily assuming actual ownership over some part of a parent structure.
This relationship is flipped on it's head in sloe: All elements of collections are divided into slots and spans which are owned by the code that parked values there in the first place.

Honestly this idea seems "obviously" useful and it's surprising I can't find other languages that lean into it (there is rust which at least enables it in userland).
I assume one reason is that linear types are required in some part to avoid leaks all over the place.

One way this helps is that nested collections aren't segmented: what is usually `Buf<Box<str>>` aka n separate memory pieces can be e.g. `Buf ... Span str-origin` + `_str str-origin`
(in rust there are I think crates like oroborus for this)

## on shadowing
since each variable can be used at most once, most introduced names that would traditionally be considered "shadowed" are aready out of scope in sloe. When their scopes actually overlap though, you'll get an error

## on defer
I love how linear types somewhat mirror the functionality of `defer ...getRidOfIt();` but without the yucky control flow. All operations happen in the specified order in sloe!
