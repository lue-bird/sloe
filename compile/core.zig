const std = @import("std");

pub fn @"Record.a.b"(VarA: type, VarB: type) type {
    return struct { a: VarA, b: VarB };
}
pub fn @".a.b"(var_a: anytype, var_b: anytype) @"Record.a.b"(@TypeOf(var_a), @TypeOf(var_b)) {
    return .{ .a = var_a, .b = var_b };
}
pub fn @"Record.p.u"(VarP: type, VarU: type) type {
    return struct { p: VarP, u: VarU };
}
pub fn @".p.u"(var_p: anytype, var_u: anytype) @"Record.p.u"(@TypeOf(var_p), @TypeOf(var_u)) {
    return .{ .p = var_p, .u = var_u };
}
pub fn @"Record.mode.n"(VarMode: type, VarN: type) type {
    return struct { mode: VarMode, n: VarN };
}
pub fn @".mode.n"(var_mode: anytype, var_n: anytype) @"Record.mode.n"(@TypeOf(var_mode), @TypeOf(var_n)) {
    return .{ .mode = var_mode, .n = var_n };
}
pub fn @"Record.new.vec"(VarNew: type, VarVec: type) type {
    return struct { new: VarNew, vec: VarVec };
}
pub fn @".new.vec"(var_new: anytype, var_vec: anytype) @"Record.new.vec"(@TypeOf(var_new), @TypeOf(var_vec)) {
    return .{ .new = var_new, .vec = var_vec };
}
pub fn @"Record.new.out"(VarNew: type, VarOut: type) type {
    return struct { new: VarNew, vec: VarOut };
}
pub fn @".new.out"(var_new: anytype, var_out: anytype) @"Record.new.out"(@TypeOf(var_new), @TypeOf(var_out)) {
    return .{ .new = var_new, .out = var_out };
}
pub fn @"Record.slot.vec"(VarSlot: type, VarVec: type) type {
    return struct { slot: VarSlot, vec: VarVec };
}
pub fn @".slot.vec"(var_slot: anytype, var_vec: anytype) @"Record.slot.vec"(@TypeOf(var_slot), @TypeOf(var_vec)) {
    return .{ .slot = var_slot, .vec = var_vec };
}
pub fn @"Record.span.vec"(VarSpan: type, VarVec: type) type {
    return struct { span: VarSpan, vec: VarVec };
}
pub fn @".span.vec"(var_span: anytype, var_vec: anytype) @"Record.span.vec"(@TypeOf(var_span), @TypeOf(var_vec)) {
    return .{ .span = var_span, .vec = var_vec };
}
pub fn @"Record.new.slot.vec"(VarNew: type, VarSlot: type, vec: type) type {
    return struct { new: VarNew, slot: VarSlot, vec: vec };
}
pub fn @".new.slot.vec"(var_new: anytype, var_slot: anytype, var_vec: anytype) @"Record.new.slot.vec"(@TypeOf(var_new), @TypeOf(var_slot), @TypeOf(var_vec)) {
    return .{ .slot = var_slot, .vec = var_vec };
}
pub fn @"Record.origin_rid.slot"(VarOrigin_rid: type, VarSlot: type) type {
    return struct { origin_rid: VarOrigin_rid, slot: VarSlot };
}
pub fn @".origin_rid.slot"(var_origin_rid: anytype, var_slot: anytype) @"Record.origin_rid.slot"(@TypeOf(var_origin_rid), @TypeOf(var_slot)) {
    return .{ .origin_rid = var_origin_rid, .slot = var_slot };
}
pub fn @"Record.origin_rid.span"(VarOrigin_rid: type, VarSpan: type) type {
    return struct { origin_rid: VarOrigin_rid, span: VarSpan };
}
pub fn @".origin_rid.span"(var_origin_rid: anytype, var_span: anytype) @"Record.origin_rid.span"(@TypeOf(var_origin_rid), @TypeOf(var_span)) {
    return .{ .origin_rid = var_origin_rid, .span = var_span };
}
pub fn @"|contained|overflowed"(VarContained: type, VarOverflowed: type) type {
    return union(enum) { contained: VarContained, overflowed: VarOverflowed };
}
pub fn @"|absent|present"(VarAbsent: type, VarPresent: type) type {
    return union(enum) { absent: VarAbsent, present: VarPresent };
}
pub fn @"|away_from_0|down|nearest_else_away_from_0|nearest_else_even|toward_0|up"(VarAway_from_0: type, VarDown: type, VarNearest_else_away_from_0: type, VarNearest_else_even: type, VarToward_0: type, VarUp: type) type {
    return union(enum) { away_from_0: VarAway_from_0, down: VarDown, nearest_else_away_from_0: VarNearest_else_away_from_0, nearest_else_even: VarNearest_else_even, toward_0: VarToward_0, up: VarUp };
}

