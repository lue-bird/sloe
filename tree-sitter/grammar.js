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
        choice(
          $.type_without_arguments,
          seq(
            $.type_with_arguments_name,
            $.type_variable,
            repeat(seq(",", $.type_variable)),
          ),
        ),
        repeat($.comment),
        $.type,
      ),

    project_fn: ($) =>
      seq(
        $.keyword_fn,
        $.project_fn_name,
        repeat($.braced_type_parameter),
        $.pattern_typed,
        $.key_symbol_colon,
        $.type,
        repeat($.comment),
        $.key_symbol_equals,
        $.expression,
      ),
    project_fn_name: ($) => $.upper_name,

    expression: ($) =>
      choice(
        $.expression_parenthesized,
        $.expression_commented,
        $.expression_number,
        $.string,
        $.char,
        $.expression_variable,
        $.expression_record_empty,
        $.expression_call,
        $.expression_origin,
        $.expression_variant,
        $.expression_record,
        $.expression_fn,
        $.expression_query,
        $.expression_origin,
        $.expression_array,
      ),
    expression_not_open_ending_in_query: ($) =>
      choice(
        $.expression_parenthesized,
        $.string,
        $.char,
        $.expression_variable,
        $.expression_record_empty,
        $.expression_call_not_open_ending_in_query,
        $.expression_number,
        $.expression_fn_not_open_ending_in_query,
        $.expression_origin_not_open_ending_in_query,
        $.expression_variant_not_open_ending_in_query,
        $.expression_record_not_open_ending_in_query,
        $.expression_commented_not_open_ending_in_query,
        $.expression_array_not_open_ending_in_query,
      ),
    expression_not_open_ending_in_record: ($) =>
      choice(
        $.expression_parenthesized,
        $.string,
        $.char,
        $.expression_variable,
        $.expression_record_empty,
        $.expression_call_not_open_ending_in_record,
        $.expression_number_not_open_ending_in_record,
        $.expression_fn_not_open_ending_in_record,
        $.expression_origin_not_open_ending_in_record,
        $.expression_variant_not_open_ending_in_record,
        $.expression_commented_not_open_ending_in_record,
        $.expression_array_not_open_ending_in_record,
      ),
    expression_not_open_ending_in_array: ($) =>
      choice(
        $.expression_parenthesized,
        $.string,
        $.char,
        $.expression_variable,
        $.expression_record_empty,
        $.expression_call_not_open_ending_in_array,
        $.expression_number,
        $.expression_fn_not_open_ending_in_array,
        $.expression_origin_not_open_ending_in_array,
        $.expression_variant_not_open_ending_in_array,
        $.expression_commented_not_open_ending_in_array,
        $.expression_record_not_open_ending_in_array,
      ),
    expression_parenthesized: ($) => seq("(", $.expression, ")"),
    expression_commented: ($) =>
      seq(
        // semantically repeat1($.comment) but for simplicity the syntactic equivalent
        $.comment,
        $.expression,
      ),
    expression_commented_not_open_ending_in_query: ($) =>
      seq($.comment, $.expression_not_open_ending_in_query),
    expression_commented_not_open_ending_in_record: ($) =>
      seq($.comment, $.expression_not_open_ending_in_record),
    expression_commented_not_open_ending_in_array: ($) =>
      seq($.comment, $.expression_not_open_ending_in_array),
    expression_number: ($) => seq($.number, $.type),
    expression_number_not_open_ending_in_record: ($) =>
      seq($.number, $.type_not_open_ending_in_record),
    expression_origin: ($) =>
      seq($.key_symbol_origin, $.expression_variable, $.expression),
    expression_origin_not_open_ending_in_query: ($) =>
      seq(
        $.key_symbol_origin,
        $.expression_variable,
        $.expression_not_open_ending_in_query,
      ),
    expression_origin_not_open_ending_in_record: ($) =>
      seq(
        $.key_symbol_origin,
        $.expression_variable,
        $.expression_not_open_ending_in_record,
      ),
    expression_origin_not_open_ending_in_array: ($) =>
      seq(
        $.key_symbol_origin,
        $.expression_variable,
        $.expression_not_open_ending_in_array,
      ),
    expression_variant: ($) =>
      seq("|", $.braced_type_argument, $.variant_name, $.expression),
    expression_variant_not_open_ending_in_query: ($) =>
      seq(
        "|",
        $.braced_type_argument,
        $.variant_name,
        $.expression_not_open_ending_in_query,
      ),
    expression_variant_not_open_ending_in_record: ($) =>
      seq(
        "|",
        $.braced_type_argument,
        $.variant_name,
        $.expression_not_open_ending_in_record,
      ),
    expression_variant_not_open_ending_in_array: ($) =>
      seq(
        "|",
        $.braced_type_argument,
        $.variant_name,
        $.expression_not_open_ending_in_array,
      ),
    expression_variable: ($) => $.lower_name,
    expression_call: ($) =>
      seq($.project_fn_name, repeat($.braced_type_argument), $.expression),
    expression_call_not_open_ending_in_query: ($) =>
      seq(
        $.project_fn_name,
        repeat($.braced_type_argument),
        $.expression_not_open_ending_in_query,
      ),
    expression_call_not_open_ending_in_record: ($) =>
      seq(
        $.project_fn_name,
        repeat($.braced_type_argument),
        $.expression_not_open_ending_in_record,
      ),
    expression_call_not_open_ending_in_array: ($) =>
      seq(
        $.project_fn_name,
        repeat($.braced_type_argument),
        $.expression_not_open_ending_in_array,
      ),
    braced_type_argument: ($) => seq("{", $.type, "}"),
    braced_type_parameter: ($) => seq("{", $.type_variable, "}"),
    expression_query: ($) =>
      seq(
        $.key_symbol_question_mark,
        $.expression_not_open_ending_in_query,
        repeat($.expression_query_case_not_open_ending_in_query),
        $.expression_query_case,
      ),
    expression_query_not_ending_in_record: ($) =>
      seq(
        $.key_symbol_question_mark,
        $.expression_not_open_ending_in_query,
        repeat($.expression_query_case_not_open_ending_in_query),
        $.expression_query_case_not_open_ending_in_record,
      ),
    expression_query_not_ending_in_array: ($) =>
      seq(
        $.key_symbol_question_mark,
        $.expression_not_open_ending_in_query,
        repeat($.expression_query_case_not_open_ending_in_query),
        $.expression_query_case_not_open_ending_in_array,
      ),
    expression_query_case: ($) => seq("[", $.pattern_untyped, "]", $.expression),
    expression_query_case_not_open_ending_in_query: ($) =>
      seq("[", $.pattern_untyped, "]", $.expression_not_open_ending_in_query),
    expression_query_case_not_open_ending_in_record: ($) =>
      seq("[", $.pattern_untyped, "]", $.expression_not_open_ending_in_record),
    expression_query_case_not_open_ending_in_array: ($) =>
      seq("[", $.pattern_untyped, "]", $.expression_not_open_ending_in_array),
    expression_record_empty: ($) => ".",
    expression_record: ($) =>
      seq(
        repeat($.expression_record_part_not_open_ending_in_record),
        $.expression_record_part,
      ),
    expression_record_not_open_ending_in_query: ($) =>
      seq(
        repeat($.expression_record_part_not_open_ending_in_record),
        $.expression_record_part_not_open_ending_in_query,
      ),
    expression_record_not_open_ending_in_array: ($) =>
      seq(
        repeat($.expression_record_part_not_open_ending_in_record),
        $.expression_record_part_not_open_ending_in_array,
      ),
    expression_record_part: ($) =>
      seq(choice($.key_symbol_spread_fields, $.field_name), $.expression),
    expression_record_part_not_open_ending_in_query: ($) =>
      seq(
        choice($.key_symbol_spread_fields, $.field_name),
        $.expression_not_open_ending_in_query,
      ),
    expression_record_part_not_open_ending_in_record: ($) =>
      seq(
        choice($.key_symbol_spread_fields, $.field_name),
        $.expression_not_open_ending_in_record,
      ),
    expression_record_part_not_open_ending_in_array: ($) =>
      seq(
        choice($.key_symbol_spread_fields, $.field_name),
        $.expression_not_open_ending_in_array,
      ),
    expression_array: ($) =>
      seq(repeat(seq(";", $.expression_not_open_ending_in_array)), ";", $.expression),
    expression_array_not_open_ending_in_record: ($) =>
      seq(
        repeat(seq(";", $.expression_not_open_ending_in_array)),
        ";",
        $.expression_not_open_ending_in_record,
      ),
    expression_array_not_open_ending_in_query: ($) =>
      seq(
        repeat(seq(";", $.expression_not_open_ending_in_array)),
        ";",
        $.expression_not_open_ending_in_query,
      ),
    expression_fn: ($) => seq("[", $.pattern_typed, "]", $.expression),
    expression_fn_not_open_ending_in_query: ($) =>
      seq("[", $.pattern_typed, "]", $.expression_not_open_ending_in_query),
    expression_fn_not_open_ending_in_record: ($) =>
      seq("[", $.pattern_typed, "]", $.expression_not_open_ending_in_record),
    expression_fn_not_open_ending_in_array: ($) =>
      seq("[", $.pattern_typed, "]", $.expression_not_open_ending_in_array),

    pattern_typed: ($) =>
      choice(
        $.pattern_parenthesized_typed,
        $.pattern_record_empty,
        $.pattern_variable_typed,
        $.pattern_variant_typed,
        $.pattern_record_typed,
      ),
    pattern_not_open_ending_in_record_typed: ($) =>
      choice(
        $.pattern_parenthesized_typed,
        $.pattern_record_empty,
        $.pattern_variable_not_open_ending_in_record_typed,
        $.pattern_record_empty,
      ),
    pattern_untyped: ($) =>
      choice(
        $.pattern_parenthesized_untyped,
        $.pattern_record_empty,
        $.pattern_variable_untyped,
        $.pattern_variant_untyped,
        $.pattern_record_untyped,
      ),
    pattern_not_open_ending_in_record_untyped: ($) =>
      choice(
        $.pattern_parenthesized_untyped,
        $.pattern_record_empty,
        $.pattern_variable_untyped,
        $.pattern_record_empty,
      ),
    pattern_parenthesized_typed: ($) => seq("(", $.pattern_typed, ")"),
    pattern_parenthesized_untyped: ($) => seq("(", $.pattern_untyped, ")"),
    pattern_variable_typed: ($) => seq($.pattern_variable_untyped, $.type),
    pattern_variable_not_open_ending_in_record_typed: ($) =>
      seq($.pattern_variable_untyped, $.type_not_open_ending_in_record),
    pattern_variable_untyped: ($) => $.lower_name,
    pattern_variant_typed: ($) => seq($.variant_name_including_bar, $.pattern_typed),
    pattern_variant_untyped: ($) => seq($.variant_name_including_bar, $.pattern_untyped),
    pattern_record_empty: ($) => ".",
    pattern_record_typed: ($) =>
      seq(repeat($.pattern_field_not_open_ending_in_record_typed), $.pattern_field_typed),
    pattern_field_not_open_ending_in_record_typed: ($) =>
      seq($.field_name, $.pattern_not_open_ending_in_record_typed),
    pattern_field_typed: ($) => seq($.field_name, $.pattern_typed),
    pattern_record_untyped: ($) =>
      seq(
        repeat($.pattern_field_not_open_ending_in_record_untyped),
        $.pattern_field_untyped,
      ),
    pattern_field_not_open_ending_in_record_untyped: ($) =>
      seq($.field_name, $.pattern_not_open_ending_in_record_untyped),
    pattern_field_untyped: ($) => seq($.field_name, $.pattern_untyped),

    type: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_without_arguments,
        $.type_choice_empty,
        $.type_record_empty,
        $.type_construct_with_arguments,
        $.type_record,
        $.type_choice,
      ),
    type_not_open_ending_in_construct: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_without_arguments,
        $.type_choice_empty,
        $.type_record_empty,
        $.type_record_not_open_ending_in_construct,
        $.type_choice_not_open_ending_in_construct,
      ),
    type_not_open_ending_in_record: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_without_arguments,
        $.type_choice_empty,
        $.type_record_empty,
        $.type_construct_with_arguments_not_open_ending_in_record,
        $.type_choice_not_open_ending_in_record,
      ),
    type_not_open_ending_in_choice: ($) =>
      choice(
        $.type_parenthesized,
        $.type_variable,
        $.type_without_arguments,
        $.type_choice_empty,
        $.type_record_empty,
        $.type_construct_with_arguments_not_open_ending_in_choice,
        $.type_record_not_open_ending_in_choice,
      ),
    type_parenthesized: ($) => seq("(", $.type, ")"),
    type_variable: ($) => seq("_", $.lower_name),
    type_construct_with_arguments: ($) =>
      seq(
        $.type_with_arguments_name,
        repeat(seq($.type_not_open_ending_in_construct, ",")),
        $.type,
      ),
    type_construct_with_arguments_not_open_ending_in_record: ($) =>
      seq(
        $.type_with_arguments_name,
        repeat(seq($.type_not_open_ending_in_construct, ",")),
        $.type_not_open_ending_in_record,
      ),
    type_construct_with_arguments_not_open_ending_in_choice: ($) =>
      seq(
        $.type_with_arguments_name,
        repeat(seq($.type_not_open_ending_in_construct, ",")),
        $.type_not_open_ending_in_choice,
      ),
    type_choice_empty: ($) => "|",
    type_choice: ($) =>
      seq(repeat($.type_choice_variant_not_open_ending_in_choice), $.type_choice_variant),
    type_choice_not_open_ending_in_record: ($) =>
      seq(
        repeat($.type_choice_variant_not_open_ending_in_choice),
        $.type_choice_variant_not_open_ending_in_record,
      ),
    type_choice_not_open_ending_in_construct: ($) =>
      seq(
        repeat($.type_choice_variant_not_open_ending_in_choice),
        $.type_choice_variant_not_open_ending_in_construct,
      ),
    type_choice_variant: ($) => seq($.variant_name_including_bar, $.type),
    type_choice_variant_not_open_ending_in_construct: ($) =>
      seq($.variant_name_including_bar, $.type_not_open_ending_in_construct),
    type_choice_variant_not_open_ending_in_record: ($) =>
      seq($.variant_name_including_bar, $.type_not_open_ending_in_record),
    type_choice_variant_not_open_ending_in_choice: ($) =>
      seq($.variant_name_including_bar, $.type_not_open_ending_in_choice),
    type_record_empty: ($) => ".",
    type_record: ($) =>
      seq(repeat($.type_record_part_not_open_ending_in_record), $.type_record_part),
    type_record_not_open_ending_in_construct: ($) =>
      seq(
        repeat($.type_record_part_not_open_ending_in_record),
        $.type_record_part_not_open_ending_in_construct,
      ),
    type_record_not_open_ending_in_choice: ($) =>
      seq(
        repeat($.type_record_part_not_open_ending_in_record),
        $.type_record_part_not_open_ending_in_choice,
      ),
    type_record_part: ($) =>
      seq(choice($.key_symbol_spread_fields, $.field_name), $.type),
    type_record_part_not_open_ending_in_record: ($) =>
      seq(
        choice($.key_symbol_spread_fields, $.field_name),
        $.type_not_open_ending_in_record,
      ),
    type_record_part_not_open_ending_in_choice: ($) =>
      seq(
        choice($.key_symbol_spread_fields, $.field_name),
        $.type_not_open_ending_in_choice,
      ),
    type_record_part_not_open_ending_in_construct: ($) =>
      seq(
        choice($.key_symbol_spread_fields, $.field_name),
        $.type_not_open_ending_in_construct,
      ),
    type_without_arguments: ($) => $.lower_name,
    type_with_arguments_name: ($) => $.upper_name,

    char: ($) => seq("'", choice("\\\\", "\\'", /[^']/), "'"),
    string: ($) => $.string_quoted,
    string_quoted: ($) => seq('"', repeat(choice("\\\\", '\\"', /[^"]/)), '"'),
    number: ($) => /-?\+?\d+\.?\d*/,
    variant_name_including_bar: ($) => /\|[a-z][a-zA-Z0-9-]*/,
    variant_name: ($) => $.lower_name,
    field_name: ($) => /\.[a-z][a-zA-Z0-9-]*/,
    upper_name: ($) => /[A-Z][a-zA-Z0-9-]*/,
    lower_name: ($) => /[a-z][a-zA-Z0-9-]*/,
    keyword_fn: ($) => "fn",
    keyword_ty: ($) => "ty",
    key_symbol_question_mark: ($) => "?",
    key_symbol_equals: ($) => "=",
    key_symbol_colon: ($) => ":",
    key_symbol_spread_fields: ($) => "..",
    key_symbol_origin: ($) => "^",
  },
});
