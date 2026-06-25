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
        const ExampleOrigin = enum { example };
        const example_origin: core.Origin_rid(ExampleOrigin) = .{ .origin_rid = .example };
        try core.origin_rid_rid(ExampleOrigin, example_origin);
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
test "vec add, element, occupiedCount, rid" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, u32).empty(origin);
    try std.testing.expectEqual(0, vec.occupiedCount());
    const slot0 = try vec.add(allocator, 123);
    const slot1 = try vec.add(allocator, 456);
    try std.testing.expectEqual(2, vec.occupiedCount());
    try std.testing.expectEqual(123, vec.element(allocator, slot0));
    try std.testing.expectEqual(1, vec.occupiedCount());
    const slot0_reused = try vec.add(allocator, 789);
    try std.testing.expectEqual(0, slot0_reused.index);
    try std.testing.expectEqual(789, vec.element(allocator, slot0_reused));
    try std.testing.expectEqual(1, vec.occupiedCount());
    try std.testing.expectEqual(456, vec.element(allocator, slot1));
    try std.testing.expectEqual(0, vec.occupiedCount());
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
    try std.testing.expectEqual(4, try vec.element(allocator, slot_causing_span_move_to_end));
    try std.testing.expectEqual(2, span1.start.index);
    try std.testing.expectEqual(2, span1.length.positive);
    const span1_moved = vec.moveSpanToVacant(span1);
    try std.testing.expectEqual(0, span1_moved.start.index);
    try std.testing.expectEqual(2, span1_moved.length.positive);
    vec.rid(allocator);
}

test "anonymous structs and union(enum)" {
    // This is very annoying: Zig just recently used to have real anonymous structs
    // which were removed from the language because their implementation was buggy.
    // Curiously, anonymous tuples still exist so I wonder what gives?
    const one = core.@"record.a.b"(@as(core.Str, ""), @as(core.Str, ""));
    const two = core.@"record.a.b"(@as(core.Str, ""), @as(core.Str, ""));
    rid_both(core.@".a.b"(core.Str, core.Str), one, two);
    try std.testing.expectEqualDeep(one, two);
}
fn rid_both(value: type, _: value, _: value) void {}
