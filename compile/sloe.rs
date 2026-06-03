#![allow(non_upper_case_globals)]

use quote::TokenStreamExt;
pub mod core;

pub type Name = compact_str::CompactString;
#[derive(Clone, Copy, Debug)]
pub struct WithStartPosition<Value> {
    pub value: Value,
    pub start: lsp_types::Position,
}

#[derive(Debug)]
pub struct SyntaxProject<Expressions, Patterns, Types> {
    pub elements: Vec<SyntaxProjectElement<Expressions, Patterns, Types>>,
}

#[derive(Debug)]
pub enum SyntaxProjectElement<Expressions, Patterns, Types> {
    TypeAlias {
        type_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        parameters: Vec<WithStartPosition<Name>>,
        documentation: Option<SyntaxComments>,
        type_: Option<SyntaxType<Types>>,
    },
    ChoiceType {
        choice_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        parameters: Vec<WithStartPosition<Name>>,
        documentation: Option<SyntaxComments>,
        variants: Vec<SyntaxVariant<Types>>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        type_parameters: Option<SyntaxAngledTypeParameters>,
        parameter: Option<SyntaxPattern<Patterns, Types>>,
        result_type: Option<SyntaxType<Types>>,
        documentation: Option<SyntaxComments>,
        result: Option<SyntaxExpression<Expressions, Patterns, Types>>,
    },
    Comments(SyntaxComments),
    Unrecognized {
        range: lsp_types::Range,
        source: Box<str>,
    },
}
#[derive(Clone, Debug)]
pub struct SyntaxComments {
    pub line0: WithStartPosition<Box<str>>,
    pub line1_up: Vec<WithStartPosition<Box<str>>>,
}
#[derive(Debug)]
pub struct SyntaxVariant<Types> {
    pub open_paren_start: lsp_types::Position,
    pub name: Option<WithStartPosition<Name>>,
    pub type_parameters: Option<SyntaxAngledTypeParameters>,
    pub value: Option<SyntaxType<Types>>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Clone, Debug)]
pub struct SyntaxAngledTypeParameters {
    pub open_angle_start: lsp_types::Position,
    pub names: Vec<WithStartPosition<Name>>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxType<Types> {
    Variable(WithStartPosition<Name>),
    Record {
        ampersand_start: lsp_types::Position,
        fields: Vec<SyntaxField<SyntaxType<Types>>>,
    },
    Construct {
        name: WithStartPosition<Name>,
        arguments: Option<core::Span<Types>>,
    },
    Parenthesized {
        open_paren_start: lsp_types::Position,
        inner: Option<core::Slot<Types>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
}
#[derive(Debug)]
pub enum SyntaxPattern<Patterns, Types> {
    Variable {
        name: WithStartPosition<Name>,
        type_: Option<SyntaxType<Types>>,
    },
    Variant {
        name: WithStartPosition<Name>,
        type_arguments: Option<SyntaxAngledTypeArguments<Types>>,
        value: Option<core::Slot<Patterns>>,
    },
    Record {
        ampersand_start: lsp_types::Position,
        fields: Vec<SyntaxField<SyntaxPattern<Patterns, Types>>>,
    },
    Parenthesized {
        open_paren_start: lsp_types::Position,
        inner: Option<core::Slot<Patterns>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
}
#[derive(Clone, Debug)]
pub struct SyntaxField<Value> {
    pub name: WithStartPosition<Name>,
    pub left_angle_start: Option<lsp_types::Position>,
    pub value: Option<Value>,
}
#[derive(Debug)]
pub struct SyntaxAngledTypeArguments<Types> {
    pub open_angle_start: lsp_types::Position,
    pub types: Option<core::Span<Types>>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxExpression<Expressions, Patterns, Types> {
    Number {
        value: WithStartPosition<Box<str>>,
        type_: Option<SyntaxType<Types>>,
    },
    Char {
        open_quote_start: lsp_types::Position,
        content: Option<char>,
        content_end: lsp_types::Position, // consider storing the content length
        closed_quote_exists: bool,
    },
    Str {
        open_quote_start: lsp_types::Position,
        content: Box<str>,
        content_end: lsp_types::Position, // consider storing the content length
        closed_quote_exists: bool,
    },
    VariableOrCall {
        name: WithStartPosition<Name>,
        type_arguments: Option<SyntaxAngledTypeArguments<Types>>,
        argument: Option<core::Slot<Expressions>>,
    },
    Variant {
        name: WithStartPosition<Name>,
        type_arguments: Option<SyntaxAngledTypeArguments<Types>>,
        value: Option<core::Slot<Expressions>>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        parameter: Option<SyntaxPattern<Patterns, Types>>,
        result: Option<core::Slot<Expressions>>,
    },
    Record {
        ampersand_start: lsp_types::Position,
        fields: Vec<SyntaxField<SyntaxExpression<Expressions, Patterns, Types>>>,
    },
    Parenthesized {
        open_paren_start: lsp_types::Position,
        inner: Option<core::Slot<Expressions>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Commented {
        comments: SyntaxComments,
        expression: Option<core::Slot<Expressions>>,
    },
    Query {
        colon_start: lsp_types::Position,
        queried: Option<core::Slot<Expressions>>,
        cases: Vec<SyntaxExpressionQueryCase<Expressions, Patterns, Types>>,
    },
    Origin {
        origin_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        result: Option<core::Slot<Expressions>>,
    },
}
#[derive(Debug)]
pub struct SyntaxExpressionQueryCase<Expressions, Patterns, Types> {
    pub pattern: SyntaxPattern<Patterns, Types>,
    pub left_angle_start: Option<lsp_types::Position>,
    pub result: Option<SyntaxExpression<Expressions, Patterns, Types>>,
}

pub fn name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, name.value.encode_utf16().count() as u32)
}
pub fn syntax_name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: name_end(name),
    }
}
pub fn variant_end<Types>(
    variant: &SyntaxVariant<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    variant
        .closed_paren_start
        .or_else(|| variant.value.as_ref().map(|value| type_end(value, types)))
        .or_else(|| {
            variant
                .name
                .as_ref()
                .map(|variant_name| name_end(with_start_position_as_ref(variant_name)))
        })
        .unwrap_or(variant.open_paren_start)
}
pub fn type_range<Types>(
    type_: &SyntaxType<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: type_start(type_),
        end: type_end(type_, types),
    }
}
pub fn type_start<Types>(type_: &SyntaxType<Types>) -> lsp_types::Position {
    match type_ {
        SyntaxType::Variable(name) => name.start,
        SyntaxType::Record {
            ampersand_start,
            fields: _,
        } => *ampersand_start,
        SyntaxType::Construct { name, arguments: _ } => name.start,
        SyntaxType::Parenthesized {
            open_paren_start,
            inner: _,
            closed_paren_start: _,
        } => *open_paren_start,
    }
}
pub fn type_end<Types>(
    type_: &SyntaxType<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    match type_ {
        SyntaxType::Variable(name) => name_end(with_start_position_as_ref(name)),
        SyntaxType::Record {
            ampersand_start,
            fields,
        } => fields
            .last()
            .map(|last_field| field_end(last_field, |value| type_end(value, types)))
            .unwrap_or_else(|| symbol_end(*ampersand_start, "&")),
        SyntaxType::Construct { name, arguments } => types
            .opt_span_slice(core::Opt::from_option(arguments.as_ref()))
            .last()
            .map(|last_argument| type_end(last_argument, types))
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxType::Parenthesized {
            open_paren_start,
            inner,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| {
                inner
                    .as_ref()
                    .map(|inner| type_end(types.element(inner), types))
            })
            .unwrap_or_else(|| symbol_end(*open_paren_start, "(")),
    }
}

pub fn pattern_range<Patterns, Types>(
    pattern: &SyntaxPattern<Patterns, Types>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: pattern_start(pattern),
        end: pattern_end(pattern, patterns, types),
    }
}
pub fn pattern_start<Patterns, Types>(
    pattern: &SyntaxPattern<Patterns, Types>,
) -> lsp_types::Position {
    match pattern {
        SyntaxPattern::Variable { name, type_: _ } => name.start,
        SyntaxPattern::Variant {
            name,
            type_arguments: _,
            value: _,
        } => name.start,
        SyntaxPattern::Record {
            ampersand_start,
            fields: _,
        } => *ampersand_start,
        SyntaxPattern::Parenthesized {
            open_paren_start,
            inner: _,
            closed_paren_start: _,
        } => *open_paren_start,
    }
}
pub fn pattern_end<Patterns, Types>(
    pattern: &SyntaxPattern<Patterns, Types>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => type_
            .as_ref()
            .map(|type_| type_end(type_, types))
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxPattern::Variant {
            name,
            type_arguments,
            value,
        } => value
            .as_ref()
            .map(|value| pattern_end(patterns.element(value), patterns, types))
            .or_else(|| {
                type_arguments
                    .as_ref()
                    .map(|type_arguments| angled_type_arguments_end(type_arguments, types))
            })
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxPattern::Record {
            ampersand_start,
            fields,
        } => fields
            .last()
            .map(|last_field| field_end(last_field, |value| pattern_end(value, patterns, types)))
            .unwrap_or_else(|| symbol_end(*ampersand_start, "&")),
        SyntaxPattern::Parenthesized {
            open_paren_start,
            inner,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| {
                inner
                    .as_ref()
                    .map(|inner| pattern_end(patterns.element(inner), patterns, types))
            })
            .unwrap_or_else(|| symbol_end(*open_paren_start, "(")),
    }
}
pub fn field_range<Value>(
    field: &SyntaxField<Value>,
    value_end: impl FnOnce(&Value) -> lsp_types::Position,
) -> lsp_types::Range {
    lsp_types::Range {
        start: field_start(field),
        end: field_end(field, value_end),
    }
}
pub fn field_start<Value>(field: &SyntaxField<Value>) -> lsp_types::Position {
    field.name.start
}
pub fn field_end<Value>(
    field: &SyntaxField<Value>,
    value_end: impl FnOnce(&Value) -> lsp_types::Position,
) -> lsp_types::Position {
    field
        .value
        .as_ref()
        .map(value_end)
        .or_else(|| {
            field
                .left_angle_start
                .map(|left_angle_start| symbol_end(left_angle_start, "<"))
        })
        .unwrap_or_else(|| name_end(with_start_position_as_ref(&field.name)))
}
pub fn angled_type_arguments_range<Types>(
    type_arguments: &SyntaxAngledTypeArguments<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: type_arguments.open_angle_start,
        end: angled_type_arguments_end(type_arguments, types),
    }
}
pub fn angled_type_arguments_end<Types>(
    type_arguments: &SyntaxAngledTypeArguments<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    type_arguments
        .closed_angle_start
        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
        .or_else(|| {
            types
                .opt_span_slice(core::Opt::from_option(type_arguments.types.as_ref()))
                .last()
                .map(|last_type| type_end(last_type, types))
        })
        .unwrap_or_else(|| symbol_end(type_arguments.open_angle_start, "<"))
}
pub fn expression_range<Expressions, Patterns, Types>(
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: expression_start(expression),
        end: expression_end(expression, expressions, patterns, types),
    }
}
pub fn expression_start<Expressions, Patterns, Types>(
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
) -> lsp_types::Position {
    match expression {
        SyntaxExpression::Number { value, type_: _ } => value.start,
        SyntaxExpression::Char {
            open_quote_start,
            content: _,
            content_end: _,
            closed_quote_exists: _,
        } => *open_quote_start,
        SyntaxExpression::Str {
            open_quote_start,
            content: _,
            content_end: _,
            closed_quote_exists: _,
        } => *open_quote_start,
        SyntaxExpression::VariableOrCall {
            name,
            type_arguments: _,
            argument: _,
        } => name.start,
        SyntaxExpression::Variant {
            name,
            type_arguments: _,
            value: _,
        } => name.start,
        SyntaxExpression::Fn {
            fn_keyword_start,
            parameter: _,
            result: _,
        } => *fn_keyword_start,
        SyntaxExpression::Record {
            ampersand_start,
            fields: _,
        } => *ampersand_start,
        SyntaxExpression::Parenthesized {
            open_paren_start,
            inner: _,
            closed_paren_start: _,
        } => *open_paren_start,
        SyntaxExpression::Commented {
            comments,
            expression: _,
        } => comments.line0.start,
        SyntaxExpression::Origin {
            origin_keyword_start,
            name: _,
            result: _,
        } => *origin_keyword_start,
        SyntaxExpression::Query {
            colon_start,
            queried: _,
            cases: _,
        } => *colon_start,
    }
}
pub fn expression_end<Expressions, Patterns, Types>(
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    match expression {
        SyntaxExpression::Number { value, type_ } => type_
            .as_ref()
            .map(|type_| type_end(type_, types))
            .unwrap_or_else(|| position_add_characters(value.start, value.value.len() as u32)),
        SyntaxExpression::Char {
            open_quote_start: _,
            content: _,
            content_end,
            closed_quote_exists,
        } => {
            if *closed_quote_exists {
                symbol_end(*content_end, "'")
            } else {
                *content_end
            }
        }
        SyntaxExpression::Str {
            open_quote_start: _,
            content: _,
            content_end,
            closed_quote_exists,
        } => {
            if *closed_quote_exists {
                symbol_end(*content_end, "\"")
            } else {
                *content_end
            }
        }
        SyntaxExpression::VariableOrCall {
            name,
            type_arguments,
            argument,
        } => argument
            .as_ref()
            .map(|argument| {
                expression_end(expressions.element(argument), expressions, patterns, types)
            })
            .or_else(|| {
                type_arguments
                    .as_ref()
                    .map(|type_arguments| angled_type_arguments_end(type_arguments, types))
            })
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxExpression::Variant {
            name,
            type_arguments,
            value,
        } => value
            .as_ref()
            .map(|value| expression_end(expressions.element(value), expressions, patterns, types))
            .or_else(|| {
                type_arguments
                    .as_ref()
                    .map(|type_arguments| angled_type_arguments_end(type_arguments, types))
            })
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxExpression::Fn {
            fn_keyword_start,
            parameter,
            result,
        } => result
            .as_ref()
            .map(|result_slot| {
                expression_end(
                    expressions.element(result_slot),
                    expressions,
                    patterns,
                    types,
                )
            })
            .or_else(|| {
                parameter
                    .as_ref()
                    .map(|parameter| pattern_end(parameter, patterns, types))
            })
            .unwrap_or_else(|| symbol_end(*fn_keyword_start, "fn")),
        SyntaxExpression::Record {
            ampersand_start,
            fields,
        } => fields
            .last()
            .map(|last_field| {
                field_end(last_field, |value| {
                    expression_end(value, expressions, patterns, types)
                })
            })
            .unwrap_or_else(|| symbol_end(*ampersand_start, "&")),
        SyntaxExpression::Parenthesized {
            open_paren_start,
            inner,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| {
                inner.as_ref().map(|inner| {
                    expression_end(expressions.element(inner), expressions, patterns, types)
                })
            })
            .unwrap_or_else(|| symbol_end(*open_paren_start, "(")),
        SyntaxExpression::Commented {
            comments,
            expression,
        } => expression
            .as_ref()
            .map(|inner| expression_end(expressions.element(inner), expressions, patterns, types))
            .unwrap_or_else(|| comments_end(comments)),
        SyntaxExpression::Origin {
            origin_keyword_start,
            name,
            result,
        } => result
            .as_ref()
            .map(|result| expression_end(expressions.element(result), expressions, patterns, types))
            .or_else(|| {
                name.as_ref()
                    .map(|name| name_end(with_start_position_as_ref(name)))
            })
            .unwrap_or_else(|| symbol_end(*origin_keyword_start, "origin")),
        SyntaxExpression::Query {
            colon_start,
            queried,
            cases,
        } => cases
            .last()
            .map(|last_case| expression_case_end(last_case, expressions, patterns, types))
            .or_else(|| {
                queried.as_ref().map(|queried| {
                    expression_end(expressions.element(queried), expressions, patterns, types)
                })
            })
            .unwrap_or_else(|| symbol_end(*colon_start, ":")),
    }
}
fn comments_end(comments: &SyntaxComments) -> lsp_types::Position {
    let last_line = comments.line1_up.last().unwrap_or(&comments.line0);
    position_add_characters(
        last_line.start,
        last_line.value.encode_utf16().count() as u32,
    )
}
fn expression_case_end<Expressions, Patterns, Types>(
    case: &SyntaxExpressionQueryCase<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    case.result
        .as_ref()
        .map(|result| expression_end(result, expressions, patterns, types))
        .or_else(|| {
            case.left_angle_start
                .map(|left_angle_start| symbol_end(left_angle_start, "<"))
        })
        .unwrap_or_else(|| pattern_end(&case.pattern, patterns, types))
}

pub struct ParseState<'a> {
    source: &'a str,
    offset_utf8: usize,
    position: lsp_types::Position,
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
fn str_starts_with_linebreak(str: &str) -> bool {
    // \r allowed because both \r and \r\n are counted as linebreak
    // see EOL in https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocuments
    str.starts_with("\n") || str.starts_with("\r")
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
fn parse_symbol_as_start(state: &mut ParseState, symbol: &str) -> Option<lsp_types::Position> {
    let start_position: lsp_types::Position = state.position;
    if parse_symbol(state, symbol) {
        Some(start_position)
    } else {
        None
    }
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

fn parse_before_next_linebreak_or_end_as_str<'a>(state: &mut ParseState<'a>) -> &'a str {
    let content: &str = state.source[state.offset_utf8..]
        .lines()
        .next()
        .unwrap_or("");
    state.offset_utf8 += content.len();
    state.position.character += content.encode_utf16().count() as u32;
    content
}

/// a valid sloe symbol that must be followed by a character that could not be part of an sloe identifier
fn parse_sloe_keyword_as_start(
    state: &mut ParseState,
    symbol: &str,
) -> Option<lsp_types::Position> {
    if state.source[state.offset_utf8..].starts_with(symbol)
        && !(state.source[(state.offset_utf8 + symbol.len())..]
            .starts_with(|c: char| c.is_ascii_alphanumeric() || c == '-'))
    {
        let start_position: lsp_types::Position = state.position;
        state.offset_utf8 += symbol.len();
        state.position.character += symbol.len() as u32;
        Some(start_position)
    } else {
        None
    }
}
fn parse_sloe_whitespace(state: &mut ParseState) {
    while parse_linebreak(state) || parse_same_line_char_if(state, char::is_whitespace) {}
}
fn parse_sloe_comments(state: &mut ParseState) -> Option<SyntaxComments> {
    let Some(line0) = parse_sloe_comment(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut line1_up = Vec::new();
    while let Some(next_comment_line) = parse_sloe_comment(state) {
        line1_up.push(next_comment_line);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxComments {
        line0: line0,
        line1_up: line1_up,
    })
}
fn parse_sloe_comment(state: &mut ParseState) -> Option<WithStartPosition<Box<str>>> {
    let Some(hashtag_start) = parse_symbol_as_start(state, "#") else {
        return None;
    };
    Some(WithStartPosition {
        start: hashtag_start,
        value: Box::from(parse_before_next_linebreak_or_end_as_str(state)),
    })
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
fn parse_sloe_lowercase_name_with_start(state: &mut ParseState) -> Option<WithStartPosition<Name>> {
    let start_position: lsp_types::Position = state.position;
    if start_position.character == 0 {
        // disambiguate from project-level fn/choice keywords
        return None;
    }
    parse_sloe_lowercase_name(state).map(|name| WithStartPosition {
        start: start_position,
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

fn parse_sloe_uppercase_name_with_start(state: &mut ParseState) -> Option<WithStartPosition<Name>> {
    let start_position: lsp_types::Position = state.position;
    parse_sloe_uppercase_name(state).map(|name| WithStartPosition {
        start: start_position,
        value: name,
    })
}

pub fn parse_project<Expressions, Patterns, Types>(
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
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
    let mut last_parsed_valid_end_offset_utf8 = state.offset_utf8;
    let mut last_parsed_valid_end_position = state.position;
    'parsing_elements: loop {
        match parse_project_element(&mut state, expressions, patterns, types) {
            None => {
                if !parse_any_char(&mut state) {
                    if !last_parsed_was_valid {
                        elements.push(SyntaxProjectElement::Unrecognized {
                            range: lsp_types::Range {
                                start: last_parsed_valid_end_position,
                                end: state.position,
                            },
                            source: Box::from(&project_source[last_parsed_valid_end_offset_utf8..]),
                        });
                    }
                    break 'parsing_elements;
                }
                last_parsed_was_valid = false;
            }
            Some(element) => {
                if !last_parsed_was_valid {
                    elements.push(SyntaxProjectElement::Unrecognized {
                        range: lsp_types::Range {
                            start: last_parsed_valid_end_position,
                            end: state.position,
                        },
                        source: Box::from(&project_source[last_parsed_valid_end_offset_utf8..]),
                    });
                }
                elements.push(element);
                last_parsed_was_valid = true;
                parse_sloe_whitespace(&mut state);
                last_parsed_valid_end_offset_utf8 = state.offset_utf8;
                last_parsed_valid_end_position = state.position;
            }
        }
    }
    SyntaxProject { elements: elements }
}

fn parse_project_element<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxProjectElement<Expressions, Patterns, Types>> {
    parse_project_fn(state, expressions, patterns, types)
        .or_else(|| parse_choice(state, types))
        .or_else(|| parse_project_type(state, types))
        .or_else(|| parse_sloe_comments(state).map(SyntaxProjectElement::Comments))
}
fn parse_project_type<Expressions, Patterns, Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxProjectElement<Expressions, Patterns, Types>> {
    let Some(type_keyword_start) = parse_symbol_as_start(state, "type") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let mut parameters = Vec::new();
    while let Some(parameter) = parse_sloe_uppercase_name_with_start(state) {
        parameters.push(parameter);
        parse_sloe_whitespace(state);
    }
    let documentation = parse_sloe_comments(state);
    parse_sloe_whitespace(state);
    let type_ = parse_type(state, types);
    Some(SyntaxProjectElement::TypeAlias {
        type_keyword_start: type_keyword_start,
        name: name,
        parameters: parameters,
        documentation: documentation,
        type_: type_,
    })
}
fn parse_choice<Expressions, Patterns, Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxProjectElement<Expressions, Patterns, Types>> {
    let Some(choice_keyword_start) = parse_symbol_as_start(state, "choice") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let mut parameters = Vec::new();
    while let Some(parameter) = parse_sloe_uppercase_name_with_start(state) {
        parameters.push(parameter);
        parse_sloe_whitespace(state);
    }
    let documentation = parse_sloe_comments(state);
    parse_sloe_whitespace(state);
    let mut variants = Vec::new();
    while let Some(variant) = parse_sloe_variant(state, types) {
        variants.push(variant);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxProjectElement::ChoiceType {
        choice_keyword_start: choice_keyword_start,
        name: name,
        parameters: parameters,
        documentation: documentation,
        variants: variants,
    })
}
fn parse_sloe_variant<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxVariant<Types>> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_uppercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let type_parameters = parse_angled_type_parameters(state);
    parse_sloe_whitespace(state);
    let value = parse_type(state, types);
    parse_sloe_whitespace(state);
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxVariant {
        open_paren_start: open_paren_start,
        name: name,
        type_parameters: type_parameters,
        value: value,
        closed_paren_start: closed_paren_start,
    })
}
fn parse_angled_type_parameters(state: &mut ParseState) -> Option<SyntaxAngledTypeParameters> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    let mut parameters = Vec::new();
    while let Some(parameter) = parse_sloe_uppercase_name_with_start(state) {
        parameters.push(parameter);
        parse_sloe_whitespace(state);
    }
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeParameters {
        open_angle_start: open_angle_start,
        names: parameters,
        closed_angle_start: closed_angle_start,
    })
}
fn parse_project_fn<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxProjectElement<Expressions, Patterns, Types>> {
    let Some(fn_keyword_start) = parse_symbol_as_start(state, "fn") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let type_parameters = parse_angled_type_parameters(state);
    parse_sloe_whitespace(state);
    let parameter = parse_pattern_not_open_ended_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let result_type = parse_type_not_open_ended(state, types);
    parse_sloe_whitespace(state);
    let documentation = parse_sloe_comments(state);
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxProjectElement::Fn {
        fn_keyword_start: fn_keyword_start,
        name: name,
        type_parameters: type_parameters,
        parameter: parameter,
        result_type: result_type,
        documentation: documentation,
        result: result,
    })
}
pub fn parse_pattern_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    parse_pattern_variable_typed(state, types)
        .or_else(|| parse_pattern_variant_typed(state, patterns, types))
        .or_else(|| parse_pattern_record_typed(state, patterns, types))
        .or_else(|| parse_pattern_parenthesized_typed(state, patterns, types))
}
pub fn parse_pattern_not_open_ended_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    parse_pattern_variable_not_open_ended_typed(state, types)
        .or_else(|| parse_pattern_variant_not_open_ended_typed(state, patterns, types))
        .or_else(|| parse_pattern_record_empty(state))
        .or_else(|| parse_pattern_parenthesized_typed(state, patterns, types))
}
pub fn parse_pattern_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    parse_pattern_variable_untyped(state)
        .or_else(|| parse_pattern_variant_untyped(state, patterns, types))
        .or_else(|| parse_pattern_record_untyped(state, patterns, types))
        .or_else(|| parse_pattern_parenthesized_untyped(state, patterns, types))
}
pub fn parse_pattern_not_open_ended_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    parse_pattern_variable_untyped(state)
        .or_else(|| parse_pattern_variant_not_open_ended_untyped(state, patterns, types))
        .or_else(|| parse_pattern_record_empty(state))
        .or_else(|| parse_pattern_parenthesized_untyped(state, patterns, types))
}
fn parse_pattern_variable_typed<Patterns, Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_ = parse_type(state, types);
    Some(SyntaxPattern::Variable {
        name: name,
        type_: type_,
    })
}
fn parse_pattern_variable_not_open_ended_typed<Patterns, Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_ = parse_type_not_open_ended(state, types);
    Some(SyntaxPattern::Variable {
        name: name,
        type_: type_,
    })
}
fn parse_pattern_variable_untyped<Patterns, Types>(
    state: &mut ParseState,
) -> Option<SyntaxPattern<Patterns, Types>> {
    parse_sloe_lowercase_name_with_start(state).map(|name| SyntaxPattern::Variable {
        name: name,
        type_: None,
    })
}
fn parse_pattern_parenthesized_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let inner = parse_pattern_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxPattern::Parenthesized {
        open_paren_start: open_paren_start,
        inner: inner.map(|inner| patterns.add(inner)),
        closed_paren_start: closed_paren_start,
    })
}
fn parse_pattern_parenthesized_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let inner = parse_pattern_untyped(state, patterns, types);
    parse_sloe_whitespace(state);
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxPattern::Parenthesized {
        open_paren_start: open_paren_start,
        inner: inner.map(|inner| patterns.add(inner)),
        closed_paren_start: closed_paren_start,
    })
}
fn parse_pattern_variant_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(name) = parse_sloe_uppercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let value = parse_pattern_typed(state, patterns, types);
    Some(SyntaxPattern::Variant {
        name: name,
        type_arguments: type_arguments,
        value: value.map(|value| patterns.add(value)),
    })
}
fn parse_pattern_variant_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(name) = parse_sloe_uppercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let value = parse_pattern_untyped(state, patterns, types);
    Some(SyntaxPattern::Variant {
        name: name,
        type_arguments: type_arguments,
        value: value.map(|value| patterns.add(value)),
    })
}
fn parse_pattern_variant_not_open_ended_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(name) = parse_sloe_uppercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let value = parse_pattern_not_open_ended_typed(state, patterns, types);
    Some(SyntaxPattern::Variant {
        name: name,
        type_arguments: type_arguments,
        value: value.map(|value| patterns.add(value)),
    })
}
fn parse_pattern_variant_not_open_ended_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(name) = parse_sloe_uppercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let value = parse_pattern_not_open_ended_untyped(state, patterns, types);
    Some(SyntaxPattern::Variant {
        name: name,
        type_arguments: type_arguments,
        value: value.map(|value| patterns.add(value)),
    })
}
fn parse_type_arguments<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxAngledTypeArguments<Types>> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut argument_types = Vec::new();
    while let Some(argument_type) = parse_type(state, types) {
        argument_types.push(argument_type);
        parse_sloe_whitespace(state);
    }
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeArguments {
        open_angle_start: open_angle_start,
        types: types.add_iterator(argument_types.into_iter()).into_option(),
        closed_angle_start: closed_angle_start,
    })
}
fn parse_pattern_record_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(ampersand_start) = parse_symbol_as_start(state, "&") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_pattern_field_typed(state, patterns, types) {
        fields.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxPattern::Record {
        ampersand_start: ampersand_start,
        fields: fields,
    })
}
fn parse_pattern_field_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxField<SyntaxPattern<Patterns, Types>>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    match parse_symbol_as_start(state, "<") {
        None => {
            let value = parse_pattern_not_open_ended_typed(state, patterns, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: None,
                value: value,
            })
        }
        Some(left_angle_start) => {
            let value = parse_pattern_typed(state, patterns, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: Some(left_angle_start),
                value: value,
            })
        }
    }
}
fn parse_pattern_record_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(ampersand_start) = parse_symbol_as_start(state, "&") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_pattern_field_untyped(state, patterns, types) {
        fields.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxPattern::Record {
        ampersand_start: ampersand_start,
        fields: fields,
    })
}
fn parse_pattern_field_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxField<SyntaxPattern<Patterns, Types>>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    match parse_symbol_as_start(state, "<") {
        None => {
            let value = parse_pattern_not_open_ended_untyped(state, patterns, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: None,
                value: value,
            })
        }
        Some(left_angle_start) => {
            let value = parse_pattern_untyped(state, patterns, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: Some(left_angle_start),
                value: value,
            })
        }
    }
}
fn parse_pattern_record_empty<Patterns, Types>(
    state: &mut ParseState,
) -> Option<SyntaxPattern<Patterns, Types>> {
    parse_symbol_as_start(state, "&").map(|ampersand_start| SyntaxPattern::Record {
        ampersand_start: ampersand_start,
        fields: Vec::new(),
    })
}

