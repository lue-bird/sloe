/**
 * @file sloe grammar for tree-sitter
 * @author lue-bird
 * @license Unlicense
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "sloe",
  extras: ($) => [/\s/],
  rules: {
    source_file: ($) =>
      seq(
        // the first project project element / comment lines are not guaranteed to be formatted with linebreaks in front
        choice(repeat($.comment), $.project_element),
        // this is more strict than sloe's parser but works for formatted code
        repeat(
          seq(
            $.indent0,
            // optional to allow extraneous linebreaks or trailing linebreaks
            optional($.project_element),
          ),
        ),
      ),
    comment: ($) => /#[^\n]*\n/,
    indent0: ($) => token.immediate("\n"),

    project_element: ($) => choice($.project_fn, $.type_alias),

    type_alias: ($) => seq("ty", $.type_name, repeat($.type_variable), $.type),

    project_fn: ($) =>
      seq(
        "fn",
        $.lower_name,
        $.pattern_not_open_ended_typed,
        $.type_not_open_ended,
        $.expression,
      ),

    expression: ($) => choice($.expression_parenthesized, $.expression_not_parenthesized),
    expression_not_parenthesized: ($) =>
      choice(
        $.expression_commented,
        $.expression_number,
        $.string,
        $.char,
        $.expression_reference_or_call,
        $.expression_origin,
        $.expression_variant,
        $.expression_record,
        $.expression_fn,
        $.expression_query,
        $.expression_origin,
      ),
    expression_not_open_ended: ($) =>
      choice($.expression_parenthesized, $.string, $.char, $.expression_variable),
    expression_parenthesized: ($) => seq("(", $.expression, ")"),
    expression_commented: ($) => seq($.comment, $.expression),
    expression_number: ($) => seq($.number, $.expression),
    expression_origin: ($) => seq("origin", $.lower_name, $.expression),
    expression_variant: ($) => seq($.variant_name, $.expression),
    expression_reference_or_call: ($) =>
      seq(
        $.lower_name,
        optional($.angled_type_arguments),
        optional($.expression_not_parenthesized),
      ),
    expression_variable: ($) => $.lower_name,
    angled_type_arguments: ($) => seq("<", repeat($.type_not_open_ended), ">"),
    angled_type_parameters: ($) => seq("<", repeat($.type_variable), ">"),
    expression_query: ($) =>
      seq(
        ":",
        $.expression_not_open_ended,
        repeat($.expression_query_case_not_open_ended),
        optional($.expression_query_case),
      ),
    expression_query_case: ($) =>
      seq($.pattern_not_open_ended_untyped, "<", $.expression),
    expression_query_case_not_open_ended: ($) =>
      seq($.pattern_not_open_ended_untyped, $.expression_not_open_ended),
    expression_record_empty: ($) => "&",
    expression_record: ($) =>
      seq("&", repeat($.expression_field_not_open_ended), optional($.expression_field)),
    expression_field: ($) => seq($.field_name, "<", $.expression),
    expression_field_not_open_ended: ($) =>
      seq($.field_name, $.expression_not_open_ended),
    expression_fn: ($) => seq("fn", $.pattern_not_open_ended_typed, $.expression),

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
    pattern_record_empty: ($) => "&",
    pattern_record_typed: ($) =>
      seq(
        "&",
        repeat($.pattern_field_not_open_ended_typed),
        optional($.pattern_field_typed),
      ),
    pattern_field_not_open_ended_typed: ($) =>
      seq($.field_name, $.pattern_not_open_ended_typed),
    pattern_field_typed: ($) => seq($.field_name, "<", $.pattern_typed),
    pattern_record_untyped: ($) =>
      seq(
        "&",
        repeat($.pattern_field_not_open_ended_untyped),
        optional($.pattern_field_untyped),
      ),
    pattern_field_not_open_ended_untyped: ($) =>
      seq($.field_name, $.pattern_not_open_ended_untyped),
    pattern_field_untyped: ($) => seq($.field_name, "<", $.pattern_untyped),

    type: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_construct,
        $.type_record,
        $.type_choice,
      ),
    type_not_open_ended: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_name,
        $.type_record_empty,
        $.type_choice_empty,
      ),
    type_parenthesized: ($) => seq("(", $.type, ")"),
    type_variable: ($) => $.upper_name,
    type_construct: ($) => seq($.type_name, repeat($.type_not_open_ended)),
    type_choice_empty: ($) => "|",
    type_choice: ($) =>
      seq(
        "|",
        repeat($.type_choice_variant_not_open_ended),
        optional($.type_choice_variant),
      ),
    type_choice_variant: ($) => seq($.variant_name, "<", $.type),
    type_choice_variant_not_open_ended: ($) => seq($.variant_name, $.type_not_open_ended),
    type_record_empty: ($) => "&",
    type_record: ($) => seq("&", repeat($.type_field)),
    type_field: ($) => seq($.field_name, $.type_not_open_ended),

    variant_name: ($) => $.upper_name,
    field_name: ($) => $.lower_name,
    type_name: ($) => $.lower_name,
    char: ($) => seq("'", choice("\\\\", "\\'", /[^']/), "'"),
    string: ($) => $.string_quoted,
    string_quoted: ($) => seq('"', repeat(choice("\\\\", '\\"', /[^"]/)), '"'),
    number: ($) => /-?\+?\d+\.?\d*/,
    upper_name: ($) => /[A-Z][a-zA-Z0-9-]*/,
    lower_name: ($) => /[a-z][a-zA-Z0-9-]*/,
  },
});