pub const P32 = struct {
    // zig does not have non-zero number types, yet.
    // This is quite wasteful
    positive: u32,
    pub const one = P32{ .positive = 1 };
    pub const max = P32{ .positive = std.math.maxInt(u32) };
    pub fn predecessor(p: @This()) u32 {
        return p.positive - 1;
    }
    // when dealing with memory, use `addOrOutOfMem` instead
    pub fn addClamp(var_p: @This(), var_increase: u32) P32 {
        return .{ .positive = var_p.positive +| var_increase };
    }
    pub fn addOrOutOfMem(var_p: @This(), var_increase: u32) error{OutOfMemory}!P32 {
        return .{ .positive = std.math.add(u32, var_p.positive, var_increase) catch {
            return error.OutOfMemory;
        } };
    }
    pub fn mulClamp(var_p: @This(), var_increase: P32) P32 {
        return .{ .positive = var_p.positive *| var_increase.positive };
    }
};
pub const U32 = u32;
pub const I32 = i32;
pub const F32 = f32;
pub const Char = u21;
pub const Str = []const u8;
pub fn Fn(VarIn: type, VarOut: type) type {
    return fn (VarIn) error{OutOfMemory}!VarOut;
}
pub fn Opt(VarPresent: type) type {
    return @"|absent|present"(void, VarPresent);
}
pub const Round_mode = @"|away_from_0|down|nearest_else_away_from_0|nearest_else_even|toward_0|up"(void, void, void, void, void, void);
/// This wrapper is largely meaningless in zig. It exists to make it safe on the rust side
pub fn Origin(VarOrigin: type) type {
    return if (@bitSizeOf(VarOrigin) == 0) VarOrigin else @compileError("Only zero-sized values should be used as origins, as they are stored within slots, spans, vecs etc. Try using e.g. `enum { myExampleVec }`");
}
pub fn Origin_rid(VarOrigin: type) type {
    return struct { origin_rid: VarOrigin };
}
pub const SpanRaw = struct {
    start: u32,
    length: P32,
    pub fn endIndexUsize(var_span: @This()) usize {
        return @as(usize, var_span.start) + @as(usize, var_span.length.predecessor());
    }
    // the fact that this can return an error and which isn't checked for earlier is a little sad
    pub fn endIndex(var_span: @This()) error{OutOfMemory}!u32 {
        return std.math.add(u32, var_span.start, var_span.length.predecessor()) catch return error.OutOfMemory;
    }
};
pub fn Span(VarOrigin: type) type {
    return struct {
        start: Slot(VarOrigin),
        length: P32,
        pub fn endIndexUsize(var_span: @This()) usize {
            return var_span.raw().endIndexUsize();
        }
        pub fn endIndex(var_span: @This()) error{OutOfMemory}!u32 {
            return var_span.raw().endIndex();
        }
        pub fn raw(var_span: @This()) SpanRaw {
            return .{ .start = var_span.start.index, .length = var_span.length };
        }
    };
}
pub fn Slot(VarOrigin: type) type {
    return struct {
        origin: VarOrigin,
        index: u32,
        pub fn to_span(var_slot: @This()) Span(VarOrigin) {
            return .{ .start = var_slot, .length = P32.one };
        }
    };
}
/// Usage is only safe when
/// - each vec has a unique origin
/// - returned slots, spans, origin-rids are never mem-copied
/// - vacated spans are respected when accesssing elements
///
/// in general, if you really want to directly access .elements,
/// be extra aware of the ABA problem (e.g. a pointer to an element could point to a wrong, new element instead of invalid memory when its index was vacated and re-populated in between)
pub fn Vec(VarOrigin: type, VarElement: type) type {
    return struct {
        origin: VarOrigin,
        elements: std.ArrayList(VarElement),
        vacant: std.ArrayList(SpanRaw),
        pub fn empty(var_origin: Origin(VarOrigin)) @This() {
            return .{ .origin = var_origin, .elements = std.ArrayList(VarElement).empty, .vacant = std.ArrayList(SpanRaw).empty };
        }
        pub fn rid(var_vec: @This(), var_allocator: std.mem.Allocator) void {
            var vec_mut = var_vec;
            vec_mut.elements.deinit(var_allocator);
            vec_mut.vacant.deinit(var_allocator);
        }
        pub fn vacantSlotCount(var_vec: @This()) u32 {
            var var_combined_length: u32 = 0;
            for (var_vec.vacant.items) |var_vacant| {
                var_combined_length += var_vacant.length.positive;
            }
            return var_combined_length;
        }
        pub fn occupiedCount(var_vec: @This()) usize {
            return var_vec.elements.items.len - var_vec.vacantSlotCount();
        }
        pub fn addIgnoringVacant(var_vec: *@This(), var_allocator: std.mem.Allocator, var_new_element: VarElement) error{OutOfMemory}!Slot(VarOrigin) {
            const var_new_slot = Slot(VarOrigin){ .origin = var_vec.origin, .index = std.math.lossyCast(u32, var_vec.elements.items.len) };
            try var_vec.elements.append(var_allocator, var_new_element);
            return var_new_slot;
        }
        pub fn add(var_vec: *@This(), var_allocator: std.mem.Allocator, var_new_element: VarElement) error{OutOfMemory}!Slot(VarOrigin) {
            if (var_vec.vacant.last()) |var_vacant_span_ref| {
                const var_new_slot = Slot(VarOrigin){ .origin = var_vec.origin, .index = var_vacant_span_ref.start };
                var_vec.elements.items[var_vacant_span_ref.start] = var_new_element;
                if (var_vacant_span_ref.length.positive >= 2) {
                    var_vacant_span_ref.length.positive -= 1;
                } else {
                    _ = var_vec.vacant.pop();
                }
                return var_new_slot;
            } else {
                return var_vec.addIgnoringVacant(var_allocator, var_new_element);
            }
        }
        pub fn element(var_vec: *@This(), var_allocator: std.mem.Allocator, var_slot: Slot(VarOrigin)) error{OutOfMemory}!VarElement {
            const var_accessed_element = var_vec.elements.items[var_slot.index];
            try var_vec.vacateSpan(var_allocator, var_slot.to_span());
            return var_accessed_element;
        }
        pub fn elementUpdate(var_vec: *@This(), VarOut: type, var_slot: Slot(VarOrigin), var_in: anytype, var_update: fn (@TypeOf(var_in)) @"Record.new.out"(VarElement, VarOut)) VarOut {
            var element_mut = var_vec.elements.items[var_slot.index];
            const out = var_update(var_in, element_mut.*);
            element_mut = out.new;
            return out.out;
        }
        // must be paired with `vacateSpan` (or a length shortening of elements) and not be accessed after.
        // A slightly harder-to-abuse function is spanSliceConsume
        pub fn spanSlice(var_vec: *@This(), var_span: Span(VarOrigin)) []VarElement {
            return var_vec.elements.items[var_span.start.index..(var_span.endIndexUsize() + 1)];
        }
        pub fn spanSliceConsume(var_vec: *@This(), VarOut: type, var_allocator: std.mem.Allocator, var_span: Span(VarOrigin), var_in: anytype, var_consume_slice: fn (@TypeOf(var_in), []VarElement) VarOut) error{OutOfMemory}!VarOut {
            const var_out = var_consume_slice(var_in, var_vec.spanSlice(var_span));
            try var_vec.vacateSpan(var_allocator, var_span);
            return var_out;
        }
        /// only use when the element values are safe to not handle or are handled immediately after
        fn vacateSpan(var_vec: *@This(), var_allocator: std.mem.Allocator, var_span_to_vacate: Span(VarOrigin)) error{OutOfMemory}!void {
            var var_maybe_vacant_span_index_connecting_earlier: ?usize = null;
            var var_maybe_vacant_span_index_connecting_later: ?usize = null;
            looking_for_connections: for (var_vec.vacant.items, 0..) |vacant_span, vacant_span_index| {
                if (var_maybe_vacant_span_index_connecting_earlier == null and var_span_to_vacate.start.index == vacant_span.endIndexUsize() + 1) {
                    var_maybe_vacant_span_index_connecting_earlier = vacant_span_index;
                    if (var_maybe_vacant_span_index_connecting_later) |_| {
                        break :looking_for_connections;
                    }
                } else if (var_maybe_vacant_span_index_connecting_later == null and var_span_to_vacate.endIndexUsize() + 1 == vacant_span.start) {
                    var_maybe_vacant_span_index_connecting_later = vacant_span_index;
                    if (var_maybe_vacant_span_index_connecting_earlier) |_| {
                        break :looking_for_connections;
                    }
                }
            }
            if (var_maybe_vacant_span_index_connecting_earlier) |vacantSpanIndexConnectingEarlier| {
                var vacantSpanConnectingEarlier = &var_vec.vacant.items[vacantSpanIndexConnectingEarlier];
                if (var_maybe_vacant_span_index_connecting_later) |vacantSpanIndexConnectingLater| {
                    const var_vacant_span_connecting_later = var_vec.vacant.swapRemove(vacantSpanIndexConnectingLater);
                    vacantSpanConnectingEarlier.length = try vacantSpanConnectingEarlier.length.addOrOutOfMem((try var_span_to_vacate.length.addOrOutOfMem(var_vacant_span_connecting_later.length.positive)).positive);
                } else {
                    // maybeVacantSpanIndexConnectingLater == null
                    vacantSpanConnectingEarlier.length = try vacantSpanConnectingEarlier.length.addOrOutOfMem(var_span_to_vacate.length.positive);
                }
            } else if (var_maybe_vacant_span_index_connecting_later) |vacantSpanIndexConnectingLater| {
                // maybeVacantSpanIndexConnectingEarlier == null
                var var_vacant_span_connecting_later = &var_vec.vacant.items[vacantSpanIndexConnectingLater];
                var_vacant_span_connecting_later.* = SpanRaw{ .start = var_span_to_vacate.start.index, .length = try var_vacant_span_connecting_later.length.addOrOutOfMem(var_span_to_vacate.length.positive) };
            } else {
                // maybeVacantSpanIndexConnectingEarlier == null and maybeVacantSpanIndexConnectingLater == null
                if (var_span_to_vacate.endIndexUsize() + 1 == var_vec.elements.items.len) {
                    var_vec.elements.shrinkRetainingCapacity(std.math.sub(usize, var_vec.elements.items.len, var_span_to_vacate.length.positive) catch 0);
                } else {
                    try var_vec.vacant.append(var_allocator, var_span_to_vacate.raw());
                }
            }
        }
        pub fn moveSpanToEnd(vec: *@This(), var_allocator: std.mem.Allocator, span: Span(VarOrigin)) error{OutOfMemory}!Span(VarOrigin) {
            if (span.endIndexUsize() + 1 == vec.elements.items.len) {
                return span;
            }
            // span is not at the end already
            const var_moved_span = switch (try vec.addSliceIgnoringVacant(var_allocator, vec.spanSlice(span))) {
                .absent => unreachable,
                .present => |moved_span| moved_span,
            };
            try vec.vacateSpan(var_allocator, span);
            return var_moved_span;
        }
        pub fn moveSpanToVacant(var_vec: *@This(), var_span: Span(VarOrigin)) Span(VarOrigin) {
            if (var_span.endIndexUsize() + 1 < var_vec.elements.items.len) {
                return var_span;
            }
            // span is at the end of elements
            if (var_vec.mark_length_positive_as_occupied(var_span.length)) |earlier_start_to_occupy_from| {
                var_vec.elements.replaceRangeAssumeCapacity(earlier_start_to_occupy_from, var_span.length.positive, var_vec.spanSlice(var_span));
                var_vec.elements.shrinkRetainingCapacity(var_vec.elements.items.len - var_span.length.positive);
                return Span(VarOrigin){ .start = .{ .origin = var_vec.origin, .index = earlier_start_to_occupy_from }, .length = var_span.length };
            } else {
                return var_span;
            }
        }
        fn mark_length_positive_as_occupied(var_vec: *@This(), var_length_to_occupy: P32) ?u32 {
            for (var_vec.vacant.items, 0..) |*var_vacant, var_vacant_index| {
                if (var_vacant.length.positive > var_length_to_occupy.positive) {
                    var_vacant.length.positive -|= var_length_to_occupy.positive;
                    return var_vacant.start;
                } else if (var_vacant.length.positive == var_length_to_occupy.positive) {
                    return var_vec.vacant.swapRemove(var_vacant_index).start;
                }
            }
            return null;
        }
        pub fn addSliceIgnoringVacant(var_vec: *@This(), var_allocator: std.mem.Allocator, var_new_elements: []const VarElement) error{OutOfMemory}!Opt(Span(VarOrigin)) {
            const var_length_before_add = var_vec.elements.items.len;
            try var_vec.elements.appendSlice(var_allocator, var_new_elements);
            return Opt(Span(VarOrigin)){ .present = .{
                .start = .{ .origin = var_vec.origin, .index = try (std.math.cast(u32, var_length_before_add) orelse error.OutOfMemory) },
                .length = P32{ .positive = try (std.math.cast(u32, var_vec.elements.items.len - var_length_before_add) orelse error.OutOfMemory) },
            } };
        }
        pub fn opt_span_add(var_vec: *@This(), var_allocator: std.mem.Allocator, var_opt_span: Opt(Span(VarOrigin)), var_new_element: VarElement) error{OutOfMemory}!Span(VarOrigin) {
            return switch (var_opt_span) {
                .absent => (try var_vec.addIgnoringVacant(var_allocator, var_new_element)).to_span(),
                .present => |var_span| var_vec.span_add(var_allocator, var_span, var_new_element),
            };
        }
        pub fn span_add(vec_vec: *@This(), var_allocator: std.mem.Allocator, var_span: Span(VarOrigin), new_element: VarElement) error{OutOfMemory}!Span(VarOrigin) {
            const var_moved_span = try vec_vec.moveSpanToEnd(var_allocator, var_span);
            try vec_vec.elements.append(var_allocator, new_element);
            return Span(VarOrigin){ .start = var_moved_span.start, .length = try var_moved_span.length.addOrOutOfMem(1) };
        }
    };
}