pub fn parse_type<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    parse_type_variable(state)
        .or_else(|| parse_type_construct(state, types))
        .or_else(|| parse_type_record(state, types))
        .or_else(|| parse_type_parenthesized(state, types))
}
pub fn parse_type_not_open_ended<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    parse_type_variable(state)
        .or_else(|| parse_type_construct_without_arguments(state))
        .or_else(|| parse_type_record_empty(state))
        .or_else(|| parse_type_parenthesized(state, types))
}
fn parse_type_variable<Types>(state: &mut ParseState) -> Option<SyntaxType<Types>> {
    parse_sloe_uppercase_name_with_start(state).map(|name| SyntaxType::Variable(name))
}
fn parse_type_record_empty<Types>(state: &mut ParseState) -> Option<SyntaxType<Types>> {
    parse_symbol_as_start(state, "&").map(|ampersand_start| SyntaxType::Record {
        ampersand_start: ampersand_start,
        fields: Vec::new(),
    })
}
fn parse_type_parenthesized<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let inner = parse_type(state, types);
    parse_sloe_whitespace(state);
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxType::Parenthesized {
        open_paren_start: open_paren_start,
        inner: inner.map(|inner| types.add(inner)),
        closed_paren_start: closed_paren_start,
    })
}
fn parse_type_record<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    let Some(ampersand_start) = parse_symbol_as_start(state, "&") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_type_field(state, types) {
        fields.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxType::Record {
        ampersand_start: ampersand_start,
        fields: fields,
    })
}
fn parse_type_field<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxField<SyntaxType<Types>>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    match parse_symbol_as_start(state, "<") {
        None => {
            let value = parse_type_not_open_ended(state, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: None,
                value: value,
            })
        }
        Some(left_angle_start) => {
            let value = parse_type(state, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: Some(left_angle_start),
                value: value,
            })
        }
    }
}
fn parse_type_construct<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut arguments = Vec::new();
    while let Some(argument) = parse_type_not_open_ended(state, types) {
        arguments.push(argument);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxType::Construct {
        name: name,
        arguments: types.add_iterator(arguments.into_iter()).into_option(),
    })
}
fn parse_type_construct_without_arguments<Types>(
    state: &mut ParseState,
) -> Option<SyntaxType<Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxType::Construct {
        name: name,
        arguments: None,
    })
}
pub fn parse_expression<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    parse_expression_number(state, types)
        .or_else(|| parse_expression_char(state))
        .or_else(|| parse_expression_str(state))
        .or_else(|| parse_expression_fn(state, expressions, patterns, types))
        .or_else(|| parse_expression_origin(state, expressions, patterns, types))
        .or_else(|| parse_expression_variable_or_call(state, expressions, patterns, types))
        .or_else(|| parse_expression_variant(state, expressions, patterns, types))
        .or_else(|| parse_expression_parenthesized(state, expressions, patterns, types))
        .or_else(|| parse_expression_record(state, expressions, patterns, types))
        .or_else(|| parse_expression_query(state, expressions, patterns, types))
}
pub fn parse_expression_not_open_ended<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    parse_expression_number_not_open_ended(state, types)
        .or_else(|| parse_expression_char(state))
        .or_else(|| parse_expression_str(state))
        .or_else(|| parse_expression_variable(state))
        .or_else(|| parse_expression_variant_not_open_ended(state, expressions, patterns, types))
        .or_else(|| parse_expression_parenthesized(state, expressions, patterns, types))
        .or_else(|| parse_expression_record_empty(state))
        .or_else(|| parse_expression_fn_not_open_ended(state, expressions, patterns, types))
        .or_else(|| parse_expression_origin_not_open_ended(state, expressions, patterns, types))
}
fn parse_expression_record_empty<Expressions, Patterns, Types>(
    state: &mut ParseState,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    parse_symbol_as_start(state, "&").map(|ampersand_start| SyntaxExpression::Record {
        ampersand_start: ampersand_start,
        fields: Vec::new(),
    })
}
fn parse_expression_record<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(ampersand_start) = parse_symbol_as_start(state, "&") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_expression_field(state, expressions, patterns, types) {
        fields.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxExpression::Record {
        ampersand_start: ampersand_start,
        fields: fields,
    })
}
fn parse_expression_field<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxField<SyntaxExpression<Expressions, Patterns, Types>>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    match parse_symbol_as_start(state, "<") {
        None => {
            let value = parse_expression_not_open_ended(state, expressions, patterns, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: None,
                value: value,
            })
        }
        Some(left_angle_start) => {
            let value = parse_expression(state, expressions, patterns, types);
            Some(SyntaxField {
                name: name,
                left_angle_start: Some(left_angle_start),
                value: value,
            })
        }
    }
}
fn parse_expression_number<Expressions, Patterns, Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let start = state.position;
    let Some(value) = parse_number(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_ = parse_type(state, types);
    Some(SyntaxExpression::Number {
        value: WithStartPosition {
            value: Box::from(value),
            start: start,
        },
        type_: type_,
    })
}
fn parse_expression_number_not_open_ended<Expressions, Patterns, Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let start = state.position;
    let Some(value) = parse_number(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_ = parse_type_not_open_ended(state, types);
    Some(SyntaxExpression::Number {
        value: WithStartPosition {
            value: Box::from(value),
            start: start,
        },
        type_: type_,
    })
}
fn parse_expression_char<Expressions, Patterns, Types>(
    state: &mut ParseState,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    parse_char(state).map(|str| SyntaxExpression::Char {
        open_quote_start: str.open_quote_start,
        content: str.content,
        content_end: str.content_end,
        closed_quote_exists: str.closed_quote_exists,
    })
}
fn parse_expression_str<Expressions, Patterns, Types>(
    state: &mut ParseState,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    parse_str(state).map(|str| SyntaxExpression::Str {
        open_quote_start: str.open_quote_start,
        content: str.content.into_boxed_str(),
        content_end: str.content_end,
        closed_quote_exists: str.closed_quote_exists,
    })
}
fn parse_number<'a>(state: &mut ParseState<'a>) -> Option<&'a str> {
    let start_offset_utf8: usize = state.offset_utf8;
    if parse_symbol(state, "-") || parse_symbol(state, "+") {
        let _: bool = parse_unsigned_integer_base10(state);
        true
    } else if parse_unsigned_integer_base10(state) {
        false
    } else {
        return None;
    };
    let has_decimal_point: bool = parse_symbol(state, ".");
    if has_decimal_point {
        parse_same_line_while(state, |c| c.is_ascii_digit());
    }
    Some(&state.source[start_offset_utf8..state.offset_utf8])
}
struct SyntaxChar {
    open_quote_start: lsp_types::Position,
    content: Option<char>,
    content_end: lsp_types::Position,
    closed_quote_exists: bool,
}
fn parse_char(state: &mut ParseState) -> Option<SyntaxChar> {
    let Some(open_quote_start) = parse_symbol_as_start(state, "'") else {
        return None;
    };
    let content = parse_text_content_char(state);
    match parse_symbol_as_start(state, "'") {
        Some(closed_quote_start) => Some(SyntaxChar {
            open_quote_start: open_quote_start,
            content: content,
            content_end: closed_quote_start,
            closed_quote_exists: true,
        }),
        None => Some(SyntaxChar {
            open_quote_start: open_quote_start,
            content: content,
            content_end: state.position,
            closed_quote_exists: false,
        }),
    }
}
struct SyntaxStr {
    open_quote_start: lsp_types::Position,
    content: String,
    content_end: lsp_types::Position,
    closed_quote_exists: bool,
}
fn parse_str(state: &mut ParseState) -> Option<SyntaxStr> {
    let Some(open_quote_start) = parse_symbol_as_start(state, "\"") else {
        return None;
    };
    let mut content: String = String::new();
    'parsing_content: loop {
        if let Some(closed_quote_start) = parse_symbol_as_start(state, "\"") {
            return Some(SyntaxStr {
                open_quote_start: open_quote_start,
                content: content,
                content_end: closed_quote_start,
                closed_quote_exists: true,
            });
        }
        match parse_text_content_char(state) {
            Some(next_content_char) => {
                content.push(next_content_char);
            }
            None => break 'parsing_content,
        }
    }
    Some(SyntaxStr {
        open_quote_start: open_quote_start,
        content: content,
        content_end: state.position,
        closed_quote_exists: false,
    })
}
fn parse_text_content_char(state: &mut ParseState) -> Option<char> {
    parse_symbol_as(state, "\\\\", '\\')
        .or_else(|| parse_symbol_as(state, "\\'", '\''))
        .or_else(|| parse_symbol_as(state, "\\n", '\n'))
        .or_else(|| parse_symbol_as(state, "\\r", '\r'))
        .or_else(|| parse_symbol_as(state, "\\t", '\t'))
        .or_else(|| parse_symbol_as(state, "\\\"", '"'))
        .or_else(|| {
            let start_offset_utf8: usize = state.offset_utf8;
            let start_position: lsp_types::Position = state.position;
            let reset_parse_state = |progressed_state: &mut ParseState| {
                progressed_state.offset_utf8 = start_offset_utf8;
                progressed_state.position = start_position;
            };
            if !parse_symbol(state, "\\u{") {
                return None;
            }
            let unicode_hex_start_offset_utf8: usize = state.offset_utf8;
            parse_same_line_while(state, |c| c.is_ascii_hexdigit());
            let unicode_hex_str: &str =
                &state.source[unicode_hex_start_offset_utf8..state.offset_utf8];
            let _: bool = parse_symbol(state, "}");
            let Ok(code_point) = u32::from_str_radix(unicode_hex_str, 16) else {
                reset_parse_state(state);
                return None;
            };
            match char::from_u32(code_point) {
                Some(char) => Some(char),
                None => {
                    reset_parse_state(state);
                    return None;
                }
            }
        })
        .or_else(|| {
            if str_starts_with_linebreak(&state.source[state.offset_utf8..]) {
                None
            } else {
                match state.source[state.offset_utf8..].chars().next() {
                    None => None,
                    Some(plain_char) => {
                        state.offset_utf8 += plain_char.len_utf8();
                        state.position.character += plain_char.len_utf16() as u32;
                        Some(plain_char)
                    }
                }
            }
        })
}
fn parse_expression_parenthesized<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let inner = parse_expression(state, expressions, patterns, types);
    parse_sloe_whitespace(state);
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxExpression::Parenthesized {
        open_paren_start: open_paren_start,
        inner: inner.map(|inner| expressions.add(inner)),
        closed_paren_start: closed_paren_start,
    })
}
fn parse_expression_variable_or_call<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let argument = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::VariableOrCall {
        name: name,
        type_arguments: type_arguments,
        argument: argument.map(|argument| expressions.add(argument)),
    })
}
fn parse_expression_variable<Expressions, Patterns, Types>(
    state: &mut ParseState,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxExpression::VariableOrCall {
        name: name,
        type_arguments: None,
        argument: None,
    })
}
fn parse_expression_variant<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(name) = parse_sloe_uppercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let value = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Variant {
        name: name,
        type_arguments: type_arguments,
        value: value.map(|argument| expressions.add(argument)),
    })
}
fn parse_expression_variant_not_open_ended<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(name) = parse_sloe_uppercase_name_with_start(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let value = parse_expression_not_open_ended(state, expressions, patterns, types);
    Some(SyntaxExpression::Variant {
        name: name,
        type_arguments: type_arguments,
        value: value.map(|argument| expressions.add(argument)),
    })
}
fn parse_expression_fn<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(fn_keyword_start) = parse_sloe_keyword_as_start(state, "fn") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let parameter = parse_pattern_not_open_ended_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Fn {
        fn_keyword_start: fn_keyword_start,
        parameter: parameter,
        result: result.map(|result| expressions.add(result)),
    })
}
fn parse_expression_fn_not_open_ended<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(fn_keyword_start) = parse_sloe_keyword_as_start(state, "fn") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let parameter = parse_pattern_not_open_ended_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let result = parse_expression_not_open_ended(state, expressions, patterns, types);
    Some(SyntaxExpression::Fn {
        fn_keyword_start: fn_keyword_start,
        parameter: parameter,
        result: result.map(|result| expressions.add(result)),
    })
}
fn parse_expression_origin<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(origin_keyword_start) = parse_sloe_keyword_as_start(state, "origin") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Origin {
        origin_keyword_start: origin_keyword_start,
        name: name,
        result: result.map(|result| expressions.add(result)),
    })
}
fn parse_expression_origin_not_open_ended<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(origin_keyword_start) = parse_sloe_keyword_as_start(state, "origin") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let result = parse_expression_not_open_ended(state, expressions, patterns, types);
    Some(SyntaxExpression::Origin {
        origin_keyword_start: origin_keyword_start,
        name: name,
        result: result.map(|result| expressions.add(result)),
    })
}
fn parse_expression_query<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(colon_start) = parse_symbol_as_start(state, ":") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let queried = parse_expression_not_open_ended(state, expressions, patterns, types);
    parse_sloe_whitespace(state);
    let mut cases = Vec::new();
    while let Some(case) = parse_expression_query_case(state, expressions, patterns, types) {
        cases.push(case);
        parse_sloe_whitespace(state);
    }
    parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Query {
        colon_start: colon_start,
        queried: queried.map(|queried| expressions.add(queried)),
        cases: cases,
    })
}
fn parse_expression_query_case<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpressionQueryCase<Expressions, Patterns, Types>> {
    let Some(pattern) = parse_pattern_not_open_ended_untyped(state, patterns, types) else {
        return None;
    };
    parse_sloe_whitespace(state);
    match parse_symbol_as_start(state, "<") {
        None => {
            let result = parse_expression_not_open_ended(state, expressions, patterns, types);
            Some(SyntaxExpressionQueryCase {
                pattern: pattern,
                result: result,
                left_angle_start: None,
            })
        }
        Some(left_angle_start) => {
            parse_sloe_whitespace(state);
            let result = parse_expression(state, expressions, patterns, types);
            Some(SyntaxExpressionQueryCase {
                pattern: pattern,
                result: result,
                left_angle_start: Some(left_angle_start),
            })
        }
    }
}

pub struct CompiledProject {
    pub rust: syn::File,
    pub type_aliases: std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    pub choice_types: std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    pub fns: std::collections::HashMap<Name, CompiledProjectFnInfo>,
    pub records: std::collections::HashSet<Vec<Name>>,
}
#[derive(Clone, Debug)]
pub struct CompiledChoiceTypeInfo {
    pub name_range: Option<lsp_types::Range>,
    pub parameters: Vec<Name>,
    pub documentation: Option<Box<str>>,
    pub variants: Vec<CompiledVariantInfo>,
    pub is_copy: bool,
}
#[derive(Clone, Debug)]
pub struct CompiledVariantInfo {
    pub name: Name,
    pub type_parameters: Vec<Name>,
    pub value: Type,
}
#[derive(Debug)]
pub struct CompiledTypeAliasInfo {
    pub name_range: Option<lsp_types::Range>,
    pub parameters: Vec<Name>,
    pub documentation: Option<Box<str>>,
    pub type_: Option<Type>,
    pub is_copy: bool,
}
#[derive(Clone, Debug)]
pub enum Type {
    Variable(Name),
    Record(Vec<TypeField>),
    // can also be an origin
    ChoiceConstruct { name: Name, arguments: Vec<Type> },
}
#[derive(Clone, Debug)]
pub struct TypeField {
    pub name: Name,
    pub value: Type,
}
#[derive(Clone, Debug)]
pub struct CompiledProjectFnInfo {
    pub documentation: Option<Box<str>>,
    pub type_parameters: Vec<Name>,
    pub parameter_type: Option<Type>,
    pub result_type: Option<Type>,
}

fn type_is_copy(
    variables_are_copy: bool,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    choice_types: &std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    type_: &Type,
) -> bool {
    match type_ {
        Type::Variable(_) => variables_are_copy,
        Type::Record(fields) => fields.iter().all(|field| {
            type_is_copy(variables_are_copy, type_aliases, choice_types, &field.value)
        }),
        Type::ChoiceConstruct { name, arguments } => {
            choice_types
                .get(name)
                .is_some_and(|origin_choice_type| origin_choice_type.is_copy)
                || arguments.iter().all(|argument| {
                    type_is_copy(variables_are_copy, type_aliases, choice_types, argument)
                })
        }
    }
}

pub fn project_compile_to_rust<Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    syntax_project: &SyntaxProject<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> CompiledProject {
    let mut type_graph: strongly_connected_components::Graph =
        strongly_connected_components::Graph::new();
    let mut type_graph_node_by_name: std::collections::HashMap<
        &str,
        strongly_connected_components::Node,
    > = std::collections::HashMap::new();
    let mut type_declaration_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectTypeInfo<Types>,
    > = std::collections::HashMap::new();

    let mut variable_graph: strongly_connected_components::Graph =
        strongly_connected_components::Graph::new();
    let mut variable_graph_node_by_name: std::collections::HashMap<
        &str,
        strongly_connected_components::Node,
    > = std::collections::HashMap::with_capacity(syntax_project.elements.len());
    let mut project_fn_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectFnInfo<Expressions, Patterns, Types>,
    > = std::collections::HashMap::with_capacity(syntax_project.elements.len());

    for project_element in &syntax_project.elements {
        match project_element {
            SyntaxProjectElement::Comments(_) => {}
            SyntaxProjectElement::Unrecognized {
                range: unknown_range,
                source: unknown_source,
            } => {
                errors.push(ErrorNode {
                    range: *unknown_range,
                    message: format!("unrecognized syntax. {}
If you wanted to start a declaration, try one of:
  - fn some-fn-name (some-parameter some-parameter-type) (some result-type) (some value)
  - type some-type-name (some type)
  - choice some-choice-type-name (First-variant &) (Second-variant some-type)",
                    if unknown_source
                        .starts_with(|c: char| c.is_ascii_lowercase())
                    {
                        "It could be that a name starting with an uppercase letter is expected here (variant and type variable names start uppercase). Also, is it indented correctly?"
                    } else if unknown_source
                        .starts_with(|c: char| c.is_ascii_uppercase())
                    {
                        "It could be that a name starting with a lowercase letter is expected here (only variant and type variable names start uppercase). Also, is it indented correctly?"
                    } else if unknown_source
                        .starts_with('#')
                    {
                        "Comments can only be put in front of expressions, patterns, types, or after the header of a declaration. Is it indented correctly?"
                    } else if unknown_source.starts_with("//")
                        || unknown_source.starts_with("--")
                    {
                        "Comments start with #"
                    } else   if unknown_source
                        .starts_with('.')
                    {
                        "Record access is not a feature in sloe. Instead, use pattern matching, like value :(your-value) ((& (field variable)) result). Otherwise, is everything indented correctly?"
                    } else if unknown_source
                        .starts_with(['+', '-', '*', '/'])
                    {
                        "Operator application are not a feature in sloe. Instead, use regular function calls like dec-add, int-negate or unt-mul. Otherwise, is everything indented correctly?"
                    } else {
                        "Is it indented correctly? Are brackets/braces/parens/quotes or similar closed prematurely or too often?"
                    }).into_boxed_str(),
                });
            }
            SyntaxProjectElement::ChoiceType {
                choice_keyword_start,
                name: maybe_name,
                parameters,
                documentation,
                variants,
            } => match maybe_name {
                None => {
                    errors.push(ErrorNode { range: symbol_range(*choice_keyword_start, "choice"), message: Box::from("missing type name after choice. Type names start with a lowercase letter any only use ascii alphanumeric characters and -") });
                }
                Some(name) => {
                    let choice_type_declaration_graph_node: strongly_connected_components::Node =
                        type_graph.new_node();
                    let existing_type_with_same_name: Option<strongly_connected_components::Node> =
                        type_graph_node_by_name
                            .insert(&name.value, choice_type_declaration_graph_node);
                    type_declaration_by_graph_node.insert(
                        choice_type_declaration_graph_node,
                        SyntaxProjectTypeInfo::ChoiceType {
                            documentation: &documentation,
                            name: &name,
                            parameters: &parameters,
                            variants: &variants,
                        },
                    );
                    if existing_type_with_same_name.is_some() {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: Box::from(
                                "a type with this name is already declared. Rename one of them",
                            ),
                        });
                    } else if core_choice_types.contains_key(name.value.as_str()) {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: Box::from("a type with this name is already part of core (core types are for example vec, int, str). Rename this type")
                        });
                    }
                }
            },
            SyntaxProjectElement::TypeAlias {
                type_keyword_start,
                name: maybe_name,
                parameters,
                documentation,
                type_,
            } => match maybe_name {
                None => {
                    errors.push(ErrorNode { range: symbol_range(*type_keyword_start, "type"), message: Box::from("missing name. Type names start with a lowercase letter any only use ascii alphanumeric characters and -") });
                }
                Some(name_node) => {
                    let type_alias_declaration_graph_node: strongly_connected_components::Node =
                        type_graph.new_node();
                    let existing_type_with_same_name: Option<strongly_connected_components::Node> =
                        type_graph_node_by_name
                            .insert(&name_node.value, type_alias_declaration_graph_node);
                    type_declaration_by_graph_node.insert(
                        type_alias_declaration_graph_node,
                        SyntaxProjectTypeInfo::TypeAlias {
                            documentation: &documentation,
                            name: &name_node,
                            parameters: &parameters,
                            type_: &type_,
                        },
                    );
                    if existing_type_with_same_name.is_some() {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name_node)),
                            message: Box::from(
                                "a type with this name is already declared. Rename one of them",
                            ),
                        });
                    }
                }
            },
            SyntaxProjectElement::Fn {
                fn_keyword_start,
                name: maybe_name,
                type_parameters,
                parameter,
                result_type,
                documentation,
                result: maybe_result,
            } => match maybe_name {
                None => {
                    errors.push(ErrorNode { range: symbol_range(*fn_keyword_start, "fn"), message: Box::from("missing name. Function names start with a lowercase letter any only use ascii alphanumeric characters and -") });
                }
                Some(name) => {
                    let project_fn_graph_node: strongly_connected_components::Node =
                        variable_graph.new_node();
                    let existing_variable_with_same_name: Option<
                        strongly_connected_components::Node,
                    > = variable_graph_node_by_name.insert(&name.value, project_fn_graph_node);
                    project_fn_by_graph_node.insert(
                        project_fn_graph_node,
                        SyntaxProjectFnInfo {
                            range: lsp_types::Range {
                                start: *fn_keyword_start,
                                end: maybe_result
                                    .as_ref()
                                    .map(|result| {
                                        expression_end(result, expressions, patterns, types)
                                    })
                                    .or_else(|| documentation.as_ref().map(comments_end))
                                    .or_else(|| {
                                        result_type
                                            .as_ref()
                                            .map(|result_type| type_end(result_type, types))
                                    })
                                    .or_else(|| {
                                        parameter.as_ref().map(|parameter| {
                                            pattern_end(parameter, patterns, types)
                                        })
                                    })
                                    .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
                            },
                            name: name,
                            type_parameters: type_parameters,
                            parameter: parameter,
                            result_type: result_type,
                            documentation: documentation,
                            result: maybe_result,
                        },
                    );
                    if existing_variable_with_same_name.is_some() {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: Box::from(
                                "a variable with this name is already declared. Rename one of them",
                            ),
                        });
                    } else if core_fns.contains_key(name.value.as_str()) {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: Box::from("a variable with this name is already part of core (core variables are for example int-to-str or dec-add). Rename this variable")
                        });
                    }
                }
            },
        }
    }
    for (&type_declaration_graph_node, &type_declaration_info) in
        type_declaration_by_graph_node.iter()
    {
        syntax_proect_type_connect_type_names_in_graph_from(
            &mut type_graph,
            type_declaration_graph_node,
            &type_graph_node_by_name,
            types,
            type_declaration_info,
        );
    }
    for (&project_fn_graph_node, project_fn_info) in project_fn_by_graph_node.iter() {
        if let Some(result_node) = project_fn_info.result {
            syntax_expression_connect_variables_in_graph_from(
                &mut variable_graph,
                project_fn_graph_node,
                &variable_graph_node_by_name,
                expressions,
                result_node,
            );
        }
    }
    project_info_to_rust(
        errors,
        &type_graph,
        &type_declaration_by_graph_node,
        variable_graph,
        project_fn_by_graph_node,
        expressions,
        patterns,
        types,
    )
}
fn syntax_proect_type_connect_type_names_in_graph_from<Types>(
    type_graph: &mut strongly_connected_components::Graph,
    origin_project_type_graph_node: strongly_connected_components::Node,
    type_graph_node_by_name: &std::collections::HashMap<&str, strongly_connected_components::Node>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    type_declaration_info: SyntaxProjectTypeInfo<Types>,
) {
    match type_declaration_info {
        SyntaxProjectTypeInfo::ChoiceType {
            documentation: _,
            name: _,
            parameters: _,
            variants,
        } => {
            for variant_value in variants.iter().filter_map(|variant| variant.value.as_ref()) {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph,
                    origin_project_type_graph_node,
                    type_graph_node_by_name,
                    types,
                    variant_value,
                );
            }
        }
        SyntaxProjectTypeInfo::TypeAlias {
            documentation: _,
            name: _,
            parameters: _,
            type_: maybe_type,
        } => {
            if let Some(aliased_type) = maybe_type {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph,
                    origin_project_type_graph_node,
                    type_graph_node_by_name,
                    types,
                    aliased_type,
                );
            }
        }
    }
}
fn syntax_type_connect_type_names_in_graph_from<Types>(
    type_graph: &mut strongly_connected_components::Graph,
    origin_type_declaration_graph_node: strongly_connected_components::Node,
    type_graph_node_by_name: &std::collections::HashMap<&str, strongly_connected_components::Node>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    type_: &SyntaxType<Types>,
) {
    match type_ {
        SyntaxType::Variable(_) => {}
        SyntaxType::Record {
            ampersand_start: _,
            fields,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_type_connect_type_names_in_graph_from(
                        type_graph,
                        origin_type_declaration_graph_node,
                        type_graph_node_by_name,
                        types,
                        value,
                    )
                }
            }
        }
        SyntaxType::Construct { name, arguments } => {
            if let Some(referenced_type_graph_node) =
                type_graph_node_by_name.get(name.value.as_str()).copied()
            {
                type_graph.new_edge(
                    origin_type_declaration_graph_node,
                    referenced_type_graph_node,
                );
            }
            for argument in types.opt_span_slice(core::Opt::from_option(arguments.as_ref())) {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph,
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    argument,
                )
            }
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph,
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    types.element(inner),
                )
            }
        }
    }
}
// TODO(important) track pattern_variables and origins to avoid accidental misconnection
fn syntax_expression_connect_variables_in_graph_from<Expressions, Patterns, Types>(
    project_fn_graph: &mut strongly_connected_components::Graph,
    origin_project_fn_graph_node: strongly_connected_components::Node,
    project_fn_graph_node_by_name: &std::collections::HashMap<
        &str,
        strongly_connected_components::Node,
    >,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {}
        SyntaxExpression::Char { .. } => {}
        SyntaxExpression::Str { .. } => {}
        SyntaxExpression::VariableOrCall {
            name,
            type_arguments: _,
            argument,
        } => {
            if let Some(referenced_fn_graph_node) = project_fn_graph_node_by_name
                .get(name.value.as_str())
                .copied()
            {
                project_fn_graph.new_edge(origin_project_fn_graph_node, referenced_fn_graph_node);
            }
            if let Some(argument) = argument {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(argument),
                );
            }
        }
        SyntaxExpression::Variant {
            name: _,
            type_arguments: _,
            value,
        } => {
            if let Some(value) = value {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(value),
                );
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            parameter: _,
            result,
        } => {
            if let Some(result) = result {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(result),
                );
            }
        }
        SyntaxExpression::Record {
            ampersand_start: _,
            fields,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_expression_connect_variables_in_graph_from(
                        project_fn_graph,
                        origin_project_fn_graph_node,
                        project_fn_graph_node_by_name,
                        expressions,
                        value,
                    );
                }
            }
        }
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(inner),
                );
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(expression) = expression {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(expression),
                );
            }
        }
        SyntaxExpression::Query {
            colon_start: _,
            queried,
            cases,
        } => {
            if let Some(queried) = queried {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(queried),
                );
            }
            for case in cases {
                if let Some(result) = &case.result {
                    syntax_expression_connect_variables_in_graph_from(
                        project_fn_graph,
                        origin_project_fn_graph_node,
                        project_fn_graph_node_by_name,
                        expressions,
                        result,
                    );
                }
            }
        }
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name: _,
            result,
        } => {
            if let Some(result) = result {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(result),
                );
            }
        }
    }
}
#[derive(Debug)]
struct SyntaxProjectFnInfo<'a, Expressions, Patterns, Types> {
    range: lsp_types::Range,
    name: &'a WithStartPosition<Name>,
    type_parameters: &'a Option<SyntaxAngledTypeParameters>,
    parameter: &'a Option<SyntaxPattern<Patterns, Types>>,
    result_type: &'a Option<SyntaxType<Types>>,
    documentation: &'a Option<SyntaxComments>,
    result: &'a Option<SyntaxExpression<Expressions, Patterns, Types>>,
}
#[derive(Debug)]
enum SyntaxProjectTypeInfo<'a, Types> {
    // consider introducing separate structs instead of separately referencing each field
    ChoiceType {
        name: &'a WithStartPosition<Name>,
        parameters: &'a Vec<WithStartPosition<Name>>,
        documentation: &'a Option<SyntaxComments>,
        variants: &'a Vec<SyntaxVariant<Types>>,
    },
    TypeAlias {
        name: &'a WithStartPosition<Name>,
        documentation: &'a Option<SyntaxComments>,
        parameters: &'a Vec<WithStartPosition<Name>>,
        type_: &'a Option<SyntaxType<Types>>,
    },
}
// Copy & Clone need to be manually implemented because derive(Clone) introduces unnecessary Expressions/Patterns/Types:Clone bounds
impl<'a, Expressions, Patterns, Types> Copy
    for SyntaxProjectFnInfo<'a, Expressions, Patterns, Types>
{
}
impl<'a, Expressions, Patterns, Types> Clone
    for SyntaxProjectFnInfo<'a, Expressions, Patterns, Types>
{
    fn clone(&self) -> Self {
        Self {
            range: self.range,
            name: self.name,
            type_parameters: self.type_parameters,
            parameter: self.parameter,
            result_type: self.result_type,
            documentation: self.documentation,
            result: self.result,
        }
    }
}
impl<'a, Types> Copy for SyntaxProjectTypeInfo<'a, Types> {}
impl<'a, Types> Clone for SyntaxProjectTypeInfo<'a, Types> {
    fn clone(&self) -> Self {
        match self {
            Self::ChoiceType {
                documentation,
                name,
                parameters,
                variants,
            } => Self::ChoiceType {
                documentation: documentation,
                name: name,
                parameters: parameters,
                variants: variants,
            },
            Self::TypeAlias {
                documentation,
                name,
                parameters,
                type_,
            } => Self::TypeAlias {
                documentation: documentation,
                name: name,
                parameters: parameters,
                type_: type_,
            },
        }
    }
}

fn project_info_to_rust<Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    type_graph: &strongly_connected_components::Graph,
    project_type_by_graph_node: &std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectTypeInfo<Types>,
    >,
    project_fn_graph: strongly_connected_components::Graph,
    project_fn_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectFnInfo<Expressions, Patterns, Types>,
    >,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> CompiledProject {
    let mut rust_items: Vec<syn::Item> =
        Vec::with_capacity(type_graph.len() * 3 + project_fn_graph.len());
    let mut compiled_type_aliases: std::collections::HashMap<Name, CompiledTypeAliasInfo> =
        std::collections::HashMap::with_capacity(project_type_by_graph_node.len());
    let mut compiled_choice_types: std::collections::HashMap<Name, CompiledChoiceTypeInfo> =
        core_choice_types.clone();
    compiled_choice_types.reserve(project_type_by_graph_node.len());
    let mut records_used: std::collections::HashSet<Vec<Name>> =
        std::collections::HashSet::with_capacity(8);
    for project_type_strongly_connected_component in type_graph.find_sccs().iter_sccs() {
        // TODO report and skip (mutually) recursive project types. Currently these are reported as "not found" at best
        for project_type in project_type_strongly_connected_component
            .iter_nodes()
            .filter_map(|variable_declaration_graph_node| {
                project_type_by_graph_node.get(&variable_declaration_graph_node)
            })
            .copied()
        {
            match project_type {
                SyntaxProjectTypeInfo::TypeAlias {
                    documentation: maybe_documentation,
                    name,
                    parameters,
                    type_: maybe_type,
                } => {
                    let maybe_compiled_type_alias: Option<CompiledTypeAlias> =
                        type_alias_declaration_to_rust(
                            errors,
                            &mut records_used,
                            &compiled_type_aliases,
                            &compiled_choice_types,
                            types,
                            maybe_documentation.as_ref(),
                            name,
                            parameters,
                            maybe_type.as_ref(),
                        );
                    let documentation = maybe_documentation.as_ref().map(|documentation| {
                        documentation
                            .line1_up
                            .iter()
                            .fold(documentation.line0.value.to_string(), |so_far, line| {
                                so_far + "\n" + &line.value
                            })
                            .into_boxed_str()
                    });
                    let parameters = parameters
                        .iter()
                        .map(|parameter| parameter.value.clone())
                        .collect();
                    match maybe_compiled_type_alias {
                        Some(compiled_type_alias) => {
                            rust_items.push(compiled_type_alias.rust);
                            compiled_type_aliases.insert(
                                name.value.clone(),
                                CompiledTypeAliasInfo {
                                    name_range: Some(syntax_name_range(
                                        with_start_position_as_ref(name),
                                    )),
                                    documentation: documentation,
                                    parameters: parameters,
                                    type_: Some(compiled_type_alias.type_),
                                    is_copy: compiled_type_alias.is_copy,
                                },
                            );
                        }
                        None => {
                            compiled_type_aliases.insert(
                                name.value.clone(),
                                CompiledTypeAliasInfo {
                                    name_range: Some(syntax_name_range(
                                        with_start_position_as_ref(name),
                                    )),
                                    documentation: documentation,
                                    parameters: parameters,
                                    type_: None,
                                    // dummy values that should not be read in practice
                                    is_copy: false,
                                },
                            );
                        }
                    }
                }
                SyntaxProjectTypeInfo::ChoiceType {
                    documentation: maybe_documentation,
                    name,
                    parameters,
                    variants,
                } => {
                    let maybe_compiled_choice_type_info: Option<CompiledChoiceType> =
                        choice_type_declaration_to_rust_into(
                            &mut rust_items,
                            errors,
                            &mut records_used,
                            &compiled_type_aliases,
                            &compiled_choice_types,
                            types,
                            maybe_documentation.as_ref(),
                            name,
                            parameters,
                            variants,
                        );
                    let documentation = maybe_documentation.as_ref().map(|documentation| {
                        documentation
                            .line1_up
                            .iter()
                            .fold(documentation.line0.value.to_string(), |so_far, line| {
                                so_far + "\n" + &line.value
                            })
                            .into_boxed_str()
                    });
                    let parameters = parameters
                        .iter()
                        .map(|parameter| parameter.value.clone())
                        .collect();
                    let info = match maybe_compiled_choice_type_info {
                        Some(compiled_choice_type_info) => CompiledChoiceTypeInfo {
                            name_range: Some(syntax_name_range(with_start_position_as_ref(name))),
                            documentation: documentation,
                            parameters: parameters,
                            variants: compiled_choice_type_info.variants,
                            is_copy: compiled_choice_type_info.is_copy,
                        },
                        None => CompiledChoiceTypeInfo {
                            name_range: Some(syntax_name_range(with_start_position_as_ref(name))),
                            documentation: documentation,
                            parameters: parameters,
                            // dummy
                            variants: vec![],
                            is_copy: false,
                        },
                    };
                    compiled_choice_types.insert(name.value.clone(), info);
                }
            }
        }
    }
    let mut compiledproject_fns: std::collections::HashMap<Name, CompiledProjectFnInfo> =
        core_fns.clone();
    compiledproject_fns.reserve(project_fn_graph.len());
    for project_fn_strongly_connected_component in project_fn_graph.find_sccs().iter_sccs() {
        let project_fns_in_strongly_connected_component: Vec<
            SyntaxProjectFnInfo<Expressions, Patterns, Types>,
        > = project_fn_strongly_connected_component
            .iter_nodes()
            .filter_map(|project_fn_graph_node| {
                project_fn_by_graph_node.get(&project_fn_graph_node)
            })
            .copied()
            .collect();
        // optimization: skip pre-compile-type-info computation when project_fns_in_strongly_connected_component is single, non-self-referencing node
        for project_fn in &project_fns_in_strongly_connected_component {
            let type_parameters = match &project_fn.type_parameters {
                None => vec![],
                Some(type_parameters) => type_parameters
                    .names
                    .iter()
                    .map(|name| name.value.clone())
                    .collect(),
            };
            // TODO instead populate actual errors etc and use the compiled pattern in expression_to_rust?
            let maybe_parameter_type = project_fn.parameter.as_ref().and_then(|parameter| {
                syntax_pattern_to_rust(
                    parameter,
                    None,
                    &mut Vec::new(),
                    &mut std::collections::HashSet::new(),
                    &mut std::collections::HashMap::new(),
                    &compiled_type_aliases,
                    &compiled_choice_types,
                    patterns,
                    types,
                    &std::collections::HashMap::new(),
                )
                .map(|compiled_parameter| compiled_parameter.type_)
            });
            match project_fn.result_type {
                None => {
                    compiledproject_fns.insert(
                        project_fn.name.value.clone(),
                        CompiledProjectFnInfo {
                            documentation: None,
                            type_parameters: type_parameters,
                            parameter_type: maybe_parameter_type,
                            result_type: None,
                        },
                    );
                }
                Some(syntax_result_type) => {
                    let result_type: Option<Type> = syntax_type_to_type(
                        syntax_result_type,
                        &mut Vec::new(),
                        &compiled_type_aliases,
                        &compiled_choice_types,
                        types,
                        &std::collections::HashMap::new(),
                    );
                    compiledproject_fns.insert(
                        project_fn.name.value.clone(),
                        CompiledProjectFnInfo {
                            documentation: None,
                            type_parameters: type_parameters,
                            parameter_type: maybe_parameter_type,
                            result_type: result_type,
                        },
                    );
                }
            }
        }
        for project_fn in project_fns_in_strongly_connected_component {
            let maybe_compiled_variable_declaration: Option<CompiledProjectFn> = project_fn_to_rust(
                errors,
                &mut records_used,
                &compiled_type_aliases,
                &compiled_choice_types,
                &compiledproject_fns,
                expressions,
                patterns,
                types,
                project_fn,
            );
            if let Some(compiled_project_fn) = maybe_compiled_variable_declaration {
                rust_items.push(compiled_project_fn.rust);
                compiledproject_fns.insert(
                    project_fn.name.value.clone(),
                    CompiledProjectFnInfo {
                        documentation: project_fn.documentation.as_ref().map(|documentation| {
                            documentation
                                .line1_up
                                .iter()
                                .fold(documentation.line0.value.to_string(), |so_far, line| {
                                    so_far + "\n" + &line.value
                                })
                                .into_boxed_str()
                        }),
                        type_parameters: match &project_fn.type_parameters {
                            None => vec![],
                            Some(type_parameters) => type_parameters
                                .names
                                .iter()
                                .map(|name| name.value.clone())
                                .collect(),
                        },
                        parameter_type: Some(compiled_project_fn.parameter_type),
                        result_type: Some(compiled_project_fn.result_type),
                    },
                );
            }
        }
    }
    rust_items.extend(
        records_used
            .iter()
            .filter(|fields| !core_records.contains(fields.as_slice()))
            .map(|used_record_fields| syntax_record_to_rust(used_record_fields)),
    );
    CompiledProject {
        rust: syn::File {
            shebang: None,
            attrs: vec![],
            items: rust_items,
        },
        type_aliases: compiled_type_aliases,
        choice_types: compiled_choice_types,
        fns: compiledproject_fns,
        records: records_used,
        // fn_graph: project_fn_graph,
        // fn_by_graph_node: project_fn_by_graph_node,
    }
}
fn syntax_record_to_rust(used_sloe_record_fields: &[Name]) -> syn::Item {
    let rust_struct_name: String =
        field_names_to_rust_record_struct_name(used_sloe_record_fields.iter());
    let rust_struct: syn::Item = syn::Item::Struct(syn::ItemStruct {
        attrs: vec![syn_attribute_derive(["Copy", "Clone", "Debug"].into_iter())],
        vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
        struct_token: syn::token::Struct(syn_span()),
        ident: syn_ident(&rust_struct_name),
        generics: syn::Generics {
            lt_token: Some(syn::token::Lt(syn_span())),
            params: used_sloe_record_fields
                .iter()
                .map(|field_name| {
                    syn::GenericParam::Type(syn::TypeParam {
                        attrs: vec![],
                        ident: syn_ident(&type_variable_to_rust(field_name)),
                        colon_token: None,
                        bounds: syn::punctuated::Punctuated::new(),
                        eq_token: None,
                        default: None,
                    })
                })
                .collect(),
            gt_token: Some(syn::token::Gt(syn_span())),
            where_clause: None,
        },
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace(syn_span()),
            named: used_sloe_record_fields
                .iter()
                .map(|field_name| syn::Field {
                    attrs: vec![],
                    vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
                    mutability: syn::FieldMutability::None,
                    ident: Some(syn_ident(&name_to_lowercase_rust(field_name))),
                    colon_token: Some(syn::token::Colon(syn_span())),
                    ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn_path_reference([&type_variable_to_rust(field_name)]),
                    }),
                })
                .collect(),
        }),
        semi_token: None,
    });
    rust_struct
}

