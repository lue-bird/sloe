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
test "trivial rounding" {
    try std.testing.expectEqual(-1, try core.f32_round_up(-1.5));
    try std.testing.expectEqual(-1, try core.f32_round_up_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-2, try core.f32_round_down(-1.5));
    try std.testing.expectEqual(-2, try core.f32_round_down_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-1, try core.f32_round_toward_0(-1.5));
    try std.testing.expectEqual(1, try core.f32_round_toward_0(1.5));
    try std.testing.expectEqual(-1, try core.f32_round_toward_0_to_i32_clamp(-1.5));
    try std.testing.expectEqual(1, try core.f32_round_toward_0_to_i32_clamp(1.5));
    try std.testing.expectEqual(-2, try core.f32_round_nearest_else_away_from_0(-1.5));
    try std.testing.expectEqual(2, try core.f32_round_nearest_else_away_from_0(1.5));
    try std.testing.expectEqual(-2, try core.f32_round_nearest_else_away_from_0_to_i32_clamp(-1.5));
    try std.testing.expectEqual(2, try core.f32_round_nearest_else_away_from_0_to_i32_clamp(1.5));
}
test "f32_round_away_from_0" {
    try std.testing.expectEqual(-2, try core.f32_round_away_from_0(-1.6));
    try std.testing.expectEqual(-2, try core.f32_round_away_from_0(-1.5));
    try std.testing.expectEqual(-2, try core.f32_round_away_from_0(-1.4));
    try std.testing.expectEqual(-1, try core.f32_round_away_from_0(-0.6));
    try std.testing.expectEqual(-1, try core.f32_round_away_from_0(-0.5));
    try std.testing.expectEqual(-1, try core.f32_round_away_from_0(-0.4));
    try std.testing.expectEqual(0, try core.f32_round_away_from_0(0.0));
    try std.testing.expectEqual(1, try core.f32_round_away_from_0(0.4));
    try std.testing.expectEqual(1, try core.f32_round_away_from_0(0.5));
    try std.testing.expectEqual(1, try core.f32_round_away_from_0(0.6));
    try std.testing.expectEqual(2, try core.f32_round_away_from_0(1.4));
    try std.testing.expectEqual(2, try core.f32_round_away_from_0(1.5));
    try std.testing.expectEqual(2, try core.f32_round_away_from_0(1.6));
}
test "f32_round_away_from_0_to_i32_clamp_to_i32_clamp" {
    try std.testing.expectEqual(-2, try core.f32_round_away_from_0_to_i32_clamp(-1.6));
    try std.testing.expectEqual(-2, try core.f32_round_away_from_0_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-2, try core.f32_round_away_from_0_to_i32_clamp(-1.4));
    try std.testing.expectEqual(-1, try core.f32_round_away_from_0_to_i32_clamp(-0.6));
    try std.testing.expectEqual(-1, try core.f32_round_away_from_0_to_i32_clamp(-0.5));
    try std.testing.expectEqual(-1, try core.f32_round_away_from_0_to_i32_clamp(-0.4));
    try std.testing.expectEqual(0, try core.f32_round_away_from_0_to_i32_clamp(0.0));
    try std.testing.expectEqual(1, try core.f32_round_away_from_0_to_i32_clamp(0.4));
    try std.testing.expectEqual(1, try core.f32_round_away_from_0_to_i32_clamp(0.5));
    try std.testing.expectEqual(1, try core.f32_round_away_from_0_to_i32_clamp(0.6));
    try std.testing.expectEqual(2, try core.f32_round_away_from_0_to_i32_clamp(1.4));
    try std.testing.expectEqual(2, try core.f32_round_away_from_0_to_i32_clamp(1.5));
    try std.testing.expectEqual(2, try core.f32_round_away_from_0_to_i32_clamp(1.6));
}
test "f32_round_nearest_else_even" {
    try std.testing.expectEqual(-2, try core.f32_round_nearest_else_even(-1.6));
    try std.testing.expectEqual(-2, try core.f32_round_nearest_else_even(-1.5));
    try std.testing.expectEqual(-1, try core.f32_round_nearest_else_even(-1.4));
    try std.testing.expectEqual(-1, try core.f32_round_nearest_else_even(-0.6));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even(-0.5));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even(-0.4));
    try std.testing.expectEqual(0, try core.f32_round_away_from_0(0.0));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even(0.4));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even(0.5));
    try std.testing.expectEqual(1, try core.f32_round_nearest_else_even(0.6));
    try std.testing.expectEqual(1, try core.f32_round_nearest_else_even(1.4));
    try std.testing.expectEqual(2, try core.f32_round_nearest_else_even(1.5));
    try std.testing.expectEqual(2, try core.f32_round_nearest_else_even(1.6));
}
test "f32_round_nearest_else_even_to_i32_clamp" {
    try std.testing.expectEqual(-2, try core.f32_round_nearest_else_even_to_i32_clamp(-1.6));
    try std.testing.expectEqual(-2, try core.f32_round_nearest_else_even_to_i32_clamp(-1.5));
    try std.testing.expectEqual(-1, try core.f32_round_nearest_else_even_to_i32_clamp(-1.4));
    try std.testing.expectEqual(-1, try core.f32_round_nearest_else_even_to_i32_clamp(-0.6));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even_to_i32_clamp(-0.5));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even_to_i32_clamp(-0.4));
    try std.testing.expectEqual(0, try core.f32_round_away_from_0(0.0));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even_to_i32_clamp(0.4));
    try std.testing.expectEqual(0, try core.f32_round_nearest_else_even_to_i32_clamp(0.5));
    try std.testing.expectEqual(1, try core.f32_round_nearest_else_even_to_i32_clamp(0.6));
    try std.testing.expectEqual(1, try core.f32_round_nearest_else_even_to_i32_clamp(1.4));
    try std.testing.expectEqual(2, try core.f32_round_nearest_else_even_to_i32_clamp(1.5));
    try std.testing.expectEqual(2, try core.f32_round_nearest_else_even_to_i32_clamp(1.6));
}
test "simple slot and span queries" {
    const ExampleOrigin = enum { vec };
    const slot4 = core.Slot(ExampleOrigin){ .index = 4 };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = slot4, .length = core.P32.fromComptime(10) };
    try std.testing.expectEqual(4, (try core.slot_index(ExampleOrigin, slot4)).index);
    try std.testing.expectEqual(10, (try core.span_length(ExampleOrigin, span4_to_13)).length.positive);
    try std.testing.expectEqual(10, (try core.opt_span_length(ExampleOrigin, .{ .yes = span4_to_13 })).length);
    try std.testing.expectEqual(0, (try core.opt_span_length(ExampleOrigin, .{ .no = {} })).length);
    const unset_slot4 = core.Unset_slot(ExampleOrigin){ .index = 4 };
    const unset_span4_to_13 = core.Unset_span(ExampleOrigin){ .start = unset_slot4, .length = core.P32.fromComptime(10) };
    try std.testing.expectEqual(4, (try core.unset_slot_index(ExampleOrigin, unset_slot4)).index);
    try std.testing.expectEqual(10, (try core.unset_span_length(ExampleOrigin, unset_span4_to_13)).length.positive);
    try std.testing.expectEqual(10, (try core.opt_unset_span_length(ExampleOrigin, .{ .yes = unset_span4_to_13 })).length);
    try std.testing.expectEqual(0, (try core.opt_unset_span_length(ExampleOrigin, .{ .no = {} })).length);
}
test "span_start" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot4_and_span5_to_13 = try core.span_start(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(4, slot4_and_span5_to_13.start.index);
    try std.testing.expectEqual(5, slot4_and_span5_to_13.end.yes.start.index);
    try std.testing.expectEqual(9, slot4_and_span5_to_13.end.yes.length.positive);
    try std.testing.expectEqual(13, try slot4_and_span5_to_13.end.yes.endIndex());
}
test "span_end" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot13_and_span4_to_12 = try core.span_end(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(13, slot13_and_span4_to_12.end.index);
    try std.testing.expectEqual(4, slot13_and_span4_to_12.start.yes.start.index);
    try std.testing.expectEqual(9, slot13_and_span4_to_12.start.yes.length.positive);
    try std.testing.expectEqual(12, try slot13_and_span4_to_12.start.yes.endIndex());
}
test "span_start_of_length_positive, normal inputs" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_10_and_11_to_13 = try core.span_start_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 7 } });
    try std.testing.expectEqual(11, span4_to_10_and_11_to_13.after.yes.start.index);
    try std.testing.expectEqual(13, try span4_to_10_and_11_to_13.after.yes.endIndex());
    try std.testing.expectEqual(4, span4_to_10_and_11_to_13.start.start.index);
    try std.testing.expectEqual(10, try span4_to_10_and_11_to_13.start.endIndex());
}
test "span_start_of_length_positive, given length > given span length" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_13_and_empty = try core.span_start_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 10000 } });
    try std.testing.expectEqual(0, (try core.opt_span_length(ExampleOrigin, span4_to_13_and_empty.after)).length);
    try std.testing.expectEqual(4, span4_to_13_and_empty.start.start.index);
    try std.testing.expectEqual(13, try span4_to_13_and_empty.start.endIndex());
}
test "span_end_of_length_positive, normal inputs" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_10_and_11_to_13 = try core.span_end_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 3 } });
    try std.testing.expectEqual(11, span4_to_10_and_11_to_13.end.start.index);
    try std.testing.expectEqual(13, try span4_to_10_and_11_to_13.end.endIndex());
    try std.testing.expectEqual(4, span4_to_10_and_11_to_13.before.yes.start.index);
    try std.testing.expectEqual(10, try span4_to_10_and_11_to_13.before.yes.endIndex());
}
test "span_end_of_length_positive, given length > given span length" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const span4_to_13_and_empty = try core.span_end_of_length_positive(ExampleOrigin, .{ .span = span4_to_13, .length = .{ .positive = 10000 } });
    try std.testing.expectEqual(0, (try core.opt_span_length(ExampleOrigin, span4_to_13_and_empty.before)).length);
    try std.testing.expectEqual(4, span4_to_13_and_empty.end.start.index);
    try std.testing.expectEqual(13, try span4_to_13_and_empty.end.endIndex());
}
test "span_fold up" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const index_sum = try core.opt_span_fold(ExampleOrigin, u32, .{
        .span = core.Opt(core.Span(ExampleOrigin)){ .yes = span4_to_13 },
        .direction = .{ .up = {} },
        .state = 0,
        .step = struct {
            pub fn step(current: core.@".slot.state"(core.Slot(ExampleOrigin), u32)) error{OutOfMemory}!u32 {
                return current.state +| current.slot.index;
            }
        }.step,
    });
    try std.testing.expectEqual(85, index_sum);
}
test "span_fold down" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    var reverse_indexes_array_list = try core.opt_span_fold(ExampleOrigin, std.ArrayList(u32), .{
        .span = core.Opt(core.Span(ExampleOrigin)){ .yes = span4_to_13 },
        .direction = .{ .down = {} },
        .state = std.ArrayList(u32).empty,
        .step = struct {
            pub fn step(
                current: core.@".slot.state"(core.Slot(ExampleOrigin), std.ArrayList(u32)),
            ) error{OutOfMemory}!std.ArrayList(u32) {
                var modified_array_list = current.state;
                try modified_array_list.append(std.testing.allocator, current.slot.index);
                return modified_array_list;
            }
        }.step,
    });
    try std.testing.expectEqualSlices(
        u32,
        &.{ 13, 12, 11, 10, 9, 8, 7, 6, 5, 4 },
        reverse_indexes_array_list.items,
    );
    reverse_indexes_array_list.deinit(std.testing.allocator);
}
test "unset_span_start" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Unset_span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot4_and_span5_to_13 = try core.unset_span_start(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(4, slot4_and_span5_to_13.start.index);
    try std.testing.expectEqual(5, slot4_and_span5_to_13.end.yes.start.index);
    try std.testing.expectEqual(9, slot4_and_span5_to_13.end.yes.length.positive);
    try std.testing.expectEqual(13, try slot4_and_span5_to_13.end.yes.endIndex());
}
test "unset_span_end" {
    const ExampleOrigin = enum { vec };
    const span4_to_13 = core.Unset_span(ExampleOrigin){ .start = .{ .index = 4 }, .length = core.P32.fromComptime(10) };
    const slot13_and_span4_to_12 = try core.unset_span_end(ExampleOrigin, span4_to_13);
    try std.testing.expectEqual(13, slot13_and_span4_to_12.end.index);
    try std.testing.expectEqual(4, slot13_and_span4_to_12.start.yes.start.index);
    try std.testing.expectEqual(9, slot13_and_span4_to_12.start.yes.length.positive);
    try std.testing.expectEqual(12, try slot13_and_span4_to_12.start.yes.endIndex());
}
test "array create" {
    const ExampleArrayRecord = struct { e0: u32, e1: u32 };
    const example_array0 = core.record_to_array(ExampleArrayRecord{ .e0 = 0, .e1 = 2 });
    try std.testing.expectEqualSlices(u32, &[_]u32{ 0, 2 }, &example_array0);
    // we can just specify them as arrays directly
    const example_array1 = [_]u32{ @as(u32, 0), @as(u32, 2) };
    try std.testing.expectEqualSlices(u32, &example_array1, &example_array0);
    try std.testing.expectEqual(@TypeOf(example_array1), @TypeOf(example_array0));
    // or as anonymus structs (unrelated record type, but nobody can care)
    // Which means sloe doesn't even need to collect and generate record types etc.
    // I do not think this is possible in rust but happy to be proven wrong
    const example_array2 = core.record_to_array(.{ .e0 = @as(u32, 0), .e1 = @as(u32, 2) });
    try std.testing.expectEqualSlices(u32, &example_array2, &example_array0);
    try std.testing.expectEqual(@TypeOf(example_array2), @TypeOf(example_array0));
}
test "vec_add_array" {
    const ExampleOrigin = enum { origin };
    const example_origin: ExampleOrigin = .origin;
    const example_vec = try core.vec_empty(u32, ExampleOrigin, example_origin);
    const ExampleArrayRecord = struct { e0: u32, e1: u32 };
    const example_array0 = core.record_to_array(ExampleArrayRecord{ .e0 = 0, .e1 = 2 });
    const with_array = try core.vec_add_array(u32, ExampleOrigin, ExampleArrayRecord, std.testing.allocator, .{
        .vec = example_vec,
        .new = example_array0,
    });
    try core.vec_rid(u32, ExampleOrigin, std.testing.allocator, with_array.vec);
}
test "vec_opt_span_add_array" {
    const ExampleOrigin = enum { origin };
    const example_origin: ExampleOrigin = .origin;
    const example_vec = try core.vec_empty(u32, ExampleOrigin, example_origin);
    const ExampleArrayRecord = struct { e0: u32, e1: u32 };
    const example_array0 = core.record_to_array(ExampleArrayRecord{ .e0 = 0, .e1 = 2 });
    const with_array = try core.vec_opt_span_add_array(u32, ExampleOrigin, ExampleArrayRecord, std.testing.allocator, .{
        .vec = example_vec,
        .span = .{ .no = {} },
        .new = example_array0,
    });
    try core.vec_rid(u32, ExampleOrigin, std.testing.allocator, with_array.vec);
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
test "vec unset slot" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    try std.testing.expect(core.Slot(VecOrigin) != core.Unset_slot(VecOrigin));
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, u32).empty(origin);
    try std.testing.expectEqual(0, vec.notVacantCount());
    const slot0 = try vec.add(allocator, 123);
    const slot1 = try vec.add(allocator, 456);
    const element0 = vec.unset(slot0);
    try std.testing.expectEqual(123, element0.element);
    try std.testing.expectEqual(0, element0.slot.index);
    const slot0_new = vec.set(element0.slot, 321);
    try std.testing.expectEqual(321, vec.element(slot0_new).*);
    try std.testing.expectEqual(0, slot0_new.index);
    const element1 = vec.unset(slot1);
    try vec.slotRid(allocator, element1.slot);
    vec.rid(allocator);
}
test "vec add to span" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, u32).empty(origin);
    const span0 = try vec.optSpanAdd(allocator, core.Opt(core.Span(VecOrigin)){ .no = {} }, 123);
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
test "vec add strs" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    const vec = core.Vec(VecOrigin, core.Char).empty(origin);
    const with_abcd = try core.vec_char_opt_span_add_str(
        VecOrigin,
        allocator,
        .{ .vec = vec, .span = .{ .no = {} }, .new = "abcd" },
    );
    try std.testing.expectEqual(4, with_abcd.span.yes.length.positive);
    const with_wrenches = try core.vec_char_opt_span_add_str(
        VecOrigin,
        allocator,
        .{ .vec = with_abcd.vec, .span = with_abcd.span, .new = "🔧🔧🔧" },
    );
    try std.testing.expectEqualSlices(
        core.Char,
        &.{ 'a', 'b', 'c', 'd', '🔧', '🔧', '🔧' },
        with_wrenches.vec.optSpanSlice(with_wrenches.span),
    );
    try std.testing.expectEqual(7, with_wrenches.span.yes.length.positive);
    with_wrenches.vec.rid(allocator);
}
test "vec char add numbers" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    const vec = core.Vec(VecOrigin, core.Char).empty(origin);
    const with_u32 = try core.vec_char_opt_span_add_u32(
        VecOrigin,
        allocator,
        .{ .vec = vec, .span = .{ .no = {} }, .new = 1234 },
    );
    try std.testing.expectEqual(4, with_u32.span.length.positive);
    const with_i32 = try core.vec_char_span_add_i32(
        VecOrigin,
        allocator,
        .{ .vec = with_u32.vec, .span = with_u32.span, .new = -2 },
    );
    try std.testing.expectEqual(6, with_i32.span.length.positive);
    const with_f32 = try core.vec_char_span_add_f32(
        VecOrigin,
        allocator,
        .{ .vec = with_i32.vec, .span = with_i32.span, .new = -0.1 },
    );
    try std.testing.expectEqualSlices(
        core.Char,
        &.{ '1', '2', '3', '4', '-', '2', '-', '0', '.', '1' },
        with_f32.vec.spanSlice(with_f32.span),
    );
    with_f32.vec.rid(allocator);
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
test "vec add remove stress test" {
    const allocator = std.testing.allocator;
    const VecOrigin = enum { vec };
    const origin: core.Origin(VecOrigin) = .vec;
    var vec = core.Vec(VecOrigin, usize).empty(origin);
    var slots = std.ArrayList(core.Slot(VecOrigin)).empty;
    for (0..100) |i| {
        try slots.append(allocator, try vec.add(allocator, i));
    }
    var rng = std.Random.DefaultPrng.init(std.testing.random_seed);
    var random = rng.random();
    random.shuffle(core.Slot(VecOrigin), slots.items);
    for (slots.items) |slot| {
        _ = try vec.remove(allocator, slot);
    }
    slots.deinit(allocator);
    try std.testing.expectEqual(0, vec.vacant.items.len);
    try std.testing.expectEqual(0, vec.elements.items.len);
    vec.rid(allocator);
}
test "vec into unset slice then reuse" {
    const allocator = std.testing.allocator;
    const AOrigin = enum { origin };
    const a_origin: core.Origin(AOrigin) = .origin;
    var a_vec = core.Vec(AOrigin, usize).empty(a_origin);
    try a_vec.preAllocateAtLeast(allocator, 20);
    const a_capacity = a_vec.elements.capacity;
    try std.testing.expect(a_capacity >= 20);
    const unset_slice = a_vec.intoUnsetSlice(allocator);
    const BOrigin = enum { origin };
    const b_origin: core.Origin(BOrigin) = .origin;
    var b_vec = core.Vec(BOrigin, usize).reuse(b_origin, unset_slice);
    try std.testing.expectEqual(0, b_vec.elements.items.len);
    try std.testing.expectEqual(a_capacity, b_vec.elements.capacity);
    b_vec.rid(allocator);
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
    const origin: core.Origin(Origin) = .origin;
    var vec = core.Vec(Origin, i63).reuse(origin, unset_slice_i63);
    try std.testing.expectEqual(0, vec.elements.items.len);
    try std.testing.expectEqual(unset_slice_u64_length, vec.elements.capacity);
    vec.rid(allocator);
}
test "unset_slice_cast_or_rid_and_allocate u64 to struct{u32,u16}" {
    const allocator = std.testing.allocator;
    const unset_slice_u64 = try core.unset_slice_allocate_length(u64, allocator, 20);
    const unset_slice_u64_length = unset_slice_u64.length();
    try std.testing.expect(unset_slice_u64_length >= 20);
    const unset_slice_tuple_u32_u16 = try core.unset_slice_cast_or_rid_and_allocate(u64, struct { u32, u16 }, allocator, unset_slice_u64);
    const Origin = enum { origin };
    const origin: core.Origin(Origin) = .origin;
    var vec = core.Vec(Origin, struct { u32, u16 }).reuse(origin, unset_slice_tuple_u32_u16);
    try std.testing.expectEqual(0, vec.elements.items.len);
    try std.testing.expectEqual(unset_slice_u64_length, vec.elements.capacity);
    vec.rid(allocator);
}
test "Unset_span != Span" {
    const Origin = enum { origin };
    const a_span: core.Span(Origin) = .{ .start = .{ .index = 0 }, .length = core.P32.one };
    const b_span: core.Unset_span(Origin) = .{ .start = .{ .index = 0 }, .length = core.P32.one };
    try std.testing.expect(@TypeOf(a_span) != @TypeOf(b_span));
}
test "Unset_slot != Slot" {
    const Origin = enum { origin };
    const a_slot: core.Slot(Origin) = .{ .index = 0 };
    const b_slot: core.Unset_slot(Origin) = .{ .index = 0 };
    try std.testing.expect(@TypeOf(a_slot) != @TypeOf(b_slot));
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
    try expect_fn(core.vec_pre_allocate_at_least);
    try expect_fn(core.vec_pre_allocation_rid);
    try expect_fn(core.vec_add);
    try expect_fn(core.vec_insert);
    try expect_fn(core.vec_insert_unset);
    try expect_fn(core.vec_add_unset);
    try expect_fn(core.vec_add_unset_length);
    try expect_fn(core.vec_add_unset_length_positive);
    try expect_fn(core.vec_span_add);
    try expect_fn(core.vec_span_add_vec_span);
    try expect_fn(core.vec_span_add_vec_opt_span);
    try expect_fn(core.vec_char_opt_span_add_str);
    try expect_fn(core.vec_char_span_add_str);
    try expect_fn(core.vec_opt_span_add);
    try expect_fn(core.vec_opt_span_add_vec_span);
    try expect_fn(core.vec_opt_span_add_vec_opt_span);
    try expect_fn(core.vec_span_add_own_span);
    try expect_fn(core.vec_span_add_own_opt_span);
    try expect_fn(core.vec_opt_span_add_own_span);
    try expect_fn(core.vec_opt_span_add_own_opt_span);
    try expect_fn(core.vec_unset_span_add_own_span);
    try expect_fn(core.vec_unset_span_add_own_opt_span);
    try expect_fn(core.vec_opt_unset_span_add_own_span);
    try expect_fn(core.vec_opt_unset_span_add_own_opt_span);
    try expect_fn(core.vec_reuse);
    try expect_fn(core.vec_to_unset);
    try expect_fn(core.unset_slice_allocate_length);
    try expect_fn(core.unset_slice_length);
    try expect_fn(core.unset_slice_cast_or_rid_and_allocate);
    try expect_fn(core.unset_slice_rid);
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
    // const one = @as(union(enum) { no: void, yes: core.Str }, .{ .yes = "" });
    // const two = @as(union(enum) { no: void, yes: core.Str }, .{ .yes = "" });
    // rid_both(union(enum) { no: void, yes: core.Str }, one, two);
    const one = @as(core.@"|no|yes"(void, core.Str), .{ .yes = "" });
    const two = @as(core.@"|no|yes"(void, core.Str), .{ .yes = "" });
    rid_both(core.@"|no|yes"(void, core.Str), one, two);
    try std.testing.expectEqualDeep(one, two);
}
fn rid_both(value: type, _: value, _: value) void {}