// You may notice that even infallible functions that can be called by sloe-generated code
// return `error{OutOfMemory}!...`.
// This is to avoid deeper analysis of e.g. when to `try`, when to annotate and convert errors etc.
// It also makes for a consistent `Fn` type alias.
// This might seem wasteful but I'd be surprised if optimizers couldn't reduce un-raisd errors away.
// The biggest actual issue I can forsee is that zig code calling into sloe-generated zig code
// may lead to unnecessary error handling (which IMO is fine because sloe code could always change to a fallible operation).
//
// If you're running the sloe code in a loop, there honestly isn't much you can do on the zig side
// once you're running out of memory as you won't receive a new state value
// (and any array pointers you may have stored independently are likely stale and/or corrupted).
// For typical applications, just exiting is fine, though hopefully a little more gracefully than
// if every sloe-generated zig function used `catch @panic` instead of returning an error.
// If this is not an option, try to e.g. keep deep-cloned state value "backups" or similar.
// (or just don't use sloe)

pub fn p32_rid(_: P32) error{OutOfMemory}!void {}
pub fn p32_dup(var_n: P32) error{OutOfMemory}!@"Record.a.b"(P32, P32) {
    return .{ .a = var_n, .b = var_n };
}
pub fn p32_add_clamp(var_: @"Record.p.u"(P32, U32)) error{OutOfMemory}!P32 {
    return var_.p.addClamp(var_.u);
}
pub fn p32_mul_clamp(var_: @"Record.a.b"(P32, P32)) error{OutOfMemory}!P32 {
    return var_.a.mulClamp(var_.b);
}