struct CompiledTypeAlias {
    rust: syn::Item,
    is_copy: bool,
    type_: Type,
}
fn type_alias_declaration_to_rust<Types>(
    errors: &mut Vec<ErrorNode>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    choice_types: &std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    maybe_documentation: Option<&SyntaxComments>,
    name: &WithStartPosition<Name>,
    parameters: &[WithStartPosition<Name>],
    maybe_type: Option<&SyntaxType<Types>>,
) -> Option<CompiledTypeAlias> {
    let rust_name: String = name_to_uppercase_rust(&name.value);
    let Some(aliased_syntax_type) = maybe_type else {
        errors.push(ErrorNode {
            range: syntax_name_range(with_start_position_as_ref(name)),
            message: Box::from("type alias declaration is missing a type the given name is equal to after type alias ..type-name.. = here"),
        });
        return None;
    };
    let Some(aliased_type) = syntax_type_to_type(
        aliased_syntax_type,
        errors,
        type_aliases,
        choice_types,
        types,
        &std::collections::HashMap::new(),
    ) else {
        return None;
    };
    let type_rust: syn::Type = type_to_rust(&aliased_type);
    let mut actually_used_type_variables: std::collections::HashSet<Name> =
        std::collections::HashSet::with_capacity(parameters.len());
    type_variables_and_records_into(
        &mut actually_used_type_variables,
        records_used,
        &aliased_type,
    );
    let mut rust_parameters: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> =
        syn::punctuated::Punctuated::new();
    if let Err(()) = parameters_to_rust_into_error_if_different_to_actual_type_parameters(
        errors,
        &mut rust_parameters,
        syntax_name_range(with_start_position_as_ref(name)),
        parameters,
        actually_used_type_variables,
    ) {
        return None;
    }
    Some(CompiledTypeAlias {
        rust: syn::Item::Type(syn::ItemType {
            attrs: maybe_documentation
                .map(|documentation| syn_attribute_doc(&syntax_comments_to_string(documentation)))
                .into_iter()
                .collect::<Vec<_>>(),
            vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
            type_token: syn::token::Type(syn_span()),
            ident: syn_ident(&rust_name),
            generics: syn::Generics {
                lt_token: Some(syn::token::Lt(syn_span())),
                params: rust_parameters,
                gt_token: Some(syn::token::Gt(syn_span())),
                where_clause: None,
            },
            eq_token: syn::token::Eq(syn_span()),
            ty: Box::new(type_rust),
            semi_token: syn::token::Semi(syn_span()),
        }),
        is_copy: type_is_copy(true, type_aliases, choice_types, &aliased_type),
        type_: aliased_type,
    })
}

struct CompiledChoiceType {
    is_copy: bool,
    variants: Vec<CompiledVariantInfo>,
}
fn choice_type_declaration_to_rust_into<'a, Types>(
    rust_items: &mut Vec<syn::Item>,
    errors: &mut Vec<ErrorNode>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    choice_types: &std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    maybe_documentation: Option<&SyntaxComments>,
    name: &WithStartPosition<Name>,
    parameters: &'a [WithStartPosition<Name>],
    variants: &'a [SyntaxVariant<Types>],
) -> Option<CompiledChoiceType> {
    let mut rust_variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma> =
        syn::punctuated::Punctuated::new();
    let mut type_variants: Vec<CompiledVariantInfo> = Vec::with_capacity(rust_variants.len());
    let mut is_copy: bool = true;
    let mut actually_used_type_variables: std::collections::HashSet<Name> =
        std::collections::HashSet::with_capacity(parameters.len());
    'compiling_variants: for variant in variants {
        let Some(variant_name) = &variant.name else {
            errors.push(ErrorNode {
                range: symbol_range(variant.open_paren_start, "("),
                message: Box::from("missing variant name"),
            });
            continue 'compiling_variants;
        };
        // TODO verify these variant_type_prameters + the variables contained in the variant value add up to parameters
        let variant_type_prameters = variant
            .type_parameters
            .as_ref()
            .map(|type_parameters| type_parameters.names.iter().map(|name| name.value.clone()))
            .into_iter()
            .flatten()
            .collect();
        let Some(variant_value) = &variant.value else {
            errors.push(ErrorNode { range: syntax_name_range(with_start_position_as_ref(variant_name)), message: Box::from("missing variant value after this name. Every variant needs to have an associated value. If there's no extra info, simply use &.") });
            continue 'compiling_variants;
        };
        let Some(value_type) = syntax_type_to_type(
            variant_value,
            errors,
            type_aliases,
            choice_types,
            types,
            &std::collections::HashMap::new(),
        ) else {
            continue 'compiling_variants;
        };
        is_copy = is_copy && type_is_copy(true, type_aliases, choice_types, &value_type);
        type_variables_and_records_into(
            &mut actually_used_type_variables,
            records_used,
            &value_type,
        );
        let rust_variant_value: syn::Type = type_to_rust(&value_type);
        type_variants.push(CompiledVariantInfo {
            name: variant_name.value.clone(),
            type_parameters: variant_type_prameters,
            value: value_type,
        });
        rust_variants.push(syn::Variant {
            attrs: vec![],
            ident: syn_ident(&name_to_uppercase_rust(&variant_name.value)),
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: syn::token::Paren(syn_span()),
                unnamed: std::iter::once(syn::Field {
                    attrs: vec![],
                    vis: syn::Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    ident: None,
                    colon_token: None,
                    ty: rust_variant_value,
                })
                .collect(),
            }),
            discriminant: None,
        });
    }
    let mut rust_parameters: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> =
        syn::punctuated::Punctuated::new();
    if let Err(()) = parameters_to_rust_into_error_if_different_to_actual_type_parameters(
        errors,
        &mut rust_parameters,
        syntax_name_range(with_start_position_as_ref(name)),
        parameters,
        actually_used_type_variables,
    ) {
        return None;
    }
    let rust_enum_name: String = name_to_uppercase_rust(name.value.as_str());
    rust_items.push(syn::Item::Enum(syn::ItemEnum {
        attrs: maybe_documentation
            .map(|comments| syn_attribute_doc(&syntax_comments_to_string(comments)))
            .into_iter()
            .chain(std::iter::once(syn_attribute_derive(
                std::iter::once("Clone").chain(if is_copy { Some("Copy") } else { None }),
            )))
            .collect::<Vec<_>>(),
        vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
        enum_token: syn::token::Enum(syn_span()),
        ident: syn_ident(&rust_enum_name),
        generics: syn::Generics {
            lt_token: Some(syn::token::Lt(syn_span())),
            params: rust_parameters,
            gt_token: Some(syn::token::Gt(syn_span())),
            where_clause: None,
        },
        brace_token: syn::token::Brace(syn_span()),
        variants: rust_variants,
    }));
    Some(CompiledChoiceType {
        is_copy: is_copy,
        variants: type_variants,
    })
}

struct CompiledProjectFn {
    rust: syn::Item,
    parameter_type: Type,
    result_type: Type,
}
fn project_fn_to_rust<'a, Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    choice_types: &std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    project_fns: &std::collections::HashMap<Name, CompiledProjectFnInfo>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    project_fn_info: SyntaxProjectFnInfo<'a, Expressions, Patterns, Types>,
) -> Option<CompiledProjectFn> {
    let Some(result_node) = project_fn_info.result else {
        errors.push(ErrorNode {
            range: project_fn_info.range,
            message: Box::from(
                "missing expression after the fn result type. An example would be my-variable & str \":)\", where & is an empty record as the parameter",
            ),
        });
        return None;
    };
    let Some(syntax_parameter) = &project_fn_info.parameter else {
        errors.push(ErrorNode {
            range: project_fn_info.range,
            message: Box::from(
                "missing parameter pattern after the fn name. An example would be my-variable & str \":)\", where & is an empty record as the parameter",
            ),
        });
        return None;
    };
    let mut parameter_introduced_bindings = std::collections::HashMap::new();
    let Some(compiled_parameter) = syntax_pattern_to_rust(
        syntax_parameter,
        None,
        errors,
        records_used,
        &mut parameter_introduced_bindings,
        type_aliases,
        choice_types,
        patterns,
        types,
        &std::collections::HashMap::new(),
    ) else {
        return None;
    };
    let mut used_pattern_variables = std::collections::HashMap::new();
    let compiled_result: CompiledExpression = syntax_expression_to_rust(
        errors,
        records_used,
        type_aliases,
        choice_types,
        project_fns,
        expressions,
        patterns,
        types,
        &mut parameter_introduced_bindings,
        &mut std::collections::HashMap::new(),
        &mut std::collections::HashMap::new(),
        &mut used_pattern_variables,
        result_node,
    );
    for parameter_introduced_binding_name in parameter_introduced_bindings.keys() {
        push_error_if_introduced_variable_collides_or_is_unused(
            errors,
            project_fns,
            &std::collections::HashMap::new(),
            parameter_introduced_binding_name,
            used_pattern_variables
                .get(parameter_introduced_binding_name)
                .copied(),
        );
    }
    let Some(actual_result_expression_type) = compiled_result.type_ else {
        // rust top level declarations need explicit types; partial types won't do
        return None;
    };
    // TODO compare with actual syntax result type
    let rust_attrs: Vec<syn::Attribute> = project_fn_info
        .documentation
        .as_ref()
        .map(|n| syn_attribute_doc(&syntax_comments_to_string(n)))
        .into_iter()
        .collect::<Vec<_>>();
    let rust_ident: syn::Ident = syn_ident(&name_to_lowercase_rust(&project_fn_info.name.value));
    let mut input_type_parameters: std::collections::HashSet<&Name> =
        std::collections::HashSet::new();
    syntax_pattern_type_variables_into(
        &mut input_type_parameters,
        syntax_parameter,
        patterns,
        types,
    );
    if let Some(result_type) = project_fn_info.result_type {
        let mut result_type_parameters: std::collections::HashSet<&Name> =
            std::collections::HashSet::new();
        syntax_type_variables_into(&mut result_type_parameters, result_type, types);
        result_type_parameters
            .retain(|result_type_parameter| !input_type_parameters.contains(result_type_parameter));
        if !result_type_parameters.is_empty() {
            let mut full_type_as_string: String = String::new();
            type_format(&mut full_type_as_string, 0, &actual_result_expression_type);
            errors.push(ErrorNode {
                range: syntax_name_range(with_start_position_as_ref(project_fn_info.name)),
                message: format!(
                    "its output type contains variables not introduced in its input types, namely {}. In sloe, every value has a concrete type, so no value could satisfy such a type. Here is the full type:\n{}",
                    result_type_parameters.iter().map(|parameter| parameter.as_str()).collect::<Vec<&str>>().join(", "),
                    &full_type_as_string
                ).into_boxed_str()
            });
            return None;
        }
    }
    let rust_generics: syn::Generics = syn::Generics {
        lt_token: Some(syn::token::Lt(syn_span())),
        params: input_type_parameters
            .iter()
            .map(|name| {
                syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident: syn_ident(&type_variable_to_rust(name)),
                    colon_token: Some(syn::token::Colon(syn_span())),
                    bounds: default_parameter_bounds().collect(),
                    eq_token: None,
                    default: None,
                })
            })
            .collect(),
        gt_token: Some(syn::token::Gt(syn_span())),
        where_clause: None,
    };
    Some(CompiledProjectFn {
        rust: syn::Item::Fn(syn::ItemFn {
            attrs: rust_attrs,
            vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
            sig: syn::Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: syn::token::Fn(syn_span()),
                ident: rust_ident,
                generics: rust_generics,
                paren_token: syn::token::Paren(syn_span()),
                inputs: [syn::FnArg::Typed(syn::PatType {
                    pat: Box::new(compiled_parameter.rust),
                    attrs: vec![],
                    colon_token: syn::token::Colon(syn_span()),
                    ty: Box::new(type_to_rust(&compiled_parameter.type_)),
                })]
                .into_iter()
                .collect(),
                output: syn::ReturnType::Type(
                    syn::token::RArrow(syn_span()),
                    Box::new(type_to_rust(&actual_result_expression_type)),
                ),
                variadic: None,
            },
            block: Box::new(syn_spread_expr_block(compiled_result.rust)),
        }),
        parameter_type: compiled_parameter.type_,
        result_type: actual_result_expression_type,
    })
}
fn syntax_comments_to_string(comments: &SyntaxComments) -> String {
    std::iter::once(&comments.line0)
        .chain(&comments.line1_up)
        .map(|line| line.value.as_ref())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn syntax_type_to_type<Types>(
    type_: &SyntaxType<Types>,
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    choice_types: &std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashMap<&Name, OriginCompileInfo>,
) -> Option<Type> {
    match type_ {
        SyntaxType::Variable(name) => Some(Type::Variable(name.value.clone())),
        SyntaxType::Parenthesized {
            open_paren_start,
            inner,
            closed_paren_start,
        } => match inner {
            None => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: *open_paren_start,
                        end: closed_paren_start
                            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
                            .unwrap_or(*open_paren_start),
                    },
                    message: Box::from("missing type inside these parens (here)"),
                });
                None
            }
            Some(inner) => syntax_type_to_type(
                types.element(inner),
                errors,
                type_aliases,
                choice_types,
                types,
                origins,
            ),
        },
        SyntaxType::Construct { name, arguments } => {
            if origins.contains_key(&name.value) {
                if let Some(_) = arguments {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(name)),
                        message : Box::from("this type refers to an origin but has type arguments. As origin types don't have type parameters, the arguments need to be removed")
                    });
                }
                return Some(Type::ChoiceConstruct {
                    name: name.value.clone(),
                    arguments: vec![],
                });
            }
            let argument_types = types
                .opt_span_slice(core::Opt::from_option(arguments.as_ref()))
                .iter()
                .map(|argument| {
                    syntax_type_to_type(
                        argument,
                        errors,
                        type_aliases,
                        choice_types,
                        types,
                        origins,
                    )
                })
                .collect::<Option<Vec<Type>>>()?;
            let argument_count = core::Opt::from_option(arguments.as_ref()).length() as usize;
            if let Some(origin_type_alias) = type_aliases.get(&name.value) {
                match origin_type_alias.parameters.len().cmp(&argument_count) {
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Less => {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: format!(
                                "this type alias has {} less parameters than arguments are provided here.",
                                argument_count - origin_type_alias.parameters.len(),
                            ).into_boxed_str()
                        });
                        return None;
                    }
                    std::cmp::Ordering::Greater => {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: format!(
                                "this type alias has {} more parameters than arguments are provided here. The additional parameters are called {}",
                                origin_type_alias.parameters.len() - argument_count,
                                origin_type_alias.parameters.iter().map(|parameter| parameter.as_str()).skip(argument_count).collect::<Vec<_>>().join(", ")
                            ).into_boxed_str()
                        });
                        // later arguments will be ignored
                    }
                }
                return type_construct_resolve_type_alias(origin_type_alias, &argument_types);
            }
            let Some(origin_choice_type) = choice_types.get(&name.value) else {
                errors.push(ErrorNode {
                    range: syntax_name_range(with_start_position_as_ref(name)),
                    message: Box::from("no type alias or choice type is declared with this name"),
                });
                return None;
            };
            match origin_choice_type.parameters.len().cmp(&argument_count) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(name)),
                        message: format!(
                            "this choice type has {} less parameters than arguments are provided here.",
                            argument_count - origin_choice_type.parameters.len(),
                        ).into_boxed_str()
                    });
                    return None;
                }
                std::cmp::Ordering::Greater => {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(name)),
                        message: format!(
                            "this choice type has {} more parameters than arguments are provided here. The additional parameters are called {}",
                            origin_choice_type.parameters.len() - argument_count,
                            origin_choice_type.parameters.iter().map(|parameter| parameter.as_str()).skip(argument_count).collect::<Vec<_>>().join(", ")
                        ).into_boxed_str()
                    });
                    // later arguments will be ignored
                }
            }
            Some(Type::ChoiceConstruct {
                name: name.value.clone(),
                arguments: argument_types,
            })
        }
        SyntaxType::Record {
            ampersand_start: _,
            fields,
        } => {
            let mut field_types: Vec<TypeField> = Vec::with_capacity(fields.capacity());
            let mut any_field_value_has_error: bool = false;
            for field in fields {
                if field_types
                    .iter()
                    .any(|type_field| type_field.name == field.name.value)
                {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from(
                            "a field with this name already exists in the record type",
                        ),
                    });
                    return None;
                }
                let Some(field_value) = &field.value else {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from(
                            "missing field value after this name ..field-name.. here",
                        ),
                    });
                    return None;
                };
                match syntax_type_to_type(
                    field_value,
                    errors,
                    type_aliases,
                    choice_types,
                    types,
                    origins,
                ) {
                    None => {
                        any_field_value_has_error = true;
                    }
                    Some(field_value_type) => {
                        field_types.push(TypeField {
                            name: field.name.value.clone(),
                            value: field_value_type,
                        });
                    }
                }
            }
            if any_field_value_has_error {
                return None;
            }
            Some(Type::Record(field_types))
        }
    }
}
fn type_construct_resolve_type_alias(
    origin_type_alias: &CompiledTypeAliasInfo,
    argument_types: &[Type],
) -> Option<Type> {
    let Some(type_alias_type) = &origin_type_alias.type_ else {
        return None;
    };
    if origin_type_alias.parameters.is_empty() {
        return Some(type_alias_type.clone());
    }
    let type_parameter_replacements: std::collections::HashMap<&str, std::borrow::Cow<Type>> =
        origin_type_alias
            .parameters
            .iter()
            .map(|n| n.as_str())
            .zip(argument_types.iter().map(std::borrow::Cow::Borrowed))
            .collect::<std::collections::HashMap<_, _>>();
    let mut peeled: Type = type_alias_type.clone();
    type_replace_variables(&type_parameter_replacements, &mut peeled);
    Some(peeled)
}
fn type_replace_variables(
    type_parameter_replacements: &std::collections::HashMap<&str, std::borrow::Cow<Type>>,
    type_: &mut Type,
) {
    match type_ {
        Type::Variable(variable) => {
            if let Some(replacement_type_node) = type_parameter_replacements.get(variable.as_str())
            {
                *type_ = replacement_type_node.as_ref().clone();
            }
        }
        Type::ChoiceConstruct { name: _, arguments } => {
            for argument_type in arguments {
                type_replace_variables(type_parameter_replacements, argument_type);
            }
        }
        Type::Record(fields) => {
            for field in fields {
                type_replace_variables(type_parameter_replacements, &mut field.value);
            }
        }
    }
}

fn type_to_rust(type_: &Type) -> syn::Type {
    match type_ {
        Type::Variable(variable) => syn_type_variable(&type_variable_to_rust(variable)),
        Type::ChoiceConstruct { name, arguments } => syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: std::iter::once(syn::PathSegment {
                    ident: syn_ident(&name_to_uppercase_rust(name)),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: syn::token::Lt(syn_span()),
                            args: arguments
                                .iter()
                                .map(|argument_type| {
                                    syn::GenericArgument::Type(type_to_rust(argument_type))
                                })
                                .collect(),
                            gt_token: syn::token::Gt(syn_span()),
                        },
                    ),
                })
                .collect(),
            },
        }),
        Type::Record(fields) => {
            let mut fields_sorted: Vec<&TypeField> = fields.iter().collect();
            fields_sorted.sort_unstable_by_key(|a| &a.name);
            syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: std::iter::once(syn::PathSegment {
                        ident: syn_ident(&field_names_to_rust_record_struct_name(
                            fields_sorted.iter().map(|field| &field.name),
                        )),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: syn::token::Lt(syn_span()),
                                gt_token: syn::token::Gt(syn_span()),
                                args: fields_sorted
                                    .into_iter()
                                    .map(|field| {
                                        syn::GenericArgument::Type(type_to_rust(&field.value))
                                    })
                                    .collect(),
                            },
                        ),
                    })
                    .collect(),
                },
            })
        }
    }
}
fn type_variables_into(type_variables: &mut std::collections::HashSet<Name>, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            type_variables.insert(name.clone());
        }
        Type::ChoiceConstruct { name: _, arguments } => {
            for argument in arguments {
                type_variables_into(type_variables, argument);
            }
        }
        Type::Record(fields) => {
            for field in fields {
                type_variables_into(type_variables, &field.value);
            }
        }
    }
}
fn type_variables_and_records_into(
    type_variables: &mut std::collections::HashSet<Name>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    type_: &Type,
) {
    match type_ {
        Type::Variable(name) => {
            type_variables.insert(name.clone());
        }
        Type::ChoiceConstruct { name: _, arguments } => {
            for argument in arguments {
                type_variables_and_records_into(type_variables, records_used, argument);
            }
        }
        Type::Record(fields) => {
            records_used.insert(sorted_field_names(fields.iter().map(|field| &field.name)));
            for field in fields {
                type_variables_and_records_into(type_variables, records_used, &field.value);
            }
        }
    }
}
fn parameters_to_rust_into_error_if_different_to_actual_type_parameters(
    errors: &mut Vec<ErrorNode>,
    rust_parameters: &mut syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    name_range: lsp_types::Range,
    parameters: &[WithStartPosition<Name>],
    mut actually_used_type_variables: std::collections::HashSet<Name>,
) -> Result<(), ()> {
    let mut bad_parameters: bool = false;
    for parameter in parameters {
        if !actually_used_type_variables.remove(parameter.value.as_str()) {
            bad_parameters = true;
            errors.push(ErrorNode {
                range: syntax_name_range(with_start_position_as_ref(parameter)),
                message: Box::from("this type variable is not used. Remove it or use it"),
            });
        }
        rust_parameters.push(syn::GenericParam::Type(syn::TypeParam::from(syn_ident(
            &type_variable_to_rust(&parameter.value),
        ))));
    }
    if !actually_used_type_variables.is_empty() {
        bad_parameters = true;
        errors.push(ErrorNode {
            range: name_range,
            message: format!(
                "some type variables are used but not declared, namely {}. Add {}",
                actually_used_type_variables
                    .iter()
                    .map(Name::as_str)
                    .collect::<Vec<&str>>()
                    .join(", "),
                if actually_used_type_variables.len() >= 2 {
                    "them"
                } else {
                    "it"
                }
            )
            .into_boxed_str(),
        });
    }
    if bad_parameters { Err(()) } else { Ok(()) }
}

struct CompiledPattern {
    rust: syn::Pat,
    type_: Type,
    catch: PatternCatch,
}
#[derive(PartialEq, Eq, Debug)]
enum PatternCatch {
    Exhaustive,
    /// invariant: all variants are never exhaustive
    /// and len is >= 2
    /// and only a single variant value is VariantCatch::Caught
    Variant(std::collections::BTreeMap<Name, VariantCatch<PatternCatch>>),
    /// invariant: all fields are never exhaustive
    /// and field count is >= 2
    Record(std::collections::BTreeMap<Name, PatternCatch>),
}
#[derive(PartialEq, Eq, Debug)]
enum VariantCatch<Catch> {
    Caught(Catch),
    // TODO this is always true and thus should be removed
    Uncaught { has_value: bool },
}

#[derive(PartialEq, Eq, Debug)]
enum CasePatternsCatch {
    Exhaustive,
    /// invariant: all variants are never exhaustive
    // and choice_type_variant_count is >= 2
    Variants(std::collections::BTreeMap<Name, VariantCatch<CasePatternsCatch>>),
    /// invariant: all fields are never exhaustive
    // and field count is >= 2
    Record(Vec<std::collections::BTreeMap<Name, PatternCatch>>),
}
fn pattern_catch_to_case_patterns_catch(pattern_catch: PatternCatch) -> CasePatternsCatch {
    match pattern_catch {
        PatternCatch::Exhaustive => CasePatternsCatch::Exhaustive,
        PatternCatch::Variant(variants) => CasePatternsCatch::Variants(
            variants
                .into_iter()
                .map(|(name, variant_catch)| {
                    (
                        name,
                        match variant_catch {
                            VariantCatch::Uncaught { has_value } => VariantCatch::Uncaught {
                                has_value: has_value,
                            },
                            VariantCatch::Caught(value_catch) => VariantCatch::Caught(
                                pattern_catch_to_case_patterns_catch(value_catch),
                            ),
                        },
                    )
                })
                .collect(),
        ),
        PatternCatch::Record(fields) => CasePatternsCatch::Record(vec![fields]),
    }
}
fn pattern_catch_merge_with(
    errors: &mut Vec<ErrorNode>,
    pattern_range: lsp_types::Range,
    catch: &mut CasePatternsCatch,
    new_catch: PatternCatch,
) {
    match catch {
        CasePatternsCatch::Exhaustive => {
            errors.push(ErrorNode { range: pattern_range, message: Box::from("unreachable pattern. All previous case patterns already exhaustively match any possible value") });
        }
        CasePatternsCatch::Variants(variants) => match new_catch {
            PatternCatch::Exhaustive => {
                *catch = CasePatternsCatch::Exhaustive;
            }
            PatternCatch::Variant(new_variants) => {
                if let Some((new_variant_name, new_variant_caught)) = new_variants
                    .into_iter()
                    .find_map(
                        |(new_variant_name, new_variant_catch)| match new_variant_catch {
                            VariantCatch::Caught(new_variant_caught) => {
                                Some((new_variant_name, new_variant_caught))
                            }
                            VariantCatch::Uncaught { .. } => None,
                        },
                    )
                    && let Some(previous_catch_of_new_variant) = variants.get_mut(&new_variant_name)
                {
                    match previous_catch_of_new_variant {
                        VariantCatch::Caught(CasePatternsCatch::Exhaustive) => {
                            errors.push(ErrorNode {
                                range: pattern_range,
                                message: Box::from("this pattern is unreachable as it's already matched by a previous case pattern"),
                            });
                        }
                        VariantCatch::Caught(previous_caught_of_new_variant) => {
                            pattern_catch_merge_with(
                                errors,
                                pattern_range,
                                previous_caught_of_new_variant,
                                new_variant_caught,
                            );
                            if variants.values().all(|variant_catch| {
                                variant_catch
                                    == &VariantCatch::Caught(CasePatternsCatch::Exhaustive)
                            }) {
                                *catch = CasePatternsCatch::Exhaustive;
                            }
                        }
                        VariantCatch::Uncaught { .. } => {
                            *previous_catch_of_new_variant = VariantCatch::Caught(
                                pattern_catch_to_case_patterns_catch(new_variant_caught),
                            );
                            if variants.values().all(|variant_catch| {
                                variant_catch
                                    == &VariantCatch::Caught(CasePatternsCatch::Exhaustive)
                            }) {
                                *catch = CasePatternsCatch::Exhaustive;
                            }
                        }
                    }
                }
            }
            _ => {}
        },
        CasePatternsCatch::Record(possibilities) => match new_catch {
            PatternCatch::Exhaustive => {
                *catch = CasePatternsCatch::Exhaustive;
            }
            PatternCatch::Record(new_possibility) => {
                if possibilities.iter().any(|record_possibility| {
                    record_possibility
                        .values()
                        .zip(new_possibility.values())
                        .all(|(possibility_field_value, new_possibility_field_value)| {
                            pattern_catch_catches_all_of_sloe_pattern_catch(
                                possibility_field_value,
                                new_possibility_field_value,
                            )
                        })
                }) {
                    errors.push(ErrorNode {
                        range: pattern_range,
                        message: Box::from("this pattern is unreachable as it's already matched by a previous case pattern"),
                    });
                } else {
                    possibilities.push(new_possibility);
                    if case_patterns_catch_record_is_exhaustive(possibilities) {
                        *catch = CasePatternsCatch::Exhaustive;
                    }
                }
            }
            _ => {}
        },
    }
}
fn pattern_catch_catches_all_of_sloe_pattern_catch(
    catch: &PatternCatch,
    to_check: &PatternCatch,
) -> bool {
    match catch {
        PatternCatch::Exhaustive => true,
        PatternCatch::Variant(variants) => {
            if let PatternCatch::Variant(variants_to_check) = to_check {
                variants.values().zip(variants_to_check.values()).all(
                    |(variant_catch, variant_catch_to_check)| match (
                        variant_catch,
                        variant_catch_to_check,
                    ) {
                        (VariantCatch::Uncaught { .. }, VariantCatch::Caught(_)) => false,
                        (VariantCatch::Uncaught { .. }, VariantCatch::Uncaught { .. }) => true,
                        (VariantCatch::Caught(_), VariantCatch::Uncaught { .. }) => true,
                        (
                            VariantCatch::Caught(variant_value),
                            VariantCatch::Caught(variant_value_to_check),
                        ) => pattern_catch_catches_all_of_sloe_pattern_catch(
                            variant_value,
                            variant_value_to_check,
                        ),
                    },
                )
            } else {
                false
            }
        }
        PatternCatch::Record(fields) => {
            if let PatternCatch::Record(fields_to_check) = to_check {
                fields.values().zip(fields_to_check.values()).all(
                    |(field_value, field_value_to_check)| {
                        pattern_catch_catches_all_of_sloe_pattern_catch(
                            field_value,
                            field_value_to_check,
                        )
                    },
                )
            } else {
                false
            }
        }
    }
}

