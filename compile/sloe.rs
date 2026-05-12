pub mod core;

pub type Name = compact_str::CompactString;
pub struct WithStartPosition<Value> {
    pub value: Value,
    pub start: lsp_types::Position,
}
pub struct SyntaxNode<Value> {
    pub span: lsp_types::Span,
    pub value: Value,
}

pub struct SyntaxProject<Expressions, Patterns, Types> {
    pub elements: Vec<SyntaxProjectElement<Expressions, Patterns, Types>>,
}

pub enum SyntaxProjectElement<Expressions, Patterns, Types> {
    // TODO comments
    ChoiceType {
        name: Option<SyntaxNode<Name>>,
        parameters: Vec<Name>,
        variants: Vec<SyntaxVariant<Types>>,
    },
    Fn {
        name: Option<SyntaxNode<Name>>,
        result: Option<SyntaxNode<SyntaxExpression<Expressions, Patterns>>>,
    },
    Unrecognized(Box<str>),
}
pub struct SyntaxVariant<Types> {
    // probably doesn't need to be Option
    pub name: Option<WithStartPosition<Name>>,
    pub value: Option<core::Slot<Types>>,
}
pub enum SyntaxType<Types> {
    Variable(Name),
    Fn {
        inputs: core::Span<Types>,
        output: core::Slot<Types>,
    },
}
pub enum SyntaxPattern<Patterns, Types> {
    Variable(Name),
    Variant {
        name: Name,
        type_arguments: Option<core::Span<Types>>,
        value: Option<core::Slot<Patterns>>,
    },
}
pub enum SyntaxExpression<Expressions, Patterns> {
    Fn {
        parameters: core::Span<Patterns>,
        result: core::Slot<Expressions>,
    },
}

struct ParseState<'a> {
    source: &'a str,
    offset_utf8: usize,
    position: lsp_types::Position,
}

fn str_starts_with_linebreak(str: &str) -> bool {
    // \r allowed because both \r and \r\n are counted as linebreak
    // see EOL in https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocuments
    str.starts_with("\n") || str.starts_with("\r")
}
fn parse_linebreak(state: &mut ParseState) -> bool {
    // see EOL in https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocuments
    if state.source[state.offset_utf8..].starts_with("\n") {
        state.offset_utf8 += 1;
        state.position.line += 1;
        state.position.character = 0;
        true
    } else if state.source[state.offset_utf8..].starts_with("\r\n") {
        state.offset_utf8 += 2;
        state.position.line += 1;
        state.position.character = 0;
        true
    } else if state.source[state.offset_utf8..].starts_with("\r") {
        state.offset_utf8 += 1;
        state.position.line += 1;
        state.position.character = 0;
        true
    } else {
        false
    }
}
/// prefer using after `parse_line_break` or similar failed
fn parse_any_guaranteed_non_linebreak_char(state: &mut ParseState) -> bool {
    match state.source[state.offset_utf8..].chars().next() {
        None => false,
        Some(parsed_char) => {
            state.offset_utf8 += parsed_char.len_utf8();
            state.position.character += parsed_char.len_utf16() as u32;
            true
        }
    }
}
/// prefer using after `parse_line_break` or similar failed
fn parse_any_guaranteed_non_linebreak_char_as_char(state: &mut ParseState) -> Option<char> {
    match state.source[state.offset_utf8..].chars().next() {
        None => None,
        Some(parsed_char) => {
            state.offset_utf8 += parsed_char.len_utf8();
            state.position.character += parsed_char.len_utf16() as u32;
            Some(parsed_char)
        }
    }
}
fn parse_any_char(state: &mut ParseState) -> bool {
    // can probably be optimized
    parse_linebreak(state) || parse_any_guaranteed_non_linebreak_char(state)
}
/// symbol cannot contain non-utf8 characters or \n
fn parse_symbol(state: &mut ParseState, symbol: &str) -> bool {
    if state.source[state.offset_utf8..].starts_with(symbol) {
        state.offset_utf8 += symbol.len();
        state.position.character += symbol.len() as u32;
        true
    } else {
        false
    }
}
/// symbol cannot contain non-utf8 characters or \n
fn parse_symbol_as<A>(state: &mut ParseState, symbol: &'static str, result: A) -> Option<A> {
    if parse_symbol(state, symbol) {
        Some(result)
    } else {
        None
    }
}
/// symbol cannot contain non-utf8 characters or \n
fn parse_symbol_as_span(state: &mut ParseState, symbol: &str) -> Option<lsp_types::Span> {
    let start_position: lsp_types::Position = state.position;
    if parse_symbol(state, symbol) {
        Some(lsp_types::Span {
            start: start_position,
            end: state.position,
        })
    } else {
        None
    }
}
/// given condition must not succeed on linebreak
fn parse_same_line_while(state: &mut ParseState, char_is_valid: impl Fn(char) -> bool) {
    let consumed_chars_iterator = state.source[state.offset_utf8..]
        .chars()
        .take_while(|&c| char_is_valid(c));
    let consumed_length_utf8: usize = consumed_chars_iterator.clone().map(char::len_utf8).sum();
    let consumed_length_utf16: usize = consumed_chars_iterator.map(char::len_utf16).sum();
    state.offset_utf8 += consumed_length_utf8;
    state.position.character += consumed_length_utf16 as u32;
}
fn parse_before_next_linebreak(state: &mut ParseState) {
    parse_same_line_while(state, |c| c != '\r' && c != '\n');
}
/// given condition must not succeed on linebreak
fn parse_same_line_char_if(state: &mut ParseState, char_is_valid: impl Fn(char) -> bool) -> bool {
    if let Some(next_char) = state.source[state.offset_utf8..].chars().next()
        && char_is_valid(next_char)
    {
        state.offset_utf8 += next_char.len_utf8();
        state.position.character += next_char.len_utf16() as u32;
        true
    } else {
        false
    }
}
fn parse_unsigned_integer_base10(state: &mut ParseState) -> bool {
    if parse_symbol(state, "0") {
        true
    } else if parse_same_line_char_if(state, |c| ('1'..='9').contains(&c)) {
        parse_same_line_while(state, |c| c.is_ascii_digit());
        true
    } else {
        false
    }
}