pub fn u32_rid(_: U32) error{OutOfMemory}!void {}
pub fn u32_dup(n: U32) error{OutOfMemory}!@"Record.a.b"(U32, U32) {
    return .{ .a = n, .b = n };
}
pub fn u32_add_clamp(var_: @"Record.a.b"(U32, U32)) error{OutOfMemory}!U32 {
    return var_.a +| var_.b;
}
pub fn u32_mul_clamp(var_: @"Record.a.b"(U32, U32)) error{OutOfMemory}!U32 {
    return var_.a *| var_.b;
}

pub fn i32_rid(_: I32) error{OutOfMemory}!void {}
pub fn i32_dup(n: I32) error{OutOfMemory}!@"Record.a.b"(I32, I32) {
    return .{ .a = n, .b = n };
}
pub fn i32_add_clamp(var_: @"Record.a.b"(I32, I32)) error{OutOfMemory}!I32 {
    return var_.a +| var_.b;
}
pub fn i32_mul_clamp(var_: @"Record.a.b"(I32, I32)) error{OutOfMemory}!I32 {
    return var_.a *| var_.b;
}
pub fn i32_negate_clamp(n: I32) error{OutOfMemory}!I32 {
    return 0 -| n;
}
pub fn i32_abs_to_u32(n: I32) error{OutOfMemory}!U32 {
    return @abs(n);
}