enum PatternCatchPossibilitiesSplit<'a> {
    // consider adding example pattern
    ByVariant(std::collections::BTreeMap<Name, Vec<Vec<&'a PatternCatch>>>),
    WithAdditionalFieldValues {
        field_count: usize,
        possibilities: Vec<Vec<&'a PatternCatch>>,
    },
    AllExhaustive(Vec<Vec<&'a PatternCatch>>),
}
fn case_patterns_catch_record_is_exhaustive(
    record_possibilities: &[std::collections::BTreeMap<Name, PatternCatch>],
) -> bool {
    possibilities_of_pattern_catches_are_exhaustive(
        // it's unfortunate that we need to allocate here,
        // since rust runs into an "reached the recursion limit while instantiating"
        // error when instantiating Iterators (recursively)
        &record_possibilities
            .iter()
            .map(|record_possibility| record_possibility.values().collect())
            .collect::<Vec<_>>(),
    )
}
/// don't ask wtf this algorithm is, I'm too dumb to understand the existing literature.
/// Here's what I've come up with:
///
/// Assume the case shape
///   [  ( a0, a1, a2, a3 )
///   or ( b0, b1, b2, b3 )
///   or ... ]
/// where we know the pattern at each index has the same type.
/// We then look at each pattern at index 0:
///
///    when this pattern type is a choice type, categorize by
///    variant name, and check the value + remaining indices individually for exhaustiveness
///    for example:
///      ( None, a1 ) or ( Some v0, b1 ) or ( None, c1 )
///      → is_exhaustive [ ( _, a1 ) or ( _, c1 ) ] && is_exhaustive [ ( v0, b1 ) ]
///    if we encounter a variable or ignore pattern, we copy it's possibilities
///    to all "by variant" possibilities
///
///   when this pattern type is a record, spread (flatten) its field values into the original possibilities
///   for example:
///      ( { x ax0, y ay0 }, a1 ) or ( { x ax0, y ay0 }, b1 )
///      → is_exhaustive [ ( ax0, ay0, a1 ) or ( ax0, ay0, b1 ) ]
///
/// when all patterns on index 0 are variable or ignore patterns
/// repeat until the patterns on index 0 together aren't exhaustive (return false) or
/// all remaining cases are exhaustive (return true)
fn possibilities_of_pattern_catches_are_exhaustive<'a>(
    possibilities_of_pattern_catches: &'a [Vec<&'a PatternCatch>],
) -> bool {
    let maybe_split: Option<PatternCatchPossibilitiesSplit> =
        possibilities_of_pattern_catches.iter().fold(
            None,
            |mut maybe_so_far, possibility_values| {
                match possibility_values.split_first() {
                    None => maybe_so_far,
                    Some((first_value_catch, remaining_value_catches)) => {
                        match first_value_catch {
                            PatternCatch::Exhaustive => match &mut maybe_so_far {
                                None => Some(PatternCatchPossibilitiesSplit::AllExhaustive(vec![
                                    remaining_value_catches.to_vec(),
                                ])),
                                Some(PatternCatchPossibilitiesSplit::AllExhaustive(
                                    possibilities,
                                )) => {
                                    possibilities.push(remaining_value_catches.to_vec());
                                    maybe_so_far
                                }
                                Some(
                                    PatternCatchPossibilitiesSplit::WithAdditionalFieldValues {
                                        field_count,
                                        possibilities,
                                    },
                                ) => {
                                    possibilities.push(
                                        std::iter::repeat_n(
                                            &PatternCatch::Exhaustive,
                                            *field_count,
                                        )
                                        .chain(remaining_value_catches.iter().copied())
                                        .collect(),
                                    );
                                    maybe_so_far
                                }
                                Some(PatternCatchPossibilitiesSplit::ByVariant(
                                    possibilities_by_variant,
                                )) => {
                                    for possibilities_for_variant in
                                        possibilities_by_variant.values_mut()
                                    {
                                        possibilities_for_variant.push(
                                            std::iter::once(&PatternCatch::Exhaustive)
                                                .chain(remaining_value_catches.iter().copied())
                                                .collect(),
                                        );
                                    }
                                    maybe_so_far
                                }
                            },
                            PatternCatch::Variant(first_field_value_variants) => {
                                let Some((
                                    first_field_value_variant_name,
                                    first_field_value_variant_value_catch,
                                )) = first_field_value_variants.iter().find_map(
                                    |(
                                        first_field_value_variant_name,
                                        first_field_value_variant_catch,
                                    )| {
                                        match first_field_value_variant_catch {
                                            VariantCatch::Uncaught { .. } => None,
                                            VariantCatch::Caught(value_caught) => {
                                                Some((first_field_value_variant_name, value_caught))
                                            }
                                        }
                                    },
                                )
                                else {
                                    return maybe_so_far;
                                };
                                let new_possibility_for_variant: Vec<&PatternCatch> =
                                    std::iter::once(first_field_value_variant_value_catch)
                                        .chain(remaining_value_catches.iter().copied())
                                        .collect();
                                match &mut maybe_so_far {
                                    None => {
                                        let mut by_variant_empty: std::collections::BTreeMap<
                                            Name,
                                            Vec<Vec<&PatternCatch>>,
                                        > = first_field_value_variants
                                            .keys()
                                            .map(|variant_name| (variant_name.clone(), vec![]))
                                            .collect();
                                        if let Some(first_field_value_variant_possibilities) =
                                            by_variant_empty.get_mut(first_field_value_variant_name)
                                        {
                                            first_field_value_variant_possibilities
                                                .push(new_possibility_for_variant);
                                        }
                                        Some(PatternCatchPossibilitiesSplit::ByVariant(
                                            by_variant_empty,
                                        ))
                                    }
                                    Some(PatternCatchPossibilitiesSplit::ByVariant(
                                        so_far_by_variant,
                                    )) => {
                                        if let Some(variant_possibilities_so_far) =
                                            so_far_by_variant
                                                .get_mut(first_field_value_variant_name)
                                        {
                                            variant_possibilities_so_far
                                                .push(new_possibility_for_variant);
                                        }
                                        maybe_so_far
                                    }
                                    Some(PatternCatchPossibilitiesSplit::AllExhaustive(
                                        possibilities,
                                    )) => {
                                        let possibilities_for_each_variant: Vec<
                                            Vec<&PatternCatch>,
                                        > = possibilities
                                            .iter()
                                            .map(|possibility| {
                                                std::iter::once(&PatternCatch::Exhaustive)
                                                    .chain(possibility.iter().copied())
                                                    .collect()
                                            })
                                            .collect();
                                        let mut by_variant_empty: std::collections::BTreeMap<
                                            Name,
                                            Vec<Vec<&PatternCatch>>,
                                        > = first_field_value_variants
                                            .keys()
                                            .map(|variant_name| {
                                                (
                                                    variant_name.clone(),
                                                    possibilities_for_each_variant.clone(),
                                                )
                                            })
                                            .collect();
                                        if let Some(first_field_value_variant_possibilities) =
                                            by_variant_empty.get_mut(first_field_value_variant_name)
                                        {
                                            first_field_value_variant_possibilities
                                                .push(new_possibility_for_variant);
                                        }
                                        Some(PatternCatchPossibilitiesSplit::ByVariant(
                                            by_variant_empty,
                                        ))
                                    }
                                    // type error
                                    Some(
                                        PatternCatchPossibilitiesSplit::WithAdditionalFieldValues {
                                            ..
                                        },
                                    ) => maybe_so_far,
                                }
                            }
                            PatternCatch::Record(first_field_value_fields) => {
                                let new_possibility_for_record: Vec<&PatternCatch> =
                                    first_field_value_fields
                                        .values()
                                        .chain(remaining_value_catches.iter().copied())
                                        .collect();
                                match &mut maybe_so_far {
                                    None => Some(
                                        PatternCatchPossibilitiesSplit::WithAdditionalFieldValues {
                                            field_count: first_field_value_fields.len(),
                                            possibilities: vec![new_possibility_for_record],
                                        },
                                    ),
                                    Some(
                                        PatternCatchPossibilitiesSplit::WithAdditionalFieldValues {
                                            possibilities:
                                                with_record_field_values_possibilities_so_far,
                                            field_count: _,
                                        },
                                    ) => {
                                        with_record_field_values_possibilities_so_far
                                            .push(new_possibility_for_record);
                                        maybe_so_far
                                    }
                                    Some(PatternCatchPossibilitiesSplit::AllExhaustive(
                                        possibilities,
                                    )) => Some(
                                        PatternCatchPossibilitiesSplit::WithAdditionalFieldValues {
                                            field_count: first_field_value_fields.len(),
                                            possibilities: std::iter::once(
                                                new_possibility_for_record,
                                            )
                                            .chain(possibilities.iter().map(|possibility| {
                                                std::iter::repeat_n(
                                                    &PatternCatch::Exhaustive,
                                                    first_field_value_fields.len(),
                                                )
                                                .chain(possibility.iter().copied())
                                                .collect()
                                            }))
                                            .collect(),
                                        },
                                    ),
                                    // type error
                                    Some(PatternCatchPossibilitiesSplit::ByVariant(_)) => {
                                        maybe_so_far
                                    }
                                }
                            }
                        }
                    }
                }
            },
        );
    match maybe_split {
        None => {
            // no possibilities at all. This case is hit when e.g. a variant never occurs
            false
        }
        Some(split) => match split {
            PatternCatchPossibilitiesSplit::ByVariant(possibilities_by_variant) => {
                possibilities_by_variant
                    .values()
                    .all(|possibilities_for_variant| {
                        possibilities_of_pattern_catches_are_exhaustive(possibilities_for_variant)
                    })
            }
            PatternCatchPossibilitiesSplit::AllExhaustive(possibilities) => {
                // a more performant way to check this
                // would be setting an "input was empty" bool
                if possibilities.iter().all(Vec::is_empty) {
                    return true;
                }
                possibilities_of_pattern_catches_are_exhaustive(&possibilities)
            }
            PatternCatchPossibilitiesSplit::WithAdditionalFieldValues {
                field_count: _,
                possibilities,
            } => possibilities_of_pattern_catches_are_exhaustive(&possibilities),
        },
    }
}

fn syntax_pattern_to_rust<'a, Patterns, Types>(
    pattern: &'a SyntaxPattern<Patterns, Types>,
    expected_type: Option<&Type>,
    errors: &mut Vec<ErrorNode>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, PatternVariableCompileInfo>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    choice_types: &std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashMap<&Name, OriginCompileInfo>,
) -> Option<CompiledPattern> {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            let maybe_compiled_variable = match type_.as_ref() {
                None => match expected_type {
                    None => {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: Box::from("fn parameters need to have an explicit type. Add one to this pattern variable by appending a type like in your-variable u32 (both in parens if necessary)"),
                        });
                        None
                    }
                    Some(expected_type) => Some(CompiledPattern {
                        rust: syn::Pat::Ident(syn::PatIdent {
                            attrs: vec![],
                            by_ref: None,
                            mutability: None,
                            ident: syn_ident(&name_to_lowercase_rust(&name.value)),
                            subpat: None,
                        }),
                        type_: expected_type.clone(),
                        catch: PatternCatch::Exhaustive,
                    }),
                },
                Some(actual_type) => {
                    let Some(actual_type) = syntax_type_to_type(
                        actual_type,
                        errors,
                        type_aliases,
                        choice_types,
                        types,
                        origins,
                    ) else {
                        return None;
                    };
                    match expected_type {
                        None => Some(CompiledPattern {
                            rust: syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: syn_ident(&name_to_lowercase_rust(&name.value)),
                                subpat: None,
                            }),
                            type_: actual_type,
                            catch: PatternCatch::Exhaustive,
                        }),
                        Some(expected_type) => {
                            // TODO report if diff?
                            Some(CompiledPattern {
                                rust: syn::Pat::Type(syn::PatType {
                                    attrs: vec![],
                                    pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                                        attrs: vec![],
                                        by_ref: None,
                                        mutability: None,
                                        ident: syn_ident(&name_to_lowercase_rust(&name.value)),
                                        subpat: None,
                                    })),
                                    colon_token: syn::token::Colon(syn_span()),
                                    ty: Box::new(type_to_rust(expected_type)),
                                }),
                                type_: actual_type,
                                catch: PatternCatch::Exhaustive,
                            })
                        }
                    }
                }
            };
            if let Some(compiled_variable) = &maybe_compiled_variable {
                let maybe_existing_variable_with_the_same_name = introduced_variables.insert(
                    &name.value,
                    PatternVariableCompileInfo {
                        origin_start: name.start,
                        type_: Some(compiled_variable.type_.clone()),
                    },
                );
                if maybe_existing_variable_with_the_same_name.is_some() {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(name)),
                        message: Box::from(
                            "a pattern variable with this name already exists. Rename it",
                        ),
                    });
                    return None;
                } else if origins.contains_key(&name.value) {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(name)),
                        message: Box::from("an origin with this name already exists. Rename it"),
                    });
                    return None;
                }
            }
            maybe_compiled_variable
        }
        SyntaxPattern::Variant {
            name,
            type_arguments,
            value,
        } => {
            let Some((origin_choice_type_name, origin_choice_type, origin_variant)) =
                choice_type_containing_variant(choice_types, &name.value)
            else {
                errors.push(ErrorNode { range: syntax_name_range(with_start_position_as_ref(name)), message: Box::from("this variant name is not declared in any choice type. Check for typos or declare this variant") });
                return None;
            };
            let syntax_type_arguments = match type_arguments {
                None => &[],
                Some(type_arguments) => {
                    types.opt_span_slice(core::Opt::from_option(type_arguments.types.as_ref()))
                }
            };
            let mut type_arguments = Vec::new();
            for syntax_type_argument in syntax_type_arguments {
                let Some(type_argument) = syntax_type_to_type(
                    syntax_type_argument,
                    errors,
                    type_aliases,
                    choice_types,
                    types,
                    origins,
                ) else {
                    return None;
                };
                type_arguments.push(type_argument);
            }
            let rust_variant_path: syn::Path = syn_path_reference([
                &name_to_uppercase_rust(origin_choice_type_name),
                &name_to_uppercase_rust(&name.value),
            ]);
            match expected_type {
                Some(expected_type) => {
                    let Type::ChoiceConstruct {
                        name: expected_choice_type_construct_name,
                        arguments: expected_choice_type_construct_arguments,
                    } = expected_type
                    else {
                        // TODO report diff?
                        return None;
                    };
                    if origin_choice_type_name != expected_choice_type_construct_name {
                        // TODO report diff?
                        return None;
                    }
                    let expected_type_parameter_replacements = origin_choice_type
                        .parameters
                        .iter()
                        .zip(expected_choice_type_construct_arguments)
                        .map(|(parameter, expected_argument)| {
                            (
                                parameter.as_str(),
                                std::borrow::Cow::Borrowed(expected_argument),
                            )
                        })
                        .collect::<std::collections::HashMap<&str, _>>();
                    for (variant_argument_index, (variant_type_parameter, type_argument)) in
                        origin_variant
                            .type_parameters
                            .iter()
                            .zip(&type_arguments)
                            .enumerate()
                    {
                        if let Some(expected_type_argument) = expected_type_parameter_replacements
                            .get(variant_type_parameter.as_str())
                            && let Some(variant_type_argument_diff) =
                                type_diff(expected_type_argument, type_argument)
                        {
                            let mut error_message: String = format!(
                                "the type arguments of this variant do not match those of the expected type. See the argument at index {}\n",
                                variant_argument_index,
                            );
                            type_diff_into(&mut error_message, 0, &variant_type_argument_diff);
                            error_message.push_str("\nNote that providing type arguments is completely optional here. I would just remove them");
                            errors.push(ErrorNode {
                                range: syntax_name_range(with_start_position_as_ref(name)),
                                message: error_message.into_boxed_str(),
                            });
                        }
                    }
                    let Some(value) = value else {
                        let mut error_message: String =
                            String::from("this variant is missing its associated value of type\n");
                        type_format(&mut error_message, 0, &origin_variant.value);
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: error_message.into_boxed_str(),
                        });
                        return None;
                    };
                    let value = patterns.element(value);
                    let mut expected_value_type = origin_variant.value.clone();
                    type_replace_variables(
                        &expected_type_parameter_replacements,
                        &mut expected_value_type,
                    );
                    let Some(compiled_value) = syntax_pattern_to_rust(
                        value,
                        Some(&expected_value_type),
                        errors,
                        records_used,
                        introduced_variables,
                        type_aliases,
                        choice_types,
                        patterns,
                        types,
                        origins,
                    ) else {
                        return None;
                    };
                    Some(CompiledPattern {
                        rust: syn::Pat::TupleStruct(syn::PatTupleStruct {
                            attrs: vec![],
                            qself: None,
                            path: rust_variant_path,
                            paren_token: syn::token::Paren(syn_span()),
                            elems: std::iter::once(compiled_value.rust).collect(),
                        }),
                        type_: expected_type.clone(),
                        catch: if origin_choice_type.variants.len() == 1 {
                            compiled_value.catch
                        } else {
                            let mut variants: std::collections::BTreeMap<
                                Name,
                                VariantCatch<PatternCatch>,
                            > = origin_choice_type
                                .variants
                                .iter()
                                .map(|variant_info| {
                                    (
                                        variant_info.name.clone(),
                                        VariantCatch::Uncaught { has_value: true },
                                    )
                                })
                                .collect();
                            if let Some(variant_catch) = variants.get_mut(&name.value) {
                                *variant_catch = VariantCatch::Caught(compiled_value.catch);
                            }
                            PatternCatch::Variant(variants)
                        },
                    })
                }
                None => {
                    if type_arguments.len() != origin_variant.type_parameters.len() {
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: format!("incorrect number of type parameters. The project fn has {parameter_count} type parameters, but you only provided {argument_count} as arguments. Type arguments are provided in a space-separated list enclosed in angle brackets after the fn name, like in arena-empty<u32> origin, each type paranthesized if necessary.",
                                parameter_count = origin_variant.type_parameters.len(),
                                argument_count = syntax_type_arguments.len()
                            ).into_boxed_str()
                        });
                        return None;
                    }
                    // can be optimized (probably)
                    let type_parameter_replacements = origin_variant
                        .type_parameters
                        .iter()
                        .zip(type_arguments)
                        .map(|(type_parameter, type_argument): (&Name, Type)| {
                            (
                                type_parameter.as_str(),
                                std::borrow::Cow::Owned(type_argument),
                            )
                        })
                        .collect();
                    let mut variant_type = Type::ChoiceConstruct {
                        name: origin_choice_type_name.clone(),
                        arguments: origin_choice_type
                            .parameters
                            .iter()
                            .map(|parameter| Type::Variable(parameter.clone()))
                            .collect(),
                    };
                    type_replace_variables(&type_parameter_replacements, &mut variant_type);
                    let Some(value) = value else {
                        let mut error_message: String =
                            String::from("this variant is missing its associated value of type\n");
                        type_format(&mut error_message, 0, &origin_variant.value);
                        errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: error_message.into_boxed_str(),
                        });
                        return None;
                    };
                    let value = patterns.element(value);
                    let mut variant_value_type = origin_variant.value.clone();
                    type_replace_variables(&type_parameter_replacements, &mut variant_value_type);
                    let Some(compiled_value) = syntax_pattern_to_rust(
                        value,
                        None,
                        errors,
                        records_used,
                        introduced_variables,
                        type_aliases,
                        choice_types,
                        patterns,
                        types,
                        origins,
                    ) else {
                        return None;
                    };
                    let variant_value_type_without_concrete_value_type_variables =
                        variant_value_type.clone();
                    let mut value_type_variable_replacements = std::collections::HashMap::new();
                    type_collect_variables_that_are_concrete_into(
                        &mut value_type_variable_replacements,
                        &variant_value_type_without_concrete_value_type_variables,
                        &compiled_value.type_,
                    );
                    type_replace_variables(&value_type_variable_replacements, &mut variant_type);
                    type_replace_variables(
                        &value_type_variable_replacements,
                        &mut variant_value_type,
                    );
                    if let Some(variant_value_type_diff) =
                        type_diff(&variant_value_type, &compiled_value.type_)
                    {
                        errors.push(ErrorNode {
                            range: pattern_range(value, patterns, types),
                            message: type_diff_error_message(&variant_value_type_diff)
                                .into_boxed_str(),
                        });
                        return None;
                    }
                    Some(CompiledPattern {
                        rust: syn::Pat::TupleStruct(syn::PatTupleStruct {
                            attrs: vec![],
                            qself: None,
                            path: rust_variant_path,
                            paren_token: syn::token::Paren(syn_span()),
                            elems: std::iter::once(compiled_value.rust).collect(),
                        }),
                        type_: variant_type,
                        catch: if origin_choice_type.variants.len() == 1 {
                            compiled_value.catch
                        } else {
                            let mut variants: std::collections::BTreeMap<
                                Name,
                                VariantCatch<PatternCatch>,
                            > = origin_choice_type
                                .variants
                                .iter()
                                .map(|variant_info| {
                                    (
                                        variant_info.name.clone(),
                                        VariantCatch::Uncaught { has_value: true },
                                    )
                                })
                                .collect();
                            if let Some(variant_catch) = variants.get_mut(&name.value) {
                                *variant_catch = VariantCatch::Caught(compiled_value.catch);
                            }
                            PatternCatch::Variant(variants)
                        },
                    })
                }
            }
        }
        SyntaxPattern::Record {
            ampersand_start: _,
            fields,
        } => {
            let mut maybe_type_fields: Option<Vec<TypeField>> =
                Some(Vec::with_capacity(fields.len()));
            let mut field_catches: std::collections::BTreeMap<Name, PatternCatch> =
                std::collections::BTreeMap::new();
            let mut rust_fields: syn::punctuated::Punctuated<syn::FieldPat, syn::token::Comma> =
                syn::punctuated::Punctuated::new();
            'converting_fields: for field in fields {
                let Some(field_value) = &field.value else {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from("missing field value after this field name"),
                    });
                    return None;
                };
                if maybe_type_fields.as_ref().is_some_and(|type_fields| {
                    type_fields
                        .iter()
                        .any(|type_field| type_field.name == field.name.value)
                }) {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from(
                            "a field with this name already exists in the record pattern",
                        ),
                    });
                    continue 'converting_fields;
                }
                let maybe_expected_type_record =
                    expected_type.and_then(|expected_type| match expected_type {
                        Type::Variable(_) | Type::ChoiceConstruct { .. } => None,
                        Type::Record(type_fields) => Some(type_fields),
                    });
                let compiled_field_value = syntax_pattern_to_rust(
                    field_value,
                    maybe_expected_type_record.and_then(|expected_record_type| {
                        // TODO report if this is none
                        expected_record_type
                            .iter()
                            .find(|expected_field| expected_field.name == field.name.value)
                            .map(|expected_field| &expected_field.value)
                    }),
                    errors,
                    records_used,
                    introduced_variables,
                    type_aliases,
                    choice_types,
                    patterns,
                    types,
                    origins,
                );
                let Some(compiled_field_value) = compiled_field_value else {
                    return None;
                };
                if let Some(ref mut type_fields) = maybe_type_fields {
                    type_fields.push(TypeField {
                        name: field.name.value.clone(),
                        value: compiled_field_value.type_,
                    });
                }
                field_catches.insert(field.name.value.clone(), compiled_field_value.catch);
                rust_fields.push(syn::FieldPat {
                    attrs: vec![],
                    member: syn::Member::Named(syn_ident(&name_to_lowercase_rust(
                        &field.name.value,
                    ))),
                    colon_token: Some(syn::token::Colon(syn_span())),
                    pat: Box::new(compiled_field_value.rust),
                });
            }
            let Some(type_fields) = maybe_type_fields else {
                return None;
            };
            // TODO report if diff maybe_expected_type_record has additional fields
            records_used.insert(sorted_field_names(
                type_fields.iter().map(|field| &field.name),
            ));
            Some(CompiledPattern {
                rust: syn::Pat::Struct(syn::PatStruct {
                    attrs: vec![],
                    qself: None,
                    path: syn_path_reference([&field_names_to_rust_record_struct_name(
                        type_fields.iter().map(|field| &field.name),
                    )]),
                    brace_token: syn::token::Brace(syn_span()),
                    fields: rust_fields,
                    rest: None,
                }),
                type_: Type::Record(type_fields),
                catch: if field_catches
                    .iter()
                    .all(|(_, field_value_catch)| field_value_catch == &PatternCatch::Exhaustive)
                {
                    PatternCatch::Exhaustive
                } else {
                    PatternCatch::Record(field_catches)
                },
            })
        }
        SyntaxPattern::Parenthesized {
            open_paren_start,
            inner,
            closed_paren_start,
        } => match inner {
            None => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: *open_paren_start,
                        end: closed_paren_start
                            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
                            .unwrap_or_else(|| symbol_end(*open_paren_start, "(")),
                    },
                    message: Box::from("missing pattern in parens between (here)"),
                });
                None
            }
            Some(inner) => syntax_pattern_to_rust(
                patterns.element(inner),
                expected_type,
                errors,
                records_used,
                introduced_variables,
                type_aliases,
                choice_types,
                patterns,
                types,
                origins,
            ),
        },
    }
}
// TODO pack into struct
pub fn choice_type_containing_variant<'a>(
    choice_types: &'a std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    variant_name: &str,
) -> Option<(
    &'a Name,
    &'a CompiledChoiceTypeInfo,
    &'a CompiledVariantInfo,
)> {
    choice_types
        .iter()
        .find_map(|(choice_type_name, choice_type)| {
            choice_type
                .variants
                .iter()
                .find(|variant| variant.name == variant_name)
                .map(|variant| (choice_type_name, choice_type, variant))
        })
}

