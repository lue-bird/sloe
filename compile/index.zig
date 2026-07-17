const std = @import("std");
const core = @import("core.zig");

test "various trivial" {
    try core.p32_rid(core.P32{ .positive = 11 });
    try core.u32_rid(11);
    try core.i32_rid(-11);
    try core.f32_rid(-1.1);
    try core.char_rid('?');
    try core.str_rid("moin");
    try core.fn_rid(i32, i32, core.i32_negate_clamp);
    {
        const example_origin: core.Origin(enum { example }) = .example;
        try core.origin_rid(@TypeOf(example_origin), example_origin);
    }
    {
        const duped = try core.p32_dup(core.P32{ .positive = 11 });
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ core.P32{ .positive = 11 }, core.P32{ .positive = 11 } });
    }
    {
        const duped = try core.u32_dup(11);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ 11, 11 });
    }
    {
        const duped = try core.i32_dup(-11);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ -11, -11 });
    }
    {
        const duped = try core.f32_dup(-1.1);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ -1.1, -1.1 });
    }
    {
        const duped = try core.char_dup('?');
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ '?', '?' });
    }
    {
        const duped = try core.str_dup("moin");
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ "moin", "moin" });
    }
    {
        const duped = try core.fn_dup(i32, i32, core.i32_negate_clamp);
        try std.testing.expectEqual(.{ duped.a, duped.b }, .{ core.i32_negate_clamp, core.i32_negate_clamp });
    }
    try std.testing.expectEqual(core.P32{ .positive = 20 }, core.p32_add_clamp(.{ .p = core.P32{ .positive = 11 }, .u = 9 }));
    try std.testing.expectEqual(20, core.u32_add_clamp(.{ .a = 11, .b = 9 }));
    try std.testing.expectEqual(-2, core.i32_add_clamp(.{ .a = 1, .b = -3 }));
    try std.testing.expectEqual(-2, core.f32_add_clamp(.{ .a = -1.6, .b = -0.4 }));
    try std.testing.expectEqual(core.P32{ .positive = 99 }, core.p32_mul_clamp(.{ .a = core.P32{ .positive = 11 }, .b = core.P32{ .positive = 9 } }));
    try std.testing.expectEqual(99, core.u32_mul_clamp(.{ .a = 11, .b = 9 }));
    try std.testing.expectEqual(-99, core.i32_mul_clamp(.{ .a = -11, .b = 9 }));
    try std.testing.expectEqual(0.6, core.f32_mul_clamp(.{ .a = -1.5, .b = -0.4 }));
}
test "f32 round ties even" {
    try std.testing.expectEqual(-2, core.f32_round(.{ .n = -1.6, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(-2, core.f32_round(.{ .n = -1.5, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(-1, core.f32_round(.{ .n = -1.4, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(-1, core.f32_round(.{ .n = -0.6, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(0, core.f32_round(.{ .n = -0.5, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(0, core.f32_round(.{ .n = -0.4, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(0, core.f32_round(.{ .n = 0.4, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(0, core.f32_round(.{ .n = 0.5, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(1, core.f32_round(.{ .n = 0.6, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(1, core.f32_round(.{ .n = 1.4, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(2, core.f32_round(.{ .n = 1.5, .mode = core.Round_mode{ .nearest_else_even = {} } }));
    try std.testing.expectEqual(2, core.f32_round(.{ .n = 1.6, .mode = core.Round_mode{ .nearest_else_even = {} } }));
}
test "simple slot and span queries" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const slot4 = core.Slot(ExampleOrigin){ .origin = origin, .index = 4 };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = slot4, .length = core.P32.fromComptime(10) };
    try std.testing.expectEqual(4, (try core.slot_index(ExampleOrigin, slot4)).index);
    try std.testing.expectEqual(10, (try core.span_length(ExampleOrigin, span4_to_13)).length.positive);
    try std.testing.expectEqual(10, (try core.opt_span_length(ExampleOrigin, .{ .present = span4_to_13 })).length);
    try std.testing.expectEqual(0, (try core.opt_span_length(ExampleOrigin, .{ .absent = {} })).length);
    const empty_slot4 = core.Empty_slot(ExampleOrigin){ .origin = origin, .index = 4 };
    const empty_span4_to_13 = core.Empty_span(ExampleOrigin){ .start = empty_slot4, .length = core.P32.fromComptime(10) };
    try std.testing.expectEqual(4, (try core.empty_slot_index(ExampleOrigin, empty_slot4)).index);
    try std.testing.expectEqual(10, (try core.empty_span_length(ExampleOrigin, empty_span4_to_13)).length.positive);
    try std.testing.expectEqual(10, (try core.opt_empty_span_length(ExampleOrigin, .{ .present = empty_span4_to_13 })).length);
    try std.testing.expectEqual(0, (try core.opt_empty_span_length(ExampleOrigin, .{ .absent = {} })).length);
}
test "span_start" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot4_and_span5_to_13 = try core.span_start(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(4, slot4_and_span5_to_13.start.index);
    try std.testing.expectEqual(5, slot4_and_span5_to_13.end.present.start.index);
    try std.testing.expectEqual(9, slot4_and_span5_to_13.end.present.length.positive);
    try std.testing.expectEqual(13, try slot4_and_span5_to_13.end.present.endIndex());
}
test "span_end" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot13_and_span4_to_12 = try core.span_end(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(13, slot13_and_span4_to_12.end.index);
    try std.testing.expectEqual(4, slot13_and_span4_to_12.start.present.start.index);
    try std.testing.expectEqual(9, slot13_and_span4_to_12.start.present.length.positive);
    try std.testing.expectEqual(12, try slot13_and_span4_to_12.start.present.endIndex());
}
test "span_start_of_length_positive, normal inputs" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_10_and_11_to_13 = try core.span_start_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 7 } });
    try std.testing.expectEqual(11, span4_to_10_and_11_to_13.after.present.start.index);
    try std.testing.expectEqual(13, try span4_to_10_and_11_to_13.after.present.endIndex());
    try std.testing.expectEqual(4, span4_to_10_and_11_to_13.start.start.index);
    try std.testing.expectEqual(10, try span4_to_10_and_11_to_13.start.endIndex());
}
test "span_start_of_length_positive, given length > given span length" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_13_and_empty = try core.span_start_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 10000 } });
    try std.testing.expectEqual(0, (try core.opt_span_length(ExampleOrigin, span4_to_13_and_empty.after)).length);
    try std.testing.expectEqual(4, span4_to_13_and_empty.start.start.index);
    try std.testing.expectEqual(13, try span4_to_13_and_empty.start.endIndex());
}
test "span_end_of_length_positive, normal inputs" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_10_and_11_to_13 = try core.span_end_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 3 } });
    try std.testing.expectEqual(11, span4_to_10_and_11_to_13.end.start.index);
    try std.testing.expectEqual(13, try span4_to_10_and_11_to_13.end.endIndex());
    try std.testing.expectEqual(4, span4_to_10_and_11_to_13.before.present.start.index);
    try std.testing.expectEqual(10, try span4_to_10_and_11_to_13.before.present.endIndex());
}
test "span_end_of_length_positive, given length > given span length" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_13_and_empty = try core.span_end_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 10000 } });
    try std.testing.expectEqual(0, (try core.opt_span_length(ExampleOrigin, span4_to_13_and_empty.before)).length);
    try std.testing.expectEqual(4, span4_to_13_and_empty.end.start.index);
    try std.testing.expectEqual(13, try span4_to_13_and_empty.end.endIndex());
}
test "span_fold" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const index_sum = try core.opt_span_fold(ExampleOrigin, u32, .{
        .span = core.Opt(core.Span(ExampleOrigin)){ .present = span4_to_13 },
        .state = 0,
        .step = struct {
            pub fn step(current: core.@".slot.state"(core.Slot(ExampleOrigin), u32)) error{OutOfMemory}!u32 {
                return current.state +| current.slot.index;
            }
        }.step,
    });
    try std.testing.expectEqual(85, index_sum);
}
test "empty_span_start" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Empty_span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot4_and_span5_to_13 = try core.empty_span_start(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(4, slot4_and_span5_to_13.start.index);
    try std.testing.expectEqual(5, slot4_and_span5_to_13.end.present.start.index);
    try std.testing.expectEqual(9, slot4_and_span5_to_13.end.present.length.positive);
    try std.testing.expectEqual(13, try slot4_and_span5_to_13.end.present.endIndex());
}
test "empty_span_end" {
    const ExampleOrigin = enum { vec };
    const origin: core.Origin(ExampleOrigin) = .vec;
    const span4_to_13 = core.Empty_span(ExampleOrigin){ .start = .{ .origin = origin, .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot13_and_span4_to_12 = try core.empty_span_end(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(13, slot13_and_span4_to_12.end.index);
    try std.testing.expectEqual(4, slot13_and_span4_to_12.start.present.start.index);
    try std.testing.expectEqual(9, slot13_and_span4_to_12.start.present.length.positive);
    try std.testing.expectEqual(12, try slot13_and_span4_to_12.start.present.endIndex());
}
test "vec insert, add, take, notVacantCount, rid" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, u32).empty(origin);
    try std.testing.expectEqual(0, vec.notVacantCount());
    const slot0 = try vec.add(allocator, 123);
    const slot1 = try vec.add(allocator, 456);
    try std.testing.expectEqual(2, vec.notVacantCount());
    try std.testing.expectEqual(123, vec.remove(allocator, slot0));
    try std.testing.expectEqual(1, vec.notVacantCount());
    const slot0_reused = try vec.insert(allocator, 789);
    try std.testing.expectEqual(0, slot0_reused.index);
    try std.testing.expectEqual(789, vec.remove(allocator, slot0_reused));
    try std.testing.expectEqual(1, vec.notVacantCount());
    try std.testing.expectEqual(456, vec.remove(allocator, slot1));
    try std.testing.expectEqual(0, vec.notVacantCount());
    vec.rid(allocator);
}
test "vec empty slot" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    try std.testing.expect(core.Slot(VecOrigin) != core.Empty_slot(VecOrigin));
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, u32).empty(origin);
    try std.testing.expectEqual(0, vec.notVacantCount());
    const slot0 = try vec.add(allocator, 123);
    const slot1 = try vec.add(allocator, 456);
    const element0 = vec.element(slot0);
    try std.testing.expectEqual(123, element0.element);
    try std.testing.expectEqual(0, element0.slot.index);
    const slot0_new = vec.set(element0.slot, 321);
    try std.testing.expectEqual(321, vec.element_ptr(slot0_new).*);
    try std.testing.expectEqual(0, slot0_new.index);
    const element1 = vec.element(slot1);
    try vec.slotRid(allocator, element1.slot);
    vec.rid(allocator);
}
test "vec add to span" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, u32).empty(origin);
    const span0 = try vec.optSpanAdd(allocator, core.Opt(core.Span(VecOrigin)){ .absent = {} }, 123);
    const slot_causing_span_move_to_end = try vec.add(allocator, 4);
    const span1 = try vec.spanAdd(allocator, span0, 567);
    try std.testing.expectEqual(4, try vec.remove(allocator, slot_causing_span_move_to_end));
    try std.testing.expectEqual(2, span1.start.index);
    try std.testing.expectEqual(2, span1.length.positive);
    const span1_moved = vec.spanMoveToVacant(span1);
    try std.testing.expectEqual(0, span1_moved.start.index);
    try std.testing.expectEqual(2, span1_moved.length.positive);
    vec.rid(allocator);
}
test "vec reverse" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, u32).empty(origin);
    const span = try vec.addSlice(allocator, &.{ 1, 2, 3, 4, 5, 6 });
    const span_reversed = vec.optSpanReverse(span);
    try std.testing.expectEqual(span, span_reversed);
    try std.testing.expectEqualSlices(u32, &.{ 6, 5, 4, 3, 2, 1 }, vec.optSpanSlice(span_reversed));
    vec.rid(allocator);
}
test "origin with enums containing the same member name" {
    const AOrigin = enum { origin };
    const a_origin: core.Origin(AOrigin) = .origin;
    const BOrigin = enum { origin };
    const b_origin: core.Origin(BOrigin) = .origin;
    try std.testing.expect(@TypeOf(a_origin) != @TypeOf(b_origin));
    try core.origin_rid(AOrigin, a_origin);
    try core.origin_rid(BOrigin, b_origin);
}
test "origin can be @src()" {
    const AOrigin = SourceLocationUniqueEnum(@src());
    const a_origin: core.Origin(AOrigin) = undefined; // this is not great
    const BOrigin = SourceLocationUniqueEnum(@src());
    const b_origin: core.Origin(BOrigin) = undefined; // this is not great
    try std.testing.expect(@TypeOf(a_origin) != @TypeOf(b_origin));
    try core.origin_rid(AOrigin, a_origin);
    try core.origin_rid(BOrigin, b_origin);
}
/// No real benefit over using explicitly named `enum { ... }`s.
/// This would be necessary if enum/struct/union(enum) were structural, not nominal.
/// You may like this more, though because you need to type less.
/// I don't like it because it basically forces `undefined`
fn SourceLocationUniqueEnum(src_loc: std.lang.SourceLocation) type {
    return @Enum(
        u0,
        .exhaustive,
        &.{std.fmt.comptimePrint("{}", .{src_loc.line})},
        &.{0},
    );
}
test "compiles" {
    try expect_fn(core.vec_add);
    try expect_fn(core.vec_insert);
    try expect_fn(core.vec_insert_empty);
    try expect_fn(core.vec_add_empty);
    try expect_fn(core.vec_span_add);
    try expect_fn(core.vec_span_add_str);
    try expect_fn(core.vec_span_add_vec_span);
    try expect_fn(core.vec_span_add_vec_opt_span);
    try expect_fn(core.vec_opt_span_add_str);
    try expect_fn(core.vec_opt_span_add);
    try expect_fn(core.vec_opt_span_add_str);
    try expect_fn(core.vec_opt_span_add_vec_span);
    try expect_fn(core.vec_opt_span_add_vec_opt_span);
    try expect_fn(core.vec_span_add_own_span);
    try expect_fn(core.vec_span_add_own_opt_span);
    try expect_fn(core.vec_opt_span_add_own_span);
    try expect_fn(core.vec_opt_span_add_own_opt_span);
    try expect_fn(core.vec_empty_span_add_own_span);
    try expect_fn(core.vec_empty_span_add_own_opt_span);
    try expect_fn(core.vec_opt_empty_span_add_own_span);
    try expect_fn(core.vec_opt_empty_span_add_own_opt_span);
}
fn expect_fn(thing: anytype) !void {
    return switch (@typeInfo(@TypeOf(thing))) {
        .@"fn" => {},
        else => std.testing.expect(false),
    };
}

test "anonymous struct" {
    // This is very annoying: Zig just recently used to have real anonymous structs
    // which were removed from the language because their implementation was buggy.
    // Curiously, anonymous tuples still exist so I wonder what gives?
    const one = core.@"record.a.b"(@as(core.Str, ""), @as(core.Str, ""));
    const two = core.@"record.a.b"(@as(core.Str, ""), @as(core.Str, ""));
    rid_both(core.@".a.b"(core.Str, core.Str), one, two);
    try std.testing.expectEqualDeep(one, two);
}
test "anonymous union(enum)" {
    // below would not type-check
    // const one = @as(union(enum) { absent: void, present: core.Str }, .{ .present = "" });
    // const two = @as(union(enum) { absent: void, present: core.Str }, .{ .present = "" });
    // rid_both(union(enum) { absent: void, present: core.Str }, one, two);
    const one = @as(core.@"|absent|present"(void, core.Str), .{ .present = "" });
    const two = @as(core.@"|absent|present"(void, core.Str), .{ .present = "" });
    rid_both(core.@"|absent|present"(void, core.Str), one, two);
    try std.testing.expectEqualDeep(one, two);
}
fn rid_both(value: type, _: value, _: value) void {}
