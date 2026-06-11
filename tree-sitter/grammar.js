/**
 * @file sloe grammar for tree-sitter
 * @author lue-bird
 * @license Unlicense
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

// TODO split _not_open_ended into versions for type arguments, fields, type variants and query cases

export default grammar({
  name: "sloe",
  extras: ($) => [/\s/],
  rules: {
    source_file: ($) =>
      seq(
        // the first project project element / comment lines are not guaranteed to be formatted with linebreaks in front
        $.project_element,
        // this is more strict than sloe's parser but works for formatted code
        repeat(seq("\n\n", $.project_element)),
      ),
    comment: ($) => /#[^\n]*\n/,

    project_element: ($) => choice(repeat1($.comment), $.project_fn, $.type_alias),

    type_alias: ($) =>
      seq(
        $.keyword_ty,
        $.type_name,
        repeat($.type_variable),
        repeat($.comment),
        $.type_not_variable,
      ),

    project_fn: ($) =>
      seq(
        $.keyword_fn,
        $.lower_name,
        optional($.angled_type_parameters),
        $.pattern_typed,
        $.key_symbol_arrow,
        $.type,
        $.key_symbol_angle_right,
        // no repeat($.comment), these can already be prepended to the resulting expression
        // which is syntactically equivalent
        $.expression,
      ),

    expression: ($) =>
      choice(
        $.expression_parenthesized,
        $.expression_commented,
        $.expression_number,
        $.string,
        $.char,
        $.expression_variable,
        $.expression_call,
        $.expression_origin,
        $.expression_variant,
        $.expression_record,
        $.expression_fn,
        $.expression_query,
        $.expression_origin,
      ),
    expression_not_open_ended: ($) =>
      choice(
        $.expression_parenthesized,
        $.string,
        $.char,
        $.expression_variable,
        $.expression_call_not_open_ended,
        $.expression_number_not_open_ended,
        $.expression_record_empty,
        $.expression_fn_not_open_ended,
        $.expression_origin_not_open_ended,
        $.expression_variant_not_open_ended,
        $.expression_commented_not_open_ended,
      ),
    expression_parenthesized: ($) => seq("(", $.expression, ")"),
    expression_commented: ($) =>
      seq(
        // semantically repeat1($.comment) but for simplicity the syntactic equivalent
        $.comment,
        $.expression,
      ),
    expression_commented_not_open_ended: ($) =>
      seq($.comment, $.expression_not_open_ended),
    expression_number: ($) => seq($.number, $.type),
    expression_number_not_open_ended: ($) => seq($.number, $.type_not_open_ended),
    expression_origin: ($) => seq($.keyword_origin, $.lower_name, $.expression),
    expression_origin_not_open_ended: ($) =>
      seq($.keyword_origin, $.lower_name, $.expression_not_open_ended),
    expression_variant: ($) => seq($.variant_name, $.type_not_open_ended, $.expression),
    expression_variant_not_open_ended: ($) =>
      seq($.variant_name, $.type_not_open_ended, $.expression_not_open_ended),
    expression_variable: ($) => $.lower_name,
    expression_call: ($) =>
      seq(
        $.symbol_call_underscore,
        $.expression_variable,
        optional($.angled_type_arguments),
        optional($.expression),
      ),
    expression_call_not_open_ended: ($) =>
      seq(
        $.symbol_call_underscore,
        $.expression_variable,
        optional($.angled_type_arguments),
        optional($.expression_not_open_ended),
      ),
    angled_type_arguments: ($) => seq("<", repeat($.type_not_open_ended), ">"),
    angled_type_parameters: ($) => seq("<", repeat($.type_variable), ">"),
    expression_query: ($) =>
      seq(
        $.key_symbol_colon,
        $.expression_not_open_ended,
        repeat($.expression_query_case_not_open_ended),
        $.expression_query_case,
      ),
    expression_query_case: ($) =>
      seq($.key_symbol_equals, $.pattern_untyped, $.key_symbol_angle_right, $.expression),
    expression_query_case_not_open_ended: ($) =>
      seq(
        $.key_symbol_equals,
        $.pattern_untyped,
        $.key_symbol_angle_right,
        $.expression_not_open_ended,
      ),
    expression_record_empty: ($) => ".",
    expression_record: ($) => seq($.expression_field_not_open_ended, $.expression_field),
    expression_field: ($) => seq($.field_name, $.expression),
    expression_field_not_open_ended: ($) =>
      seq($.field_name, $.expression_not_open_ended),
    expression_fn: ($) =>
      seq($.keyword_fn, $.pattern_typed, $.key_symbol_angle_right, $.expression),
    expression_fn_not_open_ended: ($) =>
      seq(
        $.keyword_fn,
        $.pattern_typed,
        $.key_symbol_angle_right,
        $.expression_not_open_ended,
      ),

    pattern_typed: ($) =>
      choice(
        $.pattern_parenthesized_typed,
        $.pattern_ignored_typed,
        $.pattern_variable_typed,
        $.pattern_variant_typed,
        $.pattern_record_typed,
      ),
    pattern_not_open_ended_typed: ($) =>
      choice(
        $.pattern_parenthesized_typed,
        $.pattern_ignored_not_open_ended_typed,
        $.pattern_variable_not_open_ended_typed,
        $.pattern_record_empty,
      ),
    pattern_untyped: ($) =>
      choice(
        $.pattern_ignored_untyped,
        $.pattern_variable_untyped,
        $.pattern_variant_untyped,
        $.pattern_record_untyped,
      ),
    pattern_not_open_ended_untyped: ($) =>
      choice(
        $.pattern_parenthesized_untyped,
        $.pattern_ignored_untyped,
        $.pattern_variable_untyped,
        $.pattern_record_empty,
      ),
    pattern_parenthesized_typed: ($) => seq("(", $.pattern_typed, ")"),
    pattern_parenthesized_untyped: ($) => seq("(", $.pattern_untyped, ")"),
    pattern_variable_typed: ($) => seq($.pattern_variable_untyped, $.type),
    pattern_variable_not_open_ended_typed: ($) =>
      seq($.pattern_variable_untyped, $.type_not_open_ended),
    pattern_variable_untyped: ($) => $.lower_name,
    pattern_ignored_typed: ($) => seq($.pattern_ignored_untyped, $.type_not_open_ended),
    pattern_ignored_not_open_ended_typed: ($) =>
      seq($.pattern_ignored_untyped, $.type_not_open_ended),
    pattern_ignored_untyped: ($) => "_",
    pattern_variant_typed: ($) => seq($.variant_name, $.pattern_typed),
    pattern_variant_untyped: ($) => seq($.variant_name, $.pattern_untyped),
    pattern_record_empty: ($) => ".",
    pattern_record_typed: ($) =>
      seq(repeat($.pattern_field_not_open_ended_typed), $.pattern_field_typed),
    pattern_field_not_open_ended_typed: ($) =>
      seq($.field_name, $.pattern_not_open_ended_typed),
    pattern_field_typed: ($) => seq($.field_name, $.pattern_typed),
    pattern_record_untyped: ($) =>
      seq(repeat($.pattern_field_not_open_ended_untyped), $.pattern_field_untyped),
    pattern_field_not_open_ended_untyped: ($) =>
      seq($.field_name, $.pattern_not_open_ended_untyped),
    pattern_field_untyped: ($) => seq($.field_name, $.pattern_untyped),

    type: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_name,
        $.type_construct_with_arguments,
        $.type_record,
        $.type_choice,
      ),
    type_not_variable: ($) =>
      choice(
        $.type_parenthesized,
        $.type_name,
        $.type_construct_with_arguments,
        $.type_record,
        $.type_choice,
      ),
    type_not_open_ended: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_name,
        $.type_construct_with_argument_not_open_ended,
        $.type_record_empty,
        $.type_choice_empty,
      ),
    type_parenthesized: ($) => seq("(", $.type, ")"),
    type_variable: ($) => $.upper_name,
    type_construct_with_argument_not_open_ended: ($) =>
      seq($.symbol_type_construct_underscore, $.type_name, $.type_not_open_ended),
    type_construct_with_arguments: ($) =>
      seq(
        $.symbol_type_construct_underscore,
        $.type_name,
        $.type_not_open_ended,
        repeat(seq(", ", $.type_not_open_ended)),
      ),
    type_choice_empty: ($) => "|",
    type_choice: ($) =>
      seq(repeat($.type_choice_variant_not_open_ended), $.type_choice_variant),
    type_choice_variant: ($) => seq($.variant_name, $.type),
    type_choice_variant_not_open_ended: ($) => seq($.variant_name, $.type_not_open_ended),
    type_record_empty: ($) => ".",
    type_record: ($) => seq(repeat($.type_field_not_open_ended), $.type_field),
    type_field_not_open_ended: ($) => seq($.field_name, $.type_not_open_ended),
    type_field: ($) => seq($.field_name, $.type),

    type_name: ($) => $.lower_name,
    char: ($) => seq("'", choice("\\\\", "\\'", /[^']/), "'"),
    string: ($) => $.string_quoted,
    string_quoted: ($) => seq('"', repeat(choice("\\\\", '\\"', /[^"]/)), '"'),
    number: ($) => /-?\+?\d+\.?\d*/,
    variant_name: ($) => /\|[a-z][a-zA-Z0-9-]*/,
    field_name: ($) => /\.[a-z][a-zA-Z0-9-]*/,
    upper_name: ($) => /[A-Z][a-zA-Z0-9-]*/,
    lower_name: ($) => /[a-z][a-zA-Z0-9-]*/,
    keyword_origin: ($) => "origin",
    keyword_fn: ($) => "fn",
    keyword_ty: ($) => "ty",
    key_symbol_colon: ($) => ":",
    key_symbol_angle_right: ($) => ">",
    key_symbol_arrow: ($) => "->",
    key_symbol_equals: ($) => "=",
    symbol_call_underscore: ($) => "_",
    symbol_type_construct_underscore: ($) => "_",
  },
});