struct CompiledExpression {
    rust: syn::Expr,
    type_: Option<Type>,
}
#[derive(Clone, Debug)]
struct PatternVariableCompileInfo {
    origin_start: lsp_types::Position,
    type_: Option<Type>,
}
#[derive(Clone, Copy, Debug)]
pub struct OriginCompileInfo {
    origin_start: lsp_types::Position,
}
fn syntax_expression_to_rust<'a, Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    choice_types: &std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
    project_fns: &std::collections::HashMap<Name, CompiledProjectFnInfo>,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    pattern_variables: &mut std::collections::HashMap<&'a Name, PatternVariableCompileInfo>,
    used_pattern_variables: &mut std::collections::HashMap<
        &'a Name,
        /* start */ lsp_types::Position,
    >,
    origins: &mut std::collections::HashMap<&'a Name, OriginCompileInfo>,
    used_origin_variables: &mut std::collections::HashMap<
        &'a Name,
        /* start */ lsp_types::Position,
    >,
    expression: &'a SyntaxExpression<Expressions, Patterns, Types>,
) -> CompiledExpression {
    match expression {
        SyntaxExpression::Number { value, type_ } => match type_ {
            None => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: value.start,
                        end: position_add_characters(value.start, value.value.len() as u32),
                    },
                    message: Box::from("missing type after this number. Each number requires an explicit type to know its precision and range, like 0 u32 or 0 f32 (if necessary parenthesized)"),
                });
                CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                }
            }
            Some(syntax_type) => {
                let Some(type_) = syntax_type_to_type(
                    syntax_type,
                    errors,
                    type_aliases,
                    choice_types,
                    types,
                    origins,
                ) else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                let maybe_compiled = match &type_ {
                    Type::ChoiceConstruct { name, arguments: _ } => match name.as_str() {
                        "p32" => match name.parse::<std::num::NonZeroU32>() {
                            Ok(number) => {
                                let rust_number_predecessor = syn::Expr::Lit(syn::ExprLit {
                                    attrs: vec![],
                                    lit: syn::Lit::Int(syn::LitInt::new(
                                        &((number.get() - 1).to_string() + "u32"),
                                        syn_span(),
                                    )),
                                });
                                // there is no native NonZeroU32 int literal (AFAIK),
                                // so we do saturating_add(NonZeroU32::MIN /* = 1 */, p32 - 1)
                                // which should optimize back into NonZeroU32::new_unchecked(p32)
                                Some(syn::Expr::Call(syn::ExprCall {
                                    attrs: vec![],
                                    func: Box::new(syn_expr_reference([
                                        "std",
                                        "num",
                                        "NonZeroU32",
                                        "saturating_add",
                                    ])),
                                    paren_token: syn::token::Paren(syn_span()),
                                    args: [
                                        syn_expr_reference(["std", "num", "NonZeroU32", "MIN"]),
                                        rust_number_predecessor,
                                    ]
                                    .into_iter()
                                    .collect(),
                                }))
                            }
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as a p32: {parse_error}"
                                    )),
                                });
                                Some(syn_expr_todo())
                            }
                        },
                        "u32" => match name.parse::<u32>() {
                            Ok(number) => Some(syn::Expr::Lit(syn::ExprLit {
                                attrs: vec![],
                                lit: syn::Lit::Int(syn::LitInt::new(
                                    &(number.to_string() + "u32"),
                                    syn_span(),
                                )),
                            })),
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as an u32: {parse_error}"
                                    )),
                                });
                                Some(syn_expr_todo())
                            }
                        },
                        "i32" => match name.parse::<i32>() {
                            Ok(number) => Some(syn::Expr::Lit(syn::ExprLit {
                                attrs: vec![],
                                lit: syn::Lit::Int(syn::LitInt::new(
                                    &(number.to_string() + "i32"),
                                    syn_span(),
                                )),
                            })),
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as an i32: {parse_error}"
                                    )),
                                });
                                Some(syn_expr_todo())
                            }
                        },
                        "f32" => match name.parse::<f32>() {
                            Ok(number) => Some(syn::Expr::Lit(syn::ExprLit {
                                attrs: vec![],
                                lit: syn::Lit::Float(syn::LitFloat::new(
                                    &(number.to_string() + "f32"),
                                    syn_span(),
                                )),
                            })),
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as an f32: {parse_error}"
                                    )),
                                });
                                Some(syn_expr_todo())
                            }
                        },
                        _ => None,
                    },
                    _ => None,
                };
                match maybe_compiled {
                    None => {
                        errors.push(ErrorNode {
                            range: lsp_types::Range {
                                start: value.start,
                                end: position_add_characters(value.start, value.value.len() as u32),
                            },
                            message: Box::from("the type after this number is not a number type. The possible types are: p32 u32 i32 f32"),
                        });
                        CompiledExpression {
                            rust: syn_expr_todo(),
                            type_: None,
                        }
                    }
                    Some(compiled) => CompiledExpression {
                        rust: compiled,
                        type_: Some(type_),
                    },
                }
            }
        },
        SyntaxExpression::Char {
            open_quote_start,
            content,
            content_end,
            closed_quote_exists,
        } => CompiledExpression {
            type_: Some(type_char),
            rust: match *content {
                None => {
                    errors.push(ErrorNode {
                        range: lsp_types::Range {
                            start: *open_quote_start,
                            end: if *closed_quote_exists {
                                symbol_end(*content_end, "'")
                            } else {
                                *content_end
                            },
                        },
                        message: Box::from("missing character between 'here'"),
                    });
                    syn_expr_todo()
                }
                Some(char) => syn::Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Char(syn::LitChar::new(char, syn_span())),
                }),
            },
        },
        SyntaxExpression::Str {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
        } => CompiledExpression {
            rust: syn::Expr::Lit(syn::ExprLit {
                attrs: vec![],
                lit: syn::Lit::Str(syn::LitStr::new(content, syn_span())),
            }),
            type_: Some(type_str),
        },
        SyntaxExpression::VariableOrCall {
            name,
            type_arguments,
            argument: syntax_argument,
        } => {
            if let Some(_origin_info) = origins.get_mut(&name.value) {
                let maybe_existing_origin_variable_use_start =
                    used_origin_variables.insert(&name.value, name.start);
                if let Some(existing_origin_variable_use_start) =
                    maybe_existing_origin_variable_use_start
                {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(name)),
                        message: format!("this origin variable is already used earlier starting at {}. Each value can only be used once, that includes origins. Each collection needs its own origin", position_to_string(existing_origin_variable_use_start)).into_boxed_str(),
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                }
                let rust_reference: syn::Expr =
                    syn_expr_reference([&name_to_lowercase_rust(&name.value)]);
                if let Some(type_arguments) = type_arguments {
                    errors.push(ErrorNode {
                        range: lsp_types::Range {
                            start: type_arguments.open_angle_start,
                            end: angled_type_arguments_end(type_arguments, types),
                        },
                        message: Box::from(
                            "type arguments on an origin make no sense. Remove them",
                        ),
                    })
                }
                if let Some(argument) = syntax_argument {
                    errors.push(ErrorNode {
                        range: expression_range(
                            expressions.element(argument),
                            expressions,
                            patterns,
                            types,
                        ),
                        message: Box::from(
                            "calling an origin with an argument makes no sense. Remove this argument",
                        ),
                    })
                }
                CompiledExpression {
                    rust: rust_reference,
                    type_: Some(Type::ChoiceConstruct {
                        name: name.value.clone(),
                        arguments: vec![],
                    }),
                }
            } else if let Some(variable_info) = pattern_variables.get_mut(&name.value) {
                let maybe_existing_pattern_variable_use_start =
                    used_pattern_variables.insert(&name.value, name.start);
                if let Some(existing_pattern_variable_use_start) =
                    maybe_existing_pattern_variable_use_start
                {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(name)),
                        message: format!("this variable is already used earlier starting at {}. Each value can only be used once, even simple numbers etc. To duplicate the value, use the helpers like u32-dup, char-dup or create your own dup helpers", position_to_string(existing_pattern_variable_use_start)).into_boxed_str(),
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                }
                let rust_reference: syn::Expr =
                    syn_expr_reference([&name_to_lowercase_rust(&name.value)]);
                if let Some(type_arguments) = type_arguments {
                    errors.push(ErrorNode {
                        range: lsp_types::Range {
                            start: type_arguments.open_angle_start,
                            end: angled_type_arguments_end(type_arguments, types),
                        },
                        message: Box::from(
                            "type arguments on a local variable make no sense. Remove them",
                        ),
                    })
                }
                let Some(variable_type) = variable_info.type_.clone() else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                match syntax_argument {
                    None => CompiledExpression {
                        rust: rust_reference,
                        type_: Some(variable_type),
                    },
                    Some(argument) => {
                        let syntax_argument = expressions.element(argument);
                        let compiled_argument: CompiledExpression = syntax_expression_to_rust(
                            errors,
                            records_used,
                            type_aliases,
                            choice_types,
                            project_fns,
                            expressions,
                            patterns,
                            types,
                            pattern_variables,
                            used_pattern_variables,
                            origins,
                            used_origin_variables,
                            syntax_argument,
                        );
                        let Some(argument_type) = compiled_argument.type_ else {
                            return CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            };
                        };
                        let Type::ChoiceConstruct {
                            name: variable_type_name,
                            arguments: variable_type_arguments,
                        } = variable_type
                        else {
                            errors.push(ErrorNode { range: syntax_name_range(with_start_position_as_ref(name)), message: Box::from("calling a variable whose type is not a function. Maybe you forgot some parens or similar?") });
                            return CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            };
                        };
                        if variable_type_name != "fn" {
                            errors.push(ErrorNode { range: syntax_name_range(with_start_position_as_ref(name)), message: Box::from("calling a variable whose type is not a function. Maybe you forgot some parens or similar?") });
                            return CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            };
                        }
                        let [variable_type_input, variable_type_output] =
                            variable_type_arguments.as_slice()
                        else {
                            return CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            };
                        };
                        if let Some(argument_variable_input_type_diff) =
                            type_diff(variable_type_input, &argument_type)
                        {
                            errors.push(ErrorNode {
                                range: expression_range(
                                    syntax_argument,
                                    expressions,
                                    patterns,
                                    types,
                                ),
                                message: type_diff_error_message(
                                    &argument_variable_input_type_diff,
                                )
                                .into_boxed_str(),
                            });
                            return CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            };
                        }
                        CompiledExpression {
                            rust: syn::Expr::Call(syn::ExprCall {
                                attrs: vec![],
                                func: Box::new(syn_expr_reference([&name_to_lowercase_rust(
                                    &name.value,
                                )])),
                                paren_token: syn::token::Paren(syn_span()),
                                args: std::iter::once(compiled_argument.rust)
                                    .into_iter()
                                    .collect(),
                            }),
                            type_: Some(variable_type_output.clone()),
                        }
                    }
                }
            } else {
                let Some(project_fn_info) = project_fns.get(name.value.as_str()) else {
                    errors.push(ErrorNode { range: syntax_name_range(with_start_position_as_ref(name)), message: Box::from("unknown name. No project fn or local variable has this name. Note that a local fn can not refer to any variable from the outside. Otherwise check for typos.") });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                let Some((project_fn_parameter_type, project_fn_result_type)) = project_fn_info
                    .parameter_type
                    .as_ref()
                    .zip(project_fn_info.result_type.as_ref())
                else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                let syntax_type_arguments = match type_arguments {
                    None => &[],
                    Some(type_arguments) => {
                        types.opt_span_slice(core::Opt::from_option(type_arguments.types.as_ref()))
                    }
                };
                if syntax_type_arguments.len() != project_fn_info.type_parameters.len() {
                    errors.push(ErrorNode {
                            range: syntax_name_range(with_start_position_as_ref(name)),
                            message: format!("incorrect number of type parameters. The project fn has {parameter_count} type parameters, but you only provided {argument_count} as arguments. Type arguments are provided in a space-separated list enclosed in angle brackets after the fn name, like in arena-empty<u32> origin, each type paranthesized if necessary.",
                                parameter_count = project_fn_info.type_parameters.len(),
                                argument_count = syntax_type_arguments.len()
                            ).into_boxed_str()
                        });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                }
                let mut type_arguments = Vec::new();
                for syntax_type_argument in syntax_type_arguments {
                    let Some(type_argument) = syntax_type_to_type(
                        syntax_type_argument,
                        errors,
                        type_aliases,
                        choice_types,
                        types,
                        origins,
                    ) else {
                        return CompiledExpression {
                            rust: syn_expr_todo(),
                            type_: None,
                        };
                    };
                    type_arguments.push(type_argument);
                }
                let type_parameter_replacements = project_fn_info
                    .type_parameters
                    .iter()
                    .zip(type_arguments)
                    .map(|(type_parameter, type_argument)| {
                        (
                            type_parameter.as_str(),
                            std::borrow::Cow::Owned(type_argument),
                        )
                    })
                    .collect();
                let mut fn_parameter_type = project_fn_parameter_type.clone();
                let mut fn_result_type = project_fn_result_type.clone();
                type_replace_variables(&type_parameter_replacements, &mut fn_parameter_type);
                type_replace_variables(&type_parameter_replacements, &mut fn_result_type);
                let rust_reference: syn::Expr =
                    syn_expr_reference([&name_to_lowercase_rust(&name.value)]);
                match syntax_argument {
                    None => CompiledExpression {
                        rust: rust_reference,
                        type_: Some(type_fn(fn_parameter_type, fn_result_type)),
                    },
                    Some(syntax_argument) => {
                        let syntax_argument = expressions.element(syntax_argument);
                        let compiled_argument: CompiledExpression = syntax_expression_to_rust(
                            errors,
                            records_used,
                            type_aliases,
                            choice_types,
                            project_fns,
                            expressions,
                            patterns,
                            types,
                            pattern_variables,
                            used_pattern_variables,
                            origins,
                            used_origin_variables,
                            syntax_argument,
                        );
                        let Some(argument_type) = compiled_argument.type_ else {
                            return CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            };
                        };
                        let mut argument_type_variable_replacements =
                            std::collections::HashMap::new();
                        type_collect_variables_that_are_concrete_into(
                            &mut argument_type_variable_replacements,
                            &fn_parameter_type,
                            &argument_type,
                        );
                        let mut expected_argument_type = fn_parameter_type.clone();
                        type_replace_variables(
                            &argument_type_variable_replacements,
                            &mut expected_argument_type,
                        );
                        let mut result_type = fn_result_type.clone();
                        type_replace_variables(
                            &argument_type_variable_replacements,
                            &mut result_type,
                        );
                        if let Some(argument_variable_input_type_diff) =
                            type_diff(&expected_argument_type, &argument_type)
                        {
                            errors.push(ErrorNode {
                                range: expression_range(
                                    syntax_argument,
                                    expressions,
                                    patterns,
                                    types,
                                ),
                                message: type_diff_error_message(
                                    &argument_variable_input_type_diff,
                                )
                                .into_boxed_str(),
                            });
                            return CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            };
                        }
                        CompiledExpression {
                            rust: syn::Expr::Call(syn::ExprCall {
                                attrs: vec![],
                                func: Box::new(syn_expr_reference([&name_to_lowercase_rust(
                                    &name.value,
                                )])),
                                paren_token: syn::token::Paren(syn_span()),
                                args: std::iter::once(compiled_argument.rust)
                                    .into_iter()
                                    .collect(),
                            }),
                            type_: Some(result_type),
                        }
                    }
                }
            }
        }
        SyntaxExpression::Variant {
            name,
            type_arguments,
            value,
        } => {
            let Some((origin_choice_type_name, origin_choice_type, origin_variant)) =
                choice_type_containing_variant(choice_types, &name.value)
            else {
                errors.push(ErrorNode { range: syntax_name_range(with_start_position_as_ref(name)), message: Box::from("this variant name is not declared in any choice type. Check for typos or declare this variant") });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let syntax_type_arguments = match type_arguments {
                None => &[],
                Some(type_arguments) => {
                    types.opt_span_slice(core::Opt::from_option(type_arguments.types.as_ref()))
                }
            };
            if syntax_type_arguments.len() != origin_variant.type_parameters.len() {
                errors.push(ErrorNode {
                    range: syntax_name_range(with_start_position_as_ref(name)),
                    message: format!("incorrect number of type parameters. The project fn has {parameter_count} type parameters, but you only provided {argument_count} as arguments. Type arguments are provided in a space-separated list enclosed in angle brackets after the fn name, like in arena-empty<u32> origin, each type paranthesized if necessary.",
                        parameter_count = origin_variant.type_parameters.len(),
                        argument_count = syntax_type_arguments.len()
                    ).into_boxed_str()
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            }
            let mut type_arguments = Vec::new();
            for syntax_type_argument in syntax_type_arguments {
                let Some(type_argument) = syntax_type_to_type(
                    syntax_type_argument,
                    errors,
                    type_aliases,
                    choice_types,
                    types,
                    origins,
                ) else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                type_arguments.push(type_argument);
            }
            // can be optimized (probably)
            let type_parameter_replacements = origin_variant
                .type_parameters
                .iter()
                .zip(type_arguments)
                .map(|(type_parameter, type_argument): (&Name, Type)| {
                    (
                        type_parameter.as_str(),
                        std::borrow::Cow::Owned(type_argument),
                    )
                })
                .collect();
            let mut variant_type = Type::ChoiceConstruct {
                name: origin_choice_type_name.clone(),
                arguments: origin_choice_type
                    .parameters
                    .iter()
                    .map(|parameter| Type::Variable(parameter.clone()))
                    .collect(),
            };
            type_replace_variables(&type_parameter_replacements, &mut variant_type);
            let rust_variant_expr_path = syn::Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: syn_path_reference([
                    &name_to_uppercase_rust(origin_choice_type_name),
                    &name_to_uppercase_rust(&name.value),
                ]),
            });
            let Some(value) = value else {
                let mut error_message: String =
                    String::from("this variant is missing its associated value of type\n");
                type_format(&mut error_message, 0, &origin_variant.value);
                errors.push(ErrorNode {
                    range: syntax_name_range(with_start_position_as_ref(name)),
                    message: error_message.into_boxed_str(),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let value = expressions.element(value);
            let mut variant_value_type = origin_variant.value.clone();
            type_replace_variables(&type_parameter_replacements, &mut variant_value_type);
            let CompiledExpression {
                type_: Some(compiled_value_type),
                rust: compiled_value_rust,
            } = syntax_expression_to_rust(
                errors,
                records_used,
                type_aliases,
                choice_types,
                project_fns,
                expressions,
                patterns,
                types,
                pattern_variables,
                used_pattern_variables,
                origins,
                used_origin_variables,
                value,
            )
            else {
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let variant_value_type_without_concrete_value_type_variables =
                variant_value_type.clone();
            let mut value_type_variable_replacements = std::collections::HashMap::new();
            type_collect_variables_that_are_concrete_into(
                &mut value_type_variable_replacements,
                &variant_value_type_without_concrete_value_type_variables,
                &compiled_value_type,
            );
            type_replace_variables(&value_type_variable_replacements, &mut variant_type);
            type_replace_variables(&value_type_variable_replacements, &mut variant_value_type);
            if let Some(variant_value_type_diff) =
                type_diff(&variant_value_type, &compiled_value_type)
            {
                errors.push(ErrorNode {
                    range: expression_range(value, expressions, patterns, types),
                    message: type_diff_error_message(&variant_value_type_diff).into_boxed_str(),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            }
            CompiledExpression {
                rust: syn::Expr::Call(syn::ExprCall {
                    attrs: vec![],
                    func: Box::new(rust_variant_expr_path),
                    paren_token: syn::token::Paren(syn_span()),
                    args: std::iter::once(compiled_value_rust).collect(),
                }),
                type_: Some(variant_type),
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            parameter,
            result,
        } => {
            let Some(parameter) = parameter else {
                errors.push(ErrorNode {
                    range: symbol_range(*fn_keyword_start, "fn"),
                    message: Box::from("missing parameter after fn. An example of a local fn expression is fn (n u32) u32-add & (a n) (b 1 u32)"),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Some(result) = result else {
                errors.push(ErrorNode {
                    range: symbol_range(*fn_keyword_start, "fn"),
                    message: Box::from("missing result after fn ..pattern.. here"),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let mut parameter_introduced_variables: std::collections::HashMap<
                &Name,
                PatternVariableCompileInfo,
            > = std::collections::HashMap::new();
            let Some(compiled_parameter) = syntax_pattern_to_rust(
                parameter,
                None,
                errors,
                records_used,
                &mut parameter_introduced_variables,
                type_aliases,
                choice_types,
                patterns,
                types,
                origins,
            ) else {
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let mut result_used_pattern_variables = std::collections::HashMap::new();
            let mut result_used_origin_variables = std::collections::HashMap::new();
            let compiled_result = syntax_expression_to_rust(
                errors,
                records_used,
                type_aliases,
                choice_types,
                project_fns,
                expressions,
                patterns,
                types,
                &mut parameter_introduced_variables,
                &mut result_used_pattern_variables,
                origins,
                &mut result_used_origin_variables,
                expressions.element(result),
            );
            if let Some((use_of_outside_origin_name, use_of_outside_origin_start)) =
                result_used_origin_variables
                    .into_iter()
                    .find(|(result_used_origin_variable, _)| {
                        origins.contains_key(result_used_origin_variable)
                    })
            {
                errors.push(ErrorNode {
                    range: syntax_name_range(WithStartPosition {
                        value: use_of_outside_origin_name,
                        start: use_of_outside_origin_start,
                    }),
                    message: Box::from("use of an origin variable that is created outside of a local fn. Local fns do not capture variables, so pass them in via arguments explicitly"),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            }
            let Some(actual_result_type) = compiled_result.type_ else {
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            for parameter_introduced_binding_name in parameter_introduced_variables.keys() {
                push_error_if_introduced_variable_collides_or_is_unused(
                    errors,
                    project_fns,
                    &std::collections::HashMap::new(),
                    parameter_introduced_binding_name,
                    result_used_pattern_variables
                        .get(parameter_introduced_binding_name)
                        .copied(),
                );
            }
            let mut type_variables = std::collections::HashSet::new();
            type_variables_into(&mut type_variables, &compiled_parameter.type_);
            type_variables_into(&mut type_variables, &actual_result_type);
            CompiledExpression {
                rust: syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: syn::Block {
                        brace_token: syn::token::Brace(syn_span()),
                        stmts: vec![
                            syn::Stmt::Item(syn::Item::Fn(syn::ItemFn {
                                attrs: vec![],
                                vis: syn::Visibility::Inherited,
                                sig: syn::Signature {
                                    constness: None,
                                    asyncness: None,
                                    unsafety: None,
                                    abi: None,
                                    fn_token: syn::token::Fn(syn_span()),
                                    ident: syn_ident(local_unnamed_function_name),
                                    generics: syn::Generics {
                                        lt_token: Some(syn::token::Lt(syn_span())),
                                        params: type_variables
                                            .iter()
                                            .map(|field_name| {
                                                syn::GenericParam::Type(syn::TypeParam {
                                                    attrs: vec![],
                                                    ident: syn_ident(&type_variable_to_rust(
                                                        field_name,
                                                    )),
                                                    colon_token: None,
                                                    bounds: syn::punctuated::Punctuated::new(),
                                                    eq_token: None,
                                                    default: None,
                                                })
                                            })
                                            .collect(),
                                        gt_token: Some(syn::token::Gt(syn_span())),
                                        where_clause: None,
                                    },
                                    paren_token: syn::token::Paren(syn_span()),
                                    inputs: std::iter::once(syn::FnArg::Typed(syn::PatType {
                                        attrs: vec![],
                                        pat: Box::new(compiled_parameter.rust),
                                        colon_token: syn::token::Colon(syn_span()),
                                        ty: Box::new(type_to_rust(&compiled_parameter.type_)),
                                    }))
                                    .collect(),
                                    variadic: None,
                                    output: syn::ReturnType::Type(
                                        syn::token::RArrow(syn_span()),
                                        Box::new(type_to_rust(&actual_result_type)),
                                    ),
                                },
                                block: Box::new(syn_spread_expr_block(compiled_result.rust)),
                            })),
                            syn::Stmt::Expr(
                                syn_expr_reference([local_unnamed_function_name]),
                                None,
                            ),
                        ],
                    },
                }),
                type_: Some(Type::ChoiceConstruct {
                    name: Name::const_new("fn"),
                    arguments: vec![compiled_parameter.type_, actual_result_type],
                }),
            }
        }
        SyntaxExpression::Record {
            ampersand_start: _,
            fields,
        } => {
            let (rust_fields, field_maybe_types): (
                syn::punctuated::Punctuated<syn::FieldValue, syn::token::Comma>,
                Vec<(Name, Option<Type>)>,
            ) = fields
                .iter()
                .filter_map(|field| {
                    let compiled_field_value: CompiledExpression = match &field.value {
                        None => {
                            errors.push(ErrorNode {
                                range: syntax_name_range(with_start_position_as_ref(&field.name)),
                                message: Box::from(
                                    "missing field value expression after this field name",
                                ),
                            });
                            CompiledExpression {
                                rust: syn_expr_todo(),
                                type_: None,
                            }
                        }
                        Some(field_value) => syntax_expression_to_rust(
                            errors,
                            records_used,
                            type_aliases,
                            choice_types,
                            project_fns,
                            expressions,
                            patterns,
                            types,
                            pattern_variables,
                            used_pattern_variables,
                            origins,
                            used_origin_variables,
                            field_value,
                        ),
                    };
                    Some((
                        syn::FieldValue {
                            attrs: vec![],
                            member: syn::Member::Named(syn_ident(&name_to_lowercase_rust(
                                &field.name.value,
                            ))),
                            colon_token: Some(syn::token::Colon(syn_span())),
                            expr: compiled_field_value.rust,
                        },
                        (field.name.value.clone(), compiled_field_value.type_),
                    ))
                })
                .unzip();
            let field_names: Vec<Name> =
                sorted_field_names(field_maybe_types.iter().map(|(field_name, _)| field_name));
            let rust_struct_name: String =
                field_names_to_rust_record_struct_name(field_names.iter());
            records_used.insert(field_names);
            CompiledExpression {
                rust: syn::Expr::Struct(syn::ExprStruct {
                    attrs: vec![],
                    qself: None,
                    path: syn_path_reference([&rust_struct_name]),
                    brace_token: syn::token::Brace(syn_span()),
                    fields: rust_fields,
                    dot2_token: None,
                    rest: None,
                }),
                type_: field_maybe_types
                    .into_iter()
                    .map(|(name, maybe_value_type)| {
                        maybe_value_type.map(|value_type| TypeField {
                            name: name,
                            value: value_type,
                        })
                    })
                    .collect::<Option<Vec<TypeField>>>()
                    .map(Type::Record),
            }
        }
        SyntaxExpression::Parenthesized {
            open_paren_start,
            inner,
            closed_paren_start,
        } => match inner {
            None => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: *open_paren_start,
                        end: closed_paren_start
                            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
                            .unwrap_or_else(|| symbol_end(*open_paren_start, "(")),
                    },
                    message: Box::from("missing expression in parens between (here)"),
                });
                CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                }
            }
            Some(inner) => syntax_expression_to_rust(
                errors,
                records_used,
                type_aliases,
                choice_types,
                project_fns,
                expressions,
                patterns,
                types,
                pattern_variables,
                used_pattern_variables,
                origins,
                used_origin_variables,
                expressions.element(inner),
            ),
        },
        SyntaxExpression::Commented {
            comments,
            expression,
        } => match expression {
            None => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: comments.line0.start,
                        end: comments_end(comments),
                    },
                    message: Box::from(
                        "missing expression after comments # your comment \\n ..here..",
                    ),
                });
                CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                }
            }
            Some(expression) => syntax_expression_to_rust(
                errors,
                records_used,
                type_aliases,
                choice_types,
                project_fns,
                expressions,
                patterns,
                types,
                pattern_variables,
                used_pattern_variables,
                origins,
                used_origin_variables,
                expressions.element(expression),
            ),
        },
        SyntaxExpression::Query {
            colon_start,
            queried,
            cases,
        } => {
            let Some(queried) = queried else {
                errors.push(ErrorNode {
                    range: symbol_range(*colon_start, ":"),
                    message: Box::from("missing queried expression after this colon. A full query could look like :option ((Present n) n) (Absent 0 u32)")
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let queried = expressions.element(queried);
            let Some((case0, case1_up)) = cases.split_first() else {
                errors.push(ErrorNode {
                    range: symbol_range(*colon_start, ":"),
                    message: Box::from("missing case(s) after the queried expression. Cases can be (pattern result-expression) or pattern result-expression for the last one. A full query could look like :option ((Present n) n) (Absent 0 u32)")
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let CompiledExpression {
                rust: compiled_queried_rust,
                type_: Some(compiled_queried_type),
            } = syntax_expression_to_rust(
                errors,
                records_used,
                type_aliases,
                choice_types,
                project_fns,
                expressions,
                patterns,
                types,
                pattern_variables,
                used_pattern_variables,
                origins,
                used_origin_variables,
                queried,
            )
            else {
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Some(case0_result) = &case0.result else {
                errors.push(ErrorNode {
                    range: case0.left_angle_start.map(|left_angle_start| symbol_range(left_angle_start, "<")).unwrap_or_else(|| pattern_range(&case0.pattern, patterns, types)),
                    message: Box::from("missing result expression after this query case pattern. Cases can be (pattern result-expression) or pattern result-expression for the last one. A full query could look like :option ((Present n) n) (Absent 0 u32)")
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let mut case0_pattern_introduced_variables: std::collections::HashMap<
                &Name,
                PatternVariableCompileInfo,
            > = std::collections::HashMap::new();
            let Some(case0_pattern_compiled) = syntax_pattern_to_rust(
                &case0.pattern,
                Some(&compiled_queried_type),
                errors,
                records_used,
                &mut case0_pattern_introduced_variables,
                type_aliases,
                choice_types,
                patterns,
                types,
                origins,
            ) else {
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            pattern_variables.extend(
                case0_pattern_introduced_variables
                    .iter()
                    .map(|(binding, info)| (*binding, info.clone())),
            );
            let mut case0_result_used_pattern_variables = std::collections::HashMap::new();
            let mut case0_result_used_origin_variables = std::collections::HashMap::new();
            let CompiledExpression {
                rust: case0_compiled_result_rust,
                type_: Some(query_result_type),
            } = syntax_expression_to_rust(
                errors,
                records_used,
                type_aliases,
                choice_types,
                project_fns,
                expressions,
                patterns,
                types,
                pattern_variables,
                &mut case0_result_used_pattern_variables,
                origins,
                &mut case0_result_used_origin_variables,
                case0_result,
            )
            else {
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            for case0_pattern_introduced_variable in case0_pattern_introduced_variables.keys() {
                push_error_if_introduced_variable_collides_or_is_unused(
                    errors,
                    project_fns,
                    &pattern_variables,
                    case0_pattern_introduced_variable,
                    case0_result_used_pattern_variables.remove(case0_pattern_introduced_variable),
                );
                pattern_variables.remove(case0_pattern_introduced_variable);
            }
            let mut catch = pattern_catch_to_case_patterns_catch(case0_pattern_compiled.catch);
            let mut rust_arms: Vec<syn::Arm> = vec![syn::Arm {
                attrs: vec![],
                pat: case0_pattern_compiled.rust,
                guard: None,
                fat_arrow_token: syn::token::FatArrow(syn_span()),
                body: Box::new(syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: syn_spread_expr_block(case0_compiled_result_rust),
                })),
                comma: None,
            }];
            for (case_index, case) in case1_up
                .iter()
                .enumerate()
                .map(|(i_in_1up, case)| (i_in_1up + 1, case))
            {
                let Some(case_result) = &case.result else {
                    errors.push(ErrorNode {
                        range: case.left_angle_start.map(|left_angle_start| symbol_range(left_angle_start, "<")).unwrap_or_else(||pattern_range(&case0.pattern, patterns, types)),
                        message: Box::from("missing result expression after this query case pattern. Cases can be (pattern result-expression) or pattern result-expression for the last one. A full query could look like :option ((Present n) n) (Absent 0 u32)")
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                let mut case_pattern_introduced_bindings: std::collections::HashMap<
                    &Name,
                    PatternVariableCompileInfo,
                > = std::collections::HashMap::new();
                let Some(case_pattern_compiled) = syntax_pattern_to_rust(
                    &case.pattern,
                    Some(&compiled_queried_type),
                    errors,
                    records_used,
                    &mut case_pattern_introduced_bindings,
                    type_aliases,
                    choice_types,
                    patterns,
                    types,
                    origins,
                ) else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                pattern_variables.extend(
                    case_pattern_introduced_bindings
                        .iter()
                        .map(|(binding, info)| (*binding, info.clone())),
                );
                let mut case_result_used_pattern_variables = std::collections::HashMap::new();
                let mut case_result_used_origin_variables = std::collections::HashMap::new();
                let CompiledExpression {
                    rust: case_compiled_result_rust,
                    type_: Some(case_result_type),
                } = syntax_expression_to_rust(
                    errors,
                    records_used,
                    type_aliases,
                    choice_types,
                    project_fns,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    &mut case_result_used_pattern_variables,
                    origins,
                    &mut case_result_used_origin_variables,
                    case0_result,
                )
                else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                for case_pattern_introduced_variable in case_pattern_introduced_bindings.keys() {
                    push_error_if_introduced_variable_collides_or_is_unused(
                        errors,
                        project_fns,
                        &pattern_variables,
                        case_pattern_introduced_variable,
                        case_result_used_pattern_variables.remove(case_pattern_introduced_variable),
                    );
                    pattern_variables.remove(case_pattern_introduced_variable);
                }
                for (case_result_used_pattern_variable, &case_result_used_pattern_variable_start) in
                    &case_result_used_pattern_variables
                {
                    if case0_result_used_pattern_variables
                        .get(case_result_used_pattern_variable)
                        .is_none()
                    {
                        errors.push(ErrorNode {
                            range: syntax_name_range(WithStartPosition { value: case_result_used_pattern_variable, start: case_result_used_pattern_variable_start }),
                            message: Box::from("this query case pattern variable is not used in the result of the first case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you know that this variable does not need to be handled more explicitly, you can also add a line :variable-name _ to ignore it.")
                        });
                    }
                }
                for (
                    case0_result_used_pattern_variable,
                    &case0_result_used_pattern_variable_start,
                ) in &case0_result_used_pattern_variables
                {
                    if case_result_used_pattern_variables
                        .get(case0_result_used_pattern_variable)
                        .is_none()
                    {
                        errors.push(ErrorNode {
                            range: syntax_name_range(WithStartPosition { value: case0_result_used_pattern_variable, start: case0_result_used_pattern_variable_start }),
                            message: format!("this query case pattern variable is not used in the result of the {} case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you know that this variable does not need to be handled more explicitly, you can also add a line :variable-name _ to ignore it.", index_to_th(case_index)).into_boxed_str()
                        });
                    }
                }
                for (case_result_used_origin_variable, &case_result_used_origin_variable_start) in
                    &case_result_used_origin_variables
                {
                    if case0_result_used_origin_variables
                        .get(case_result_used_origin_variable)
                        .is_none()
                    {
                        errors.push(ErrorNode {
                            range: syntax_name_range(WithStartPosition { value: case_result_used_origin_variable, start: case_result_used_origin_variable_start }),
                            message: Box::from("this query case origin variable is not used in the result of the first case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you know that this variable does not need to be handled more explicitly, you can also add a line :variable-name _ to ignore it.")
                        });
                    }
                }
                for (case0_result_used_origin_variable, &case0_result_used_origin_variable_start) in
                    &case0_result_used_origin_variables
                {
                    if case_result_used_origin_variables
                        .get(case0_result_used_origin_variable)
                        .is_none()
                    {
                        errors.push(ErrorNode {
                            range: syntax_name_range(WithStartPosition { value: case0_result_used_origin_variable, start: case0_result_used_origin_variable_start }),
                            message: format!("this query case origin variable is not used in the result of the {} case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you know that this variable does not need to be handled more explicitly, you can also add a line :variable-name _ to ignore it.", index_to_th(case_index)).into_boxed_str()
                        });
                    }
                }
                if let Some(match_result_case_result_type_diff) =
                    type_diff(&query_result_type, &case_result_type)
                {
                    errors.push(ErrorNode {
                        range: expression_range(case_result, expressions, patterns, types),
                        message: (type_diff_error_message(&match_result_case_result_type_diff)
                            + "\n\nAll query case results must have the same type")
                            .into_boxed_str(),
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: Some(case_result_type),
                    };
                }
                if let Some(queried_pattern_type_diff) =
                    type_diff(&compiled_queried_type, &case_pattern_compiled.type_)
                {
                    errors.push(ErrorNode {
                        range: pattern_range(&case.pattern, patterns, types),
                        message: (type_diff_error_message(&queried_pattern_type_diff)
                            + "\n\nA query case pattern must have the same type as the queried expression")
                                .into_boxed_str(),
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: Some(case_result_type),
                    };
                }
                pattern_catch_merge_with(
                    errors,
                    pattern_range(&case.pattern, patterns, types),
                    &mut catch,
                    case_pattern_compiled.catch,
                );
                rust_arms.push(syn::Arm {
                    attrs: vec![],
                    pat: case_pattern_compiled.rust,
                    guard: None,
                    fat_arrow_token: syn::token::FatArrow(syn_span()),
                    body: Box::new(syn::Expr::Block(syn::ExprBlock {
                        attrs: vec![],
                        label: None,
                        block: syn_spread_expr_block(case_compiled_result_rust),
                    })),
                    comma: None,
                })
            }
            match catch {
                CasePatternsCatch::Exhaustive => {}
                _catch_not_exhaustive => {
                    errors.push(ErrorNode {
                        range: symbol_range(*colon_start, ":"),
                        message: Box::from("inexhaustive pattern match.
A pattern match must cover all possible cases, otherwise the program would need to crash if such a value was matched on.
It might be that a case is not indented enough."),
                    });
                    // _ => todo!() is appended to still make inexhaustive matching compile
                    // and be able to be run, rust will emit a warning
                    rust_arms.push(syn::Arm {
                        attrs: vec![],
                        pat: syn::Pat::Wild(syn::PatWild {
                            attrs: vec![],
                            underscore_token: syn::token::Underscore(syn_span()),
                        }),
                        fat_arrow_token: syn::token::FatArrow(syn_span()),
                        guard: None,
                        body: Box::new(syn_expr_todo()),
                        comma: None,
                    });
                }
            }
            used_pattern_variables.extend(case0_result_used_pattern_variables);
            used_origin_variables.extend(case0_result_used_origin_variables);
            CompiledExpression {
                rust: syn::Expr::Match(syn::ExprMatch {
                    attrs: vec![],
                    match_token: syn::token::Match(syn_span()),
                    expr: Box::new(compiled_queried_rust),
                    brace_token: syn::token::Brace(syn_span()),
                    arms: rust_arms,
                }),
                type_: Some(query_result_type),
            }
        }
        SyntaxExpression::Origin {
            origin_keyword_start,
            name,
            result,
        } => {
            let Some(result) = result else {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: *origin_keyword_start,
                        end: name
                            .as_ref()
                            .map(|name| name_end(with_start_position_as_ref(name)))
                            .unwrap_or_else(|| symbol_end(*origin_keyword_start, "origin")),
                    },
                    message: Box::from("missing expression after origin origin-name ..here.."),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            if let Some(origin_name) = name {
                if let Some(existing_origin_with_same_name) = origins.get(&origin_name.value) {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(origin_name)),
                        message: format!(
                            "an origin with this name already exists at {}",
                            position_to_string(existing_origin_with_same_name.origin_start)
                        )
                        .into_boxed_str(),
                    });
                } else if choice_types.contains_key(&origin_name.value) {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(origin_name)),
                        message: Box::from(
                            "a choice type with this name already exists. Rename this origin",
                        ),
                    });
                } else if core_choice_types.contains_key(&origin_name.value) {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(origin_name)),
                        message: Box::from(
                            "a core choice type with this name already exists. Rename this origin",
                        ),
                    });
                } else if type_aliases.contains_key(&origin_name.value) {
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(origin_name)),
                        message: Box::from(
                            "a type alias with this name already exists. Rename this origin",
                        ),
                    });
                } else {
                    origins.insert(
                        &origin_name.value,
                        OriginCompileInfo {
                            origin_start: origin_name.start,
                        },
                    );
                }
            }
            let result_compiled = syntax_expression_to_rust(
                errors,
                records_used,
                type_aliases,
                choice_types,
                project_fns,
                expressions,
                patterns,
                types,
                pattern_variables,
                used_pattern_variables,
                origins,
                used_origin_variables,
                expressions.element(result),
            );
            let Some(origin_name) = name else {
                return result_compiled;
            };
            if let Some(result_type) = &result_compiled.type_ {
                if type_references_origin(result_type, &origin_name.value) {
                    let mut type_string = String::new();
                    type_format(&mut type_string, 0, result_type);
                    errors.push(ErrorNode {
                        range: syntax_name_range(with_start_position_as_ref(origin_name)),
                        message: format!(
                            "the type of the resulting expression references this origin:\n{}. This is not allowed as it would allow creating multiple collections with the same origin. Move this origin creation to before the outer expression and/or pass the origin as an argument",
                            type_string
                        ).into_boxed_str(),
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                }
            }
            if used_origin_variables.remove(&origin_name.value).is_none() {
                errors.push(ErrorNode {
                    range: syntax_name_range(with_start_position_as_ref(origin_name)),
                    message: Box::from(
                        "this origin is never used as a variable. Use it or remove it",
                    ),
                });
                return result_compiled;
            }
            CompiledExpression {
                rust: syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: syn_spread_expr_block(syn::Expr::Macro(syn::ExprMacro {
                        attrs: vec![],
                        mac: syn::Macro {
                            path: syn_path_reference(["origin_new"]),
                            bang_token: syn::token::Not(syn_span()),
                            delimiter: syn::MacroDelimiter::Paren(syn::token::Paren(syn_span())),
                            tokens: {
                                let mut token_stream = proc_macro2::TokenStream::new();
                                proc_macro2::TokenStream::append_separated(
                                    &mut token_stream,
                                    [
                                        origin_name.value.as_str(),
                                        &name_to_uppercase_rust(origin_name.value.as_str()),
                                    ],
                                    syn::token::Comma(syn_span()),
                                );
                                token_stream
                            },
                        },
                    })),
                }),
                type_: result_compiled.type_,
            }
        }
    }
}
fn push_error_if_introduced_variable_collides_or_is_unused(
    errors: &mut Vec<ErrorNode>,
    project_fns: &std::collections::HashMap<Name, CompiledProjectFnInfo>,
    local_bindings: &std::collections::HashMap<&Name, PatternVariableCompileInfo>,
    binding_name: &Name,
    binding_use: Option<lsp_types::Position>,
) {
    let Some(binding_info) = local_bindings.get(binding_name) else {
        return;
    };
    if project_fns.contains_key(binding_name) {
        if core_fns.contains_key(binding_name) {
            errors.push(ErrorNode {
                range: syntax_name_range(WithStartPosition{value: binding_name,start: binding_info.origin_start}),
                message: Box::from("a variable with this name is already part of core (core variables are for example int-to-str or dec-add). Rename this variable")
            });
        } else {
            errors.push(ErrorNode {
                range: syntax_name_range(WithStartPosition{value: binding_name,start: binding_info.origin_start}),
                message: Box::from(
                    "a variable with this name is already declared in this project. Rename one of them",
                ),
            });
        }
    } else if binding_use.is_none() {
        errors.push(ErrorNode {
            range: syntax_name_range(WithStartPosition {
                value: binding_name,
                start: binding_info.origin_start,
            }),
            message: Box::from(
                "variable not used in the resulting expression. Use it or replace this variable by _ to explicitly never handle the incoming value"
            )
        });
    }
}
fn type_references_origin(type_: &Type, origin: &Name) -> bool {
    match type_ {
        Type::Variable(_) => false,
        Type::Record(fields) => fields
            .iter()
            .any(|field| type_references_origin(&field.value, origin)),
        Type::ChoiceConstruct { name, arguments } => {
            (name == origin)
                || arguments
                    .iter()
                    .any(|argument| type_references_origin(argument, origin))
        }
    }
}

pub fn syntax_type_variables_into<'a, Types>(
    type_variables: &mut std::collections::HashSet<&'a Name>,
    type_: &'a SyntaxType<Types>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
) {
    match type_ {
        SyntaxType::Variable(name) => {
            type_variables.insert(&name.value);
        }
        SyntaxType::Record {
            ampersand_start: _,
            fields,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_type_variables_into(type_variables, value, types);
                }
            }
        }
        SyntaxType::Construct { name: _, arguments } => {
            for argument in types.opt_span_slice(core::Opt::from_option(arguments.as_ref())) {
                syntax_type_variables_into(type_variables, argument, types);
            }
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_type_variables_into(type_variables, types.element(inner), types);
            }
        }
    }
}
pub fn syntax_pattern_type_variables_into<'a, Patterns, Types>(
    type_variables: &mut std::collections::HashSet<&'a Name>,
    pattern: &'a SyntaxPattern<Patterns, Types>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
) {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => {
            if let Some(type_) = type_ {
                syntax_type_variables_into(type_variables, type_, types);
            }
        }
        SyntaxPattern::Variant {
            name: _,
            type_arguments,
            value,
        } => {
            if let Some(type_arguments) = type_arguments {
                for type_argument in
                    types.opt_span_slice(core::Opt::from_option(type_arguments.types.as_ref()))
                {
                    syntax_type_variables_into(type_variables, type_argument, types);
                }
            }
            if let Some(value) = value {
                syntax_pattern_type_variables_into(
                    type_variables,
                    patterns.element(value),
                    patterns,
                    types,
                );
            }
        }
        SyntaxPattern::Record {
            ampersand_start: _,
            fields,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_type_variables_into(type_variables, value, patterns, types)
                }
            }
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_pattern_type_variables_into(
                    type_variables,
                    patterns.element(inner),
                    patterns,
                    types,
                )
            }
        }
    }
}