pub fn f32_rid(_: F32) error{OutOfMemory}!void {}
pub fn f32_dup(n: F32) error{OutOfMemory}!@"Record.a.b"(F32, F32) {
    return .{ .a = n, .b = n };
}
pub fn f32_negate(n: F32) F32 {
    return -n;
}
pub fn f32_abs(n: F32) error{OutOfMemory}!F32 {
    return @abs(n);
}
pub fn f32_round(var_: @"Record.mode.n"(Round_mode, F32)) error{OutOfMemory}!F32 {
    return switch (var_.mode) {
        .up => @ceil(var_.n),
        .down => @floor(var_.n),
        .toward_0 => @trunc(var_.n),
        .away_from_0 => @ceil(@abs(var_.n)) * std.math.sign(var_.n),
        .nearest_else_away_from_0 => @round(var_.n),
        .nearest_else_even => {
            // your move zig. Please add an intrinsic
            const mod = std.math.modf(var_.n);
            return if (mod.fpart == 0.0) var_.n
                // var_.n is on the midpoint
            else if (@abs(mod.fpart) == 0.5)
                (
                    // var_.n is on the midpoint
                    if (@mod(mod.ipart, 2) == 1)
                        // is odd
                        //  11.5 ->  12
                        // -11.5 -> -12
                        @round(var_.n)
                    else
                        // var_.n is even
                        //  10.5 ->  10, not  11
                        // -10.5 -> -10, not -11
                        (@round(var_.n) - std.math.sign(var_.n)))
            else
                @round(var_.n);
        },
    };
}
pub fn f32_to_i32_clamp(var_: @"Record.mode.n"(Round_mode, F32)) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, f32_round(var_));
}
pub fn f32_add_clamp(var_: @"Record.a.b"(F32, F32)) error{OutOfMemory}!F32 {
    const sum = var_.a + var_.b;
    return if (std.math.isNegativeInf(sum)) std.math.floatMin(f32) else if (std.math.isPositiveInf(sum)) std.math.floatMax(f32) else sum;
}
pub fn f32_mul_clamp(var_: @"Record.a.b"(F32, F32)) error{OutOfMemory}!F32 {
    const product = var_.a * var_.b;
    return if (std.math.isNegativeInf(product)) std.math.floatMin(f32) else if (std.math.isPositiveInf(product)) std.math.floatMax(f32) else product;
}
pub fn f32_div_clamp(var_: @"Record.a.b"(F32, F32)) error{OutOfMemory}!F32 {
    return if (var_.b == 0) 0 else {
        const div_result = var_.a / var_.b;
        return if (std.math.isNegativeInf(div_result)) std.math.floatMin(f32) else if (std.math.isPositiveInf(div_result)) std.math.floatMax(f32) else div_result;
    };
}

