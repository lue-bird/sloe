made [a simple language (+ lsp)](https://codeberg.org/lue-bird/sloe) which compiles to zig/rust/js:
```sloe
fn Greet
    .name name str .buf buf Buf _origin, char
: .buf Buf _origin, char .span Span _origin =
    ? Buf-add-str-chars .buf buf .new "Hello, " str [string]
    ? Buf-span-add-str-chars .. string .new name [string]
    Buf-span-add .. string .new "!" char
```
The above example is more or less equivalent to
```elm
greet :
    { name : String, buf : Array Char }
    -> { buf : Array Char, span : { start : Int, length : Int } }
greet { name, buf } =
    let
        string0 = Array.append buf (Array.fromList (String.toList "Hello, "))
        string1 = Array.append string0 (Array.fromList (String.toList name))
        string2 = Array.push '!' string1
    in
    { buf = string2, span = {- would need to be updated on each array operation -} }
```
[online repl with more examples](https://lue-bird.github.io/sloe/)

why?
- memory safe without gc, rc or lifetimes
- mutation without reference semantics
- express tree structures without memory being scattered and without using raw indexes
- express collections that can store multiple of such trees

>>> thread

Language nerd description:
- every (!) type is linear. You for example even need to explicitly call `U32-dup` to copy an unsigned int and `U32-rid` to destroy one
- every collection and their contained slots and spans are tagged with a locally unique type.
  A collection could e.g. be of type `Buf names-origin, Span chars-origin` and you could point to an item in that collection by holding a `Slot names-origin` which gives you exclusive access to that index
- multiple locally-unique-typed values together can be isolated and erase their tag, thus allowing e.g. an outer collection to store multiple of them
- every user type is structural (records, choice types)
- types go bottom up. This means every expression has itself a known type that's independent from the context it is in (excluding pattern variables and locally unique types)
- if functions are called with pure funcitons as inputs, they are pure. Passing impure functions from the platform language will make the called function impure
- every syntax is open-ended. For example in a record, the last field value never needs to be parenthesized and there are no closing parens or similar. This means you get things like short-cutting case-of for free and avoid unnecessary indentation

Vaguely similar languages (although all are more ambitious): austral, rust, carbon, hylo (, swift, valen, dada)