/// Fully validated type
#[derive(Clone, Debug)]
enum TypeDiff {
    Conflict {
        expected: Type,
        actual: Type,
    },
    Variable(Name),
    ChoiceConstruct {
        name: Name,
        arguments: Vec<TypeDiff>,
    },
    Record(Vec<TypeDiffField>),
}
#[derive(Clone, Debug)]
struct TypeDiffField {
    name: Name,
    value: TypeDiff,
}

#[cfg(test)]
mod test_type_collect_variables_that_are_concrete_into {
    use super::*;
    #[test]
    fn test_type_collect_variables_that_are_concrete_into() {
        fn concrete_type_variables<'a>(
            type_with_variables: &'a Type,
            concrete_type: &'a Type,
        ) -> std::collections::HashMap<&'a str, std::borrow::Cow<'a, Type>> {
            let mut type_parameter_replacements = std::collections::HashMap::new();
            type_collect_variables_that_are_concrete_into(
                &mut type_parameter_replacements,
                type_with_variables,
                concrete_type,
            );
            type_parameter_replacements
        }
        fn type_variables_from<const N: usize>(
            type_variables: [(&'static str, Type); N],
        ) -> std::collections::HashMap<&'static str, std::borrow::Cow<'static, Type>> {
            std::collections::HashMap::from_iter(
                [("A", type_unt)]
                    .into_iter()
                    .map(|(name, type_)| (name, std::borrow::Cow::Owned(type_))),
            )
        }
        assert_eq!(
            concrete_type_variables(&type_variable("A"), &type_unt,),
            type_variables_from([("A", type_unt)])
        );
        assert_eq!(
            concrete_type_variables(
                &type_function([type_variable("A")], type_variable("A")),
                &type_function([type_variable("A")], type_unt),
            ),
            type_variables_from([("A", type_unt)])
        );
    }
}
fn type_collect_variables_that_are_concrete_into<'a>(
    type_parameter_replacements: &mut std::collections::HashMap<
        &'a str,
        std::borrow::Cow<'a, Type>,
    >,
    type_with_variables: &'a Type,
    concrete_type: &'a Type,
) {
    match type_with_variables {
        Type::Variable(variable_name) => {
            type_parameter_replacements
                .entry(variable_name.as_str())
                .and_modify(|existing_type_variable_replacement| {
                    // this feels too loose.
                    // coming up with an example where this fails would be awesome
                    *existing_type_variable_replacement = std::borrow::Cow::Owned(type_unify(
                        existing_type_variable_replacement.as_ref(),
                        concrete_type,
                    ));
                })
                .or_insert_with(|| std::borrow::Cow::Borrowed(concrete_type));
        }
        Type::ChoiceConstruct { name, arguments } => {
            if let Type::ChoiceConstruct {
                name: concrete_choice_type_construct_name,
                arguments: concrete_choice_type_construct_arguments,
            } = concrete_type
                && name == concrete_choice_type_construct_name
            {
                for (argument_type, concrete_argument_type) in arguments
                    .iter()
                    .zip(concrete_choice_type_construct_arguments.iter())
                {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        argument_type,
                        concrete_argument_type,
                    );
                }
            }
        }
        Type::Record(fields) => {
            if let Type::Record(concrete_fields) = concrete_type {
                for field in fields {
                    if let Some(matching_concrete_field) = concrete_fields
                        .iter()
                        .find(|concrete_field| concrete_field.name == field.name)
                    {
                        type_collect_variables_that_are_concrete_into(
                            type_parameter_replacements,
                            &field.value,
                            &matching_concrete_field.value,
                        );
                    }
                }
            }
        }
    }
}
/// consider taking a_type: &mut Type instead
fn type_unify(a_type: &Type, b_type: &Type) -> Type {
    match a_type {
        Type::Variable(_) => b_type.clone(),
        Type::ChoiceConstruct {
            name: a_name,
            arguments: a_arguments,
        } => {
            if let Type::ChoiceConstruct {
                name: b_name,
                arguments: b_arguments,
            } = b_type
                && a_name == b_name
            {
                Type::ChoiceConstruct {
                    name: a_name.clone(),
                    arguments: a_arguments
                        .iter()
                        .zip(b_arguments)
                        .map(|(a, b)| type_unify(a, b))
                        .collect(),
                }
            } else {
                a_type.clone()
            }
        }
        Type::Record(a_fields) => {
            if let Type::Record(b_fields) = b_type {
                Type::Record(
                    a_fields
                        .iter()
                        .map(|a| TypeField {
                            name: a.name.clone(),
                            value: match b_fields.iter().find(|b| b.name == a.name) {
                                None => a.value.clone(),
                                Some(b) => type_unify(&a.value, &b.value),
                            },
                        })
                        .collect(),
                )
            } else {
                a_type.clone()
            }
        }
    }
}

/// None means the types are equal
fn type_diff(expected_type: &Type, actual_type: &Type) -> Option<TypeDiff> {
    match expected_type {
        Type::Variable(expected_variable) => {
            if let Type::Variable(actual_variable) = actual_type
                && expected_variable == actual_variable
            {
                None
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type.clone(),
                    actual: actual_type.clone(),
                })
            }
        }
        Type::ChoiceConstruct {
            name: expected_name,
            arguments: expected_arguments,
        } => {
            if let Type::ChoiceConstruct {
                name: actual_choice_type_construct_name,
                arguments: actual_choice_type_construct_arguments,
            } = actual_type
                && expected_name == actual_choice_type_construct_name
            {
                if expected_arguments
                    .iter()
                    .zip(actual_choice_type_construct_arguments.iter())
                    .all(|(expected_argument, actual_argument)| {
                        type_diff(expected_argument, actual_argument).is_none()
                    })
                {
                    return None;
                }
                Some(TypeDiff::ChoiceConstruct {
                    name: expected_name.clone(),
                    arguments: expected_arguments
                        .iter()
                        .zip(actual_choice_type_construct_arguments.iter())
                        .map(|(expected_argument, actual_argument)| {
                            type_diff(expected_argument, actual_argument)
                                .unwrap_or_else(|| type_to_diff_without_conflict(expected_argument))
                        })
                        .collect(),
                })
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type.clone(),
                    actual: actual_type.clone(),
                })
            }
        }
        Type::Record(expected_fields) => {
            if let Type::Record(actual_fields) = actual_type
                && expected_fields.len() == actual_fields.len()
                && expected_fields.iter().all(|expected_field| {
                    actual_fields
                        .iter()
                        .any(|actual_field| actual_field.name == expected_field.name)
                })
            {
                if expected_fields
                    .iter()
                    .filter_map(|expected_field| {
                        actual_fields
                            .iter()
                            .find(|actual_field| actual_field.name == expected_field.name)
                            .map(|actual_field| (&expected_field.value, &actual_field.value))
                    })
                    .all(|(expected_field_value, actual_field_value)| {
                        type_diff(expected_field_value, actual_field_value).is_none()
                    })
                {
                    return None;
                }
                Some(TypeDiff::Record(
                    expected_fields
                        .iter()
                        .filter_map(|expected_field| {
                            actual_fields
                                .iter()
                                .find(|actual_field| actual_field.name == expected_field.name)
                                .map(|actual_field| (expected_field, &actual_field.value))
                        })
                        .map(|(expected_field, actual_field_value)| TypeDiffField {
                            name: expected_field.name.clone(),
                            value: type_diff(&expected_field.value, actual_field_value)
                                .unwrap_or_else(|| {
                                    type_to_diff_without_conflict(&expected_field.value)
                                }),
                        })
                        .collect(),
                ))
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type.clone(),
                    actual: actual_type.clone(),
                })
            }
        }
    }
}
fn type_to_diff_without_conflict(type_: &Type) -> TypeDiff {
    match type_ {
        Type::Variable(name) => TypeDiff::Variable(name.clone()),
        Type::ChoiceConstruct { name, arguments } => TypeDiff::ChoiceConstruct {
            name: name.clone(),
            arguments: arguments
                .iter()
                .map(type_to_diff_without_conflict)
                .collect(),
        },
        Type::Record(fields) => TypeDiff::Record(
            fields
                .iter()
                .map(|field| TypeDiffField {
                    name: field.name.clone(),
                    value: type_to_diff_without_conflict(&field.value),
                })
                .collect(),
        ),
    }
}

fn type_diff_error_message(type_diff: &TypeDiff) -> String {
    let mut builder: String = String::from("type mismatch:\n");
    type_diff_into(&mut builder, 0, type_diff);
    builder
}
fn type_diff_into(so_far: &mut String, indent: usize, type_diff: &TypeDiff) {
    match type_diff {
        TypeDiff::Conflict { expected, actual } => {
            so_far.push_str("expected:");
            space_or_linebreak_indented_into(
                so_far,
                type_info_line_span(expected),
                next_indent(indent),
            );
            type_format(so_far, next_indent(indent), expected);
            linebreak_indented_into(so_far, indent);
            so_far.push_str("actual:");
            space_or_linebreak_indented_into(
                so_far,
                type_info_line_span(actual),
                next_indent(indent),
            );
            type_format(so_far, next_indent(indent), actual);
        }
        TypeDiff::Variable(name) => {
            so_far.push_str(name);
        }
        TypeDiff::ChoiceConstruct { name, arguments } => {
            so_far.push_str(name);
            let line_span: LineSpan = type_diff_line_span(type_diff);
            for argument in arguments {
                space_or_linebreak_indented_into(so_far, line_span, next_indent(indent));
                let should_parenthesize_argument: bool = match argument {
                    TypeDiff::Variable(_) => false,
                    TypeDiff::Conflict { .. } => true,
                    TypeDiff::ChoiceConstruct {
                        name: _,
                        arguments: argument_arguments,
                    } => !argument_arguments.is_empty(),
                    TypeDiff::Record(fields) => !fields.is_empty(),
                };
                if should_parenthesize_argument {
                    so_far.push('(');
                    type_diff_into(so_far, next_indent(indent) + 1, argument);
                    if type_diff_line_span(argument) == LineSpan::Multiple {
                        linebreak_indented_into(so_far, next_indent(indent));
                    }
                    so_far.push(')');
                } else {
                    type_diff_into(so_far, next_indent(indent), argument);
                }
            }
        }
        TypeDiff::Record(fields) => match fields.as_slice() {
            [] => {
                so_far.push_str("&");
            }
            [field0, field1_up @ ..] => {
                so_far.push_str("& ");
                let line_span: LineSpan = type_diff_line_span(type_diff);
                type_diff_field_into(so_far, indent, field0);
                for field in field1_up {
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(so_far, indent);
                    }
                    so_far.push(' ');
                    type_diff_field_into(so_far, indent, field);
                }
            }
        },
    }
}
fn type_diff_field_into(so_far: &mut String, indent: usize, type_diff_field: &TypeDiffField) {
    so_far.push('(');
    so_far.push_str(&type_diff_field.name);
    space_or_linebreak_indented_into(
        so_far,
        type_diff_line_span(&type_diff_field.value),
        next_indent(indent),
    );
    type_diff_into(so_far, next_indent(indent), &type_diff_field.value);
    // TODO sometimes linebreak
    so_far.push(')');
}
const type_info_line_length_estimate_maximum: usize = 56;
fn type_diff_line_span(type_diff: &TypeDiff) -> LineSpan {
    if type_diff_length_estimate(type_diff) <= type_info_line_length_estimate_maximum {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn type_diff_length_estimate(type_diff: &TypeDiff) -> usize {
    match type_diff {
        TypeDiff::Conflict { .. } => type_info_line_length_estimate_maximum + 1,
        TypeDiff::Variable(variable_name) => variable_name.len(),
        TypeDiff::ChoiceConstruct { name, arguments } => {
            name.len()
                + arguments
                    .iter()
                    .map(type_diff_length_estimate)
                    .sum::<usize>()
        }
        TypeDiff::Record(fields) => fields
            .iter()
            .map(|field| field.name.len() + type_diff_length_estimate(&field.value))
            .sum(),
    }
}
pub fn type_format(formatted: &mut String, indent: usize, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            formatted.push_str(name);
        }
        Type::ChoiceConstruct { name, arguments } => {
            formatted.push_str(name);
            let line_span: LineSpan = type_info_line_span(type_);
            for argument in arguments {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                let should_parenthesize_argument: bool = match argument {
                    Type::Variable(_) => false,
                    Type::Record(fields) => !fields.is_empty(),
                    Type::ChoiceConstruct {
                        name: _,
                        arguments: argument_arguments,
                    } => !argument_arguments.is_empty(),
                };
                if should_parenthesize_argument {
                    formatted.push('(');
                    type_format(formatted, next_indent(indent) + 1, argument);
                    if type_info_line_span(argument) == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                    formatted.push(')');
                } else {
                    type_format(formatted, next_indent(indent), argument);
                }
            }
        }
        Type::Record(fields) => match fields.as_slice() {
            [] => {
                formatted.push_str("&");
            }
            [field0, field1_up @ ..] => {
                formatted.push_str("&");
                let line_span: LineSpan = type_info_line_span(type_);
                space_or_linebreak_indented_into(formatted, line_span, indent);
                type_field_format(formatted, indent, field0);
                for field in field1_up {
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, indent);
                    }
                    formatted.push_str(" ");
                    type_field_format(formatted, indent, field);
                }
            }
        },
    }
}
fn type_field_format(formatted: &mut String, indent: usize, type_field: &TypeField) {
    let line_span = type_info_line_span(&type_field.value);
    formatted.push('(');
    formatted.push_str(&type_field.name);
    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
    type_format(formatted, next_indent(indent), &type_field.value);
    if line_span == LineSpan::Multiple {
        formatted.push('\n');
    }
    formatted.push(')');
}
fn type_info_line_span(type_: &Type) -> LineSpan {
    if type_length_estimate(type_) <= type_info_line_length_estimate_maximum {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn type_length_estimate(type_: &Type) -> usize {
    match type_ {
        Type::Variable(variable_name) => variable_name.len(),

        Type::ChoiceConstruct { name, arguments } => {
            name.len() + arguments.iter().map(type_length_estimate).sum::<usize>()
        }
        Type::Record(fields) => fields
            .iter()
            .map(|field| field.name.len() + type_length_estimate(&field.value))
            .sum(),
    }
}

fn syn_spread_expr_block(syn_expr: syn::Expr) -> syn::Block {
    match syn_expr {
        syn::Expr::Block(block) => block.block,
        _ => syn::Block {
            brace_token: syn::token::Brace(syn_span()),
            stmts: vec![syn::Stmt::Expr(syn_expr, None)],
        },
    }
}

fn name_to_uppercase_rust(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    if [
        "Self",
        "Clone",
        "Copy",
        "PartialEq",
        "Eq",
        "Debug",
        "Hash",
        "PartialOrd",
        "Ord",
        "Blank",
        "Fn",
        // type variables used in core
        "A",
        "B",
        "C",
        "E",
        "N",
        "S",
    ]
    .contains(&sanitized.as_str())
    {
        sanitized + "ø_"
    } else {
        sanitized
    }
}
fn name_to_lowercase_rust(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    if rust_lowercase_keywords.contains(&sanitized.as_str())
        || sanitized == local_unnamed_function_name
    {
        sanitized + "ø"
    } else {
        sanitized
    }
}
const local_unnamed_function_name: &str = "local_unnamed_function";
/// both weak, reserved and strong.
/// see <https://doc.rust-lang.org/reference/keywords.html>
const rust_lowercase_keywords: [&str; 55] = [
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "gen",
    "static",
    "macro_rules",
    "raw",
    "safe",
    "union",
];
fn type_variable_to_rust(name: &str) -> String {
    // to disambiguate from choice type and type alias names
    name_to_uppercase_rust(name) + "ø"
}
fn sorted_field_names<'a>(field_names: impl Iterator<Item = &'a Name>) -> Vec<Name> {
    let mut field_names_vec: Vec<Name> = field_names.map(Name::clone).collect();
    field_names_vec.sort_unstable();
    field_names_vec
}
fn field_names_to_rust_record_struct_name<'a>(
    field_names: impl Iterator<Item = &'a Name>,
) -> String {
    let mut rust_field_names_vec: Vec<String> = field_names
        .map(|field_name| name_to_lowercase_rust(field_name))
        .collect::<Vec<_>>();
    rust_field_names_vec.sort_unstable();
    // the separator between fields is the "middle dot": https://util.unicode.org/UnicodeJsps/character.jsp?a=00B7
    // It is chosen because
    // - it can be typed on regular keyboards (on my keyboard at least it's AltGr+., on mac it seems to be Option+Shift+9, not sure for the rest.
    //   if it cannot be typed on your keyboard, please open an issue!)
    // - it looks similar to the field access dot
    // - it is somewhat commonly understood as a separator
    let mut field_names_joined: String = rust_field_names_vec.join("·");
    match field_names_joined.get_mut(0..=0) {
        Some(first) => {
            first.make_ascii_uppercase();
            if rust_field_names_vec.len() == 1 {
                field_names_joined.push('·');
            }
            field_names_joined
        }
        None => "Blank".to_string(),
    }
}
fn syn_span() -> proc_macro2::Span {
    proc_macro2::Span::call_site()
}
fn syn_ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, syn_span())
}
fn syn_path_reference<const N: usize>(segments: [&str; N]) -> syn::Path {
    syn::Path {
        leading_colon: None,
        segments: segments
            .into_iter()
            .map(|name| syn_path_segment_ident(name))
            .collect(),
    }
}
fn syn_path_segment_ident(name: &str) -> syn::PathSegment {
    syn::PathSegment {
        ident: syn_ident(name),
        arguments: syn::PathArguments::None,
    }
}
fn syn_attribute_doc(documentation: &str) -> syn::Attribute {
    syn::Attribute {
        pound_token: syn::token::Pound(syn_span()),
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket(syn_span()),
        meta: syn::Meta::NameValue(syn::MetaNameValue {
            path: syn::Path::from(syn_ident("doc")),
            eq_token: syn::token::Eq(syn_span()),
            value: syn::Expr::Lit(syn::ExprLit {
                attrs: vec![],
                lit: syn::Lit::Str(syn::LitStr::new(documentation, syn_span())),
            }),
        }),
    }
}
fn syn_type_variable(name: &str) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path::from(syn_ident(name)),
    })
}
fn default_parameter_bounds() -> impl Iterator<Item = syn::TypeParamBound> {
    [
        // syn::TypeParamBound::Trait(syn::TraitBound {
        //     paren_token: None,
        //     modifier: syn::TraitBoundModifier::None,
        //     lifetimes: None,
        //     path: syn::Path::from(syn_ident("Clone")),
        // }),
        // TODO is 'static necessary for anything? It should not need to be
        syn::TypeParamBound::Lifetime(syn_lifetime_static()),
    ]
    .into_iter()
}
fn syn_lifetime_static() -> syn::Lifetime {
    syn::Lifetime {
        apostrophe: syn_span(),
        ident: syn_ident("static"),
    }
}
fn syn_attribute_derive<'a>(trait_macro_names: impl Iterator<Item = &'a str>) -> syn::Attribute {
    syn::Attribute {
        pound_token: syn::token::Pound(syn_span()),
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket(syn_span()),
        meta: syn::Meta::List(syn::MetaList {
            path: syn_path_reference(["derive"]),
            delimiter: syn::MacroDelimiter::Paren(syn::token::Paren(syn_span())),
            // is there really no way to print e.g. Punctuated?
            tokens: trait_macro_names
                .flat_map(|token| {
                    [
                        proc_macro2::TokenTree::Ident(syn_ident(token)),
                        proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                            ',',
                            proc_macro2::Spacing::Alone,
                        )),
                    ]
                })
                .collect(),
        }),
    }
}
fn syn_expr_todo() -> syn::Expr {
    syn::Expr::Macro(syn::ExprMacro {
        attrs: vec![],
        mac: syn::Macro {
            path: syn_path_reference(["std", "todo"]),
            bang_token: syn::token::Not(syn_span()),
            delimiter: syn::MacroDelimiter::Paren(syn::token::Paren(syn_span())),
            tokens: proc_macro2::TokenStream::new(),
        },
    })
}
fn syn_expr_reference<const N: usize>(segments: [&str; N]) -> syn::Expr {
    syn::Expr::Path(syn::ExprPath {
        attrs: vec![],
        qself: None,
        path: syn_path_reference(segments),
    })
}

fn type_variable(name: &'static str) -> Type {
    Type::Variable(Name::const_new(name))
}
fn type_fn(in_: Type, out: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("fn"),
        arguments: vec![in_, out],
    }
}
fn type_record(fields: impl IntoIterator<Item = (&'static str, Type)>) -> Type {
    Type::Record(
        fields
            .into_iter()
            .map(|(field_name, field_value)| TypeField {
                name: Name::const_new(field_name),
                value: field_value,
            })
            .collect(),
    )
}
const type_p32: Type = Type::ChoiceConstruct {
    name: Name::const_new("p32"),
    arguments: vec![],
};
const type_u32: Type = Type::ChoiceConstruct {
    name: Name::const_new("u32"),
    arguments: vec![],
};
const type_i32: Type = Type::ChoiceConstruct {
    name: Name::const_new("i32"),
    arguments: vec![],
};
const type_f32: Type = Type::ChoiceConstruct {
    name: Name::const_new("f32"),
    arguments: vec![],
};
const type_char: Type = Type::ChoiceConstruct {
    name: Name::const_new("char"),
    arguments: vec![],
};
const type_str: Type = Type::ChoiceConstruct {
    name: Name::const_new("str"),
    arguments: vec![],
};
fn type_vec(origin: Type, element: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("vec"),
        arguments: vec![origin, element],
    }
}
fn type_arena(origin: Type, element: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("arena"),
        arguments: vec![origin, element],
    }
}
fn type_slot(origin: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("slot"),
        arguments: vec![origin],
    }
}
fn type_span(origin: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("span"),
        arguments: vec![origin],
    }
}
fn type_opt_span_build(backing: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("opt-span-build"),
        arguments: vec![backing],
    }
}
fn type_span_build(backing: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("span-build"),
        arguments: vec![backing],
    }
}
fn type_opt(present: Type) -> Type {
    Type::ChoiceConstruct {
        name: Name::const_new("opt"),
        arguments: vec![present],
    }
}
pub static core_fns: std::sync::LazyLock<std::collections::HashMap<Name, CompiledProjectFnInfo>> =
    std::sync::LazyLock::new(|| {
        std::collections::HashMap::from([
            (
                Name::const_new("p32-add"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from("Saturating a + b")),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([("p", type_p32), ("u", type_u32)])),
                    result_type: Some(type_p32),
                },
            ),
            (
                Name::const_new("u32-add"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from("Saturating a + b")),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([("a", type_u32), ("b", type_u32)])),
                    result_type: Some(type_u32),
                },
            ),
            (
                Name::const_new("i32-add"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from("Saturating a + b")),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([("a", type_i32), ("b", type_i32)])),
                    result_type: Some(type_i32),
                },
            ),
            (
                Name::const_new("f32-add"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from("Saturating a + b")),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([("a", type_f32), ("b", type_f32)])),
                    result_type: Some(type_f32),
                },
            ),
            (
                Name::const_new("arena-empty"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Initialize an `arena` with 0 elements. Modify with `arena-pre-allocate-at-least`, `arena-add` etc.",
                    )),
                    type_parameters: vec![Name::const_new("Element")],
                    parameter_type: Some(type_variable("Origin")),
                    result_type: Some(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    )),
                },
            ),
            (
                Name::const_new("arena-pre-allocate-at-least"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Reserves capacity for at least `length` more elements to be added. This can prevent frequent re-allocation of the underlying array.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "arena",
                            type_arena(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("length", type_u32),
                    ])),
                    result_type: Some(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    )),
                },
            ),
            (
                Name::const_new("arena-add"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Reserves capacity for at least `length` more elements to be added. This can prevent frequent re-allocation of the underlying array.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "arena",
                            type_arena(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("new", type_variable("Element")),
                    ])),
                    result_type: Some(type_record([
                        (
                            "arena",
                            type_arena(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("slot", type_slot(type_variable("Origin"))),
                    ])),
                },
            ),
            (
                Name::const_new("arena-span-empty"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Start a `span-build` backed by the given arena. Modify with `arena-span-build-add`, `arena-span-build-add-str` etc. and finish with `arena-span-build`",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    )),
                    result_type: Some(type_opt_span_build(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                },
            ),
            (
                Name::const_new("arena-opt-span-add-str"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Attach a given `str` to the span of a given `span-build`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "build",
                            type_opt_span_build(type_arena(
                                type_variable("Origin"),
                                type_variable("Element"),
                            )),
                        ),
                        ("new", type_str),
                    ])),
                    result_type: Some(type_opt_span_build(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                },
            ),
            (
                Name::const_new("arena-span-add-str"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Attach a given `str` to the span of a given `span-build`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "build",
                            type_span_build(type_arena(
                                type_variable("Origin"),
                                type_variable("Element"),
                            )),
                        ),
                        ("new", type_str),
                    ])),
                    result_type: Some(type_span_build(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                },
            ),
            (
                Name::const_new("arena-opt-span-build"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Finish an `opt-span-build` and split it into the backing `arena` and the built `opt span`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_opt_span_build(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                    result_type: Some(type_record([
                        (
                            "arena",
                            type_arena(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("span", type_opt(type_span(type_variable("Origin")))),
                    ])),
                },
            ),
            (
                Name::const_new("arena-span-build"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Finish a `span-build` and split it into the backing `arena` and the `span`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_span_build(type_arena(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                    result_type: Some(type_record([
                        (
                            "arena",
                            type_arena(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("span", type_span(type_variable("Origin"))),
                    ])),
                },
            ),
            (
                Name::const_new("vec-empty"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Initialize a `vec` with 0 elements. Modify with `vec-pre-allocate-at-least`, `vec-add` etc.",
                    )),
                    type_parameters: vec![Name::const_new("Element")],
                    parameter_type: Some(type_variable("Origin")),
                    result_type: Some(type_vec(type_variable("Origin"), type_variable("Element"))),
                },
            ),
            (
                Name::const_new("vec-pre-allocate-at-least"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Reserves capacity for at least `length` more elements to be added. This can prevent frequent re-allocation of the underlying array.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("length", type_u32),
                    ])),
                    result_type: Some(type_vec(type_variable("Origin"), type_variable("Element"))),
                },
            ),
            (
                Name::const_new("vec-span-empty"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Start a `span-build` backed by the given vec. Modify with `vec-opt-span-add`, `vec-opt-span-add-str` etc. and finish with `vec-opt-span-build`",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_vec(
                        type_variable("Origin"),
                        type_variable("Element"),
                    )),
                    result_type: Some(type_opt_span_build(type_vec(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                },
            ),
            (
                Name::const_new("vec-opt-span-add-str"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Attach a given `str` to the span of a given `span-build`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "build",
                            type_opt_span_build(type_vec(
                                type_variable("Origin"),
                                type_variable("Element"),
                            )),
                        ),
                        ("new", type_str),
                    ])),
                    result_type: Some(type_opt_span_build(type_vec(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                },
            ),
            (
                Name::const_new("vec-span-add-str"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Attach a given `str` to the span of a given `span-build`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "build",
                            type_span_build(type_vec(
                                type_variable("Origin"),
                                type_variable("Element"),
                            )),
                        ),
                        ("new", type_str),
                    ])),
                    result_type: Some(type_span_build(type_vec(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                },
            ),
            (
                Name::const_new("vec-opt-span-build"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Finish an `opt-span-build` and split it into the backing `vec` and the built `opt span`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_opt_span_build(type_vec(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                    result_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("span", type_opt(type_span(type_variable("Origin")))),
                    ])),
                },
            ),
            (
                Name::const_new("vec-span-build"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Finish a `span-build` and split it into the backing `vec` and the built `span`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_span_build(type_vec(
                        type_variable("Origin"),
                        type_variable("Element"),
                    ))),
                    result_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("span", type_span(type_variable("Origin"))),
                    ])),
                },
            ),
        ])
    });
pub static core_choice_types: std::sync::LazyLock<
    std::collections::HashMap<Name, CompiledChoiceTypeInfo>,