pub fn char_rid(_: Char) error{OutOfMemory}!void {}
pub fn char_dup(n: Char) error{OutOfMemory}!@"Record.a.b"(Char, Char) {
    return .{ .a = n, .b = n };
}

pub fn str_rid(_: Str) error{OutOfMemory}!void {}
pub fn str_dup(n: Str) error{OutOfMemory}!@"Record.a.b"(Str, Str) {
    return .{ .a = n, .b = n };
}

pub fn fn_rid(VarIn: type, VarOut: type, _: Fn(VarIn, VarOut)) error{OutOfMemory}!void {}
pub fn fn_dup(VarIn: type, VarOut: type, function: Fn(VarIn, VarOut)) error{OutOfMemory}!@"Record.a.b"(Fn(VarIn, VarOut), Fn(VarIn, VarOut)) {
    return .{ .a = function, .b = function };
}

pub fn origin_rid(VarOrigin: type, _: Origin(VarOrigin)) error{OutOfMemory}!void {}

pub fn origin_rid_rid(VarOrigin: type, _: Origin_rid(VarOrigin)) error{OutOfMemory}!void {}
pub fn origin_rid_dup(VarOrigin: type, origin_rid_proof: Origin_rid(VarOrigin)) error{OutOfMemory}!@"Record.a.b"(Origin_rid(VarOrigin), Origin_rid(VarOrigin)) {
    return .{ .a = origin_rid_proof, .b = origin_rid_proof };
}

