Small, fast pure functional programming language where indexes are valid and values can't be shared.

The goal is representing tree-like data structures without segmented memory or plain index integers (along with the need to handle failure and generations),
instead offering a safe, infallible way to refer to values and slices stored in flat memory structures.

[skip to examples](#examples)

Note that while as a side effect this avoids any bounds checks,
bounds-checking in general is not slow (typically only around 2% slower than unchecked access in practice).

Install with

```bash
cargo install --git https://github.com/lue-bird/sloe sloe
```

> ⚠️ do not use, yet

# concept: each value can only be used used/consumed at most once
Matching a value? Consumes it. Passing a value as an argument? Consumes it.
Even e.g. variables holding plain numbers have to be explicitly duplicated to use them in multiple places.

This allows
- values know when they aren't used anymore at compile time. Their memory can be reclaimed without garbage collection or similar
- values can be mutated internally without mutation being detectable
- representing things that can only be consumed once, like thread join handles

This can feel annoying and clunky. Think e.g. `fn vec-occupied-count .vec _vec ... :> .vec ... .occupied-count u32`.
Not ony is it clunky, it is also conceptually less constrained than taking an immutable view (like &Vec in rust) because `vec-occupied-count` could return a modified vec.

The _big_ advantage is that it is easy to understand and _way simpler and faster to statically analyze_ than lifetimes or similar.

Further reading if interested: "affine types", rust owned values.

# concept: flat memory collection `vec`
A collection which can mark some spans within itself as vacant.
This can be used to "return" memory which has become outdated or useless with for example `vec-element` and `vec-span-add-vec-span`.
Note that this functionality is entirely optional and you can at no cost just use it for temporary builders etc.

This concept is often called slot map, reusing memory.
In rust, a prominent example is [slab](https://docs.rs/crate/slab/latest).
Various kinds of similar rust collections are compared here: https://donsz.nl/blog/arenas/

# concept: distinct origin of a collection in your code
Every created collection has a correlated origin.
A value whose type contains an origin can't escape the function scope of it's origin.
This is checked at compile-time for the expression following origin creation but you'll likely realize it before then:
```
fn some-vec . :> _vec ??origin cannot even be annotated??, u32 >
    origin vec-origin
    ? _vec-empty<u32> vec-origin = vec >
    ? _vec-add .vec vec .new 123 u32 = .vec vec .slot slot >
    ...
    vec

# compiles
fn add-some-values<Origin> vec _vec Origin, u32 :> _vec Origin, u32 >
    ? _vec-add .vec vec .new 123 u32 = .vec vec .slot slot >
    ...
    vec
```

Further reading if interested: The idea of "fresh, distinct type instances by code" seems to generally be called "path-dependent types". In rust I know of 2 crates that successfully implement this: https://docs.rs/compact_arena/0.5.0/compact_arena/index.html (safe, pragmatic, simple but bare-bones) and https://docs.rs/indexing/0.4.1/indexing/ (safe, cumbersome, complicated).
The same idea but with runtime checking instead of compile-time checking can quite easily be implemented by storing an ID in each collection and the same id in each contained slot, and incrementing a global variable (or similar) for the next available ID: https://github.com/thomcc/handy/blob/master/src/lib.rs#L111-L126
(apart from security I'm not sure this is ever worth it for regular users, considering it is also slower).

# examples
## pass in an origin from the outside (rare)
```
fn vec-empty<Element> origin _origin Origin :> _vec Origin, Element
```
shift the responsibility for cleanup to the caller.
This is done for most initializer functions, e.g. for the initial persistent application state.
For most other functions, it's more common to pass in an existing collection

## creating a new origin, slots and spans
`origin some-name` creates a new origin variable and a local unique type for the start offset of its scope.
An origin type does not have a `-dup` helper and thus can only be used for one collection.
At the end of the underlying origin of the annotated origin type, the memory of the value with that origin will be deallocated.
```
# use a temporary value within a scope
fn use-vec . :> u32 >
    origin vec-origin
  	? _vec-empty<u32> vec-origin = vec >
  	? _vec-add .vec vec .new 123 u32 = .vec vec .slot first-slot >
  	? _vec-element .vec vec .slot first-slot = .vec vec .element first >
  	? _vec-span-build-empty vec = after-first >
  	? _vec-opt-span-build-add .build after-first .new 456 u32 = after-first >
  	? _vec-span-build-add .build after-first .new 789 u32 after-first >
    ? _vec-span-build after-first = .vec vec .span span-after-first >
    ...
  	first # = 123 u32

# different branches, different scopes
fn use-opt opt _opt u32 :> ... >
    # this won't compile as their origins come from different branches
    :(
        ? opt
        = |absent . >
            origin vec-origin
            _vec-empty<u32> vec-origin
        = |present number > (
            origin vec-origin
            ? _vec-one .origin vec-origin .element number = .vec vec .slot slot >
            ...
            vec
        )
    )
    = vec >
    # this will compile:
    origin vec-origin
    :(
        :opt
        = |absent . >
            _vec-empty<u32> vec-origin
        = |present number > (
            ? _vec-one .origin vec-origin .element number = .vec vec .slot slot >
            ...
            vec
            )
        )
    = vec >
    ...

# recursive structure. One cool thing is that expression will turn every slot
# into an exclusive slot
ty expression Expressions-origin Patterns-origin Str-origin
    |int i32
    |string _opt _span Str-origin
    |vec _opt _span Expressions-origin
    |call
        .function _slot Expressions-origin
        .arguments _span Expressions-origin
    |lambda
        .parameters _span Patterns-origin
        .result _slot Expressions-origin

ty state Expressions-origin
    # ...patterns, strings etc
    .expressions _vec Expressions-origin, _expression Expressions-origin
    .root-expression _expression Expressions-origin

fn initial-state
    .expressions-origin expressions-origin _origin Expressions-origin
    :> _state Expressions-origin >
    .expressions _vec-empty<_expression Expressions-origin, ...> expressions-origin
    .root-expression (..do parsing..)

fn state-to-interfaces-into
    .interfaces interfaces _vec Interfaces-origin, _interface _state Expressions-origin
    .state state _state Expressions-origin
    :> _vec Interfaces-origin, _interface _state Expressions-origin >
    :(
        vec-one
        .origin interfaces-origin
        .element |console-log (_interface _state Expressions-origin) "hello"
    )
    = .slot slot .vec interfaces >
    ...
    interfaces
```

# known limitations
- scattered sub-spans/slots in a persistent vec cannot be easily de-allocated in bulk (so without walking the whole tree and removing spans and slots one by one, aka pointer chasing).
  For example, preferably expressions etc. would be stored in different spans per module, each with their own origin for bulk de-allocation and new allocation.
  However, this would mean that slots and spans within the AST would not be owning
- the pattern of removing, then re-inserting an element at a slot just to access it (potentially immutably) is not optimal. This can be mitigated somewhat by using `vec-update` & friends or compiling to/asking for code that uses `vec-replace .slot new-element .slot old-element` with a dummy element followed by `vec-replace` ignoring the returned dummy new-element instead

# syntax
Syntax is secondary but I tried to make it coherent and practical, avoiding parens and indentation when possible, especially for trailing syntax.
```
# line comment

# number type, so for example
3.2 f32 # number types are p32, u32, i32, f32

# str
"hello"

# char
'a'

# most identifiers
some-function-or-variable-or-field-or-type-or-variant-name-2012

# type variable name
Some-type-variable-name

# function call, always starting with _
# Functions can require space-separated type arguments in <...>.
# Any function is of type `fn` and always requires an argument (which does not need to be parenthesized)
_some-function<Type Arguments> inner-call-as-the-argument inner-inner-argument

# record. if values are open-ended they need to be parenthesized.
# The last field value can end in a record without needing to be parenthesized
.first-field first-value .second-field second-value

# "empty value", like an empty record/void/unit.
# commonly used for variants "without a value" or empty state
.

# local fn of type fn.
# the pattern must add a type to all variables
# can **not** use variables from the outer scope.
fn parameter-pattern > result

# pattern variable
# appending a type is only necessary and allowed in function parameters
some-variable some-type

# pattern match, checked for exhaustiveness. expressions must be parenthesized if they themselves end in a query.
# The last case result does not need to be parenthesized
? value = first-case-pattern > first-result = second-case-pattern > second-result

# introduce a new origin. The given name can be used as a variable and type
origin new-origin-name expression-that uses the-origin

# project function declaration
fn function-name<Potential Type-Arguments Only-Used-In-The-Result>
    parameter-pattern-usually-wrapped-in-parens
    :> result-type-usually-wrapped-in-parens >
    # optional documentation
    # comment
    result-expression

# type name without arguments
u32

# type with multiple arguments.
# Arguments before the last must be parenthesized if they end in a type with arguments
_span Origin
_my-function Env, Input, Output

# project type which is an alias for an existing type.
# Here a "choice type" that can come in different shapes ("variants")
# which each have a unique name and one associated value.
ty type-name Potential Type-Parameters
    |first-option .
    |second-option _vec Potential, u32
    |third-option _type-name-alias Potential, Type-Arguments

# creating a variant. Note that the type could refer to a type alias or be a choice type directly (|... ...)
|some-variant its-choice-type its value

# variant pattern
|some-variant its value
```
(This list might be incomplete, examples show more)

# potential improvements in the (far) future
- add field and variant rename and references
- suggest full parameter field patterns of existing project fns (just as rust does). This is super convenient, especially because stuff like `expressions vec Expressions, expression Expressions Patterns Types` doesn't exactly roll easily over one's keyboard
- add `set Origin, Element` with an initialization function like `set-empty .origin ... .hash fn Element, Hash :> set Origin, Element`
- add something like `map Origin, Key, Value` (or just `map Origin, Element` where key is derived from element) which still gives out `slot Origin`s for each entry but can be queried by key or similar. `map-empty` will require providing a `fn .a Key .b Key, .a Key .b Key .order order` or similar.
  Alternatively, check if implementing in userland via e.g. AVL or red-black tree backed by a regular `vec` is fast enough
- consider adding record update syntax
- consider adding `vec-generational`, `slot-strong` (and `slot-weak`?) which can reference a slot that is already in use, without any guarantee that it still points to an occupied slot: `slot-dup-weak slot-strong slot Origin :> .slot slot-strong, Origin .weak slot-weak Origin` + `slot-weak-dup`. This would enable graph structures, child-parent relations, doubly-linked lists etc.
  **important**: This requires generational slots.
  Can be implemented using e.g. [slotmap](https://docs.rs/crate/slotmap/latest), [thunderdome](https://docs.rs/crate/thunderdome/latest), [riddance](https://docs.rs/riddance/latest/riddance/) or on top of `vec` with an added generation counter in slot and collection.
  Shelved for now because the model is quite different and I don't yet have a use case
- consider adding `vec-add-vacant vec _vec Origin, Element :> _vec Origin, ELement` (probably same for vacant length)
  and `vec-fold-including-vacant .vec vec _vec Origin, Element .state State .step fn .state State .element opt Element, State :> State` (and maybe ways to peek for vacant spaces by plain `u32`)
  which usefully exposes a way to use vacant spaces
- verify that origin creation is correct for all kinds of recursion! e.g. this one seems on the edge of correct:
  _different vecs have the same origin_ but their slots can't intermix.
  ```
  fn recurse
      .consume-origin consume-origin Consume-origin
      .result-origin result-origin Result-origin
      :> _vec Result-origin, u32 >
      origin local-origin
      ? _vec-empty<u32> consume-origin = temporary >
      ? _recurse local-origin result-origin = result >
      ? _vec-add .a temporary .b 1 u32 = .slot slot .vec temporary >
      ...
      result
  ```
  If we find a problem, creating a new `origin` should be disallowed in (mutually) recursive calls.
  This is a bit restrictive but alright I believe.
  If feeling motived, look into proof languages and make sure this is rock solid
- improve memory efficiency of string operations (currently vec of char)
- allocate all collections with an origin that was declared in sloe using a locally-passed `impl Allocator<>`. This preferably builds on a stabilized allocator feature
- I think in theory there should be all the bits and pieces present to allow for struct-of-arrays and arrays-of-variant-values (made up name). E.g. internally compiling
    - `vec Origin, .a A .b B` to `A·B<Vec<A>, Vec<B>>`
    - for `vec Origin, |a A |b B` to either 
        - `Tag·ValueIndex·A·B<Vec<A_or_B_Tag>, Vec<u32>, Vec<A>, Vec<B>>` (which also has ~2 hops but makes sense when sizes of A and B are different enough)
        - `A·B<Vec<A>, Vec<B>>` (which requires the index to hold both the tag and the value index, aka 64 bit instead of 32, which somewhat defeats the point of reducing padding of the variant when values get bigger. Potentially there could however be struct-of-arrays for individual variant values making this worth it: https://github.com/dist1ll/osmium & https://alic.dev/blog/dense-enums)
        - `A·B<Vec<A>, Map<u32, B>>` (which is inefficient, and wasteful if `B` is common, and also doesn't scale with more than 2 variants)
- look into soa_derive for rust, maybe this already does most of the useful work
- try adding a compiler output to zig or similar which I think fits well (few free(), no need for lifetimes, fast compilation, anonymous structs, allocators, MultiArrayList. Downsides: pattern matching is underdeveloped, less-utilized memory niches (e.g. I think no NonZeroU32 and variant-in-variant niche usage for example), ecosystem, no language-level ownership, making sloe values prone to mistakes south of the the ffi border) and the overall philosophy (explicitness, data oriented)
- imagine what a logic programming language with this concept would look like. I imagine it wouldn't look much different (!) though with some different tradeoffs (e.g. more complex stdlib and compiler output, potentially a different typing and exhaustivess system)

# not coherently formulated thoughts
in rust, collections tend to own their element data, so safely keeping references to inside is tough.
This relationship is flipped on it's head in sloe: All elements of collections are divided into slots and spans
which are owned by the code that parked values there i the first place.

Honestly this idea seems to overwhelmingly useful that I'm surprised I can't find other languages that lean into it (I only know of rust which at least enables it in userland).

One way this helps is that nested collections aren't segmented: what is usually `Vec<Box<str>>` aka n separate memory pieces can be e.g. `_vec ... _span str-origin` + `_str str-origin`
(in rust there are I think crates like oroborus for this)

## on shadowing
since each variable can be used at most once, most introduced names that would traditionally be considered "shadowed" are aready out of scope in sloe. When their scopes actually overlap though, you'll get an error

# rejected ideas
As a hobby language that deliberately cannot by itself interface with the operating system, C etc. we can afford to skip many complex features. First some smaller-scale rejected ideas

- allow expressions whose type is known (basically anything except inputs to queries) to omit extra type info (namely number, variant<> and project-fn<>). I'm a little torn because this makes construction inconsistent and increases the distance between the known type and expression. On the other hand this is already the case for patterns (deliberately so) but has a much higher convenience gain there
- add special syntax `fn-once` that automatically assembles the environment from the used local variables.
  Rejected in favor of more explicit construction with contextual names and potentially multiple fns.
  More info in "not coherently formulated thoughts"
- add tuples: (* a * b * c). I dislike them conceptually but operations like `u32-add` or `u32-dup` are nicer with them. The field names `.a .b ` are just noise. Adding tuples might make for a nicer user interface when calling from rust
- consider allowing `origin name` at the project scope. This allows reducing the number of type parameters flying around in things like `expression Expressions Patterns Types Source Cases ...` if desired.
It also makes initial_state much easier to call from the rust side (though we need to be careful how...).
  Rejected because this makes it more or less impossible to run multiple sloe instances from a single rust program
  Issue is that in general single-return-continuation is rare in sloe
- requiring all (!) generic type parameters to be passed to calls
  I feel like this is more "natural", easier to type-check but way more verbose / redundant.
  And especially because having _many_ origin type variables is common, this sadly won't fly
- adding function call syntax sugar similar to piping.
  While this is bloody wonderful (succinct, intuitive-ish, great for builders), it doesn't quite have much of a purpose which pattern matching doesn't fill well already. But more importantly it is quite limiting (requires positional arguments, requires them in the right order, doesn't apply to variants and similar). It also introduces "yet another way of writing the same code" which is dislike
- making `vec` etc store multiple kinds of data (heterogenous) and letting them give out `slot origin data-type` and `span origin data-type`. This means that usually only one `origin` needs to be passed to things like `expression` and slots/spans actually tell you what data they point to.
  This makes the porpose of `vec` being allocator-ish spaces rather that collections to query and edit more clear and makes passing them around to operations very simple, e.g. `expression-end .expression _expression Origin .data _vec Origin, ... :> .vec _vec Origin ... .end text-position`.
  This would also in theory enable a crazy representation of tagged unions as:
  ```sloe
  expression-slot origin
      |int slot origin, i32
      |plus slot origin, .left _expression-slot origin .right _expression-slot origin
      ...
  ```
  This also means slices etc need to be stored separately in the origin vec.
  The issue currently is that it feels hard to optimally construct/query such a heterogenous structure.
  Its structure _must_ be created at compile-time. Dynamically this doesn't fly: `vec origin = { bucket: Map<for type_byte_size: { key: type_byte_size, value: Vec<type_byte_size> }> }`.
  However, really providing this in sloe would require sloe to add _some_ kind of "type variable must be record" constraint:
  ```
  origin vec-origin
  ? _vec-empty<.expression expression .pattern pattern vec-origin> vec-origin = vec >
  ? _vec-add .vec vec .new some-expression = .vec vec slot some-expression-slot >
  ? _vec-add .vec vec .new some-pattern  = .vec vec .slot some-pattern-slot >
  ...
  ```
  This is probably doable in zig but hardly in rust without significant macro magic. Any ideas welcome!


## why no `&mut`/`inout`
While seemingly convenient and magnitudes better than regular mutable pointers,
- it's less obvious than passing values through
- there's no easy way to change the name of a resulting value that represents something different now
- there's no way to "reconstruct" a different out value. Especially for non-trivial edits the &mut approach can get messy or it's straight up impossible and parts will need to get cloned unnecessarily
- there's no way to change the type (e.g. from `span` to `span-filled`)
- there's no there's two ways to specify most conversions, with usually no clear method of converting one to the other
- it's surprisingly common that one path consumes an argument, the other path keeps it in tact (e.g. when searching a tree with intermediate information. Either we find something, consuming the context or we come up empty-handed with the original context, like `fn .context context ... :> |exit found |go-on context` where found contains some parts of the context). This isn't modelled well with `&mut`
- `&mut` means the resulting changed collection is not returned, making use as the input to another function impossible. This almost necessarily results in the classic procedural-style statement form as opposed to the functional-style expression form. Minor gripe: especially in languages that don't allow local scopes with local returns (far, far too many) this basically makes it impossible to locally introduce a value, change it and implant it somewhere; instead you have to move the variable up to the top level.
- returning `.` (like returning `Unit` in gleam) feels super awkward to my brain. Most often, languages then automatically return void/... in the absence of a return and introduce all kinds of constructs like re-assignable variables, additional constructs for looping and branching that all can only return void/... . To my brain, this just confuses matters; it loves simple to follow flow of state!
- &mut usually comes with the need to check for non-overlapping references to the same parts of data. This isn't possible with owned data passing in the first place

rusts immutable references `&` have some similar trade-offs but seem kind of unavoidable at least for languages like rust.

## why no closures that capture environment variables automatically like in rust
- its type cannot be specified. as such, it cannot generally be stored as part of a type
- no clear unified interface. There could be multiple functions, there could be an output that additionally returns the captured values (allowing it to be called again), there could be an output that only sometimes returns the captured variables, etc.
- I personally never had a need for this. Usually you can just make the environment a type variable and you're golden

I'm strangely really convinced that this is the obvious, correct design decision (for most programming languages at that!) which really surprises me.
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
- operators introduce a good amount of complexity: infix (and prefix) notation, associativity, precedence, most likely a way to overload based on context.
- edge-case behavior (e.g. saturating vs overflowing vs checked vs ...) should be easier to control
- in general, operators are concise but as a result quite ambiguous. For example, changing a boolean to an integer may silently not generate a compiler error when `!` is binary not, or when changing a list to a string with `++`
- while numbers are not exactly uncommon, there are features that would deserve these symbols more, even in typical imparative languages (think `return`, `switch { case }`, `structure`, `import`, `public`, `static`, ...)
- 

Somehow despite it's issues (math syntax kind of sucks, even the tiny subset), operators are one of the most prevalent features in programming languages, even hobby and experimental ones (0th class citizen). I do not quite understand this (well I guess not adding operators adds to the weirdness budget) .

## why no single-field access
a.k.a `record.field`. Quick and easy answer: Because this makes it embarassingly easy to forget handling a field (now or in the future). I've identified this as the second most common source of bugs in my own code. And in sloe, not handling a field could mean leaking some memory, so it would be even worse potentially!

## why no positional function arguments
- with positional arguments it isn't really possible to make the last argument open ended (at least with keeping the current syntax)
- it's tough (usually) to annotate a function whose arguments and argument types are unknown. E.g. what would `fn-dup`'s type be?
- positional arguments (usually) means no passing in bulk
  ```sloe
  fn u32-square natural u32 :> u32 >
      _u32-add _u32-dup natural
  ```

"Positionality" in general is pretty much absent in sloe. E.g. positional arguments are super convenient, so they tend to be used for everything, even arguments that would benefit from a clear description.
Sloe had positional arguments once, largely because the rust-sloe interface is simpler in rust with positional arguments.

# dev setup
to re-compile
```bash
cargo install --offline --debug --path . sloe
```

# TODO

- correct expression variant type formatting

- change `ty` syntax to reflect usage, so
  ```sloe
  ty no-arguments aliased-type
  ty _single-argument Argument aliased-type
  ty _multiple A, B, C aliased-type
  ```
  one tiny win is that identity type aliases get parsed correctly

- fix bugs and inline TODOs including completions for functions (should not wrap fields and replace anything before it!)

- officially convert values from "affine" (<= 1 use) to "linear" (exactly 1 use) to avoid potential leaks (https://smallcultfollowing.com/babysteps/blog/2023/03/16/must-move-types/). I think this would work great but leads to a bunch of cleanup for temporary vec members (which most likely would get optimized away though).
  Doing this "change" mostly means changing error messages and documentation

- add fold2s and or preferably a ways to fold over arbitrarily many spans etc.
  This should make things like `.field-names span Field-names .field-values span Values`
  much more attractive/viable. This pattern can avoid "type parameter spam" for any record

- add `vec-span-add-repeat`

- (not specific, yet) change function type arguments from `<First Second>` to `[First; Second]` and `|variant-name type value-expression` to `[type] |variant-name value-expression`
