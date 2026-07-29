#![allow(non_upper_case_globals)]

use gen_lsp_types as lsp_types;
pub mod core;

pub type Name = kstring::KString;
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
#[allow(
    clippy::large_enum_variant,
    reason = "::Fn is the largest but almost all variants in practice are ::Fn anyway"
)]
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
    pub parameter1_up: Vec<SyntaxTrailingTypeParameter>,
}
#[derive(Clone, Debug)]
pub struct SyntaxAngledTypeParameters {
    pub open_angle_start: lsp_types::Position,
    pub parameter0: Option<WithStartPosition<Name>>,
    pub parameter1_up: Vec<SyntaxTrailingTypeParameter>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Clone, Debug)]
pub struct SyntaxTrailingTypeParameter {
    pub comma_start: lsp_types::Position,
    pub name: Option<WithStartPosition<Name>>,
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
        part0: SyntaxRecordPart<Patterns>,
        // possible optimization: use SyntaxPattern directly, not slot
        part1_up: Vec<SyntaxRecordPart<Patterns>>,
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
pub struct SyntaxAngledTypeArgument<Types> {
    pub open_angle_start: lsp_types::Position,
    pub type_: Option<SyntaxType<Types>>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub struct SyntaxAngledTypeArguments<Types> {
    pub open_angle_start: lsp_types::Position,
    pub argument0: Option<SyntaxType<Types>>,
    pub argument1_up: Vec<SyntaxTrailingTypeArgument<Types>>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub struct SyntaxTrailingTypeArgument<Types> {
    pub comma_start: lsp_types::Position,
    pub type_: Option<SyntaxType<Types>>,
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
        type_: Option<SyntaxAngledTypeArgument<Types>>,
        value: Option<core::Slot<Expressions>>,
    },
    Fn {
        open_bracket_start: lsp_types::Position,
        parameter: Option<SyntaxPattern<Patterns, Types>>,
        closed_bracket_start: Option<lsp_types::Position>,
        result: Option<core::Slot<Expressions>>,
    },
    RecordEmpty {
        dot_start: lsp_types::Position,
    },
    Record {
        part0: SyntaxRecordPart<Expressions>,
        // possible optimization: use SyntaxExpression directly, not slot
        part1_up: Vec<SyntaxRecordPart<Expressions>>,
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
    pub open_bracket_start: lsp_types::Position,
    pub pattern: Option<SyntaxPattern<Patterns, Types>>,
    pub closed_bracket_start: Option<lsp_types::Position>,
    pub result: Option<SyntaxExpression<Expressions, Patterns, Types>>,
}
#[derive(Debug)]
pub enum SyntaxRecordPart<Sub> {
    Field {
        name: WithStartPosition<Option<Name>>,
        value: Option<core::Slot<Sub>>,
    },
    Spread {
        dot_dot_start: lsp_types::Position,
        record: Option<core::Slot<Sub>>,
    },
}

#[must_use]
pub fn name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, name.value.len() as u32)
}
#[must_use]
pub fn name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: name_end(name),
    }
}
#[must_use]
pub fn variant_name_length(variant_name: &Name) -> usize {
    1 + variant_name.len()
}
#[must_use]
pub fn variant_name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, variant_name_length(name.value) as u32)
}
#[must_use]
pub fn variant_name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: variant_name_end(name),
    }
}
#[must_use]
pub fn optional_variant_name_length(variant_name: Option<&Name>) -> usize {
    match variant_name {
        None => 1,
        Some(name) => variant_name_length(name),
    }
}
#[must_use]
pub fn optional_variant_name_end(
    variant_name: &WithStartPosition<Option<Name>>,
) -> lsp_types::Position {
    position_add_characters(
        variant_name.start,
        optional_variant_name_length(variant_name.value.as_ref()) as u32,
    )
}
#[must_use]
pub fn optional_variant_name_range(
    variant_name: &WithStartPosition<Option<Name>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: variant_name.start,
        end: optional_variant_name_end(variant_name),
    }
}
#[must_use]
pub fn field_name_length(field_name: &Name) -> usize {
    1 + field_name.len()
}
#[must_use]
pub fn field_name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, field_name_length(name.value) as u32)
}
#[must_use]
pub fn field_name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: field_name_end(name),
    }
}
#[must_use]
pub fn optional_field_name_length(field_name: Option<&Name>) -> usize {
    match field_name {
        None => 1,
        Some(name) => field_name_length(name),
    }
}
#[must_use]
pub fn optional_field_name_end(
    field_name: &WithStartPosition<Option<Name>>,
) -> lsp_types::Position {
    position_add_characters(
        field_name.start,
        optional_field_name_length(field_name.value.as_ref()) as u32,
    )
}
#[must_use]
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
#[must_use]
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
#[must_use]
pub fn pattern_start<Patterns, Types>(
    pattern: &SyntaxPattern<Patterns, Types>,
) -> lsp_types::Position {
    match pattern {
        SyntaxPattern::Variable { name, type_: _ } => name.start,
        SyntaxPattern::Variant { name, value: _ } => name.start,
        SyntaxPattern::RecordEmpty { dot_start } => *dot_start,
        SyntaxPattern::Record { part0, part1_up: _ } => match part0 {
            SyntaxRecordPart::Field { name, value: _ } => name.start,
            SyntaxRecordPart::Spread {
                dot_dot_start,
                record: _,
            } => *dot_dot_start,
        },
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
        SyntaxPattern::Record { part0, part1_up } => match part1_up.last().unwrap_or(part0) {
            SyntaxRecordPart::Field { name, value } => value
                .as_ref()
                .map(|value| pattern_end(patterns.element(value), patterns, types))
                .unwrap_or_else(|| optional_field_name_end(name)),
            SyntaxRecordPart::Spread {
                dot_dot_start,
                record,
            } => record
                .as_ref()
                .map(|record| pattern_end(patterns.element(record), patterns, types))
                .unwrap_or_else(|| symbol_end(*dot_dot_start, "..")),
        },
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
pub fn angled_type_argument_end<Types>(
    type_arguments: &SyntaxAngledTypeArgument<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    type_arguments
        .closed_angle_start
        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
        .or_else(|| {
            type_arguments
                .type_
                .as_ref()
                .map(|last_type| type_end(last_type, types))
        })
        .unwrap_or_else(|| symbol_end(type_arguments.open_angle_start, "<"))
}
pub fn angled_type_arguments_end<Types>(
    type_arguments: &SyntaxAngledTypeArguments<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    type_arguments
        .closed_angle_start
        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
        .or_else(|| {
            type_arguments
                .argument1_up
                .last()
                .map(|argument| argument.type_.as_ref())
                .unwrap_or(type_arguments.argument0.as_ref())
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
#[must_use]
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
            open_bracket_start,
            parameter: _,
            closed_bracket_start: _,
            result: _,
        } => *open_bracket_start,
        SyntaxExpression::RecordEmpty { dot_start } => *dot_start,
        SyntaxExpression::Record { part0, part1_up: _ } => expression_record_part_start(part0),
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
fn expression_record_part_start<Expressions>(
    part: &SyntaxRecordPart<Expressions>,
) -> lsp_types::Position {
    match part {
        SyntaxRecordPart::Field { name, value: _ } => name.start,
        SyntaxRecordPart::Spread {
            dot_dot_start,
            record: _,
        } => *dot_dot_start,
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
            .or_else(|| {
                type_
                    .as_ref()
                    .map(|type_| angled_type_argument_end(type_, types))
            })
            .unwrap_or_else(|| optional_variant_name_end(name)),
        SyntaxExpression::Fn {
            open_bracket_start,
            parameter,
            closed_bracket_start,
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
                closed_bracket_start
                    .map(|closed_bracket_start| symbol_end(closed_bracket_start, "]"))
            })
            .or_else(|| {
                parameter
                    .as_ref()
                    .map(|parameter| pattern_end(parameter, patterns, types))
            })
            .unwrap_or_else(|| symbol_end(*open_bracket_start, "[")),
        SyntaxExpression::RecordEmpty { dot_start } => symbol_end(*dot_start, "."),
        SyntaxExpression::Record { part0, part1_up } => expression_record_part_end(
            part1_up.last().unwrap_or(part0),
            expressions,
            patterns,
            types,
        ),
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
pub fn expression_record_part_end<Expressions, Patterns, Types>(
    part: &SyntaxRecordPart<Expressions>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> lsp_types::Position {
    match part {
        SyntaxRecordPart::Field { name, value } => value
            .as_ref()
            .map(|value| expression_end(expressions.element(value), expressions, patterns, types))
            .unwrap_or_else(|| optional_field_name_end(name)),
        SyntaxRecordPart::Spread {
            dot_dot_start,
            record,
        } => record
            .as_ref()
            .map(|value| expression_end(expressions.element(value), expressions, patterns, types))
            .unwrap_or_else(|| symbol_end(*dot_dot_start, "..")),
    }
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
            case.closed_bracket_start
                .map(|closed_bracket_start| symbol_end(closed_bracket_start, "]"))
        })
        .or_else(|| {
            case.pattern
                .as_ref()
                .map(|pattern| pattern_end(pattern, patterns, types))
        })
        .unwrap_or_else(|| symbol_end(case.open_bracket_start, "["))
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
        Some(Name::from_ref(parsed_str))
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
        Some(Name::from_ref(parsed_str))
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
    // entirely unscientific lower estimates c:
    let mut elements = Vec::with_capacity(project_source.len() / 24);
    expressions.pre_allocate_at_least_usize(project_source.len() / 4);
    patterns.pre_allocate_at_least_usize(project_source.len() / 6);
    types.pre_allocate_at_least_usize(project_source.len() / 5);

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
        parameter1_up.push(SyntaxTrailingTypeParameter {
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
    let parameter0 = parse_sloe_uppercase_name_with_start(state);
    parse_sloe_whitespace(state);
    let mut parameter1_up = Vec::new();
    while let Some(comma_start) = parse_symbol_as_start(state, ",") {
        parse_sloe_whitespace(state);
        let name = parse_sloe_uppercase_name_with_start(state);
        parameter1_up.push(SyntaxTrailingTypeParameter {
            comma_start: comma_start,
            name: name,
        });
        parse_sloe_whitespace(state);
    }
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeParameters {
        open_angle_start: open_angle_start,
        parameter0: parameter0,
        parameter1_up: parameter1_up,
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
        inner: inner.map(|inner| patterns.insert(inner)),
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
        inner: inner.map(|inner| patterns.insert(inner)),
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
        value: value.map(|value| patterns.insert(value)),
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
        value: value.map(|value| patterns.insert(value)),
    })
}
fn parse_type_argument<Types>(
    state: &mut ParseState,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxAngledTypeArgument<Types>> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let type_ = parse_type(state, types);
    parse_sloe_whitespace(state);
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeArgument {
        open_angle_start: open_angle_start,
        type_: type_,
        closed_angle_start: closed_angle_start,
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
    let argument0 = parse_type(state, types);
    parse_sloe_whitespace(state);
    let mut argument1_up = Vec::new();
    while let Some(comma_start) = parse_symbol_as_start(state, ",") {
        parse_sloe_whitespace(state);
        let type_ = parse_type(state, types);
        argument1_up.push(SyntaxTrailingTypeArgument {
            comma_start: comma_start,
            type_: type_,
        });
        parse_sloe_whitespace(state);
    }
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeArguments {
        open_angle_start: open_angle_start,
        argument0: argument0,
        argument1_up: argument1_up,
        closed_angle_start: closed_angle_start,
    })
}
fn parse_pattern_record_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxPattern<Patterns, Types>> {
    let part0 = if let Some(dot_dot_start) = parse_symbol_as_start(state, "..") {
        parse_sloe_whitespace(state);
        let record = parse_pattern_typed(state, patterns, types);
        SyntaxRecordPart::Spread {
            dot_dot_start: dot_dot_start,
            record: record.map(|record| patterns.insert(record)),
        }
    } else if let Some(name) = parse_field_name(state) {
        let Some(name_value) = name.value else {
            return Some(SyntaxPattern::RecordEmpty {
                dot_start: name.start,
            });
        };
        parse_sloe_whitespace(state);
        let value = parse_pattern_typed(state, patterns, types);
        SyntaxRecordPart::Field {
            name: WithStartPosition {
                value: Some(name_value),
                start: name.start,
            },
            value: value.map(|record| patterns.insert(record)),
        }
    } else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut part1_up = Vec::new();
    while let Some(field) = parse_pattern_record_part_typed(state, patterns, types) {
        part1_up.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxPattern::Record {
        part0: part0,
        part1_up: part1_up,
    })
}
fn parse_pattern_record_part_typed<Patterns, Types>(
    state: &mut ParseState,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxRecordPart<Patterns>> {
    if let Some(dot_dot_start) = parse_symbol_as_start(state, "..") {
        parse_sloe_whitespace(state);
        let record = parse_pattern_typed(state, patterns, types);
        Some(SyntaxRecordPart::Spread {
            dot_dot_start: dot_dot_start,
            record: record.map(|record| patterns.insert(record)),
        })
    } else if let Some(name) = parse_field_name(state) {
        parse_sloe_whitespace(state);
        let value = parse_pattern_typed(state, patterns, types);
        Some(SyntaxRecordPart::Field {
            name: name,
            value: value.map(|record| patterns.insert(record)),
        })
    } else {
        return None;
    }
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
        field1_up.push(SyntaxRecordPart::Field {
            name: field.name,
            value: field.value.map(|field_value| patterns.insert(field_value)),
        });
        parse_sloe_whitespace(state);
    }
    Some(SyntaxPattern::Record {
        part0: SyntaxRecordPart::Field {
            name: WithStartPosition {
                start: field0_name.start,
                value: Some(field0_name_value),
            },
            value: field0_value.map(|field0_value| patterns.insert(field0_value)),
        },
        part1_up: field1_up,
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
        inner: inner.map(|inner| types.insert(inner)),
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
        field0_value: field0_value.map(|field0_value| types.insert(field0_value)),
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
        variant0_value: variant0_value.map(|value| types.insert(value)),
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
        argument0: argument0.map(|argument0| types.insert(argument0)),
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
    let part0 = if let Some(dot_dot_start) = parse_symbol_as_start(state, "..") {
        parse_sloe_whitespace(state);
        let record = parse_expression(state, expressions, patterns, types);
        SyntaxRecordPart::Spread {
            dot_dot_start: dot_dot_start,
            record: record.map(|record| expressions.insert(record)),
        }
    } else if let Some(name) = parse_field_name(state) {
        // there are most likely more elegant ways of parsing and representing this.
        // best is probs make parse_field_name lookahead (so . itself doesn't parse).
        // This would also avoid some of the weird WithStart<Option<Name>> stuff
        let Some(name_value) = name.value else {
            return Some(SyntaxExpression::RecordEmpty {
                dot_start: name.start,
            });
        };
        parse_sloe_whitespace(state);
        let value = parse_expression(state, expressions, patterns, types);
        SyntaxRecordPart::Field {
            name: WithStartPosition {
                value: Some(name_value),
                start: name.start,
            },
            value: value.map(|record| expressions.insert(record)),
        }
    } else {
        return None;
    };
    parse_sloe_whitespace(state);
    let mut part1_up = Vec::new();
    while let Some(field) = parse_expression_record_part(state, expressions, patterns, types) {
        part1_up.push(field);
        parse_sloe_whitespace(state);
    }
    Some(SyntaxExpression::Record {
        part0: part0,
        part1_up,
    })
}
fn parse_expression_record_part<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxRecordPart<Expressions>> {
    if let Some(dot_dot_start) = parse_symbol_as_start(state, "..") {
        parse_sloe_whitespace(state);
        let record = parse_expression(state, expressions, patterns, types);
        Some(SyntaxRecordPart::Spread {
            dot_dot_start: dot_dot_start,
            record: record.map(|record| expressions.insert(record)),
        })
    } else if let Some(name) = parse_field_name(state) {
        parse_sloe_whitespace(state);
        let value = parse_expression(state, expressions, patterns, types);
        Some(SyntaxRecordPart::Field {
            name: name,
            value: value.map(|record| expressions.insert(record)),
        })
    } else {
        None
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
                    None
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
        inner: inner.map(|inner| expressions.insert(inner)),
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
        expression: expression.map(|expression| expressions.insert(expression)),
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
        argument: argument.map(|argument| expressions.insert(argument)),
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
    let type_argument = parse_type_argument(state, types);
    parse_sloe_whitespace(state);
    let value = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Variant {
        name: name,
        type_: type_argument,
        value: value.map(|argument| expressions.insert(argument)),
    })
}
fn parse_expression_fn<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpression<Expressions, Patterns, Types>> {
    let Some(open_bracket_start) = parse_symbol_as_start(state, "[") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let parameter = parse_pattern_typed(state, patterns, types);
    parse_sloe_whitespace(state);
    let closed_bracket_start = parse_symbol_as_start(state, "]");
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpression::Fn {
        open_bracket_start,
        parameter: parameter,
        closed_bracket_start,
        result: result.map(|result| expressions.insert(result)),
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
        result: result.map(|result| expressions.insert(result)),
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
        queried: queried.map(|queried| expressions.insert(queried)),
        cases: cases,
    })
}
fn parse_expression_query_case<Expressions, Patterns, Types>(
    state: &mut ParseState,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) -> Option<SyntaxExpressionQueryCase<Expressions, Patterns, Types>> {
    let Some(open_bracket_start) = parse_symbol_as_start(state, "[") else {
        return None;
    };
    parse_sloe_whitespace(state);
    let pattern = parse_pattern_untyped(state, patterns, types);
    parse_sloe_whitespace(state);
    let closed_bracket_start = parse_symbol_as_start(state, "]");
    parse_sloe_whitespace(state);
    let result = parse_expression(state, expressions, patterns, types);
    Some(SyntaxExpressionQueryCase {
        open_bracket_start: open_bracket_start,
        pattern: pattern,
        closed_bracket_start,
        result: result,
    })
}

pub struct CompiledProject {
    pub rust: syn::File,
    pub type_aliases: std::collections::HashMap<Name, CheckedTypeAlias>,
    pub fns: std::collections::HashMap<Name, CheckedProjectFn>,
    pub records: std::collections::HashSet<Vec<Name>>,
    pub queries: std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    pub spread_records: std::collections::HashMap<lsp_types::Position, Vec<Name>>,
}
#[derive(Clone, Debug)]
pub struct CheckedTypeAlias {
    pub name_range: Option<lsp_types::Range>,
    pub parameters: Vec<Name>,
    pub documentation: Option<Box<str>>,
    pub type_: Option<Type>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Variable(Name),
    Origin(Name),
    Record(Vec<TypeField>),
    Choice(Vec<TypeVariant>),
    CoreConstruct { name: Name, arguments: Vec<Type> },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeField {
    pub name: Name,
    pub value: Type,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVariant {
    pub name: Name,
    pub value: Type,
}
#[derive(Clone, Debug)]
pub struct CheckedProjectFn {
    pub documentation: Option<Box<str>>,
    pub type_parameters: Vec<Name>,
    pub parameter_type: Option<Type>,
    pub result_type: Option<Type>,
    pub result_expression_is_invalid: bool,
}
// TODO inline use sites
pub fn syntax_project_to_rust<Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    syntax_project: &SyntaxProject<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> CompiledProject {
    let project_info = syntax_project_check(errors, syntax_project, expressions, patterns, types);
    checked_project_to_rust(project_info, expressions, patterns, types)
}
pub fn syntax_project_check<'a, Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    syntax_project: &'a SyntaxProject<Expressions, Patterns, Types>,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
) -> CheckedSyntaxProject<'a, Expressions, Patterns, Types> {
    let mut type_graph: strongly_connected_components::Graph =
        strongly_connected_components::Graph::new();
    let mut type_graph_node_by_name: std::collections::HashMap<
        &str,
        strongly_connected_components::Node,
    > = std::collections::HashMap::new();
    let mut project_type_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectTypeInfo<Types>,
    > = std::collections::HashMap::new();

    let mut project_fn_graph: strongly_connected_components::Graph =
        strongly_connected_components::Graph::new();
    let mut project_fn_graph_node_by_name: std::collections::HashMap<
        &Name,
        strongly_connected_components::Node,
    > = std::collections::HashMap::with_capacity(syntax_project.elements.len());
    let mut project_fn_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectFnInfo<Expressions, Patterns, Types>,
    > = std::collections::HashMap::with_capacity(syntax_project.elements.len());

    let mut records_used: std::collections::HashSet<Vec<Name>> =
        std::collections::HashSet::with_capacity(16);
    let mut choices_used: std::collections::HashSet<Vec<Name>> =
        std::collections::HashSet::with_capacity(4);

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
If you wanted to start a project declaration, try one of:
  - fn some-fn-name some-parameter some-parameter-type :> some result type > some result value
  - ty some-type-name some type",
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
                        "Comments can only be put in front of expressions, after the header of a project fn or ty or between these project elements. Is it indented correctly?"
                    } else if unknown_source.starts_with("//")
                        || unknown_source.starts_with("--")
                    {
                        "Comments start with #"
                    } else   if unknown_source
                        .starts_with('.')
                    {
                        "Record access is not a feature in sloe. Instead, use pattern matching, like value ? your-value [.field variable ..other fields..] result. Otherwise, is everything indented correctly?"
                    } else if unknown_source
                        .starts_with(['+', '-', '*', '^', '/', '!', '&'])
                    {
                        "Operator application are not a feature in sloe. Instead, use regular function calls like f32-add, int-negate or unt-mul. Otherwise, is everything indented correctly?"
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
                    project_type_by_graph_node.insert(
                        type_alias_declaration_graph_node,
                        SyntaxProjectTypeInfo {
                            documentation: documentation,
                            name: name_node,
                            parameters: parameters,
                            type_: type_,
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
                        project_fn_graph.new_node();
                    let existing_variable_with_same_name: Option<
                        strongly_connected_components::Node,
                    > = project_fn_graph_node_by_name.insert(&name.value, project_fn_graph_node);
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
    for (&type_declaration_graph_node, &type_declaration_info) in project_type_by_graph_node.iter()
    {
        syntax_project_type_connect_type_names_in_graph_from(
            type_declaration_graph_node,
            &type_graph_node_by_name,
            types,
            type_declaration_info,
            &mut type_graph,
        );
    }
    for (&project_fn_graph_node, project_fn_info) in project_fn_by_graph_node.iter() {
        syntax_project_fn_connect_type_names_in_graph_from(
            project_fn_graph_node,
            &project_fn_graph_node_by_name,
            expressions,
            patterns,
            types,
            project_fn_info,
            &mut project_fn_graph,
        );
    }

    let mut checked_type_aliases: std::collections::HashMap<Name, CheckedTypeAlias> =
        core_type_aliases.clone();
    checked_type_aliases.reserve(project_type_by_graph_node.len());
    for project_type_strongly_connected_component in type_graph.find_sccs().iter_sccs() {
        // TODO report and skip (mutually) recursive project types. Currently these are reported as "not found" at best
        for project_type in project_type_strongly_connected_component
            .iter_nodes()
            .filter_map(|variable_declaration_graph_node| {
                project_type_by_graph_node.get(&variable_declaration_graph_node)
            })
            .copied()
        {
            checked_type_aliases.insert(
                project_type.name.value.clone(),
                project_type_alias_check(
                    errors,
                    &checked_type_aliases,
                    types,
                    project_type,
                    &mut records_used,
                    &mut choices_used,
                ),
            );
        }
    }

    let mut checked_project_fns: std::collections::HashMap<Name, CheckedProjectFn> =
        core_fns.clone();
    checked_project_fns.reserve(project_fn_graph.len());
    let mut checked_local_fns: std::collections::HashMap<lsp_types::Position, CheckedLocalFn> =
        std::collections::HashMap::new();
    let mut checked_queries: std::collections::HashMap<lsp_types::Position, CheckedQuery> =
        std::collections::HashMap::new();
    let mut checked_spread_records: std::collections::HashMap<lsp_types::Position, Vec<Name>> =
        std::collections::HashMap::new();
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
        // possible optimization: skip pre-compile-type-info computation when project_fns_in_strongly_connected_component is single, non-self-referencing node
        for project_fn in project_fns_in_strongly_connected_component.iter().copied() {
            checked_project_fns.insert(
                project_fn.name.value.clone(),
                syntax_project_fn_header_check(
                    errors,
                    &checked_type_aliases,
                    patterns,
                    types,
                    project_fn,
                    &mut records_used,
                    &mut choices_used,
                    &mut checked_spread_records,
                ),
            );
        }
        for project_fn in project_fns_in_strongly_connected_component {
            checked_project_fns.insert(
                project_fn.name.value.clone(),
                syntax_project_fn_check(
                    errors,
                    &checked_type_aliases,
                    &checked_project_fns,
                    expressions,
                    patterns,
                    types,
                    project_fn,
                    &mut checked_local_fns,
                    &mut checked_queries,
                    &mut checked_spread_records,
                    &mut records_used,
                    &mut choices_used,
                ),
            );
        }
    }
    CheckedSyntaxProject {
        type_graph: type_graph,
        project_type_by_graph_node: project_type_by_graph_node,
        project_fn_graph: project_fn_graph,
        project_fn_graph_node_by_name: project_fn_graph_node_by_name,
        project_fn_by_graph_node: project_fn_by_graph_node,
        records_used: records_used,
        choices_used: choices_used,
        checked_type_aliases: checked_type_aliases,
        checked_project_fns: checked_project_fns,
        checked_local_fns: checked_local_fns,
        checked_queries: checked_queries,
        checked_spread_records: checked_spread_records,
    }
}
pub struct CheckedSyntaxProject<'a, Expressions, Patterns, Types> {
    pub type_graph: strongly_connected_components::Graph,
    pub project_type_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectTypeInfo<'a, Types>,
    >,
    pub project_fn_graph: strongly_connected_components::Graph,
    pub project_fn_graph_node_by_name:
        std::collections::HashMap<&'a Name, strongly_connected_components::Node>,
    pub project_fn_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Node,
        SyntaxProjectFnInfo<'a, Expressions, Patterns, Types>,
    >,
    pub records_used: std::collections::HashSet<Vec<Name>>,
    pub choices_used: std::collections::HashSet<Vec<Name>>,
    pub checked_type_aliases: std::collections::HashMap<Name, CheckedTypeAlias>,
    pub checked_project_fns: std::collections::HashMap<Name, CheckedProjectFn>,
    pub checked_local_fns: std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    pub checked_queries: std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    pub checked_spread_records: std::collections::HashMap<lsp_types::Position, Vec<Name>>,
}
fn syntax_project_type_connect_type_names_in_graph_from<Types>(
    origin_project_type_graph_node: strongly_connected_components::Node,
    type_graph_node_by_name: &std::collections::HashMap<&str, strongly_connected_components::Node>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    project_type_info: SyntaxProjectTypeInfo<Types>,
    type_graph: &mut strongly_connected_components::Graph,
) {
    if let Some(aliased_type) = &project_type_info.type_ {
        syntax_type_connect_type_names_in_graph_from(
            origin_project_type_graph_node,
            type_graph_node_by_name,
            types,
            aliased_type,
            type_graph,
        );
    }
}
fn syntax_project_fn_connect_type_names_in_graph_from<Expressions, Patterns, Types>(
    project_fn_graph_node: strongly_connected_components::Node,
    project_fn_graph_node_by_name: &std::collections::HashMap<
        &Name,
        strongly_connected_components::Node,
    >,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    project_fn: &SyntaxProjectFnInfo<'_, Expressions, Patterns, Types>,
    project_fn_graph: &mut strongly_connected_components::Graph,
) {
    if let Some(result_node) = project_fn.result {
        syntax_expression_connect_variables_in_graph_from(
            project_fn_graph_node,
            project_fn_graph_node_by_name,
            expressions,
            patterns,
            types,
            result_node,
            project_fn_graph,
        );
    }
}
fn syntax_type_connect_type_names_in_graph_from<Types>(
    origin_type_declaration_graph_node: strongly_connected_components::Node,
    type_graph_node_by_name: &std::collections::HashMap<&str, strongly_connected_components::Node>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    type_: &SyntaxType<Types>,
    type_graph: &mut strongly_connected_components::Graph,
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
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    argument,
                    type_graph,
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
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    types.element(inner),
                    type_graph,
                );
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
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    types.element(field0_value),
                    type_graph,
                );
            }
            for field in field1_up {
                if let Some(value) = &field.value {
                    syntax_type_connect_type_names_in_graph_from(
                        origin_type_declaration_graph_node,
                        type_graph_node_by_name,
                        types,
                        value,
                        type_graph,
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
                    origin_type_declaration_graph_node,
                    type_graph_node_by_name,
                    types,
                    types.element(variant0_value),
                    type_graph,
                );
            }
            for variant in variant1_up {
                if let Some(value) = &variant.value {
                    syntax_type_connect_type_names_in_graph_from(
                        origin_type_declaration_graph_node,
                        type_graph_node_by_name,
                        types,
                        value,
                        type_graph,
                    );
                }
            }
        }
    }
}
// TODO check if currently pattern and origin variables can shadow project names.
// If yes, track pattern variables and origins to avoid accidental misconnection
fn syntax_expression_connect_variables_in_graph_from<Expressions, Patterns, Types>(
    origin_project_fn_graph_node: strongly_connected_components::Node,
    project_fn_graph_node_by_name: &std::collections::HashMap<
        &Name,
        strongly_connected_components::Node,
    >,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
    project_fn_graph: &mut strongly_connected_components::Graph,
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
                && let Some(referenced_fn_graph_node) =
                    project_fn_graph_node_by_name.get(&name.value).copied()
            {
                project_fn_graph.new_edge(origin_project_fn_graph_node, referenced_fn_graph_node);
            }
            if let Some(argument) = argument {
                syntax_expression_connect_variables_in_graph_from(
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    patterns,
                    types,
                    expressions.element(argument),
                    project_fn_graph,
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
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    patterns,
                    types,
                    expressions.element(value),
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Fn {
            open_bracket_start: _,
            parameter: _,
            closed_bracket_start: _,
            result,
        } => {
            if let Some(result) = result {
                syntax_expression_connect_variables_in_graph_from(
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    patterns,
                    types,
                    expressions.element(result),
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::RecordEmpty { dot_start: _ } => {}
        SyntaxExpression::Record { part0, part1_up } => {
            for part in std::iter::once(part0).chain(part1_up.iter()) {
                match part {
                    SyntaxRecordPart::Field { name: _, value } => {
                        if let Some(value) = value {
                            syntax_expression_connect_variables_in_graph_from(
                                origin_project_fn_graph_node,
                                project_fn_graph_node_by_name,
                                expressions,
                                patterns,
                                types,
                                expressions.element(value),
                                project_fn_graph,
                            );
                        }
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start: _,
                        record,
                    } => {
                        if let Some(record) = record {
                            syntax_expression_connect_variables_in_graph_from(
                                origin_project_fn_graph_node,
                                project_fn_graph_node_by_name,
                                expressions,
                                patterns,
                                types,
                                expressions.element(record),
                                project_fn_graph,
                            );
                        }
                    }
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
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    patterns,
                    types,
                    expressions.element(inner),
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(expression) = expression {
                syntax_expression_connect_variables_in_graph_from(
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    patterns,
                    types,
                    expressions.element(expression),
                    project_fn_graph,
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
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    patterns,
                    types,
                    expressions.element(queried),
                    project_fn_graph,
                );
            }
            for case in cases {
                if let Some(result) = &case.result {
                    syntax_expression_connect_variables_in_graph_from(
                        origin_project_fn_graph_node,
                        project_fn_graph_node_by_name,
                        expressions,
                        patterns,
                        types,
                        result,
                        project_fn_graph,
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
                    origin_project_fn_graph_node,
                    project_fn_graph_node_by_name,
                    expressions,
                    patterns,
                    types,
                    expressions.element(result),
                    project_fn_graph,
                );
            }
        }
    }
}
#[derive(Debug)]
pub struct SyntaxProjectFnInfo<'a, Expressions, Patterns, Types> {
    pub range: lsp_types::Range,
    pub name: &'a WithStartPosition<Name>,
    pub type_parameters: &'a Option<SyntaxAngledTypeParameters>,
    pub parameter: &'a Option<SyntaxPattern<Patterns, Types>>,
    pub result_type: &'a Option<SyntaxType<Types>>,
    pub documentation: &'a Option<SyntaxComments>,
    pub result: &'a Option<SyntaxExpression<Expressions, Patterns, Types>>,
}
#[derive(Debug)]
pub struct SyntaxProjectTypeInfo<'a, Types> {
    // consider introducing separate structs instead of separately referencing each field
    pub name: &'a WithStartPosition<Name>,
    pub documentation: &'a Option<SyntaxComments>,
    pub parameters: &'a Option<TyParameters>,
    pub type_: &'a Option<SyntaxType<Types>>,
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
        *self
    }
}
impl<'a, Types> Copy for SyntaxProjectTypeInfo<'a, Types> {}
impl<'a, Types> Clone for SyntaxProjectTypeInfo<'a, Types> {
    fn clone(&self) -> Self {
        *self
    }
}

fn checked_project_to_rust<Expressions, Patterns, Types>(
    CheckedSyntaxProject {
        type_graph,
        project_type_by_graph_node: _,
        project_fn_graph,
        project_fn_graph_node_by_name,
        project_fn_by_graph_node,
        records_used,
        choices_used,
        checked_type_aliases,
        checked_project_fns,
        checked_local_fns,
        checked_queries,
        checked_spread_records,
    }: CheckedSyntaxProject<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> CompiledProject {
    let mut rust_items: Vec<syn::Item> =
        Vec::with_capacity(type_graph.len() * 3 + project_fn_graph.len());
    for (checked_type_alias_name, checked_type_alias) in &checked_type_aliases {
        // TODO a better solution is likely to set core .type_ = None
        if let Some(checked_aliased_type) = &checked_type_alias.type_
            && !core_type_aliases.contains_key(checked_type_alias_name)
        {
            rust_items.push(project_type_alias_to_rust(
                checked_type_alias.documentation.as_deref(),
                checked_type_alias_name,
                &checked_type_alias.parameters,
                checked_aliased_type,
            ));
        }
    }
    for (project_fn_name, checked_project_fn) in &checked_project_fns {
        if let Some(syntax_project_fn_node) = project_fn_graph_node_by_name.get(project_fn_name)
            && let Some(syntax_project_fn) = project_fn_by_graph_node.get(syntax_project_fn_node)
            && let Some(parameter_type) = &checked_project_fn.parameter_type
            && let Some(result_type) = &checked_project_fn.result_type
            && let Some(parameter) = syntax_project_fn.parameter.as_ref()
            && let Some(result) = syntax_project_fn.result.as_ref()
        {
            rust_items.push(syntax_project_fn_to_rust(
                &checked_type_aliases,
                &checked_project_fns,
                &checked_local_fns,
                &checked_queries,
                &checked_spread_records,
                expressions,
                patterns,
                types,
                project_fn_name,
                checked_project_fn.documentation.as_deref(),
                parameter_type,
                result_type,
                checked_project_fn.result_expression_is_invalid,
                parameter,
                result,
            ));
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
            frontmatter: None,
            attrs: vec![],
            items: rust_items,
        },
        type_aliases: checked_type_aliases,
        fns: checked_project_fns,
        records: records_used,
        queries: checked_queries,
        spread_records: checked_spread_records,
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
                        modifiers: syn::FieldModifiers::default(),
                        vis: syn::Visibility::Inherited,
                        ident: None,
                        colon_token: None,
                        ty: syn::Type::Path(syn::TypePath {
                            attrs: vec![],
                            qself: None,
                            path: syn_path_reference([&type_variable_to_rust(variant_name)]),
                        }),
                        default: None,
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
                    modifiers: syn::FieldModifiers::default(),
                    ident: Some(syn_ident(&name_to_lowercase_rust(field_name))),
                    colon_token: Some(syn::token::Colon(syn_span())),
                    ty: syn::Type::Path(syn::TypePath {
                        attrs: vec![],
                        qself: None,
                        path: syn_path_reference([&type_variable_to_rust(field_name)]),
                    }),
                    default: None,
                })
                .collect(),
        }),
        semi_token: None,
    });
    rust_struct
}

fn project_type_alias_check<Types>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    project_type: SyntaxProjectTypeInfo<Types>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> CheckedTypeAlias {
    let documentation = project_type.documentation.as_ref().map(|documentation| {
        documentation
            .line1_up
            .iter()
            .fold(documentation.line0.value.to_string(), |so_far, line| {
                so_far + "\n" + &line.value
            })
            .into_boxed_str()
    });
    match &project_type.type_ {
        None => {
            errors.push(ErrorNode {
                range: name_range(with_start_position_as_ref(project_type.name)),
                message: Box::from("missing type after the project ty name ty ..type-name.. here"),
            });
            CheckedTypeAlias {
                name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                documentation: documentation,
                parameters: project_type
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
                    .collect::<Vec<_>>(),
                type_: None,
            }
        }
        Some(aliased_syntax_type) => {
            match syntax_type_check(
                aliased_syntax_type,
                errors,
                type_aliases,
                types,
                &std::collections::HashMap::new(),
                records_used,
                choices_used,
            ) {
                None => CheckedTypeAlias {
                    name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                    documentation: documentation,
                    parameters: project_type
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
                        .collect::<Vec<_>>(),
                    type_: None,
                },
                Some(aliased_type) => {
                    let mut actually_used_type_variables: std::collections::BTreeSet<&Name> =
                        std::collections::BTreeSet::new();
                    type_variables_into(&mut actually_used_type_variables, &aliased_type);
                    let parameters = parameters_check_if_different_to_actual_type_parameters(
                        errors,
                        name_range(with_start_position_as_ref(project_type.name)),
                        project_type.parameters.iter().flat_map(|parameters| {
                            std::iter::once(&parameters.parameter0).chain(
                                parameters
                                    .parameter1_up
                                    .iter()
                                    .filter_map(|parameter| parameter.name.as_ref()),
                            )
                        }),
                        actually_used_type_variables,
                    );
                    CheckedTypeAlias {
                        name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                        documentation: documentation,
                        parameters: parameters,
                        type_: Some(aliased_type),
                    }
                }
            }
        }
    }
}
fn project_type_alias_to_rust(
    maybe_documentation: Option<&str>,
    name: &Name,
    parameters: &[Name],
    aliased_type: &Type,
) -> syn::Item {
    let rust_name = name_to_uppercase_rust(name);
    let type_rust: syn::Type = type_to_rust(aliased_type);
    let rust_parameters: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> =
        parameters
            .iter()
            .map(|parameter| {
                syn::GenericParam::Type(syn::TypeParam::from(syn_ident(&type_variable_to_rust(
                    parameter,
                ))))
            })
            .collect();
    syn::Item::Type(syn::ItemType {
        attrs: maybe_documentation
            .map(|documentation| syn_attribute_doc(documentation))
            .into_iter()
            .collect::<Vec<_>>(),
        vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
        modifiers: syn::TypeModifiers::default(),
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
        where_clause_placement: syn::WhereClausePlacement::Late,
    })
}

fn syntax_project_fn_header_check<'a, Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    project_fn: SyntaxProjectFnInfo<'a, Expressions, Patterns, Types>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
) -> CheckedProjectFn {
    let maybe_parameter_type = project_fn.parameter.as_ref().and_then(|parameter| {
        syntax_pattern_check(
            parameter,
            None,
            errors,
            &mut std::collections::HashMap::new(),
            type_aliases,
            patterns,
            types,
            &std::collections::HashMap::new(),
            checked_spread_records,
            records_used,
            choices_used,
        )
        .map(|checked_parameter| checked_parameter.type_)
    });
    let result_type: Option<Type> =
        project_fn
            .result_type
            .as_ref()
            .and_then(|syntax_result_type| {
                syntax_type_check(
                    syntax_result_type,
                    errors,
                    type_aliases,
                    types,
                    &std::collections::HashMap::new(),
                    records_used,
                    choices_used,
                )
            });
    match result_type {
        Some(result_type) => {
            let mut type_variables_exclusively_used_in_result =
                std::collections::BTreeSet::<&Name>::new();
            type_variables_into(&mut type_variables_exclusively_used_in_result, &result_type);
            if let Some(parameter_type) = &maybe_parameter_type {
                // can be optimized
                let mut parameter_type_variables = std::collections::BTreeSet::<&Name>::new();
                type_variables_into(&mut parameter_type_variables, parameter_type);
                type_variables_exclusively_used_in_result
                    .retain(|var| !parameter_type_variables.contains(var));
            }
            let actually_used_parameters = parameters_check_if_different_to_actual_type_parameters(
                errors,
                name_range(with_start_position_as_ref(project_fn.name)),
                project_fn.type_parameters.iter().flat_map(|parameters| {
                    parameters.parameter0.iter().chain(
                        parameters
                            .parameter1_up
                            .iter()
                            .filter_map(|parameter| parameter.name.as_ref()),
                    )
                }),
                type_variables_exclusively_used_in_result,
            );
            CheckedProjectFn {
                documentation: None,
                type_parameters: actually_used_parameters,
                parameter_type: maybe_parameter_type,
                result_type: Some(result_type),
                result_expression_is_invalid: false,
            }
        }
        None => CheckedProjectFn {
            documentation: None,
            type_parameters: match &project_fn.type_parameters {
                None => vec![],
                Some(type_parameters) => type_parameters
                    .parameter0
                    .iter()
                    .chain(
                        type_parameters
                            .parameter1_up
                            .iter()
                            .filter_map(|parameter| parameter.name.as_ref()),
                    )
                    .map(|name| name.value.clone())
                    .collect(),
            },
            parameter_type: maybe_parameter_type,
            result_type: result_type,
            result_expression_is_invalid: false,
        },
    }
}
fn syntax_project_fn_check<'a, Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    project_fn: SyntaxProjectFnInfo<'a, Expressions, Patterns, Types>,
    checked_local_fns: &mut std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &mut std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> CheckedProjectFn {
    let checked_header = project_fns
        .get(&project_fn.name.value)
        .cloned()
        .unwrap_or_else(|| {
            syntax_project_fn_header_check(
                errors,
                type_aliases,
                patterns,
                types,
                project_fn,
                records_used,
                choices_used,
                checked_spread_records,
            )
        });
    let documentation = project_fn.documentation.as_ref().map(|documentation| {
        documentation
            .line1_up
            .iter()
            .fold(documentation.line0.value.to_string(), |so_far, line| {
                so_far + "\n" + &line.value
            })
            .into_boxed_str()
    });
    let Some(header_parameter_type) = checked_header.parameter_type else {
        // rust top level declarations need explicit types; partial types won't do
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_type: None,
            result_type: checked_header.result_type,
            result_expression_is_invalid: true,
        };
    };
    let Some(header_result_type) = checked_header.result_type else {
        // rust top level declarations need explicit types; partial types won't do
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_type: Some(header_parameter_type),
            result_type: None,
            result_expression_is_invalid: true,
        };
    };
    let Some(syntax_result) = project_fn.result else {
        errors.push(ErrorNode {
            range: name_range(with_start_position_as_ref(project_fn.name)),
            message: Box::from(
                "missing expression after the fn result type. An example would be fn my-function . str \":)\", where . is an empty record as the parameter",
            ),
        });
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_type: Some(header_parameter_type),
            result_type: Some(header_result_type),
            result_expression_is_invalid: true,
        };
    };
    let mut parameter_introduced_variables = std::collections::HashMap::new();
    if let Some(syntax_parameter) = &project_fn.parameter {
        syntax_pattern_untyped_variables_fold(
            syntax_parameter,
            (),
            &mut |(), name, type_| {
                parameter_introduced_variables.insert(
                    name.value,
                    CheckedPatternVariable {
                        origin_start: name.start,
                        type_: type_.and_then(|type_| {
                            syntax_type_to_type(
                                type_,
                                type_aliases,
                                types,
                                &std::collections::HashMap::<&Name, CheckedOrigin>::new(),
                            )
                        }),
                    },
                );
            },
            patterns,
        );
    }
    let mut result_used_pattern_variables = std::collections::HashMap::new();
    let Some(checked_result_expression_type) = syntax_expression_check(
        errors,
        type_aliases,
        project_fns,
        expressions,
        patterns,
        types,
        &mut parameter_introduced_variables,
        &mut result_used_pattern_variables,
        &mut std::collections::HashMap::new(),
        &mut std::collections::HashMap::new(),
        syntax_result,
        checked_local_fns,
        checked_queries,
        checked_spread_records,
        records_used,
        choices_used,
    ) else {
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_type: Some(header_parameter_type),
            result_type: Some(header_result_type),
            result_expression_is_invalid: true,
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
    if let Some(result_type_diff) = type_diff(&header_result_type, &checked_result_expression_type)
    {
        errors.push(ErrorNode {
            range: expression_range(syntax_result, expressions, patterns, types),
            message: type_diff_error_message(&result_type_diff).into_boxed_str(),
        });
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_type: Some(header_parameter_type),
            result_type: Some(header_result_type),
            result_expression_is_invalid: true,
        };
    }
    CheckedProjectFn {
        documentation: documentation,
        type_parameters: checked_header.type_parameters,
        parameter_type: Some(header_parameter_type),
        result_type: Some(header_result_type),
        result_expression_is_invalid: false,
    }
}
fn syntax_project_fn_to_rust<Expressions, Patterns, Types>(
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    checked_local_fns: &std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    project_fn_name: &Name,
    project_fn_documentation: Option<&str>,
    parameter_type: &Type,
    result_type: &Type,
    result_expression_is_invalid: bool,
    syntax_parameter: &SyntaxPattern<Patterns, Types>,
    syntax_result: &SyntaxExpression<Expressions, Patterns, Types>,
) -> syn::Item {
    let rust_attrs: Vec<syn::Attribute> = project_fn_documentation
        .map(|n| syn_attribute_doc(n))
        .into_iter()
        .collect::<Vec<_>>();
    let rust_ident: syn::Ident = syn_ident(&name_to_lowercase_rust(project_fn_name));
    let mut type_parameters = std::collections::BTreeSet::<&Name>::new();
    type_variables_into(&mut type_parameters, parameter_type);
    type_variables_into(&mut type_parameters, result_type);
    let rust_generics: syn::Generics = syn::Generics {
        lt_token: Some(syn::token::Lt(syn_span())),
        params: type_parameters
            .into_iter()
            .map(|name| {
                syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident: syn_ident(&type_variable_to_rust(name)),
                    colon_token: Some(syn::token::Colon(syn_span())),
                    bounds: syn::punctuated::Punctuated::new(),
                    default: None,
                })
            })
            .collect(),
        gt_token: Some(syn::token::Gt(syn_span())),
        where_clause: None,
    };
    let mut parameter_introduced_variables = std::collections::HashMap::new();
    let mut rust_statements = Vec::new();
    let compiled_parameter = match syntax_pattern_to_rust(
        syntax_parameter,
        None,
        &mut parameter_introduced_variables,
        type_aliases,
        checked_spread_records,
        patterns,
        types,
        &std::collections::HashMap::new(),
        &mut rust_statements,
    ) {
        None => syn::Pat::Wild(syn::PatWild {
            attrs: vec![],
            underscore_token: syn::token::Underscore(syn_span()),
        }),
        Some(compiled_parameter) => compiled_parameter,
    };
    let compiled_result = if result_expression_is_invalid {
        syn_expr_todo()
    } else {
        syntax_expression_to_rust(
            type_aliases,
            project_fns,
            expressions,
            patterns,
            types,
            checked_local_fns,
            checked_queries,
            checked_spread_records,
            &mut parameter_introduced_variables,
            &mut std::collections::HashMap::new(),
            syntax_result,
        )
    };
    rust_statements.extend(syn_spread_expr_block_into_stmts(compiled_result));
    syn::Item::Fn(syn::ItemFn {
        attrs: rust_attrs,
        vis: syn::Visibility::Public(syn::token::Pub(syn_span())),
        modifiers: syn::FnModifiers::default(),
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            safety: syn::Safety::Default,
            abi: None,
            fn_token: syn::token::Fn(syn_span()),
            ident: rust_ident,
            generics: rust_generics,
            paren_token: syn::token::Paren(syn_span()),
            inputs: std::iter::once(syn::FnArg::Typed(syn::PatType {
                pat: Box::new(compiled_parameter),
                attrs: vec![],
                colon_token: syn::token::Colon(syn_span()),
                ty: Box::new(type_to_rust(parameter_type)),
            }))
            .collect(),
            output: syn::ReturnType::Type(
                syn::token::RArrow(syn_span()),
                Box::new(type_to_rust(result_type)),
            ),
            variadic: None,
        },
        block: Box::new(syn::Block {
            brace_token: syn::token::Brace(syn_span()),
            stmts: rust_statements,
        }),
    })
}
/// only use if you know `syntax_type` has already been called on it before
pub fn syntax_type_to_type<Types, OriginInfo>(
    type_: &SyntaxType<Types>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashMap<&Name, OriginInfo>,
) -> Option<Type> {
    match type_ {
        SyntaxType::Variable(name) => Some(Type::Variable(name.value.clone())),
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => None,
            Some(inner) => syntax_type_to_type(types.element(inner), type_aliases, types, origins),
        },
        SyntaxType::ConstructWithoutArguments(name) => {
            if origins.contains_key(&name.value) {
                Some(Type::Origin(name.value.clone()))
            } else if let Some(origin_type_alias) = type_aliases.get(&name.value) {
                origin_type_alias.type_.clone()
            } else {
                None
            }
        }
        SyntaxType::ConstructWithArguments {
            underscore_start: _,
            name,
            argument0,
            argument1_up,
        } => {
            let Some(name) = name else {
                return None;
            };
            if origins.contains_key(&name.value) {
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
                        syntax_type_to_type(argument_type, type_aliases, types, origins)
                    })
                    .collect::<Option<Vec<Type>>>()?;
                type_construct_resolve_type_alias(origin_type_alias, &argument_types)
            } else {
                None
            }
        }
        SyntaxType::RecordEmpty { dot_start: _ } => Some(Type::Record(vec![])),
        SyntaxType::Record {
            field0_name,
            field0_value,
            field1_up,
        } => {
            let Some(field0_value) = field0_value else {
                return None;
            };
            let mut field_types: Vec<TypeField> = Vec::with_capacity(1 + field1_up.len());
            match syntax_type_to_type(types.element(field0_value), type_aliases, types, origins) {
                None => {}
                Some(field0_value_type) => {
                    field_types.push(TypeField {
                        name: field0_name.value.clone(),
                        value: field0_value_type,
                    });
                }
            }
            for field in field1_up {
                let Some(field_name) = &field.name.value else {
                    return None;
                };
                let Some(field_value) = &field.value else {
                    return None;
                };
                match syntax_type_to_type(field_value, type_aliases, types, origins) {
                    None => {}
                    Some(field_value_type) => {
                        field_types.push(TypeField {
                            name: field_name.clone(),
                            value: field_value_type,
                        });
                    }
                }
            }
            Some(Type::Record(field_types))
        }
        SyntaxType::ChoiceEmpty { bar_start: _ } => Some(Type::Choice(vec![])),
        SyntaxType::Choice {
            variant0_name,
            variant0_value,
            variant1_up,
        } => {
            let Some(variant0_value) = variant0_value else {
                return None;
            };
            let mut variant_types: Vec<TypeVariant> = Vec::with_capacity(1 + variant1_up.len());
            match syntax_type_to_type(types.element(variant0_value), type_aliases, types, origins) {
                None => {}
                Some(variant_value_type) => {
                    variant_types.push(TypeVariant {
                        name: variant0_name.value.clone(),
                        value: variant_value_type,
                    });
                }
            }
            for syntax_variant in variant1_up {
                let Some(variant_name) = &syntax_variant.name.value else {
                    return None;
                };
                let Some(syntax_variant_value) = &syntax_variant.value else {
                    return None;
                };
                match syntax_type_to_type(syntax_variant_value, type_aliases, types, origins) {
                    None => {}
                    Some(variant_value_type) => {
                        variant_types.push(TypeVariant {
                            name: variant_name.clone(),
                            value: variant_value_type,
                        });
                    }
                }
            }
            Some(Type::Choice(variant_types))
        }
    }
}
pub fn syntax_type_check<Types>(
    type_: &SyntaxType<Types>,
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashMap<&Name, CheckedOrigin>,
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
            Some(inner) => syntax_type_check(
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
                    message : Box::from("missing type name after this underscore _ . An example of a valid type construct is _vec Origin, u32")
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
                        syntax_type_check(
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
                                "this type alias has {} more parameters than arguments are provided here. Maybe you forgot a comma between the arguments? The additional {} called {}",
                                origin_type_alias.parameters.len() - argument_count,
                                if origin_type_alias.parameters.len() - argument_count == 1 {
                                    "parameter is"
                                } else {
                                    "parameters are"
                                },
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
            match syntax_type_check(
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
                    .any(|type_field| &type_field.name == field_name)
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
                match syntax_type_check(
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
            records_used.insert(sorted_field_names(
                field_types.iter().map(|field_type| &field_type.name),
            ));
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
            match syntax_type_check(
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
                    .any(|type_variant| &type_variant.name == variant_name)
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
                match syntax_type_check(
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
            choices_used.insert(sorted_variant_names(
                variant_types.iter().map(|variant_type| &variant_type.name),
            ));
            if any_variant_value_has_error {
                return None;
            }
            Some(Type::Choice(variant_types))
        }
    }
}
fn type_construct_resolve_type_alias(
    origin_type_alias: &CheckedTypeAlias,
    argument_types: &[Type],
) -> Option<Type> {
    let Some(type_alias_type) = &origin_type_alias.type_ else {
        return None;
    };
    if origin_type_alias.parameters.is_empty() {
        return Some(type_alias_type.clone());
    }
    let type_parameter_replacements: std::collections::BTreeMap<&str, std::borrow::Cow<Type>> =
        origin_type_alias
            .parameters
            .iter()
            .map(|n| n.as_str())
            .zip(argument_types.iter().map(std::borrow::Cow::Borrowed))
            .collect::<std::collections::BTreeMap<_, _>>();
    let mut peeled: Type = type_alias_type.clone();
    type_replace_variables(&type_parameter_replacements, &mut peeled);
    Some(peeled)
}
fn type_replace_variables(
    type_parameter_replacements: &std::collections::BTreeMap<&str, std::borrow::Cow<Type>>,
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
            attrs: vec![],
            qself: None,
            path: syn_path_reference([&name_to_uppercase_rust(name)]),
        }),
        Type::CoreConstruct { name, arguments } => syn::Type::Path(syn::TypePath {
            attrs: vec![],
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
                attrs: vec![],
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
                attrs: vec![],
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
fn type_variables_into<'a>(
    type_variables: &mut std::collections::BTreeSet<&'a Name>,
    type_: &'a Type,
) {
    match type_ {
        Type::Variable(name) => {
            type_variables.insert(name);
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
fn parameters_check_if_different_to_actual_type_parameters<'a>(
    errors: &mut Vec<ErrorNode>,
    parent_name_range: lsp_types::Range,
    parameters: impl Iterator<Item = &'a WithStartPosition<Name>>,
    mut actually_used_type_variables: std::collections::BTreeSet<&Name>,
) -> Vec<Name> {
    let mut actually_used_parameters = Vec::<Name>::with_capacity(parameters.size_hint().0);
    for parameter in parameters {
        if actually_used_type_variables.remove(&parameter.value) {
            actually_used_parameters.push(parameter.value.clone());
        } else {
            errors.push(ErrorNode {
                range: name_range(with_start_position_as_ref(parameter)),
                message: Box::from("this type variable is not used. Remove it or use it"),
            });
        }
    }
    if !actually_used_type_variables.is_empty() {
        errors.push(ErrorNode {
            range: parent_name_range,
            message: format!(
                "some type variables are used but not declared, namely {}. Add {}",
                actually_used_type_variables
                    .iter()
                    .copied()
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
    actually_used_parameters
}

struct CheckedPattern {
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
    Uncaught,
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
                            VariantCatch::Uncaught => VariantCatch::Uncaught,
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
                            VariantCatch::Uncaught => None,
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
                        VariantCatch::Uncaught => {
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
            PatternCatch::Record(_) => {}
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
            PatternCatch::Variant(_) => {}
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
                        (VariantCatch::Uncaught, VariantCatch::Caught(_)) => false,
                        (VariantCatch::Uncaught, VariantCatch::Uncaught) => true,
                        (VariantCatch::Caught(_), VariantCatch::Uncaught) => true,
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
///    if we encounter a variable pattern, we copy it's possibilities
///    to all "by variant" possibilities
///
///   when this pattern type is a record, spread (flatten) its field values into the original possibilities
///   for example:
///      ( { x ax0, y ay0 }, a1 ) or ( { x ax0, y ay0 }, b1 )
///      → is_exhaustive [ ( ax0, ay0, a1 ) or ( ax0, ay0, b1 ) ]
///
/// when all patterns on index 0 are variable patterns
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
                                            VariantCatch::Uncaught => None,
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

fn syntax_pattern_check<'a, Patterns, Types>(
    pattern: &'a SyntaxPattern<Patterns, Types>,
    expected_type: Option<&Type>,
    errors: &mut Vec<ErrorNode>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, CheckedPatternVariable>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashMap<&Name, CheckedOrigin>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> Option<CheckedPattern> {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            let maybe_checked_variable = match type_.as_ref() {
                None => match expected_type {
                    None => {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(name)),
                            message: Box::from("fn parameters need to have an explicit type. Add one to this pattern variable by appending a type like in your-variable u32 (both in parens if necessary)"),
                        });
                        None
                    }
                    Some(expected_type) => Some(CheckedPattern {
                        type_: expected_type.clone(),
                        catch: PatternCatch::Exhaustive,
                    }),
                },
                Some(actual_type) => {
                    let Some(actual_type) = syntax_type_check(
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
                    Some(CheckedPattern {
                        type_: actual_type,
                        catch: PatternCatch::Exhaustive,
                    })
                }
            };
            if let Some(checked_variable) = &maybe_checked_variable {
                let maybe_existing_variable_with_the_same_name = introduced_variables.insert(
                    &name.value,
                    CheckedPatternVariable {
                        origin_start: name.start,
                        type_: Some(checked_variable.type_.clone()),
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
            maybe_checked_variable
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
                    let Some(value) = value else {
                        errors.push(ErrorNode {
                            range: optional_variant_name_range(name),
                            message: Box::from("missing variant value after this variant name. Each variants has a value, even if just ., an example of a variant pattern is |present variable")
                        });
                        return None;
                    };
                    let Some(checked_value) = syntax_pattern_check(
                        patterns.element(value),
                        None,
                        errors,
                        introduced_variables,
                        type_aliases,
                        patterns,
                        types,
                        origins,
                        checked_spread_records,
                        records_used,
                        choices_used,
                    ) else {
                        return None;
                    };
                    Some(CheckedPattern {
                        type_: Type::Choice(vec![TypeVariant {
                            name: name_value.clone(),
                            value: checked_value.type_,
                        }]),
                        catch: checked_value.catch,
                    })
                }
                Some(expected_type) => {
                    let Type::Choice(origin_choice_type_variants) = &expected_type else {
                        let mut error_message: String = String::from(
                            "A variant is part of a choice type (for example |a u32 |b str) but the expected type here is\n",
                        );
                        type_format(&mut error_message, 0, expected_type);
                        error_message.push_str("\nYou might have intended this pattern to belong to a different query. Use parens for query case results");
                        errors.push(ErrorNode {
                            range: optional_variant_name_range(name),
                            message: error_message.into_boxed_str(),
                        });
                        return None;
                    };
                    let Some(expected_value_type) =
                        origin_choice_type_variants.iter().find_map(|variant| {
                            if &variant.name == name_value {
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
                    let Some(checked_value) = syntax_pattern_check(
                        value,
                        Some(expected_value_type),
                        errors,
                        introduced_variables,
                        type_aliases,
                        patterns,
                        types,
                        origins,
                        checked_spread_records,
                        records_used,
                        choices_used,
                    ) else {
                        return None;
                    };
                    if let Some(variant_value_type_diff) =
                        type_diff(expected_value_type, &checked_value.type_)
                    {
                        errors.push(ErrorNode {
                            range: pattern_range(value, patterns, types),
                            message: type_diff_error_message(&variant_value_type_diff)
                                .into_boxed_str(),
                        });
                        return None;
                    }
                    Some(CheckedPattern {
                        type_: expected_type.clone(),
                        catch: if origin_choice_type_variants.len() == 1 {
                            checked_value.catch
                        } else {
                            let mut variants: std::collections::BTreeMap<
                                Name,
                                VariantCatch<PatternCatch>,
                            > = origin_choice_type_variants
                                .iter()
                                .map(|variant| (variant.name.clone(), VariantCatch::Uncaught))
                                .collect();
                            if let Some(variant_catch) = variants.get_mut(name_value) {
                                *variant_catch = VariantCatch::Caught(checked_value.catch);
                            }
                            PatternCatch::Variant(variants)
                        },
                    })
                }
            }
        }
        SyntaxPattern::RecordEmpty { dot_start: _ } => Some(CheckedPattern {
            type_: Type::Record(vec![]),
            catch: PatternCatch::Exhaustive,
        }),
        SyntaxPattern::Record { part0, part1_up } => {
            let mut type_fields: Vec<TypeField> = Vec::with_capacity(1 + part1_up.len());
            let mut field_catches: std::collections::BTreeMap<Name, PatternCatch> =
                std::collections::BTreeMap::new();
            for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name, value } => {
                        let Some(field_name_value) = &name.value else {
                            errors.push(ErrorNode {
                                range: symbol_range(name.start, "."),
                                message: Box::from("missing field name after this dot ."),
                            });
                            return None;
                        };
                        let Some(value) = value else {
                            errors.push(ErrorNode {
                                range: field_name_range(WithStartPosition {
                                    start: name.start,
                                    value: field_name_value,
                                }),
                                message: Box::from("missing field value after this field name"),
                            });
                            return None;
                        };
                        if type_fields
                            .iter()
                            .any(|type_field| &type_field.name == field_name_value)
                        {
                            errors.push(ErrorNode {
                                range: field_name_range(WithStartPosition {
                                    start: name.start,
                                    value: field_name_value,
                                }),
                                message: Box::from(
                                    "a field with this name already exists in the record pattern",
                                ),
                            });
                            return None;
                        }
                        let maybe_expected_type_record =
                            expected_type.and_then(|expected_type| match expected_type {
                                Type::Record(type_fields) => Some(type_fields),
                                _ => None,
                            });
                        let Some(checked_field_value) = syntax_pattern_check(
                            patterns.element(value),
                            maybe_expected_type_record.and_then(|expected_record_type| {
                                expected_record_type
                                    .iter()
                                    .find(|expected_field| &expected_field.name == field_name_value)
                                    .map(|expected_field| &expected_field.value)
                            }),
                            errors,
                            introduced_variables,
                            type_aliases,
                            patterns,
                            types,
                            origins,
                            checked_spread_records,
                            records_used,
                            choices_used,
                        ) else {
                            return None;
                        };
                        type_fields.push(TypeField {
                            name: field_name_value.clone(),
                            value: checked_field_value.type_,
                        });
                        field_catches.insert(field_name_value.clone(), checked_field_value.catch);
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start,
                        record,
                    } => {
                        if expected_type.is_some() {
                            errors.push(ErrorNode {
                                range: symbol_range(*dot_dot_start, ".."),
                                message: Box::from("record spread .. syntax is not allowed in query case patterns as it's not immediately clear which specific fields are contained. Switch to matching all fields explicitly")
                            });
                            return None;
                        }
                        let Some(record) = record else {
                            errors.push(ErrorNode {
                                range: symbol_range(*dot_dot_start, ".."),
                                message: Box::from("missing pattern to spread into the record after this .. syntax. An example of a record spread pattern is .. variable its-record-type")
                            });
                            return None;
                        };
                        let Some(checked_record) = syntax_pattern_check(
                            patterns.element(record),
                            None,
                            errors,
                            introduced_variables,
                            type_aliases,
                            patterns,
                            types,
                            origins,
                            checked_spread_records,
                            records_used,
                            choices_used,
                        ) else {
                            return None;
                        };
                        let Type::Record(checked_record_type_fields) = checked_record.type_ else {
                            let mut error_message =
                                "the pattern after this record spread .. is not a record but\n"
                                    .to_string();
                            type_format(&mut error_message, 0, &checked_record.type_);
                            errors.push(ErrorNode {
                                range: symbol_range(*dot_dot_start, ".."),
                                message: error_message.into_boxed_str(),
                            });
                            return None;
                        };
                        let PatternCatch::Record(checked_record_catch_fields) =
                            checked_record.catch
                        else {
                            return None;
                        };
                        checked_spread_records.insert(
                            *dot_dot_start,
                            checked_record_type_fields
                                .iter()
                                .map(|checked_record_field| checked_record_field.name.clone())
                                .collect::<Vec<Name>>(),
                        );
                        if let Some(overlapping_field) = type_fields.iter().find(|type_field| {
                            checked_record_type_fields
                                .iter()
                                .any(|checked_record_type_field| {
                                    type_field.name == checked_record_type_field.name
                                })
                        }) {
                            let mut error_message = format!(
                                "The type of the record pattern after this .. spread contains a field with the name {}. A field with this name already exists in the record pattern. The type of the record pattern after .. is\n",
                                overlapping_field.name
                            );
                            type_record_format(&mut error_message, 0, &checked_record_type_fields);
                            errors.push(ErrorNode {
                                range: symbol_range(*dot_dot_start, ".."),
                                message: error_message.into_boxed_str(),
                            });
                            return None;
                        }
                        type_fields.extend(checked_record_type_fields);
                        field_catches.extend(checked_record_catch_fields);
                    }
                }
            }
            records_used.insert(sorted_field_names(
                type_fields.iter().map(|type_field| &type_field.name),
            ));
            Some(CheckedPattern {
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
            Some(inner) => syntax_pattern_check(
                patterns.element(inner),
                expected_type,
                errors,
                introduced_variables,
                type_aliases,
                patterns,
                types,
                origins,
                checked_spread_records,
                records_used,
                choices_used,
            ),
        },
    }
}
fn syntax_pattern_to_rust<'a, Patterns, Types>(
    pattern: &'a SyntaxPattern<Patterns, Types>,
    expected_type: Option<&Type>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, CheckedPatternVariable>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    origins: &std::collections::HashMap<&Name, CheckedOrigin>,
    recombine_statements: &mut Vec<syn::Stmt>,
) -> Option<syn::Pat> {
    match pattern {
        SyntaxPattern::Variable {
            name,
            type_: syntax_type,
        } => {
            let (variable_type, variable_rust) = match syntax_type.as_ref() {
                None => match expected_type {
                    None => return None,
                    Some(expected_type) => (
                        expected_type.clone(),
                        syn::Pat::Ident(syn::PatIdent {
                            attrs: vec![],
                            by_ref: None,
                            mutability: None,
                            ident: syn_ident(&name_to_lowercase_rust(&name.value)),
                            subpat: None,
                        }),
                    ),
                },
                Some(syntax_type) => {
                    let Some(type_) =
                        syntax_type_to_type(syntax_type, type_aliases, types, origins)
                    else {
                        return None;
                    };
                    (
                        type_,
                        match expected_type {
                            None => syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: syn_ident(&name_to_lowercase_rust(&name.value)),
                                subpat: None,
                            }),
                            Some(expected_type) => syn::Pat::Type(syn::PatType {
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
                        },
                    )
                }
            };
            let maybe_existing_variable_with_the_same_name = introduced_variables.insert(
                &name.value,
                CheckedPatternVariable {
                    origin_start: name.start,
                    type_: Some(variable_type),
                },
            );
            if maybe_existing_variable_with_the_same_name.is_some()
                || origins.contains_key(&name.value)
            {
                return None;
            }
            Some(variable_rust)
        }
        SyntaxPattern::Variant { name, value } => {
            let Some(name_value) = &name.value else {
                return None;
            };
            match expected_type {
                None => {
                    let Some(value) = value else {
                        return None;
                    };
                    let Some(compiled_value) = syntax_pattern_to_rust(
                        patterns.element(value),
                        None,
                        introduced_variables,
                        type_aliases,
                        checked_spread_records,
                        patterns,
                        types,
                        origins,
                        recombine_statements,
                    ) else {
                        return None;
                    };
                    Some(syn::Pat::TupleStruct(syn::PatTupleStruct {
                        attrs: vec![],
                        qself: None,
                        path: syn_path_reference([
                            &name_to_uppercase_rust(&variant_names_to_rust_enum_name(
                                std::iter::once(name_value),
                            )),
                            &name_to_uppercase_rust(name_value),
                        ]),
                        paren_token: syn::token::Paren(syn_span()),
                        elems: std::iter::once(compiled_value).collect(),
                    }))
                }
                Some(expected_type) => {
                    let Type::Choice(origin_choice_type_variants) = &expected_type else {
                        return None;
                    };
                    let Some(expected_value_type) =
                        origin_choice_type_variants.iter().find_map(|variant| {
                            if &variant.name == name_value {
                                Some(&variant.value)
                            } else {
                                None
                            }
                        })
                    else {
                        return None;
                    };
                    let Some(value) = value else {
                        return None;
                    };
                    let value = patterns.element(value);
                    let Some(compiled_value) = syntax_pattern_to_rust(
                        value,
                        Some(expected_value_type),
                        introduced_variables,
                        type_aliases,
                        checked_spread_records,
                        patterns,
                        types,
                        origins,
                        recombine_statements,
                    ) else {
                        return None;
                    };
                    Some(syn::Pat::TupleStruct(syn::PatTupleStruct {
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
                        elems: std::iter::once(compiled_value).collect(),
                    }))
                }
            }
        }
        SyntaxPattern::RecordEmpty { dot_start: _ } => Some(syn::Pat::Tuple(syn::PatTuple {
            attrs: vec![],
            paren_token: syn::token::Paren(syn_span()),
            elems: syn::punctuated::Punctuated::new(),
        })),
        SyntaxPattern::Record { part0, part1_up } => {
            let mut rust_fields: syn::punctuated::Punctuated<syn::FieldPat, syn::token::Comma> =
                syn::punctuated::Punctuated::new();
            let mut field_names: Vec<&Name> = Vec::new();
            for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name, value } => {
                        let Some(field_name_value) = &name.value else {
                            return None;
                        };
                        let Some(value) = value else {
                            return None;
                        };
                        let maybe_expected_type_record =
                            expected_type.and_then(|expected_type| match expected_type {
                                Type::Variable(_)
                                | Type::Origin(_)
                                | Type::CoreConstruct { .. }
                                | Type::Choice { .. } => None,
                                Type::Record(type_fields) => Some(type_fields),
                            });
                        let compiled_field_value = syntax_pattern_to_rust(
                            patterns.element(value),
                            maybe_expected_type_record.and_then(|expected_record_type| {
                                expected_record_type
                                    .iter()
                                    .find(|expected_field| &expected_field.name == field_name_value)
                                    .map(|expected_field| &expected_field.value)
                            }),
                            introduced_variables,
                            type_aliases,
                            checked_spread_records,
                            patterns,
                            types,
                            origins,
                            recombine_statements,
                        );
                        let Some(compiled_field_value) = compiled_field_value else {
                            return None;
                        };
                        field_names.push(field_name_value);
                        rust_fields.push(syn::FieldPat {
                            attrs: vec![],
                            member: syn::Member::Named(syn_ident(&name_to_lowercase_rust(
                                field_name_value,
                            ))),
                            colon_token: Some(syn::token::Colon(syn_span())),
                            pat: Box::new(compiled_field_value),
                        });
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start,
                        record,
                    } => {
                        let Some(record) = record else {
                            return None;
                        };
                        let Some(record_spread_field_names) =
                            checked_spread_records.get(dot_dot_start)
                        else {
                            return None;
                        };
                        let Some(compiled_record) = syntax_pattern_to_rust(
                            patterns.element(record),
                            None,
                            introduced_variables,
                            type_aliases,
                            checked_spread_records,
                            patterns,
                            types,
                            origins,
                            recombine_statements,
                        ) else {
                            return None;
                        };
                        let generated_record_field_variable_name = |rust_field_name: &str| {
                            format!(
                                "to_spread_{}·{}_{rust_field_name}",
                                dot_dot_start.line, dot_dot_start.character
                            )
                        };
                        rust_fields.extend(record_spread_field_names.iter().map(
                            |record_spread_field_name| {
                                let rust_record_spread_field_name =
                                    name_to_lowercase_rust(record_spread_field_name);
                                syn::FieldPat {
                                    attrs: vec![],
                                    member: syn::Member::Named(syn_ident(
                                        &rust_record_spread_field_name,
                                    )),
                                    colon_token: Some(syn::token::Colon(syn_span())),
                                    pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                                        attrs: vec![],
                                        by_ref: None,
                                        mutability: None,
                                        ident: syn_ident(&generated_record_field_variable_name(
                                            &rust_record_spread_field_name,
                                        )),
                                        subpat: None,
                                    })),
                                }
                            },
                        ));
                        let recombined = syn::Expr::Struct(syn::ExprStruct {
                            attrs: vec![],
                            qself: None,
                            path: syn_path_reference([&field_names_to_rust_record_struct_name(
                                record_spread_field_names.iter(),
                            )]),
                            brace_token: syn::token::Brace(syn_span()),
                            fields: record_spread_field_names
                                .iter()
                                .map(|record_spread_field_name| {
                                    let rust_record_spread_field_name =
                                        name_to_lowercase_rust(record_spread_field_name);
                                    syn::FieldValue {
                                        attrs: vec![],
                                        member: syn::Member::Named(syn_ident(
                                            &rust_record_spread_field_name,
                                        )),
                                        colon_token: Some(syn::token::Colon(syn_span())),
                                        expr: syn_expr_reference([
                                            &generated_record_field_variable_name(
                                                &rust_record_spread_field_name,
                                            ),
                                        ]),
                                    }
                                })
                                .collect(),
                            dot2_token: None,
                            rest: None,
                        });
                        // typed patterns always succeed, so we can use let destructuring
                        recombine_statements.push(syn::Stmt::Local(syn::Local {
                            attrs: vec![],
                            let_token: syn::token::Let(syn_span()),
                            modifiers: syn::LocalModifiers::default(),
                            pat: compiled_record,
                            init: Some(syn::LocalInit {
                                eq_token: syn::token::Eq(syn_span()),
                                expr: Box::new(recombined),
                                diverge: None,
                            }),
                            semi_token: syn::token::Semi(syn_span()),
                        }));
                    }
                }
            }
            Some(syn::Pat::Struct(syn::PatStruct {
                attrs: vec![],
                qself: None,
                path: syn_path_reference([&field_names_to_rust_record_struct_name(
                    field_names.into_iter(),
                )]),
                brace_token: syn::token::Brace(syn_span()),
                fields: rust_fields,
                rest: None,
            }))
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => None,
            Some(inner) => syntax_pattern_to_rust(
                patterns.element(inner),
                expected_type,
                introduced_variables,
                type_aliases,
                checked_spread_records,
                patterns,
                types,
                origins,
                recombine_statements,
            ),
        },
    }
}

#[derive(Clone, Debug)]
struct CheckedPatternVariable {
    origin_start: lsp_types::Position,
    type_: Option<Type>,
}
#[derive(Clone, Copy, Debug)]
pub struct CheckedOrigin {
    origin_start: lsp_types::Position,
}
fn syntax_expression_check<'a, Expressions, Patterns, Types>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    pattern_variables: &mut std::collections::HashMap<&'a Name, CheckedPatternVariable>,
    used_pattern_variables: &mut std::collections::HashMap<
        &'a Name,
        /* start */ lsp_types::Position,
    >,
    origins: &mut std::collections::HashMap<&'a Name, CheckedOrigin>,
    used_origin_variables: &mut std::collections::HashMap<
        &'a Name,
        /* start */ lsp_types::Position,
    >,
    expression: &'a SyntaxExpression<Expressions, Patterns, Types>,
    checked_local_fns: &mut std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &mut std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> Option<Type> {
    match expression {
        SyntaxExpression::Number {
            value,
            type_: syntax_type,
        } => match syntax_type {
            None => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: value.start,
                        end: position_add_characters(value.start, value.value.len() as u32),
                    },
                    message: Box::from("missing type after this number. Each number requires an explicit type to know its precision and range, like 0 u32 or 0 f32"),
                });
                None
            }
            Some(syntax_type) => {
                let Some(type_) = syntax_type_check(
                    syntax_type,
                    errors,
                    type_aliases,
                    types,
                    origins,
                    records_used,
                    choices_used,
                ) else {
                    return None;
                };
                match &type_ {
                    Type::CoreConstruct { name, arguments: _ } => match name.as_str() {
                        "p32" => match value.value.parse::<std::num::NonZeroU32>() {
                            Ok(_) => Some(type_p32),
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
                                None
                            }
                        },
                        "u32" => match value.value.parse::<u32>() {
                            Ok(_) => Some(type_u32),
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
                                None
                            }
                        },
                        "i32" => match value.value.parse::<i32>() {
                            Ok(_) => Some(type_i32),
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
                                None
                            }
                        },
                        "f32" => match value.value.parse::<f32>() {
                            Ok(_) => Some(type_f32),
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
                                None
                            }
                        },
                        _ => {
                            errors.push(ErrorNode {
                            range: lsp_types::Range {
                                start: value.start,
                                end: position_add_characters(value.start, value.value.len() as u32),
                            },
                            message: Box::from("the type after this number is not a number type. The possible types are: p32 u32 i32 f32"),
                        });
                            None
                        }
                    },
                    _ => {
                        errors.push(ErrorNode {
                            range: lsp_types::Range {
                                start: value.start,
                                end: position_add_characters(value.start, value.value.len() as u32),
                            },
                            message: Box::from("the type after this number is not a number type. The possible types are: p32 u32 i32 f32"),
                        });
                        None
                    }
                }
            }
        },
        SyntaxExpression::Char {
            open_quote_start,
            content,
            content_end,
            closed_quote_exists,
        } => match *content {
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
                None
            }
            Some(_) => Some(type_char),
        },
        SyntaxExpression::Str {
            open_quote_start: _,
            content: _,
            content_end: _,
            closed_quote_exists: _,
        } => Some(type_str),
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
                    return None;
                }
                Some(type_origin(Type::Origin(name.value.clone())))
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
                    return None;
                }
                let Some(variable_type) = variable_info.type_.clone() else {
                    return None;
                };
                Some(variable_type)
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from(
                        if project_fns.contains_key(name.value.as_str()) {
                            "functions always need to be called with an argument and start with an underscore, like _u32-add .a 0 u32 .b 1 u32. Otherwise check for typos."
                        } else {
                            "unknown variable name. No local variable has this name. Note that a local fn result can not refer to any variable from the outside. Otherwise check for typos."
                        }
                    )
                });
                None
            }
        }
        SyntaxExpression::Call {
            underscore_start,
            name,
            type_arguments: syntax_type_arguments,
            argument: syntax_argument,
        } => {
            let Some(name) = name else {
                errors.push(ErrorNode {
                    range: symbol_range(*underscore_start, "_"),
                    message: Box::from("missing function name after this underscore _ . An example of a valid function call is _u32-dup 2 u32"),
                });
                return None;
            };
            let Some(syntax_argument) = syntax_argument else {
                errors.push(ErrorNode {
                        message: Box::from("missing function call argument after this function name. Some functions like vec-empty just take . (the empty record) as an argument, so try putting . after the name and then check for potential type errors"),
                        range: name_range(with_start_position_as_ref(name)),
                    });
                return None;
            };
            let syntax_argument = expressions.element(syntax_argument);
            let Some(checked_argument_type) = syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
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
                    return None;
                }
                if let Some(type_arguments) = syntax_type_arguments {
                    errors.push(ErrorNode {
                        range: lsp_types::Range {
                            start: type_arguments.open_angle_start,
                            end: angled_type_arguments_end(type_arguments, types),
                        },
                        message: Box::from(
                            "type arguments on a local variable make no sense. Remove them",
                        ),
                    });
                }
                let Some(variable_type) = variable_info.type_.clone() else {
                    return None;
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
                        return None;
                    }
                };
                let [variable_type_input, variable_type_output] =
                    variable_type_arguments.as_slice()
                else {
                    return None;
                };
                if let Some(argument_variable_input_type_diff) =
                    type_diff(variable_type_input, &checked_argument_type)
                {
                    errors.push(ErrorNode {
                        range: expression_range(syntax_argument, expressions, patterns, types),
                        message: type_diff_error_message(&argument_variable_input_type_diff)
                            .into_boxed_str(),
                    });
                    return None;
                }
                Some(variable_type_output.clone())
            } else {
                let Some(project_fn_info) = project_fns.get(name.value.as_str()) else {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: Box::from("unknown function name. No project fn or local variable has this name. Note that a local fn expression can not refer to any variable from the outside. Otherwise check for typos.")
                    });
                    return None;
                };
                let Some((project_fn_parameter_type, project_fn_result_type)) = project_fn_info
                    .parameter_type
                    .as_ref()
                    .zip(project_fn_info.result_type.as_ref())
                else {
                    return None;
                };
                let (syntax_type_argument_count, syntax_type_arguments) =
                    match syntax_type_arguments {
                        None => (0, None),
                        Some(syntax_type_arguments) => (
                            1 + syntax_type_arguments.argument1_up.len(),
                            Some(
                                syntax_type_arguments.argument0.iter().chain(
                                    syntax_type_arguments
                                        .argument1_up
                                        .iter()
                                        .filter_map(|argument| argument.type_.as_ref()),
                                ),
                            ),
                        ),
                    };
                if syntax_type_argument_count != project_fn_info.type_parameters.len() {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!("incorrect number of type parameters. The project fn has {parameter_count} type {parameter_pluralized}, but you only provided {argument_count} as arguments. Type arguments are provided in a comma-separated list enclosed in angle brackets after the fn name, like in _vec-empty<u32> origin, each type parenthesized if necessary.",
                            parameter_count = project_fn_info.type_parameters.len(),
                            parameter_pluralized = if project_fn_info.type_parameters.len() == 1 {
                                "parameter"
                            } else {
                                "parameters"
                            },
                            argument_count = syntax_type_argument_count
                        ).into_boxed_str()
                    });
                    return None;
                }
                let mut type_arguments = Vec::new();
                for syntax_type_argument in syntax_type_arguments.into_iter().flatten() {
                    let Some(type_argument) = syntax_type_check(
                        syntax_type_argument,
                        errors,
                        type_aliases,
                        types,
                        origins,
                        records_used,
                        choices_used,
                    ) else {
                        return None;
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
                let mut argument_type_variable_replacements = std::collections::BTreeMap::new();
                type_collect_variables_that_are_concrete_into(
                    &mut argument_type_variable_replacements,
                    &fn_parameter_type,
                    &checked_argument_type,
                );
                let mut expected_argument_type = fn_parameter_type.clone();
                type_replace_variables(
                    &argument_type_variable_replacements,
                    &mut expected_argument_type,
                );
                let mut result_type = fn_result_type.clone();
                type_replace_variables(&argument_type_variable_replacements, &mut result_type);
                if let Some(argument_variable_input_type_diff) =
                    type_diff(&expected_argument_type, &checked_argument_type)
                {
                    errors.push(ErrorNode {
                        range: expression_range(syntax_argument, expressions, patterns, types),
                        message: type_diff_error_message(&argument_variable_input_type_diff)
                            .into_boxed_str(),
                    });
                    return None;
                }
                Some(result_type)
            }
        }
        SyntaxExpression::Variant { name, type_, value } => {
            let Some(name_value) = &name.value else {
                errors.push(ErrorNode {
                    range: optional_variant_name_range(name),
                    message: Box::from("missing variant name after this bar | . An example of a valid variant is |present<_opt str> \"hi c:\""),
                });
                return None;
            };
            let Some(syntax_type_argument) = type_ else {
                errors.push(ErrorNode {
                    range: optional_variant_name_range(name),
                    message: Box::from("missing type in angle brackets after this variant name. An example of a valid variant is |present<_opt str> \"hi c:\". If there should only ever by one variant, using a record with a single field is recommended over a single variant choice."),
                });
                return None;
            };
            let Some(syntax_type) = &syntax_type_argument.type_ else {
                errors.push(ErrorNode {
                    range: symbol_range(syntax_type_argument.open_angle_start, "<"),
                    message: Box::from("missing type argument in angle brackets. An example of a valid variant is |present<_opt str> \"hi c:\""),
                });
                return None;
            };
            let Some(checked_type) = syntax_type_check(
                syntax_type,
                errors,
                type_aliases,
                types,
                origins,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let Type::Choice(origin_choice_type) = &checked_type else {
                let mut error_message: String = String::from(
                    "this variant type should be a choice (for example |a u32 |b str) but it's\n",
                );
                type_format(&mut error_message, 0, &checked_type);
                errors.push(ErrorNode {
                    range: optional_variant_name_range(name),
                    message: error_message.into_boxed_str(),
                });
                return None;
            };
            let Some(expected_value_type) = origin_choice_type.iter().find_map(|variant| {
                if &variant.name == name_value {
                    Some(&variant.value)
                } else {
                    None
                }
            }) else {
                let mut error_message: String = format!(
                    "the actual variant name {} is not included in this type\n",
                    name_value
                );
                type_format(&mut error_message, 0, &checked_type);
                errors.push(ErrorNode {
                    range: type_range(syntax_type, types),
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
            let value = expressions.element(value);
            let Some(checked_value_type) = syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            if let Some(variant_value_type_diff) =
                type_diff(expected_value_type, &checked_value_type)
            {
                errors.push(ErrorNode {
                    range: expression_range(value, expressions, patterns, types),
                    message: type_diff_error_message(&variant_value_type_diff).into_boxed_str(),
                });
                return None;
            }
            Some(checked_type)
        }
        SyntaxExpression::Fn {
            open_bracket_start,
            parameter,
            closed_bracket_start: _,
            result,
        } => {
            let Some(parameter) = parameter else {
                errors.push(ErrorNode {
                    range: symbol_range(*open_bracket_start, "["),
                    message: Box::from("missing parameter after open bracket [. An example of a local fn expression is [n u32] _u32-add .a n .b 1 u32"),
                });
                return None;
            };
            let mut parameter_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedPatternVariable,
            > = std::collections::HashMap::new();
            let Some(checked_parmeter) = syntax_pattern_check(
                parameter,
                None,
                errors,
                &mut parameter_introduced_variables,
                type_aliases,
                patterns,
                types,
                origins,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let mut result_used_pattern_variables = std::collections::HashMap::new();
            let mut result_used_origin_variables = std::collections::HashMap::new();
            let Some(result) = result else {
                errors.push(ErrorNode {
                    range: symbol_range(*open_bracket_start, "["),
                    message: Box::from("missing result after [..pattern..] here"),
                });
                return None;
            };
            let Some(checked_result_type) = syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
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
                return None;
            }
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
            checked_local_fns.insert(
                *open_bracket_start,
                CheckedLocalFn {
                    parameter_type: checked_parmeter.type_.clone(),
                    result_type: checked_result_type.clone(),
                },
            );
            Some(type_fn(checked_parmeter.type_, checked_result_type))
        }
        SyntaxExpression::RecordEmpty { dot_start: _ } => Some(Type::Record(vec![])),
        SyntaxExpression::Record { part0, part1_up } => {
            let mut maybe_field_types: Option<Vec<TypeField>> = Some(Vec::new());
            'checking_fields: for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name, value } => {
                        let Some(field_name) = &name.value else {
                            errors.push(ErrorNode {
                                range: symbol_range(name.start, "."),
                                message: Box::from(
                                    "missing field name after this dot. An example of a valid record is .name \"svalbard\" .status \"hi\"",
                                ),
                            });
                            continue 'checking_fields;
                        };
                        let checked_field_value_type: Option<Type> = match value {
                            None => {
                                errors.push(ErrorNode {
                                    range: optional_field_name_range(name),
                                    message: Box::from(
                                        "missing field value expression after this field name",
                                    ),
                                });
                                None
                            }
                            Some(field_value) => syntax_expression_check(
                                errors,
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
                                checked_local_fns,
                                checked_queries,
                                checked_spread_records,
                                records_used,
                                choices_used,
                            ),
                        };
                        if let Some(field_types) = &mut maybe_field_types {
                            match checked_field_value_type {
                                None => {
                                    maybe_field_types = None;
                                }
                                Some(checked_field_value_type) => {
                                    field_types.push(TypeField {
                                        name: field_name.clone(),
                                        value: checked_field_value_type,
                                    });
                                }
                            }
                        }
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start,
                        record,
                    } => {
                        let Some(record) = record else {
                            errors.push(ErrorNode {
                                range: symbol_range(*dot_dot_start, ".."),
                                message: Box::from(
                                    "missing record expression after this .. spread",
                                ),
                            });
                            continue 'checking_fields;
                        };
                        let checked_record_type: Option<Type> = syntax_expression_check(
                            errors,
                            type_aliases,
                            project_fns,
                            expressions,
                            patterns,
                            types,
                            pattern_variables,
                            used_pattern_variables,
                            origins,
                            used_origin_variables,
                            expressions.element(record),
                            checked_local_fns,
                            checked_queries,
                            checked_spread_records,
                            records_used,
                            choices_used,
                        );
                        if let Some(field_types) = &mut maybe_field_types {
                            match checked_record_type {
                                None => {
                                    maybe_field_types = None;
                                }
                                Some(checked_record_type) => {
                                    let Type::Record(checked_record_type_fields) =
                                        checked_record_type
                                    else {
                                        let mut error_message = "expression after this .. spread must be a known record but it's type is\n".to_string();
                                        type_format(&mut error_message, 0, &checked_record_type);
                                        errors.push(ErrorNode {
                                            range: symbol_range(*dot_dot_start, ".."),
                                            message: error_message.into_boxed_str(),
                                        });
                                        maybe_field_types = None;
                                        continue 'checking_fields;
                                    };
                                    for checked_record_type_field in &checked_record_type_fields {
                                        if let Some(existing_clashing_field) =
                                            field_types.iter().find(|existing_field| {
                                                checked_record_type_field.name
                                                    == existing_field.name
                                            })
                                        {
                                            let mut error_message = format!(
                                                "record after this .. spread contains the field .{} which clashes with a previous field with the same name. The full type of the spread record is\n",
                                                &existing_clashing_field.name
                                            );
                                            type_record_format(
                                                &mut error_message,
                                                0,
                                                &checked_record_type_fields,
                                            );
                                            errors.push(ErrorNode {
                                                range: symbol_range(*dot_dot_start, ".."),
                                                message: error_message.into_boxed_str(),
                                            });
                                            maybe_field_types = None;
                                            continue 'checking_fields;
                                        }
                                    }
                                    checked_spread_records.insert(
                                        *dot_dot_start,
                                        checked_record_type_fields
                                            .iter()
                                            .map(|field| field.name.clone())
                                            .collect::<Vec<_>>(),
                                    );
                                    field_types.extend(checked_record_type_fields);
                                }
                            }
                        }
                    }
                }
            }
            // the only way to use a record expression is through
            // a pattern or type. Nothing to add to used_records here
            maybe_field_types.map(Type::Record)
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
                None
            }
            Some(inner) => syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
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
                None
            }
            Some(expression) => syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
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
                    message: Box::from("missing queried expression after this colon. An example of a query is ? option [|present n] n [|absent .] 0 u32")
                });
                return None;
            };
            let queried = expressions.element(queried);
            let Some((case0, case1_up)) = cases.split_first() else {
                errors.push(ErrorNode {
                    range: symbol_range(*question_mark_start, "?"),
                    message: Box::from("missing case(s) after the queried expression. Cases look like [pattern] result-expression. An example of a query is ? option [|present n] n [|absent .] 0 u32. If everything looks good on your end, try to parenthesize the expression after the ?, as the queried expression cannot already be an unpqrenthesized query")
                });
                return None;
            };
            let Some(checked_queried_type) = syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let Some(case0_pattern) = &case0.pattern else {
                errors.push(ErrorNode {
                    range:  symbol_range(case0.open_bracket_start, "["),
                    message: Box::from("missing query case pattern after this open bracket [. Cases consist of [pattern] result-expression. An example of a query is ? option [|present n] n [|absent] 0 u32")
                });
                return None;
            };
            let Some(case0_result) = &case0.result else {
                errors.push(ErrorNode {
                    range: case0.closed_bracket_start.map(|closed_bracket_start| symbol_range(closed_bracket_start, "]")).unwrap_or_else(|| pattern_range(case0_pattern, patterns, types)),
                    message: Box::from("missing result expression after this query case pattern. Cases look like [pattern] result-expression. An example of a query is ? option [|present n] n [|absent] 0 u32")
                });
                return None;
            };
            let mut case0_pattern_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedPatternVariable,
            > = std::collections::HashMap::new();
            let Some(checked_case0_pattern) = syntax_pattern_check(
                case0_pattern,
                Some(&checked_queried_type),
                errors,
                &mut case0_pattern_introduced_variables,
                type_aliases,
                patterns,
                types,
                origins,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            pattern_variables.extend(
                case0_pattern_introduced_variables
                    .iter()
                    .map(|(binding, info)| (*binding, info.clone())),
            );
            let mut case0_result_used_pattern_variables = std::collections::HashMap::new();
            let mut case0_result_used_origin_variables = std::collections::HashMap::new();
            let Some(checked_query_result_type) = syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                pattern_variables.retain(|variable, _| {
                    !case0_pattern_introduced_variables.contains_key(variable)
                });
                return None;
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
            let mut catch = pattern_catch_to_case_patterns_catch(checked_case0_pattern.catch);
            let mut invalid_case_indexes = Vec::new();
            'checking_case1_up: for (case_index, case) in case1_up
                .iter()
                .enumerate()
                .map(|(i_in_1up, case)| (i_in_1up + 1, case))
            {
                let Some(case_pattern) = &case.pattern else {
                    errors.push(ErrorNode {
                        range:  symbol_range(case.open_bracket_start, "["),
                        message: Box::from("missing query case pattern after this open bracket [. Cases are written as [pattern] result-expression. A full query could look like ? option [|present n] n [|absent] 0 u32")
                    });
                    continue 'checking_case1_up;
                };
                let mut case_pattern_introduced_variables: std::collections::HashMap<
                    &Name,
                    CheckedPatternVariable,
                > = std::collections::HashMap::new();
                let Some(checked_case_pattern) = syntax_pattern_check(
                    case_pattern,
                    Some(&checked_queried_type),
                    errors,
                    &mut case_pattern_introduced_variables,
                    type_aliases,
                    patterns,
                    types,
                    origins,
                    checked_spread_records,
                    records_used,
                    choices_used,
                ) else {
                    invalid_case_indexes.push(case_index);
                    continue 'checking_case1_up;
                };
                if let Some(queried_pattern_type_diff) =
                    type_diff(&checked_queried_type, &checked_case_pattern.type_)
                {
                    errors.push(ErrorNode {
                        range: pattern_range(case_pattern, patterns, types),
                        message: (type_diff_error_message(&queried_pattern_type_diff)
                            + "\n\nA query case pattern must have the same type as the queried expression")
                                .into_boxed_str(),
                    });
                    invalid_case_indexes.push(case_index);
                    continue 'checking_case1_up;
                }
                pattern_catch_merge_with(
                    errors,
                    pattern_range(case_pattern, patterns, types),
                    &mut catch,
                    checked_case_pattern.catch,
                );
                let Some(case_result) = &case.result else {
                    errors.push(ErrorNode {
                        range: case.closed_bracket_start.map(|closed_bracket_start| symbol_range(closed_bracket_start, "]")).unwrap_or_else(||pattern_range(case_pattern, patterns, types)),
                        message: Box::from("missing result expression after this query case pattern. Cases are written as [pattern] result-expression. An example of a query is ? option [|present n] n [|absent .] 0 u32")
                    });
                    continue 'checking_case1_up;
                };
                pattern_variables.extend(
                    case_pattern_introduced_variables
                        .iter()
                        .map(|(binding, info)| (*binding, info.clone())),
                );
                let mut case_result_used_pattern_variables = std::collections::HashMap::new();
                let mut case_result_used_origin_variables = std::collections::HashMap::new();
                let Some(checked_case_result_type) = syntax_expression_check(
                    errors,
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
                    checked_local_fns,
                    checked_queries,
                    checked_spread_records,
                    records_used,
                    choices_used,
                ) else {
                    pattern_variables.retain(|variable, _| {
                        !case_pattern_introduced_variables.contains_key(variable)
                    });
                    invalid_case_indexes.push(case_index);
                    continue 'checking_case1_up;
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
                    if !case0_result_used_pattern_variables
                        .contains_key(case_result_used_pattern_variable)
                    {
                        errors.push(ErrorNode {
                            range: name_range(WithStartPosition { value: case_result_used_pattern_variable, start: case_result_used_pattern_variable_start }),
                            message: Box::from("this query case pattern variable is not used in the result of the first case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you do not need to use this variable in that case, just use any of the -rid functions to scrap it, like ? u32-rid your-variable [.] ..your existing case result..")
                        });
                    }
                }
                for (
                    case0_result_used_pattern_variable,
                    &case0_result_used_pattern_variable_start,
                ) in &case0_result_used_pattern_variables
                {
                    if !case_result_used_pattern_variables
                        .contains_key(case0_result_used_pattern_variable)
                    {
                        errors.push(ErrorNode {
                            range: name_range(WithStartPosition { value: case0_result_used_pattern_variable, start: case0_result_used_pattern_variable_start }),
                            message: format!(
                                "this query case pattern variable is not used in the result of the {} case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you do not need to use this variable in that case, just use any of the -rid functions to scrap it, like ? u32-rid your-variable [.] ..your existing case result..",
                                index_to_th(case_index)
                            ).into_boxed_str()
                        });
                    }
                }
                for (case_result_used_origin_variable, &case_result_used_origin_variable_start) in
                    &case_result_used_origin_variables
                {
                    if !case0_result_used_origin_variables
                        .contains_key(case_result_used_origin_variable)
                    {
                        errors.push(ErrorNode {
                            range: name_range(WithStartPosition { value: case_result_used_origin_variable, start: case_result_used_origin_variable_start }),
                            message: Box::from("this query case origin variable is not used in the result of the first case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you do not need to use this variable in that case, just use any of the -rid functions to scrap it, like ? u32-rid your-variable [.] ..your existing case result..")
                        });
                    }
                }
                for (case0_result_used_origin_variable, &case0_result_used_origin_variable_start) in
                    &case0_result_used_origin_variables
                {
                    if !case_result_used_origin_variables
                        .contains_key(case0_result_used_origin_variable)
                    {
                        errors.push(ErrorNode {
                            range: name_range(WithStartPosition { value: case0_result_used_origin_variable, start: case0_result_used_origin_variable_start }),
                            message: format!(
                                "this query case origin variable is not used in the result of the {} case. This is problematic because accidentally not handling a value in one branch could lead to leaked memory. If you do not need to use this variable in that case, just use any of the -rid functions to scrap it, like ? u32-rid your-variable [.] ..your existing case result..",
                                index_to_th(case_index)
                            ).into_boxed_str()
                        });
                    }
                }
                if let Some(match_result_case_result_type_diff) =
                    type_diff(&checked_query_result_type, &checked_case_result_type)
                {
                    errors.push(ErrorNode {
                        range: expression_range(case_result, expressions, patterns, types),
                        message: (type_diff_error_message(&match_result_case_result_type_diff)
                            + "\n\nAll query case results must have the same type")
                            .into_boxed_str(),
                    });
                    invalid_case_indexes.push(case_index);
                }
            }
            match catch {
                CasePatternsCatch::Exhaustive => {}
                _ => {
                    if invalid_case_indexes.is_empty() {
                        errors.push(ErrorNode {
                            range: symbol_range(*question_mark_start, "?"),
                            message: Box::from("inexhaustive pattern match.
    A pattern match must cover all possible cases, otherwise the program would need to crash if such a value was matched on.
    It might be that a case is not indented enough."),
                        });
                    }
                }
            }
            used_pattern_variables.extend(case0_result_used_pattern_variables);
            used_origin_variables.extend(case0_result_used_origin_variables);
            checked_queries.insert(
                *question_mark_start,
                CheckedQuery {
                    is_exhaustive: match catch {
                        CasePatternsCatch::Exhaustive => true,
                        _ => false,
                    },
                    queried_type: checked_queried_type,
                    invalid_case_indexes: invalid_case_indexes,
                },
            );
            Some(checked_query_result_type)
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
                return None;
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
                    CheckedOrigin {
                        origin_start: origin_name.start,
                    },
                );
            }
            let checked_result_type = syntax_expression_check(
                errors,
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
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            );
            let Some(origin_name) = name else {
                return checked_result_type;
            };
            if let Some(result_type) = &checked_result_type
                && type_references_origin(result_type, &origin_name.value)
            {
                let mut type_string = String::new();
                type_format(&mut type_string, 0, result_type);
                errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(origin_name)),
                        message: format!(
                            "the type of the resulting expression references this origin:\n{}. This is not allowed as it would allow creating multiple collections with the same origin. Move this origin creation to before the outer expression and/or pass the origin as an argument",
                            type_string
                        ).into_boxed_str(),
                    });
                return None;
            }
            if used_origin_variables.remove(&origin_name.value).is_none() {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(origin_name)),
                    message: Box::from(
                        "this origin is never used as a variable. Use it or remove it",
                    ),
                });
                return checked_result_type;
            }
            checked_result_type
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
                "this pattern variable is not used in the resulting expression. Use it or use any of the -rid functions to scrap it, like ? u32-rid your-variable [.] ..your existing case result.."
            )
        });
    }
}
pub struct CheckedQuery {
    pub is_exhaustive: bool,
    pub queried_type: Type,
    pub invalid_case_indexes: Vec<usize>,
}
pub struct CheckedLocalFn {
    pub parameter_type: Type,
    pub result_type: Type,
}
fn syntax_expression_to_rust<'a, Expressions, Patterns, Types>(
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    checked_local_fns: &std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<
        /* .. start */ lsp_types::Position,
        Vec<Name>,
    >,
    pattern_variables: &mut std::collections::HashMap<&'a Name, CheckedPatternVariable>,
    origins: &mut std::collections::HashMap<&'a Name, CheckedOrigin>,
    expression: &'a SyntaxExpression<Expressions, Patterns, Types>,
) -> syn::Expr {
    match expression {
        SyntaxExpression::Number {
            value,
            type_: syntax_type,
        } => {
            let Some(syntax_type) = syntax_type else {
                return syn_expr_todo();
            };
            let Some(type_) = syntax_type_to_type(syntax_type, type_aliases, types, origins) else {
                return syn_expr_todo();
            };
            match &type_ {
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
                            syn::Expr::Call(syn::ExprCall {
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
                            })
                        }
                        Err(_) => syn_expr_todo(),
                    },
                    "u32" => match value.value.parse::<u32>() {
                        Ok(number) => syn::Expr::Lit(syn::ExprLit {
                            attrs: vec![],
                            lit: syn::Lit::Int(syn::LitInt::new(
                                &(number.to_string() + "u32"),
                                syn_span(),
                            )),
                        }),
                        Err(_) => syn_expr_todo(),
                    },
                    "i32" => match value.value.parse::<i32>() {
                        Ok(number) => syn::Expr::Lit(syn::ExprLit {
                            attrs: vec![],
                            lit: syn::Lit::Int(syn::LitInt::new(
                                &(number.to_string() + "i32"),
                                syn_span(),
                            )),
                        }),
                        Err(_) => syn_expr_todo(),
                    },
                    "f32" => match value.value.parse::<f32>() {
                        Ok(number) => syn::Expr::Lit(syn::ExprLit {
                            attrs: vec![],
                            lit: syn::Lit::Float(syn::LitFloat::new(
                                &(number.to_string() + "f32"),
                                syn_span(),
                            )),
                        }),
                        Err(_) => syn_expr_todo(),
                    },
                    _ => syn_expr_todo(),
                },
                _ => syn_expr_todo(),
            }
        }
        SyntaxExpression::Char {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
        } => match *content {
            None => syn_expr_todo(),
            Some(char) => syn::Expr::Lit(syn::ExprLit {
                attrs: vec![],
                lit: syn::Lit::Char(syn::LitChar::new(char, syn_span())),
            }),
        },
        SyntaxExpression::Str {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
        } => syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Str(syn::LitStr::new(content, syn_span())),
        }),
        SyntaxExpression::Variable(name) => {
            if let Some(_origin_info) = origins.get(&name.value) {
                syn_expr_reference([&name_to_lowercase_rust(&name.value)])
            } else if let Some(variable_info) = pattern_variables.get(&name.value) {
                let Some(_) = variable_info.type_.clone() else {
                    return syn_expr_todo();
                };
                syn_expr_reference([&name_to_lowercase_rust(&name.value)])
            } else {
                syn_expr_todo()
            }
        }
        SyntaxExpression::Call {
            underscore_start: _,
            name,
            type_arguments: syntax_type_arguments,
            argument: syntax_argument,
        } => {
            let Some(name) = name else {
                return syn_expr_todo();
            };
            if let Some(variable_info) = pattern_variables.get(&name.value) {
                let Some(_) = variable_info.type_.clone() else {
                    return syn_expr_todo();
                };
                let rust_reference: syn::Expr =
                    syn_expr_reference([&name_to_lowercase_rust(&name.value)]);
                match syntax_argument {
                    None => rust_reference,
                    Some(syntax_argument) => {
                        let syntax_argument = expressions.element(syntax_argument);
                        let compiled_argument: syn::Expr = syntax_expression_to_rust(
                            type_aliases,
                            project_fns,
                            expressions,
                            patterns,
                            types,
                            checked_local_fns,
                            checked_queries,
                            checked_spread_records,
                            pattern_variables,
                            origins,
                            syntax_argument,
                        );
                        syn::Expr::Call(syn::ExprCall {
                            attrs: vec![],
                            func: Box::new(syn_expr_reference([&name_to_lowercase_rust(
                                &name.value,
                            )])),
                            paren_token: syn::token::Paren(syn_span()),
                            args: std::iter::once(compiled_argument).collect(),
                        })
                    }
                }
            } else if let Some(_origin_info) = origins.get(&name.value) {
                syn_expr_reference([&name_to_lowercase_rust(&name.value)])
            } else {
                let Some(project_fn_info) = project_fns.get(name.value.as_str()) else {
                    return syn_expr_todo();
                };
                let Some((project_fn_parameter_type, project_fn_result_type)) = project_fn_info
                    .parameter_type
                    .as_ref()
                    .zip(project_fn_info.result_type.as_ref())
                else {
                    return syn_expr_todo();
                };
                let (syntax_type_argument_count, syntax_type_arguments) =
                    match syntax_type_arguments {
                        None => (0, None),
                        Some(syntax_type_arguments) => (
                            1 + syntax_type_arguments.argument1_up.len(),
                            Some(
                                syntax_type_arguments.argument0.iter().chain(
                                    syntax_type_arguments
                                        .argument1_up
                                        .iter()
                                        .filter_map(|argument| argument.type_.as_ref()),
                                ),
                            ),
                        ),
                    };
                if syntax_type_argument_count != project_fn_info.type_parameters.len() {
                    return syn_expr_todo();
                }
                let mut type_arguments = Vec::new();
                for syntax_type_argument in syntax_type_arguments.into_iter().flatten() {
                    let Some(type_argument) =
                        syntax_type_to_type(syntax_type_argument, type_aliases, types, origins)
                    else {
                        return syn_expr_todo();
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
                    None => rust_reference,
                    Some(syntax_argument) => {
                        let syntax_argument = expressions.element(syntax_argument);
                        let compiled_argument: syn::Expr = syntax_expression_to_rust(
                            type_aliases,
                            project_fns,
                            expressions,
                            patterns,
                            types,
                            checked_local_fns,
                            checked_queries,
                            checked_spread_records,
                            pattern_variables,
                            origins,
                            syntax_argument,
                        );
                        syn::Expr::Call(syn::ExprCall {
                            attrs: vec![],
                            func: Box::new(syn_expr_reference([&name_to_lowercase_rust(
                                &name.value,
                            )])),
                            paren_token: syn::token::Paren(syn_span()),
                            args: std::iter::once(compiled_argument).collect(),
                        })
                    }
                }
            }
        }
        SyntaxExpression::Variant { name, type_, value } => {
            let Some(name_value) = &name.value else {
                return syn_expr_todo();
            };
            let Some(syntax_type_argument) = type_ else {
                return syn_expr_todo();
            };
            let Some(syntax_type) = &syntax_type_argument.type_ else {
                return syn_expr_todo();
            };
            let Some(compiled_type) =
                syntax_type_to_type(syntax_type, type_aliases, types, origins)
            else {
                return syn_expr_todo();
            };
            let Type::Choice(origin_choice_type) = &compiled_type else {
                return syn_expr_todo();
            };
            let Some(value) = value else {
                return syn_expr_todo();
            };
            let value = expressions.element(value);
            let compiled_value_rust = syntax_expression_to_rust(
                type_aliases,
                project_fns,
                expressions,
                patterns,
                types,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                pattern_variables,
                origins,
                value,
            );
            syn::Expr::Call(syn::ExprCall {
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
            })
        }
        SyntaxExpression::Fn {
            open_bracket_start,
            parameter,
            closed_bracket_start: _,
            result,
        } => {
            let Some(checked_local_fn) = checked_local_fns.get(open_bracket_start) else {
                return syn_expr_todo();
            };
            let Some(parameter) = parameter else {
                return syn_expr_todo();
            };
            let Some(result) = result else {
                return syn_expr_todo();
            };
            let mut parameter_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedPatternVariable,
            > = std::collections::HashMap::new();
            let mut fn_result_statements: Vec<syn::Stmt> = Vec::new();
            let Some(compiled_parameter) = syntax_pattern_to_rust(
                parameter,
                None,
                &mut parameter_introduced_variables,
                type_aliases,
                checked_spread_records,
                patterns,
                types,
                origins,
                &mut fn_result_statements,
            ) else {
                return syn_expr_todo();
            };
            let compiled_result = syntax_expression_to_rust(
                type_aliases,
                project_fns,
                expressions,
                patterns,
                types,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                &mut parameter_introduced_variables,
                origins,
                expressions.element(result),
            );
            let mut type_variables = std::collections::BTreeSet::new();
            type_variables_into(&mut type_variables, &checked_local_fn.parameter_type);
            type_variables_into(&mut type_variables, &checked_local_fn.result_type);
            fn_result_statements.extend(syn_spread_expr_block_into_stmts(compiled_result));
            syn::Expr::Block(syn::ExprBlock {
                attrs: vec![],
                label: None,
                block: syn::Block {
                    brace_token: syn::token::Brace(syn_span()),
                    stmts: vec![
                        syn::Stmt::Item(syn::Item::Fn(syn::ItemFn {
                            attrs: vec![],
                            vis: syn::Visibility::Inherited,
                            modifiers: syn::FnModifiers::default(),
                            sig: syn::Signature {
                                constness: None,
                                asyncness: None,
                                safety: syn::Safety::Default,
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
                                    pat: Box::new(compiled_parameter),
                                    colon_token: syn::token::Colon(syn_span()),
                                    ty: Box::new(type_to_rust(&checked_local_fn.parameter_type)),
                                }))
                                .collect(),
                                variadic: None,
                                output: syn::ReturnType::Type(
                                    syn::token::RArrow(syn_span()),
                                    Box::new(type_to_rust(&checked_local_fn.result_type)),
                                ),
                            },
                            block: Box::new(syn::Block {
                                brace_token: syn::token::Brace(syn_span()),
                                stmts: fn_result_statements,
                            }),
                        })),
                        // local_unnamed_function_name as fn(_) -> _
                        syn::Stmt::Expr(
                            syn::Expr::Cast(syn::ExprCast {
                                attrs: vec![],
                                expr: Box::new(syn_expr_reference([local_unnamed_function_name])),
                                as_token: syn::token::As(syn_span()),
                                ty: Box::new(syn::Type::FnPtr(syn::TypeFnPtr {
                                    attrs: vec![],
                                    lifetimes: None,
                                    unsafety: None,
                                    abi: None,
                                    fn_token: syn::token::Fn(syn_span()),
                                    paren_token: syn::token::Paren(syn_span()),
                                    inputs: std::iter::once(syn::NamedArg {
                                        attrs: vec![],
                                        name: None,
                                        ty: syn::Type::Infer(syn::TypeInfer {
                                            attrs: vec![],
                                            underscore_token: syn::token::Underscore(syn_span()),
                                        }),
                                    })
                                    .collect(),
                                    variadic: None,
                                    output: syn::ReturnType::Type(
                                        syn::token::RArrow(syn_span()),
                                        Box::new(syn::Type::Infer(syn::TypeInfer {
                                            attrs: vec![],
                                            underscore_token: syn::token::Underscore(syn_span()),
                                        })),
                                    ),
                                })),
                            }),
                            None,
                        ),
                    ],
                },
            })
        }
        SyntaxExpression::RecordEmpty { dot_start: _ } => syn::Expr::Tuple(syn::ExprTuple {
            attrs: vec![],
            paren_token: syn::token::Paren(syn_span()),
            elems: syn::punctuated::Punctuated::new(),
        }),
        SyntaxExpression::Record { part0, part1_up } => {
            let mut rust_statements: Vec<syn::Stmt> = Vec::new();
            let mut rust_field_names = Vec::new();
            let mut rust_fields = syn::punctuated::Punctuated::new();
            'compiling_fields: for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name, value } => {
                        let Some(field_name) = &name.value else {
                            continue 'compiling_fields;
                        };
                        let compiled_field_value: syn::Expr = match value {
                            None => syn_expr_todo(),
                            Some(field_value) => syntax_expression_to_rust(
                                type_aliases,
                                project_fns,
                                expressions,
                                patterns,
                                types,
                                checked_local_fns,
                                checked_queries,
                                checked_spread_records,
                                pattern_variables,
                                origins,
                                expressions.element(field_value),
                            ),
                        };
                        rust_field_names.push(field_name);
                        rust_fields.push(syn::FieldValue {
                            attrs: vec![],
                            member: syn::Member::Named(syn_ident(&name_to_lowercase_rust(
                                field_name,
                            ))),
                            colon_token: Some(syn::token::Colon(syn_span())),
                            expr: compiled_field_value,
                        });
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start,
                        record,
                    } => {
                        let Some(record) = record else {
                            continue 'compiling_fields;
                        };
                        let Some(record_fields) = checked_spread_records.get(dot_dot_start) else {
                            continue 'compiling_fields;
                        };
                        let record_rust = syntax_expression_to_rust(
                            type_aliases,
                            project_fns,
                            expressions,
                            patterns,
                            types,
                            checked_local_fns,
                            checked_queries,
                            checked_spread_records,
                            pattern_variables,
                            origins,
                            expressions.element(record),
                        );
                        // potential improvement for readability:
                        // if the record value itself is a variable
                        // use that name and do not emit a statement
                        let generated_record_variable_name = format!(
                            "to_spread_{}·{}",
                            dot_dot_start.line, dot_dot_start.character
                        );
                        rust_statements.push(syn::Stmt::Local(syn::Local {
                            attrs: vec![],
                            let_token: syn::token::Let(syn_span()),
                            modifiers: syn::LocalModifiers::default(),
                            pat: syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: syn_ident(&generated_record_variable_name),
                                subpat: None,
                            }),
                            init: Some(syn::LocalInit {
                                eq_token: syn::token::Eq(syn_span()),
                                expr: Box::new(record_rust),
                                diverge: None,
                            }),
                            semi_token: syn::token::Semi(syn_span()),
                        }));
                        rust_field_names.extend(record_fields);
                        rust_fields.extend(record_fields.iter().map(|field_name| {
                            syn::FieldValue {
                                attrs: vec![],
                                member: syn::Member::Named(syn_ident(&field_name)),
                                colon_token: Some(syn::token::Colon(syn_span())),
                                expr: syn::Expr::Field(syn::ExprField {
                                    attrs: vec![],
                                    base: Box::new(syn_expr_reference([
                                        &generated_record_variable_name,
                                    ])),
                                    dot_token: syn::token::Dot(syn_span()),
                                    member: syn::Member::Named(syn_ident(&field_name)),
                                }),
                            }
                        }));
                    }
                }
            }
            let rust_struct_name: String =
                field_names_to_rust_record_struct_name(rust_field_names.into_iter());
            let rust_struct = syn::Expr::Struct(syn::ExprStruct {
                attrs: vec![],
                qself: None,
                path: syn_path_reference([&rust_struct_name]),
                brace_token: syn::token::Brace(syn_span()),
                fields: rust_fields,
                dot2_token: None,
                rest: None,
            });
            if rust_statements.is_empty() {
                rust_struct
            } else {
                rust_statements.push(syn::Stmt::Expr(rust_struct, None));
                syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: syn::Block {
                        brace_token: syn::token::Brace(syn_span()),
                        stmts: rust_statements,
                    },
                })
            }
        }
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => syn_expr_todo(),
            Some(inner) => syntax_expression_to_rust(
                type_aliases,
                project_fns,
                expressions,
                patterns,
                types,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                pattern_variables,
                origins,
                expressions.element(inner),
            ),
        },
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => match expression {
            None => syn_expr_todo(),
            Some(expression) => syntax_expression_to_rust(
                type_aliases,
                project_fns,
                expressions,
                patterns,
                types,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                pattern_variables,
                origins,
                expressions.element(expression),
            ),
        },
        SyntaxExpression::Query {
            question_mark_start,
            queried,
            cases,
        } => {
            let Some(checked_query) = checked_queries.get(question_mark_start) else {
                return syn_expr_todo();
            };
            let Some(queried) = queried else {
                return syn_expr_todo();
            };
            let queried = expressions.element(queried);
            let Some((case0, case1_up)) = cases.split_first() else {
                return syn_expr_todo();
            };
            let compiled_queried_rust = syntax_expression_to_rust(
                type_aliases,
                project_fns,
                expressions,
                patterns,
                types,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                pattern_variables,
                origins,
                queried,
            );
            let Some(case0_pattern) = &case0.pattern else {
                return syn_expr_todo();
            };
            let Some(case0_result) = &case0.result else {
                return syn_expr_todo();
            };
            let mut case0_pattern_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedPatternVariable,
            > = std::collections::HashMap::new();
            let mut case0_statements: Vec<syn::Stmt> = Vec::new();
            let Some(case0_pattern_compiled) = syntax_pattern_to_rust(
                case0_pattern,
                Some(&checked_query.queried_type),
                &mut case0_pattern_introduced_variables,
                type_aliases,
                checked_spread_records,
                patterns,
                types,
                origins,
                &mut case0_statements,
            ) else {
                return syn_expr_todo();
            };
            pattern_variables.extend(
                case0_pattern_introduced_variables
                    .iter()
                    .map(|(binding, info)| (*binding, info.clone())),
            );
            let case0_compiled_result = syntax_expression_to_rust(
                type_aliases,
                project_fns,
                expressions,
                patterns,
                types,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                pattern_variables,
                origins,
                case0_result,
            );
            for (case0_pattern_introduced_variable, _) in case0_pattern_introduced_variables {
                pattern_variables.remove(case0_pattern_introduced_variable);
            }
            case0_statements.extend(syn_spread_expr_block_into_stmts(case0_compiled_result));
            fn syn_arm(pattern: syn::Pat, statements: Vec<syn::Stmt>) -> syn::Arm {
                syn::Arm {
                    attrs: vec![],
                    pat: pattern,
                    fat_arrow_token: syn::token::FatArrow(syn_span()),
                    body: Box::new(syn::Expr::Block(syn::ExprBlock {
                        attrs: vec![],
                        label: None,
                        block: syn::Block {
                            brace_token: syn::token::Brace(syn_span()),
                            stmts: statements,
                        },
                    })),
                    comma: None,
                }
            }
            let mut rust_arms: Vec<syn::Arm> =
                vec![syn_arm(case0_pattern_compiled, case0_statements)];
            'compiling_case1_up: for (case_index, case) in case1_up
                .iter()
                .enumerate()
                .map(|(i_in_1up, case)| (i_in_1up + 1, case))
            {
                if checked_query.invalid_case_indexes.contains(&case_index) {
                    continue 'compiling_case1_up;
                }
                let Some(case_pattern) = &case.pattern else {
                    continue 'compiling_case1_up;
                };
                let mut case_pattern_introduced_variables: std::collections::HashMap<
                    &Name,
                    CheckedPatternVariable,
                > = std::collections::HashMap::new();
                let mut case_statements: Vec<syn::Stmt> = Vec::new();
                let Some(case_pattern_compiled) = syntax_pattern_to_rust(
                    case_pattern,
                    Some(&checked_query.queried_type),
                    &mut case_pattern_introduced_variables,
                    type_aliases,
                    checked_spread_records,
                    patterns,
                    types,
                    origins,
                    &mut case_statements,
                ) else {
                    continue 'compiling_case1_up;
                };
                pattern_variables.extend(
                    case_pattern_introduced_variables
                        .iter()
                        .map(|(binding, info)| (*binding, info.clone())),
                );
                let Some(case_result) = &case.result else {
                    rust_arms.push(syn_arm(
                        case_pattern_compiled,
                        vec![syn::Stmt::Expr(syn_expr_todo(), None)],
                    ));
                    continue 'compiling_case1_up;
                };
                let case_compiled_result_rust = syntax_expression_to_rust(
                    type_aliases,
                    project_fns,
                    expressions,
                    patterns,
                    types,
                    checked_local_fns,
                    checked_queries,
                    checked_spread_records,
                    pattern_variables,
                    origins,
                    case_result,
                );
                for (case_pattern_introduced_variable, _) in case_pattern_introduced_variables {
                    pattern_variables.remove(case_pattern_introduced_variable);
                }
                case_statements.extend(syn_spread_expr_block_into_stmts(case_compiled_result_rust));
                rust_arms.push(syn_arm(case_pattern_compiled, case_statements));
            }
            if !checked_query.is_exhaustive {
                // _ => todo!() is appended to still make inexhaustive matching compile
                // and be able to be run, rust will emit a warning
                rust_arms.push(syn::Arm {
                    attrs: vec![],
                    pat: syn::Pat::Wild(syn::PatWild {
                        attrs: vec![],
                        underscore_token: syn::token::Underscore(syn_span()),
                    }),
                    fat_arrow_token: syn::token::FatArrow(syn_span()),
                    body: Box::new(syn_expr_todo()),
                    comma: None,
                });
            }
            if rust_arms.len() == 1
                && let Some(only_match_arm) = rust_arms.pop()
            {
                syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: syn::Block {
                        brace_token: syn::token::Brace(syn_span()),
                        stmts: std::iter::once(
                            if let syn::Pat::Tuple(only_match_arm_pattern_tuple) =
                                &only_match_arm.pat
                                && only_match_arm_pattern_tuple.elems.is_empty()
                            {
                                // omit let () =
                                syn::Stmt::Expr(
                                    compiled_queried_rust,
                                    Some(syn::token::Semi(syn_span())),
                                )
                            } else {
                                syn::Stmt::Local(syn::Local {
                                    attrs: vec![],
                                    let_token: syn::token::Let(syn_span()),
                                    modifiers: syn::LocalModifiers::default(),
                                    pat: only_match_arm.pat,
                                    init: Some(syn::LocalInit {
                                        eq_token: syn::token::Eq(syn_span()),
                                        expr: Box::new(compiled_queried_rust),
                                        diverge: None,
                                    }),
                                    semi_token: syn::token::Semi(syn_span()),
                                })
                            },
                        )
                        .chain(syn_spread_expr_block_into_stmts(*only_match_arm.body))
                        .collect(),
                    },
                })
            } else {
                syn::Expr::Match(syn::ExprMatch {
                    attrs: vec![],
                    match_token: syn::token::Match(syn_span()),
                    expr: Box::new(compiled_queried_rust),
                    brace_token: syn::token::Brace(syn_span()),
                    arms: rust_arms,
                })
            }
        }
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name,
            result,
        } => {
            let Some(result) = result else {
                return syn_expr_todo();
            };
            if let Some(origin_name) = name {
                let _existing_origin_with_same_name = origins.remove(&origin_name.value);
                origins.insert(
                    &origin_name.value,
                    CheckedOrigin {
                        origin_start: origin_name.start,
                    },
                );
            }
            let result_compiled = syntax_expression_to_rust(
                type_aliases,
                project_fns,
                expressions,
                patterns,
                types,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                pattern_variables,
                origins,
                expressions.element(result),
            );
            let Some(origin_name) = name else {
                return result_compiled;
            };
            syn::Expr::Block(syn::ExprBlock {
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
                                delimiter: syn::MacroDelimiter::Paren(
                                    syn::token::Paren(syn_span()),
                                ),
                                tokens: {
                                    let mut token_stream = proc_macro2::TokenStream::new();
                                    <proc_macro2::TokenStream as quote::TokenStreamExt>::append_separated(
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
                        syn::Stmt::Expr(result_compiled, None),
                    ],
                },
            })
        }
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
        SyntaxPattern::Record { part0, part1_up } => {
            for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name: _, value } => {
                        if let Some(value) = value {
                            syntax_pattern_type_variables_into(
                                type_variables,
                                patterns.element(value),
                                patterns,
                                types,
                            );
                        }
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start: _,
                        record,
                    } => {
                        if let Some(record) = record {
                            syntax_pattern_type_variables_into(
                                type_variables,
                                patterns.element(record),
                                patterns,
                                types,
                            );
                        }
                    }
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
                );
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
        ) -> std::collections::BTreeMap<&'a str, std::borrow::Cow<'a, Type>> {
            let mut type_parameter_replacements = std::collections::BTreeMap::new();
            type_collect_variables_that_are_concrete_into(
                &mut type_parameter_replacements,
                type_with_variables,
                concrete_type,
            );
            type_parameter_replacements
        }
        fn type_variables_from<const N: usize>(
            type_variables: [(&'static str, Type); N],
        ) -> std::collections::BTreeMap<&'static str, std::borrow::Cow<'static, Type>> {
            std::collections::BTreeMap::from_iter(
                type_variables
                    .into_iter()
                    .map(|(name, type_)| (name, std::borrow::Cow::Owned(type_))),
            )
        }
        assert_eq!(
            concrete_type_variables(&type_variable("A"), &type_u32,),
            type_variables_from([("A", type_u32)])
        );
        assert_eq!(
            concrete_type_variables(
                &type_fn(type_variable("A"), type_variable("A")),
                &type_fn(type_variable("A"), type_u32),
            ),
            type_variables_from([("A", type_u32)])
        );
    }
}
fn type_collect_variables_that_are_concrete_into<'a>(
    type_parameter_replacements: &mut std::collections::BTreeMap<
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
            formatted.push('_');
            formatted.push_str(name);
            let line_span: LineSpan = type_diff_line_span(type_diff);
            for argument in arguments {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                type_diff_parenthesized_if_open_ended_into(
                    formatted,
                    next_indent(indent),
                    argument,
                );
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
/// this is set to a low-seeming number because hover windows and similar
/// are often small and sometimes unresizable
const type_info_line_length_estimate_maximum: usize = 50;
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
            1 + name.len()
                + arguments
                    .iter()
                    .map(|argument| 2 + type_diff_length_estimate(argument))
                    .sum::<usize>()
        }
        TypeDiff::Record(fields) => fields
            .iter()
            .map(|field| 3 + field.name.len() + type_diff_length_estimate(&field.value))
            .sum(),
        TypeDiff::Choice(variants) => variants
            .iter()
            .map(|variant| 3 + variant.name.len() + type_diff_length_estimate(&variant.value))
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
        Type::Record(fields) => type_record_format(formatted, indent, fields),
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
fn type_record_format(formatted: &mut String, indent: usize, fields: &[TypeField]) {
    match fields {
        [] => {
            formatted.push('.');
        }
        [field0, field1_up @ ..] => {
            type_field_format(formatted, indent, field0);
            let line_span: LineSpan = type_record_line_span(fields);
            for field in field1_up {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                type_field_format(formatted, indent, field);
            }
        }
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
        type_format(formatted, indent + 1, type_);
        if type_line_span(type_) == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
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
fn type_record_line_span(fields: &[TypeField]) -> LineSpan {
    if type_record_length_estimate(fields) <= type_info_line_length_estimate_maximum {
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
            1 + name.len()
                + arguments
                    .iter()
                    .map(|argument| 2 + type_length_estimate(argument))
                    .sum::<usize>()
        }
        Type::Record(fields) => type_record_length_estimate(fields),
        Type::Choice(variants) => variants
            .iter()
            .map(|variant| 3 + variant.name.len() + type_length_estimate(&variant.value))
            .sum(),
    }
}
fn type_record_length_estimate(fields: &[TypeField]) -> usize {
    fields
        .iter()
        .map(|field| 3 + field.name.len() + type_length_estimate(&field.value))
        .sum()
}

fn syn_spread_expr_block_into_stmts(syn_expr: syn::Expr) -> Vec<syn::Stmt> {
    match syn_expr {
        syn::Expr::Block(block) => block.block.stmts,
        _ => vec![syn::Stmt::Expr(syn_expr, None)],
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
        | "Unset"
        | "Set" => sanitized + "øø",
        _ => sanitized,
    }
}
const record_empty_rust_struct_name: &str = "Record";
const choice_empty_rust_struct_name: &str = "Choice";
fn name_to_lowercase_rust(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    let needs_to_be_disambiguated = rust_lowercase_keywords.contains(&sanitized.as_str())
        || match sanitized.as_str() {
            local_unnamed_function_name
            | "copy_ref_to_owned"
            | "origin_new"
            | "std"
            | "iterator_fold_in_direction"
            | "iterator_try_fold_in_direction" => true,
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
    rust_field_names_vec
        .into_iter()
        .fold("Record".to_string(), |so_far, field_name| {
            so_far + "·" + &field_name
        })
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
    rust_variant_names_vec
        .into_iter()
        .fold("Choice".to_string(), |so_far, variant_name| {
            so_far + "·" + &variant_name
        })
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
        attrs: vec![],
        qself: None,
        path: syn::Path::from(syn_ident(name)),
    })
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

const fn type_variable(name: &'static str) -> Type {
    Type::Variable(Name::from_static(name))
}
fn type_fn(in_: Type, out: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("fn"),
        arguments: vec![in_, out],
    }
}
fn type_record(fields: impl IntoIterator<Item = (&'static str, Type)>) -> Type {
    Type::Record(
        fields
            .into_iter()
            .map(|(field_name, field_value)| TypeField {
                name: Name::from_static(field_name),
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
                name: Name::from_static(variant_name),
                value: variant_value,
            })
            .collect(),
    )
}
const type_p32: Type = Type::CoreConstruct {
    name: Name::from_static("p32"),
    arguments: vec![],
};
const type_u32: Type = Type::CoreConstruct {
    name: Name::from_static("u32"),
    arguments: vec![],
};
const type_i32: Type = Type::CoreConstruct {
    name: Name::from_static("i32"),
    arguments: vec![],
};
const type_f32: Type = Type::CoreConstruct {
    name: Name::from_static("f32"),
    arguments: vec![],
};
const type_char: Type = Type::CoreConstruct {
    name: Name::from_static("char"),
    arguments: vec![],
};
const type_str: Type = Type::CoreConstruct {
    name: Name::from_static("str"),
    arguments: vec![],
};
fn type_origin(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("origin"),
        arguments: vec![origin],
    }
}
fn type_vec(origin: Type, element: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("vec"),
        arguments: vec![origin, element],
    }
}
fn type_slot(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("slot"),
        arguments: vec![origin],
    }
}
fn type_unset_slot(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("unset-slot"),
        arguments: vec![origin],
    }
}
fn type_span(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("span"),
        arguments: vec![origin],
    }
}
fn type_unset_span(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("unset-span"),
        arguments: vec![origin],
    }
}
fn type_array(element: Type, record: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("array"),
        arguments: vec![element, record],
    }
}
fn type_unset_slice(element: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("unset-slice"),
        arguments: vec![element],
    }
}
fn type_opt(present: Type) -> Type {
    type_choice([("absent", type_record([])), ("present", present)])
}
struct CoreFnInfo {
    name: &'static str,
    documentation: &'static str,
    type_parameters: Vec<Name>,
    parameter_type: Type,
    result_type: Type,
}
pub static core_fns: std::sync::LazyLock<std::collections::HashMap<Name, CheckedProjectFn>> =
    std::sync::LazyLock::new(|| {
        std::collections::HashMap::from([
            CoreFnInfo {
                name: "p32-dup",
                documentation:
                    "Split the p32 in two values with the same content",
                type_parameters: vec![],
                parameter_type: type_p32,
                result_type: type_record([("a", type_p32), ("b", type_p32)]),
            },
            CoreFnInfo {
                name: "p32-rid",
                documentation:
                    "Mark the given p32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                type_parameters: vec![],
                parameter_type:  (type_p32),
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "p32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameter_type: type_record([("p", type_p32), ("u", type_u32)]),
                result_type: type_p32,
            },
            CoreFnInfo {
                name: "u32-dup",
                documentation: "Split the u32 in two values with the same content",
                type_parameters: vec![],
                parameter_type: type_u32,
                result_type: type_record([("a", type_u32), ("b", type_u32)]),
            },
            CoreFnInfo {
                name: "u32-rid",
                documentation: "Mark the given u32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                type_parameters: vec![],
                parameter_type: type_u32,
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "u32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameter_type: type_record([("a", type_u32), ("b", type_u32)]),
                result_type: type_u32,
            },
            CoreFnInfo {
                name: "i32-dup",
                documentation: "Split the i32 in two values with the same content",
                type_parameters: vec![],
                parameter_type: type_i32,
                result_type: type_record([("a", type_i32), ("b", type_i32)]),
            },
            CoreFnInfo {
                name: "i32-rid",
                documentation: "Mark the given i32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                type_parameters: vec![],
                parameter_type: type_i32,
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "i32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameter_type: type_record([("a", type_i32), ("b", type_i32)]),
                result_type: type_i32,
            },
            CoreFnInfo {
                name: "f32-dup",
                documentation: "Split the f32 in two values with the same content",
                type_parameters: vec![],
                parameter_type: type_f32,
                result_type: type_record([("a", type_f32), ("b", type_f32)]),
            },
            CoreFnInfo {
                name: "f32-rid",
                documentation: "Mark the given f32 value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                type_parameters: vec![],
                parameter_type: type_f32,
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "f32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameter_type: type_record([("a", type_f32), ("b", type_f32)]),
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-mul-clamp",
                documentation: "Saturating a * b",
                type_parameters: vec![],
                parameter_type: type_record([("a", type_f32), ("b", type_f32)]),
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-div-clamp",
                documentation: "Saturating n / by.
Try not to divide by 0.0, as 0.0 will be returned which is not mathematically correct. This behaviour is consistent with gleam, pony, coq, lean.",
                type_parameters: vec![],
                parameter_type: type_record([("n", type_f32), ("by", type_f32)]),
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-away-from-0",
                documentation: "If not already equal to an integer value, find the closest neighboring integer.
Round midpoint of a negative number to the lower neighbor
and round midpoint of a positive number to the higher neighbor.
This is (I think) often the semantically correct mode for geometry, ui and similar use cases
where behavior should be the same everywhere and e.g adding 1 should not change it.
It's the default round operation implementation in e.g. [C](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf#page=271), [rust](https://doc.rust-lang.org/std/primitive.f32.html#method.round), [zig](https://ziglang.org/documentation/master/#round), ([llvm](https://llvm.org/docs/LangRef.html#llvm-round-intrinsic)), python 2.
However, it's slower than `f32-round-nearest-else-even` on most architectures.
```sloe
fn age . :> f32 >
    f32-round-nearest-else-away-from-0 68.8 f32
```",
                type_parameters: vec![],
                parameter_type: type_f32 ,
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-even",
                documentation: r#"If not already equal to an integer value, find the closest neighboring integer.
Round negative midpoint to the nearest even neighbor.
This is sometimes called banker's rounding and is well-supported by architectures.
An argument could be made that this is more "fair" than `f32-round-nearest-else-away-from-0`
when operating in a linear scale where numbers are distributed evenly (even numbers as likely as uneven in all cases)
but as a result it can feel less predictable.
If fairness is a real concern, the midpoint should be explicitly handled, e.g.
by ignoring midpoint values, counting midpoint values or actually [randomizing their outcome](https://en.wikipedia.org/wiki/Rounding#Randomized_rounding_to_an_integer).
It's the default round operation implementation in e.g. python 3, dotnet, haskell, [erlang](https://erlangcentral.org/wiki/index.php?title=Floating_Point_Rounding), lisp"#,
                type_parameters: vec![],
                parameter_type: type_f32 ,
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-up",
                documentation: "If not already equal to an integer value, find the closest greater (not absolute greater) neighboring integer.
Often called ceiling",
                type_parameters: vec![],
                parameter_type: type_f32 ,
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-down",
                documentation: "If not already equal to an integer value, find the closest smaller (not absolute smaller) neighboring integer.
Often called floor",
                type_parameters: vec![],
                parameter_type: type_f32 ,
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-toward-0",
                documentation: "If not already equal to an integer value, find the closest neighboring integer with a smaller absolute value.
Often called truncate",
                type_parameters: vec![],
                parameter_type: type_f32 ,
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-away-from-0",
                documentation: "If not already equal to an integer value, find the closest neighboring integer with a greater absolute value",
                type_parameters: vec![],
                parameter_type: type_f32 ,
                result_type: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-up-to-i32-clamp",
                documentation: "`f32-round-up`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameter_type: type_f32,
                result_type: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-down-to-i32-clamp",
                documentation: "`f32-round-down`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameter_type: type_f32,
                result_type: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-toward-0-to-i32-clamp",
                documentation: "`f32-round-toward-0`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameter_type: type_f32,
                result_type: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-away-from-0-to-i32-clamp",
                documentation: "`f32-round-away-from-0`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameter_type: type_f32,
                result_type: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-away-from-0-to-i32-clamp",
                documentation: "`f32-round-nearest-else-away-from-0`, then clamp to within 32 bits.
In effect, `f32-round-toward-0-to-i32-clamp` truncates off at the decimal point:
```sloe
fn age . :> f32 >
    f32-round-toward-0-to-i32-clamp 68.8 f32
```",
                type_parameters: vec![],
                parameter_type: type_f32 ,
                result_type: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-even-to-i32-clamp",
                documentation: "`f32-round-nearest-else-even`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameter_type: type_f32,
                result_type: type_i32,
            },
            CoreFnInfo {
                name: "char-dup",
                documentation: "Split the char in two values with the same content",
                type_parameters: vec![],
                parameter_type: type_char,
                result_type: type_record([("a", type_char), ("b", type_char)]),
            },
            CoreFnInfo {
                name: "char-rid",
                documentation: "Mark the given char value as \"won't be used anymore\". This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                type_parameters: vec![],
                parameter_type: type_char,
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "str-dup",
                documentation: "Split the str in two values with the same content",
                type_parameters: vec![],
                parameter_type: type_str,
                result_type: type_record([("a", type_str), ("b", type_str)]),
            },
            CoreFnInfo {
                name: "str-rid",
                documentation: "Mark the given str value as \"won't be used anymore\".
This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                type_parameters: vec![],
                parameter_type: type_str,
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "opt-present",
                documentation: "Shorthand for |present<opt ..value type..> value",
                type_parameters: vec![],
                parameter_type: type_variable("Present"),
                result_type: type_opt(type_variable("Present"))
            },
            CoreFnInfo {
                name: "fn-dup",
                documentation: "Split the fn in two values with the same content",
                type_parameters: vec![],
                parameter_type: type_fn(type_variable("In"), type_variable("Out")),
                result_type: type_record([
                    ("a", type_fn(type_variable("In"), type_variable("Out"))),
                    ("b", type_fn(type_variable("In"), type_variable("Out"))),
                ]),
            },
            CoreFnInfo {
                name: "fn-rid",
                documentation: "Mark the given fn value as \"won't be used anymore\".
This is usually done to scrap some function byproduct or to decompose some temporary storage at the end of some scope",
                type_parameters: vec![],
                parameter_type: type_fn(type_variable("In"), type_variable("Out")),
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "origin-rid",
                documentation: "Mark the given origin value as \"won't be used anymore\". This is usually done to ignore it only in some case",
                type_parameters: vec![],
                parameter_type: type_origin(type_variable("Origin")),
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "slot-to-span",
                documentation: "Create a span covering just the one given slot",
                type_parameters: vec![],
                parameter_type: type_slot(type_variable("Origin")),
                result_type: type_span(type_variable("Origin")),
            },
            CoreFnInfo {
                name: "unset-slot-to-span",
                documentation: "Create an unset-span covering just the one given slot",
                type_parameters: vec![],
                parameter_type: type_unset_slot(type_variable("Origin")),
                result_type: type_unset_span(type_variable("Origin")),
            },
            CoreFnInfo {
                name: "span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameter_type: type_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "span",
                            type_span(type_variable("Origin"))
                        ),
                        ("length", type_p32)
                    ]),
            },
            CoreFnInfo {
                name: "opt-span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameter_type: type_opt(type_span(type_variable("Origin"))),
                result_type:
                    type_record([
                        (
                            "span",
                            type_opt(type_span(type_variable("Origin")))
                        ),
                        ("length", type_u32)
                    ]),
            },
            CoreFnInfo {
                name: "span-start",
                documentation: "Split into the first slot and span after.
To join disconnected slots and spans back together, use helpers like `vec-span-add-own-opt-span`",
                type_parameters: vec![],
                parameter_type: type_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "start",
                            type_slot(type_variable("Origin"))
                        ),
                        ("after", type_opt(type_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "span-end",
                documentation: "Split into the last slot and span before",
                type_parameters: vec![],
                parameter_type: type_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "end",
                            type_slot(type_variable("Origin"))
                        ),
                        ("before", type_opt(type_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "span-start-of-length-positive",
                documentation: "Split after a given length with the start half known to have positive length.
If the length is greater than the given span's length, .start will be the existing span and .after will be empty.
```sloe
fn span-slot-at
    .span span _span Origin
    .index index u32
    :>
    .before _opt _span Origin
    .at _slot Origin
    .after _opt _span Origin
    >
    ?
        _span-start-of-length-positive
        .span span
        .length (_p32-add .p 1 p32 .u index)
    [.start start .after after]
    ? _span-end start [.before before .end at]
    .before before .at at .after after
```
See also `span-end-of-length-positive`, `span-start`.",
                type_parameters: vec![],
                parameter_type: type_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "start",
                            type_span(type_variable("Origin"))
                        ),
                        ("after", type_opt(type_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "span-end-of-length-positive",
                documentation: "Split before a given length from the end with the end half known to have positive length.
If the length is greater than the given span's length, .end will be the existing span and .before will be empty.
See also `span-start-of-length-positive`, `span-end`.",
                type_parameters: vec![],
                parameter_type: type_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "end",
                            type_span(type_variable("Origin"))
                        ),
                        ("before", type_opt(type_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "opt-span-fold",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line",
                type_parameters: vec![],
                parameter_type:
                    type_record([
                        (
                            "span",
                            type_span(type_variable("Origin"))
                        ),
                        ("direction", type_choice([("up", type_record([])), ("down", type_record([]))])),
                        ("state", type_variable("State")),
                        (
                            "step",
                            type_fn(
                                type_record([
                                    ("slot", type_variable("Origin")),
                                    ("state", type_slot(type_variable("State"))),
                                ]),
                                type_variable("State")
                            )
                        )
                    ]),
                result_type: type_variable("State"),
            },
            CoreFnInfo {
                name: "unset-span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameter_type: type_unset_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "span",
                            type_unset_span(type_variable("Origin"))
                        ),
                        ("length", type_p32)
                    ]),
            },
            CoreFnInfo {
                name: "opt-unset-span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameter_type: type_opt(type_unset_span(type_variable("Origin"))),
                result_type:
                    type_record([
                        (
                            "span",
                            type_opt(type_unset_span(type_variable("Origin")))
                        ),
                        ("length", type_u32)
                    ]),
            },
            CoreFnInfo {
                name: "unset-span-start",
                documentation: "Split into the first slot and span after",
                type_parameters: vec![],
                parameter_type: type_unset_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "start",
                            type_slot(type_variable("Origin"))
                        ),
                        ("after", type_opt(type_unset_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "unset-span-end",
                documentation: "Split into the last slot and span before",
                type_parameters: vec![],
                parameter_type: type_unset_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "end",
                            type_slot(type_variable("Origin"))
                        ),
                        ("before", type_opt(type_unset_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "unset-span-start-of-length-positive",
                documentation: "Split after a given length with the start half known to have positive length.
If the length is greater than the given span's length, .start will be the existing span and .after will be empty.
```sloe
fn unset-span-slot-at
    .span span _unset-span Origin
    .index index u32
    :>
    .before _opt _unset-span Origin
    .at _slot Origin
    .after _opt _unset-span Origin
    >
    ?
        _unset-span-start-of-length-positive
        .span span
        .length (_p32-add .p 1 p32 .u index)
    [.start start .after after]
    ? _unset-span-end start [.before before .end at]
    .before before .at at .after after
```
See also `unset-span-end-of-length-positive`, `unset-span-start`.",
                type_parameters: vec![],
                parameter_type: type_unset_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "start",
                            type_unset_span(type_variable("Origin"))
                        ),
                        ("after", type_opt(type_unset_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "unset-span-end-of-length-positive",
                documentation: "Split before a given length from the end with the end half known to have positive length.
If the length is greater than the given span's length, .end will be the existing span and .before will be empty.
See also `unset-span-start-of-length-positive`, `unset-span-end`.",
                type_parameters: vec![],
                parameter_type: type_unset_span(type_variable("Origin")),
                result_type:
                    type_record([
                        (
                            "end",
                            type_unset_span(type_variable("Origin"))
                        ),
                        ("before", type_opt(type_unset_span(type_variable("Origin"))))
                    ]),
            },
            CoreFnInfo {
                name: "opt-unset-span-fold",
                documentation: "Step through all unset slots, updating the given initial state for each taken slot in line",
                type_parameters: vec![],
                parameter_type:
                    type_record([
                        (
                            "span",
                            type_unset_span(type_variable("Origin") )
                        ),
                        ("direction", type_choice([("up", type_record([])), ("down", type_record([]))])),
                        ("state", type_variable("State")),
                        (
                            "step",
                            type_fn(
                                type_record([
                                    ("slot", type_variable("Origin")),
                                    ("state", type_unset_slot(type_variable("State"))),
                                ]),
                                type_variable("State")
                            )
                        )
                    ]),
                result_type: type_variable("State"),
            },
            CoreFnInfo {
                name: "vec-empty",
                documentation: "Initialize a `vec` with 0 elements. Modify with `vec-pre-allocate-at-least`, `vec-add`, `vec-add-unset` etc.",
                type_parameters: vec![Name::from_static("Element")],
                parameter_type: type_origin(type_variable("Origin")),
                result_type: type_vec(type_variable("Origin"), type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-reuse",
                documentation: "Initialize a `vec` with 0 elements and spare allocated memory from an `unset-slice`.
This can be used to recycle vec memory from one vec with one origin into another vec with a different origin.
```sloe
fn vec-recycle-empty-vec
    .new-origin new-origin NewOrigin
    .old old _vec OldOrigin, Element
    :> _vec NewOrigin Element >
    ? _vec-to-unset old [unset-slice]
    _vec-reuse .origin new-origin .slice unset-slice
```",
                type_parameters: vec![],
                parameter_type: type_record([
                    ("origin", type_origin(type_variable("Origin"))),
                    ("slice", type_unset_slice(type_variable("Element"))),
                ]),
                result_type: type_vec(type_variable("Origin"), type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-pre-allocate-at-least",
                documentation: "Reserves spare capacity for at least `length` more elements to be added.
This can prevent frequent re-allocation of the underlying array.
If you can guesstimate a lower bound of how many elements are ultimately added, this is always worth it!
Equivalent to `vec-add-unset-length` followed by `vec-opt-span-rid`",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("length", type_u32),
                ]),
                result_type: type_vec(type_variable("Origin"), type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-pre-allocation-rid",
                documentation: "Shrinks down spare capacity as much as possible.
Some allocators may scrap and re-allocate the whole vec as a result.
It's rarely useful but can reuce idle memory usage for vecs that are very unlikely to be added to in the future.
You may also use it to adjust memory usage after `vec-reuse` when the given `unset-slice` was large",
                type_parameters: vec![],
                parameter_type: type_vec(type_variable("Origin"), type_variable("Element")),
                result_type: type_vec(type_variable("Origin"), type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-insert",
                documentation: "Add a new element into the vec and keep a slot to it,
reusing vacant space earlier in the vec when available.
Use `vec-add` if you don't care about reuse.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("new", type_variable("Element")),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_slot(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-insert-unset",
                documentation: "Like `vec-insert` but without assigning a value just yet.
This like initializing an element with `undefined` memory,
with the difference that you can't possibly access it :)
Assign an unset-slot with `vec-set` or vacate it with `vec-vacate`",
                type_parameters: vec![],
                parameter_type: type_vec(type_variable("Origin"), type_variable("Element")),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-add",
                documentation: "Add a new element to the end of the vec and keep a slot to it without trying to reuse already vacant slots.
Can be faster than `vec-insert` when you expect no vacant elements or when all the storage gets scrapped soon anyway.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("new", type_variable("Element")),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_slot(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-add-unset",
                documentation: "Like `vec-add` but without assigning a value just yet.
Assign an unset-slot with `vec-set` or vacate it with `vec-vacate`",
                type_parameters: vec![],
                parameter_type: type_vec(type_variable("Origin"), type_variable("Element")),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-add-length-positive",
                documentation: "Claim a given count of new end slots to be set in the near future.
Combined with `vec-span-rid` this has the same effect as `vec-pre-allocate-at-least` for example.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("length", type_p32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_unset_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-add-length",
                documentation: "Claim a given count of new end slots to be set in the near future.
Combined with `vec-opt-span-rid` this has the same effect as `vec-pre-allocate-at-least` for example.
To get non-empty spans use `vec-add-length-positive`",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("length", type_u32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_unset_span(type_variable("Origin")))),
                ]),
            },
            CoreFnInfo {
                name: "vec-remove",
                documentation: "Remove and retrieve an element from the vec at a given slot (the inverse of vec-insert/vec-add).
Short for `vec-unset` followed by `vec-slot-rid`",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_slot(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("element", type_variable("Element")),
                ]),
            },
            CoreFnInfo {
                name: "vec-unset",
                documentation: "Retrieve an element from the vec at a given slot (the inverse of vec-set)
```sloe
fn vec-copy-u32-at
    .vec vec _vec Origin, u32
    .slot slot _slot Origin
    :>
    .vec vec _vec Origin, u32
    .slot slot _slot Origin
    .element u32
    >
    ? vec-unset .vec vec .slot slot
    [.vec vec .element element .slot unset-slot]
    ? u32-dup element [.a element .b element-copied]
    ? vec-set .vec vec .slot unset-slot .new element [.vec vec .slot slot]
    .vec vec .slot slot .element element-copied
```
A little roundabout but it works.
To remove the element entirely, use `vec-take`",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("element",type_variable("Element") ),
                    ("slot", type_slot(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                    ("element", type_variable("Element")),
                ]),
            },
            CoreFnInfo {
                name: "vec-set",
                documentation: "Put an element back into the given `unset-slot` (the inverse of vec-unset).
To instead replace a `slot`, use `slot-replace`",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                    ("new", type_slot(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-set",
                documentation: "Put an element back into the given `unset-slot` (the inverse of vec-unset).
To instead replace a `slot`, use `slot-replace`",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                    ("new", type_slot(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-slot-rid",
                documentation: "Return an `unset-slot` back to the vec for potential future reuse",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("slot", type_unset_slot(type_variable("Origin"))),
                ]),
                result_type: type_vec(type_variable("Origin"), type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-span-rid",
                documentation: "Return an `unset-span` back to the vec for potential future reuse",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_unset_span(type_variable("Origin"))),
                ]),
                result_type: type_vec(type_variable("Origin"), type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-opt-span-rid",
                documentation: "Return an `opt unset-span` back to the vec for potential future reuse",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_unset_span(type_variable("Origin")))),
                ]),
                result_type: type_vec(type_variable("Origin"), type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-span-reverse",
                documentation: "Order the referenced elements such that the previously last is now first, second last is second etc.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-opt-span-reverse",
                documentation: "Order the referenced elements such that the previously last is now first, second last is second etc.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                ]),
            },
            CoreFnInfo {
                name: "vec-opt-span-add",
                documentation: "Attach a given element at the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                    ("new", type_variable("Element")),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin")))
                ]),
            },
            CoreFnInfo {
                name: "vec-span-add",
                documentation: "Attach a given element at the end of the span",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                    ("new", type_variable("Element")),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin")))
                ]),
            },
            CoreFnInfo {
                name: "vec-span-add-array",
                documentation: "Attach a given `array` of elements at the end of the span.
This can remove a bunch of noise compared to chaining `vec-span-add` operations",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                    ("new", type_array(type_variable("Element"), type_variable("Record"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin")))
                ]),
            },
            CoreFnInfo {
                name: "vec-opt-span-add-array",
                documentation: "Attach a given `array` of elements at the end of the span.
This can remove a bunch of noise compared to chaining `vec-span-add` operations",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                    ("new", type_array(type_variable("Element"), type_variable("Record"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin")))
                ]),
            },
            CoreFnInfo {
                name: "vec-char-opt-span-add-str",
                documentation: "Attach a given `str` at the end of the span",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                    ("new", type_str),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin"))))
                ]),
            },
            CoreFnInfo {
                name: "vec-char-span-add-str",
                documentation: "Attach a given `str` to the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                    ("new", type_str),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-char-span-add-u32",
                documentation: "Print a given `u32` after the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                    ("new", type_u32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-char-opt-span-add-u32",
                documentation: "Print a given `u32` after the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                    ("new", type_u32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-char-span-add-i32",
                documentation: "Print a given `i32` after the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                    ("new", type_i32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-char-opt-span-add-i32",
                documentation: "Print a given `i32` after the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                    ("new", type_i32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-char-span-add-f32",
                documentation: "Print a given `f32` after the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                    ("new", type_f32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-char-opt-span-add-f32",
                documentation: "Print a given `f32` after the end of the span.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                    ("new", type_f32),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_char),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-move-opt-span-to-vacant",
                documentation: "Move the given span to a vacant range if there is vacant space available where moving the given span to would reduce the amount of vacant space.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                ]),
            },
            CoreFnInfo {
                name: "vec-move-span-to-vacant",
                documentation: "Move the given span to a vacant range if there is vacant space available where moving the given span to would reduce the amount of vacant space.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-move-span-to-end",
                documentation: "Move the given span to after all existing elements if necessary.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-move-opt-span-to-end",
                documentation: "Move the given span to after all existing elements if necessary.",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                ]),
            },
            CoreFnInfo {
                name: "vec-span-add-own-span",
                documentation: "Append the elements of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are vacated.
As an example, you could implement `vec-span-add` in sloe itself as
```sloe
fn vec-span-add
    .vec vec _vec Origin, Element
    .span span _span Origin
    .new new Element
    :>
        .vec _vec Origin, Element
        .span _span Origin
    >
    # the first line is optional: it ensures that the new slot will actually be connected,
    # meaning the new element can stay at its position
    ? _vec-span-move-to-end .vec vec .span span [.vec vec .span .span]
    ? _vec-add .vec vec .new new [.vec vec .slot new-slot]
    _vec-span-add-own-span
    .vec vec
    .start span
    .end _slot-to-span new-slot
```",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("start", type_span(type_variable("Origin"))),
                    ("end", type_span(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-span-add-own-opt-span",
                documentation: "Append the elements of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are vacated.
The most common use case is re-combining spans that have been split up with e.g. `span-start` (see also `slot-to-span`)",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("start", type_opt(type_span(type_variable("Origin")))),
                    ("end", type_span(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-opt-span-add-own-span",
                documentation: "Append the elements of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are vacated.
The most common use case is re-combining spans that have been split up with e.g. `span-end` (see also `slot-to-span`)",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("start", type_opt(type_span(type_variable("Origin")))),
                    ("end", type_span(type_variable("Origin"))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_span(type_variable("Origin"))),
                ]),
            },
            CoreFnInfo {
                name: "vec-opt-span-add-own-opt-span",
                documentation: "Append the elements of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are vacated",
                type_parameters: vec![],
                parameter_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("start", type_opt(type_span(type_variable("Origin")))),
                    ("end", type_opt(type_span(type_variable("Origin")))),
                ]),
                result_type: type_record([
                    (
                        "vec",
                        type_vec(type_variable("Origin"), type_variable("Element")),
                    ),
                    ("span", type_opt(type_span(type_variable("Origin")))),
                ]),
            },
            CoreFnInfo {
                name: "vec-to-unset",
                documentation: "Extract the underlying slice that been used to store elements in including spare capacity.
This `unset-slice` can if necessary be casted to a new element type with `unset-slice-cast-or-rid-and-allocate`.
Finally, the allocation can be the base of a new vec with `vec-reuse` or be scrapped with `unset-slice-rid`",
                type_parameters: vec![],
                parameter_type: type_vec(type_variable("Origin"), type_variable("Element")),
                result_type: type_unset_slice(type_variable("Element")),
            },
            CoreFnInfo {
                name: "vec-rid",
                documentation: "Mark the given vec value as \"won't be used anymore\".
Used for temporary vecs at the end of their scope once all of their elements are used up.
If any slots or spans are still floating around, you will not be able to get rid of them.
This nicely forces you to handle all remaining elements before you can get rid of the vec.
To reuse the underlying allocation, use `vec-to-unset`",
                type_parameters: vec![],
                parameter_type: type_vec(type_variable("Origin"), type_variable("Element")),
                result_type: type_record([]),
            },
            CoreFnInfo {
                name: "unset-slice-allocate-length",
                documentation: "Create a new `unset-slice` with a given length.
There rarely is a need to use this except when a function expects an `unset-slice` as an argument",
                type_parameters: vec![Name::from_static("Element")],
                parameter_type: type_u32,
                result_type: type_unset_slice(type_variable("Element")),
            },
            CoreFnInfo {
                name: "unset-slice-length",
                documentation: "How many elements could fit",
                type_parameters: vec![],
                parameter_type: type_unset_slice(type_variable("Element")),
                result_type: type_record([
                    ("slice", type_unset_slice(type_variable("Element"))),
                    ("length", type_u32),
                ]),
            },
            CoreFnInfo {
                name: "unset-slice-cast-or-rid-and-allocate",
                documentation: r#"Reinterpret the slice of unset bytes as a slice of a different element type.
This only works when the new element type has the same "size" (byte count including padding bits)
and equal "alignment" (byte count of the biggest field).
For example, `u32` consists of 4 bytes, just like `i32`, so they can be reinterpreted.
Similarly `.x f32 .y f32` can be reinterpreted as `.width i32 .height i32` and so forth.
If the types are incompatible, this function calls `unset-slice-rid` on the old slice
and allocates a new one with the same length as the scrapped one.

Note: This difference in behavior does not get reported at compile-time
which is a bit of a stinker. The reasons are:
- different compilation targets may have a different memory packing of sloe values,
  so we can never really guarantee this reinterpretation works
- it could be possible to fit different element sizes into the slice
  (if size of NewElement is multiple of unset-slice length).
  For example, 3 u32s could be transformed into 6 u16s and the other way around.
  For bigger sizes, this is only possible at runtime
- it could be possible to reinterpret element types with different alignments.
  For growing alignments, this is only possible at runtime
  when the slice pointer happens to align correctly
- sloe has a simple type system so I'm happy such a compromise can exist at all"#,
                type_parameters: vec![Name::from_static("NewElement")],
                parameter_type: type_unset_slice(type_variable("Element")),
                result_type: type_unset_slice(type_variable("NewElement"))
            },
            CoreFnInfo {
                name: "unset-slice-rid",
                documentation: "Deallocate an `unset-slice` in full. Very rarely useful.
```sloe
fn hand-warmer-in-debug-mode . :> . >
    _unset-slice-rid _unset-slice-allocate-length 9999999
```",
                type_parameters: vec![],
                parameter_type: type_unset_slice(type_variable("Element")),
                result_type: type_record([]),
            },
        ].map(|core_fn_info| {
            (
                Name::from_static(core_fn_info.name),
                CheckedProjectFn {
                    documentation: Some(Box::from(core_fn_info.documentation)),
                    type_parameters: core_fn_info.type_parameters,
                    parameter_type: Some(core_fn_info.parameter_type),
                    result_type: Some(core_fn_info.result_type),
                    result_expression_is_invalid: true
                },
            )
        }))
    });