> = std::sync::LazyLock::new(|| {
    std::collections::HashMap::from([
        (
            Name::const_new("p32"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 1 (positive integer) with 32 bits.
```sloe
fn answer & p32
    p32-add (2 p32) (40 u32)
```
",
                )),
                parameters: vec![],
                variants: vec![],
                is_copy: true,
            },
        ),
        (
            Name::const_new("u32"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 0 (unsigned integer) with 32 bits.
```sloe
fn answer & u32
    u32-add (2 u32) (40 u32)
```
",
                )),
                parameters: vec![],
                variants: vec![],
                is_copy: true,
            },
        ),
        (
            Name::const_new("i32"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed whole number (integer) with 32 bits.
```sloe
fn answer & i32
    i32-add (-8 i32) (50 i32)
```
",
                )),
                parameters: vec![],
                variants: vec![],
                is_copy: true,
            },
        ),
        (
            Name::const_new("f32"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed decimal number (floating-point) with 32 bit precision.
Does not allow infinities or NaN. If you need these error states, explicitly model them with a choice type.
```sloe
fn answer & i32
    i32-add (-8.5 f32) (50.5 f32)
```
",
                )),
                parameters: vec![],
                variants: vec![],
                is_copy: true,
            },
        ),
        (
            Name::const_new("char"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r#"A unicode scalar like `'a'` or `'👀'` or `\u{2665}` (hex code for ♥).
Keep in mind that a human-readable visual symbol can be composed of multiple such unicode scalars (forming a grapheme cluster), For example:
```sloe
str-start "🇺🇸"
# = Present & (start '\u{1F1FA}') (after "\u{1F1F8}")
#                    Indicator U         Indicator S
```
Read if interested: [swift's grapheme cluster docs](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/stringsandcharacters/#Extended-Grapheme-Clusters)
"#,
                )),
                parameters: vec![],
                variants: vec![],
                is_copy: true,
            },
        ),
        (
            Name::const_new("str"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r#"Text valid for the entire program like `"abc"` or `"\"hello 👀 \\\r\n world \u{2665}\""` (`\u{2665}` represents the hex code for ♥, `\"` represents ", `\\` represents \\, `\n` represents line break, `\r` represents carriage return).
Internally, a string is compactly represented as UTF-8 bytes and can be accessed as such.
When building strings, use functions like `arena-add-str`.
"#,
                )),
                parameters: vec![],
                variants: vec![],
                is_copy: false,
            },
        ),
        (
            Name::const_new("opt"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"Either you have some value or you have nothing.",
                )),
                parameters: vec![Name::const_new("A")],
                variants: vec![
                    CompiledVariantInfo {
                        name: Name::const_new("Absent"),
                        type_parameters: vec![Name::const_new("A")],
                        value: type_record([]),
                    },
                    CompiledVariantInfo {
                        name: Name::const_new("Present"),
                        type_parameters: vec![],
                        value: type_variable("A"),
                    },
                ],
                is_copy: true,
            },
        ),
        (
            Name::const_new("exit-or-go-on"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"Either done with a final result or continuing with a partial result.
Typically used for operations that can shortcut, see for example `span-fold-while-from`.
```sloe
fn loop-from (& (state state State) (step step fn State go-on-or-exit State Exit))
    :(step state)
    ((Exit exit) exit)
    ((Go-on updated-state)
        loop-from updated-state step
    )
```
",
                )),
                parameters: vec![Name::const_new("Exit"), Name::const_new("Go-on")],
                variants: vec![
                    CompiledVariantInfo {
                        name: Name::const_new("Exit"),
                        type_parameters: vec![Name::const_new("Go-on")],
                        value: type_variable("Exit"),
                    },
                    CompiledVariantInfo {
                        name: Name::const_new("Go-on"),
                        type_parameters: vec![Name::const_new("Exit")],
                        value: type_variable("Go-on"),
                    },
                ],
                is_copy: true,
            },
        ),
        (
            Name::const_new("vec"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A grow- and shrinkable array of elements. Arrays have constant time access and mutation and amortized constant time push.
```sloe
fn use-a-vec & u32
    origin my-elements-origin
    :(vec-empty<u32> my-elements-origin) my-elements-vec
    :(vec-add & (vec my-elements-vec) (element 609 u32)) (& (vec my-elements-vec) (slot first-element-slot))
    :(vec-vacate & (vec _) (slot first-element-slot))
    first-element-slot # 609 u32
```
For temporary, non-shrinkable arrays, use `arena`
"
                )),
                parameters: vec![Name::const_new("Origin"), Name::const_new("Element")],
                variants: vec![],
                is_copy: false,
            },
        ),
        (
            Name::const_new("arena"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A grow- and shrinkable array of elements. Arrays have constant time access and mutation and amortized constant time push.
```sloe
fn use-an-arena & u32
    origin my-elements-origin
    :(arena-empty<u32> my-elements-origin) my-elements-arena
    :(arena-add & (arena my-elements-arena) (element 609 u32)) (& (arena my-elements-arena) (slot first-element-slot))
    :(arena-element & (arena _) (slot first-element-slot))
    first-element-slot # 609 u32
```
For temporary, non-shrinkable arrays, use `arena`
"
                )),
                parameters: vec![Name::const_new("Origin"), Name::const_new("Element")],
                variants: vec![],
                is_copy: false,
            },
        ),
        (
            Name::const_new("slot"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A valid index into a collection.
This works because each collection has a unique origin and only gives out one slot for each index.
"
                )),
                parameters: vec![Name::const_new("Origin")],
                variants: vec![],
                is_copy: false,
            },
        ),
        (
            Name::const_new("span-filled"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A range of consecutive valid indexes into a collection with at least one known index.
This works because each collection has a unique origin and only gives out one span for each range.
"
                )),
                parameters: vec![Name::const_new("Origin")],
                variants: vec![],
                is_copy: false,
            },
        ),
        (
            Name::const_new("span-build"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A `span` at the end of a backing collecion, plus that collection.
This makes it easy to add elements to the end, as we know there's enough space to occupy.
"
                )),
                parameters: vec![Name::const_new("Backing")],
                variants: vec![],
                is_copy: false,
            },
        ),
        (
            Name::const_new("span-filled-build"),
            CompiledChoiceTypeInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A `span-filled` at the end of a backing collecion, plus that collection.
This makes it easy to add elements to the end, as we know there's enough space to occupy.
"
                )),
                parameters: vec![Name::const_new("Backing")],
                variants: vec![],
                is_copy: false,
            },
        ),
    ])
});
pub static core_records: std::sync::LazyLock<std::collections::HashSet<&'static [Name]>> =
    std::sync::LazyLock::new(|| {
        fn type_records(type_: &Type, records: &mut std::collections::HashSet<&'static [Name]>) {
            match type_ {
                Type::Variable(_) => {}
                Type::Record(fields) => {
                    records.insert(
                        fields
                            .iter()
                            .map(|field| field.name.clone())
                            .collect::<Vec<Name>>()
                            // static variables will only be created once and won't be dropped anyway
                            .leak(),
                    );
                    for field in fields {
                        type_records(&field.value, records);
                    }
                }
                Type::ChoiceConstruct { name: _, arguments } => {
                    for argument in arguments {
                        type_records(argument, records);
                    }
                }
            }
        }
        let mut records = std::collections::HashSet::new();
        for core_fn_info in core_fns.values() {
            if let Some(parameter_type) = &core_fn_info.parameter_type {
                type_records(parameter_type, &mut records);
            }
            if let Some(result_type) = &core_fn_info.result_type {
                type_records(result_type, &mut records);
            }
        }
        records
    });

pub struct ErrorNode {
    pub message: Box<str>,
    pub range: lsp_types::Range,
}

pub fn compiled_rust_to_file_content(rust_file: &syn::File) -> String {
    format!(
        "// jump to compiled code by searching for // compiled
{}


// compiled code //


{}",
        // TODO this is horrible. (It also does not respect potential output file names given by sloe build)
        include_str!("core.rs").replacen("$crate::core", "$crate::sloe", 1),
        prettyplease::unparse(rust_file)
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineSpan {
    Single,
    Multiple,
}
fn range_line_span(range: lsp_types::Range) -> LineSpan {
    if range.start.line == range.end.line {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn linebreak_indented_into(formatted: &mut String, indent: usize) {
    formatted.push('\n');
    formatted.extend(std::iter::repeat_n(' ', indent));
}
fn space_or_linebreak_indented_into(formatted: &mut String, line_span: LineSpan, indent: usize) {
    match line_span {
        LineSpan::Single => {
            formatted.push(' ');
        }
        LineSpan::Multiple => {
            linebreak_indented_into(formatted, indent);
        }
    }
}
fn next_indent(current_indent: usize) -> usize {
    (current_indent + 1).next_multiple_of(4)
}

fn syntax_comments_format(formatted: &mut String, indent: usize, comments: &SyntaxComments) {
    formatted.push_str(&comments.line0.value);
    linebreak_indented_into(formatted, indent);
    for line in &comments.line1_up {
        formatted.push_str(&line.value);
        linebreak_indented_into(formatted, indent);
    }
}
pub fn syntax_project_format<Expressions, Patterns, Types>(
    project: &SyntaxProject<Expressions, Patterns, Types>,
    _source: &str,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> String {
    let mut formatted = String::with_capacity(project.elements.len() * 128);
    formatted.push('\n');
    for element in &project.elements {
        // consider not formatting an element that is followed by Unrecognized
        match element {
            SyntaxProjectElement::TypeAlias {
                type_keyword_start: _,
                name,
                parameters,
                documentation,
                type_,
            } => {
                formatted.push_str("type ");
                if let Some(name) = name {
                    formatted.push_str(&name.value);
                }
                for parameter in parameters {
                    formatted.push(' ');
                    formatted.push_str(&parameter.value);
                }
                if let Some(documentation) = documentation {
                    linebreak_indented_into(&mut formatted, next_indent(0));
                    syntax_comments_format(&mut formatted, next_indent(0), documentation);
                }
                formatted.push(' ');
                if let Some(type_) = type_ {
                    syntax_type_unparenthesized_format(
                        &mut formatted,
                        next_indent(0),
                        types,
                        type_,
                    );
                }
            }
            SyntaxProjectElement::ChoiceType {
                choice_keyword_start: _,
                name,
                parameters,
                documentation,
                variants,
            } => {
                formatted.push_str("choice ");
                if let Some(name) = name {
                    formatted.push_str(&name.value);
                }
                for parameter in parameters {
                    formatted.push(' ');
                    formatted.push_str(&parameter.value);
                }
                if let Some(documentation) = documentation {
                    linebreak_indented_into(&mut formatted, next_indent(0));
                    syntax_comments_format(&mut formatted, next_indent(0), documentation);
                }
                for variant in variants {
                    linebreak_indented_into(&mut formatted, next_indent(0));
                    syntax_variant_format(&mut formatted, next_indent(0), types, &variant);
                }
            }
            SyntaxProjectElement::Fn {
                fn_keyword_start: _,
                name,
                type_parameters,
                parameter,
                result_type,
                documentation,
                result,
            } => {
                formatted.push_str("fn ");
                if let Some(name) = name {
                    formatted.push_str(&name.value);
                }
                if let Some(type_parameters) = type_parameters {
                    syntax_angled_type_parameters_format(&mut formatted, type_parameters);
                }
                formatted.push(' ');
                if let Some(parameter) = parameter {
                    let parameter_line_span =
                        range_line_span(pattern_range(parameter, patterns, types));
                    syntax_pattern_parenthesized_if_open_ended_format(
                        &mut formatted,
                        next_indent(0),
                        patterns,
                        types,
                        parameter,
                    );
                    space_or_linebreak_indented_into(
                        &mut formatted,
                        parameter_line_span,
                        next_indent(0),
                    );
                }
                if let Some(result_type) = result_type {
                    syntax_type_parenthesized_if_open_ended_format(
                        &mut formatted,
                        next_indent(0),
                        types,
                        result_type,
                    );
                }
                if let Some(documentation) = documentation {
                    linebreak_indented_into(&mut formatted, next_indent(0));
                    syntax_comments_format(&mut formatted, next_indent(0), documentation);
                }
                if let Some(result) = result {
                    linebreak_indented_into(&mut formatted, next_indent(0));
                    syntax_expression_unparenthesized_format(
                        &mut formatted,
                        next_indent(0),
                        expressions,
                        patterns,
                        types,
                        result,
                    )
                }
            }
            SyntaxProjectElement::Comments(comments) => {
                syntax_comments_format(&mut formatted, 0, comments);
            }
            SyntaxProjectElement::Unrecognized { range: _, source } => {
                formatted.push_str(source);
            }
        }
        formatted.push_str("\n\n");
    }
    formatted
}
fn syntax_angled_type_parameters_format(
    formatted: &mut String,
    angled_type_parameters: &SyntaxAngledTypeParameters,
) {
    formatted.push('<');
    if let Some((name0, name1_up)) = angled_type_parameters.names.split_first() {
        formatted.push_str(&name0.value);
        for name in name1_up {
            formatted.push(' ');
            formatted.push_str(&name.value);
        }
    }
    formatted.push('>');
}
fn syntax_variant_format<Types>(
    formatted: &mut String,
    indent: usize,
    types: &core::Vec<Types, SyntaxType<Types>>,
    variant: &SyntaxVariant<Types>,
) {
    formatted.push('(');
    if let Some(name) = &variant.name {
        formatted.push_str(&name.value);
    }
    if let Some(type_parameters) = &variant.type_parameters {
        syntax_angled_type_parameters_format(formatted, type_parameters);
    }
    formatted.push(' ');
    if let Some(value) = &variant.value {
        syntax_type_unparenthesized_format(formatted, indent, types, value);
        let line_span = range_line_span(type_range(value, types));
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, next_indent(indent));
        }
    }
    formatted.push(')');
}
fn syntax_field_format<Value>(
    formatted: &mut String,
    indent: usize,
    field: &SyntaxField<Value>,
    value_end: impl FnOnce(&Value) -> lsp_types::Position,
    value_format: impl FnOnce(&mut String, usize, &Value),
) {
    formatted.push('(');
    formatted.push_str(&field.name.value);
    if field.left_angle_start.is_some() {
        formatted.push_str(" <");
    }
    formatted.push(' ');
    if let Some(value) = &field.value {
        value_format(formatted, next_indent(indent), value);
    }
    let line_span = range_line_span(field_range(field, value_end));
    if line_span == LineSpan::Multiple {
        linebreak_indented_into(formatted, indent);
    }
    formatted.push(')');
}
fn syntax_char_format(formatted: &mut String, maybe_char: Option<char>) {
    match maybe_char {
        None => {
            formatted.push_str("''");
        }
        Some(char) => {
            formatted.push('\'');
            match char {
                '\'' => formatted.push_str("\\'"),
                '\\' => formatted.push_str("\\\\"),
                '\t' => formatted.push_str("\\t"),
                '\n' => formatted.push_str("\\n"),
                '\r' => formatted.push_str("\\r"),
                other_character => {
                    if char_needs_unicode_escaping(other_character) {
                        unicode_char_escape_into(formatted, other_character);
                    } else {
                        formatted.push(other_character);
                    }
                }
            }
            formatted.push('\'');
        }
    }
}
fn char_needs_unicode_escaping(char: char) -> bool {
    char.is_control()
}
fn unicode_char_escape_into(so_far: &mut String, char: char) {
    let code: u32 = char.into();
    use std::fmt::Write as _;
    let _ = write!(so_far, "\\u{{{:X}}}", code);
}
fn syntax_string_format(formatted: &mut String, content: &str) {
    formatted.push('"');
    for char in content.chars() {
        match char {
            '\"' => formatted.push_str("\\\""),
            '\\' => formatted.push_str("\\\\"),
            '\t' => formatted.push_str("\\t"),
            '\n' => formatted.push_str("\\n"),
            '\u{000D}' => formatted.push_str("\\u{000D}"),
            other_character => {
                if char_needs_unicode_escaping(other_character) {
                    unicode_char_escape_into(formatted, other_character);
                } else {
                    formatted.push(other_character);
                }
            }
        }
    }
    formatted.push('"');
}
fn syntax_expression_parenthesized_if_open_ended_format<Expressions, Patterns, Types>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
) {
    if syntax_expression_is_open_ended(expression, expressions, types) {
        formatted.push('(');
        let line_span = range_line_span(expression_range(expression, expressions, patterns, types));
        syntax_expression_unparenthesized_format(
            formatted,
            next_indent(indent),
            expressions,
            patterns,
            types,
            expression,
        );
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
        formatted.push(')');
    } else {
        syntax_expression_unparenthesized_format(
            formatted,
            indent,
            expressions,
            patterns,
            types,
            expression,
        );
    }
}
fn syntax_expression_is_open_ended<Expressions, Patterns, Types>(
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> bool {
    match expression {
        SyntaxExpression::Number { value: _, type_ } => type_
            .as_ref()
            .is_some_and(|type_| syntax_type_is_open_ended(type_, types)),
        SyntaxExpression::Char { .. } => false,
        SyntaxExpression::Str { .. } => false,
        SyntaxExpression::VariableOrCall {
            name: _,
            type_arguments,
            argument,
        } => type_arguments.is_some() || argument.is_some(),
        SyntaxExpression::Variant {
            name: _,
            type_arguments: _,
            value,
        } => value.as_ref().is_some_and(|value| {
            syntax_expression_is_open_ended(expressions.element(value), expressions, types)
        }),
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            parameter: _,
            result,
        } => result.as_ref().is_some_and(|result| {
            syntax_expression_is_open_ended(expressions.element(result), expressions, types)
        }),
        SyntaxExpression::Record {
            ampersand_start: _,
            fields,
        } => !fields.is_empty(),
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().is_some_and(|inner| {
            syntax_expression_is_open_ended(expressions.element(inner), expressions, types)
        }),
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => expression.as_ref().is_some_and(|expression| {
            syntax_expression_is_open_ended(expressions.element(expression), expressions, types)
        }),
        SyntaxExpression::Query { .. } => true,
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name: _,
            result,
        } => result.as_ref().is_some_and(|result| {
            syntax_expression_is_open_ended(expressions.element(result), expressions, types)
        }),
    }
}
fn syntax_expression_unparenthesized_format<Expressions, Patterns, Types>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
) {
    match expression {
        SyntaxExpression::Number { value, type_ } => {
            if value.value.starts_with("0") {
                formatted.push('0');
                formatted.push_str(&value.value.trim_start_matches("0"));
            } else if value.value.starts_with('.') {
                formatted.push('0');
                formatted.push_str(&value.value);
            } else {
                formatted.push_str(&value.value);
            }
            formatted.push(' ');
            if let Some(type_) = type_ {
                syntax_type_unparenthesized_format(formatted, next_indent(indent), types, type_);
            }
        }
        SyntaxExpression::Char {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
        } => {
            syntax_char_format(formatted, *content);
        }
        SyntaxExpression::Str {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
        } => {
            syntax_string_format(formatted, content);
        }
        SyntaxExpression::VariableOrCall {
            name,
            type_arguments,
            argument,
        } => {
            formatted.push_str(&name.value);
            if let Some(type_arguments) = type_arguments {
                let type_arguments_line_span =
                    range_line_span(angled_type_arguments_range(type_arguments, types));
                syntax_angled_type_arguments_format(formatted, indent, types, type_arguments);
                space_or_linebreak_indented_into(formatted, type_arguments_line_span, indent);
            } else {
                formatted.push(' ');
            }
            if let Some(argument) = argument {
                syntax_expression_unparenthesized_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    expressions.element(argument),
                );
            }
        }
        SyntaxExpression::Variant {
            name,
            type_arguments,
            value,
        } => {
            formatted.push_str(&name.value);
            if let Some(type_arguments) = type_arguments {
                let type_arguments_line_span =
                    range_line_span(angled_type_arguments_range(type_arguments, types));
                syntax_angled_type_arguments_format(formatted, indent, types, type_arguments);
                space_or_linebreak_indented_into(formatted, type_arguments_line_span, indent);
            } else {
                formatted.push(' ');
            }
            if let Some(value) = value {
                syntax_expression_unparenthesized_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    expressions.element(value),
                );
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            parameter,
            result,
        } => {
            formatted.push_str("fn ");
            if let Some(parameter) = parameter {
                let parameter_line_span =
                    range_line_span(pattern_range(parameter, patterns, types));
                syntax_pattern_parenthesized_if_open_ended_format(
                    formatted, indent, patterns, types, parameter,
                );
                space_or_linebreak_indented_into(formatted, parameter_line_span, indent);
            }
            if let Some(result) = result {
                syntax_expression_unparenthesized_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    expressions.element(result),
                );
            }
        }
        SyntaxExpression::Record {
            ampersand_start: _,
            fields,
        } => {
            formatted.push('&');
            let line_span =
                range_line_span(expression_range(expression, expressions, patterns, types));
            for field in fields {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_field_format(
                    formatted,
                    indent,
                    field,
                    |value| expression_end(value, expressions, patterns, types),
                    |formatted, indent, value| {
                        syntax_expression_unparenthesized_format(
                            formatted,
                            indent,
                            expressions,
                            patterns,
                            types,
                            value,
                        )
                    },
                );
            }
        }
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => {
                formatted.push_str("()");
            }
            Some(inner) => {
                syntax_expression_unparenthesized_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    expressions.element(inner),
                );
            }
        },
        SyntaxExpression::Commented {
            comments,
            expression,
        } => {
            syntax_comments_format(formatted, indent, comments);
            if let Some(expression) = expression {
                syntax_expression_unparenthesized_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    expressions.element(expression),
                );
            }
        }
        SyntaxExpression::Query {
            colon_start: _,
            queried,
            cases,
        } => {
            formatted.push(':');
            if let Some(queried) = queried {
                let queried = expressions.element(queried);
                let queried_line_span =
                    range_line_span(expression_range(queried, expressions, patterns, types));
                syntax_expression_parenthesized_if_open_ended_format(
                    formatted,
                    next_indent(indent),
                    expressions,
                    patterns,
                    types,
                    queried,
                );
                space_or_linebreak_indented_into(formatted, queried_line_span, indent);
            }
            for case in cases {
                syntax_expression_query_case_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    case,
                )
            }
        }
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name,
            result,
        } => {
            formatted.push_str("origin ");
            if let Some(name) = name {
                formatted.push_str(&name.value);
            }
            if let Some(result) = result {
                let line_span =
                    range_line_span(expression_range(expression, expressions, patterns, types));
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_expression_unparenthesized_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    expressions.element(result),
                );
            }
        }
    }
}
fn syntax_expression_query_case_format<Expressions, Patterns, Types>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    case: &SyntaxExpressionQueryCase<Expressions, Patterns, Types>,
) {
    let pattern_line_span = range_line_span(pattern_range(&case.pattern, patterns, types));
    syntax_pattern_parenthesized_if_open_ended_format(
        formatted,
        indent,
        patterns,
        types,
        &case.pattern,
    );
    space_or_linebreak_indented_into(formatted, pattern_line_span, next_indent(indent));
    if case.left_angle_start.is_some() {
        formatted.push_str(" <");
        if let Some(result) = &case.result {
            syntax_expression_unparenthesized_format(
                formatted,
                indent,
                expressions,
                patterns,
                types,
                result,
            );
        }
    } else {
        if let Some(result) = &case.result {
            syntax_expression_parenthesized_if_open_ended_format(
                formatted,
                next_indent(indent),
                expressions,
                patterns,
                types,
                result,
            );
        }
    }
}
fn syntax_pattern_parenthesized_if_open_ended_format<Patterns, Types>(
    formatted: &mut String,
    indent: usize,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    pattern: &SyntaxPattern<Patterns, Types>,
) {
    if syntax_pattern_is_open_ended(pattern, patterns) {
        formatted.push('(');
        let line_span = range_line_span(pattern_range(pattern, patterns, types));
        syntax_pattern_unparenthesized_format(
            formatted,
            next_indent(indent),
            patterns,
            types,
            pattern,
        );
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
        formatted.push(')');
    } else {
        syntax_pattern_unparenthesized_format(formatted, indent, patterns, types, pattern);
    }
}
fn syntax_pattern_is_open_ended<Patterns, Types>(
    pattern: &SyntaxPattern<Patterns, Types>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
) -> bool {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => type_.is_some(),
        SyntaxPattern::Variant {
            name: _,
            type_arguments: _,
            value,
        } => value.is_some(),
        SyntaxPattern::Record {
            ampersand_start: _,
            fields,
        } => !fields.is_empty(),
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner
            .as_ref()
            .is_some_and(|inner| syntax_pattern_is_open_ended(patterns.element(inner), patterns)),
    }
}
fn syntax_pattern_unparenthesized_format<Types, Patterns>(
    formatted: &mut String,
    indent: usize,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    pattern: &SyntaxPattern<Patterns, Types>,
) {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            formatted.push_str(&name.value);
            if let Some(type_) = type_ {
                formatted.push(' ');
                syntax_type_unparenthesized_format(formatted, indent, types, type_);
            }
        }
        SyntaxPattern::Variant {
            name,
            type_arguments,
            value,
        } => {
            formatted.push_str(&name.value);
            if let Some(angled_type_arguments) = type_arguments {
                syntax_angled_type_arguments_format(formatted, indent, types, angled_type_arguments)
            }
            if let Some(value) = value {
                formatted.push(' ');
                syntax_pattern_unparenthesized_format(
                    formatted,
                    indent,
                    patterns,
                    types,
                    patterns.element(value),
                );
            }
        }
        SyntaxPattern::Record {
            ampersand_start: _,
            fields,
        } => {
            formatted.push('&');
            let line_span = range_line_span(pattern_range(pattern, patterns, types));
            for field in fields {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_field_format(
                    formatted,
                    indent,
                    field,
                    |value| pattern_end(value, patterns, types),
                    |formatted, indent, value| {
                        syntax_pattern_unparenthesized_format(
                            formatted, indent, patterns, types, value,
                        )
                    },
                );
            }
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => {
                formatted.push_str("()");
            }
            Some(inner) => {
                syntax_pattern_unparenthesized_format(
                    formatted,
                    indent,
                    patterns,
                    types,
                    patterns.element(inner),
                );
            }
        },
    }
}
fn syntax_angled_type_arguments_format<Types>(
    formatted: &mut String,
    indent: usize,
    types: &core::Vec<Types, SyntaxType<Types>>,
    angled_type_arguments: &SyntaxAngledTypeArguments<Types>,
) {
    formatted.push('<');
    let line_span = range_line_span(angled_type_arguments_range(angled_type_arguments, types));
    if let Some((argument0, argument1_up)) = angled_type_arguments
        .types
        .as_ref()
        .and_then(|type_arguments| types.span_slice(type_arguments).split_first())
    {
        syntax_type_parenthesized_if_open_ended_format(
            formatted,
            next_indent(indent),
            types,
            argument0,
        );
        for argument in argument1_up {
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_type_parenthesized_if_open_ended_format(
                formatted,
                next_indent(indent),
                types,
                argument,
            );
        }
    }
    if line_span == LineSpan::Multiple {
        linebreak_indented_into(formatted, indent);
    }
    formatted.push('>');
}
fn syntax_type_parenthesized_if_open_ended_format<Types>(
    formatted: &mut String,
    indent: usize,
    types: &core::Vec<Types, SyntaxType<Types>>,
    type_: &SyntaxType<Types>,
) {
    if syntax_type_is_open_ended(type_, types) {
        formatted.push('(');
        let line_span = range_line_span(type_range(type_, types));
        syntax_type_unparenthesized_format(formatted, next_indent(indent), types, type_);
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
        formatted.push(')');
    } else {
        syntax_type_unparenthesized_format(formatted, indent, types, type_);
    }
}
fn syntax_type_is_open_ended<Types>(
    type_: &SyntaxType<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> bool {
    match type_ {
        SyntaxType::Variable(_) => false,
        SyntaxType::Record {
            ampersand_start: _,
            fields,
        } => !fields.is_empty(),
        SyntaxType::Construct { name: _, arguments } => arguments.is_some(),
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner
            .as_ref()
            .is_some_and(|inner| syntax_type_is_open_ended(types.element(inner), types)),
    }
}
fn syntax_type_unparenthesized_format<Types>(
    formatted: &mut String,
    indent: usize,
    types: &core::Vec<Types, SyntaxType<Types>>,
    type_: &SyntaxType<Types>,
) {
    match type_ {
        SyntaxType::Variable(name) => {
            formatted.push_str(&name.value);
        }
        SyntaxType::Record {
            ampersand_start: _,
            fields,
        } => {
            formatted.push('&');
            let line_span = range_line_span(type_range(type_, types));
            for field in fields {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_field_format(
                    formatted,
                    indent,
                    field,
                    |value| type_end(value, types),
                    |formatted, indent, value| {
                        syntax_type_unparenthesized_format(formatted, indent, types, value)
                    },
                );
            }
        }
        SyntaxType::Construct { name, arguments } => {
            formatted.push_str(&name.value);
            let line_span = range_line_span(type_range(type_, types));
            for argument in types.opt_span_slice(core::Opt::from_option(arguments.as_ref())) {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                syntax_type_parenthesized_if_open_ended_format(formatted, indent, types, argument);
            }
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => {
                formatted.push_str("()");
            }
            Some(inner) => {
                syntax_type_unparenthesized_format(formatted, indent, types, types.element(inner));
            }
        },
    }
}

pub enum SyntaxSymbol<'a, Expressions, Patterns, Types> {
    ProjectTypeOrUnknown {
        name: WithStartPosition<&'a Name>,
        origins: std::collections::HashMap<
            &'a Name,
            OriginStartAndScope<'a, Expressions, Patterns, Types>,
        >,
    },
    // applies to both type and variable name
    Origin {
        name: &'a Name,
        use_start: lsp_types::Position,
        origin: OriginStartAndScope<'a, Expressions, Patterns, Types>,
    },
    TypeVariable {
        name: &'a Name,
        use_start: lsp_types::Position,
        scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    },
    VariantOrUnknown(WithStartPosition<&'a Name>),
    ProjectFnOrUnknown {
        name: WithStartPosition<&'a Name>,
        pattern_variables: std::collections::HashMap<
            &'a Name,
            OriginStartAndScope<'a, Expressions, Patterns, Types>,
        >,
        origins: std::collections::HashMap<
            &'a Name,
            OriginStartAndScope<'a, Expressions, Patterns, Types>,
        >,
    },
    PatternVariable {
        name: &'a Name,
        use_start: lsp_types::Position,
        origin: OriginStartAndScope<'a, Expressions, Patterns, Types>,
    },
}
pub struct OriginStartAndScope<'a, Expressions, Patterns, Types> {
    pub start: lsp_types::Position,
    pub scope: Option<&'a SyntaxExpression<Expressions, Patterns, Types>>,
}
impl<'a, Expressions, Patterns, Types> Copy
    for OriginStartAndScope<'a, Expressions, Patterns, Types>
{
}
impl<'a, Expressions, Patterns, Types> Clone
    for OriginStartAndScope<'a, Expressions, Patterns, Types>
{
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            scope: self.scope,
        }
    }
}
pub fn syntax_project_symbol_at_position<'a, Expressions, Patterns, Types>(
    project: &'a SyntaxProject<Expressions, Patterns, Types>,
    position: lsp_types::Position,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    // TODO strongly consider binary search
    project.elements.iter().find_map(|element| match element {
        SyntaxProjectElement::TypeAlias {
            type_keyword_start: _,
            name,
            parameters,
            documentation: _,
            type_,
        } => {
            if let Some(name) = name
                && range_includes_position(
                    syntax_name_range(with_start_position_as_ref(name)),
                    position,
                )
            {
                return Some(SyntaxSymbol::ProjectTypeOrUnknown {
                    name: with_start_position_as_ref(name),
                    origins: std::collections::HashMap::new(),
                });
            }
            parameters
                .iter()
                .find_map(|name| {
                    if range_includes_position(
                        syntax_name_range(with_start_position_as_ref(name)),
                        position,
                    ) {
                        Some(SyntaxSymbol::TypeVariable {
                            name: &name.value,
                            use_start: name.start,
                            scope: element,
                        })
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    type_.as_ref().and_then(|value| {
                        syntax_type_symbol_at_position(
                            value,
                            position,
                            types,
                            element,
                            &mut std::collections::HashMap::new(),
                        )
                    })
                })
        }
        SyntaxProjectElement::ChoiceType {
            choice_keyword_start: _,
            name,
            parameters,
            documentation: _,
            variants,
        } => {
            if let Some(name) = name
                && range_includes_position(
                    syntax_name_range(with_start_position_as_ref(name)),
                    position,
                )
            {
                return Some(SyntaxSymbol::ProjectTypeOrUnknown {
                    name: with_start_position_as_ref(name),
                    origins: std::collections::HashMap::new(),
                });
            }
            parameters
                .iter()
                .find_map(|name| {
                    if range_includes_position(
                        syntax_name_range(with_start_position_as_ref(name)),
                        position,
                    ) {
                        Some(SyntaxSymbol::TypeVariable {
                            name: &name.value,
                            use_start: name.start,
                            scope: element,
                        })
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    variants.iter().find_map(|variant| {
                        syntax_variant_symbol_at_position(variant, position, types, element)
                    })
                })
        }
        SyntaxProjectElement::Fn {
            fn_keyword_start: _,
            name,
            type_parameters,
            parameter,
            result_type,
            documentation: _,
            result,
        } => {
            if let Some(name) = name
                && range_includes_position(
                    syntax_name_range(with_start_position_as_ref(name)),
                    position,
                )
            {
                return Some(SyntaxSymbol::ProjectFnOrUnknown {
                    name: with_start_position_as_ref(name),
                    pattern_variables: std::collections::HashMap::new(),
                    origins: std::collections::HashMap::new(),
                });
            }
            type_parameters
                .as_ref()
                .and_then(|type_parameters| {
                    syntax_angled_type_parameters_symbol_at_position(
                        type_parameters,
                        position,
                        element,
                    )
                })
                .or_else(|| {
                    parameter.as_ref().and_then(|parameter| {
                        syntax_pattern_symbol_at_position(
                            parameter,
                            position,
                            patterns,
                            types,
                            element,
                            result.as_ref(),
                            &mut std::collections::HashMap::new(),
                        )
                    })
                })
                .or_else(|| {
                    result_type.as_ref().and_then(|result_type| {
                        syntax_type_symbol_at_position(
                            result_type,
                            position,
                            types,
                            element,
                            &mut std::collections::HashMap::new(),
                        )
                    })
                })
                .or_else(|| {
                    result.as_ref().and_then(|result| {
                        let mut pattern_variables = std::collections::HashMap::new();
                        if let Some(parameter) = parameter {
                            syntax_pattern_variables_fold(
                                parameter,
                                (),
                                &mut |(), name, _type_| {
                                    pattern_variables.insert(
                                        name.value,
                                        OriginStartAndScope {
                                            start: name.start,
                                            scope: Some(result),
                                        },
                                    );
                                },
                                patterns,
                            )
                        }
                        let mut origins = std::collections::HashMap::new();
                        syntax_expression_symbol_at_position(
                            result,
                            position,
                            expressions,
                            patterns,
                            types,
                            element,
                            &mut pattern_variables,
                            &mut origins,
                        )
                    })
                })
        }
        SyntaxProjectElement::Comments(_) => None,
        SyntaxProjectElement::Unrecognized { .. } => None,
    })
}
fn syntax_variant_symbol_at_position<'a, Expressions, Patterns, Types>(
    variant: &'a SyntaxVariant<Types>,
    position: lsp_types::Position,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    if let Some(name) = &variant.name
        && range_includes_position(
            syntax_name_range(with_start_position_as_ref(name)),
            position,
        )
    {
        return Some(SyntaxSymbol::VariantOrUnknown(with_start_position_as_ref(
            name,
        )));
    }
    variant
        .type_parameters
        .as_ref()
        .and_then(|type_parameters| {
            syntax_angled_type_parameters_symbol_at_position(type_parameters, position, scope)
        })
        .or_else(|| {
            variant.value.as_ref().and_then(|value| {
                syntax_type_symbol_at_position(
                    value,
                    position,
                    types,
                    scope,
                    &mut std::collections::HashMap::new(),
                )
            })
        })
}
fn syntax_expression_symbol_at_position<'a, Expressions, Patterns, Types>(
    expression: &'a SyntaxExpression<Expressions, Patterns, Types>,
    position: lsp_types::Position,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    pattern_variables: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
    origins: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    if !range_includes_position(
        expression_range(expression, expressions, patterns, types),
        position,
    ) {
        return None;
    }
    match expression {
        SyntaxExpression::Number { value: _, type_ } => type_.as_ref().and_then(|value| {
            syntax_type_symbol_at_position(value, position, types, scope, origins)
        }),
        SyntaxExpression::Char { .. } => None,
        SyntaxExpression::Str { .. } => None,
        SyntaxExpression::VariableOrCall {
            name,
            type_arguments,
            argument,
        } => {
            if range_includes_position(
                syntax_name_range(with_start_position_as_ref(name)),
                position,
            ) {
                return Some(match pattern_variables.get(&name.value) {
                    None => SyntaxSymbol::ProjectFnOrUnknown {
                        name: with_start_position_as_ref(name),
                        pattern_variables: std::mem::take(pattern_variables),
                        origins: std::mem::take(origins),
                    },
                    Some(pattern_variable) => SyntaxSymbol::PatternVariable {
                        name: &name.value,
                        use_start: name.start,
                        origin: *pattern_variable,
                    },
                });
            }
            type_arguments
                .as_ref()
                .and_then(|type_arguments| {
                    syntax_angled_type_arguments_symbol_at_position(
                        type_arguments,
                        position,
                        types,
                        scope,
                        origins,
                    )
                })
                .or_else(|| {
                    argument.as_ref().and_then(|argument| {
                        syntax_expression_symbol_at_position(
                            expressions.element(argument),
                            position,
                            expressions,
                            patterns,
                            types,
                            scope,
                            pattern_variables,
                            origins,
                        )
                    })
                })
        }
        SyntaxExpression::Variant {
            name,
            type_arguments,
            value,
        } => {
            if range_includes_position(
                syntax_name_range(with_start_position_as_ref(name)),
                position,
            ) {
                return Some(SyntaxSymbol::VariantOrUnknown(with_start_position_as_ref(
                    name,
                )));
            }
            type_arguments
                .as_ref()
                .and_then(|type_arguments| {
                    syntax_angled_type_arguments_symbol_at_position(
                        type_arguments,
                        position,
                        types,
                        scope,
                        origins,
                    )
                })
                .or_else(|| {
                    value.as_ref().and_then(|value| {
                        syntax_expression_symbol_at_position(
                            expressions.element(value),
                            position,
                            expressions,
                            patterns,
                            types,
                            scope,
                            pattern_variables,
                            origins,
                        )
                    })
                })
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            parameter,
            result,
        } => {
            let result = result.as_ref().map(|result| expressions.element(result));
            pattern_variables.clear();
            parameter
                .as_ref()
                .and_then(|parameter| {
                    syntax_pattern_symbol_at_position(
                        parameter, position, patterns, types, scope, result, origins,
                    )
                })
                .or_else(|| {
                    result.as_ref().and_then(|result| {
                        if let Some(parameter) = parameter {
                            syntax_pattern_variables_fold(
                                parameter,
                                (),
                                &mut |(), name, _type_| {
                                    pattern_variables.insert(
                                        name.value,
                                        OriginStartAndScope {
                                            start: name.start,
                                            scope: Some(result),
                                        },
                                    );
                                },
                                patterns,
                            )
                        };
                        syntax_expression_symbol_at_position(
                            result,
                            position,
                            expressions,
                            patterns,
                            types,
                            scope,
                            pattern_variables,
                            origins,
                        )
                    })
                })
        }
        SyntaxExpression::Record {
            ampersand_start: _,
            fields,
        } => syntax_fields_find_symbol_at_position(fields, |value| {
            syntax_expression_symbol_at_position(
                value,
                position,
                expressions,
                patterns,
                types,
                scope,
                pattern_variables,
                origins,
            )
        }),
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().and_then(|inner| {
            syntax_expression_symbol_at_position(
                expressions.element(inner),
                position,
                expressions,
                patterns,
                types,
                scope,
                pattern_variables,
                origins,
            )
        }),
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => expression.as_ref().and_then(|expression| {
            syntax_expression_symbol_at_position(
                expressions.element(expression),
                position,
                expressions,
                patterns,
                types,
                scope,
                pattern_variables,
                origins,
            )
        }),
        SyntaxExpression::Query {
            colon_start: _,
            queried,
            cases,
        } => queried
            .as_ref()
            .and_then(|queried| {
                syntax_expression_symbol_at_position(
                    expressions.element(queried),
                    position,
                    expressions,
                    patterns,
                    types,
                    scope,
                    pattern_variables,
                    origins,
                )
            })
            .or_else(|| {
                cases.iter().find_map(|case| {
                    syntax_expression_query_case_symbol_at_position(
                        case,
                        position,
                        expressions,
                        patterns,
                        types,
                        scope,
                        pattern_variables,
                        origins,
                    )
                })
            }),
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name,
            result,
        } => {
            let result = result.as_ref().map(|result| expressions.element(result));
            if let Some(name) = name {
                let origin_info = OriginStartAndScope {
                    start: name.start,
                    scope: result,
                };
                origins.insert(&name.value, origin_info);
                if range_includes_position(
                    syntax_name_range(with_start_position_as_ref(name)),
                    position,
                ) {
                    return Some(SyntaxSymbol::Origin {
                        name: &name.value,
                        use_start: name.start,
                        origin: origin_info,
                    });
                }
            }
            result.as_ref().and_then(|result| {
                syntax_expression_symbol_at_position(
                    result,
                    position,
                    expressions,
                    patterns,
                    types,
                    scope,
                    pattern_variables,
                    origins,
                )
            })
        }
    }
}
fn syntax_expression_query_case_symbol_at_position<'a, Expressions, Patterns, Types>(
    case: &'a SyntaxExpressionQueryCase<Expressions, Patterns, Types>,
    position: lsp_types::Position,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    pattern_variables: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
    origins: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    syntax_pattern_symbol_at_position(
        &case.pattern,
        position,
        patterns,
        types,
        scope,
        case.result.as_ref(),
        origins,
    )
    .or_else(|| {
        let Some(result) = &case.result else {
            return None;
        };
        // don't modify pattern_variables unless known to be in the right case
        if !range_includes_position(
            expression_range(result, expressions, patterns, types),
            position,
        ) {
            return None;
        }
        syntax_pattern_variables_fold(
            &case.pattern,
            (),
            &mut |(), name, _type_| {
                pattern_variables.insert(
                    name.value,
                    OriginStartAndScope {
                        start: name.start,
                        scope: Some(result),
                    },
                );
            },
            patterns,
        );
        syntax_expression_symbol_at_position(
            result,
            position,
            expressions,
            patterns,
            types,
            scope,
            pattern_variables,
            origins,
        )
    })
}
fn syntax_pattern_variables_fold<'a, Patterns, Types, State>(
    pattern: &'a SyntaxPattern<Patterns, Types>,
    state: State,
    reduce: &mut impl FnMut(State, WithStartPosition<&'a Name>, Option<&'a SyntaxType<Types>>) -> State,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
) -> State {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            reduce(state, with_start_position_as_ref(name), type_.as_ref())
        }
        SyntaxPattern::Variant {
            name: _,
            type_arguments: _,
            value,
        } => match value {
            None => state,
            Some(value) => {
                syntax_pattern_variables_fold(patterns.element(value), state, reduce, patterns)
            }
        },
        SyntaxPattern::Record {
            ampersand_start: _,
            fields,
        } => fields.iter().fold(state, |state, field| {
            if let Some(value) = &field.value {
                syntax_pattern_variables_fold(value, state, reduce, patterns)
            } else {
                state
            }
        }),
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => state,
            Some(inner) => {
                syntax_pattern_variables_fold(patterns.element(inner), state, reduce, patterns)
            }
        },
    }
}
fn syntax_pattern_symbol_at_position<'a, Expressions, Patterns, Types>(
    pattern: &'a SyntaxPattern<Patterns, Types>,
    position: lsp_types::Position,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    project_element_scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    expression_scope: Option<&'a SyntaxExpression<Expressions, Patterns, Types>>,
    origins: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    if !range_includes_position(pattern_range(pattern, patterns, types), position) {
        return None;
    }
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            if range_includes_position(
                syntax_name_range(with_start_position_as_ref(name)),
                position,
            ) {
                return Some(SyntaxSymbol::PatternVariable {
                    name: &name.value,
                    use_start: name.start,
                    origin: OriginStartAndScope {
                        start: name.start,
                        scope: expression_scope,
                    },
                });
            }
            type_.as_ref().and_then(|type_| {
                syntax_type_symbol_at_position(
                    type_,
                    position,
                    types,
                    project_element_scope,
                    origins,
                )
            })
        }
        SyntaxPattern::Variant {
            name,
            type_arguments,
            value,
        } => {
            if range_includes_position(
                syntax_name_range(with_start_position_as_ref(name)),
                position,
            ) {
                return Some(SyntaxSymbol::VariantOrUnknown(with_start_position_as_ref(
                    name,
                )));
            }
            type_arguments
                .as_ref()
                .and_then(|type_arguments| {
                    syntax_angled_type_arguments_symbol_at_position(
                        type_arguments,
                        position,
                        types,
                        project_element_scope,
                        origins,
                    )
                })
                .or_else(|| {
                    value.as_ref().and_then(|value| {
                        syntax_pattern_symbol_at_position(
                            patterns.element(value),
                            position,
                            patterns,
                            types,
                            project_element_scope,
                            expression_scope,
                            origins,
                        )
                    })
                })
        }
        SyntaxPattern::Record {
            ampersand_start: _,
            fields,
        } => syntax_fields_find_symbol_at_position(fields, |value| {
            syntax_pattern_symbol_at_position(
                value,
                position,
                patterns,
                types,
                project_element_scope,
                expression_scope,
                origins,
            )
        }),
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().and_then(|inner| {
            syntax_pattern_symbol_at_position(
                patterns.element(inner),
                position,
                patterns,
                types,
                project_element_scope,
                expression_scope,
                origins,
            )
        }),
    }
}
fn syntax_type_symbol_at_position<'a, Expressions, Patterns, Types>(
    type_: &'a SyntaxType<Types>,
    position: lsp_types::Position,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    origins: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    if !range_includes_position(type_range(type_, types), position) {
        return None;
    }
    match type_ {
        SyntaxType::Variable(name) => {
            if range_includes_position(
                syntax_name_range(with_start_position_as_ref(name)),
                position,
            ) {
                Some(SyntaxSymbol::TypeVariable {
                    name: &name.value,
                    use_start: name.start,
                    scope: scope,
                })
            } else {
                None
            }
        }
        SyntaxType::Record {
            ampersand_start: _,
            fields,
        } => syntax_fields_find_symbol_at_position(fields, |value| {
            syntax_type_symbol_at_position(value, position, types, scope, origins)
        }),
        SyntaxType::Construct { name, arguments } => {
            if range_includes_position(
                syntax_name_range(with_start_position_as_ref(name)),
                position,
            ) {
                return Some(match origins.get(&name.value) {
                    Some(&origin_info) => SyntaxSymbol::Origin {
                        name: &name.value,
                        use_start: name.start,
                        origin: origin_info,
                    },
                    None => SyntaxSymbol::ProjectTypeOrUnknown {
                        name: with_start_position_as_ref(name),
                        origins: std::mem::take(origins),
                    },
                });
            }
            types
                .opt_span_slice(core::Opt::from_option(arguments.as_ref()))
                .iter()
                .find_map(|argument| {
                    syntax_type_symbol_at_position(argument, position, types, scope, origins)
                })
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().and_then(|inner| {
            syntax_type_symbol_at_position(types.element(inner), position, types, scope, origins)
        }),
    }
}
fn syntax_angled_type_parameters_symbol_at_position<'a, Expressions, Patterns, Types>(
    angled_type_parameters: &'a SyntaxAngledTypeParameters,
    position: lsp_types::Position,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    angled_type_parameters.names.iter().find_map(|name| {
        if range_includes_position(
            syntax_name_range(with_start_position_as_ref(name)),
            position,
        ) {
            Some(SyntaxSymbol::TypeVariable {
                name: &name.value,
                use_start: name.start,
                scope: scope,
            })
        } else {
            None
        }
    })
}
fn syntax_angled_type_arguments_symbol_at_position<'a, Expressions, Patterns, Types>(
    angled_type_parameters: &'a SyntaxAngledTypeArguments<Types>,
    position: lsp_types::Position,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    origins: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    types
        .opt_span_slice(core::Opt::from_option(
            angled_type_parameters.types.as_ref(),
        ))
        .iter()
        .find_map(|type_| syntax_type_symbol_at_position(type_, position, types, scope, origins))
}
fn syntax_fields_find_symbol_at_position<'a, Value, Expressions, Patterns, Types>(
    fields: &'a [SyntaxField<Value>],
    mut value_symbol_at_position: impl FnMut(
        &'a Value,
    )
        -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    fields.iter().find_map(|field| {
        field
            .value
            .as_ref()
            .and_then(|value| value_symbol_at_position(value))
    })
}

