const std = @import("std");
const core = @import("core.zig");

test "various trivial" {
    core.p32_rid(core.P32{ .positive = 11 });
    core.u32_rid(11);
    core.i32_rid(-11);
    core.f32_rid(-1.1);
    core.char_rid('?');
    core.str_rid(core.Str.fromComptime("moin"));
    core.fn_rid(i32, i32, struct {
        pub fn f(_: std.mem.Allocator, n: core.I32) error{OutOfMemory}!core.I32 {
            return core.i32_negate_clamp(n);
        }
    }.f);
    {
        const example_origin: core.Origin(enum { example }, void) = .{};
        core.origin_rid(@TypeOf(example_origin).origin, void, example_origin);
    }
    {
        const duped = core.p32_dup(core.P32{ .positive = 11 });
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ core.P32{ .positive = 11 }, core.P32{ .positive = 11 } });
    }
    {
        const duped = core.u32_dup(11);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ 11, 11 });
    }
    {
        const duped = core.i32_dup(-11);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ -11, -11 });
    }
    {
        const duped = core.f32_dup(-1.1);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ -1.1, -1.1 });
    }
    {
        const duped = core.char_dup('?');
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ '?', '?' });
    }
    {
        const duped = core.str_dup(core.Str.fromComptime("moin"));
        try std.testing.expectEqual(.{ duped.a.utf8.bytes, duped.b.utf8.bytes }, .{ "moin", "moin" });
    }
    {
        const example_fn = struct {
            pub fn f(_: std.mem.Allocator, n: core.I32) error{OutOfMemory}!core.I32 {
                return core.i32_negate_clamp(n);
            }
        }.f;
        const duped = core.fn_dup(i32, i32, example_fn);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ example_fn, example_fn });
    }
    try std.testing.expectEqual(314.0, @trunc(core.f32_pi({}) * 100));
    try std.testing.expectEqual(core.P32{ .positive = 20 }, core.p32_add_clamp(.{ .p = core.P32{ .positive = 11 }, .u = 9 }));
    try std.testing.expectEqual(20, core.u32_add_clamp(.{ .a = 11, .b = 9 }));
    try std.testing.expectEqual(2, core.u32_add_i32_clamp(.{ .u = 11, .i = -9 }));
    try std.testing.expectEqual(-2, core.i32_add_clamp(.{ .a = 1, .b = -3 }));
    try std.testing.expectEqual(-2, core.f32_add_clamp(.{ .a = -1.6, .b = -0.4 }));
    try std.testing.expectEqual(core.P32{ .positive = 99 }, core.p32_mul_clamp(.{ .a = core.P32{ .positive = 11 }, .b = core.P32{ .positive = 9 } }));
    try std.testing.expectEqual(99, core.u32_mul_clamp(.{ .a = 11, .b = 9 }));
    try std.testing.expectEqual(-99, core.i32_mul_clamp(.{ .a = -11, .b = 9 }));
    try std.testing.expectEqual(0.6, core.f32_mul_clamp(.{ .a = -1.5, .b = -0.4 }));
    try std.testing.expectEqual(121, core.u32_pow_clamp(.{ .base = 11, .exponent = core.P32{ .positive = 2 } }));
    try std.testing.expectEqual(121, core.i32_pow_clamp(.{ .base = -11, .exponent = core.P32{ .positive = 2 } }));
    try std.testing.expectEqual(core.Opt(core.F32){ .yes = 134.56001 }, core.f32_pow_i32(.{ .base = -11.6, .exponent = 2 }));
    try std.testing.expectEqual(core.Opt(core.F32){ .yes = 50.118725 }, core.f32_pow(.{ .base = 10, .exponent = 1.7 }));
    try std.testing.expectEqual(core.Opt(core.F32){ .no = {} }, core.f32_pow(.{ .base = -11, .exponent = 0.5 }));
    try std.testing.expectEqual(std.math.maxInt(i32), core.u32_to_i32_clamp(std.math.maxInt(u32)));
    try std.testing.expectEqual(core.P32{ .positive = std.math.maxInt(u32) }, core.u32_successor_clamp(std.math.maxInt(u32)));
}
test "i32_to_u32" {
    try std.testing.expectEqual(core.Opt(core.U32){ .yes = 1 }, core.i32_to_u32(1));
    try std.testing.expectEqual(core.Opt(core.U32){ .no = {} }, core.i32_to_u32(-1));
}
test "trivial rounding" {
    try std.testing.expectEqual(-1, core.f32_round_up(-1.5));
    try std.testing.expectEqual(-1, core.f32_round_up_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-2, core.f32_round_down(-1.5));
    try std.testing.expectEqual(-2, core.f32_round_down_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-1, core.f32_round_toward_0(-1.5));
    try std.testing.expectEqual(1, core.f32_round_toward_0(1.5));
    try std.testing.expectEqual(-1, core.f32_round_toward_0_to_i32_clamp(-1.5));
    try std.testing.expectEqual(1, core.f32_round_toward_0_to_i32_clamp(1.5));
    try std.testing.expectEqual(-2, core.f32_round_nearest_else_away_from_0(-1.5));
    try std.testing.expectEqual(2, core.f32_round_nearest_else_away_from_0(1.5));
    try std.testing.expectEqual(-2, core.f32_round_nearest_else_away_from_0_to_i32_clamp(-1.5));
    try std.testing.expectEqual(2, core.f32_round_nearest_else_away_from_0_to_i32_clamp(1.5));
}
test "f32_round_away_from_0" {
    try std.testing.expectEqual(-2, core.f32_round_away_from_0(-1.6));
    try std.testing.expectEqual(-2, core.f32_round_away_from_0(-1.5));
    try std.testing.expectEqual(-2, core.f32_round_away_from_0(-1.4));
    try std.testing.expectEqual(-1, core.f32_round_away_from_0(-0.6));
    try std.testing.expectEqual(-1, core.f32_round_away_from_0(-0.5));
    try std.testing.expectEqual(-1, core.f32_round_away_from_0(-0.4));
    try std.testing.expectEqual(0, core.f32_round_away_from_0(0.0));
    try std.testing.expectEqual(1, core.f32_round_away_from_0(0.4));
    try std.testing.expectEqual(1, core.f32_round_away_from_0(0.5));
    try std.testing.expectEqual(1, core.f32_round_away_from_0(0.6));
    try std.testing.expectEqual(2, core.f32_round_away_from_0(1.4));
    try std.testing.expectEqual(2, core.f32_round_away_from_0(1.5));
    try std.testing.expectEqual(2, core.f32_round_away_from_0(1.6));
}
test "f32_round_away_from_0_to_i32_clamp_to_i32_clamp" {
    try std.testing.expectEqual(-2, core.f32_round_away_from_0_to_i32_clamp(-1.6));
    try std.testing.expectEqual(-2, core.f32_round_away_from_0_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-2, core.f32_round_away_from_0_to_i32_clamp(-1.4));
    try std.testing.expectEqual(-1, core.f32_round_away_from_0_to_i32_clamp(-0.6));
    try std.testing.expectEqual(-1, core.f32_round_away_from_0_to_i32_clamp(-0.5));
    try std.testing.expectEqual(-1, core.f32_round_away_from_0_to_i32_clamp(-0.4));
    try std.testing.expectEqual(0, core.f32_round_away_from_0_to_i32_clamp(0.0));
    try std.testing.expectEqual(1, core.f32_round_away_from_0_to_i32_clamp(0.4));
    try std.testing.expectEqual(1, core.f32_round_away_from_0_to_i32_clamp(0.5));
    try std.testing.expectEqual(1, core.f32_round_away_from_0_to_i32_clamp(0.6));
    try std.testing.expectEqual(2, core.f32_round_away_from_0_to_i32_clamp(1.4));
    try std.testing.expectEqual(2, core.f32_round_away_from_0_to_i32_clamp(1.5));
    try std.testing.expectEqual(2, core.f32_round_away_from_0_to_i32_clamp(1.6));
}
test "f32_round_nearest_else_even" {
    try std.testing.expectEqual(-2, core.f32_round_nearest_else_even(-1.6));
    try std.testing.expectEqual(-2, core.f32_round_nearest_else_even(-1.5));
    try std.testing.expectEqual(-1, core.f32_round_nearest_else_even(-1.4));
    try std.testing.expectEqual(-1, core.f32_round_nearest_else_even(-0.6));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even(-0.5));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even(-0.4));
    try std.testing.expectEqual(0, core.f32_round_away_from_0(0.0));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even(0.4));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even(0.5));
    try std.testing.expectEqual(1, core.f32_round_nearest_else_even(0.6));
    try std.testing.expectEqual(1, core.f32_round_nearest_else_even(1.4));
    try std.testing.expectEqual(2, core.f32_round_nearest_else_even(1.5));
    try std.testing.expectEqual(2, core.f32_round_nearest_else_even(1.6));
}
test "f32_round_nearest_else_even_to_i32_clamp" {
    try std.testing.expectEqual(-2, core.f32_round_nearest_else_even_to_i32_clamp(-1.6));
    try std.testing.expectEqual(-2, core.f32_round_nearest_else_even_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-1, core.f32_round_nearest_else_even_to_i32_clamp(-1.4));
    try std.testing.expectEqual(-1, core.f32_round_nearest_else_even_to_i32_clamp(-0.6));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even_to_i32_clamp(-0.5));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even_to_i32_clamp(-0.4));
    try std.testing.expectEqual(0, core.f32_round_away_from_0(0.0));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even_to_i32_clamp(0.4));
    try std.testing.expectEqual(0, core.f32_round_nearest_else_even_to_i32_clamp(0.5));
    try std.testing.expectEqual(1, core.f32_round_nearest_else_even_to_i32_clamp(0.6));
    try std.testing.expectEqual(1, core.f32_round_nearest_else_even_to_i32_clamp(1.4));
    try std.testing.expectEqual(2, core.f32_round_nearest_else_even_to_i32_clamp(1.5));
    try std.testing.expectEqual(2, core.f32_round_nearest_else_even_to_i32_clamp(1.6));
}
test "order" {
    try std.testing.expectEqual(core.Order{ .equal = {} }, core.p32_order(.{ .left = core.P32.one, .right = core.P32.fromComptime(1) }));
    try std.testing.expectEqual(core.Order{ .equal = {} }, core.u32_order(.{ .left = 60, .right = 60 }));
    try std.testing.expectEqual(core.Order{ .greater = {} }, core.i32_order(.{ .left = 60, .right = -60 }));
    try std.testing.expectEqual(core.Order{ .less = {} }, core.f32_order(.{ .left = 40.2, .right = 60.1 }));
}
test "char-to-u32" {
    try std.testing.expectEqual(97, core.char_to_u32('a'));
}
test "str_start more after start" {
    const split = core.str_start(core.Str.fromComptime("abcde"));
    try std.testing.expectEqual('a', split.start);
    try std.testing.expectEqualStrings("bcde", split.after.yes.utf8.bytes);
}
test "str_start more after start (length 2 bytes)" {
    const split = core.str_start(core.Str.fromComptime("Ճbcde"));
    try std.testing.expectEqual('Ճ', split.start);
    try std.testing.expectEqualStrings("bcde", split.after.yes.utf8.bytes);
}
test "str_start more after start (length 3 bytes)" {
    const split = core.str_start(core.Str.fromComptime("ໆbcde"));
    try std.testing.expectEqual('ໆ', split.start);
    try std.testing.expectEqualStrings("bcde", split.after.yes.utf8.bytes);
}
test "str_start empty after start" {
    const split = core.str_start(core.Str.fromComptime("a"));
    try std.testing.expectEqual('a', split.start);
    try std.testing.expectEqual(core.Opt(core.Str){ .no = {} }, split.after);
}
test "str_end more before end" {
    const split = core.str_end(core.Str.fromComptime("abcde"));
    try std.testing.expectEqual('e', split.end);
    try std.testing.expectEqualStrings("abcd", split.before.yes.utf8.bytes);
}
test "str_end more before end (length 2 bytes)" {
    const split = core.str_end(core.Str.fromComptime("abcdՃ"));
    try std.testing.expectEqual('Ճ', split.end);
    try std.testing.expectEqualStrings("abcd", split.before.yes.utf8.bytes);
}
test "str_end more before end (length 3 bytes)" {
    const split = core.str_end(core.Str.fromComptime("abcdໆ"));
    try std.testing.expectEqual('ໆ', split.end);
    try std.testing.expectEqualStrings("abcd", split.before.yes.utf8.bytes);
}
test "str_end empty before end" {
    const split = core.str_end(core.Str.fromComptime("a"));
    try std.testing.expectEqual('a', split.end);
    try std.testing.expectEqual(core.Opt(core.Str){ .no = {} }, split.before);
}
test "choice_empty_to" {
    const choice_empty_rid: core.Fn(core.Choice, void) = struct {
        pub fn f(_: std.mem.Allocator, imp: core.Choice) error{OutOfMemory}!void {
            core.choice_empty_to(void, imp);
        }
    }.f;
    _ = choice_empty_rid;
}
test "simple slot and span queries" {
    const ExampleOrigin = enum { buf };
    const slot4 = core.Slot(ExampleOrigin){ .index = 4 };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = slot4, .length = core.P32.fromComptime(10) };
    try std.testing.expectEqual(4, (core.slot_index(ExampleOrigin, slot4)).index);
    try std.testing.expectEqual(10, (core.span_length(ExampleOrigin, span4_to_13)).length.positive);
    try std.testing.expectEqual(10, (core.opt_span_length(ExampleOrigin, .{ .yes = span4_to_13 })).length);
    try std.testing.expectEqual(0, (core.opt_span_length(ExampleOrigin, .{ .no = {} })).length);
    const unset_slot4 = core.Unset_slot(ExampleOrigin){ .index = 4 };
    const unset_span4_to_13 = core.Unset_span(ExampleOrigin){ .start = unset_slot4, .length = core.P32.fromComptime(10) };
    try std.testing.expectEqual(4, (core.unset_slot_index(ExampleOrigin, unset_slot4)).index);
    try std.testing.expectEqual(10, (core.unset_span_length(ExampleOrigin, unset_span4_to_13)).length.positive);
    try std.testing.expectEqual(10, (core.opt_unset_span_length(ExampleOrigin, .{ .yes = unset_span4_to_13 })).length);
    try std.testing.expectEqual(0, (core.opt_unset_span_length(ExampleOrigin, .{ .no = {} })).length);
}
test "span_start" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot4_and_span5_to_13 = core.span_start(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(4, slot4_and_span5_to_13.start.index);
    try std.testing.expectEqual(5, slot4_and_span5_to_13.after.yes.start.index);
    try std.testing.expectEqual(9, slot4_and_span5_to_13.after.yes.length.positive);
    try std.testing.expectEqual(13, slot4_and_span5_to_13.after.yes.endIndex());
}
test "span_end" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot13_and_span4_to_12 = core.span_end(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(13, slot13_and_span4_to_12.end.index);
    try std.testing.expectEqual(4, slot13_and_span4_to_12.before.yes.start.index);
    try std.testing.expectEqual(9, slot13_and_span4_to_12.before.yes.length.positive);
    try std.testing.expectEqual(12, slot13_and_span4_to_12.before.yes.endIndex());
}
test "span_start_of_length_positive, normal inputs" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_10_and_11_to_13 = core.span_start_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 7 } });
    try std.testing.expectEqual(11, span4_to_10_and_11_to_13.after.yes.start.index);
    try std.testing.expectEqual(13, span4_to_10_and_11_to_13.after.yes.endIndex());
    try std.testing.expectEqual(4, span4_to_10_and_11_to_13.start.start.index);
    try std.testing.expectEqual(10, span4_to_10_and_11_to_13.start.endIndex());
}
test "span_start_of_length_positive, given length > given span length" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_13_and_empty = core.span_start_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 10000 } });
    try std.testing.expectEqual(0, (core.opt_span_length(ExampleOrigin, span4_to_13_and_empty.after)).length);
    try std.testing.expectEqual(4, span4_to_13_and_empty.start.start.index);
    try std.testing.expectEqual(13, span4_to_13_and_empty.start.endIndex());
}
test "span_end_of_length_positive, normal inputs" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_10_and_11_to_13 = core.span_end_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 3 } });
    try std.testing.expectEqual(11, span4_to_10_and_11_to_13.end.start.index);
    try std.testing.expectEqual(13, span4_to_10_and_11_to_13.end.endIndex());
    try std.testing.expectEqual(4, span4_to_10_and_11_to_13.before.yes.start.index);
    try std.testing.expectEqual(10, span4_to_10_and_11_to_13.before.yes.endIndex());
}
test "span_end_of_length_positive, given length > given span length" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_13_and_empty = core.span_end_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 10000 } });
    try std.testing.expectEqual(0, (core.opt_span_length(ExampleOrigin, span4_to_13_and_empty.before)).length);
    try std.testing.expectEqual(4, span4_to_13_and_empty.end.start.index);
    try std.testing.expectEqual(13, span4_to_13_and_empty.end.endIndex());
}
test "span_fold up" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const index_sum = try core.opt_span_fold(ExampleOrigin, u32, std.testing.allocator, .{
        .span = core.Opt(core.Span(ExampleOrigin)){ .yes = span4_to_13 },
        .direction = .{ .up = {} },
        .state = 0,
        .step = struct {
            pub fn step(_: std.mem.Allocator, current: core.Record(struct { slot: core.Slot(ExampleOrigin), state: u32 })) error{OutOfMemory}!u32 {
                return current.state +| current.slot.index;
            }
        }.step,
    });
    try std.testing.expectEqual(85, index_sum);
}
test "span_fold down" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    var reverse_indexes_array_list = try core.opt_span_fold(
        ExampleOrigin,
        std.ArrayList(u32),
        std.testing.allocator,
        .{
            .span = core.Opt(core.Span(ExampleOrigin)){ .yes = span4_to_13 },
            .direction = .{ .down = {} },
            .state = std.ArrayList(u32).empty,
            .step = struct {
                pub fn step(
                    _: std.mem.Allocator,
                    current: core.Record(struct { slot: core.Slot(ExampleOrigin), state: std.ArrayList(u32) }),
                ) error{OutOfMemory}!std.ArrayList(u32) {
                    var modified_array_list = current.state;
                    try modified_array_list.append(std.testing.allocator, current.slot.index);
                    return modified_array_list;
                }
            }.step,
        },
    );
    try std.testing.expectEqualSlices(
        u32,
        &.{ 13, 12, 11, 10, 9, 8, 7, 6, 5, 4 },
        reverse_indexes_array_list.items,
    );
    reverse_indexes_array_list.deinit(std.testing.allocator);
}
test "unset_span_start" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Unset_span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot4_and_span5_to_13 = core.unset_span_start(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(4, slot4_and_span5_to_13.start.index);
    try std.testing.expectEqual(5, slot4_and_span5_to_13.after.yes.start.index);
    try std.testing.expectEqual(9, slot4_and_span5_to_13.after.yes.length.positive);
    try std.testing.expectEqual(13, slot4_and_span5_to_13.after.yes.endIndex());
}
test "unset_span_end" {
    const ExampleOrigin = enum { buf };
    const span4_to_13 = core.Unset_span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot13_and_span4_to_12 = core.unset_span_end(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(13, slot13_and_span4_to_12.end.index);
    try std.testing.expectEqual(4, slot13_and_span4_to_12.before.yes.start.index);
    try std.testing.expectEqual(9, slot13_and_span4_to_12.before.yes.length.positive);
    try std.testing.expectEqual(12, slot13_and_span4_to_12.before.yes.endIndex());
}
test "array create" {
    const ExampleArrayRecord = struct { e0: u32, e1: u32 };
    const example_array0 = core.recordToArray(ExampleArrayRecord{ .e0 = 0, .e1 = 2 });
    try std.testing.expectEqualSlices(u32, &[_]u32{ 0, 2 }, &example_array0);
    // we can just specify them as arrays directly
    const example_array1 = [_]u32{ @as(u32, 0), @as(u32, 2) };
    try std.testing.expectEqualSlices(u32, &example_array1, &example_array0);
    try std.testing.expectEqual(@TypeOf(example_array1), @TypeOf(example_array0));
    // or as anonymus structs (unrelated record type, but nobody can care)
    // Which means sloe doesn't even need to collect and generate record types etc.
    // I do not think this is possible in rust but happy to be proven wrong
    const example_array2 = core.recordToArray(.{ .e0 = @as(u32, 0), .e1 = @as(u32, 2) });
    try std.testing.expectEqualSlices(u32, &example_array2, &example_array0);
    try std.testing.expectEqual(@TypeOf(example_array2), @TypeOf(example_array0));
}
test "buf_add_array" {
    const ExampleOrigin = enum { origin };
    const example_origin: core.Origin(ExampleOrigin, void) = .{};
    const example_buf = core.buf_empty(u32, ExampleOrigin, void, example_origin);
    const ExampleArrayRecord = struct { e0: u32, e1: u32 };
    const example_array0 = core.recordToArray(ExampleArrayRecord{ .e0 = 0, .e1 = 2 });
    const with_array = try core.buf_add_array(u32, @TypeOf(example_origin), ExampleArrayRecord, std.testing.allocator, .{
        .buf = example_buf,
        .new = example_array0,
    });
    core.buf_rid(u32, @TypeOf(example_origin), std.testing.allocator, with_array.buf);
}
test "buf_opt_span_add_array" {
    const ExampleOrigin = enum { origin };
    const example_origin: core.Origin(ExampleOrigin, void) = .{};
    const example_buf = core.buf_empty(u32, ExampleOrigin, void, example_origin);
    const ExampleArrayRecord = struct { e0: u32, e1: u32 };
    const example_array0 = core.recordToArray(ExampleArrayRecord{ .e0 = 0, .e1 = 2 });
    const with_array = try core.buf_opt_span_add_array(u32, @TypeOf(example_origin), ExampleArrayRecord, std.testing.allocator, .{
        .buf = example_buf,
        .span = .{ .no = {} },
        .new = example_array0,
    });
    core.buf_rid(u32, @TypeOf(example_origin), std.testing.allocator, with_array.buf);
}
test "unset_slice castOrRidAndAllocate working" {
    const allocator = std.testing.allocator;
    const unset_slice_u32 = try core.Unset_slice(u32).allocateLength(allocator, 10);
    const unset_slice_i32 = try unset_slice_u32.castOrRidAndAllocate(i32, allocator);
    unset_slice_i32.rid(allocator);
}
test "unset_slice castOrRidAndAllocate fallback" {
    const allocator = std.testing.allocator;
    const unset_slice_u32 = try core.Unset_slice(u32).allocateLength(allocator, 10);
    const unset_slice_f128 = try unset_slice_u32.castOrRidAndAllocate(f128, allocator);
    try std.testing.expect(@intFromPtr(unset_slice_u32.undefined_items.ptr) != @intFromPtr(unset_slice_f128.undefined_items.ptr));
    unset_slice_f128.rid(allocator);
}
test "buf insert, add, take, notVacantCount, rid" {
    const allocator = std.testing.allocator;
    const BufOrigin = enum { buf };
    const origin: core.Origin(BufOrigin, void) = .{};
    var buf = core.buf_empty(u32, BufOrigin, void, origin);
    try std.testing.expectEqual(0, buf.notVacantCount());
    const slot0 = try buf.add(allocator, 123);
    const slot1 = try buf.add(allocator, 456);
    try std.testing.expectEqual(2, buf.notVacantCount());
    try std.testing.expectEqual(123, buf.remove(allocator, slot0));
    try std.testing.expectEqual(1, buf.notVacantCount());
    const slot0_reused = try buf.insert(allocator, 789);
    try std.testing.expectEqual(0, slot0_reused.index);
    try std.testing.expectEqual(789, buf.remove(allocator, slot0_reused));
    try std.testing.expectEqual(1, buf.notVacantCount());
    try std.testing.expectEqual(456, buf.remove(allocator, slot1));
    try std.testing.expectEqual(0, buf.notVacantCount());
    buf.rid(allocator);
}
test "buf unset slot" {
    const allocator = std.testing.allocator;
    const BufOrigin = enum { buf };
    try std.testing.expect(core.Slot(BufOrigin) != core.Unset_slot(BufOrigin));
    const origin: core.Origin(BufOrigin, void) = .{};
    var buf = core.buf_empty(u32, BufOrigin, void, origin);
    try std.testing.expectEqual(0, buf.notVacantCount());
    const slot0 = try buf.add(allocator, 123);
    const slot1 = try buf.add(allocator, 456);
    const element0 = buf.unset(slot0);
    try std.testing.expectEqual(123, element0.element);
    try std.testing.expectEqual(0, element0.slot.index);
    const slot0_new = buf.set(element0.slot, 321);
    try std.testing.expectEqual(321, buf.element(slot0_new).*);
    try std.testing.expectEqual(0, slot0_new.index);
    const element1 = buf.unset(slot1);
    try buf.unsetSlotRid(allocator, element1.slot);
    buf.rid(allocator);
}
test "buf add to span" {
    const allocator = std.testing.allocator;
    const BufOrigin = enum { buf };
    const origin: core.Origin(BufOrigin, void) = .{};
    var buf = core.buf_empty(u32, BufOrigin, void, origin);
    const span0 = try buf.optSpanAdd(allocator, core.Opt(core.Span(@TypeOf(origin))){ .no = {} }, 123);
    const slot_causing_span_move_to_end = try buf.add(allocator, 4);
    const span1 = try buf.spanAdd(allocator, span0, 567);
    try std.testing.expectEqual(4, try buf.remove(allocator, slot_causing_span_move_to_end));
    try std.testing.expectEqual(2, span1.start.index);
    try std.testing.expectEqual(2, span1.length.positive);
    const span1_moved = buf.spanMoveToVacant(span1);
    try std.testing.expectEqual(0, span1_moved.start.index);
    try std.testing.expectEqual(2, span1_moved.length.positive);
    buf.rid(allocator);
}
test "buf add strs" {
    const allocator = std.testing.allocator;
    const BufOrigin = enum { buf };
    const origin: core.Origin(BufOrigin, void) = .{};
    const buf = core.buf_empty(core.Char, BufOrigin, void, origin);
    const with_abcd = try core.buf_char_opt_span_add_str(
        @TypeOf(origin),
        allocator,
        .{ .buf = buf, .span = .{ .no = {} }, .new = core.Str.fromComptime("abcd") },
    );
    try std.testing.expectEqual(4, with_abcd.span.length.positive);
    const with_wrenches = try core.buf_char_opt_span_add_str(
        @TypeOf(origin),
        allocator,
        .{ .buf = with_abcd.buf, .span = .{ .yes = with_abcd.span }, .new = core.Str.fromComptime("🔧🔧🔧") },
    );
    try std.testing.expectEqualSlices(
        core.Char,
        &.{ 'a', 'b', 'c', 'd', '🔧', '🔧', '🔧' },
        with_wrenches.buf.spanSlice(with_wrenches.span),
    );
    try std.testing.expectEqual(7, with_wrenches.span.length.positive);
    with_wrenches.buf.rid(allocator);
}
test "buf char add numbers" {
    const allocator = std.testing.allocator;
    const BufOrigin = enum { buf };
    const origin: core.Origin(BufOrigin, void) = .{};
    const buf = core.buf_empty(core.Char, BufOrigin, void, origin);
    const with_u32 = try core.buf_char_opt_span_add_u32(
        @TypeOf(origin),
        allocator,
        .{ .buf = buf, .span = .{ .no = {} }, .new = 1234 },
    );
    try std.testing.expectEqual(4, with_u32.span.length.positive);
    const with_i32 = try core.buf_char_span_add_i32(
        @TypeOf(origin),
        allocator,
        .{ .buf = with_u32.buf, .span = with_u32.span, .new = -2 },
    );
    try std.testing.expectEqual(6, with_i32.span.length.positive);
    const with_f32 = try core.buf_char_span_add_f32(
        @TypeOf(origin),
        allocator,
        .{ .buf = with_i32.buf, .span = with_i32.span, .new = -0.1 },
    );
    try std.testing.expectEqualSlices(
        core.Char,
        &.{ '1', '2', '3', '4', '-', '2', '-', '0', '.', '1' },
        with_f32.buf.spanSlice(with_f32.span),
    );
    with_f32.buf.rid(allocator);
}
test "buf reverse" {
    const allocator = std.testing.allocator;
    const BufOrigin = enum { buf };
    const origin: core.Origin(BufOrigin, void) = .{};
    var buf = core.buf_empty(u32, BufOrigin, void, origin);
    const span = try buf.addSlice(allocator, &.{ 1, 2, 3, 4, 5, 6 });
    const span_reversed = buf.optSpanReverse(span);
    try std.testing.expectEqual(span, span_reversed);
    try std.testing.expectEqualSlices(u32, &.{ 6, 5, 4, 3, 2, 1 }, buf.optSpanSlice(span_reversed));
    buf.rid(allocator);
}
test "buf add remove stress test" {
    const allocator = std.testing.allocator;
    const BufOrigin = enum { buf };
    const origin: core.Origin(BufOrigin, void) = .{};
    var buf = core.buf_empty(usize, BufOrigin, void, origin);
    var slots = std.ArrayList(core.Slot(@TypeOf(origin))).empty;
    for (0..100) |i| {
        try slots.append(allocator, try buf.add(allocator, i));
    }
    var rng = std.Random.DefaultPrng.init(std.testing.random_seed);
    var random = rng.random();
    random.shuffle(core.Slot(@TypeOf(origin)), slots.items);
    for (slots.items) |slot| {
        _ = try buf.remove(allocator, slot);
    }
    slots.deinit(allocator);
    try std.testing.expectEqual(0, buf.vacant.items.len);
    try std.testing.expectEqual(0, buf.elements.items.len);
    buf.rid(allocator);
}
test "buf into unset slice then reuse" {
    const allocator = std.testing.allocator;
    const AOrigin = enum { origin };
    const a_origin: core.Origin(AOrigin, void) = .{};
    var a_buf = core.buf_empty(usize, AOrigin, void, a_origin);
    try a_buf.preAllocateAtLeast(allocator, 20);
    const a_capacity = a_buf.elements.capacity;
    try std.testing.expect(a_capacity >= 20);
    const unset_slice = a_buf.intoUnsetSlice(allocator);
    const BOrigin = enum { origin };
    const b_origin: core.Origin(BOrigin, void) = .{};
    var b_buf = core.buf_reuse(
        usize,
        BOrigin,
        void,
        .{ .origin = b_origin, .slice = unset_slice },
    );
    try std.testing.expectEqual(0, b_buf.elements.items.len);
    try std.testing.expectEqual(a_capacity, b_buf.elements.capacity);
    b_buf.rid(allocator);
}
test "unset_slice_cast_or_rid_and_allocate u64 to i63" {
    const allocator = std.testing.allocator;
    const unset_slice_u64 = try core.unset_slice_allocate_length(u64, allocator, 20);
    const unset_slice_u64_length = unset_slice_u64.length();
    try std.testing.expect(unset_slice_u64_length >= 20);
    const unset_slice_i63 = try core.unset_slice_cast_or_rid_and_allocate(u64, i63, allocator, unset_slice_u64);
    // memory is reused, not re-allocated
    try std.testing.expectEqual(
        @intFromPtr(unset_slice_u64.undefined_items.ptr),
        @intFromPtr(unset_slice_i63.undefined_items.ptr),
    );
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    var buf = core.buf_reuse(
        i63,
        Origin,
        void,
        .{ .origin = origin, .slice = unset_slice_i63 },
    );
    try std.testing.expectEqual(0, buf.elements.items.len);
    try std.testing.expectEqual(unset_slice_u64_length, buf.elements.capacity);
    buf.rid(allocator);
}
test "unset_slice_cast_or_rid_and_allocate u64 to struct{u32,u16}" {
    const allocator = std.testing.allocator;
    const unset_slice_u64 = try core.unset_slice_allocate_length(u64, allocator, 20);
    const unset_slice_u64_length = unset_slice_u64.length();
    try std.testing.expect(unset_slice_u64_length >= 20);
    const unset_slice_tuple_u32_u16 = try core.unset_slice_cast_or_rid_and_allocate(u64, struct { u32, u16 }, allocator, unset_slice_u64);
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    var buf = core.buf_reuse(
        struct { u32, u16 },
        Origin,
        void,
        .{ .origin = origin, .slice = unset_slice_tuple_u32_u16 },
    );
    try std.testing.expectEqual(0, buf.elements.items.len);
    try std.testing.expectEqual(unset_slice_u64_length, buf.elements.capacity);
    buf.rid(allocator);
}
test "Unset_span != Span" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    const a_span: core.Span(@TypeOf(origin)) = .{ .start = .{ .index = 0 }, .length = core.P32.one };
    const b_span: core.Unset_span(@TypeOf(origin)) = .{ .start = .{ .index = 0 }, .length = core.P32.one };
    try std.testing.expect(@TypeOf(a_span) != @TypeOf(b_span));
}
test "Unset_slot != Slot" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    const a_slot: core.Slot(@TypeOf(origin)) = .{ .index = 0 };
    const b_slot: core.Unset_slot(@TypeOf(origin)) = .{ .index = 0 };
    try std.testing.expect(@TypeOf(a_slot) != @TypeOf(b_slot));
}
test "origin with enums containing the same member name" {
    const AOrigin = enum { origin };
    const a_origin: core.Origin(AOrigin, void) = .{};
    const BOrigin = enum { origin };
    const b_origin: core.Origin(BOrigin, void) = .{};
    try std.testing.expect(@TypeOf(a_origin) != @TypeOf(b_origin));
    core.origin_rid(AOrigin, void, a_origin);
    core.origin_rid(BOrigin, void, b_origin);
}
test "origin can be @src()" {
    const AOrigin = SourceLocationUniqueEnum(@src());
    const a_origin: core.Origin(AOrigin, void) = .{};
    const BOrigin = SourceLocationUniqueEnum(@src());
    const b_origin: core.Origin(BOrigin, void) = .{};
    try std.testing.expect(@TypeOf(a_origin) != @TypeOf(b_origin));
    core.origin_rid(AOrigin, void, a_origin);
    core.origin_rid(BOrigin, void, b_origin);
}
/// No real benefit over using explicitly named `enum { ... }`s.
/// This would be necessary if enum/struct/union(enum) were structural, not nominal.
/// You may like this more, though because you need to type less
fn SourceLocationUniqueEnum(src_loc: std.lang.SourceLocation) type {
    return @Enum(
        u0,
        .exhaustive,
        &.{std.fmt.comptimePrint("{}", .{src_loc.line})},
        &.{0},
    );
}
test "slot origin erase, then unerase" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    const slot = core.Slot(@TypeOf(origin)){ .index = 69 };
    const slot_erased = core.origin_erase(
        Origin,
        core.Slot(core.Origin(core.Erased, void)),
        core.slot_origin_isolate(Origin, void, slot),
    );
    try std.testing.expectEqual(slot.index, slot_erased.erased.index);
    const NewOrigin = enum { origin };
    const uneraser = core.Origin_uneraser(NewOrigin){};
    const slot_unerased = core.slot_origin_unerase(
        NewOrigin,
        void,
        .{ .slot = slot_erased.erased, .uneraser = uneraser },
    );
    try std.testing.expectEqual(slot_erased.erased.index, slot_unerased.slot.index);
}
test "span origin erase, then unerase" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    const span = core.Span(@TypeOf(origin)){
        .start = .{ .index = 66 },
        .length = core.P32{ .positive = 3 },
    };
    const span_erased = core.origin_erase(
        Origin,
        core.Span(core.Origin(core.Erased, void)),
        core.span_origin_isolate(Origin, void, span),
    );
    try std.testing.expectEqual(span.endIndex(), span_erased.erased.endIndex());
    const NewOrigin = enum { origin };
    const uneraser = core.Origin_uneraser(NewOrigin){};
    const span_unerased = core.span_origin_unerase(
        NewOrigin,
        void,
        .{ .span = span_erased.erased, .uneraser = uneraser },
    );
    try std.testing.expectEqual(span_erased.erased.endIndex(), span_unerased.span.endIndex());
}
test "buf origin erase with elements, keeping elements" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    var buf = core.buf_empty(u32, Origin, void, origin);
    _ = try buf.add(std.testing.allocator, 60);
    const buf_isolated = try core.buf_origin_isolate(
        u32,
        u32,
        Origin,
        void,
        std.testing.allocator,
        .{
            .buf = buf,
            .element_isolate = struct {
                pub fn f(_: std.mem.Allocator, element: u32) error{OutOfMemory}!core.Origin_isolated(Origin, u32) {
                    return core.u32_origin_isolate(Origin, element);
                }
            }.f,
        },
    );
    const buf_erased = core.origin_erase(Origin, core.Buf_origin_erased(void, u32), buf_isolated);
    try std.testing.expectEqual(1, buf_erased.erased.erased.elements.items.len);
    const uneraser = core.Origin_uneraser(Origin){};
    const buf_unerased = core.buf_origin_unerase_keep_elements(
        u32,
        Origin,
        void,
        .{ .buf = buf_erased.erased, .uneraser = uneraser },
    );
    try std.testing.expectEqual(1, buf_unerased.buf.elements.items.len);
    // scrap the original buf, showing that the allocation is the same as for buf_unerased
    buf.rid(std.testing.allocator);
}
test "buf origin erase with elements, same size and alignment" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    var buf = core.buf_empty(u32, Origin, void, origin);
    _ = try buf.add(std.testing.allocator, 60);
    const buf_isolated = try core.buf_origin_isolate(
        u32,
        u32,
        Origin,
        void,
        std.testing.allocator,
        .{
            .buf = buf,
            .element_isolate = struct {
                pub fn f(_: std.mem.Allocator, element: u32) error{OutOfMemory}!core.Origin_isolated(Origin, u32) {
                    return core.u32_origin_isolate(Origin, element);
                }
            }.f,
        },
    );
    const buf_erased = core.origin_erase(Origin, core.Buf_origin_erased(void, u32), buf_isolated);
    try std.testing.expectEqual(1, buf_erased.erased.erased.elements.items.len);
    const uneraser = core.Origin_uneraser(Origin){};
    const buf_unerased = try core.buf_origin_unerase(
        u32,
        u32,
        Origin,
        void,
        std.testing.allocator,
        .{
            .buf = buf_erased.erased,
            .uneraser = uneraser,
            .element_unerase = struct {
                pub fn f(_: std.mem.Allocator, unerase: core.Record(struct {
                    element: u32,
                    uneraser: core.Origin_uneraser(Origin),
                })) error{OutOfMemory}!core.Record(struct {
                    element: u32,
                    uneraser: core.Origin_uneraser(Origin),
                }) {
                    return .{ .element = unerase.element, .uneraser = unerase.uneraser };
                }
            }.f,
        },
    );
    try std.testing.expectEqual(1, buf_unerased.buf.elements.items.len);
    // scrap the original buf, showing that the allocation is the same as for buf_unerased
    buf.rid(std.testing.allocator);
}
test "buf origin erase with elements, different size and alignment" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    var buf = core.buf_empty(u32, Origin, void, origin);
    _ = try buf.add(std.testing.allocator, 60);
    const buf_isolated = try core.buf_origin_isolate(
        u32,
        u64,
        Origin,
        void,
        std.testing.allocator,
        .{
            .buf = buf,
            .element_isolate = struct {
                pub fn f(_: std.mem.Allocator, element: u32) error{OutOfMemory}!core.Origin_isolated(Origin, u64) {
                    return .{ .erased = @as(u64, element) };
                }
            }.f,
        },
    );
    const buf_erased = core.origin_erase(Origin, core.Buf_origin_erased(void, u64), buf_isolated);
    try std.testing.expectEqual(1, buf_erased.erased.erased.elements.items.len);
    const uneraser = core.Origin_uneraser(Origin){};
    const buf_unerased = try core.buf_origin_unerase(
        u32,
        u64,
        Origin,
        void,
        std.testing.allocator,
        .{
            .buf = buf_erased.erased,
            .uneraser = uneraser,
            .element_unerase = struct {
                pub fn f(_: std.mem.Allocator, unerase: core.Record(struct {
                    element: u64,
                    uneraser: core.Origin_uneraser(Origin),
                })) error{OutOfMemory}!core.Record(struct {
                    element: u32,
                    uneraser: core.Origin_uneraser(Origin),
                }) {
                    return .{
                        .element = std.math.lossyCast(u32, unerase.element),
                        .uneraser = unerase.uneraser,
                    };
                }
            }.f,
        },
    );
    try std.testing.expectEqual(1, buf_unerased.buf.elements.items.len);
    buf_unerased.buf.rid(std.testing.allocator);
}
test "origin_erase span + buf, then origin_unerase" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    var buf = core.buf_empty(u32, Origin, void, origin);
    const span = (try buf.add(std.testing.allocator, 1)).to_span();
    const span_isolated = core.span_origin_isolate(Origin, void, span);
    const buf_isolated = try core.buf_origin_isolate(
        u32,
        u32,
        Origin,
        void,
        std.testing.allocator,
        .{
            .buf = buf,
            .element_isolate = struct {
                pub fn f(_: std.mem.Allocator, element: u32) error{OutOfMemory}!core.Origin_isolated(Origin, u32) {
                    return core.u32_origin_isolate(Origin, element);
                }
            }.f,
        },
    );
    const isolated = core.origin_isolated_merge(
        core.Buf_origin_erased(void, u32),
        core.Span(core.Origin(core.Erased, void)),
        Origin,
        .{ .a = buf_isolated, .b = span_isolated },
    );
    const erased = core.origin_erase(Origin, core.Record(struct {
        a: core.Buf_origin_erased(void, u32),
        b: core.Span(core.Origin(core.Erased, void)),
    }), isolated);
    try std.testing.expectEqual(0, erased.erased.b.endIndex());
    const NewOrigin = enum { origin };
    const new_origin: core.Origin(NewOrigin, void) = .{};
    const unerased = try core.origin_unerase(
        NewOrigin,
        struct {
            core.Buf(@TypeOf(new_origin), u32),
            core.Span(@TypeOf(new_origin)),
        },
        core.Record(struct {
            a: core.Buf_origin_erased(void, u32),
            b: core.Span(core.Origin(core.Erased, void)),
        }),
        std.testing.allocator,
        .{
            .erased = erased,
            .origin = new_origin,
            .unerase = struct {
                pub fn f(_: std.mem.Allocator, unerase: core.Record(struct {
                    erased: core.Record(struct {
                        a: core.Buf_origin_erased(void, u32),
                        b: core.Span(core.Origin(core.Erased, void)),
                    }),
                    uneraser: core.Origin_uneraser(NewOrigin),
                })) error{OutOfMemory}!core.Record(struct {
                    unerased: struct {
                        core.Buf(@TypeOf(new_origin), u32),
                        core.Span(@TypeOf(new_origin)),
                    },
                    uneraser: core.Origin_uneraser(NewOrigin),
                }) {
                    const span_unerased = core.span_origin_unerase(
                        NewOrigin,
                        void,
                        .{ .span = unerase.erased.b, .uneraser = unerase.uneraser },
                    );
                    const buf_unerased = try core.buf_origin_unerase(
                        u32,
                        u32,
                        NewOrigin,
                        void,
                        std.testing.allocator,
                        .{
                            .buf = unerase.erased.a,
                            .uneraser = span_unerased.uneraser,
                            .element_unerase = struct {
                                pub fn f(_: std.mem.Allocator, element_unerase: core.Record(struct {
                                    element: u32,
                                    uneraser: core.Origin_uneraser(NewOrigin),
                                })) error{OutOfMemory}!core.Record(struct {
                                    element: u32,
                                    uneraser: core.Origin_uneraser(NewOrigin),
                                }) {
                                    return .{ .element = element_unerase.element, .uneraser = element_unerase.uneraser };
                                }
                            }.f,
                        },
                    );
                    return .{
                        .unerased = .{ buf_unerased.buf, span_unerased.span },
                        .uneraser = buf_unerased.uneraser,
                    };
                }
            }.f,
        },
    );
    try std.testing.expectEqual(0, unerased.@"1".endIndex());
    // scrap the original buf, showing that the allocation is the same as for unerased.buf
    buf.rid(std.testing.allocator);
}
test "origin_erase span + buf, then origin_erased_rid" {
    const Origin = enum { origin };
    const origin: core.Origin(Origin, void) = .{};
    var buf = core.buf_empty(u32, Origin, void, origin);
    const span = (try buf.add(std.testing.allocator, 1)).to_span();
    const span_isolated = core.span_origin_isolate(Origin, void, span);
    const buf_isolated = try core.buf_origin_isolate(
        u32,
        u32,
        Origin,
        void,
        std.testing.allocator,
        .{
            .buf = buf,
            .element_isolate = struct {
                pub fn f(_: std.mem.Allocator, element: u32) error{OutOfMemory}!core.Origin_isolated(Origin, u32) {
                    return core.u32_origin_isolate(Origin, element);
                }
            }.f,
        },
    );
    const isolated = core.origin_isolated_merge(
        core.Buf_origin_erased(void, u32),
        core.Span(core.Origin(core.Erased, void)),
        Origin,
        .{ .a = buf_isolated, .b = span_isolated },
    );
    const erased = core.origin_erase(Origin, core.Record(struct {
        a: core.Buf_origin_erased(void, u32),
        b: core.Span(core.Origin(core.Erased, void)),
    }), isolated);
    try core.origin_erased_rid(
        core.Record(struct {
            a: core.Buf_origin_erased(void, u32),
            b: core.Span(core.Origin(core.Erased, void)),
        }),
        std.testing.allocator,
        .{
            .erased = erased,
            .rid = struct {
                pub fn f(allocator: std.mem.Allocator, value_erased: core.Record(struct {
                    a: core.Buf_origin_erased(void, u32),
                    b: core.Span(core.Origin(core.Erased, void)),
                })) error{OutOfMemory}!void {
                    // in real code we would remove span
                    value_erased.a.erased.rid(allocator);
                }
            }.f,
        },
    );
}
test "anonymous struct default does not work" {
    // This is very annoying: Zig just recently used to have real anonymous structs
    // which were removed from the language because their implementation was buggy.
    // Curiously, anonymous tuples still exist so I wonder what gives?
    const one: struct { a: core.Str, b: core.Str } = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    const two: struct { a: core.Str, b: core.Str } = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    try std.testing.expect(@TypeOf(one) != @TypeOf(two));
}
test "anonymous struct fresh inferred vs default type" {
    const one = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    const two: core.Record(struct { a: core.Str, b: core.Str }) = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    try std.testing.expect(@TypeOf(one) != @TypeOf(two));
}
test "anonymous struct fresh inferred" {
    const one: core.Record(struct { a: core.Str, b: core.Str }) = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    const two: core.Record(struct { a: core.Str, b: core.Str }) = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    // would fail to compile:
    // const three: core.Record(struct { a: core.Str, b: core.Str }) = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    rid_both(core.Record(struct { a: core.Str, b: core.Str }), one, two);
    try std.testing.expectEqualDeep(one, two);
}
test "anonymous struct fresh core.record" {
    const one = core.record(.{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") });
    const two = core.record(.{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") });
    rid_both(core.Record(struct { a: core.Str, b: core.Str }), one, two);
    try std.testing.expectEqualDeep(one, two);
}
test "anonymous struct fresh core.record works even through anonymous default struct type" {
    const one = core.record(.{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") });
    const two_default = .{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") };
    const two = core.record(two_default);
    rid_both(core.Record(struct { a: core.Str, b: core.Str }), one, two);
    try std.testing.expectEqualDeep(one, two);
}
test "anonymous struct fresh core.record works even through anonymous default struct type, equal types" {
    const one = core.record(.{ .a = "a", .b = "a" });
    const two_default = .{ .a = "a", .b = "a" };
    const two = core.record(two_default);
    try std.testing.expectEqual(@TypeOf(one), @TypeOf(two));
    try std.testing.expectEqual(@TypeOf(one), core.Record(struct { a: *const [1:0]u8, b: *const [1:0]u8 }));
}
test "anonymous struct fresh core.record from different origins" {
    const one = core.record(.{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") });
    const two = core.str_dup(core.Str.fromComptime("a"));
    rid_both(core.Record(struct { a: core.Str, b: core.Str }), one, two);
    try std.testing.expectEqualDeep(one, two);
}
test "anonymous struct fresh core.record from different branches" {
    for ([_]bool{ true, false }) |logic| {
        const one = if (logic) core.record(.{
            .a = core.Str.fromComptime("a"),
            .b = core.Str.fromComptime("a"),
        }) else core.str_dup(core.Str.fromComptime("a"));
        rid_both(core.Str, one.a, one.b);
        try std.testing.expectEqualStrings(one.a.utf8.bytes, one.b.utf8.bytes);
    }
}
test "anonymous struct out of order fields" {
    const one = core.record(.{ .a = core.Str.fromComptime("a"), .b = core.Str.fromComptime("a") });
    const two = core.record(.{ .b = core.Str.fromComptime("a"), .a = core.Str.fromComptime("a") });
    // I do not want this behavior but want to know if it changes
    try std.testing.expect(@TypeOf(one) != @TypeOf(two));
}
test "anonymous union(enum)" {
    // below would not type-check
    // const one = @as(union(enum) { no: void, yes: core.Str }, .{ .yes = "a" });
    // const two = @as(union(enum) { no: void, yes: core.Str }, .{ .yes = "a" });
    // rid_both(union(enum) { no: void, yes: core.Str }, one, two);
    const one = @as(core.@"|no|yes"(void, core.Str), .{ .yes = core.Str.fromComptime("a") });
    const two = @as(core.@"|no|yes"(void, core.Str), .{ .yes = core.Str.fromComptime("a") });
    rid_both(core.@"|no|yes"(void, core.Str), one, two);
    try std.testing.expectEqualDeep(one, two);
}
fn rid_both(value: type, _: value, _: value) void {}

test {
    std.testing.refAllDecls(core);
}
fn expect_fn(thing: anytype) !void {
    return switch (@typeInfo(@TypeOf(thing))) {
        .@"fn" => {},
        else => std.testing.expect(false),
    };
}

test "anonymous functions can access context types" {
    var allocator = std.testing.allocator;
    const T = @TypeOf(allocator);
    const function = struct {
        pub fn f() T {
            return std.testing.allocator;
        }
    }.f;
    allocator = std.testing.allocator;
    _ = function();
}