pub static core_type_aliases: std::sync::LazyLock<
    std::collections::HashMap<Name, CheckedTypeAlias>,
> = std::sync::LazyLock::new(|| {
    std::collections::HashMap::from([
        (
            Name::from_static("p32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 1 (positive integer) with 32 bits.
```sloe
fn answer . :> p32 >
    _p32-add .p 2 p32 .u 40 u32
```",
                )),
                parameters: vec![],
                type_: Some(type_p32),
            },
        ),
        (
            Name::from_static("u32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 0 (unsigned integer) with 32 bits.
```sloe
fn answer . :> u32 >
    _u32-add .a 2 u32 .b 40 u32
```",
                )),
                parameters: vec![],
                type_: Some(type_u32),
            },
        ),
        (
            Name::from_static("i32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed whole number (integer) with 32 bits.
```sloe
fn answer . :> i32 >
    _i32-add .a -8 i32 .b 50 i32
```",
                )),
                parameters: vec![],
                type_: Some(type_i32),
            },
        ),
        (
            Name::from_static("f32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed decimal number (floating-point) with 32 bit precision.
Does not allow infinities or NaN. If you need these error states, explicitly model them with a choice type.
```sloe
fn answer . :> f32 >
    _f32-add .a -8.5 f32 .b 50.5 f32
```",
                )),
                parameters: vec![],
                type_: Some(type_f32),
            },
        ),
        (
            Name::from_static("char"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r#"A unicode scalar like `'a'` or `'👀'` or `'\u{2665}'` (hex code for ♥).
Keep in mind that a human-readable visual symbol can be composed of multiple such unicode scalars (forming a grapheme cluster), For example:
```sloe
_str-start "🇺🇸"
# = |present .start '\u{1F1FA}' .after "\u{1F1F8}"
#                   Indicator U        Indicator S
```
Read if interested: [swift's grapheme cluster docs](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/stringsandcharacters/#Extended-Grapheme-Clusters)"#,
                )),
                parameters: vec![],
                type_: Some(type_char),
            },
        ),
        (
            Name::from_static("str"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r#"Text valid for the entire program like `"abc"` or `"\"hello 👀 \\\r\n world \u{2665}\""` (`\u{2665}` represents the hex code for ♥, `\"` represents ", `\\` represents \\, `\n` represents line break, `\r` represents carriage return).
Internally, a string is compactly represented as UTF-8 bytes and can be accessed as such.
When building strings, use functions like `vec-char-opt-span-add-str`."#,
                )),
                parameters: vec![],
                type_: Some(type_str),
            },
        ),
        (
            Name::from_static("opt"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"Either you have some value or you have nothing.",
                )),
                parameters: vec![Name::from_static("A")],
                type_: Some(type_opt(type_variable("A"))),
            },
        ),
        (
            Name::from_static("origin"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "Each variable created with `origin some-origin expression` is of this type.
Origins can not be arbitrary values because values like `u32` could be duplicated leading to different collections with the same origin type.
This is not possible for values of type `origin`.
The type argument to an `origin` is the type that also gets created with `origin some-origin expression`.
This type argument is also used in slot, span, arena, vec as the first type argument."
                )),
                parameters: vec![Name::from_static("LocalOrigin")],
                type_: Some(type_origin(type_variable("LocalOrigin"))),
            },
        ),
        (
            Name::from_static("vec"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A grow- and shrinkable array of elements. Arrays have constant time access and update and constant time add.
```sloe
fn use-a-vec . u32
    origin my-elements-origin
    ? _vec-empty<u32> my-elements-origin [my-elements]
    ? _vec-add .vec my-elements .element 609 u32 [.vec my-elements .slot first-element-slot]
    ? _vec-remove .vec my-elements .slot first-element-slot [.vec my-elements .element first-element]
    ? vec-rid my-elements [.]
    first-element # = 609 u32
```"
                )),
                parameters: vec![Name::from_static("Origin"), Name::from_static("Element")],
                type_: Some(type_vec(type_variable("Origin"), type_variable("Element"))),
            },
        ),
        (
            Name::from_static("slot"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A valid position of an element in a collection.
This works because each collection has a unique origin and only gives out one slot for each position.
For consecutive `slot`s, check out `span`."
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_slot(type_variable("Origin"))),
            },
        ),
        (
            Name::from_static("span"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A range of ≥1 consecutive valid positions in a collection.
This works because each collection has a unique origin and only gives out one span for each range.
For potentially 0-length spans, use `_opt _span Origin`"
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_span(type_variable("Origin"))),
            },
        ),
        (
            Name::from_static("unset-slot"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "Like `slot` but referencing an unoccupied position.
It's similar to what languages use uninitialized memory/undefined for.
As this prevents another element from filling this position, you shouldn't keep it around for too long."
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_unset_slot(type_variable("Origin"))),
            },
        ),
        (
            Name::from_static("unset-span"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "Like `span` but referencing an unoccupied range.
It's similar to what languages use uninitialized memory/undefined for.
As this prevents other elements from filling these positions, you shouldn't keep it around for too long."
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_unset_span(type_variable("Origin"))),
            },
        ),
        (
            Name::from_static("array"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A stack-allocated array of known, positive length.
Arrays make adding multiple elements of the same type much less cumbersome,
see `vec-span-add-array`/`vec-opt-span-add-array`.
This is a very bare-bones feature. In fact, as of writing this you cannot even create an array."
                )),
                parameters: vec![Name::from_static("Element"), Name::from_static("Record")],
                type_: Some(type_array(type_variable("Element"), type_variable("Record"))),
            },
        ),
        (
            Name::from_static("unset-slice"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A heap-allocated array with unknown length and undefined contents.
Can be constructed manually or as an intermediate type when recycling the allocated space of a collection,
see `vec-to-unset`.
Since you can't read from it, you can also safely attempt to reuse this allocation for a different element type,
see `unset-slice-cast-or-rid-and-allocate`.
Note that `unset-slice` does not have a dup function to make heap allocation explicit.
Use `unset-slice-length` and `unset-slice-allocate-length` to achieve the same effect."
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_unset_span(type_variable("Origin"))),
            },
        ),
        (
            Name::from_static("fn"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A transformation from In to Out.
It cannot access local variables from the outside; everything must be passed in and out explicitly.
The parameter pattern must always have a known type.
Functions, even local ones are called as `_function argument`.
Functions values can be copied with `fn-dup` and scrapped with `fn-rid`.
This is only possible because functions do not have access to variables from the outside.
```sloe
fn three . :> . >
    ? ([n u32] _u32-add-clamp .a n .b 1 u32) [increment]
    _increment 2
```"
                )),
                parameters: vec![Name::from_static("In"), Name::from_static("Out")],
                type_: Some(type_fn(type_variable("In"), type_variable("Out"))),
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
    // possible optimization: change to cow
    pub message: Box<str>,
    pub range: lsp_types::Range,
}