pub fn syntax_project_symbol_origin_range<Expressions, Patterns, Types>(
    project: &SyntaxProject<Expressions, Patterns, Types>,
    symbol: &SyntaxSymbol<Expressions, Patterns, Types>,
) -> Option<lsp_types::Range> {
    match symbol {
        SyntaxSymbol::ProjectTypeOrUnknown {
            name: symbol_name,
            origins: _,
        } => project.elements.iter().find_map(|element| match element {
            SyntaxProjectElement::TypeAlias {
                type_keyword_start: _,
                name: type_alias_name,
                parameters: _,
                documentation: _,
                type_: _,
            } => {
                if let Some(type_alias_name) = type_alias_name
                    && type_alias_name.value == symbol_name.value
                {
                    Some(syntax_name_range(with_start_position_as_ref(
                        type_alias_name,
                    )))
                } else {
                    None
                }
            }
            SyntaxProjectElement::ChoiceType {
                choice_keyword_start: _,
                name: choice_type_name,
                parameters: _,
                documentation: _,
                variants: _,
            } => {
                if let Some(choice_type_name) = choice_type_name
                    && choice_type_name.value == symbol_name.value
                {
                    Some(syntax_name_range(with_start_position_as_ref(
                        choice_type_name,
                    )))
                } else {
                    None
                }
            }
            SyntaxProjectElement::Fn { .. } => None,
            SyntaxProjectElement::Comments(_) => None,
            SyntaxProjectElement::Unrecognized { .. } => None,
        }),
        SyntaxSymbol::Origin {
            name,
            use_start: _,
            origin,
        } => Some(syntax_name_range(WithStartPosition {
            value: name,
            start: origin.start,
        })),
        SyntaxSymbol::TypeVariable {
            name: symbol_name,
            use_start: _,
            scope,
        } => match scope {
            SyntaxProjectElement::TypeAlias {
                type_keyword_start: _,
                name: _,
                parameters,
                documentation: _,
                type_: _,
            } => parameters.iter().find_map(|parameter| {
                if &parameter.value == symbol_name {
                    Some(syntax_name_range(with_start_position_as_ref(parameter)))
                } else {
                    None
                }
            }),
            SyntaxProjectElement::ChoiceType {
                choice_keyword_start: _,
                name: _,
                parameters,
                documentation: _,
                variants: _,
            } => parameters.iter().find_map(|parameter| {
                if &parameter.value == symbol_name {
                    Some(syntax_name_range(with_start_position_as_ref(parameter)))
                } else {
                    None
                }
            }),
            SyntaxProjectElement::Fn {
                fn_keyword_start: _,
                name: _,
                type_parameters,
                parameter: _,
                result_type: _,
                documentation: _,
                result: _,
            } => type_parameters
                .as_ref()
                .into_iter()
                .flat_map(|type_parameters| &type_parameters.names)
                .find_map(|parameter| {
                    if &parameter.value == symbol_name {
                        Some(syntax_name_range(with_start_position_as_ref(parameter)))
                    } else {
                        None
                    }
                }),
            SyntaxProjectElement::Comments(_) => None,
            SyntaxProjectElement::Unrecognized { .. } => None,
        },
        SyntaxSymbol::VariantOrUnknown(symbol_name) => {
            project.elements.iter().find_map(|element| match element {
                SyntaxProjectElement::ChoiceType {
                    choice_keyword_start: _,
                    name: _,
                    parameters: _,
                    documentation: _,
                    variants,
                } => variants.iter().find_map(|variant| {
                    if let Some(variant_name) = &variant.name
                        && variant_name.value == symbol_name.value
                    {
                        Some(syntax_name_range(with_start_position_as_ref(variant_name)))
                    } else {
                        None
                    }
                }),
                _ => None,
            })
        }
        SyntaxSymbol::ProjectFnOrUnknown {
            name: symbol_name,
            pattern_variables: _,
            origins: _,
        } => project.elements.iter().find_map(|element| match element {
            SyntaxProjectElement::Fn {
                fn_keyword_start: _,
                name: Some(fn_name),
                type_parameters: _,
                parameter: _,
                result_type: _,
                documentation: _,
                result: _,
            } if fn_name.value == symbol_name.value => todo!(),
            _ => None,
        }),
        SyntaxSymbol::PatternVariable {
            name,
            use_start: _,
            origin,
        } => Some(syntax_name_range(WithStartPosition {
            value: name,
            start: origin.start,
        })),
    }
}
/// resulting uses do not include the origin. For that, use syntax_project_symbol_origin_range
pub fn syntax_project_symbol_uses<Expressions, Patterns, Types>(
    project: &SyntaxProject<Expressions, Patterns, Types>,
    symbol: &SyntaxSymbol<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> Vec<lsp_types::Range> {
    let mut uses = Vec::new();
    match symbol {
        SyntaxSymbol::Origin {
            name: _,
            use_start: _,
            origin,
        } => {
            if let Some(origin_scope) = origin.scope {
                syntax_expression_symbol_uses_into(
                    &mut uses,
                    origin_scope,
                    symbol,
                    expressions,
                    patterns,
                    types,
                    &std::collections::HashSet::new(),
                    &std::collections::HashSet::new(),
                )
            }
        }
        SyntaxSymbol::TypeVariable {
            name: symbol_name,
            use_start: _,
            scope,
        } => match scope {
            SyntaxProjectElement::TypeAlias {
                type_keyword_start: _,
                name: _,
                parameters: _,
                documentation: _,
                type_,
            } => {
                if let Some(type_) = type_ {
                    syntax_type_symbol_uses_into(
                        &mut uses,
                        type_,
                        symbol,
                        types,
                        &std::collections::HashSet::new(),
                    );
                }
            }
            SyntaxProjectElement::ChoiceType {
                choice_keyword_start: _,
                name: _,
                parameters: _,
                documentation: _,
                variants,
            } => {
                for variant in variants.iter() {
                    if let Some(type_parameters) = &variant.type_parameters
                        && let Some(type_parameter_use) = type_parameters
                            .names
                            .iter()
                            .find(|name| name.value == symbol_name)
                    {
                        uses.push(syntax_name_range(with_start_position_as_ref(
                            type_parameter_use,
                        )));
                    }
                    if let Some(value) = &variant.value {
                        syntax_type_symbol_uses_into(
                            &mut uses,
                            value,
                            symbol,
                            types,
                            &std::collections::HashSet::new(),
                        );
                    }
                }
            }
            SyntaxProjectElement::Fn {
                fn_keyword_start: _,
                name: _,
                type_parameters: _,
                parameter,
                result_type,
                documentation: _,
                result,
            } => {
                if let Some(parameter) = parameter {
                    syntax_pattern_symbol_uses_into(
                        &mut uses,
                        parameter,
                        symbol,
                        patterns,
                        types,
                        &std::collections::HashSet::new(),
                    );
                }
                if let Some(result_type) = result_type {
                    syntax_type_symbol_uses_into(
                        &mut uses,
                        result_type,
                        symbol,
                        types,
                        &std::collections::HashSet::new(),
                    );
                }
                if let Some(result) = result {
                    syntax_expression_symbol_uses_into(
                        &mut uses,
                        result,
                        symbol,
                        expressions,
                        patterns,
                        types,
                        &std::collections::HashSet::new(),
                        &std::collections::HashSet::new(),
                    );
                }
            }
            SyntaxProjectElement::Comments(_) => {}
            SyntaxProjectElement::Unrecognized { .. } => {}
        },
        SyntaxSymbol::PatternVariable {
            name: _,
            use_start: _,
            origin,
        } => {
            if let Some(scope) = origin.scope {
                syntax_expression_symbol_uses_into(
                    &mut uses,
                    scope,
                    symbol,
                    expressions,
                    patterns,
                    types,
                    &std::collections::HashSet::new(),
                    &std::collections::HashSet::new(),
                );
            }
        }
        SyntaxSymbol::ProjectTypeOrUnknown { .. } => {
            for element in &project.elements {
                match element {
                    SyntaxProjectElement::Fn {
                        fn_keyword_start: _,
                        name: _,
                        type_parameters: _,
                        parameter,
                        result_type,
                        documentation: _,
                        result,
                    } => {
                        if let Some(parameter) = parameter {
                            syntax_pattern_symbol_uses_into(
                                &mut uses,
                                parameter,
                                symbol,
                                patterns,
                                types,
                                &std::collections::HashSet::new(),
                            );
                        }
                        if let Some(result_type) = result_type {
                            syntax_type_symbol_uses_into(
                                &mut uses,
                                result_type,
                                symbol,
                                types,
                                &std::collections::HashSet::new(),
                            );
                        }
                        if let Some(result) = result {
                            syntax_expression_symbol_uses_into(
                                &mut uses,
                                result,
                                symbol,
                                expressions,
                                patterns,
                                types,
                                &std::collections::HashSet::new(),
                                &std::collections::HashSet::new(),
                            );
                        }
                    }
                    SyntaxProjectElement::TypeAlias {
                        type_keyword_start: _,
                        name: _,
                        parameters: _,
                        documentation: _,
                        type_,
                    } => {
                        if let Some(type_) = type_ {
                            syntax_type_symbol_uses_into(
                                &mut uses,
                                type_,
                                symbol,
                                types,
                                &std::collections::HashSet::new(),
                            );
                        }
                    }
                    SyntaxProjectElement::ChoiceType {
                        choice_keyword_start: _,
                        name: _,
                        parameters: _,
                        documentation: _,
                        variants,
                    } => {
                        for variant_value in
                            variants.iter().filter_map(|variant| variant.value.as_ref())
                        {
                            syntax_type_symbol_uses_into(
                                &mut uses,
                                variant_value,
                                symbol,
                                types,
                                &std::collections::HashSet::new(),
                            );
                        }
                    }
                    SyntaxProjectElement::Comments(_) => {}
                    SyntaxProjectElement::Unrecognized { .. } => {}
                }
            }
        }
        SyntaxSymbol::VariantOrUnknown(_) | SyntaxSymbol::ProjectFnOrUnknown { .. } => {
            for element in &project.elements {
                match element {
                    SyntaxProjectElement::Fn {
                        fn_keyword_start: _,
                        name: _,
                        type_parameters: _,
                        parameter,
                        result_type,
                        documentation: _,
                        result,
                    } => {
                        if let Some(parameter) = parameter {
                            syntax_pattern_symbol_uses_into(
                                &mut uses,
                                parameter,
                                symbol,
                                patterns,
                                types,
                                &std::collections::HashSet::new(),
                            );
                        }
                        if let Some(result_type) = result_type {
                            syntax_type_symbol_uses_into(
                                &mut uses,
                                result_type,
                                symbol,
                                types,
                                &std::collections::HashSet::new(),
                            );
                        }
                        if let Some(result) = result {
                            let mut parameter_introduced_variables =
                                std::collections::HashSet::new();
                            if let Some(parameter) = parameter {
                                syntax_pattern_variables_fold(
                                    parameter,
                                    (),
                                    &mut |(), parameter_introduced_variable_name, _type_| {
                                        parameter_introduced_variables
                                            .insert(parameter_introduced_variable_name.value);
                                    },
                                    patterns,
                                )
                            }
                            syntax_expression_symbol_uses_into(
                                &mut uses,
                                result,
                                symbol,
                                expressions,
                                patterns,
                                types,
                                &parameter_introduced_variables,
                                &std::collections::HashSet::new(),
                            );
                        }
                    }
                    SyntaxProjectElement::TypeAlias { .. } => {}
                    SyntaxProjectElement::ChoiceType { .. } => {}
                    SyntaxProjectElement::Comments(_) => {}
                    SyntaxProjectElement::Unrecognized { .. } => {}
                }
            }
        }
    }
    uses
}
fn syntax_type_symbol_uses_into<Expressions, Patterns, Types>(
    uses: &mut Vec<lsp_types::Range>,
    type_: &SyntaxType<Types>,
    symbol: &SyntaxSymbol<Expressions, Patterns, Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashSet<&Name>,
) {
    match type_ {
        SyntaxType::Variable(name) => {
            if let SyntaxSymbol::TypeVariable {
                name: symbol_name,
                use_start: _,
                scope: _,
            } = symbol
                && &name.value == symbol_name
            {
                uses.push(syntax_name_range(with_start_position_as_ref(name)));
            }
        }
        SyntaxType::Record {
            ampersand_start: _,
            fields,
        } => {
            for field_value in fields.iter().filter_map(|field| field.value.as_ref()) {
                syntax_type_symbol_uses_into(uses, field_value, symbol, types, origins);
            }
        }
        SyntaxType::Construct { name, arguments } => {
            if let SyntaxSymbol::ProjectTypeOrUnknown {
                name: symbol_name,
                origins: _,
            } = symbol
                && name.value == symbol_name.value
                && !origins.contains(&name.value)
            {}
            for argument in types.opt_span_slice(core::Opt::from_option(arguments.as_ref())) {
                syntax_type_symbol_uses_into(uses, argument, symbol, types, origins);
            }
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_type_symbol_uses_into(uses, types.element(inner), symbol, types, origins)
            }
        }
    }
}
fn syntax_pattern_symbol_uses_into<Expressions, Patterns, Types>(
    uses: &mut Vec<lsp_types::Range>,
    pattern: &SyntaxPattern<Patterns, Types>,
    symbol: &SyntaxSymbol<Expressions, Patterns, Types>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashSet<&Name>,
) {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => {
            if let Some(type_) = type_ {
                syntax_type_symbol_uses_into(uses, type_, symbol, types, origins);
            }
        }
        SyntaxPattern::Variant {
            name,
            type_arguments,
            value,
        } => {
            if let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && name.value == symbol_name.value
            {
                uses.push(syntax_name_range(with_start_position_as_ref(name)));
            }
            for type_argument in type_arguments.iter().flat_map(|angled| {
                types.opt_span_slice(core::Opt::from_option(angled.types.as_ref()))
            }) {
                syntax_type_symbol_uses_into(uses, type_argument, symbol, types, origins);
            }
            if let Some(value) = value {
                syntax_pattern_symbol_uses_into(
                    uses,
                    patterns.element(value),
                    symbol,
                    patterns,
                    types,
                    origins,
                );
            }
        }
        SyntaxPattern::Record {
            ampersand_start: _,
            fields,
        } => {
            for field in fields {
                if let Some(field_value) = &field.value {
                    syntax_pattern_symbol_uses_into(
                        uses,
                        field_value,
                        symbol,
                        patterns,
                        types,
                        origins,
                    );
                }
            }
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_pattern_symbol_uses_into(
                    uses,
                    patterns.element(inner),
                    symbol,
                    patterns,
                    types,
                    origins,
                );
            }
        }
    }
}
fn syntax_expression_symbol_uses_into<Expressions, Patterns, Types>(
    uses: &mut Vec<lsp_types::Range>,
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
    symbol: &SyntaxSymbol<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    pattern_variables: &std::collections::HashSet<&Name>,
    origins: &std::collections::HashSet<&Name>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {}
        SyntaxExpression::Char { .. } => {}
        SyntaxExpression::Str { .. } => {}
        SyntaxExpression::VariableOrCall {
            name,
            type_arguments,
            argument,
        } => {
            {
                match symbol {
                    SyntaxSymbol::TypeVariable { .. }
                    | SyntaxSymbol::ProjectTypeOrUnknown { .. }
                    | SyntaxSymbol::VariantOrUnknown(_) => todo!(),
                    SyntaxSymbol::Origin {
                        name: symbol_name,
                        use_start: _,
                        origin: _,
                    }
                    | SyntaxSymbol::PatternVariable {
                        name: symbol_name,
                        use_start: _,
                        origin: _,
                    }
                    | SyntaxSymbol::ProjectFnOrUnknown {
                        name:
                            WithStartPosition {
                                start: _,
                                value: symbol_name,
                            },
                        pattern_variables: _,
                        origins: _,
                    } => {
                        if symbol_name == &name.value
                            && !pattern_variables.contains(&name.value)
                            && !origins.contains(&name.value)
                        {
                            uses.push(syntax_name_range(with_start_position_as_ref(name)));
                        }
                    }
                }
            }
            for type_argument in type_arguments.iter().flat_map(|angled| {
                types.opt_span_slice(core::Opt::from_option(angled.types.as_ref()))
            }) {
                syntax_type_symbol_uses_into(uses, type_argument, symbol, types, origins);
            }
            if let Some(argument) = argument {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(argument),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
            }
        }
        SyntaxExpression::Variant {
            name,
            type_arguments,
            value,
        } => {
            if let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && name.value == symbol_name.value
            {
                uses.push(syntax_name_range(with_start_position_as_ref(name)));
            }
            for type_argument in type_arguments.iter().flat_map(|angled| {
                types.opt_span_slice(core::Opt::from_option(angled.types.as_ref()))
            }) {
                syntax_type_symbol_uses_into(uses, type_argument, symbol, types, origins);
            }
            if let Some(value) = value {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(value),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            parameter,
            result,
        } => {
            if let Some(parameter) = parameter {
                syntax_pattern_symbol_uses_into(uses, parameter, symbol, patterns, types, origins);
            }
            if let Some(result) = result {
                let mut parameter_pattern_variables = std::borrow::Cow::Borrowed(pattern_variables);
                if let Some(parameter) = parameter {
                    syntax_pattern_symbol_uses_into(
                        uses, parameter, symbol, patterns, types, origins,
                    );
                    syntax_pattern_variables_fold(
                        parameter,
                        (),
                        &mut |(), pattern_variable_name, _type_| {
                            parameter_pattern_variables
                                .to_mut()
                                .insert(&pattern_variable_name.value);
                        },
                        patterns,
                    );
                }
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(result),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    &std::collections::HashSet::new(),
                    origins,
                );
            }
        }
        SyntaxExpression::Record {
            ampersand_start: _,
            fields,
        } => {
            for field in fields {
                if let Some(field_value) = &field.value {
                    syntax_expression_symbol_uses_into(
                        uses,
                        field_value,
                        symbol,
                        expressions,
                        patterns,
                        types,
                        pattern_variables,
                        origins,
                    );
                }
            }
        }
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(inner),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(expression) = expression {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(expression),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
            }
        }
        SyntaxExpression::Query {
            colon_start: _,
            queried,
            cases,
        } => {
            if let Some(queried) = queried {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(queried),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
            }
            for case in cases {
                if let Some(result) = &case.result {
                    let mut pattern_variables = std::borrow::Cow::Borrowed(pattern_variables);
                    syntax_pattern_symbol_uses_into(
                        uses,
                        &case.pattern,
                        symbol,
                        patterns,
                        types,
                        origins,
                    );
                    syntax_pattern_variables_fold(
                        &case.pattern,
                        (),
                        &mut |(), pattern_variable_name, _type_| {
                            pattern_variables
                                .to_mut()
                                .insert(&pattern_variable_name.value);
                        },
                        patterns,
                    );
                    syntax_expression_symbol_uses_into(
                        uses,
                        result,
                        symbol,
                        expressions,
                        patterns,
                        types,
                        &pattern_variables,
                        origins,
                    );
                }
            }
        }
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name: introduced_origin_name,
            result,
        } => {
            if let Some(result) = result {
                let mut origins = std::borrow::Cow::Borrowed(origins);
                if let Some(introduced_origin_name) = introduced_origin_name {
                    if let SyntaxSymbol::Origin {
                        name: symbol_name,
                        use_start: _,
                        origin: _,
                    }
                    | SyntaxSymbol::PatternVariable {
                        name: symbol_name,
                        use_start: _,
                        origin: _,
                    } = symbol
                        && symbol_name == &introduced_origin_name.value
                    {
                        return;
                    }
                    origins.to_mut().insert(&introduced_origin_name.value);
                };
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(result),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    &origins,
                );
            }
        }
    }
}

fn symbol_range(start: lsp_types::Position, symbol: &'static str) -> lsp_types::Range {
    lsp_types::Range {
        start: start,
        end: symbol_end(start, symbol),
    }
}
fn symbol_end(start: lsp_types::Position, symbol: &'static str) -> lsp_types::Position {
    position_add_characters(start, symbol.len() as u32)
}
pub fn with_start_position_as_ref<Value>(
    with_start_position: &WithStartPosition<Value>,
) -> WithStartPosition<&Value> {
    WithStartPosition {
        start: with_start_position.start,
        value: &with_start_position.value,
    }
}
fn range_includes_position(range: lsp_types::Range, position: lsp_types::Position) -> bool {
    position >= range.start && position <= range.end
}
fn position_add_characters(
    position: lsp_types::Position,
    additional_characters: u32,
) -> lsp_types::Position {
    lsp_types::Position {
        line: position.line,
        character: position.character + additional_characters,
    }
}
fn position_to_string(lsp_position: lsp_types::Position) -> String {
    format!("{}:{}", lsp_position.line, lsp_position.character)
}
fn index_to_th(index: usize) -> String {
    let n = index + 1;
    let last_digit = n % 10;
    let th = match last_digit {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{n}{th}")
}