/// a valid sloe symbol that must be followed by a character that could not be part of an sloe identifier
fn parse_sloe_keyword_as_span(state: &mut ParseState, symbol: &str) -> Option<lsp_types::Span> {
    if state.source[state.offset_utf8..].starts_with(symbol)
        && !(state.source[(state.offset_utf8 + symbol.len())..]
            .starts_with(|c: char| c.is_ascii_alphanumeric() || c == '-'))
    {
        let start_position: lsp_types::Position = state.position;
        state.offset_utf8 += symbol.len();
        state.position.character += symbol.len() as u32;
        Some(lsp_types::Span {
            start: start_position,
            end: state.position,
        })
    } else {
        None
    }
}
fn parse_before_next_linebreak_or_end_as_str<'a>(state: &mut ParseState<'a>) -> &'a str {
    let content: &str = state.source[state.offset_utf8..]
        .lines()
        .next()
        .unwrap_or("");
    state.offset_utf8 += content.len();
    state.position.character += content.encode_utf16().count() as u32;
    content
}

fn parse_sloe_whitespace(state: &mut ParseState) {
    while parse_linebreak(state) || parse_same_line_char_if(state, char::is_whitespace) {}
}
fn parse_sloe_whitespace_until_linebreak(state: &mut ParseState) {
    while parse_same_line_char_if(state, |c| c != '\n' && c != '\r' && c.is_whitespace()) {}
}
fn parse_sloe_comment_lines_then_same_line_whitespace(
    state: &mut ParseState,
) -> Option<SyntaxNode<Box<str>>> {
    let start_position: lsp_types::Position = state.position;
    let first_comment_line: &str = parse_sloe_comment(state)?;
    let mut full_comment_content: String = first_comment_line.to_string();
    let _: bool = parse_linebreak(state);
    let mut end_position: lsp_types::Position = state.position;
    parse_sloe_whitespace_until_linebreak(state);
    while let Some(next_comment_line) = parse_sloe_comment(state) {
        full_comment_content.push('\n');
        full_comment_content.push_str(next_comment_line);
        let _: bool = parse_linebreak(state);
        end_position = state.position;
        parse_sloe_whitespace_until_linebreak(state);
    }
    Some(SyntaxNode {
        span: lsp_types::Span {
            start: start_position,
            end: end_position,
        },
        value: full_comment_content.into_boxed_str(),
    })
}
fn parse_sloe_comment<'a>(state: &mut ParseState<'a>) -> Option<&'a str> {
    if !parse_symbol(state, "#") {
        return None;
    }
    Some(parse_before_next_linebreak_or_end_as_str(state))
}
fn parse_sloe_lowercase_name(state: &mut ParseState) -> Option<Name> {
    let mut chars_from_offset: std::str::Chars = state.source[state.offset_utf8..].chars();
    if let Some(first_char) = chars_from_offset.next()
        && first_char.is_ascii_lowercase()
    {
        let parsed_length: usize = first_char.len_utf8()
            + chars_from_offset
                .take_while(|&c| c.is_ascii_alphanumeric() || c == '-')
                .map(char::len_utf8)
                .sum::<usize>();
        let end_offset_utf8: usize = state.offset_utf8 + parsed_length;
        let parsed_str: &str = &state.source[state.offset_utf8..end_offset_utf8];
        state.offset_utf8 = end_offset_utf8;
        state.position.character += parsed_str.encode_utf16().count() as u32;
        Some(Name::from(parsed_str))
    } else {
        None
    }
}
fn parse_sloe_lowercase_name_node(state: &mut ParseState) -> Option<SyntaxNode<Name>> {
    let start_position: lsp_types::Position = state.position;
    parse_sloe_lowercase_name(state).map(|name| SyntaxNode {
        span: lsp_types::Span {
            start: start_position,
            end: state.position,
        },
        value: name,
    })
}