pub fn slot_rid(VarOrigin: type, _: @"Record.origin_rid.slot"(Origin_rid(VarOrigin), Slot(VarOrigin))) error{OutOfMemory}!void {}
pub fn slot_to_span(VarOrigin: type, slot: Slot(VarOrigin)) Span(VarOrigin) {
    return slot.to_span();
}

pub fn span_rid(VarOrigin: type, _: @"Record.origin_rid.span"(Origin_rid(VarOrigin), Span(VarOrigin))) error{OutOfMemory}!void {}

pub fn vec_empty(VarOrigin: type, VarElement: type, origin: Origin(VarOrigin)) error{OutOfMemory}!Vec(VarOrigin, VarElement) {
    return Vec(VarOrigin, VarElement).empty(origin);
}
pub fn vec_add(VarOrigin: type, VarElement: type, var_allocator: std.mem.Allocator, var_: @"Record.new.vec"(VarElement, Vec(VarOrigin, VarElement))) error{OutOfMemory}!@"Record.slot.vec"(Slot(VarOrigin), Vec(VarOrigin, VarElement)) {
    const slot = try var_.vec.add(var_allocator, var_.new);
    return .{ .vec = var_.vec, .slot = slot };
}
pub fn vec_add_ignoring_vacant(VarOrigin: type, VarElement: type, var_allocator: std.mem.Allocator, var_: @"Record.new.vec"(VarElement, Vec(VarOrigin, VarElement))) error{OutOfMemory}!@"Record.slot.vec"(Slot(VarOrigin), Vec(VarOrigin, VarElement)) {
    const slot = try var_.vec.addIgnoringVacant(var_allocator, var_.new);
    return .{ .vec = var_.vec, .slot = slot };
}
pub fn vec_rid(VarOrigin: type, VarElement: type, var_allocator: std.mem.Allocator, vec: Vec(VarOrigin, VarElement)) error{OutOfMemory}!void {
    vec.rid(var_allocator);
}
