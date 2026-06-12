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
        ty_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        parameters: Option<TyParameters>,
        documentation: Option<SyntaxComments>,
        type_: Option<SyntaxType<Types>>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        type_parameters: Option<SyntaxAngledTypeParameters>,
        parameter: Option<SyntaxPattern<Patterns, Types>>,
        arrow_start: Option<lsp_types::Position>,
        result_type: Option<SyntaxType<Types>>,
        angle_right_start: Option<lsp_types::Position>,
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
#[derive(Clone, Debug)]
pub struct TyParameters {
    pub parameter0: WithStartPosition<Name>,
    pub parameter1_up: Vec<TyTrailingParameter>,
}
#[derive(Clone, Debug)]
pub struct TyTrailingParameter {
    pub comma_start: lsp_types::Position,
    pub name: Option<WithStartPosition<Name>>,
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
    ConstructWithoutArguments(WithStartPosition<Name>),
    ConstructWithArguments {
        underscore_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        argument0: Option<core::Slot<Types>>,
        argument1_up: Vec<SyntaxTypeConstructTrailingArgument<Types>>,
    },
    Parenthesized {
        open_paren_start: lsp_types::Position,
        inner: Option<core::Slot<Types>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    RecordEmpty {
        dot_start: lsp_types::Position,
    },
    Record {
        field0_name: WithStartPosition<Name>,
        field0_value: Option<core::Slot<Types>>,
        field1_up: Vec<SyntaxTrailingField<SyntaxType<Types>>>,
    },
    ChoiceEmpty {
        bar_start: lsp_types::Position,
    },
    Choice {
        variant0_name: WithStartPosition<Name>,
        variant0_value: Option<core::Slot<Types>>,
        variant1_up: Vec<SyntaxTypeTrailingVariant<Types>>,
    },
}
#[derive(Debug)]
pub struct SyntaxTypeConstructTrailingArgument<Types> {
    pub comma_start: lsp_types::Position,
    pub type_: Option<SyntaxType<Types>>,
}
#[derive(Debug)]
pub struct SyntaxTypeTrailingVariant<Types> {
    pub name: WithStartPosition<Option<Name>>,
    pub value: Option<SyntaxType<Types>>,
}
#[derive(Debug)]
pub enum SyntaxPattern<Patterns, Types> {
    Variable {
        name: WithStartPosition<Name>,
        type_: Option<SyntaxType<Types>>,
    },
    Variant {
        name: WithStartPosition<Option<Name>>,
        value: Option<core::Slot<Patterns>>,
    },
    RecordEmpty {
        dot_start: lsp_types::Position,
    },
    Record {
        field0_name: WithStartPosition<Name>,
        field0_value: Option<core::Slot<Patterns>>,
        field1_up: Vec<SyntaxTrailingField<SyntaxPattern<Patterns, Types>>>,
    },
    Parenthesized {
        open_paren_start: lsp_types::Position,
        inner: Option<core::Slot<Patterns>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
}
#[derive(Clone, Debug)]
pub struct SyntaxTrailingField<Value> {
    pub name: WithStartPosition<Option<Name>>,
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
    Variable(WithStartPosition<Name>),
    Call {
        underscore_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        type_arguments: Option<SyntaxAngledTypeArguments<Types>>,
        argument: Option<core::Slot<Expressions>>,
    },
    Variant {
        name: WithStartPosition<Option<Name>>,
        type_: Option<SyntaxType<Types>>,
        value: Option<core::Slot<Expressions>>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        parameter: Option<SyntaxPattern<Patterns, Types>>,
        angle_right_start: Option<lsp_types::Position>,
        result: Option<core::Slot<Expressions>>,
    },
    RecordEmpty {
        dot_start: lsp_types::Position,
    },
    Record {
        field0_name: WithStartPosition<Name>,
        field0_value: Option<core::Slot<Expressions>>,
        field1_up: Vec<SyntaxTrailingField<SyntaxExpression<Expressions, Patterns, Types>>>,
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
        question_mark_start: lsp_types::Position,
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
    pub equals_start: lsp_types::Position,
    pub pattern: Option<SyntaxPattern<Patterns, Types>>,
    pub right_angle_start: Option<lsp_types::Position>,
    pub result: Option<SyntaxExpression<Expressions, Patterns, Types>>,
}

pub fn name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, name.value.len() as u32)
}
pub fn name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: name_end(name),
    }
}
pub fn variant_name_length(variant_name: &Name) -> usize {
    1 + variant_name.len()
}
pub fn variant_name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, variant_name_length(name.value) as u32)
}
pub fn variant_name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: variant_name_end(name),
    }
}
pub fn optional_variant_name_length(variant_name: Option<&Name>) -> usize {
    match variant_name {
        None => 1,
        Some(name) => variant_name_length(name),
    }
}
pub fn optional_variant_name_end(
    variant_name: &WithStartPosition<Option<Name>>,
) -> lsp_types::Position {
    position_add_characters(
        variant_name.start,
        optional_variant_name_length(variant_name.value.as_ref()) as u32,
    )
}
pub fn optional_variant_name_range(
    variant_name: &WithStartPosition<Option<Name>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: variant_name.start,
        end: optional_variant_name_end(variant_name),
    }
}
pub fn field_name_length(field_name: &Name) -> usize {
    1 + field_name.len()
}
pub fn field_name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, field_name_length(name.value) as u32)
}
pub fn field_name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: field_name_end(name),
    }
}
pub fn optional_field_name_length(field_name: Option<&Name>) -> usize {
    match field_name {
        None => 1,
        Some(name) => field_name_length(name),
    }
}
pub fn optional_field_name_end(
    field_name: &WithStartPosition<Option<Name>>,
) -> lsp_types::Position {
    position_add_characters(
        field_name.start,
        optional_field_name_length(field_name.value.as_ref()) as u32,
    )
}
pub fn optional_field_name_range(field_name: &WithStartPosition<Option<Name>>) -> lsp_types::Range {
    lsp_types::Range {
        start: field_name.start,
        end: optional_field_name_end(field_name),
    }
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
        SyntaxType::ConstructWithoutArguments(name) => name.start,
        SyntaxType::ConstructWithArguments {
            underscore_start,
            name: _,
            argument0: _,
            argument1_up: _,
        } => *underscore_start,
        SyntaxType::Parenthesized {
            open_paren_start,
            inner: _,
            closed_paren_start: _,
        } => *open_paren_start,
        SyntaxType::RecordEmpty { dot_start } => *dot_start,
        SyntaxType::Record {
            field0_name,
            field0_value: _,
            field1_up: _,
        } => field0_name.start,
        SyntaxType::ChoiceEmpty { bar_start } => *bar_start,
        SyntaxType::Choice {
            variant0_name,
            variant0_value: _,
            variant1_up: _,
        } => variant0_name.start,
    }
}
pub fn type_end<Types>(
    type_: &SyntaxType<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    match type_ {
        SyntaxType::Variable(name) => name_end(with_start_position_as_ref(name)),
        SyntaxType::ConstructWithoutArguments(name) => name_end(with_start_position_as_ref(name)),
        SyntaxType::ConstructWithArguments {
            underscore_start,
            name,
            argument0,
            argument1_up,
        } => argument1_up
            .last()
            .map(|last_argument| {
                last_argument
                    .type_
                    .as_ref()
                    .map(|last_argument| type_end(last_argument, types))
                    .unwrap_or_else(|| symbol_end(last_argument.comma_start, ","))
            })
            .or_else(|| {
                argument0
                    .as_ref()
                    .map(|argument0| type_end(types.element(argument0), types))
            })
            .or_else(|| {
                name.as_ref()
                    .map(|name| name_end(with_start_position_as_ref(name)))
            })
            .unwrap_or_else(|| symbol_end(*underscore_start, "_")),
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
        SyntaxType::RecordEmpty { dot_start } => symbol_end(*dot_start, "."),
        SyntaxType::Record {
            field0_name,
            field0_value,
            field1_up,
        } => field1_up
            .last()
            .map(|last_field| trailing_field_end(last_field, |value| type_end(value, types)))
            .or_else(|| {
                field0_value
                    .as_ref()
                    .map(|field0_value| type_end(types.element(field0_value), types))
            })
            .unwrap_or_else(|| field_name_end(with_start_position_as_ref(field0_name))),
        SyntaxType::ChoiceEmpty { bar_start } => symbol_end(*bar_start, "|"),
        SyntaxType::Choice {
            variant0_name,
            variant0_value,
            variant1_up,
        } => variant1_up
            .last()
            .map(|last_variant| type_trailing_variant_end(last_variant, types))
            .or_else(|| {
                variant0_value
                    .as_ref()
                    .map(|variant0_value| type_end(types.element(variant0_value), types))
            })
            .unwrap_or_else(|| field_name_end(with_start_position_as_ref(variant0_name))),
    }
}
pub fn type_trailing_variant_end<Types>(
    variant: &SyntaxTypeTrailingVariant<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    variant
        .value
        .as_ref()
        .map(|value| type_end(value, types))
        .unwrap_or_else(|| optional_variant_name_end(&variant.name))
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
        SyntaxPattern::Variant { name, value: _ } => name.start,
        SyntaxPattern::RecordEmpty { dot_start } => *dot_start,
        SyntaxPattern::Record {
            field0_name,
            field0_value: _,
            field1_up: _,
        } => field0_name.start,
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
        SyntaxPattern::Variant { name, value } => value
            .as_ref()
            .map(|value| pattern_end(patterns.element(value), patterns, types))
            .unwrap_or_else(|| optional_variant_name_end(name)),
        SyntaxPattern::RecordEmpty { dot_start } => symbol_end(*dot_start, "."),
        SyntaxPattern::Record {
            field0_name,
            field0_value,
            field1_up,
        } => field1_up
            .last()
            .map(|last_field| {
                trailing_field_end(last_field, |value| pattern_end(value, patterns, types))
            })
            .or_else(|| {
                field0_value.as_ref().map(|field0_value| {
                    pattern_end(patterns.element(field0_value), patterns, types)
                })
            })
            .unwrap_or_else(|| field_name_end(with_start_position_as_ref(field0_name))),
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
pub fn trailing_field_range<Value>(
    field: &SyntaxTrailingField<Value>,
    value_end: impl FnOnce(&Value) -> lsp_types::Position,
) -> lsp_types::Range {
    lsp_types::Range {
        start: trailing_field_start(field),
        end: trailing_field_end(field, value_end),
    }
}
pub fn trailing_field_start<Value>(field: &SyntaxTrailingField<Value>) -> lsp_types::Position {
    field.name.start
}
pub fn trailing_field_end<Value>(
    field: &SyntaxTrailingField<Value>,
    value_end: impl FnOnce(&Value) -> lsp_types::Position,
) -> lsp_types::Position {
    field
        .value
        .as_ref()
        .map(value_end)
        .unwrap_or_else(|| optional_field_name_end(&field.name))
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
        SyntaxExpression::Variable(name) => name.start,
        SyntaxExpression::Call {
            underscore_start,
            name: _,
            type_arguments: _,
            argument: _,
        } => *underscore_start,
        SyntaxExpression::Variant {
            name,
            type_: _,
            value: _,
        } => name.start,
        SyntaxExpression::Fn {
            fn_keyword_start,
            parameter: _,
            angle_right_start: _,
            result: _,
        } => *fn_keyword_start,
        SyntaxExpression::RecordEmpty { dot_start } => *dot_start,
        SyntaxExpression::Record {
            field0_name,
            field0_value: _,
            field1_up: _,
        } => field0_name.start,
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
            question_mark_start,
            queried: _,
            cases: _,
        } => *question_mark_start,
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
        SyntaxExpression::Variable(name) => name_end(with_start_position_as_ref(name)),
        SyntaxExpression::Call {
            underscore_start,
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
            .or_else(|| {
                name.as_ref()
                    .map(|name| name_end(with_start_position_as_ref(name)))
            })
            .unwrap_or_else(|| symbol_end(*underscore_start, "_")),
        SyntaxExpression::Variant { name, type_, value } => value
            .as_ref()
            .map(|value| expression_end(expressions.element(value), expressions, patterns, types))
            .or_else(|| type_.as_ref().map(|type_| type_end(type_, types)))
            .unwrap_or_else(|| optional_variant_name_end(name)),
        SyntaxExpression::Fn {
            fn_keyword_start,
            parameter,
            angle_right_start,
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
                angle_right_start.map(|angle_right_start| symbol_end(angle_right_start, ">"))
            })
            .or_else(|| {
                parameter
                    .as_ref()
                    .map(|parameter| pattern_end(parameter, patterns, types))
            })
            .unwrap_or_else(|| symbol_end(*fn_keyword_start, "fn")),
        SyntaxExpression::RecordEmpty { dot_start } => symbol_end(*dot_start, "."),
        SyntaxExpression::Record {
            field0_name,
            field0_value,
            field1_up,
        } => field1_up
            .last()
            .map(|last_field| {
                trailing_field_end(last_field, |value| {
                    expression_end(value, expressions, patterns, types)
                })
            })
            .or_else(|| {
                field0_value.as_ref().map(|field0_value| {
                    expression_end(
                        expressions.element(field0_value),
                        expressions,
                        patterns,
                        types,
                    )
                })
            })
            .unwrap_or_else(|| field_name_end(with_start_position_as_ref(field0_name))),
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
            question_mark_start,
            queried,
            cases,
        } => cases
            .last()
            .map(|last_case| expression_query_case_end(last_case, expressions, patterns, types))
            .or_else(|| {
                queried.as_ref().map(|queried| {
                    expression_end(expressions.element(queried), expressions, patterns, types)
                })
            })
            .unwrap_or_else(|| symbol_end(*question_mark_start, "?")),
    }
}
fn comments_end(comments: &SyntaxComments) -> lsp_types::Position {
    let last_line = comments.line1_up.last().unwrap_or(&comments.line0);
    position_add_characters(
        last_line.start,
        last_line.value.encode_utf16().count() as u32,
    )
}
fn expression_query_case_end<Expressions, Patterns, Types>(
    case: &SyntaxExpressionQueryCase<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    case.result
        .as_ref()
        .map(|result| expression_end(result, expressions, patterns, types))
        .or_else(|| {
            case.right_angle_start
                .map(|left_angle_start| symbol_end(left_angle_start, ">"))
        })
        .or_else(|| {
            case.pattern
                .as_ref()
                .map(|pattern| pattern_end(pattern, patterns, types))
        })
        .unwrap_or_else(|| symbol_end(case.equals_start, "="))
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
fn is_sloe_lowercase_name_start(name_start: char) -> bool {
    name_start.is_ascii_lowercase()
}
fn parse_sloe_lowercase_name(state: &mut ParseState) -> Option<Name> {
    let mut chars_from_offset: std::str::Chars = state.source[state.offset_utf8..].chars();
    if let Some(first_char) = chars_from_offset.next()
        && is_sloe_lowercase_name_start(first_char)
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
        // disambiguate from project-level fn/ty keywords
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

fn parse_variant_name(state: &mut ParseState) -> Option<WithStartPosition<Option<Name>>> {
    let Some(start_position) = parse_symbol_as_start(state, "|") else {
        return None;
    };
    let name = parse_sloe_lowercase_name(state);
    Some(WithStartPosition {
        start: start_position,
        value: name,
    })
}
fn parse_field_name(state: &mut ParseState) -> Option<WithStartPosition<Option<Name>>> {
    let Some(start_position) = parse_symbol_as_start(state, ".") else {
        return None;
    };
    let name = parse_sloe_lowercase_name(state);
    Some(WithStartPosition {
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
    parse_sloe_whitespace(&mut state);
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
        .or_else(|| parse_project_ty(state, types))
        .or_else(|| parse_sloe_comments(state).map(SyntaxProjectElement::Comments))
}
fn parse_project_ty<Expressions, Patterns, Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxProjectElement<Expressions, Patterns, Types>> {
    let Some(ty_keyword_start) = parse_symbol_as_start(state, "ty") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let parameters = parse_ty_parameters(state);
    parse_sloe_whitespace(state);
    let documentation = parse_sloe_comments(state);
    parse_sloe_whitespace(state);
    let type_ = parse_type(state, types);
    Some(SyntaxProjectElement::TypeAlias {
        ty_keyword_start,
        name: name,
        parameters: parameters,
        documentation: documentation,
        type_: type_,
    })
}
fn parse_ty_parameters(state: &mut ParseState) -> Option<TyParameters> {
    let Some(parameter0) = parse_sloe_uppercase_name_with_start(state) else {
        return None;
    };
    let mut parameter1_up = Vec::new();
    while let Some(comma_start) = parse_symbol_as_start(state, ",") {
        parse_sloe_whitespace(state);
        let name = parse_sloe_uppercase_name_with_start(state);
        parameter1_up.push(TyTrailingParameter {
            comma_start: comma_start,
            name: name,
        });
        parse_sloe_whitespace(state);
    }
    Some(TyParameters {
        parameter0: parameter0,
        parameter1_up: parameter1_up,
    })
}
fn parse_angled_type_parameters(state: &mut ParseState) -> Option<SyntaxAngledTypeParameters> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_sloe_whitespace(state);
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
    let parameter = parse_pattern_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let arrow_start = parse_symbol_as_start(state, ":>");
    parse_sloe_whitespace(state);
    let result_type = parse_type(state, types);
    parse_sloe_whitespace(state);
    let angle_right_start = parse_symbol_as_start(state, ">");
    parse_sloe_whitespace(state);
    let documentation = parse_sloe_comments(state);
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxProjectElement::Fn {
        fn_keyword_start: fn_keyword_start,
        name: name,
        type_parameters: type_parameters,
        parameter: parameter,
        arrow_start: arrow_start,
        result_type: result_type,
        angle_right_start: angle_right_start,
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
    let Some(name) = parse_variant_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let value = parse_pattern_typed(state, patterns, types);
    Some(SyntaxPattern::Variant {
        name: name,
        value: value.map(|value| patterns.add(value)),
    })
}
fn parse_pattern_variant_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(name) = parse_variant_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let value = parse_pattern_untyped(state, patterns, types);
    Some(SyntaxPattern::Variant {
        name: name,
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
    let Some(field0_name) = parse_field_name(state) else {
        return None;
    };
    let Some(field0_name_value) = field0_name.value else {
        return Some(SyntaxPattern::RecordEmpty {
            dot_start: field0_name.start,
        });
    };
    parse_sloe_whitespace(state);
    let field0_value = parse_pattern_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let mut field1_up = Vec::new();
    while let Some(field) = parse_pattern_field_typed(state, patterns, types) {
        field1_up.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxPattern::Record {
        field0_name: WithStartPosition {
            start: field0_name.start,
            value: field0_name_value,
        },
        field0_value: field0_value.map(|field0_value| patterns.add(field0_value)),
        field1_up: field1_up,
    })
}
fn parse_pattern_field_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxTrailingField<SyntaxPattern<Patterns, Types>>> {
    let Some(name) = parse_field_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let value = parse_pattern_typed(state, patterns, types);
    Some(SyntaxTrailingField {
        name: name,
        value: value,
    })
}
fn parse_pattern_record_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let Some(field0_name) = parse_field_name(state) else {
        return None;
    };
    let Some(field0_name_value) = field0_name.value else {
        return Some(SyntaxPattern::RecordEmpty {
            dot_start: field0_name.start,
        });
    };
    parse_sloe_whitespace(state);
    let field0_value = parse_pattern_untyped(state, patterns, types);
    parse_sloe_whitespace(state);
    let mut field1_up = Vec::new();
    while let Some(field) = parse_pattern_field_untyped(state, patterns, types) {
        field1_up.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxPattern::Record {
        field0_name: WithStartPosition {
            start: field0_name.start,
            value: field0_name_value,
        },
        field0_value: field0_value.map(|field0_value| patterns.add(field0_value)),
        field1_up: field1_up,
    })
}
fn parse_pattern_field_untyped<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxTrailingField<SyntaxPattern<Patterns, Types>>> {
    let Some(name) = parse_field_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let value = parse_pattern_untyped(state, patterns, types);
    Some(SyntaxTrailingField {
        name: name,
        value: value,
    })
}

pub fn parse_type<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    parse_type_variable(state)
        .or_else(|| parse_type_construct_without_arguments(state))
        .or_else(|| parse_type_construct_with_arguments(state, types))
        .or_else(|| parse_type_parenthesized(state, types))
        .or_else(|| parse_type_record(state, types))
        .or_else(|| parse_type_choice(state, types))
}
pub fn parse_type_not_open_ended<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    parse_type_variable(state)
        .or_else(|| parse_type_construct_without_arguments(state))
        .or_else(|| parse_type_record_empty(state))
        .or_else(|| parse_type_choice_empty(state))
        .or_else(|| parse_type_parenthesized(state, types))
}
fn parse_type_variable<Types>(state: &mut ParseState) -> Option<SyntaxType<Types>> {
    parse_sloe_uppercase_name_with_start(state).map(|name| SyntaxType::Variable(name))
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
fn parse_type_record_empty<Types>(state: &mut ParseState) -> Option<SyntaxType<Types>> {
    parse_sloe_keyword_as_start(state, ".").map(|dot_start| SyntaxType::RecordEmpty {
        dot_start: dot_start,
    })
}
fn parse_type_record<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    let Some(field0_name) = parse_field_name(state) else {
        return None;
    };
    let Some(field0_name_value) = field0_name.value else {
        return Some(SyntaxType::RecordEmpty {
            dot_start: field0_name.start,
        });
    };
    parse_sloe_whitespace(state);
    let field0_value = parse_type(state, types);
    parse_sloe_whitespace(state);
    let mut field1_up = Vec::new();
    while let Some(field) = parse_type_field(state, types) {
        field1_up.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxType::Record {
        field0_name: WithStartPosition {
            start: field0_name.start,
            value: field0_name_value,
        },
        field0_value: field0_value.map(|field0_value| types.add(field0_value)),
        field1_up: field1_up,
    })
}
fn parse_type_field<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxTrailingField<SyntaxType<Types>>> {
    let Some(name) = parse_field_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let value = parse_type(state, types);
    Some(SyntaxTrailingField {
        name: name,
        value: value,
    })
}
fn parse_type_choice_empty<Types>(state: &mut ParseState) -> Option<SyntaxType<Types>> {
    parse_sloe_keyword_as_start(state, "|").map(|bar_start| SyntaxType::ChoiceEmpty {
        bar_start: bar_start,
    })
}
fn parse_type_choice<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    let Some(variant0_name) = parse_variant_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let Some(variant0_name_value) = variant0_name.value else {
        return Some(SyntaxType::ChoiceEmpty {
            bar_start: variant0_name.start,
        });
    };
    let variant0_value = parse_type(state, types);
    parse_sloe_whitespace(state);
    let mut variant1_up = Vec::new();
    while let Some(variant) = parse_type_variant(state, types) {
        variant1_up.push(variant);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxType::Choice {
        variant0_name: WithStartPosition {
            value: variant0_name_value,
            start: variant0_name.start,
        },
        variant0_value: variant0_value.map(|value| types.add(value)),
        variant1_up: variant1_up,
    })
}
fn parse_type_variant<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxTypeTrailingVariant<Types>> {
    let Some(name) = parse_variant_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let value = parse_type(state, types);
    Some(SyntaxTypeTrailingVariant {
        name: name,
        value: value,
    })
}
fn parse_type_construct_with_arguments<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxType<Types>> {
    let Some(underscore_start) = parse_symbol_as_start(state, "_") else {
        return None;
    };
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let argument0 = parse_type(state, types);
    parse_sloe_whitespace(state);
    let mut argument1_up = Vec::new();
    while let Some(comma_start) = parse_symbol_as_start(state, ",") {
        parse_sloe_whitespace(state);
        let argument_type = parse_type(state, types);
        parse_sloe_whitespace(state);
        argument1_up.push(SyntaxTypeConstructTrailingArgument {
            comma_start: comma_start,
            type_: argument_type,
        });
    }
    Some(SyntaxType::ConstructWithArguments {
        underscore_start: underscore_start,
        name: name,
        argument0: argument0.map(|argument0| types.add(argument0)),
        argument1_up: argument1_up,
    })
}
fn parse_type_construct_without_arguments<Types>(
    state: &mut ParseState,
) -> Option<SyntaxType<Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxType::ConstructWithoutArguments(name))
}
pub fn parse_expression<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    // fn and origin must be checked before variable or call
    parse_expression_number(state, types)
        .or_else(|| parse_expression_char(state))
        .or_else(|| parse_expression_str(state))
        .or_else(|| parse_expression_fn(state, expressions, patterns, types))
        .or_else(|| parse_expression_origin(state, expressions, patterns, types))
        .or_else(|| parse_expression_variable(state))
        .or_else(|| parse_expression_call(state, expressions, patterns, types))
        .or_else(|| parse_expression_variant(state, expressions, patterns, types))
        .or_else(|| parse_expression_parenthesized(state, expressions, patterns, types))
        .or_else(|| parse_expression_commented(state, expressions, patterns, types))
        .or_else(|| parse_expression_record(state, expressions, patterns, types))
        .or_else(|| parse_expression_query(state, expressions, patterns, types))
}
fn parse_expression_record<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(field0_name) = parse_field_name(state) else {
        return None;
    };
    let Some(field0_name_value) = field0_name.value else {
        return Some(SyntaxExpression::RecordEmpty {
            dot_start: field0_name.start,
        });
    };
    parse_sloe_whitespace(state);
    let field0_value = parse_expression(state, expressions, patterns, types);
    parse_sloe_whitespace(state);
    let mut field1_up = Vec::new();
    while let Some(field) = parse_expression_trailing_field(state, expressions, patterns, types) {
        field1_up.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxExpression::Record {
        field0_name: WithStartPosition {
            start: field0_name.start,
            value: field0_name_value,
        },
        field0_value: field0_value.map(|field0_value| expressions.add(field0_value)),
        field1_up: field1_up,
    })
}
fn parse_expression_trailing_field<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxTrailingField<SyntaxExpression<Expressions, Patterns, Types>>> {
    let Some(name) = parse_field_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let value = parse_expression(state, expressions, patterns, types);
    Some(SyntaxTrailingField {
        name: name,
        value: value,
    })
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
fn parse_expression_commented<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(comments) = parse_sloe_comments(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let expression = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Commented {
        comments: comments,
        expression: expression.map(|expression| expressions.add(expression)),
    })
}
fn parse_expression_variable<Expressions, Patterns, Types>(
    state: &mut ParseState,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(name) = parse_sloe_lowercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxExpression::Variable(name))
}
fn parse_expression_call<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(underscore_start) = parse_symbol_as_start(state, "_") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let name = parse_sloe_lowercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let type_arguments = parse_type_arguments(state, types);
    parse_sloe_whitespace(state);
    let argument = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Call {
        underscore_start: underscore_start,
        name: name,
        type_arguments: type_arguments,
        argument: argument.map(|argument| expressions.add(argument)),
    })
}
fn parse_expression_variant<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(name) = parse_variant_name(state) else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_ = parse_type_not_open_ended(state, types);
    parse_sloe_whitespace(state);
    let value = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Variant {
        name: name,
        type_: type_,
        value: value.map(|argument| expressions.add(argument)),
    })
}
fn parse_expression_fn<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    if state.position.character == 0 {
        return None;
    }
    let Some(fn_keyword_start) = parse_sloe_keyword_as_start(state, "fn") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let parameter = parse_pattern_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let angle_right_start = parse_symbol_as_start(state, ">");
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Fn {
        fn_keyword_start: fn_keyword_start,
        parameter: parameter,
        angle_right_start: angle_right_start,
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
fn parse_expression_query<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(question_mark_start) = parse_symbol_as_start(state, "?") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let queried = parse_expression(state, expressions, patterns, types);
    parse_sloe_whitespace(state);
    let mut cases = Vec::new();
    while let Some(case) = parse_expression_query_case(state, expressions, patterns, types) {
        cases.push(case);
        parse_sloe_whitespace(state);
    }
    parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Query {
        question_mark_start: question_mark_start,
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
    let Some(equals_start) = parse_symbol_as_start(state, "=") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let pattern = parse_pattern_untyped(state, patterns, types);
    parse_sloe_whitespace(state);
    let angle_right_start = parse_symbol_as_start(state, ">");
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpressionQueryCase {
        equals_start: equals_start,
        pattern: pattern,
        right_angle_start: angle_right_start,
        result: result,
    })
}