fn parse_sloe_uppercase_name(state: &mut ParseState) -> Option<Name> {
    let mut chars_from_offset = state.source[state.offset_utf8..].chars();
    if let Some(first_char) = chars_from_offset.next()
        && first_char.is_ascii_uppercase()
    {
        let parsed_length: usize = first_char.len_utf8()
            + chars_from_offset
                .take_while(|&c| c.is_ascii_alphanumeric() || c == '-')
                .map(char::len_utf8)
                .sum::<usize>();
        let end_offset_utf8: usize = state.offset_utf8 + parsed_length;
        let parsed_str: &str = &state.source[state.offset_utf8..end_offset_utf8];
        state.offset_utf8 = end_offset_utf8;
        state.position.character += parsed_str.encode_utf16().count() as u32;
        Some(Name::from(parsed_str))
    } else {
        None
    }
}

fn parse_sloe_uppercase_name_node(state: &mut ParseState) -> Option<SyntaxNode<Name>> {
    let start_position: lsp_types::Position = state.position;
    parse_sloe_uppercase_name(state).map(|name| SyntaxNode {
        span: lsp_types::Span {
            start: start_position,
            end: state.position,
        },
        value: name,
    })
}

pub fn parse_syntax_project<Expressions, Patterns, Types>(
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
    project_source: &str,
) -> SyntaxProject<Expressions, Patterns, Types> {
    let mut elements = Vec::with_capacity(8);
    let mut state = ParseState {
        source: project_source,
        offset_utf8: 0,
        position: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    let mut last_parsed_was_valid = true;
    let mut last_parsed_valid_end_offset_utf8 = 0usize;
    'parsing_elements: loop {
        match parse_syntax_project_element(&mut state, expressions, patterns) {
            None => {
                last_parsed_was_valid = false;
                if !parse_any_char(&mut state) {
                    elements.push(SyntaxProjectElement::Unrecognized(Box::from(
                        &project_source[last_parsed_valid_end_offset_utf8..],
                    )));
                    break 'parsing_elements;
                }
            }
            Some(element) => {
                if !last_parsed_was_valid {
                    elements.push(SyntaxProjectElement::Unrecognized(Box::from(
                        &project_source[last_parsed_valid_end_offset_utf8..],
                    )));
                }
                elements.push(element);
                last_parsed_was_valid = true;
                last_parsed_valid_end_offset_utf8 = state.offset_utf8;
            }
        }
    }
    SyntaxProject { elements: elements }
}

fn parse_syntax_project_element<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
) -> Option<SyntaxProjectElement<Expressions, Patterns, Types>> {
    todo!()
}

pub struct CompiledProject {
    pub rust: syn::File,
    pub choice_types: std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    pub fns: std::collections::HashMap<Name, CompiledProjectFnInfo>,
    pub records: std::collections::HashSet<Vec<Name>>,
    // TODO
}

pub struct CompiledChoiceTypeInfo {
    // TODO
}

pub struct CompiledProjectFnInfo {
    // TODO
}

pub fn project_compile_to_rust<Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    syntax: &SyntaxProject<Expressions, Patterns, Types>,
) -> CompiledProject {
    todo!()
}

pub struct ErrorNode {
    pub message: Box<str>,
    pub span: lsp_types::Span,
}

pub fn compiled_rust_to_file_content(rust_file: &syn::File) -> String {
    prettyplease::unparse(rust_file)
}

pub enum SyntaxHighlightKind {
    KeySymbol,
    Field,
    Type,
    Variable,
    Variant,
    DeclaredVariable,
    Comment,
    Number,
    String,
    TypeVariable,
}