#[must_use]
pub fn compiled_rust_to_file_content(rust_file: &syn::File, compiled_mod_name: &str) -> String {
    format!(
        "// jump to compiled code by searching for // compiled

{}


// compiled code //


{}",
        // I don't like this but I also haven't found any other way
        // to make a macro automatically adapt to the mod name it's placed in :(
        include_str!("core.rs").replacen("crate::core", &format!("crate::{compiled_mod_name}"), 1),
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
                    None => match type_ {
                        Some(type_) => {
                            space_or_linebreak_indented_into(
                                &mut formatted,
                                range_line_span(type_range(type_, types)),
                                next_indent(0),
                            );
                        }
                        None => {
                            formatted.push(' ');
                        }
                    },
                }
                if let Some(type_) = type_ {
                    syntax_type_unparenthesized_format(
                        &mut formatted,
                        next_indent(0),
                        types,
                        type_,
                    );
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
                match result_type {
                    Some(result_type) => {
                        let result_type_lne_span = range_line_span(type_range(result_type, types));
                        space_or_linebreak_indented_into(
                            &mut formatted,
                            result_type_lne_span,
                            next_indent(0),
                        );
                        syntax_type_unparenthesized_format(
                            &mut formatted,
                            next_indent(0),
                            types,
                            result_type,
                        );
                        space_or_linebreak_indented_into(
                            &mut formatted,
                            result_type_lne_span,
                            next_indent(0),
                        );
                    }
                    None => {
                        space_or_linebreak_indented_into(
                            &mut formatted,
                            header_line_span,
                            next_indent(0),
                        );
                    }
                }

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
                    );
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
    let mut names = angled_type_parameters.parameter0.iter().chain(
        angled_type_parameters
            .parameter1_up
            .iter()
            .filter_map(|parameter| parameter.name.as_ref()),
    );
    if let Some(name0) = names.next() {
        formatted.push_str(&name0.value);
        for name in names {
            formatted.push_str(", ");
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
struct OpenEndKinds {
    type_construct: bool,
    type_choice: bool,
    record: bool,
    expression_query: bool,
}
const no_open_end_kinds: OpenEndKinds = OpenEndKinds {
    type_construct: false,
    type_choice: false,
    record: false,
    expression_query: false,
};
fn syntax_expression_open_end<Expressions, Patterns, Types>(
    expression: &SyntaxExpression<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> OpenEndKinds {
    match expression {
        SyntaxExpression::Number { value: _, type_ } => match type_ {
            Some(type_) => syntax_type_open_end(type_, types),
            None => no_open_end_kinds,
        },
        SyntaxExpression::Char { .. } => no_open_end_kinds,
        SyntaxExpression::Str { .. } => no_open_end_kinds,
        SyntaxExpression::Variable(_) => no_open_end_kinds,
        SyntaxExpression::Call {
            underscore_start: _,
            name: _,
            type_arguments: _,
            argument,
        } => match argument {
            Some(argument) => {
                syntax_expression_open_end(expressions.element(argument), expressions, types)
            }
            None => no_open_end_kinds,
        },
        SyntaxExpression::Variant {
            name: _,
            type_: _,
            value,
        } => match value {
            Some(value) => {
                syntax_expression_open_end(expressions.element(value), expressions, types)
            }
            None => no_open_end_kinds,
        },
        SyntaxExpression::Fn {
            open_bracket_start: _,
            parameter: _,
            closed_bracket_start: _,
            result,
        } => match result {
            Some(result) => {
                syntax_expression_open_end(expressions.element(result), expressions, types)
            }
            None => no_open_end_kinds,
        },
        SyntaxExpression::RecordEmpty { dot_start: _ } => no_open_end_kinds,
        SyntaxExpression::Record { part0, part1_up } => {
            let last_field_open_end = match part1_up.last().unwrap_or(part0) {
                SyntaxRecordPart::Field { name: _, value } => match value {
                    Some(last_part_value) => syntax_expression_open_end(
                        expressions.element(last_part_value),
                        expressions,
                        types,
                    ),
                    None => no_open_end_kinds,
                },
                SyntaxRecordPart::Spread {
                    dot_dot_start: _,
                    record,
                } => match record {
                    Some(last_part_record) => syntax_expression_open_end(
                        expressions.element(last_part_record),
                        expressions,
                        types,
                    ),
                    None => no_open_end_kinds,
                },
            };
            OpenEndKinds {
                record: true,
                ..last_field_open_end
            }
        }
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            Some(inner) => {
                syntax_expression_open_end(expressions.element(inner), expressions, types)
            }
            None => no_open_end_kinds,
        },
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => match expression {
            Some(expression) => {
                syntax_expression_open_end(expressions.element(expression), expressions, types)
            }
            None => no_open_end_kinds,
        },
        SyntaxExpression::Query {
            question_mark_start: _,
            queried: _,
            cases,
        } => {
            let last_case_open_end = cases
                .last()
                .and_then(|last_case| last_case.result.as_ref())
                .map(|last_case_result| {
                    syntax_expression_open_end(last_case_result, expressions, types)
                })
                .unwrap_or(no_open_end_kinds);
            OpenEndKinds {
                expression_query: true,
                ..last_case_open_end
            }
        }
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name: _,
            result,
        } => match result {
            Some(result) => {
                syntax_expression_open_end(expressions.element(result), expressions, types)
            }
            None => no_open_end_kinds,
        },
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
                formatted.push_str(value.value.trim_start_matches("0"));
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
            match type_
                .as_ref()
                .and_then(|type_argument| type_argument.type_.as_ref())
            {
                None => {
                    formatted.push_str("<>");
                }
                Some(type_) => {
                    let line_span = range_line_span(lsp_types::Range {
                        start: name.start,
                        end: type_end(type_, types),
                    });
                    formatted.push('<');
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                    syntax_type_unparenthesized_format(
                        formatted,
                        next_indent(indent),
                        types,
                        type_,
                    );
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                    formatted.push('>');
                }
            }
            if let Some(value) = value {
                let value = expressions.element(value);
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(lsp_types::Range {
                        start: name.start,
                        end: expression_end(value, expressions, patterns, types),
                    }),
                    indent,
                );
                syntax_expression_unparenthesized_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    value,
                );
            }
        }
        SyntaxExpression::Fn {
            open_bracket_start: _,
            parameter,
            closed_bracket_start: _,
            result,
        } => {
            formatted.push_str("[");
            if let Some(parameter) = parameter {
                let parameter_line_span =
                    range_line_span(pattern_range(parameter, patterns, types));
                syntax_pattern_unparenthesized_format(
                    formatted, indent, patterns, types, parameter,
                );
                if parameter_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(']');
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
        SyntaxExpression::Record { part0, part1_up } => {
            let part_count = 1 + part1_up.len();
            syntax_expression_record_part_format(
                formatted,
                indent,
                expressions,
                patterns,
                types,
                part_count,
                part0,
                0,
            );
            let line_span =
                range_line_span(expression_range(expression, expressions, patterns, types));

            for (part_index, part) in part1_up.iter().enumerate().map(|(i, e)| (i + 1, e)) {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_expression_record_part_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    types,
                    part_count,
                    part,
                    part_index,
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
                        |open_end| open_end.expression_query,
                        syntax_expression_open_end(queried, expressions, types),
                        range_line_span(expression_range(queried, expressions, patterns, types)),
                    );
                }
            }
            match cases.as_slice() {
                [] => {
                    formatted.push_str(" [] ");
                }
                [case0, case1_up @ ..] => {
                    let line_span_before_last_case_pattern = range_line_span(lsp_types::Range {
                        start: *question_mark_start,
                        end: {
                            let last_case = case1_up.last().unwrap_or(case0);
                            last_case
                                .closed_bracket_start
                                .map(|closed_bracket_start| symbol_end(closed_bracket_start, "]"))
                                .or_else(|| {
                                    last_case
                                        .pattern
                                        .as_ref()
                                        .map(|pattern| pattern_end(pattern, patterns, types))
                                })
                                .unwrap_or_else(|| symbol_end(last_case.open_bracket_start, "["))
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
fn syntax_expression_record_part_format<Expressions, Patterns, Types>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    record_part_count: usize,
    record_part: &SyntaxRecordPart<Expressions>,
    record_part_index: usize,
) {
    match record_part {
        SyntaxRecordPart::Field { name, value } => {
            optional_field_name_format(formatted, name.value.as_ref());
            match value {
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
                        |open_end| open_end.record,
                        record_part_count,
                        record_part_index,
                        name.start,
                        expression_range(value, expressions, patterns, types),
                    );
                }
            }
        }
        SyntaxRecordPart::Spread {
            dot_dot_start,
            record,
        } => match record {
            None => {
                formatted.push_str(".. ");
            }
            Some(record) => {
                let record = expressions.element(record);
                if record_part_count == 1 {
                    syntax_expression_unparenthesized_format(
                        formatted,
                        indent,
                        expressions,
                        patterns,
                        types,
                        record,
                    );
                } else {
                    formatted.push_str("..");
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
                                record,
                            );
                        },
                        || syntax_expression_open_end(record, expressions, types),
                        |open_end| open_end.record,
                        record_part_count,
                        record_part_index,
                        *dot_dot_start,
                        expression_range(record, expressions, patterns, types),
                    );
                }
            }
        },
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
    formatted.push_str("[");
    if let Some(pattern) = &case.pattern {
        let pattern_line_span = range_line_span(pattern_range(pattern, patterns, types));
        syntax_pattern_unparenthesized_format(
            formatted,
            next_indent(indent),
            patterns,
            types,
            pattern,
        );
        if pattern_line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
    }
    formatted.push(']');
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
                |open_end| open_end.expression_query,
                case_count,
                case_index,
                match &case.pattern {
                    None => case.open_bracket_start,
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
    element_open_end: impl FnOnce() -> OpenEndKinds,
    open_end_kind_to_parenthesize_before_last_element: fn(OpenEndKinds) -> bool,
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
    element_open_end: impl FnOnce() -> OpenEndKinds,
    open_end_kind_to_parenthesize_before_last_element: fn(OpenEndKinds) -> bool,
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
    open_end_kind_to_parenthesize_before_last_element: fn(OpenEndKinds) -> bool,
    element_open_end: OpenEndKinds,
    line_span: LineSpan,
) {
    if open_end_kind_to_parenthesize_before_last_element(element_open_end) {
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
) -> OpenEndKinds {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => match type_ {
            Some(type_) => syntax_type_open_end(type_, types),
            None => no_open_end_kinds,
        },
        SyntaxPattern::Variant { name: _, value } => match value {
            Some(value) => syntax_pattern_open_end(patterns.element(value), patterns, types),
            None => no_open_end_kinds,
        },
        SyntaxPattern::RecordEmpty { dot_start: _ } => no_open_end_kinds,
        SyntaxPattern::Record { part0, part1_up } => {
            let last_field_open_end = match part1_up.last().unwrap_or_else(|| part0) {
                SyntaxRecordPart::Field { name: _, value } => match value {
                    None => no_open_end_kinds,
                    Some(value) => {
                        syntax_pattern_open_end(patterns.element(value), patterns, types)
                    }
                },
                SyntaxRecordPart::Spread {
                    dot_dot_start: _,
                    record,
                } => match record {
                    None => no_open_end_kinds,
                    Some(record) => {
                        syntax_pattern_open_end(patterns.element(record), patterns, types)
                    }
                },
            };
            OpenEndKinds {
                record: true,
                ..last_field_open_end
            }
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            Some(inner) => syntax_pattern_open_end(patterns.element(inner), patterns, types),
            None => no_open_end_kinds,
        },
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
        SyntaxPattern::Record { part0, part1_up } => {
            let part_count = 1 + part1_up.len();
            syntax_pattern_record_part_unparenthesized_format(
                formatted, indent, patterns, types, part_count, part0, 0,
            );
            let line_span = range_line_span(pattern_range(pattern, patterns, types));
            for (part_index, part) in part1_up.iter().enumerate().map(|(i, e)| (1 + i, e)) {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_pattern_record_part_unparenthesized_format(
                    formatted, indent, patterns, types, part_count, part, part_index,
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
fn syntax_pattern_record_part_unparenthesized_format<Types, Patterns>(
    formatted: &mut String,
    indent: usize,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    part_count: usize,
    part: &SyntaxRecordPart<Patterns>,
    part_index: usize,
) {
    match part {
        SyntaxRecordPart::Field { name, value } => {
            optional_field_name_format(formatted, name.value.as_ref());
            match value {
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
                        |open_end| open_end.record,
                        part_count,
                        part_index,
                        name.start,
                        pattern_range(value, patterns, types),
                    );
                }
            }
        }
        SyntaxRecordPart::Spread {
            dot_dot_start,
            record,
        } => {
            formatted.push_str("..");
            match record {
                None => {
                    formatted.push(' ');
                }
                Some(record) => {
                    let record = patterns.element(record);
                    maybe_open_end_whitespace_then_element_format(
                        formatted,
                        indent,
                        |formatted, indent| {
                            syntax_pattern_unparenthesized_format(
                                formatted, indent, patterns, types, record,
                            );
                        },
                        || syntax_pattern_open_end(record, patterns, types),
                        |open_end| open_end.record,
                        part_count,
                        part_index,
                        *dot_dot_start,
                        pattern_range(record, patterns, types),
                    );
                }
            }
        }
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
    if line_span == LineSpan::Multiple {
        linebreak_indented_into(formatted, indent);
    }
    let type_argument_count = 1 + angled_type_arguments.argument1_up.len();
    match &angled_type_arguments.argument0 {
        None => {}
        Some(argument0) => {
            if angled_type_arguments.argument1_up.is_empty() {
                syntax_type_unparenthesized_format(formatted, indent, types, argument0);
            } else {
                let argument0_line_span = range_line_span(type_range(argument0, types));
                if syntax_type_open_end(argument0, types).type_construct {
                    formatted.push('(');
                    if argument0_line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                    syntax_type_unparenthesized_format(
                        formatted,
                        next_indent(indent),
                        types,
                        argument0,
                    );
                    if argument0_line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                    formatted.push(')');
                } else {
                    syntax_type_unparenthesized_format(
                        formatted,
                        next_indent(indent),
                        types,
                        argument0,
                    );
                }
            }
        }
    }
    for (argument_index, argument) in angled_type_arguments
        .argument1_up
        .iter()
        .enumerate()
        .map(|(i, e)| (i + 1, e))
    {
        match &argument.type_ {
            None => {}
            Some(argument_type) => {
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(',');
                maybe_open_end_whitespace_then_element_last_always_unparenthesized_format(
                    formatted,
                    indent,
                    |formatted, indent| {
                        syntax_type_unparenthesized_format(formatted, indent, types, argument_type);
                    },
                    || syntax_type_open_end(argument_type, types),
                    |open_end| open_end.type_construct,
                    type_argument_count,
                    argument_index,
                    argument.comma_start,
                    type_range(argument_type, types),
                );
            }
        }
    }
    if line_span == LineSpan::Multiple {
        linebreak_indented_into(formatted, next_indent(indent));
    }
    formatted.push('>');
}
fn syntax_type_open_end<Types>(
    type_: &SyntaxType<Types>,
    types: &core::Vec<Types, SyntaxType<Types>>,
) -> OpenEndKinds {
    match type_ {
        SyntaxType::Variable(_) => no_open_end_kinds,
        SyntaxType::RecordEmpty { dot_start: _ } => no_open_end_kinds,
        SyntaxType::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            let last_field_open_end = field1_up
                .last()
                .map(|last_field| last_field.value.as_ref())
                .unwrap_or_else(|| {
                    field0_value
                        .as_ref()
                        .map(|field0_value| types.element(field0_value))
                })
                .map(|last_field_value| syntax_type_open_end(last_field_value, types))
                .unwrap_or(no_open_end_kinds);
            OpenEndKinds {
                record: true,
                ..last_field_open_end
            }
        }
        SyntaxType::ChoiceEmpty { bar_start: _ } => no_open_end_kinds,
        SyntaxType::Choice {
            variant0_name: _,
            variant0_value,
            variant1_up,
        } => {
            let last_variant_open_end = variant1_up
                .last()
                .map(|last_variant| last_variant.value.as_ref())
                .unwrap_or_else(|| {
                    variant0_value
                        .as_ref()
                        .map(|variant0_value| types.element(variant0_value))
                })
                .map(|last_variant_value| syntax_type_open_end(last_variant_value, types))
                .unwrap_or(no_open_end_kinds);
            OpenEndKinds {
                type_choice: true,
                ..last_variant_open_end
            }
        }
        SyntaxType::ConstructWithoutArguments(_) => no_open_end_kinds,
        SyntaxType::ConstructWithArguments {
            underscore_start: _,
            name: _,
            argument0,
            argument1_up,
        } => {
            let last_argument_open_end = argument1_up
                .last()
                .map(|last_argument| last_argument.type_.as_ref())
                .unwrap_or_else(|| argument0.as_ref().map(|argument0| types.element(argument0)))
                .map(|last_argument| syntax_type_open_end(last_argument, types))
                .unwrap_or(no_open_end_kinds);
            OpenEndKinds {
                type_construct: true,
                ..last_argument_open_end
            }
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            Some(inner) => syntax_type_open_end(types.element(inner), types),
            None => no_open_end_kinds,
        },
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
                        |open_end| open_end.type_construct,
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
                        |open_end| open_end.type_construct,
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
                        |open_end| open_end.record,
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
                            |open_end| open_end.record,
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
                        |open_end| open_end.type_choice,
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
                            |open_end| open_end.type_choice,
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
        construct_info: ConstructInfo,
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
        construct_info: ConstructInfo,
        pattern_variables: std::collections::HashMap<
            &'a Name,
            PatternVariableSymbolOrigin<'a, Expressions, Patterns, Types>,
        >,
        origins: std::collections::HashMap<
            &'a Name,
            OriginStartAndScope<'a, Expressions, Patterns, Types>,
        >,
    },
    PatternVariable {
        name: &'a Name,
        use_start: lsp_types::Position,
        origin: PatternVariableSymbolOrigin<'a, Expressions, Patterns, Types>,
    },
}
pub enum ConstructInfo {
    NotExpectingArgument,
    ArgumentMissing,
    ArgumentExists,
    Declaration,
}
pub struct PatternVariableSymbolOrigin<'a, Expressions, Patterns, Types> {
    pub start: lsp_types::Position,
    pub scope: Option<&'a SyntaxExpression<Expressions, Patterns, Types>>,
    pub type_: Option<Type>,
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
        *self
    }
}
pub fn project_symbol_at_position<'a, Expressions, Patterns, Types>(
    project: &'a SyntaxProject<Expressions, Patterns, Types>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
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
                    construct_info: ConstructInfo::Declaration,
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
                        .find_map(|parameter_name| {
                            if range_includes_position(
                                name_range(with_start_position_as_ref(parameter_name)),
                                position,
                            ) {
                                Some(SyntaxSymbol::TypeVariable {
                                    name: &parameter_name.value,
                                    use_start: parameter_name.start,
                                    scope: element,
                                })
                            } else {
                                None
                            }
                        })
                })
                .or_else(|| {
                    type_.as_ref().and_then(|value| {
                        type_symbol_at_position(
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
                    construct_info: ConstructInfo::Declaration,
                    pattern_variables: std::collections::HashMap::new(),
                    origins: std::collections::HashMap::new(),
                });
            }
            type_parameters
                .as_ref()
                .and_then(|type_parameters| {
                    angled_type_parameters_symbol_at_position(type_parameters, position, element)
                })
                .or_else(|| {
                    parameter.as_ref().and_then(|parameter| {
                        pattern_symbol_at_position(
                            parameter,
                            None,
                            position,
                            type_aliases,
                            checked_spread_records,
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
                        type_symbol_at_position(
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
                            syntax_pattern_untyped_variables_fold(
                                parameter,
                                (),
                                &mut |(), parameter_variable_name, parameter_variable_type| {
                                    pattern_variables.insert(
                                        parameter_variable_name.value,
                                        PatternVariableSymbolOrigin {
                                            start: parameter_variable_name.start,
                                            scope: Some(result),
                                            type_: parameter_variable_type.and_then(|type_| {
                                                syntax_type_to_type(
                                                    type_,
                                                    type_aliases,
                                                    types,
                                                    #[allow(clippy::zero_sized_map_values)]
                                                    &std::collections::HashMap::<&Name, ()>::new(),
                                                )
                                            }),
                                        },
                                    );
                                },
                                patterns,
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
                            expressions,
                            patterns,
                            types,
                            element,
                            &mut pattern_variables,
                            &mut std::collections::HashMap::new(),
                        )
                    })
                })
        }
        SyntaxProjectElement::Comments(_) => None,
        SyntaxProjectElement::Unrecognized { .. } => None,
    })
}
fn expression_symbol_at_position<'a, Expressions, Patterns, Types>(
    expression: &'a SyntaxExpression<Expressions, Patterns, Types>,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    pattern_variables: &mut std::collections::HashMap<
        &'a Name,
        PatternVariableSymbolOrigin<'a, Expressions, Patterns, Types>,
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
        SyntaxExpression::Number { value: _, type_ } => type_
            .as_ref()
            .and_then(|value| type_symbol_at_position(value, position, types, scope, origins)),
        SyntaxExpression::Char { .. } => None,
        SyntaxExpression::Str { .. } => None,
        SyntaxExpression::Variable(name) => Some(match pattern_variables.remove(&name.value) {
            Some(pattern_variable) => SyntaxSymbol::PatternVariable {
                name: &name.value,
                use_start: name.start,
                origin: pattern_variable,
            },
            None => SyntaxSymbol::ProjectFnOrUnknown {
                name: with_start_position_as_ref(name),
                construct_info: ConstructInfo::NotExpectingArgument,
                pattern_variables: std::mem::take(pattern_variables),
                origins: std::mem::take(origins),
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
                return Some(match pattern_variables.remove(&name.value) {
                    Some(pattern_variable) => SyntaxSymbol::PatternVariable {
                        name: &name.value,
                        use_start: name.start,
                        origin: pattern_variable,
                    },
                    None => SyntaxSymbol::ProjectFnOrUnknown {
                        name: with_start_position_as_ref(name),
                        construct_info: if argument.is_some() || type_arguments.is_some() {
                            ConstructInfo::ArgumentExists
                        } else {
                            ConstructInfo::ArgumentMissing
                        },
                        pattern_variables: std::mem::take(pattern_variables),
                        origins: std::mem::take(origins),
                    },
                });
            }
            type_arguments
                .as_ref()
                .and_then(|type_arguments| {
                    type_arguments
                        .argument0
                        .iter()
                        .chain(
                            type_arguments
                                .argument1_up
                                .iter()
                                .filter_map(|type_argument| type_argument.type_.as_ref()),
                        )
                        .find_map(|type_argument| {
                            type_symbol_at_position(type_argument, position, types, scope, origins)
                        })
                })
                .or_else(|| {
                    argument.as_ref().and_then(|argument| {
                        expression_symbol_at_position(
                            expressions.element(argument),
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
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
                .and_then(|type_argument| type_argument.type_.as_ref())
                .and_then(|type_| type_symbol_at_position(type_, position, types, scope, origins))
                .or_else(|| {
                    value.as_ref().and_then(|value| {
                        expression_symbol_at_position(
                            expressions.element(value),
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
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
            open_bracket_start: _,
            parameter,
            closed_bracket_start: _,
            result,
        } => {
            let result = result.as_ref().map(|result| expressions.element(result));
            pattern_variables.clear();
            parameter
                .as_ref()
                .and_then(|parameter| {
                    pattern_symbol_at_position(
                        parameter,
                        None,
                        position,
                        type_aliases,
                        checked_spread_records,
                        patterns,
                        types,
                        scope,
                        result,
                        origins,
                    )
                })
                .or_else(|| {
                    result.as_ref().and_then(|result| {
                        if let Some(parameter) = parameter {
                            syntax_pattern_untyped_variables_fold(
                                parameter,
                                (),
                                &mut |(), name, type_| {
                                    pattern_variables.insert(
                                        name.value,
                                        PatternVariableSymbolOrigin {
                                            start: name.start,
                                            scope: Some(result),
                                            type_: type_.and_then(|type_| {
                                                syntax_type_to_type(
                                                    type_,
                                                    type_aliases,
                                                    types,
                                                    origins,
                                                )
                                            }),
                                        },
                                    );
                                },
                                patterns,
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
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
        SyntaxExpression::Record { part0, part1_up } => std::iter::once(part0)
            .chain(part1_up)
            .find_map(|part| match part {
                SyntaxRecordPart::Field { name: _, value } => value.as_ref().and_then(|value| {
                    expression_symbol_at_position(
                        expressions.element(value),
                        position,
                        type_aliases,
                        checked_queries,
                        checked_spread_records,
                        expressions,
                        patterns,
                        types,
                        scope,
                        pattern_variables,
                        origins,
                    )
                }),
                SyntaxRecordPart::Spread {
                    dot_dot_start: _,
                    record,
                } => record.as_ref().and_then(|record| {
                    expression_symbol_at_position(
                        expressions.element(record),
                        position,
                        type_aliases,
                        checked_queries,
                        checked_spread_records,
                        expressions,
                        patterns,
                        types,
                        scope,
                        pattern_variables,
                        origins,
                    )
                }),
            }),
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().and_then(|inner| {
            expression_symbol_at_position(
                expressions.element(inner),
                position,
                type_aliases,
                checked_queries,
                checked_spread_records,
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
            expression_symbol_at_position(
                expressions.element(expression),
                position,
                type_aliases,
                checked_queries,
                checked_spread_records,
                expressions,
                patterns,
                types,
                scope,
                pattern_variables,
                origins,
            )
        }),
        SyntaxExpression::Query {
            question_mark_start,
            queried,
            cases,
        } => queried
            .as_ref()
            .and_then(|queried| {
                expression_symbol_at_position(
                    expressions.element(queried),
                    position,
                    type_aliases,
                    checked_queries,
                    checked_spread_records,
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
                    let checked_query = checked_queries.get(question_mark_start);
                    expression_query_case_symbol_at_position(
                        case,
                        checked_query.map(|checked_query| &checked_query.queried_type),
                        position,
                        type_aliases,
                        checked_queries,
                        checked_spread_records,
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
                if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                    return Some(SyntaxSymbol::Origin {
                        name: &name.value,
                        use_start: name.start,
                        origin: origin_info,
                    });
                }
                origins.insert(&name.value, origin_info);
            }
            result.as_ref().and_then(|result| {
                expression_symbol_at_position(
                    result,
                    position,
                    type_aliases,
                    checked_queries,
                    checked_spread_records,
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
fn expression_query_case_symbol_at_position<'a, Expressions, Patterns, Types>(
    case: &'a SyntaxExpressionQueryCase<Expressions, Patterns, Types>,
    expected_pattern_type: Option<&Type>,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    expressions: &'a core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &'a core::Vec<Types, SyntaxType<Types>>,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
    pattern_variables: &mut std::collections::HashMap<
        &'a Name,
        PatternVariableSymbolOrigin<'a, Expressions, Patterns, Types>,
    >,
    origins: &mut std::collections::HashMap<
        &'a Name,
        OriginStartAndScope<'a, Expressions, Patterns, Types>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    case.pattern
        .as_ref()
        .and_then(|pattern| {
            pattern_symbol_at_position(
                pattern,
                expected_pattern_type,
                position,
                type_aliases,
                checked_spread_records,
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
                syntax_pattern_typed_variables_fold(
                    pattern,
                    expected_pattern_type,
                    (),
                    &mut |(), name, type_| {
                        pattern_variables.insert(
                            name.value,
                            PatternVariableSymbolOrigin {
                                start: name.start,
                                scope: Some(result),
                                type_: type_,
                            },
                        );
                    },
                    checked_spread_records,
                    patterns,
                );
            }
            expression_symbol_at_position(
                result,
                position,
                type_aliases,
                checked_queries,
                checked_spread_records,
                expressions,
                patterns,
                types,
                scope,
                pattern_variables,
                origins,
            )
        })
}
fn syntax_pattern_untyped_variables_fold<'a, Patterns, Types, State>(
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
            Some(value) => syntax_pattern_untyped_variables_fold(
                patterns.element(value),
                state,
                reduce,
                patterns,
            ),
        },
        SyntaxPattern::RecordEmpty { dot_start: _ } => state,
        SyntaxPattern::Record { part0, part1_up } => {
            std::iter::once(part0)
                .chain(part1_up)
                .fold(state, |state, part| match part {
                    SyntaxRecordPart::Field { name: _, value } => match value {
                        None => state,
                        Some(value) => syntax_pattern_untyped_variables_fold(
                            patterns.element(value),
                            state,
                            reduce,
                            patterns,
                        ),
                    },
                    SyntaxRecordPart::Spread {
                        dot_dot_start: _,
                        record,
                    } => match record {
                        None => state,
                        Some(record) => syntax_pattern_untyped_variables_fold(
                            patterns.element(record),
                            state,
                            reduce,
                            patterns,
                        ),
                    },
                })
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => state,
            Some(inner) => syntax_pattern_untyped_variables_fold(
                patterns.element(inner),
                state,
                reduce,
                patterns,
            ),
        },
    }
}
fn syntax_pattern_typed_variables_fold<'a, Patterns, Types, State>(
    pattern: &'a SyntaxPattern<Patterns, Types>,
    expected_type: Option<&Type>,
    state: State,
    reduce: &mut impl FnMut(State, WithStartPosition<&'a Name>, Option<Type>) -> State,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    patterns: &'a core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
) -> State {
    match pattern {
        SyntaxPattern::Variable { name, type_: _ } => reduce(
            state,
            with_start_position_as_ref(name),
            expected_type.cloned(),
        ),
        SyntaxPattern::Variant { name, value } => match value {
            None => state,
            Some(value) => syntax_pattern_typed_variables_fold(
                patterns.element(value),
                expected_type
                    .and_then(|expected_type| match expected_type {
                        Type::Choice(expected_variants) => {
                            expected_variants.iter().find(|expected_variant| {
                                name.value
                                    .as_ref()
                                    .is_some_and(|name_value| name_value == &expected_variant.name)
                            })
                        }
                        _ => None,
                    })
                    .map(|variant| &variant.value),
                state,
                reduce,
                checked_spread_records,
                patterns,
            ),
        },
        SyntaxPattern::RecordEmpty { dot_start: _ } => state,
        SyntaxPattern::Record { part0, part1_up } => {
            std::iter::once(part0)
                .chain(part1_up)
                .fold(state, |state, part| match part {
                    SyntaxRecordPart::Field { name, value } => match value {
                        None => state,
                        Some(value) => syntax_pattern_typed_variables_fold(
                            patterns.element(value),
                            expected_type
                                .and_then(|expected_type| match expected_type {
                                    Type::Record(expected_fields) => {
                                        expected_fields.iter().find(|expected_field| {
                                            name.value.as_ref().is_some_and(|field_name_value| {
                                                field_name_value == &expected_field.name
                                            })
                                        })
                                    }
                                    _ => None,
                                })
                                .map(|field| &field.value),
                            state,
                            reduce,
                            checked_spread_records,
                            patterns,
                        ),
                    },
                    SyntaxRecordPart::Spread {
                        dot_dot_start,
                        record,
                    } => match record {
                        None => state,
                        Some(record) => syntax_pattern_typed_variables_fold(
                            patterns.element(record),
                            expected_type
                                .and_then(|expected_type| match expected_type {
                                    Type::Record(expected_fields) => checked_spread_records
                                        .get(dot_dot_start)
                                        .map(|checked_spread_record_fields| {
                                            Type::Record(
                                                expected_fields
                                                    .iter()
                                                    .filter(|expected_field| {
                                                        checked_spread_record_fields
                                                            .contains(&expected_field.name)
                                                    })
                                                    .cloned()
                                                    .collect::<Vec<TypeField>>(),
                                            )
                                        }),
                                    _ => None,
                                })
                                .as_ref(),
                            state,
                            reduce,
                            checked_spread_records,
                            patterns,
                        ),
                    },
                })
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => match inner {
            None => state,
            Some(inner) => syntax_pattern_typed_variables_fold(
                patterns.element(inner),
                expected_type,
                state,
                reduce,
                checked_spread_records,
                patterns,
            ),
        },
    }
}
fn pattern_symbol_at_position<'a, Expressions, Patterns, Types>(
    pattern: &'a SyntaxPattern<Patterns, Types>,
    expected_type: Option<&Type>,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
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
                    origin: PatternVariableSymbolOrigin {
                        start: name.start,
                        scope: expression_scope,
                        type_: expected_type.cloned().or_else(|| {
                            type_.as_ref().and_then(|type_| {
                                syntax_type_to_type(type_, type_aliases, types, origins)
                            })
                        }),
                    },
                });
            }
            type_.as_ref().and_then(|type_| {
                type_symbol_at_position(type_, position, types, project_element_scope, origins)
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
                pattern_symbol_at_position(
                    patterns.element(value),
                    expected_type
                        .and_then(|expected_type| match expected_type {
                            Type::Choice(expected_variants) => {
                                expected_variants.iter().find(|expected_variant| {
                                    name.value.as_ref().is_some_and(|name_value| {
                                        name_value == &expected_variant.name
                                    })
                                })
                            }
                            _ => None,
                        })
                        .map(|variant| &variant.value),
                    position,
                    type_aliases,
                    checked_spread_records,
                    patterns,
                    types,
                    project_element_scope,
                    expression_scope,
                    origins,
                )
            })
        }
        SyntaxPattern::RecordEmpty { dot_start: _ } => None,
        SyntaxPattern::Record { part0, part1_up } => std::iter::once(part0)
            .chain(part1_up)
            .find_map(|part| match part {
                SyntaxRecordPart::Field { name, value } => value.as_ref().and_then(|value| {
                    pattern_symbol_at_position(
                        patterns.element(value),
                        expected_type
                            .and_then(|expected_type| match expected_type {
                                Type::Record(expected_fields) => {
                                    expected_fields.iter().find(|expected_field| {
                                        name.value.as_ref().is_some_and(|field_name_value| {
                                            field_name_value == &expected_field.name
                                        })
                                    })
                                }
                                _ => None,
                            })
                            .map(|field| &field.value),
                        position,
                        type_aliases,
                        checked_spread_records,
                        patterns,
                        types,
                        project_element_scope,
                        expression_scope,
                        origins,
                    )
                }),
                SyntaxRecordPart::Spread {
                    dot_dot_start,
                    record,
                } => record.as_ref().and_then(|record| {
                    pattern_symbol_at_position(
                        patterns.element(record),
                        expected_type
                            .and_then(|expected_type| match expected_type {
                                Type::Record(expected_fields) => checked_spread_records
                                    .get(dot_dot_start)
                                    .map(|checked_spread_record_fields| {
                                        Type::Record(
                                            expected_fields
                                                .iter()
                                                .filter(|expected_field| {
                                                    checked_spread_record_fields
                                                        .contains(&expected_field.name)
                                                })
                                                .cloned()
                                                .collect::<Vec<TypeField>>(),
                                        )
                                    }),
                                _ => None,
                            })
                            .as_ref(),
                        position,
                        type_aliases,
                        checked_spread_records,
                        patterns,
                        types,
                        project_element_scope,
                        expression_scope,
                        origins,
                    )
                }),
            }),
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().and_then(|inner| {
            pattern_symbol_at_position(
                patterns.element(inner),
                expected_type,
                position,
                type_aliases,
                checked_spread_records,
                patterns,
                types,
                project_element_scope,
                expression_scope,
                origins,
            )
        }),
    }
}
fn type_symbol_at_position<'a, Expressions, Patterns, Types>(
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
                construct_info: ConstructInfo::NotExpectingArgument,
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
                    construct_info: if argument0.is_some() || !argument1_up.is_empty() {
                        ConstructInfo::ArgumentExists
                    } else {
                        ConstructInfo::ArgumentMissing
                    },
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
                    type_symbol_at_position(argument, position, types, scope, origins)
                })
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => inner.as_ref().and_then(|inner| {
            type_symbol_at_position(types.element(inner), position, types, scope, origins)
        }),
        SyntaxType::RecordEmpty { dot_start: _ } => None,
        SyntaxType::Record {
            field0_name,
            field0_value,
            field1_up,
        } => fields_find_symbol_at_position(
            with_start_position_as_ref(field0_name),
            field0_value.as_ref().map(|value| types.element(value)),
            field1_up,
            |_, value| type_symbol_at_position(value, position, types, scope, origins),
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
            .find_map(|value| type_symbol_at_position(value, position, types, scope, origins)),
    }
}
fn angled_type_parameters_symbol_at_position<'a, Expressions, Patterns, Types>(
    angled_type_parameters: &'a SyntaxAngledTypeParameters,
    position: lsp_types::Position,
    scope: &'a SyntaxProjectElement<Expressions, Patterns, Types>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    angled_type_parameters
        .parameter0
        .iter()
        .chain(
            angled_type_parameters
                .parameter1_up
                .iter()
                .filter_map(|parameter| parameter.name.as_ref()),
        )
        .find_map(|name| {
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
fn fields_find_symbol_at_position<'a, Value, Expressions, Patterns, Types>(
    field0_name: WithStartPosition<&Name>,
    field0_value: Option<&'a Value>,
    field1_up: &'a [SyntaxTrailingField<Value>],
    mut value_symbol_at_position: impl FnMut(
        Option<&Name>,
        &'a Value,
    )
        -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns, Types>> {
    field0_value
        .into_iter()
        .map(|value| (Some(field0_name.value), value))
        .chain(field1_up.iter().filter_map(|field| {
            field
                .value
                .as_ref()
                .map(|value| (field.name.value.as_ref(), value))
        }))
        .find_map(|(field_name, field_value)| value_symbol_at_position(field_name, field_value))
}

#[must_use]
pub fn syntax_project_symbol_origin_range<Expressions, Patterns, Types>(
    project: &SyntaxProject<Expressions, Patterns, Types>,
    symbol: &SyntaxSymbol<Expressions, Patterns, Types>,
) -> Option<lsp_types::Range> {
    match symbol {
        SyntaxSymbol::ProjectTypeOrUnknown {
            name: symbol_name,
            construct_info: _,
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
                    && &type_alias_name.value == symbol_name.value
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
        &SyntaxSymbol::TypeVariable {
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
            } => type_parameters.as_ref().and_then(|type_parameters| {
                type_parameters
                    .parameter0
                    .iter()
                    .chain(
                        type_parameters
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
            SyntaxProjectElement::Comments(_) | SyntaxProjectElement::Unrecognized { .. } => None,
        },
        SyntaxSymbol::VariantOrUnknown(_) => None,
        SyntaxSymbol::ProjectFnOrUnknown {
            name: symbol_name,
            construct_info: _,
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
            } if &fn_name.value == symbol_name.value => {
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
                );
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
                                syntax_pattern_untyped_variables_fold(
                                    parameter,
                                    (),
                                    &mut |(), parameter_introduced_variable_name, _type_| {
                                        parameter_introduced_variables
                                            .insert(parameter_introduced_variable_name.value);
                                    },
                                    patterns,
                                );
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
            if let &SyntaxSymbol::TypeVariable {
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
            if let &SyntaxSymbol::ProjectTypeOrUnknown {
                name: symbol_name,
                construct_info: _,
                origins: _,
            } = symbol
                && &name.value == symbol_name.value
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
                    construct_info: _,
                    origins: _,
                } = symbol
                && &name.value == symbol_name.value
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
                syntax_type_symbol_uses_into(uses, types.element(inner), symbol, types, origins);
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
        SyntaxPattern::Record { part0, part1_up } => {
            for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name: _, value } => {
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
                    SyntaxRecordPart::Spread {
                        dot_dot_start: _,
                        record,
                    } => {
                        if let Some(record) = record {
                            syntax_pattern_symbol_uses_into(
                                uses,
                                patterns.element(record),
                                symbol,
                                patterns,
                                types,
                                origins,
                            );
                        }
                    }
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
                construct_info: _,
                pattern_variables: _,
                origins: _,
            } => {
                if *symbol_name == &name.value
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
                        construct_info: _,
                        pattern_variables: _,
                        origins: _,
                    } => {
                        if *symbol_name == &name.value
                            && !pattern_variables.contains(&name.value)
                            && !origins.contains(&name.value)
                        {
                            uses.push(name_range(with_start_position_as_ref(name)));
                        }
                    }
                }
            }
            if let Some(type_arguments) = type_arguments {
                for type_argument in type_arguments.argument0.iter().chain(
                    type_arguments
                        .argument1_up
                        .iter()
                        .filter_map(|type_argument| type_argument.type_.as_ref()),
                ) {
                    syntax_type_symbol_uses_into(uses, type_argument, symbol, types, origins);
                }
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
            type_: type_argument,
            value,
        } => {
            if let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && let Some(name_value) = &name.value
                && name_value == symbol_name.value
            {
                uses.push(optional_variant_name_range(name));
            }
            if let Some(type_argument) = type_argument
                && let Some(type_) = &type_argument.type_
            {
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
            open_bracket_start: _,
            parameter,
            closed_bracket_start: _,
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
                    syntax_pattern_untyped_variables_fold(
                        parameter,
                        (),
                        &mut |(), pattern_variable_name, _type_| {
                            parameter_pattern_variables
                                .to_mut()
                                .insert(pattern_variable_name.value);
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
        SyntaxExpression::Record { part0, part1_up } => {
            for part in std::iter::once(part0).chain(part1_up.iter()) {
                syntax_expression_record_part_symbol_uses_into(
                    uses,
                    part,
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
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
                        syntax_pattern_untyped_variables_fold(
                            pattern,
                            (),
                            &mut |(), pattern_variable_name, _type_| {
                                pattern_variables
                                    .to_mut()
                                    .insert(pattern_variable_name.value);
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
                        && *symbol_name == &introduced_origin_name.value
                    {
                        return;
                    }
                    origins.to_mut().insert(&introduced_origin_name.value);
                }
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
fn syntax_expression_record_part_symbol_uses_into<Expressions, Patterns, Types>(
    uses: &mut Vec<lsp_types::Range>,
    part: &SyntaxRecordPart<Expressions>,
    symbol: &SyntaxSymbol<Expressions, Patterns, Types>,
    expressions: &core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &core::Vec<Types, SyntaxType<Types>>,
    pattern_variables: &std::collections::HashSet<&Name>,
    origins: &std::collections::HashSet<&Name>,
) {
    match part {
        SyntaxRecordPart::Field { name: _, value } => {
            if let Some(field_value) = value {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(field_value),
                    symbol,
                    expressions,
                    patterns,
                    types,
                    pattern_variables,
                    origins,
                );
            }
        }
        SyntaxRecordPart::Spread {
            dot_dot_start: _,
            record,
        } => {
            if let Some(record) = record {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.element(record),
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
}
pub fn syntax_project_element_rid<Expressions, Patterns, Types>(
    element: SyntaxProjectElement<Expressions, Patterns, Types>,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) {
    match element {
        SyntaxProjectElement::TypeAlias {
            ty_keyword_start: _,
            name: _,
            parameters: _,
            documentation: _,
            type_,
        } => {
            if let Some(type_) = type_ {
                syntax_type_rid(type_, types);
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
            if let Some(result_type) = result_type {
                syntax_type_rid(result_type, types);
            }
            if let Some(parameter) = parameter {
                syntax_pattern_rid(parameter, patterns, types);
            }
            if let Some(result) = result {
                syntax_expression_rid(result, expressions, patterns, types);
            }
        }
        SyntaxProjectElement::Comments(_) => {}
        SyntaxProjectElement::Unrecognized {
            range: _,
            source: _,
        } => {}
    }
}
fn syntax_type_rid<Types>(
    type_: SyntaxType<Types>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) {
    match type_ {
        SyntaxType::Variable(_) => {}
        SyntaxType::ConstructWithoutArguments(_) => {}
        SyntaxType::ConstructWithArguments {
            underscore_start: _,
            name: _,
            argument0,
            argument1_up,
        } => {
            if let Some(argument0) = argument0 {
                syntax_type_rid(types.remove(argument0), types);
            }
            for SyntaxTypeConstructTrailingArgument {
                comma_start: _,
                type_: argument_type,
            } in argument1_up
            {
                if let Some(argument_type) = argument_type {
                    syntax_type_rid(argument_type, types);
                }
            }
        }
        SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_type_rid(types.remove(inner), types);
            }
        }
        SyntaxType::RecordEmpty { dot_start: _ } => {}
        SyntaxType::Record {
            field0_name: _,
            field0_value,
            field1_up,
        } => {
            if let Some(field0_value) = field0_value {
                syntax_type_rid(types.remove(field0_value), types);
            }
            for SyntaxTrailingField { name: _, value } in field1_up {
                if let Some(value) = value {
                    syntax_type_rid(value, types);
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
                syntax_type_rid(types.remove(variant0_value), types);
            }
            for SyntaxTypeTrailingVariant { name: _, value } in variant1_up {
                if let Some(value) = value {
                    syntax_type_rid(value, types);
                }
            }
        }
    }
}
fn syntax_pattern_rid<Patterns, Types>(
    pattern: SyntaxPattern<Patterns, Types>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => {
            if let Some(type_) = type_ {
                syntax_type_rid(type_, types);
            }
        }
        SyntaxPattern::Variant { name: _, value } => {
            if let Some(value) = value {
                syntax_pattern_rid(patterns.remove(value), patterns, types);
            }
        }
        SyntaxPattern::RecordEmpty { dot_start: _ } => {}
        SyntaxPattern::Record { part0, part1_up } => {
            for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name: _, value } => {
                        if let Some(value) = value {
                            syntax_pattern_rid(patterns.remove(value), patterns, types);
                        }
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start: _,
                        record,
                    } => {
                        if let Some(record) = record {
                            syntax_pattern_rid(patterns.remove(record), patterns, types);
                        }
                    }
                }
            }
        }
        SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_pattern_rid(patterns.remove(inner), patterns, types);
            }
        }
    }
}
fn syntax_expression_rid<Expressions, Patterns, Types>(
    expression: SyntaxExpression<Expressions, Patterns, Types>,
    expressions: &mut core::Vec<Expressions, SyntaxExpression<Expressions, Patterns, Types>>,
    patterns: &mut core::Vec<Patterns, SyntaxPattern<Patterns, Types>>,
    types: &mut core::Vec<Types, SyntaxType<Types>>,
) {
    match expression {
        SyntaxExpression::Number { value: _, type_ } => {
            if let Some(type_) = type_ {
                syntax_type_rid(type_, types);
            }
        }
        SyntaxExpression::Char {
            open_quote_start: _,
            content: _,
            content_end: _,
            closed_quote_exists: _,
        } => {}
        SyntaxExpression::Str {
            open_quote_start: _,
            content: _,
            content_end: _,
            closed_quote_exists: _,
        } => {}
        SyntaxExpression::Variable(_) => {}
        SyntaxExpression::Call {
            underscore_start: _,
            name: _,
            type_arguments,
            argument,
        } => {
            if let Some(SyntaxAngledTypeArguments {
                open_angle_start: _,
                argument0,
                argument1_up,
                closed_angle_start: _,
            }) = type_arguments
            {
                if let Some(argument0) = argument0 {
                    syntax_type_rid(argument0, types);
                }
                for SyntaxTrailingTypeArgument {
                    comma_start: _,
                    type_: argument_type,
                } in argument1_up
                {
                    if let Some(argument_type) = argument_type {
                        syntax_type_rid(argument_type, types);
                    }
                }
            }
            if let Some(argument) = argument {
                syntax_expression_rid(expressions.remove(argument), expressions, patterns, types);
            }
        }
        SyntaxExpression::Variant {
            name: _,
            type_,
            value,
        } => {
            if let Some(SyntaxAngledTypeArgument {
                open_angle_start: _,
                type_: Some(type_),
                closed_angle_start: _,
            }) = type_
            {
                syntax_type_rid(type_, types);
            }
            if let Some(value) = value {
                syntax_expression_rid(expressions.remove(value), expressions, patterns, types);
            }
        }
        SyntaxExpression::Fn {
            open_bracket_start: _,
            parameter,
            closed_bracket_start: _,
            result,
        } => {
            if let Some(paramter) = parameter {
                syntax_pattern_rid(paramter, patterns, types);
            }
            if let Some(result) = result {
                syntax_expression_rid(expressions.remove(result), expressions, patterns, types);
            }
        }
        SyntaxExpression::RecordEmpty { dot_start: _ } => {}
        SyntaxExpression::Record { part0, part1_up } => {
            for part in std::iter::once(part0).chain(part1_up) {
                match part {
                    SyntaxRecordPart::Field { name: _, value } => {
                        if let Some(value) = value {
                            syntax_expression_rid(
                                expressions.remove(value),
                                expressions,
                                patterns,
                                types,
                            );
                        }
                    }
                    SyntaxRecordPart::Spread {
                        dot_dot_start: _,
                        record,
                    } => {
                        if let Some(record) = record {
                            syntax_expression_rid(
                                expressions.remove(record),
                                expressions,
                                patterns,
                                types,
                            );
                        }
                    }
                }
            }
        }
        SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                syntax_expression_rid(expressions.remove(inner), expressions, patterns, types);
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression: after_comments,
        } => {
            if let Some(after_comments) = after_comments {
                syntax_expression_rid(
                    expressions.remove(after_comments),
                    expressions,
                    patterns,
                    types,
                );
            }
        }
        SyntaxExpression::Query {
            question_mark_start: _,
            queried,
            cases,
        } => {
            if let Some(queried) = queried {
                syntax_expression_rid(expressions.remove(queried), expressions, patterns, types);
            }
            for SyntaxExpressionQueryCase {
                open_bracket_start: _,
                pattern,
                closed_bracket_start: _,
                result,
            } in cases
            {
                if let Some(pattern) = pattern {
                    syntax_pattern_rid(pattern, patterns, types);
                }
                if let Some(result) = result {
                    syntax_expression_rid(result, expressions, patterns, types);
                }
            }
        }
        SyntaxExpression::Origin {
            origin_keyword_start: _,
            name: _,
            result,
        } => {
            if let Some(result) = result {
                syntax_expression_rid(expressions.remove(result), expressions, patterns, types);
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
    format!("{}:{}", lsp_position.line + 1, lsp_position.character + 1)
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