pub struct CompiledProject {
    pub rust: syn::File,
    pub type_aliases: std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    pub fns: std::collections::HashMap<Name, CompiledProjectFnInfo>,
    pub records: std::collections::HashSet<Vec<Name>>,
}
#[derive(Clone, Debug)]
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
    Origin(Name),
    Record(Vec<TypeField>),
    Choice(Vec<TypeVariant>),
    CoreConstruct { name: Name, arguments: Vec<Type> },
}
#[derive(Clone, Debug)]
pub struct TypeField {
    pub name: Name,
    pub value: Type,
}
#[derive(Clone, Debug)]
pub struct TypeVariant {
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

fn type_is_copy(variables_are_copy: bool, type_: &Type) -> bool {
    match type_ {
        Type::Variable(_) => variables_are_copy,
        Type::Origin(_) => false,
        Type::CoreConstruct { name, arguments } => {
            core_type_aliases
                .get(name)
                .is_some_and(|core_type_info| core_type_info.is_copy)
                && arguments
                    .iter()
                    .all(|argument| type_is_copy(variables_are_copy, argument))
        }
        Type::Record(fields) => fields
            .iter()
            .all(|field| type_is_copy(variables_are_copy, &field.value)),
        Type::Choice(variants) => variants
            .iter()
            .all(|variant| type_is_copy(variables_are_copy, &variant.value)),
    }
}

pub fn syntax_project_to_rust<Expressions, Patterns, Types>(
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
  - ty some-type-name (some type)",
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
            SyntaxProjectElement::TypeAlias {
                ty_keyword_start,
                name: maybe_name,
                parameters,
                documentation,
                type_,
            } => match maybe_name {
                None => {
                    errors.push(ErrorNode { range: symbol_range(*ty_keyword_start, "ty"), message: Box::from("missing name. Type names start with a lowercase letter and only use ascii letters, digits and -") });
                }
                Some(name_node) => {
                    let type_alias_declaration_graph_node: strongly_connected_components::Node =
                        type_graph.new_node();
                    let existing_type_with_same_name: Option<strongly_connected_components::Node> =
                        type_graph_node_by_name
                            .insert(&name_node.value, type_alias_declaration_graph_node);
                    type_declaration_by_graph_node.insert(
                        type_alias_declaration_graph_node,
                        SyntaxProjectTypeInfo {
                            documentation: &documentation,
                            name: &name_node,
                            parameters: &parameters,
                            type_: &type_,
                        },
                    );
                    if existing_type_with_same_name.is_some() {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(name_node)),
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
                arrow_start: _,
                result_type,
                angle_right_start: _,
                documentation,
                result: maybe_result,
            } => match maybe_name {
                None => {
                    errors.push(ErrorNode { range: symbol_range(*fn_keyword_start, "fn"), message: Box::from("missing name. Function names start with a lowercase letter and only use ascii letters, digits and -") });
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
                            range: name_range(with_start_position_as_ref(name)),
                            message: Box::from(
                                "a variable with this name is already declared. Rename one of them",
                            ),
                        });
                    } else if core_fns.contains_key(name.value.as_str()) {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(name)),
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
    project_type_info: SyntaxProjectTypeInfo<Types>,
) {
    if let Some(aliased_type) = &project_type_info.type_ {
        syntax_type_connect_type_names_in_graph_from(
            type_graph,
            origin_project_type_graph_node,
            type_graph_node_by_name,
            types,
            aliased_type,
        );
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
        SyntaxType::ConstructWithoutArguments(name) => {
            if let Some(referenced_type_graph_node) =
                type_graph_node_by_name.get(name.value.as_str()).copied()
            {
                type_graph.new_edge(
                    origin_type_declaration_graph_node,
                    referenced_type_graph_node,
                );
            }
        }
        SyntaxType::ConstructWithArguments {
            underscore_start: _,
            name,
            argument0,
            argument1_up,
        } => {
            if let Some(name) = name
                && let Some(referenced_type_graph_node) =
                    type_graph_node_by_name.get(name.value.as_str()).copied()
            {
                type_graph.new_edge(
                    origin_type_declaration_graph_node,
                    referenced_type_graph_node,
                );
            }
            for argument in argument0
                .iter()
                .map(|argument0| types.element(argument0))
                .chain(
                    argument1_up
                        .iter()
                        .filter_map(|argument| argument.type_.as_ref()),
                )
            {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph,
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    argument,
                );
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
        SyntaxType::RecordEmpty { dot_start: _ } => {}
        SyntaxType::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            if let Some(field0_value) = field0_value {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph,
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    types.element(field0_value),
                );
            }
            for field in field1_up {
                if let Some(value) = &field.value {
                    syntax_type_connect_type_names_in_graph_from(
                        type_graph,
                        origin_type_declaration_graph_node,
                        type_graph_node_by_name,
                        types,
                        value,
                    );
                }
            }
        }
        SyntaxType::ChoiceEmpty { bar_start: _ } => {}
        SyntaxType::Choice {
            variant0_name: _,
            variant0_value,
            variant1_up,
        } => {
            if let Some(variant0_value) = variant0_value {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph,
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    types.element(variant0_value),
                )
            }
            for variant in variant1_up {
                if let Some(value) = &variant.value {
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
    }
}
// TODO(important) track pattern variables and origins to avoid accidental misconnection
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
        SyntaxExpression::Variable(_) => {}
        SyntaxExpression::Call {
            underscore_start: _,
            name,
            type_arguments: _,
            argument,
        } => {
            if let Some(name) = name
                && let Some(referenced_fn_graph_node) = project_fn_graph_node_by_name
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
            type_: _,
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
            angle_right_start: _,
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
        SyntaxExpression::RecordEmpty { dot_start: _ } => {}
        SyntaxExpression::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            if let Some(field0_value) = field0_value {
                syntax_expression_connect_variables_in_graph_from(
                    project_fn_graph,
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.element(field0_value),
                );
            }
            for field in field1_up {
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
            question_mark_start: _,
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
struct SyntaxProjectTypeInfo<'a, Types> {
    // consider introducing separate structs instead of separately referencing each field
    name: &'a WithStartPosition<Name>,
    documentation: &'a Option<SyntaxComments>,
    parameters: &'a Option<TyParameters>,
    type_: &'a Option<SyntaxType<Types>>,
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
        Self {
            documentation: self.documentation,
            name: self.name,
            parameters: self.parameters,
            type_: self.type_,
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
        core_type_aliases.clone();
    compiled_type_aliases.reserve(project_type_by_graph_node.len());
    let mut records_used: std::collections::HashSet<Vec<Name>> =
        std::collections::HashSet::with_capacity(16);
    let mut choices_used: std::collections::HashSet<Vec<Name>> =
        std::collections::HashSet::with_capacity(4);
    for project_type_strongly_connected_component in type_graph.find_sccs().iter_sccs() {
        // TODO report and skip (mutually) recursive project types. Currently these are reported as "not found" at best
        for project_type in project_type_strongly_connected_component
            .iter_nodes()
            .filter_map(|variable_declaration_graph_node| {
                project_type_by_graph_node.get(&variable_declaration_graph_node)
            })
            .copied()
        {
            let maybe_compiled_type_alias: Option<CompiledTypeAlias> =
                type_alias_declaration_to_rust(
                    errors,
                    &mut records_used,
                    &mut choices_used,
                    &compiled_type_aliases,
                    types,
                    project_type.documentation.as_ref(),
                    &project_type.name,
                    project_type.parameters.as_ref(),
                    project_type.type_.as_ref(),
                );
            let documentation = project_type.documentation.as_ref().map(|documentation| {
                documentation
                    .line1_up
                    .iter()
                    .fold(documentation.line0.value.to_string(), |so_far, line| {
                        so_far + "\n" + &line.value
                    })
                    .into_boxed_str()
            });
            let parameters = project_type
                .parameters
                .iter()
                .flat_map(|parameters| {
                    std::iter::once(parameters.parameter0.value.clone()).chain(
                        parameters
                            .parameter1_up
                            .iter()
                            .filter_map(|parameter| parameter.name.as_ref())
                            .map(|parameter_name| parameter_name.value.clone()),
                    )
                })
                .collect();
            match maybe_compiled_type_alias {
                Some(compiled_type_alias) => {
                    rust_items.push(compiled_type_alias.rust);
                    compiled_type_aliases.insert(
                        project_type.name.value.clone(),
                        CompiledTypeAliasInfo {
                            name_range: Some(name_range(with_start_position_as_ref(
                                &project_type.name,
                            ))),
                            documentation: documentation,
                            parameters: parameters,
                            type_: Some(compiled_type_alias.type_),
                            is_copy: compiled_type_alias.is_copy,
                        },
                    );
                }
                None => {
                    compiled_type_aliases.insert(
                        project_type.name.value.clone(),
                        CompiledTypeAliasInfo {
                            name_range: Some(name_range(with_start_position_as_ref(
                                &project_type.name,
                            ))),
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
    }
    let mut compiled_project_fns: std::collections::HashMap<Name, CompiledProjectFnInfo> =
        core_fns.clone();
    compiled_project_fns.reserve(project_fn_graph.len());
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
                    &mut std::collections::HashSet::new(),
                    &mut std::collections::HashMap::new(),
                    &compiled_type_aliases,
                    patterns,
                    types,
                    &std::collections::HashMap::new(),
                )
                .map(|compiled_parameter| compiled_parameter.type_)
            });
            match project_fn.result_type {
                None => {
                    compiled_project_fns.insert(
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
                        types,
                        &std::collections::HashMap::new(),
                        &mut std::collections::HashSet::new(),
                        &mut std::collections::HashSet::new(),
                    );
                    compiled_project_fns.insert(
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
            let maybe_compiled_project_fn: Option<CompiledProjectFn> = syntax_project_fn_to_rust(
                errors,
                &mut records_used,
                &mut choices_used,
                &compiled_type_aliases,
                &compiled_project_fns,
                expressions,
                patterns,
                types,
                project_fn,
            );
            if let Some(compiled_project_fn) = maybe_compiled_project_fn {
                rust_items.push(compiled_project_fn.rust);
                compiled_project_fns.insert(
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
    rust_items.extend(
        choices_used
            .iter()
            .filter(|choice| !core_choices.contains(choice.as_slice()))
            .map(|used_choice_variants| syntax_choice_to_rust(used_choice_variants)),
    );
    CompiledProject {
        rust: syn::File {
            shebang: None,
            attrs: vec![],
            items: rust_items,
        },
        type_aliases: compiled_type_aliases,
        fns: compiled_project_fns,
        records: records_used,
        // fn_graph: project_fn_graph,
        // fn_by_graph_node: project_fn_by_graph_node,
    }
}
fn syntax_choice_to_rust(used_choice_variants: &[Name]) -> syn::Item {
    let rust_enum_name: String = variant_names_to_rust_enum_name(used_choice_variants.iter());
    let rust_struct: syn::Item = syn::Item::Enum(syn::ItemEnum {
        attrs: vec![syn_attribute_derive(["Copy", "Clone", "Debug"].into_iter())],
        vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
        enum_token: syn::token::Enum(syn_span()),
        ident: syn_ident(&rust_enum_name),
        generics: syn::Generics {
            lt_token: Some(syn::token::Lt(syn_span())),
            params: used_choice_variants
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
        brace_token: syn::token::Brace(syn_span()),
        variants: used_choice_variants
            .iter()
            .map(|variant_name| syn::Variant {
                attrs: vec![],
                ident: syn_ident(&name_to_uppercase_rust(variant_name)),
                fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                    paren_token: syn::token::Paren(syn_span()),
                    unnamed: std::iter::once(syn::Field {
                        attrs: vec![],
                        mutability: syn::FieldMutability::None,
                        vis: syn::Visibility::Inherited,
                        ident: None,
                        colon_token: None,
                        ty: syn::Type::Path(syn::TypePath {
                            qself: None,
                            path: syn_path_reference([&type_variable_to_rust(variant_name)]),
                        }),
                    })
                    .collect(),
                }),
                discriminant: None,
            })
            .collect(),
    });
    rust_struct
}
fn syntax_record_to_rust(used_choice_variants: &[Name]) -> syn::Item {
    let rust_struct_name: String =
        field_names_to_rust_record_struct_name(used_choice_variants.iter());
    let rust_struct: syn::Item = syn::Item::Struct(syn::ItemStruct {
        attrs: vec![syn_attribute_derive(["Copy", "Clone", "Debug"].into_iter())],
        vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
        struct_token: syn::token::Struct(syn_span()),
        ident: syn_ident(&rust_struct_name),
        generics: syn::Generics {
            lt_token: Some(syn::token::Lt(syn_span())),
            params: used_choice_variants
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
            named: used_choice_variants
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
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    maybe_documentation: Option<&SyntaxComments>,
    name: &WithStartPosition<Name>,
    parameters: Option<&TyParameters>,
    maybe_type: Option<&SyntaxType<Types>>,
) -> Option<CompiledTypeAlias> {
    let rust_name: String = name_to_uppercase_rust(&name.value);
    let Some(aliased_syntax_type) = maybe_type else {
        errors.push(ErrorNode {
            range: name_range(with_start_position_as_ref(name)),
            message: Box::from("missing type after the project ty name ty ..type-name.. here"),
        });
        return None;
    };
    let Some(aliased_type) = syntax_type_to_type(
        aliased_syntax_type,
        errors,
        type_aliases,
        types,
        &std::collections::HashMap::new(),
        records_used,
        choices_used,
    ) else {
        return None;
    };
    let type_rust: syn::Type = type_to_rust(&aliased_type);
    let mut actually_used_type_variables: std::collections::HashSet<Name> =
        std::collections::HashSet::with_capacity(
            parameters
                .map(|parameters| 1 + parameters.parameter1_up.len())
                .unwrap_or(0),
        );
    type_variables_into(&mut actually_used_type_variables, &aliased_type);
    let mut rust_parameters: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> =
        syn::punctuated::Punctuated::new();
    if let Err(()) = parameters_to_rust_into_error_if_different_to_actual_type_parameters(
        errors,
        &mut rust_parameters,
        name_range(with_start_position_as_ref(name)),
        parameters.iter().flat_map(|parameters| {
            std::iter::once(&parameters.parameter0).chain(
                parameters
                    .parameter1_up
                    .iter()
                    .filter_map(|parameter| parameter.name.as_ref()),
            )
        }),
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
        is_copy: type_is_copy(true, &aliased_type),
        type_: aliased_type,
    })
}

struct CompiledProjectFn {
    rust: syn::Item,
    parameter_type: Type,
    result_type: Type,
}
fn syntax_project_fn_to_rust<'a, Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
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
                "missing expression after the fn result type. An example would be fn my-function & str \":)\", where & is an empty record as the parameter",
            ),
        });
        return None;
    };
    let Some(syntax_parameter) = &project_fn_info.parameter else {
        errors.push(ErrorNode {
            range: project_fn_info.range,
            message: Box::from(
                "missing parameter pattern after the fn name. An example would be fn my-function & str \":)\", where & is an empty record as the parameter",
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
        choices_used,
        &mut parameter_introduced_bindings,
        type_aliases,
        patterns,
        types,
        &std::collections::HashMap::new(),
    ) else {
        return None;
    };
    let mut used_origin_variables = std::collections::HashMap::new();
    let mut used_pattern_variables = std::collections::HashMap::new();
    let compiled_result: CompiledExpression = syntax_expression_to_rust(
        errors,
        records_used,
        choices_used,
        type_aliases,
        project_fns,
        expressions,
        patterns,
        types,
        &mut parameter_introduced_bindings,
        &mut used_pattern_variables,
        &mut std::collections::HashMap::new(),
        &mut used_origin_variables,
        result_node,
    );
    for (parameter_introduced_binding_name, parameter_introduced_binding_origin) in
        parameter_introduced_bindings
    {
        push_error_if_introduced_pattern_variable_is_unused(
            errors,
            parameter_introduced_binding_origin.origin_start,
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
                range: name_range(with_start_position_as_ref(project_fn_info.name)),
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
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashMap<&Name, OriginCompileInfo>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
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
                types,
                origins,
                records_used,
                choices_used,
            ),
        },
        SyntaxType::ConstructWithoutArguments(name) => {
            if origins.contains_key(&name.value) {
                Some(Type::Origin(name.value.clone()))
            } else if let Some(origin_type_alias) = type_aliases.get(&name.value) {
                if !origin_type_alias.parameters.is_empty() {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!(
                            "this type alias has {} parameters but there aren't any arguments provided after this name. The expected parameters are called {}",
                            origin_type_alias.parameters.len(),
                            origin_type_alias.parameters.iter().map(|parameter| parameter.as_str()).collect::<Vec<_>>().join(", ")
                        ).into_boxed_str()
                    });
                    return None;
                }
                origin_type_alias.type_.clone()
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("no type alias or origin exists with this name"),
                });
                None
            }
        }
        SyntaxType::ConstructWithArguments {
            underscore_start,
            name,
            argument0,
            argument1_up,
        } => {
            let Some(name) = name else {
                errors.push(ErrorNode {
                    range: symbol_range(*underscore_start, "_"),
                    message : Box::from("missing type name after this underscore _ . An example of a valid type construct is _vec Origin u32")
                });
                return None;
            };
            if origins.contains_key(&name.value) {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message : Box::from("this type refers to an origin but has type arguments. As origin types don't have type parameters, the arguments need to be removed")
                });
                Some(Type::Origin(name.value.clone()))
            } else if let Some(origin_type_alias) = type_aliases.get(&name.value) {
                let argument_types = argument0
                    .iter()
                    .map(|argument0| types.element(argument0))
                    .chain(
                        argument1_up
                            .iter()
                            .filter_map(|argument| argument.type_.as_ref()),
                    )
                    .map(|argument_type| {
                        syntax_type_to_type(
                            argument_type,
                            errors,
                            type_aliases,
                            types,
                            origins,
                            records_used,
                            choices_used,
                        )
                    })
                    .collect::<Option<Vec<Type>>>()?;
                let argument_count = 1 + argument1_up.len();
                match origin_type_alias.parameters.len().cmp(&argument_count) {
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Less => {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(name)),
                            message: format!(
                                "this type alias has {} less parameters than arguments are provided here.",
                                argument_count - origin_type_alias.parameters.len(),
                            ).into_boxed_str()
                        });
                        return None;
                    }
                    std::cmp::Ordering::Greater => {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(name)),
                            message: format!(
                                "this type alias has {} more parameters than arguments are provided here. The additional parameters are called {}",
                                origin_type_alias.parameters.len() - argument_count,
                                origin_type_alias.parameters.iter().map(|parameter| parameter.as_str()).skip(argument_count).collect::<Vec<_>>().join(", ")
                            ).into_boxed_str()
                        });
                        return None;
                    }
                }
                type_construct_resolve_type_alias(origin_type_alias, &argument_types)
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("no type alias or origin exists with this name"),
                });
                None
            }
        }
        SyntaxType::RecordEmpty { dot_start: _ } => Some(Type::Record(vec![])),
        SyntaxType::Record {
            field0_name,
            field0_value,
            field1_up,
        } => {
            records_used.insert(sorted_field_names(
                std::iter::once(&field0_name.value).chain(
                    field1_up
                        .iter()
                        .filter_map(|field| field.name.value.as_ref()),
                ),
            ));
            let Some(field0_value) = field0_value else {
                errors.push(ErrorNode {
                    range: field_name_range(with_start_position_as_ref(field0_name)),
                    message: Box::from(
                        "missing field value after this first field name .field-name here",
                    ),
                });
                return None;
            };
            let mut field_types: Vec<TypeField> = Vec::with_capacity(1 + field1_up.len());
            let mut any_field_value_has_error: bool = false;
            match syntax_type_to_type(
                types.element(field0_value),
                errors,
                type_aliases,
                types,
                origins,
                records_used,
                choices_used,
            ) {
                None => {
                    any_field_value_has_error = true;
                }
                Some(field0_value_type) => {
                    field_types.push(TypeField {
                        name: field0_name.value.clone(),
                        value: field0_value_type,
                    });
                }
            }
            for field in field1_up {
                let Some(field_name) = &field.name.value else {
                    errors.push(ErrorNode {
                        range: symbol_range(field.name.start, "."),
                        message: Box::from("missing field name after this dot ."),
                    });
                    return None;
                };
                if field_types
                    .iter()
                    .any(|type_field| type_field.name == field_name)
                {
                    errors.push(ErrorNode {
                        range: optional_field_name_range(&field.name),
                        message: Box::from(
                            "a field with this name already exists in the record type",
                        ),
                    });
                    return None;
                }
                let Some(field_value) = &field.value else {
                    errors.push(ErrorNode {
                        range: optional_field_name_range(&field.name),
                        message: Box::from(
                            "missing field value after this field name .field-name here",
                        ),
                    });
                    return None;
                };
                match syntax_type_to_type(
                    field_value,
                    errors,
                    type_aliases,
                    types,
                    origins,
                    records_used,
                    choices_used,
                ) {
                    None => {
                        any_field_value_has_error = true;
                    }
                    Some(field_value_type) => {
                        field_types.push(TypeField {
                            name: field_name.clone(),
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
        SyntaxType::ChoiceEmpty { bar_start: _ } => Some(Type::Choice(vec![])),
        SyntaxType::Choice {
            variant0_name,
            variant0_value,
            variant1_up,
        } => {
            choices_used.insert(sorted_variant_names(
                std::iter::once(&variant0_name.value).chain(
                    variant1_up
                        .iter()
                        .filter_map(|variant| variant.name.value.as_ref()),
                ),
            ));
            let Some(variant0_value) = variant0_value else {
                errors.push(ErrorNode {
                    range: variant_name_range(with_start_position_as_ref(variant0_name)),
                    message: Box::from(
                        "missing variant value after this first variant name |..variant-name.. here. Every variant has a value, even if just .",
                    ),
                });
                return None;
            };
            let mut variant_types: Vec<TypeVariant> = Vec::with_capacity(1 + variant1_up.len());
            let mut any_variant_value_has_error: bool = false;
            match syntax_type_to_type(
                types.element(variant0_value),
                errors,
                type_aliases,
                types,
                origins,
                records_used,
                choices_used,
            ) {
                None => {
                    any_variant_value_has_error = true;
                }
                Some(variant_value_type) => {
                    variant_types.push(TypeVariant {
                        name: variant0_name.value.clone(),
                        value: variant_value_type,
                    });
                }
            }
            for syntax_variant in variant1_up {
                let Some(variant_name) = &syntax_variant.name.value else {
                    errors.push(ErrorNode {
                        range: symbol_range(syntax_variant.name.start, "|"),
                        message: Box::from("missing variant name after this bar |"),
                    });
                    return None;
                };
                if variant_types
                    .iter()
                    .any(|type_variant| type_variant.name == variant_name)
                {
                    errors.push(ErrorNode {
                        range: optional_variant_name_range(&syntax_variant.name),
                        message: Box::from(
                            "a variant with this name already exists in the choice type",
                        ),
                    });
                    return None;
                }
                let Some(syntax_variant_value) = &syntax_variant.value else {
                    errors.push(ErrorNode {
                        range: optional_variant_name_range(&syntax_variant.name),
                        message: Box::from(
                            "missing variant value after this name ..Variant-name.. here. Every variant has a value, even if just .",
                        ),
                    });
                    return None;
                };
                match syntax_type_to_type(
                    syntax_variant_value,
                    errors,
                    type_aliases,
                    types,
                    origins,
                    records_used,
                    choices_used,
                ) {
                    None => {
                        any_variant_value_has_error = true;
                    }
                    Some(variant_value_type) => {
                        variant_types.push(TypeVariant {
                            name: variant_name.clone(),
                            value: variant_value_type,
                        });
                    }
                }
            }
            if any_variant_value_has_error {
                return None;
            }
            Some(Type::Choice(variant_types))
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
        Type::Origin(_) => {}
        Type::CoreConstruct { name: _, arguments } => {
            for argument_type in arguments {
                type_replace_variables(type_parameter_replacements, argument_type);
            }
        }
        Type::Record(fields) => {
            for field in fields {
                type_replace_variables(type_parameter_replacements, &mut field.value);
            }
        }
        Type::Choice(variants) => {
            for variant in variants {
                type_replace_variables(type_parameter_replacements, &mut variant.value);
            }
        }
    }
}

fn type_to_rust(type_: &Type) -> syn::Type {
    match type_ {
        Type::Variable(variable) => syn_type_variable(&type_variable_to_rust(variable)),
        Type::Origin(name) => syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn_path_reference([&name_to_uppercase_rust(name)]),
        }),
        Type::CoreConstruct { name, arguments } => syn::Type::Path(syn::TypePath {
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
        Type::Choice(variants) => {
            let mut variants_sorted: Vec<&TypeVariant> = variants.iter().collect();
            variants_sorted.sort_unstable_by_key(|a| &a.name);
            syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: std::iter::once(syn::PathSegment {
                        ident: syn_ident(&variant_names_to_rust_enum_name(
                            variants_sorted.iter().map(|variant| &variant.name),
                        )),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: syn::token::Lt(syn_span()),
                                gt_token: syn::token::Gt(syn_span()),
                                args: variants_sorted
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
        Type::Origin(_) => {}
        Type::CoreConstruct { name: _, arguments } => {
            for argument in arguments {
                type_variables_into(type_variables, argument);
            }
        }
        Type::Record(fields) => {
            for field in fields {
                type_variables_into(type_variables, &field.value);
            }
        }
        Type::Choice(variants) => {
            for variant in variants {
                type_variables_into(type_variables, &variant.value);
            }
        }
    }
}
fn parameters_to_rust_into_error_if_different_to_actual_type_parameters<'a>(
    errors: &mut Vec<ErrorNode>,
    rust_parameters: &mut syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    parameter_name_range: lsp_types::Range,
    parameters: impl Iterator<Item = &'a WithStartPosition<Name>>,
    mut actually_used_type_variables: std::collections::HashSet<Name>,
) -> Result<(), ()> {
    let mut bad_parameters: bool = false;
    for parameter in parameters {
        if !actually_used_type_variables.remove(parameter.value.as_str()) {
            bad_parameters = true;
            errors.push(ErrorNode {
                range: name_range(with_start_position_as_ref(parameter)),
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
            range: parameter_name_range,
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
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, PatternVariableCompileInfo>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
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
                            range: name_range(with_start_position_as_ref(name)),
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
                        types,
                        origins,
                        records_used,
                        choices_used,
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
                        range: name_range(with_start_position_as_ref(name)),
                        message: Box::from(
                            "a pattern variable with this name already exists. Rename it",
                        ),
                    });
                    return None;
                } else if origins.contains_key(&name.value) {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: Box::from("an origin with this name already exists. Rename it"),
                    });
                    return None;
                }
            }
            maybe_compiled_variable
        }
        SyntaxPattern::Variant { name, value } => {
            let Some(name_value) = &name.value else {
                errors.push(ErrorNode {
                    range: symbol_range(name.start, "|"),
                    message: Box::from("missing variant name after this bar |. An example of a variant pattern is |present variable")
                });
                return None;
            };
            match expected_type {
                None => {
                    choices_used.insert(vec![name_value.clone()]);
                    let Some(value) = value else {
                        errors.push(ErrorNode {
                            range: optional_variant_name_range(name),
                            message: Box::from("missing variant value after this variant name. Each variants has a value, even if just ., an example of a variant pattern is |present variable")
                        });
                        return None;
                    };
                    let Some(compiled_value) = syntax_pattern_to_rust(
                        patterns.element(value),
                        None,
                        errors,
                        records_used,
                        choices_used,
                        introduced_variables,
                        type_aliases,
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
                            path: syn_path_reference([
                                &name_to_uppercase_rust(&variant_names_to_rust_enum_name(
                                    std::iter::once(name_value),
                                )),
                                &name_to_uppercase_rust(&name_value),
                            ]),
                            paren_token: syn::token::Paren(syn_span()),
                            elems: std::iter::once(compiled_value.rust).collect(),
                        }),
                        type_: Type::Choice(vec![TypeVariant {
                            name: name_value.clone(),
                            value: compiled_value.type_,
                        }]),
                        catch: compiled_value.catch,
                    })
                }
                Some(expected_type) => {
                    let Type::Choice(origin_choice_type_variants) = &expected_type else {
                        let mut error_message: String = String::from(
                            "A variant is of type that is a choice (for example | A u32 B str) but the expected type here is\n",
                        );
                        type_format(&mut error_message, 0, &expected_type);
                        errors.push(ErrorNode {
                            range: optional_variant_name_range(name),
                            message: error_message.into_boxed_str(),
                        });
                        return None;
                    };
                    let Some(expected_value_type) =
                        origin_choice_type_variants.iter().find_map(|variant| {
                            if variant.name == name_value {
                                Some(&variant.value)
                            } else {
                                None
                            }
                        })
                    else {
                        let mut error_message: String = format!(
                            "this variant name {} is not included in it's expected type\n",
                            name_value
                        );
                        type_format(&mut error_message, 0, expected_type);
                        errors.push(ErrorNode {
                            range: optional_variant_name_range(name),
                            message: error_message.into_boxed_str(),
                        });
                        return None;
                    };
                    let Some(value) = value else {
                        let mut error_message: String =
                            String::from("this variant is missing its associated value of type\n");
                        type_format(&mut error_message, 0, expected_value_type);
                        errors.push(ErrorNode {
                            range: optional_variant_name_range(name),
                            message: error_message.into_boxed_str(),
                        });
                        return None;
                    };
                    let value = patterns.element(value);
                    let Some(compiled_value) = syntax_pattern_to_rust(
                        value,
                        Some(expected_value_type),
                        errors,
                        records_used,
                        choices_used,
                        introduced_variables,
                        type_aliases,
                        patterns,
                        types,
                        origins,
                    ) else {
                        return None;
                    };
                    if let Some(variant_value_type_diff) =
                        type_diff(expected_value_type, &compiled_value.type_)
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
                            path: syn_path_reference([
                                &name_to_uppercase_rust(&variant_names_to_rust_enum_name(
                                    origin_choice_type_variants
                                        .iter()
                                        .map(|variant| &variant.name),
                                )),
                                &name_to_uppercase_rust(name_value),
                            ]),
                            paren_token: syn::token::Paren(syn_span()),
                            elems: std::iter::once(compiled_value.rust).collect(),
                        }),
                        type_: expected_type.clone(),
                        catch: if origin_choice_type_variants.len() == 1 {
                            compiled_value.catch
                        } else {
                            let mut variants: std::collections::BTreeMap<
                                Name,
                                VariantCatch<PatternCatch>,
                            > = origin_choice_type_variants
                                .iter()
                                .map(|variant| {
                                    (
                                        variant.name.clone(),
                                        VariantCatch::Uncaught { has_value: true },
                                    )
                                })
                                .collect();
                            if let Some(variant_catch) = variants.get_mut(name_value) {
                                *variant_catch = VariantCatch::Caught(compiled_value.catch);
                            }
                            PatternCatch::Variant(variants)
                        },
                    })
                }
            }
        }
        SyntaxPattern::RecordEmpty { dot_start: _ } => Some(CompiledPattern {
            rust: syn::Pat::Struct(syn::PatStruct {
                attrs: vec![],
                qself: None,
                path: syn_path_reference([record_empty_rust_struct_name]),
                brace_token: syn::token::Brace(syn_span()),
                fields: syn::punctuated::Punctuated::new(),
                rest: None,
            }),
            type_: Type::Record(vec![]),
            catch: PatternCatch::Exhaustive,
        }),
        SyntaxPattern::Record {
            field0_name,
            field0_value,
            field1_up,
        } => {
            let mut maybe_type_fields: Option<Vec<TypeField>> =
                Some(Vec::with_capacity(1 + field1_up.len()));
            let mut field_catches: std::collections::BTreeMap<Name, PatternCatch> =
                std::collections::BTreeMap::new();
            let mut rust_fields: syn::punctuated::Punctuated<syn::FieldPat, syn::token::Comma> =
                syn::punctuated::Punctuated::new();
            'converting_fields: for (field_name, field_value) in std::iter::once((
                WithStartPosition {
                    start: field0_name.start,
                    value: Some(&field0_name.value),
                },
                field0_value.as_ref().map(|value| patterns.element(value)),
            ))
            .chain(field1_up.iter().map(|field| {
                (
                    WithStartPosition {
                        start: field.name.start,
                        value: field.name.value.as_ref(),
                    },
                    field.value.as_ref(),
                )
            })) {
                let Some(field_name_value) = field_name.value else {
                    errors.push(ErrorNode {
                        range: symbol_range(field_name.start, "."),
                        message: Box::from("missing field name after this dot ."),
                    });
                    return None;
                };
                let Some(field_value) = field_value else {
                    errors.push(ErrorNode {
                        range: field_name_range(WithStartPosition {
                            start: field_name.start,
                            value: field_name_value,
                        }),
                        message: Box::from("missing field value after this field name"),
                    });
                    return None;
                };
                if maybe_type_fields.as_ref().is_some_and(|type_fields| {
                    type_fields
                        .iter()
                        .any(|type_field| type_field.name == field_name_value)
                }) {
                    errors.push(ErrorNode {
                        range: field_name_range(WithStartPosition {
                            start: field_name.start,
                            value: field_name_value,
                        }),
                        message: Box::from(
                            "a field with this name already exists in the record pattern",
                        ),
                    });
                    continue 'converting_fields;
                }
                let maybe_expected_type_record =
                    expected_type.and_then(|expected_type| match expected_type {
                        Type::Variable(_)
                        | Type::Origin(_)
                        | Type::CoreConstruct { .. }
                        | Type::Choice { .. } => None,
                        Type::Record(type_fields) => Some(type_fields),
                    });
                let compiled_field_value = syntax_pattern_to_rust(
                    field_value,
                    maybe_expected_type_record.and_then(|expected_record_type| {
                        // TODO report if this is none
                        expected_record_type
                            .iter()
                            .find(|expected_field| expected_field.name == field_name_value)
                            .map(|expected_field| &expected_field.value)
                    }),
                    errors,
                    records_used,
                    choices_used,
                    introduced_variables,
                    type_aliases,
                    patterns,
                    types,
                    origins,
                );
                let Some(compiled_field_value) = compiled_field_value else {
                    return None;
                };
                if let Some(type_fields) = &mut maybe_type_fields {
                    type_fields.push(TypeField {
                        name: field_name_value.clone(),
                        value: compiled_field_value.type_,
                    });
                }
                field_catches.insert(field_name_value.clone(), compiled_field_value.catch);
                rust_fields.push(syn::FieldPat {
                    attrs: vec![],
                    member: syn::Member::Named(syn_ident(&name_to_lowercase_rust(
                        field_name_value,
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
                choices_used,
                introduced_variables,
                type_aliases,
                patterns,
                types,
                origins,
            ),
        },
    }
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
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CompiledTypeAliasInfo>,
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
                    types,
                    origins,
                    records_used,
                    choices_used,
                ) else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                let maybe_compiled = match &type_ {
                    Type::CoreConstruct { name, arguments: _ } => match name.as_str() {
                        "p32" => match value.value.parse::<std::num::NonZeroU32>() {
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
                        "u32" => match value.value.parse::<u32>() {
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
                        "i32" => match value.value.parse::<i32>() {
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
                        "f32" => match value.value.parse::<f32>() {
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
        SyntaxExpression::Variable(name) => {
            if let Some(_origin_info) = origins.get(&name.value) {
                let maybe_existing_origin_variable_use_start =
                    used_origin_variables.insert(&name.value, name.start);
                if let Some(existing_origin_variable_use_start) =
                    maybe_existing_origin_variable_use_start
                {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!("this origin variable is already used earlier starting at {}. Each value can only be used once, that includes origins. Each collection needs its own origin", position_to_string(existing_origin_variable_use_start)).into_boxed_str(),
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                }
                let rust_reference: syn::Expr =
                    syn_expr_reference([&name_to_lowercase_rust(&name.value)]);
                CompiledExpression {
                    rust: rust_reference,
                    type_: Some(type_origin(Type::Origin(name.value.clone()))),
                }
            } else if let Some(variable_info) = pattern_variables.get(&name.value) {
                let maybe_existing_pattern_variable_use_start =
                    used_pattern_variables.insert(&name.value, name.start);
                if let Some(existing_pattern_variable_use_start) =
                    maybe_existing_pattern_variable_use_start
                {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!("this variable is already used earlier starting at {}. Each value can only be used once, even simple numbers etc. To duplicate the value, use the helpers like u32-dup, char-dup or create your own dup helpers", position_to_string(existing_pattern_variable_use_start)).into_boxed_str(),
                    });
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                }
                let rust_reference: syn::Expr =
                    syn_expr_reference([&name_to_lowercase_rust(&name.value)]);
                let Some(variable_type) = variable_info.type_.clone() else {
                    return CompiledExpression {
                        rust: syn_expr_todo(),
                        type_: None,
                    };
                };
                CompiledExpression {
                    rust: rust_reference,
                    type_: Some(variable_type),
                }
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from(
                        if let Some(_) = project_fns.get(name.value.as_str()) {
                            "functions always need to be called with an argument and start with an underscore, like _u32-add .a 0 u32 .b 1 u32. Otherwise check for typos."
                        } else {
                            "unknown variable name. No local variable has this name. Note that a local fn result can not refer to any variable from the outside. Otherwise check for typos."
                        }
                    )
                });
                CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                }
            }
        }
        SyntaxExpression::Call {
            underscore_start,
            name,
            type_arguments,
            argument: syntax_argument,
        } => {
            let Some(name) = name else {
                errors.push(ErrorNode {
                    range: symbol_range(*underscore_start, "_"),
                    message: Box::from("missing function name after this _"),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            if let Some(variable_info) = pattern_variables.get(&name.value) {
                let maybe_existing_pattern_variable_use_start =
                    used_pattern_variables.insert(&name.value, name.start);
                if let Some(existing_pattern_variable_use_start) =
                    maybe_existing_pattern_variable_use_start
                {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
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
                            choices_used,
                            type_aliases,
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
                        let variable_type_arguments = match variable_type {
                            Type::CoreConstruct {
                                name: variable_type_name,
                                arguments: variable_type_arguments,
                            } if variable_type_name == "fn" => variable_type_arguments,
                            variable_type => {
                                let mut error_message = String::from(
                                    "calling a variable whose type is not a function. Maybe you forgot some parens or similar? Its full type is\n",
                                );
                                type_format(&mut error_message, 4, &variable_type);
                                errors.push(ErrorNode {
                                    range: name_range(with_start_position_as_ref(name)),
                                    message: error_message.into_boxed_str(),
                                });
                                return CompiledExpression {
                                    rust: syn_expr_todo(),
                                    type_: None,
                                };
                            }
                        };
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
            } else if let Some(_origin_info) = origins.get(&name.value) {
                let maybe_existing_origin_variable_use_start =
                    used_origin_variables.insert(&name.value, name.start);
                if let Some(existing_origin_variable_use_start) =
                    maybe_existing_origin_variable_use_start
                {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
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
                    type_: Some(type_origin(Type::Origin(name.value.clone()))),
                }
            } else {
                let Some(project_fn_info) = project_fns.get(name.value.as_str()) else {
                    errors.push(ErrorNode { range: name_range(with_start_position_as_ref(name)), message: Box::from("unknown name. No project fn or local variable has this name. Note that a local fn can not refer to any variable from the outside. Otherwise check for typos.") });
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
                            range: name_range(with_start_position_as_ref(name)),
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
                        types,
                        origins,
                        records_used,
                        choices_used,
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
                            choices_used,
                            type_aliases,
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
        SyntaxExpression::Variant { name, type_, value } => {
            let Some(name_value) = &name.value else {
                errors.push(ErrorNode {
                    range: optional_variant_name_range(name),
                    message: Box::from("missing variant name after this bar |. An example of a valid variant is |present (opt str) \"hi c:\""),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Some(syntax_type) = type_ else {
                errors.push(ErrorNode {
                    range: symbol_range(name.start, "|"),
                    message: Box::from("missing type after this variant name. An example of a valid variant is |present (opt str) \"hi c:\""),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Some(compiled_type) = syntax_type_to_type(
                syntax_type,
                errors,
                type_aliases,
                types,
                origins,
                records_used,
                choices_used,
            ) else {
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Type::Choice(origin_choice_type) = &compiled_type else {
                let mut error_message: String = String::from(
                    "this variant type should be a choice (for example |a u32 |b str) but it's\n",
                );
                type_format(&mut error_message, 0, &compiled_type);
                errors.push(ErrorNode {
                    range: optional_variant_name_range(name),
                    message: error_message.into_boxed_str(),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Some(expected_value_type) = origin_choice_type.iter().find_map(|variant| {
                if variant.name == name_value {
                    Some(&variant.value)
                } else {
                    None
                }
            }) else {
                let mut error_message: String = format!(
                    "the actual variant name {} is not included in this type\n",
                    name_value
                );
                type_format(&mut error_message, 0, &compiled_type);
                errors.push(ErrorNode {
                    range: type_range(syntax_type, types),
                    message: error_message.into_boxed_str(),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Some(value) = value else {
                let mut error_message: String =
                    String::from("this variant is missing its associated value of type\n");
                type_format(&mut error_message, 0, expected_value_type);
                errors.push(ErrorNode {
                    range: optional_variant_name_range(name),
                    message: error_message.into_boxed_str(),
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let value = expressions.element(value);
            let CompiledExpression {
                type_: Some(compiled_value_type),
                rust: compiled_value_rust,
            } = syntax_expression_to_rust(
                errors,
                records_used,
                choices_used,
                type_aliases,
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
            if let Some(variant_value_type_diff) =
                type_diff(expected_value_type, &compiled_value_type)
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
                    func: Box::new(syn::Expr::Path(syn::ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: syn_path_reference([
                            &name_to_uppercase_rust(&variant_names_to_rust_enum_name(
                                origin_choice_type.iter().map(|variant| &variant.name),
                            )),
                            &name_to_uppercase_rust(name_value),
                        ]),
                    })),
                    paren_token: syn::token::Paren(syn_span()),
                    args: std::iter::once(compiled_value_rust).collect(),
                }),
                type_: Some(compiled_type),
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            parameter,
            angle_right_start: _,
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
                choices_used,
                &mut parameter_introduced_variables,
                type_aliases,
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
                choices_used,
                type_aliases,
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
                    range: name_range(WithStartPosition {
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
            for (parameter_introduced_variable_name, parameter_introduced_variable_origin) in
                parameter_introduced_variables
            {
                push_error_if_introduced_pattern_variable_is_unused(
                    errors,
                    parameter_introduced_variable_origin.origin_start,
                    parameter_introduced_variable_name,
                    result_used_pattern_variables
                        .get(parameter_introduced_variable_name)
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
                type_: Some(type_fn(compiled_parameter.type_, actual_result_type)),
            }
        }
        SyntaxExpression::RecordEmpty { dot_start: _ } => CompiledExpression {
            rust: syn::Expr::Struct(syn::ExprStruct {
                attrs: vec![],
                qself: None,
                path: syn_path_reference([record_empty_rust_struct_name]),
                brace_token: syn::token::Brace(syn_span()),
                fields: syn::punctuated::Punctuated::new(),
                dot2_token: None,
                rest: None,
            }),
            type_: Some(Type::Record(vec![])),
        },
        SyntaxExpression::Record {
            field0_name,
            field0_value,
            field1_up,
        } => {
            let compiled_field_value: CompiledExpression = match &field0_value {
                None => {
                    errors.push(ErrorNode {
                        range: field_name_range(with_start_position_as_ref(field0_name)),
                        message: Box::from(
                            "missing field value expression after this first field name",
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
                    choices_used,
                    type_aliases,
                    project_fns,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    used_pattern_variables,
                    origins,
                    used_origin_variables,
                    expressions.element(field_value),
                ),
            };
            let mut rust_fields = syn::punctuated::Punctuated::new();
            let mut maybe_field_types: Option<Vec<TypeField>> = match compiled_field_value.type_ {
                None => None,
                Some(compiled_value_type) => {
                    rust_fields.push(syn::FieldValue {
                        attrs: vec![],
                        member: syn::Member::Named(syn_ident(&name_to_lowercase_rust(
                            &field0_name.value,
                        ))),
                        colon_token: Some(syn::token::Colon(syn_span())),
                        expr: compiled_field_value.rust,
                    });
                    Some(vec![TypeField {
                        name: field0_name.value.clone(),
                        value: compiled_value_type,
                    }])
                }
            };
            'compiling_fields: for field in field1_up {
                let Some(field_name) = &field.name.value else {
                    errors.push(ErrorNode {
                        range: symbol_range(field.name.start, "."),
                        message: Box::from("missing field value expression after this field name"),
                    });
                    continue 'compiling_fields;
                };
                let compiled_field_value: CompiledExpression = match &field.value {
                    None => {
                        errors.push(ErrorNode {
                            range: optional_field_name_range(&field.name),
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
                        choices_used,
                        type_aliases,
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
                if let Some(field_types) = &mut maybe_field_types {
                    match compiled_field_value.type_ {
                        None => {
                            maybe_field_types = None;
                        }
                        Some(compiled_value_type) => {
                            field_types.push(TypeField {
                                name: field_name.clone(),
                                value: compiled_value_type,
                            });
                            rust_fields.push(syn::FieldValue {
                                attrs: vec![],
                                member: syn::Member::Named(syn_ident(&name_to_lowercase_rust(
                                    field_name,
                                ))),
                                colon_token: Some(syn::token::Colon(syn_span())),
                                expr: compiled_field_value.rust,
                            });
                        }
                    }
                }
            }
            match maybe_field_types {
                None => CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                },
                Some(field_types) => {
                    let field_names: Vec<Name> =
                        sorted_field_names(field_types.iter().map(|field| &field.name));
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
                        type_: Some(Type::Record(field_types)),
                    }
                }
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
                choices_used,
                type_aliases,
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
                choices_used,
                type_aliases,
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
            question_mark_start,
            queried,
            cases,
        } => {
            let Some(queried) = queried else {
                errors.push(ErrorNode {
                    range: symbol_range(*question_mark_start, "?"),
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
                    range: symbol_range(*question_mark_start, "?"),
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
                choices_used,
                type_aliases,
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
            let Some(case0_pattern) = &case0.pattern else {
                errors.push(ErrorNode {
                    range:  symbol_range(case0.equals_start, "="),
                    message: Box::from("missing query case pattern after this equals = . Cases consist of = pattern > result-expression. A full query could look like :option = |present n > n = |absent > 0 u32")
                });
                return CompiledExpression {
                    rust: syn_expr_todo(),
                    type_: None,
                };
            };
            let Some(case0_result) = &case0.result else {
                errors.push(ErrorNode {
                    range: case0.right_angle_start.map(|right_angle_start| symbol_range(right_angle_start, ">")).unwrap_or_else(|| pattern_range(case0_pattern, patterns, types)),
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
                case0_pattern,
                Some(&compiled_queried_type),
                errors,
                records_used,
                choices_used,
                &mut case0_pattern_introduced_variables,
                type_aliases,
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
                choices_used,
                type_aliases,
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
            for (case0_pattern_introduced_variable, case0_pattern_introduced_variable_origin) in
                case0_pattern_introduced_variables
            {
                push_error_if_introduced_pattern_variable_is_unused(
                    errors,
                    case0_pattern_introduced_variable_origin.origin_start,
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
            let mut cases_were_skipped = false;
            'compiling_case1_up: for (case_index, case) in case1_up
                .iter()
                .enumerate()
                .map(|(i_in_1up, case)| (i_in_1up + 1, case))
            {
                let Some(case_pattern) = &case.pattern else {
                    errors.push(ErrorNode {
                        range:  symbol_range(case.equals_start, "="),
                        message: Box::from("missing query case pattern after this equals = . Cases consist of = pattern > result-expression. A full query could look like :option = |present n > n = |absent > 0 u32")
                    });
                    continue 'compiling_case1_up;
                };
                let mut case_pattern_introduced_variables: std::collections::HashMap<
                    &Name,
                    PatternVariableCompileInfo,
                > = std::collections::HashMap::new();
                let Some(case_pattern_compiled) = syntax_pattern_to_rust(
                    case_pattern,
                    Some(&compiled_queried_type),
                    errors,
                    records_used,
                    choices_used,
                    &mut case_pattern_introduced_variables,
                    type_aliases,
                    patterns,
                    types,
                    origins,
                ) else {
                    cases_were_skipped = true;
                    continue 'compiling_case1_up;
                };
                pattern_variables.extend(
                    case_pattern_introduced_variables
                        .iter()
                        .map(|(binding, info)| (*binding, info.clone())),
                );
                if let Some(queried_pattern_type_diff) =
                    type_diff(&compiled_queried_type, &case_pattern_compiled.type_)
                {
                    errors.push(ErrorNode {
                        range: pattern_range(case_pattern, patterns, types),
                        message: (type_diff_error_message(&queried_pattern_type_diff)
                            + "\n\nA query case pattern must have the same type as the queried expression")
                                .into_boxed_str(),
                    });
                    cases_were_skipped = true;
                    continue 'compiling_case1_up;
                }
                pattern_catch_merge_with(
                    errors,
                    pattern_range(case_pattern, patterns, types),
                    &mut catch,
                    case_pattern_compiled.catch,
                );
                let Some(case_result) = &case.result else {
                    errors.push(ErrorNode {
                        range: case.right_angle_start.map(|right_angle_start| symbol_range(right_angle_start, "<")).unwrap_or_else(||pattern_range(case_pattern, patterns, types)),
                        message: Box::from("missing result expression after this query case pattern. Cases can be (pattern result-expression) or pattern result-expression for the last one. An example of a query is : option = |present n > n = |absent > 0 u32")
                    });
                    rust_arms.push(syn_arm(case_pattern_compiled.rust, syn_expr_todo()));
                    continue 'compiling_case1_up;
                };
                let mut case_result_used_pattern_variables = std::collections::HashMap::new();
                let mut case_result_used_origin_variables = std::collections::HashMap::new();
                let CompiledExpression {
                    rust: case_compiled_result_rust,
                    type_: Some(case_result_type),
                } = syntax_expression_to_rust(
                    errors,
                    records_used,
                    choices_used,
                    type_aliases,
                    project_fns,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    &mut case_result_used_pattern_variables,
                    origins,
                    &mut case_result_used_origin_variables,
                    case_result,
                )
                else {
                    rust_arms.push(syn_arm(case_pattern_compiled.rust, syn_expr_todo()));
                    continue 'compiling_case1_up;
                };
                for (case_pattern_introduced_variable, case0_pattern_introduced_variable_origin) in
                    case_pattern_introduced_variables
                {
                    push_error_if_introduced_pattern_variable_is_unused(
                        errors,
                        case0_pattern_introduced_variable_origin.origin_start,
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
                            range: name_range(WithStartPosition { value: case_result_used_pattern_variable, start: case_result_used_pattern_variable_start }),
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
                            range: name_range(WithStartPosition { value: case0_result_used_pattern_variable, start: case0_result_used_pattern_variable_start }),
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
                            range: name_range(WithStartPosition { value: case_result_used_origin_variable, start: case_result_used_origin_variable_start }),
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
                            range: name_range(WithStartPosition { value: case0_result_used_origin_variable, start: case0_result_used_origin_variable_start }),
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
                    rust_arms.push(syn_arm(case_pattern_compiled.rust, syn_expr_todo()));
                    continue 'compiling_case1_up;
                }
                fn syn_arm(pattern: syn::Pat, result: syn::Expr) -> syn::Arm {
                    syn::Arm {
                        attrs: vec![],
                        pat: pattern,
                        guard: None,
                        fat_arrow_token: syn::token::FatArrow(syn_span()),
                        body: Box::new(syn::Expr::Block(syn::ExprBlock {
                            attrs: vec![],
                            label: None,
                            block: syn_spread_expr_block(result),
                        })),
                        comma: None,
                    }
                }
                rust_arms.push(syn_arm(
                    case_pattern_compiled.rust,
                    case_compiled_result_rust,
                ));
            }
            match catch {
                CasePatternsCatch::Exhaustive => {}
                _catch_not_exhaustive => {
                    if !cases_were_skipped {
                        errors.push(ErrorNode {
                            range: symbol_range(*question_mark_start, "?"),
                            message: Box::from("inexhaustive pattern match.
    A pattern match must cover all possible cases, otherwise the program would need to crash if such a value was matched on.
    It might be that a case is not indented enough."),
                        });
                    }
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
                if let Some(existing_origin_with_same_name) = origins.remove(&origin_name.value) {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(origin_name)),
                        message: format!(
                            "an origin with this name already exists at {}",
                            position_to_string(existing_origin_with_same_name.origin_start)
                        )
                        .into_boxed_str(),
                    });
                } else if core_type_aliases.contains_key(&origin_name.value) {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(origin_name)),
                        message: Box::from(
                            "a core choice type with this name already exists. Rename this origin",
                        ),
                    });
                } else if type_aliases.contains_key(&origin_name.value) {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(origin_name)),
                        message: Box::from(
                            "a type alias with this name already exists. Rename this origin",
                        ),
                    });
                }
                origins.insert(
                    &origin_name.value,
                    OriginCompileInfo {
                        origin_start: origin_name.start,
                    },
                );
            }
            let result_compiled = syntax_expression_to_rust(
                errors,
                records_used,
                choices_used,
                type_aliases,
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
                        range: name_range(with_start_position_as_ref(origin_name)),
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
                    range: name_range(with_start_position_as_ref(origin_name)),
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
                    block: syn::Block {
                        brace_token: syn::token::Brace(syn_span()),
                        stmts: vec![
                            syn::Stmt::Macro(syn::StmtMacro {
                                attrs: vec![],
                                mac: syn::Macro {
                                    path: syn_path_reference(["origin_new"]),
                                    bang_token: syn::token::Not(syn_span()),
                                    delimiter: syn::MacroDelimiter::Paren(syn::token::Paren(
                                        syn_span(),
                                    )),
                                    tokens: {
                                        let mut token_stream = proc_macro2::TokenStream::new();
                                        proc_macro2::TokenStream::append_separated(
                                            &mut token_stream,
                                            [
                                                syn_ident(&name_to_lowercase_rust(
                                                    origin_name.value.as_str(),
                                                )),
                                                syn_ident(&name_to_uppercase_rust(
                                                    origin_name.value.as_str(),
                                                )),
                                            ],
                                            syn::token::Comma(syn_span()),
                                        );
                                        token_stream
                                    },
                                },
                                semi_token: None,
                            }),
                            syn::Stmt::Expr(result_compiled.rust, None),
                        ],
                    },
                }),
                type_: result_compiled.type_,
            }
        }
    }
}
fn push_error_if_introduced_pattern_variable_is_unused(
    errors: &mut Vec<ErrorNode>,
    origin_start: lsp_types::Position,
    binding_name: &Name,
    binding_use: Option<lsp_types::Position>,
) {
    if binding_use.is_none() {
        errors.push(ErrorNode {
            range: name_range(WithStartPosition {
                value: binding_name,
                start: origin_start,
            }),
            message: Box::from(
                "this pattern variable is not used in the resulting expression. Use it or replace this variable by _ to explicitly never handle the incoming value"
            )
        });
    }
}
fn type_references_origin(type_: &Type, origin: &Name) -> bool {
    match type_ {
        Type::Variable(_) => false,
        Type::Origin(name) => name == origin,
        Type::Record(fields) => fields
            .iter()
            .any(|field| type_references_origin(&field.value, origin)),
        Type::Choice(variants) => variants
            .iter()
            .any(|variant| type_references_origin(&variant.value, origin)),
        Type::CoreConstruct { name: _, arguments } => arguments
            .iter()
            .any(|argument| type_references_origin(argument, origin)),
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
        SyntaxType::ConstructWithoutArguments(_) => {}
        SyntaxType::ConstructWithArguments {
            underscore_start: _,
            name: _,
            argument0,
            argument1_up,
        } => {
            if let Some(argument0) = argument0 {
                syntax_type_variables_into(type_variables, types.element(argument0), types);
            }
            for argument in argument1_up {
                if let Some(argument_type) = &argument.type_ {
                    syntax_type_variables_into(type_variables, argument_type, types);
                }
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
        SyntaxType::RecordEmpty { dot_start: _ } => {}
        SyntaxType::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            if let Some(field0_value) = field0_value {
                syntax_type_variables_into(type_variables, types.element(field0_value), types);
            }
            for field in field1_up {
                if let Some(value) = &field.value {
                    syntax_type_variables_into(type_variables, value, types);
                }
            }
        }
        SyntaxType::ChoiceEmpty { bar_start: _ } => {}
        SyntaxType::Choice {
            variant0_name: _,
            variant0_value,
            variant1_up,
        } => {
            if let Some(variant0_value) = variant0_value {
                syntax_type_variables_into(type_variables, types.element(variant0_value), types);
            }
            for variant in variant1_up {
                if let Some(value) = &variant.value {
                    syntax_type_variables_into(type_variables, value, types);
                }
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
        SyntaxPattern::Variant { name: _, value } => {
            if let Some(value) = value {
                syntax_pattern_type_variables_into(
                    type_variables,
                    patterns.element(value),
                    patterns,
                    types,
                );
            }
        }
        SyntaxPattern::RecordEmpty { dot_start: _ } => {}
        SyntaxPattern::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            if let Some(field0_value) = field0_value {
                syntax_pattern_type_variables_into(
                    type_variables,
                    patterns.element(field0_value),
                    patterns,
                    types,
                )
            }
            for field in field1_up {
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
    Origin(Name),
    CoreConstruct {
        name: Name,
        arguments: Vec<TypeDiff>,
    },
    Record(Vec<TypeDiffField>),
    Choice(Vec<TypeDiffVariant>),
}
#[derive(Clone, Debug)]
struct TypeDiffField {
    name: Name,
    value: TypeDiff,
}
#[derive(Clone, Debug)]
struct TypeDiffVariant {
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
        Type::Origin(_) => {}
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
        Type::CoreConstruct { name, arguments } => {
            if let Type::CoreConstruct {
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
        Type::Choice(variants) => {
            if let Type::Choice(concrete_variants) = concrete_type {
                for variant in variants {
                    if let Some(matching_concrete_variant) = concrete_variants
                        .iter()
                        .find(|concrete_variant| concrete_variant.name == variant.name)
                    {
                        type_collect_variables_that_are_concrete_into(
                            type_parameter_replacements,
                            &variant.value,
                            &matching_concrete_variant.value,
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
        Type::Origin(_) => b_type.clone(),
        Type::CoreConstruct {
            name: a_name,
            arguments: a_arguments,
        } => {
            if let Type::CoreConstruct {
                name: b_name,
                arguments: b_arguments,
            } = b_type
                && a_name == b_name
            {
                Type::CoreConstruct {
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
        Type::Choice(a_variants) => {
            if let Type::Choice(b_variants) = b_type {
                Type::Choice(
                    a_variants
                        .iter()
                        .map(|a| TypeVariant {
                            name: a.name.clone(),
                            value: match b_variants.iter().find(|b| b.name == a.name) {
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
        Type::CoreConstruct {
            name: expected_name,
            arguments: expected_arguments,
        } => {
            if let Type::CoreConstruct {
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
                Some(TypeDiff::CoreConstruct {
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
        Type::Origin(expected_name) => {
            if let Type::Origin(actual_name) = actual_type
                && expected_name == actual_name
            {
                None
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
        Type::Choice(expected_variants) => {
            if let Type::Choice(actual_variants) = actual_type
                && expected_variants.len() == actual_variants.len()
                && expected_variants.iter().all(|expected_variant| {
                    actual_variants
                        .iter()
                        .any(|actual_variant| actual_variant.name == expected_variant.name)
                })
            {
                if expected_variants
                    .iter()
                    .filter_map(|expected_variant| {
                        actual_variants
                            .iter()
                            .find(|actual_variant| actual_variant.name == expected_variant.name)
                            .map(|actual_variant| (&expected_variant.value, &actual_variant.value))
                    })
                    .all(|(expected_variant_value, actual_variant_value)| {
                        type_diff(expected_variant_value, actual_variant_value).is_none()
                    })
                {
                    return None;
                }
                Some(TypeDiff::Choice(
                    expected_variants
                        .iter()
                        .filter_map(|expected_variant| {
                            actual_variants
                                .iter()
                                .find(|actual_variant| actual_variant.name == expected_variant.name)
                                .map(|actual_variant| (expected_variant, &actual_variant.value))
                        })
                        .map(|(expected_variant, actual_variant_value)| TypeDiffVariant {
                            name: expected_variant.name.clone(),
                            value: type_diff(&expected_variant.value, actual_variant_value)
                                .unwrap_or_else(|| {
                                    type_to_diff_without_conflict(&expected_variant.value)
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
        Type::Origin(name) => TypeDiff::Origin(name.clone()),
        Type::CoreConstruct { name, arguments } => TypeDiff::CoreConstruct {
            name: name.clone(),
            arguments: arguments
                .iter()
                .map(type_to_diff_without_conflict)
                .collect(),
        },
        Type::Choice(variants) => TypeDiff::Choice(
            variants
                .iter()
                .map(|variant| TypeDiffVariant {
                    name: variant.name.clone(),
                    value: type_to_diff_without_conflict(&variant.value),
                })
                .collect(),
        ),
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
fn type_diff_into(formatted: &mut String, indent: usize, type_diff: &TypeDiff) {
    match type_diff {
        TypeDiff::Conflict { expected, actual } => {
            formatted.push_str("expected:");
            space_or_linebreak_indented_into(
                formatted,
                type_line_span(expected),
                next_indent(indent),
            );
            type_format(formatted, next_indent(indent), expected);
            linebreak_indented_into(formatted, indent);
            formatted.push_str("actual:");
            space_or_linebreak_indented_into(
                formatted,
                type_line_span(actual),
                next_indent(indent),
            );
            type_format(formatted, next_indent(indent), actual);
        }
        TypeDiff::Variable(name) => {
            formatted.push_str(name);
        }
        TypeDiff::Origin(name) => {
            formatted.push_str(name);
        }
        TypeDiff::CoreConstruct { name, arguments } => {
            formatted.push_str(name);
            let line_span: LineSpan = type_diff_line_span(type_diff);
            for argument in arguments {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                type_diff_parenthesized_if_open_ended_into(formatted, next_indent(indent), argument)
            }
        }
        TypeDiff::Record(fields) => match fields.as_slice() {
            [] => {
                formatted.push('.');
            }
            [field0, field1_up @ ..] => {
                type_diff_field_format(formatted, indent, field0);
                let line_span: LineSpan = type_diff_line_span(type_diff);
                for field in field1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    type_diff_field_format(formatted, indent, field);
                }
            }
        },
        TypeDiff::Choice(variants) => match variants.as_slice() {
            [] => {
                formatted.push('|');
            }
            [variant0, variant1_up @ ..] => {
                type_diff_variant_format(formatted, indent, variant0);
                let line_span: LineSpan = type_diff_line_span(type_diff);
                for variant in variant1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    type_diff_variant_format(formatted, indent, variant);
                }
            }
        },
    }
}
fn type_diff_parenthesized_if_open_ended_into(
    formatted: &mut String,
    indent: usize,
    type_diff: &TypeDiff,
) {
    let should_parenthesize_argument: bool = match type_diff {
        TypeDiff::Variable(_) => false,
        TypeDiff::Origin(_) => false,
        TypeDiff::Conflict { .. } => true,
        TypeDiff::CoreConstruct {
            name: _,
            arguments: argument_arguments,
        } => !argument_arguments.is_empty(),
        TypeDiff::Record(fields) => !fields.is_empty(),
        TypeDiff::Choice(variants) => !variants.is_empty(),
    };
    if should_parenthesize_argument {
        formatted.push('(');
        type_diff_into(formatted, indent, type_diff);
        if type_diff_line_span(type_diff) == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
        formatted.push(')');
    } else {
        type_diff_into(formatted, indent, type_diff);
    }
}
fn type_diff_field_format(formatted: &mut String, indent: usize, type_diff_field: &TypeDiffField) {
    formatted.push('.');
    formatted.push_str(&type_diff_field.name);
    space_or_linebreak_indented_into(
        formatted,
        type_diff_line_span(&type_diff_field.value),
        next_indent(indent),
    );
    type_diff_parenthesized_if_open_ended_into(
        formatted,
        next_indent(indent),
        &type_diff_field.value,
    );
}
fn type_diff_variant_format(
    formatted: &mut String,
    indent: usize,
    type_diff_variant: &TypeDiffVariant,
) {
    formatted.push('|');
    formatted.push_str(&type_diff_variant.name);
    space_or_linebreak_indented_into(
        formatted,
        type_diff_line_span(&type_diff_variant.value),
        next_indent(indent),
    );
    type_diff_parenthesized_if_open_ended_into(
        formatted,
        next_indent(indent),
        &type_diff_variant.value,
    );
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
        TypeDiff::Origin(name) => name.len(),
        TypeDiff::CoreConstruct { name, arguments } => {
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
        TypeDiff::Choice(variants) => variants
            .iter()
            .map(|variant| variant.name.len() + type_diff_length_estimate(&variant.value))
            .sum(),
    }
}
pub fn type_format(formatted: &mut String, indent: usize, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            formatted.push_str(name);
        }
        Type::Origin(name) => {
            formatted.push_str(name);
        }
        Type::CoreConstruct { name, arguments } => match arguments.as_slice() {
            [] => {
                formatted.push_str(name);
            }
            [argument0, argument1_up @ ..] => {
                formatted.push('_');
                formatted.push_str(name);
                let line_span: LineSpan = type_line_span(type_);
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                type_parenthesized_if_open_ended_format(formatted, next_indent(indent), argument0);
                for argument in argument1_up {
                    formatted.push(',');
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    type_parenthesized_if_open_ended_format(
                        formatted,
                        next_indent(indent),
                        argument,
                    );
                }
            }
        },
        Type::Record(fields) => match fields.as_slice() {
            [] => {
                formatted.push('.');
            }
            [field0, field1_up @ ..] => {
                type_field_format(formatted, indent, field0);
                let line_span: LineSpan = type_line_span(type_);
                for field in field1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    type_field_format(formatted, indent, field);
                }
            }
        },
        Type::Choice(variants) => match variants.as_slice() {
            [] => {
                formatted.push('|');
            }
            [variant0, variant1_up @ ..] => {
                type_variant_format(formatted, indent, variant0);
                let line_span: LineSpan = type_line_span(type_);
                for variant in variant1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    type_variant_format(formatted, indent, variant);
                }
            }
        },
    }
}
fn type_parenthesized_if_open_ended_format(formatted: &mut String, indent: usize, type_: &Type) {
    let should_parenthesize_argument: bool = match type_ {
        Type::Variable(_) => false,
        Type::Origin(_) => false,
        Type::Record(fields) => !fields.is_empty(),
        Type::Choice(variants) => !variants.is_empty(),
        Type::CoreConstruct { name: _, arguments } => !arguments.is_empty(),
    };
    if should_parenthesize_argument {
        formatted.push('(');
        type_format(formatted, next_indent(indent) + 1, type_);
        if type_line_span(type_) == LineSpan::Multiple {
            linebreak_indented_into(formatted, next_indent(indent));
        }
        formatted.push(')');
    } else {
        type_format(formatted, next_indent(indent), type_);
    }
}
fn type_field_format(formatted: &mut String, indent: usize, type_field: &TypeField) {
    formatted.push('.');
    formatted.push_str(&type_field.name);
    let line_span = type_line_span(&type_field.value);
    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
    type_parenthesized_if_open_ended_format(formatted, next_indent(indent), &type_field.value);
}
fn type_variant_format(formatted: &mut String, indent: usize, type_variant: &TypeVariant) {
    formatted.push('|');
    formatted.push_str(&type_variant.name);
    let line_span = type_line_span(&type_variant.value);
    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
    type_parenthesized_if_open_ended_format(formatted, next_indent(indent), &type_variant.value);
}
fn type_line_span(type_: &Type) -> LineSpan {
    if type_length_estimate(type_) <= type_info_line_length_estimate_maximum {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn type_length_estimate(type_: &Type) -> usize {
    match type_ {
        Type::Variable(variable_name) => variable_name.len(),
        Type::Origin(name) => name.len(),
        Type::CoreConstruct { name, arguments } => {
            name.len() + arguments.iter().map(type_length_estimate).sum::<usize>()
        }
        Type::Record(fields) => fields
            .iter()
            .map(|field| field.name.len() + type_length_estimate(&field.value))
            .sum(),
        Type::Choice(variants) => variants
            .iter()
            .map(|variant| variant.name.len() + type_length_estimate(&variant.value))
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
    // Not sure if type variables in core code can actually collide with generated type names?
    match sanitized.as_str() {
        "Self"
        | "Clone"
        | "Copy"
        | "Debug"
        | record_empty_rust_struct_name
        | choice_empty_rust_struct_name
        | "OwnedSliceIterator"
        | "SpanRaw"
        | "VecIter"
        | "Element"
        | "State" => sanitized + "ø_",
        _ => sanitized,
    }
}
const record_empty_rust_struct_name: &str = "Blank";
const choice_empty_rust_struct_name: &str = "Never";
fn name_to_lowercase_rust(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    let needs_to_be_disambiguated = rust_lowercase_keywords.contains(&sanitized.as_str())
        || match sanitized.as_str() {
            local_unnamed_function_name | "copy_ref_to_owned" | "origin_new" => true,
            _ => false,
        };
    if needs_to_be_disambiguated {
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
        None => record_empty_rust_struct_name.to_string(),
    }
}
fn sorted_variant_names<'a>(variant_names: impl Iterator<Item = &'a Name>) -> Vec<Name> {
    let mut variant_names_vec: Vec<Name> = variant_names.map(Name::clone).collect();
    variant_names_vec.sort_unstable();
    variant_names_vec
}
fn variant_names_to_rust_enum_name<'a>(field_names: impl Iterator<Item = &'a Name>) -> String {
    let mut rust_variant_names_vec: Vec<String> = field_names
        .map(|variant_name| name_to_uppercase_rust(variant_name))
        .collect::<Vec<_>>();
    rust_variant_names_vec.sort_unstable();
    // same separator as field_names_to_rust_record_struct_name
    // but an additional separator appended to single-variant which could otherwise collide with single-field
    let mut variant_names_joined: String = rust_variant_names_vec.join("·");
    match variant_names_joined.get_mut(0..=0) {
        Some(first) => {
            first.make_ascii_uppercase();
            if rust_variant_names_vec.len() == 1 {
                variant_names_joined.push_str("··");
            }
            variant_names_joined
        }
        None => choice_empty_rust_struct_name.to_string(),
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
    Type::CoreConstruct {
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
fn type_choice(variants: impl IntoIterator<Item = (&'static str, Type)>) -> Type {
    Type::Choice(
        variants
            .into_iter()
            .map(|(variant_name, variant_value)| TypeVariant {
                name: Name::const_new(variant_name),
                value: variant_value,
            })
            .collect(),
    )
}
const type_p32: Type = Type::CoreConstruct {
    name: Name::const_new("p32"),
    arguments: vec![],
};
const type_u32: Type = Type::CoreConstruct {
    name: Name::const_new("u32"),
    arguments: vec![],
};
const type_i32: Type = Type::CoreConstruct {
    name: Name::const_new("i32"),
    arguments: vec![],
};
const type_f32: Type = Type::CoreConstruct {
    name: Name::const_new("f32"),
    arguments: vec![],
};
const type_char: Type = Type::CoreConstruct {
    name: Name::const_new("char"),
    arguments: vec![],
};
const type_str: Type = Type::CoreConstruct {
    name: Name::const_new("str"),
    arguments: vec![],
};
fn type_origin(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::const_new("origin"),
        arguments: vec![origin],
    }
}
fn type_origin_rid(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::const_new("origin-rid"),
        arguments: vec![origin],
    }
}
fn type_vec(origin: Type, element: Type) -> Type {
    Type::CoreConstruct {
        name: Name::const_new("vec"),
        arguments: vec![origin, element],
    }
}
fn type_slot(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::const_new("slot"),
        arguments: vec![origin],
    }
}
fn type_span(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::const_new("span"),
        arguments: vec![origin],
    }
}
fn type_opt_span_build(backing: Type) -> Type {
    Type::CoreConstruct {
        name: Name::const_new("opt-span-build"),
        arguments: vec![backing],
    }
}
fn type_span_build(backing: Type) -> Type {
    Type::CoreConstruct {
        name: Name::const_new("span-build"),
        arguments: vec![backing],
    }
}
fn type_opt(present: Type) -> Type {
    type_choice([("absent", type_record([])), ("present", present)])
}
pub static core_fns: std::sync::LazyLock<std::collections::HashMap<Name, CompiledProjectFnInfo>> =
    std::sync::LazyLock::new(|| {
        std::collections::HashMap::from([
            (
                Name::const_new("p32-dup"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the p32 in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_p32),
                    result_type: Some(type_record([("a", type_p32), ("b", type_p32)])),
                },
            ),
            (
                Name::const_new("p32-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given p32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_p32),
                    result_type: Some(type_record([])),
                },
            ),
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
                Name::const_new("u32-dup"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the u32 in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_u32),
                    result_type: Some(type_record([("a", type_u32), ("b", type_u32)])),
                },
            ),
            (
                Name::const_new("u32-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given u32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_u32),
                    result_type: Some(type_record([])),
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
                Name::const_new("i32-dup"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the i32 in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_i32),
                    result_type: Some(type_record([("a", type_i32), ("b", type_i32)])),
                },
            ),
            (
                Name::const_new("i32-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given i32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_i32),
                    result_type: Some(type_record([])),
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
                Name::const_new("f32-dup"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the f32 in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_f32),
                    result_type: Some(type_record([("a", type_f32), ("b", type_f32)])),
                },
            ),
            (
                Name::const_new("f32-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given f32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_f32),
                    result_type: Some(type_record([])),
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
                Name::const_new("char-dup"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the char in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_char),
                    result_type: Some(type_record([("a", type_char), ("b", type_char)])),
                },
            ),
            (
                Name::const_new("char-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given char value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_char),
                    result_type: Some(type_record([])),
                },
            ),
            (
                Name::const_new("str-dup"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the str in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_str),
                    result_type: Some(type_record([("a", type_str), ("b", type_str)])),
                },
            ),
            (
                Name::const_new("str-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given str value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_str),
                    result_type: Some(type_record([])),
                },
            ),
            (
                Name::const_new("fn-dup"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the fn in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_fn(type_variable("In"), type_variable("Out"))),
                    result_type: Some(type_record([
                        ("a", type_fn(type_variable("In"), type_variable("Out"))),
                        ("b", type_fn(type_variable("In"), type_variable("Out"))),
                    ])),
                },
            ),
            (
                Name::const_new("fn-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given fn value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_fn(type_variable("In"), type_variable("Out"))),
                    result_type: Some(type_record([])),
                },
            ),
            (
                Name::const_new("origin-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Split the origin-rid in two values with the same content",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_origin_rid(type_variable("Origin"))),
                    result_type: Some(type_record([
                        ("a", type_origin_rid(type_variable("Origin"))),
                        ("b", type_origin_rid(type_variable("Origin"))),
                    ])),
                },
            ),
            (
                Name::const_new("origin-rid-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given origin-rid value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to ignore it only in some case",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_origin_rid(type_variable("Origin"))),
                    result_type: Some(type_record([])),
                },
            ),
            (
                Name::const_new("slot-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given slot value as \"won't be used anymore\", given a proof that the backing collection is gone. This is usually done to scrap some function byproduct or to ignore it only in some case",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        ("slot", type_slot(type_variable("Origin"))),
                        ("origin-rid", type_origin_rid(type_variable("Origin"))),
                    ])),
                    result_type: Some(type_record([])),
                },
            ),
            (
                Name::const_new("span-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given span value as \"won't be used anymore\", given a proof that the backing collection is gone. This is usually done to scrap some function byproduct or to ignore it only in some case",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        ("span", type_span(type_variable("Origin"))),
                        ("origin-rid", type_origin_rid(type_variable("Origin"))),
                    ])),
                    result_type: Some(type_record([])),
                },
            ),
            (
                Name::const_new("opt-span-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given opt span value as \"won't be used anymore\", given a proof that the backing collection is gone. This is usually done to scrap some function byproduct or to ignore it only in some case",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        ("span", type_opt(type_span(type_variable("Origin")))),
                        ("origin-rid", type_origin_rid(type_variable("Origin"))),
                    ])),
                    result_type: Some(type_record([])),
                },
            ),
            (
                Name::const_new("vec-empty"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Initialize a `vec` with 0 elements. Modify with `vec-pre-allocate-at-least`, `vec-add` etc.",
                    )),
                    type_parameters: vec![Name::const_new("Element")],
                    parameter_type: Some(type_origin(type_variable("Origin"))),
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
                Name::const_new("vec-add"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Add a new element into the vec and keep a slot to it.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("new", type_variable("Element")),
                    ])),
                    result_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("slot", type_slot(type_variable("Origin"))),
                    ])),
                },
            ),
            (
                Name::const_new("vec-add-ignoring-vacant"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Add a new element into the vec and keep a slot to it without trying to reuse already vacant slots. Can be faster than vec-add for temporary vecs where all the storage gets scrapped anyway, see also vec-element-without-vacating.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("new", type_variable("Element")),
                    ])),
                    result_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("slot", type_slot(type_variable("Origin"))),
                    ])),
                },
            ),
            (
                Name::const_new("vec-element"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Remove and retrieve an element from the vec at a given slot (the inverse of vec-add).",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("slot", type_slot(type_variable("Origin"))),
                    ])),
                    result_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("element", type_variable("Element")),
                    ])),
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
                Name::const_new("vec-opt-span-build-ignoring-vacant"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Finish an `opt-span-build` and split it into the backing `vec` and the built `opt span`, without trying to reuse vacant spans. Can be faster than vec-add for temporary vecs where all the storage gets scrapped anyway.",
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
            (
                Name::const_new("vec-span-build-ignoring-vacant"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Finish a `span-build` and split it into the backing `vec` and the built `span`, without trying to reuse vacant spans. Can be faster than vec-add for temporary vecs where all the storage gets scrapped anyway.",
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
            (
                Name::const_new("vec-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Mark the given vec value as \"won't be used anymore\". This is usually done for temporary vecs at the end of their scope
once you've used up all its elements.
If any slots or spans are still floating around, you will not be able to get rid of them.
This nicely forces you to handle all remaining elements before you can get rid of the vec.
If you still hold slots and spans to the elements inside
that you don't want to vacate one-by-one yet,
there are also helpers like `vec-fold` or `vec-fold-with-origin-rid`.
",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_vec(type_variable("Origin"), type_variable("Element"))),
                    result_type: Some(type_variable("State")),
                },
            ),
            (
                Name::const_new("vec-fold"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Consume all elements, stepping a state through first to last.
This is mainly intended as a cleanup mechanism to flush out any remaining elements.
```sloe
origin example-origin
? _vec-empty<u32> example-origin = example-vec >
_vec-fold
.vec example-vec
.state .
.step fn .state state .element element u32 > _u32-rid element
```
If you know there are no more slots and spans left, use `vec-rid`.
For regular folds over spans, use helpers like `span-fold`.
If you end up needing to scrap some remaining slots and spans after or during `vec-fold`,
check out `vec-fold-with-origin-rid`.
",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("state", type_variable("State")),
                        (
                            "step",
                            type_fn(
                                type_record([
                                    ("state", type_variable("State")),
                                    ("element", type_variable("Element")),
                                ]),
                                type_variable("State"),
                            ),
                        ),
                    ])),
                    result_type: Some(type_variable("State")),
                },
            ),
            (
                Name::const_new("vec-fold-with-origin-rid"),
                CompiledProjectFnInfo {
                    documentation: Some(Box::from(
                        "Consume all elements, stepping a state through first to last.
If any element contains slots and spans to the exact same vec, you can use the provided `origin-rid`
to get rid of them.
The resulting `origin-rid` can be used to get rid of remaining slots and spans elsewhere.
If you know there were already no more slots and spans left from the start,
use `vec-rid` instead of `vec-fold-with-origin-rid`.",
                    )),
                    type_parameters: vec![],
                    parameter_type: Some(type_record([
                        (
                            "vec",
                            type_vec(type_variable("Origin"), type_variable("Element")),
                        ),
                        ("state", type_variable("State")),
                        (
                            "step",
                            type_fn(
                                type_record([
                                    ("state", type_variable("State")),
                                    ("element", type_variable("Element")),
                                    ("origin-rid", type_origin_rid(type_variable("Origin"))),
                                ]),
                                type_variable("State"),
                            ),
                        ),
                    ])),
                    result_type: Some(type_record([
                        ("state", type_variable("State")),
                        ("origin-rid", type_origin_rid(type_variable("Origin"))),
                    ])),
                },
            ),
        ])
    });
pub static core_type_aliases: std::sync::LazyLock<
    std::collections::HashMap<Name, CompiledTypeAliasInfo>,
> = std::sync::LazyLock::new(|| {
    std::collections::HashMap::from([
        (
            Name::const_new("p32"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 1 (positive integer) with 32 bits.
```sloe
fn answer . :> p32 >
    _p32-add .p 2 p32 .u 40 u32
```
",
                )),
                parameters: vec![],
                type_: Some(type_p32),
                is_copy: true,
            },
        ),
        (
            Name::const_new("u32"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 0 (unsigned integer) with 32 bits.
```sloe
fn answer . :> u32 >
    _u32-add .a 2 u32 .b 40 u32
```
",
                )),
                parameters: vec![],
                type_: Some(type_u32),
                is_copy: true,
            },
        ),
        (
            Name::const_new("i32"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed whole number (integer) with 32 bits.
```sloe
fn answer . :> i32 >
    _i32-add .a -8 i32 .b 50 i32
```
",
                )),
                parameters: vec![],
                type_: Some(type_i32),
                is_copy: true,
            },
        ),
        (
            Name::const_new("f32"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed decimal number (floating-point) with 32 bit precision.
Does not allow infinities or NaN. If you need these error states, explicitly model them with a choice type.
```sloe
fn answer . :> f32 >
    _f32-add .a -8.5 f32 .b 50.5 f32
```
",
                )),
                parameters: vec![],
                type_: Some(type_f32),
                is_copy: true,
            },
        ),
        (
            Name::const_new("char"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r#"A unicode scalar like `'a'` or `'👀'` or `'\u{2665}'` (hex code for ♥).
Keep in mind that a human-readable visual symbol can be composed of multiple such unicode scalars (forming a grapheme cluster), For example:
```sloe
_str-start "🇺🇸"
# = |present .start '\u{1F1FA}' .after "\u{1F1F8}"
#                   Indicator U        Indicator S
```
Read if interested: [swift's grapheme cluster docs](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/stringsandcharacters/#Extended-Grapheme-Clusters)
"#,
                )),
                parameters: vec![],
                type_: Some(type_char),
                is_copy: true,
            },
        ),
        (
            Name::const_new("str"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r#"Text valid for the entire program like `"abc"` or `"\"hello 👀 \\\r\n world \u{2665}\""` (`\u{2665}` represents the hex code for ♥, `\"` represents ", `\\` represents \\, `\n` represents line break, `\r` represents carriage return).
Internally, a string is compactly represented as UTF-8 bytes and can be accessed as such.
When building strings, use functions like `arena-add-str`.
"#,
                )),
                parameters: vec![],
                type_: Some(type_str),
                is_copy: true,
            },
        ),
        (
            Name::const_new("opt"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    r"Either you have some value or you have nothing.",
                )),
                parameters: vec![Name::const_new("A")],
                type_: Some(type_opt(type_variable("A"))),
                is_copy: true,
            },
        ),
        (
            Name::const_new("origin"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "Each variable created with `origin some-origin expression` is of this type.
Origins can not be arbitrary values because values like `u32` could be duplicated leading to different collections with the same origin type.
This is not possible for values of type `origin`.
The type argument to an `origin` is the type that also gets created with `origin some-origin expression`.
This type argument is also used in slot, span, arena, vec as the first type argument.
"
                )),
                parameters: vec![Name::const_new("LocalOrigin")],
                type_: Some(type_origin(type_variable("LocalOrigin"))),
                is_copy: false,
            },
        ),
        (
            Name::const_new("origin-rid"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "If you get this value (for example from `vec-fold-with-origin-rid`),
it's proof that the value with this origin is not available anymore.
Providing an `origin-rid` allows you to get rid of any remaining slots or spans you have lying around,
see `slot-rid`, `span-rid`, `opt-span-rid`.
"
                )),
                parameters: vec![Name::const_new("Origin")],
                type_: Some(type_origin_rid(type_variable("Origin"))),
                is_copy: true,
            },
        ),
        (
            Name::const_new("vec"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A grow- and shrinkable array of elements. Arrays have constant time access and update and constant time add.
```sloe
fn use-a-vec & u32
    origin my-elements-origin
    ? _vec-empty<u32> my-elements-origin = my-elements >
    ? _vec-add .vec my-elements .element 609 u32 = .vec my-elements .slot first-element-slot >
    ? _vec-element .vec my-elements .slot first-element-slot = .vec _ .element first-element >
    first-element # = 609 u32
```
"
                )),
                parameters: vec![Name::const_new("Origin"), Name::const_new("Element")],
                type_: Some(type_vec(type_variable("Origin"), type_variable("Element"))),
                is_copy: false,
            },
        ),
        (
            Name::const_new("slot"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A valid index into a collection.
This works because each collection has a unique origin and only gives out one slot for each index.
"
                )),
                parameters: vec![Name::const_new("Origin")],
                type_: Some(type_slot(type_variable("Origin"))),
                is_copy: false,
            },
        ),
        (
            Name::const_new("span"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A range of consecutive valid indexes into a collection with at least one known index.
This works because each collection has a unique origin and only gives out one span for each range.
"
                )),
                parameters: vec![Name::const_new("Origin")],
                type_: Some(type_span(type_variable("Origin"))),
                is_copy: false,
            },
        ),
        (
            Name::const_new("opt-span-build"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "An `opt span` at the end of a backing collecion, plus that collection.
This makes it easy to add elements to the end, as we know there's enough space to occupy.
"
                )),
                parameters: vec![Name::const_new("Backing")],
                type_: Some(type_opt_span_build(type_variable("Backing"))),
                is_copy: false,
            },
        ),
        (
            Name::const_new("span-build"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A `span` at the end of a backing collecion, plus that collection.
This makes it easy to add elements to the end, as we know there's enough space to occupy.
"
                )),
                parameters: vec![Name::const_new("Backing")],
                type_: Some(type_span_build(type_variable("Backing"))),
                is_copy: false,
            },
        ),
        (
            Name::const_new("fn"),
            CompiledTypeAliasInfo {
                name_range: None,
                documentation: Some(Box::from(
                    "A `span` at the end of a backing collecion, plus that collection.
This makes it easy to add elements to the end, as we know there's enough space to occupy.
"
                )),
                parameters: vec![Name::const_new("In"), Name::const_new("Out")],
                type_: Some(type_fn(type_variable("In"), type_variable("Out"))),
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
                        sorted_field_names(fields.iter().map(|field| &field.name))
                            // static variables will only be created once and won't be dropped anyway
                            .leak(),
                    );
                    for field in fields {
                        type_records(&field.value, records);
                    }
                }
                Type::Origin(_) => {}
                Type::Choice(variants) => {
                    for variant in variants {
                        type_records(&variant.value, records);
                    }
                }
                Type::CoreConstruct { name: _, arguments } => {
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
pub static core_choices: std::sync::LazyLock<std::collections::HashSet<&'static [Name]>> =
    std::sync::LazyLock::new(|| {
        fn type_choices(type_: &Type, records: &mut std::collections::HashSet<&'static [Name]>) {
            match type_ {
                Type::Variable(_) => {}
                Type::Record(fields) => {
                    for field in fields {
                        type_choices(&field.value, records);
                    }
                }
                Type::Origin(_) => {}
                Type::Choice(variants) => {
                    records.insert(
                        sorted_variant_names(variants.iter().map(|field| &field.name))
                            // static variables will only be created once and won't be dropped anyway
                            .leak(),
                    );
                    for variant in variants {
                        type_choices(&variant.value, records);
                    }
                }
                Type::CoreConstruct { name: _, arguments } => {
                    for argument in arguments {
                        type_choices(argument, records);
                    }
                }
            }
        }
        let mut variants = std::collections::HashSet::new();
        for core_fn_info in core_fns.values() {
            if let Some(parameter_type) = &core_fn_info.parameter_type {
                type_choices(parameter_type, &mut variants);
            }
            if let Some(result_type) = &core_fn_info.result_type {
                type_choices(result_type, &mut variants);
            }
        }
        variants
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
    formatted.push_str("# ");
    formatted.push_str(comments.line0.value.trim());
    linebreak_indented_into(formatted, indent);
    for line in &comments.line1_up {
        formatted.push_str("# ");
        formatted.push_str(line.value.trim());
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
                ty_keyword_start: _,
                name,
                parameters,
                documentation,
                type_,
            } => {
                formatted.push_str("ty ");
                if let Some(name) = name {
                    formatted.push_str(&name.value);
                }
                if let Some(parameters) = parameters {
                    formatted.push(' ');
                    formatted.push_str(&parameters.parameter0.value);
                    for parameter in parameters
                        .parameter1_up
                        .iter()
                        .filter_map(|parameter| parameter.name.as_ref())
                    {
                        formatted.push_str(", ");
                        formatted.push_str(&parameter.value);
                    }
                }
                match documentation {
                    Some(documentation) => {
                        linebreak_indented_into(&mut formatted, next_indent(0));
                        syntax_comments_format(&mut formatted, next_indent(0), documentation);
                    }
                    None => {
                        formatted.push(' ');
                    }
                }
                if let Some(type_) = type_ {
                    if let SyntaxType::Variable(variable) = type_to_unparenthesized(type_, types) {
                        formatted.push('(');
                        formatted.push_str(&variable.value);
                        formatted.push(')');
                    } else {
                        syntax_type_unparenthesized_format(
                            &mut formatted,
                            next_indent(0),
                            types,
                            type_,
                        );
                    }
                }
            }
            SyntaxProjectElement::Fn {
                fn_keyword_start,
                name,
                type_parameters,
                parameter,
                arrow_start: _,
                result_type,
                angle_right_start: _,
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
                let header_line_span = range_line_span(lsp_types::Range {
                    start: *fn_keyword_start,
                    end: result_type
                        .as_ref()
                        .map(|result_type| type_end(result_type, types))
                        .or_else(|| {
                            parameter
                                .as_ref()
                                .map(|parameter| pattern_end(parameter, patterns, types))
                        })
                        .or_else(|| {
                            name.as_ref()
                                .map(|name| name_end(with_start_position_as_ref(name)))
                        })
                        .unwrap_or_else(|| symbol_end(*fn_keyword_start, "fn")),
                });
                space_or_linebreak_indented_into(&mut formatted, header_line_span, next_indent(0));
                if let Some(parameter) = parameter {
                    syntax_pattern_unparenthesized_format(
                        &mut formatted,
                        next_indent(0),
                        patterns,
                        types,
                        parameter,
                    );
                }
                space_or_linebreak_indented_into(&mut formatted, header_line_span, next_indent(0));
                formatted.push_str(":>");
                space_or_linebreak_indented_into(&mut formatted, header_line_span, next_indent(0));
                if let Some(result_type) = result_type {
                    syntax_type_unparenthesized_format(
                        &mut formatted,
                        next_indent(0),
                        types,
                        result_type,
                    );
                }
                space_or_linebreak_indented_into(&mut formatted, header_line_span, next_indent(0));
                formatted.push('>');
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
fn type_to_unparenthesized<'a, Types>(
    type_: &'a SyntaxType<Types>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
) -> &'a SyntaxType<Types> {
    match type_ {
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => type_,
            Some(inner) => type_to_unparenthesized(types.element(inner), types),
        },
        SyntaxType::Variable(_)
        | SyntaxType::ConstructWithoutArguments(_)
        | SyntaxType::ConstructWithArguments { .. }
        | SyntaxType::RecordEmpty { .. }
        | SyntaxType::Record { .. }
        | SyntaxType::ChoiceEmpty { .. }
        | SyntaxType::Choice { .. } => type_,
    }
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OpenEndKind {
    TypeConstruct,
    TypeChoice,
    Record,
    ExpressionQuery,
}
fn syntax_expression_open_end<Expressions, Patterns, Types>(
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> Option<OpenEndKind> {
    match expression {
        SyntaxExpression::Number { value: _, type_ } => type_
            .as_ref()
            .and_then(|type_| syntax_type_open_end(type_, types)),
        SyntaxExpression::Char { .. } => None,
        SyntaxExpression::Str { .. } => None,
        SyntaxExpression::Variable(_) => None,
        SyntaxExpression::Call {
            underscore_start: _,
            name: _,
            type_arguments: _,
            argument,
        } => argument.as_ref().and_then(|argument| {
            syntax_expression_open_end(expressions.element(argument), expressions, types)
        }),
        SyntaxExpression::Variant {
            name: _,
            type_: _,
            value,
        } => value.as_ref().and_then(|value| {
            syntax_expression_open_end(expressions.element(value), expressions, types)
        }),
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            parameter: _,
            angle_right_start: _,
            result,
        } => result.as_ref().and_then(|result| {
            syntax_expression_open_end(expressions.element(result), expressions, types)
        }),
        SyntaxExpression::RecordEmpty { dot_start: _ } => None,
        SyntaxExpression::Record { .. } => Some(OpenEndKind::Record),
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().and_then(|inner| {
            syntax_expression_open_end(expressions.element(inner), expressions, types)
        }),
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => expression.as_ref().and_then(|expression| {
            syntax_expression_open_end(expressions.element(expression), expressions, types)
        }),
        SyntaxExpression::Query { .. } => Some(OpenEndKind::ExpressionQuery),
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name: _,
            result,
        } => result.as_ref().and_then(|result| {
            syntax_expression_open_end(expressions.element(result), expressions, types)
        }),
    }
}
fn optional_variant_name_format(formatted: &mut String, variant_name: Option<&Name>) {
    formatted.push('|');
    if let Some(variant_name) = variant_name {
        formatted.push_str(variant_name);
    }
}
fn variant_name_format(formatted: &mut String, variant_name: &Name) {
    formatted.push('|');
    formatted.push_str(variant_name);
}
fn optional_field_name_format(formatted: &mut String, field_name: Option<&Name>) {
    formatted.push('.');
    if let Some(field_name) = field_name {
        formatted.push_str(field_name);
    }
}
fn field_name_format(formatted: &mut String, field_name: &Name) {
    formatted.push('.');
    formatted.push_str(field_name);
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
        SyntaxExpression::Variable(name) => {
            formatted.push_str(&name.value);
        }
        SyntaxExpression::Call {
            underscore_start: _,
            name,
            type_arguments,
            argument,
        } => {
            formatted.push('_');
            if let Some(name) = name {
                formatted.push_str(&name.value);
            }
            if let Some(type_arguments) = type_arguments {
                syntax_angled_type_arguments_format(formatted, indent, types, type_arguments);
            }
            if let Some(argument) = argument {
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(expression_range(expression, expressions, patterns, types)),
                    indent,
                );
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
        SyntaxExpression::Variant { name, type_, value } => {
            optional_variant_name_format(formatted, name.value.as_ref());
            formatted.push(' ');
            if let Some(type_) = type_ {
                let type_line_span = range_line_span(type_range(type_, types));
                syntax_type_parenthesized_if_open_ended_format(
                    formatted,
                    indent,
                    types,
                    type_,
                    |_open_end| true,
                );
                space_or_linebreak_indented_into(formatted, type_line_span, indent);
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
            angle_right_start: _,
            result,
        } => {
            formatted.push_str("fn ");
            if let Some(parameter) = parameter {
                let parameter_line_span =
                    range_line_span(pattern_range(parameter, patterns, types));
                syntax_pattern_unparenthesized_format(
                    formatted, indent, patterns, types, parameter,
                );
                space_or_linebreak_indented_into(formatted, parameter_line_span, indent);
                formatted.push('>');
            }
            if let Some(result) = result {
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(expression_range(expression, expressions, patterns, types)),
                    indent,
                );
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
        SyntaxExpression::RecordEmpty { dot_start: _ } => {
            formatted.push('.');
        }
        SyntaxExpression::Record {
            field0_name,
            field0_value,
            field1_up,
        } => {
            field_name_format(formatted, &field0_name.value);
            let field_count = 1 + field1_up.len();
            match field0_value {
                None => {
                    formatted.push(' ');
                }
                Some(value) => {
                    let value = expressions.element(value);
                    maybe_open_end_whitespace_then_element_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_expression_unparenthesized_format(
                                formatted,
                                indent,
                                expressions,
                                patterns,
                                types,
                                value,
                            );
                        },
                        || syntax_expression_open_end(value, expressions, types),
                        OpenEndKind::Record,
                        field_count,
                        0,
                        field0_name.start,
                        expression_range(value, expressions, patterns, types),
                    );
                }
            }
            let line_span =
                range_line_span(expression_range(expression, expressions, patterns, types));
            for (field_index, field) in field1_up.iter().enumerate().map(|(i, e)| (1 + i, e)) {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                optional_field_name_format(formatted, field.name.value.as_ref());
                match &field.value {
                    None => {
                        formatted.push(' ');
                    }
                    Some(value) => {
                        maybe_open_end_whitespace_then_element_format(
                            formatted,
                            indent,
                            |formatted, indent| {
                                syntax_expression_unparenthesized_format(
                                    formatted,
                                    indent,
                                    expressions,
                                    patterns,
                                    types,
                                    value,
                                );
                            },
                            || syntax_expression_open_end(value, expressions, types),
                            OpenEndKind::Record,
                            field_count,
                            field_index,
                            field.name.start,
                            expression_range(value, expressions, patterns, types),
                        );
                    }
                }
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
            question_mark_start,
            queried,
            cases,
        } => {
            formatted.push('?');
            match queried {
                None => {
                    formatted.push(' ');
                }
                Some(queried) => {
                    let queried = expressions.element(queried);
                    parenthesize_if_open_ended_whitespace_then_element_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_expression_unparenthesized_format(
                                formatted,
                                indent,
                                expressions,
                                patterns,
                                types,
                                queried,
                            );
                        },
                        OpenEndKind::ExpressionQuery,
                        syntax_expression_open_end(queried, expressions, types),
                        range_line_span(expression_range(queried, expressions, patterns, types)),
                    );
                }
            }
            match cases.as_slice() {
                [] => {
                    formatted.push(' ');
                }
                [case0, case1_up @ ..] => {
                    let line_span_before_last_case_pattern = range_line_span(lsp_types::Range {
                        start: *question_mark_start,
                        end: {
                            let last_case = case1_up.last().unwrap_or(case0);
                            last_case
                                .right_angle_start
                                .map(|left_angle_start| symbol_end(left_angle_start, ">"))
                                .or_else(|| {
                                    last_case
                                        .pattern
                                        .as_ref()
                                        .map(|pattern| pattern_end(pattern, patterns, types))
                                })
                                .unwrap_or_else(|| symbol_end(last_case.equals_start, "="))
                        },
                    });
                    let case_count = 1 + case1_up.len();
                    space_or_linebreak_indented_into(
                        formatted,
                        line_span_before_last_case_pattern,
                        indent,
                    );
                    syntax_expression_query_case_format(
                        formatted,
                        indent,
                        expressions,
                        patterns,
                        types,
                        case_count,
                        0,
                        case0,
                    );
                    for (case_index, case) in case1_up.iter().enumerate().map(|(i, e)| (i + 1, e)) {
                        space_or_linebreak_indented_into(
                            formatted,
                            line_span_before_last_case_pattern,
                            indent,
                        );
                        syntax_expression_query_case_format(
                            formatted,
                            indent,
                            expressions,
                            patterns,
                            types,
                            case_count,
                            case_index,
                            case,
                        );
                    }
                }
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
            match result {
                None => {
                    formatted.push(' ');
                }
                Some(result) => {
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
}
fn syntax_expression_query_case_format<Expressions, Patterns, Types>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    case_count: usize,
    case_index: usize,
    case: &SyntaxExpressionQueryCase<Expressions, Patterns, Types>,
) {
    formatted.push_str("= ");
    if let Some(pattern) = &case.pattern {
        let pattern_line_span = range_line_span(pattern_range(pattern, patterns, types));
        syntax_pattern_unparenthesized_format(
            formatted,
            next_indent(indent),
            patterns,
            types,
            pattern,
        );
        space_or_linebreak_indented_into(formatted, pattern_line_span, indent);
    }
    formatted.push('>');
    match &case.result {
        None => {
            formatted.push(' ');
        }
        Some(result) => {
            maybe_open_end_whitespace_then_element_format(
                formatted,
                indent,
                |formatted, indent| {
                    syntax_expression_unparenthesized_format(
                        formatted,
                        indent,
                        expressions,
                        patterns,
                        types,
                        result,
                    );
                },
                || syntax_expression_open_end(result, expressions, types),
                OpenEndKind::ExpressionQuery,
                case_count,
                case_index,
                match &case.pattern {
                    None => case.equals_start,
                    Some(case_pattern) => pattern_start(case_pattern),
                },
                expression_range(result, expressions, patterns, types),
            );
        }
    }
}
fn maybe_open_end_whitespace_then_element_format(
    formatted: &mut String,
    indent: usize,
    element_unparenthesized_format: impl FnOnce(&mut String, usize),
    element_open_end: impl FnOnce() -> Option<OpenEndKind>,
    open_end_kind_to_parenthesize_before_last_element: OpenEndKind,
    element_count: usize,
    element_index: usize,
    syntax_before_element_start: lsp_types::Position,
    element_range: lsp_types::Range,
) {
    let line_span = range_line_span(lsp_types::Range {
        start: syntax_before_element_start,
        end: element_range.end,
    });
    if (element_index == element_count - 1) && (element_range.start.character as usize <= indent) {
        space_or_linebreak_indented_into(formatted, line_span, indent);
        element_unparenthesized_format(formatted, indent);
    } else {
        parenthesize_if_open_ended_whitespace_then_element_format(
            formatted,
            indent,
            element_unparenthesized_format,
            open_end_kind_to_parenthesize_before_last_element,
            element_open_end(),
            line_span,
        );
    }
}
fn maybe_open_end_whitespace_then_element_last_always_unparenthesized_format(
    formatted: &mut String,
    indent: usize,
    element_unparenthesized_format: impl FnOnce(&mut String, usize),
    element_open_end: impl FnOnce() -> Option<OpenEndKind>,
    open_end_kind_to_parenthesize_before_last_element: OpenEndKind,
    element_count: usize,
    element_index: usize,
    syntax_before_element_start: lsp_types::Position,
    element_range: lsp_types::Range,
) {
    let line_span = range_line_span(lsp_types::Range {
        start: syntax_before_element_start,
        end: element_range.end,
    });
    if element_index == element_count - 1 {
        space_or_linebreak_indented_into(formatted, line_span, indent);
        element_unparenthesized_format(formatted, indent);
    } else {
        parenthesize_if_open_ended_whitespace_then_element_format(
            formatted,
            indent,
            element_unparenthesized_format,
            open_end_kind_to_parenthesize_before_last_element,
            element_open_end(),
            line_span,
        );
    }
}
fn parenthesize_if_open_ended_whitespace_then_element_format(
    formatted: &mut String,
    indent: usize,
    element_unparenthesized_format: impl FnOnce(&mut String, usize),
    open_end_kind_to_parenthesize_before_last_element: OpenEndKind,
    element_open_end: Option<OpenEndKind>,
    line_span: LineSpan,
) {
    if element_open_end
        .is_some_and(|open_end| open_end == open_end_kind_to_parenthesize_before_last_element)
    {
        formatted.push_str(" (");
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, next_indent(indent));
        }
        element_unparenthesized_format(formatted, next_indent(indent));
        if line_span == LineSpan::Multiple {
            // this one is an explicit decision. Most languages e.g. align the field name with the value close paren.
            // However, I find
            // - having the close on the same line as the value is more legible
            // - having the closing paren on the same level as is confusing and not consistent
            linebreak_indented_into(formatted, next_indent(indent));
        }
        formatted.push(')');
    } else {
        space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
        element_unparenthesized_format(formatted, next_indent(indent));
    }
}
fn syntax_pattern_open_end<Patterns, Types>(
    pattern: &SyntaxPattern<Patterns, Types>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> Option<OpenEndKind> {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => type_
            .as_ref()
            .and_then(|type_| syntax_type_open_end(type_, types)),
        SyntaxPattern::Variant { name: _, value } => value
            .as_ref()
            .and_then(|value| syntax_pattern_open_end(patterns.element(value), patterns, types)),
        SyntaxPattern::RecordEmpty { dot_start: _ } => None,
        SyntaxPattern::Record { .. } => Some(OpenEndKind::Record),
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner
            .as_ref()
            .and_then(|inner| syntax_pattern_open_end(patterns.element(inner), patterns, types)),
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
        SyntaxPattern::Variant { name, value } => {
            optional_variant_name_format(formatted, name.value.as_ref());
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
        SyntaxPattern::RecordEmpty { dot_start: _ } => {
            formatted.push('.');
        }
        SyntaxPattern::Record {
            field0_name,
            field0_value,
            field1_up,
        } => {
            field_name_format(formatted, &field0_name.value);
            let field_count = 1 + field1_up.len();
            match field0_value {
                None => {
                    formatted.push(' ');
                }
                Some(value) => {
                    let value = patterns.element(value);
                    maybe_open_end_whitespace_then_element_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_pattern_unparenthesized_format(
                                formatted, indent, patterns, types, value,
                            );
                        },
                        || syntax_pattern_open_end(value, patterns, types),
                        OpenEndKind::Record,
                        field_count,
                        0,
                        field0_name.start,
                        pattern_range(value, patterns, types),
                    );
                }
            }
            let line_span = range_line_span(pattern_range(pattern, patterns, types));
            for (field_index, field) in field1_up.iter().enumerate().map(|(i, e)| (1 + i, e)) {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                optional_field_name_format(formatted, field.name.value.as_ref());
                match &field.value {
                    None => {
                        formatted.push(' ');
                    }
                    Some(value) => {
                        maybe_open_end_whitespace_then_element_format(
                            formatted,
                            indent,
                            |formatted, indent| {
                                syntax_pattern_unparenthesized_format(
                                    formatted, indent, patterns, types, value,
                                );
                            },
                            || syntax_pattern_open_end(value, patterns, types),
                            OpenEndKind::Record,
                            field_count,
                            field_index,
                            field.name.start,
                            pattern_range(value, patterns, types),
                        );
                    }
                }
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
            |_| true,
        );
        for argument in argument1_up {
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_type_parenthesized_if_open_ended_format(
                formatted,
                next_indent(indent),
                types,
                argument,
                |_| true,
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
    should_parenthesize_open_end_kind: impl FnOnce(OpenEndKind) -> bool,
) {
    if syntax_type_open_end(type_, types).is_some_and(should_parenthesize_open_end_kind) {
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
fn syntax_type_open_end<Types>(
    type_: &SyntaxType<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> Option<OpenEndKind> {
    match type_ {
        SyntaxType::Variable(_) => None,
        SyntaxType::RecordEmpty { dot_start: _ } => None,
        SyntaxType::Record { .. } => Some(OpenEndKind::Record),
        SyntaxType::ChoiceEmpty { bar_start: _ } => None,
        SyntaxType::Choice { .. } => Some(OpenEndKind::TypeChoice),
        SyntaxType::ConstructWithoutArguments(_) => None,
        SyntaxType::ConstructWithArguments { .. } => Some(OpenEndKind::TypeConstruct),
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner
            .as_ref()
            .and_then(|inner| syntax_type_open_end(types.element(inner), types)),
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
        SyntaxType::ConstructWithoutArguments(name) => {
            formatted.push_str(&name.value);
        }
        SyntaxType::ConstructWithArguments {
            underscore_start,
            name,
            argument0,
            argument1_up,
        } => {
            formatted.push('_');
            match name {
                None => {
                    formatted.push(' ');
                }
                Some(name) => {
                    formatted.push_str(&name.value);
                }
            }
            let line_span = range_line_span(type_range(type_, types));
            let argument_count = 1 + argument1_up.len();
            match argument0 {
                None => {
                    formatted.push(' ');
                }
                Some(argument0) => {
                    let argument0 = types.element(argument0);
                    maybe_open_end_whitespace_then_element_last_always_unparenthesized_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_type_unparenthesized_format(formatted, indent, types, argument0);
                        },
                        || syntax_type_open_end(argument0, types),
                        OpenEndKind::TypeConstruct,
                        argument_count,
                        0,
                        *underscore_start,
                        type_range(argument0, types),
                    );
                }
            }
            for (argument_index, argument) in
                argument1_up.iter().enumerate().map(|(i, e)| (1 + i, e))
            {
                if let Some(argument_type) = &argument.type_ {
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, indent);
                    }
                    formatted.push(',');
                    maybe_open_end_whitespace_then_element_last_always_unparenthesized_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_type_unparenthesized_format(
                                formatted,
                                indent,
                                types,
                                argument_type,
                            );
                        },
                        || syntax_type_open_end(argument_type, types),
                        OpenEndKind::TypeConstruct,
                        argument_count,
                        argument_index,
                        argument.comma_start,
                        type_range(argument_type, types),
                    );
                }
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
        SyntaxType::RecordEmpty { dot_start: _ } => {
            formatted.push('.');
        }
        SyntaxType::Record {
            field0_name,
            field0_value,
            field1_up,
        } => {
            field_name_format(formatted, &field0_name.value);
            let field_count = 1 + field1_up.len();
            match field0_value {
                None => {
                    formatted.push(' ');
                }
                Some(value) => {
                    let value = types.element(value);
                    maybe_open_end_whitespace_then_element_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_type_unparenthesized_format(formatted, indent, types, value);
                        },
                        || syntax_type_open_end(value, types),
                        OpenEndKind::Record,
                        field_count,
                        0,
                        field0_name.start,
                        type_range(value, types),
                    );
                }
            }
            let line_span = range_line_span(type_range(type_, types));
            for (field_index, field) in field1_up.iter().enumerate().map(|(i, e)| (1 + i, e)) {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                optional_field_name_format(formatted, field.name.value.as_ref());
                match &field.value {
                    None => {
                        formatted.push(' ');
                    }
                    Some(value) => {
                        maybe_open_end_whitespace_then_element_format(
                            formatted,
                            indent,
                            |formatted, indent| {
                                syntax_type_unparenthesized_format(formatted, indent, types, value);
                            },
                            || syntax_type_open_end(value, types),
                            OpenEndKind::Record,
                            field_count,
                            field_index,
                            field.name.start,
                            type_range(value, types),
                        );
                    }
                }
            }
        }
        SyntaxType::ChoiceEmpty { bar_start: _ } => {
            formatted.push('|');
        }
        SyntaxType::Choice {
            variant0_name,
            variant0_value,
            variant1_up,
        } => {
            variant_name_format(formatted, &variant0_name.value);
            let variant_count = 1 + variant1_up.len();
            match variant0_value {
                None => {
                    formatted.push(' ');
                }
                Some(value) => {
                    let value = types.element(value);
                    maybe_open_end_whitespace_then_element_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_type_unparenthesized_format(formatted, indent, types, value);
                        },
                        || syntax_type_open_end(value, types),
                        OpenEndKind::TypeChoice,
                        variant_count,
                        0,
                        variant0_name.start,
                        type_range(value, types),
                    );
                }
            }
            let line_span = range_line_span(type_range(type_, types));
            for (variant_index, variant) in variant1_up.iter().enumerate().map(|(i, e)| (1 + i, e))
            {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                optional_variant_name_format(formatted, variant.name.value.as_ref());
                match &variant.value {
                    None => {
                        formatted.push(' ');
                    }
                    Some(value) => {
                        maybe_open_end_whitespace_then_element_format(
                            formatted,
                            indent,
                            |formatted, indent| {
                                syntax_type_unparenthesized_format(formatted, indent, types, value);
                            },
                            || syntax_type_open_end(value, types),
                            OpenEndKind::TypeChoice,
                            variant_count,
                            variant_index,
                            variant.name.start,
                            type_range(value, types),
                        );
                    }
                }
            }
        }
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
            ty_keyword_start: _,
            name,
            parameters,
            documentation: _,
            type_,
        } => {
            if let Some(name) = name
                && range_includes_position(name_range(with_start_position_as_ref(name)), position)
            {
                return Some(SyntaxSymbol::ProjectTypeOrUnknown {
                    name: with_start_position_as_ref(name),
                    origins: std::collections::HashMap::new(),
                });
            }
            parameters
                .as_ref()
                .and_then(|parameters| {
                    std::iter::once(&parameters.parameter0)
                        .chain(
                            parameters
                                .parameter1_up
                                .iter()
                                .filter_map(|parameter| parameter.name.as_ref()),
                        )
                        .find_map(|name| {
                            if range_includes_position(
                                name_range(with_start_position_as_ref(name)),
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
        SyntaxProjectElement::Fn {
            fn_keyword_start: _,
            name,
            type_parameters,
            parameter,
            arrow_start: _,
            result_type,
            angle_right_start: _,
            documentation: _,
            result,
        } => {
            if let Some(name) = name
                && range_includes_position(name_range(with_start_position_as_ref(name)), position)
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
        SyntaxExpression::Variable(name) => Some(match pattern_variables.get(&name.value) {
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
        }),
        SyntaxExpression::Call {
            underscore_start,
            name,
            type_arguments,
            argument,
        } => {
            if let Some(name) = name
                && range_includes_position(
                    lsp_types::Range {
                        start: *underscore_start,
                        end: name_end(with_start_position_as_ref(name)),
                    },
                    position,
                )
            {
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
        SyntaxExpression::Variant { name, type_, value } => {
            if range_includes_position(optional_variant_name_range(name), position)
                && let Some(name_value) = &name.value
            {
                return Some(SyntaxSymbol::VariantOrUnknown(WithStartPosition {
                    start: name.start,
                    value: name_value,
                }));
            }
            type_
                .as_ref()
                .and_then(|type_| {
                    syntax_type_symbol_at_position(type_, position, types, scope, origins)
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
            angle_right_start: _,
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
        SyntaxExpression::RecordEmpty { dot_start: _ } => None,
        SyntaxExpression::Record {
            field0_name,
            field0_value,
            field1_up,
        } => syntax_fields_find_symbol_at_position(
            with_start_position_as_ref(field0_name),
            field0_value
                .as_ref()
                .map(|field0_value| expressions.element(field0_value)),
            field1_up,
            |value| {
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
            },
        ),
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
            question_mark_start: _,
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
                if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
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
    case.pattern
        .as_ref()
        .and_then(|pattern| {
            syntax_pattern_symbol_at_position(
                pattern,
                position,
                patterns,
                types,
                scope,
                case.result.as_ref(),
                origins,
            )
        })
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
            if let Some(pattern) = &case.pattern {
                syntax_pattern_variables_fold(
                    pattern,
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
            }
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
        SyntaxPattern::Variant { name: _, value } => match value {
            None => state,
            Some(value) => {
                syntax_pattern_variables_fold(patterns.element(value), state, reduce, patterns)
            }
        },
        SyntaxPattern::RecordEmpty { dot_start: _ } => state,
        SyntaxPattern::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => field0_value
            .iter()
            .map(|field0_value| patterns.element(field0_value))
            .chain(field1_up.iter().filter_map(|field| field.value.as_ref()))
            .fold(state, |state, field_value| {
                syntax_pattern_variables_fold(field_value, state, reduce, patterns)
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
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
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
        SyntaxPattern::Variant { name, value } => {
            if range_includes_position(optional_variant_name_range(name), position)
                && let Some(name_value) = &name.value
            {
                return Some(SyntaxSymbol::VariantOrUnknown(WithStartPosition {
                    value: name_value,
                    start: name.start,
                }));
            }
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
        }
        SyntaxPattern::RecordEmpty { dot_start: _ } => None,
        SyntaxPattern::Record {
            field0_name,
            field0_value,
            field1_up,
        } => syntax_fields_find_symbol_at_position(
            with_start_position_as_ref(field0_name),
            field0_value.as_ref().map(|value| patterns.element(value)),
            field1_up,
            |value| {
                syntax_pattern_symbol_at_position(
                    value,
                    position,
                    patterns,
                    types,
                    project_element_scope,
                    expression_scope,
                    origins,
                )
            },
        ),
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
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                Some(SyntaxSymbol::TypeVariable {
                    name: &name.value,
                    use_start: name.start,
                    scope: scope,
                })
            } else {
                None
            }
        }
        SyntaxType::ConstructWithoutArguments(name) => Some(match origins.get(&name.value) {
            Some(&origin_info) => SyntaxSymbol::Origin {
                name: &name.value,
                use_start: name.start,
                origin: origin_info,
            },
            None => SyntaxSymbol::ProjectTypeOrUnknown {
                name: with_start_position_as_ref(name),
                origins: std::mem::take(origins),
            },
        }),
        SyntaxType::ConstructWithArguments {
            underscore_start,
            name,
            argument0,
            argument1_up,
        } => {
            if let Some(name) = name
                && range_includes_position(
                    lsp_types::Range {
                        start: *underscore_start,
                        end: name_end(with_start_position_as_ref(name)),
                    },
                    position,
                )
            {
                return Some(SyntaxSymbol::ProjectTypeOrUnknown {
                    name: with_start_position_as_ref(name),
                    origins: std::mem::take(origins),
                });
            }
            argument0
                .iter()
                .map(|argument0| types.element(argument0))
                .chain(
                    argument1_up
                        .iter()
                        .filter_map(|argument| argument.type_.as_ref()),
                )
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
        SyntaxType::RecordEmpty { dot_start: _ } => None,
        SyntaxType::Record {
            field0_name,
            field0_value,
            field1_up,
        } => syntax_fields_find_symbol_at_position(
            with_start_position_as_ref(field0_name),
            field0_value.as_ref().map(|value| types.element(value)),
            field1_up,
            |value| syntax_type_symbol_at_position(value, position, types, scope, origins),
        ),
        SyntaxType::ChoiceEmpty { bar_start: _ } => None,
        SyntaxType::Choice {
            variant0_name: _,
            variant0_value,
            variant1_up,
        } => variant0_value
            .iter()
            .map(|value| types.element(value))
            .chain(
                variant1_up
                    .iter()
                    .filter_map(|variant| variant.value.as_ref()),
            )
            .find_map(|value| {
                syntax_type_symbol_at_position(value, position, types, scope, origins)
            }),
    }
}
fn syntax_angled_type_parameters_symbol_at_position<'a, Expressions, Patterns, Types>(
    angled_type_parameters: &'a SyntaxAngledTypeParameters,
    position: lsp_types::Position,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    angled_type_parameters.names.iter().find_map(|name| {
        if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
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
    _field0_name: WithStartPosition<&Name>,
    field0_value: Option<&'a Value>,
    field1_up: &'a [SyntaxTrailingField<Value>],
    mut value_symbol_at_position: impl FnMut(
        &'a Value,
    )
        -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    field0_value
        .into_iter()
        .chain(field1_up.iter().filter_map(|field| field.value.as_ref()))
        .find_map(|field_value| value_symbol_at_position(field_value))
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
                ty_keyword_start: _,
                name: type_alias_name,
                parameters: _,
                documentation: _,
                type_: _,
            } => {
                if let Some(type_alias_name) = type_alias_name
                    && type_alias_name.value == symbol_name.value
                {
                    Some(name_range(with_start_position_as_ref(type_alias_name)))
                } else {
                    None
                }
            }
            SyntaxProjectElement::Fn { .. }
            | SyntaxProjectElement::Comments(_)
            | SyntaxProjectElement::Unrecognized { .. } => None,
        }),
        SyntaxSymbol::Origin {
            name,
            use_start: _,
            origin,
        } => Some(name_range(WithStartPosition {
            value: name,
            start: origin.start,
        })),
        SyntaxSymbol::TypeVariable {
            name: symbol_name,
            use_start: _,
            scope,
        } => match scope {
            SyntaxProjectElement::TypeAlias {
                ty_keyword_start: _,
                name: _,
                parameters,
                documentation: _,
                type_: _,
            } => parameters.as_ref().and_then(|parameters| {
                std::iter::once(&parameters.parameter0)
                    .chain(
                        parameters
                            .parameter1_up
                            .iter()
                            .filter_map(|parameter| parameter.name.as_ref()),
                    )
                    .find_map(|parameter| {
                        if &parameter.value == symbol_name {
                            Some(name_range(with_start_position_as_ref(parameter)))
                        } else {
                            None
                        }
                    })
            }),
            SyntaxProjectElement::Fn {
                fn_keyword_start: _,
                name: _,
                type_parameters,
                parameter: _,
                arrow_start: _,
                result_type: _,
                angle_right_start: _,
                documentation: _,
                result: _,
            } => type_parameters
                .as_ref()
                .into_iter()
                .flat_map(|type_parameters| &type_parameters.names)
                .find_map(|parameter| {
                    if &parameter.value == symbol_name {
                        Some(name_range(with_start_position_as_ref(parameter)))
                    } else {
                        None
                    }
                }),
            SyntaxProjectElement::Comments(_) | SyntaxProjectElement::Unrecognized { .. } => None,
        },
        SyntaxSymbol::VariantOrUnknown(_) => None,
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
                arrow_start: _,
                result_type: _,
                angle_right_start: _,
                documentation: _,
                result: _,
            } if fn_name.value == symbol_name.value => {
                Some(name_range(with_start_position_as_ref(fn_name)))
            }
            _ => None,
        }),
        SyntaxSymbol::PatternVariable {
            name,
            use_start: _,
            origin,
        } => Some(name_range(WithStartPosition {
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
            name: _,
            use_start: _,
            scope,
        } => match scope {
            SyntaxProjectElement::TypeAlias {
                ty_keyword_start: _,
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
            SyntaxProjectElement::Fn {
                fn_keyword_start: _,
                name: _,
                type_parameters: _,
                parameter,
                arrow_start: _,
                result_type,
                angle_right_start: _,
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
                        arrow_start: _,
                        result_type,
                        angle_right_start: _,
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
                        ty_keyword_start: _,
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
                        arrow_start: _,
                        parameter,
                        result_type,
                        angle_right_start: _,
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
                uses.push(name_range(with_start_position_as_ref(name)));
            }
        }
        SyntaxType::RecordEmpty { dot_start: _ } => {}
        SyntaxType::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            for field_value in field0_value
                .iter()
                .map(|value| types.element(value))
                .chain(field1_up.iter().filter_map(|field| field.value.as_ref()))
            {
                syntax_type_symbol_uses_into(uses, field_value, symbol, types, origins);
            }
        }
        SyntaxType::ChoiceEmpty { bar_start: _ } => {}
        SyntaxType::Choice {
            variant0_name: _,
            variant0_value,
            variant1_up,
        } => {
            for variant_value in variant0_value
                .iter()
                .map(|value| types.element(value))
                .chain(
                    variant1_up
                        .iter()
                        .filter_map(|variant| variant.value.as_ref()),
                )
            {
                syntax_type_symbol_uses_into(uses, variant_value, symbol, types, origins);
            }
        }
        SyntaxType::ConstructWithoutArguments(name) => {
            if let SyntaxSymbol::ProjectTypeOrUnknown {
                name: symbol_name,
                origins: _,
            } = symbol
                && name.value == symbol_name.value
                && !origins.contains(&name.value)
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
        }
        SyntaxType::ConstructWithArguments {
            underscore_start: _,
            name,
            argument0,
            argument1_up,
        } => {
            if let Some(name) = name
                && let SyntaxSymbol::ProjectTypeOrUnknown {
                    name: symbol_name,
                    origins: _,
                } = symbol
                && name.value == symbol_name.value
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
            for argument in argument0
                .iter()
                .map(|argument0| types.element(argument0))
                .chain(
                    argument1_up
                        .iter()
                        .filter_map(|argument| argument.type_.as_ref()),
                )
            {
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
        SyntaxPattern::Variant { name, value } => {
            if let Some(name_value) = &name.value
                && let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && name_value == symbol_name.value
            {
                uses.push(optional_variant_name_range(name));
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
        SyntaxPattern::RecordEmpty { dot_start: _ } => {}
        SyntaxPattern::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            if let Some(field0_value) = field0_value {
                syntax_pattern_symbol_uses_into(
                    uses,
                    patterns.element(field0_value),
                    symbol,
                    patterns,
                    types,
                    origins,
                );
            }
            for field in field1_up {
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
        SyntaxExpression::Variable(name) => match symbol {
            SyntaxSymbol::TypeVariable { .. }
            | SyntaxSymbol::ProjectTypeOrUnknown { .. }
            | SyntaxSymbol::VariantOrUnknown(_) => {}
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
                    uses.push(name_range(with_start_position_as_ref(name)));
                }
            }
        },
        SyntaxExpression::Call {
            underscore_start: _,
            name,
            type_arguments,
            argument,
        } => {
            if let Some(name) = name {
                match symbol {
                    SyntaxSymbol::TypeVariable { .. }
                    | SyntaxSymbol::Origin { .. }
                    | SyntaxSymbol::ProjectTypeOrUnknown { .. }
                    | SyntaxSymbol::VariantOrUnknown(_) => {}
                    SyntaxSymbol::PatternVariable {
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
                            uses.push(name_range(with_start_position_as_ref(name)));
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
        SyntaxExpression::Variant { name, type_, value } => {
            if let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && let Some(name_value) = &name.value
                && name_value == symbol_name.value
            {
                uses.push(optional_variant_name_range(name));
            }
            if let Some(type_) = type_ {
                syntax_type_symbol_uses_into(uses, type_, symbol, types, origins);
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
            angle_right_start: _,
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
        SyntaxExpression::RecordEmpty { dot_start: _ } => {}
        SyntaxExpression::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            if let Some(field0_value) = field0_value {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(field0_value),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
            }
            for field in field1_up {
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
            question_mark_start: _,
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
                    if let Some(pattern) = &case.pattern {
                        syntax_pattern_symbol_uses_into(
                            uses, pattern, symbol, patterns, types, origins,
                        );
                        syntax_pattern_variables_fold(
                            pattern,
                            (),
                            &mut |(), pattern_variable_name, _type_| {
                                pattern_variables
                                    .to_mut()
                                    .insert(&pattern_variable_name.value);
                            },
                            patterns,
                        );
                    }
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
