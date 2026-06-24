const std = @import("std");

// If you're wondering about the strange names:
// - @"|variant_a|variant_b" and @".field_a.field_b": since zig removed
//   support for proper anonymous union(enum)s and structs outside of tuples,
//   this workaround is necessary to make zig believe they all belong to the same type
// - @"%Type" for type variables to not overlap with existing type names
// - @"%variable" for expression variables to not overlap with existing file-scope const/fn names.
// For the last 2, alternative naming schemes like `var_name` are much harder to
// properly disambiguate. E.g. what if a sloe name actually also starts with var-?
// Prefixing with % is inspired by LLVM IR. Could have just as well used $ or others.
//
// When writing core declarations, this is a little error prone. It is what it is

pub fn @"Record.a.b"(@"%A": type, @"%B": type) type {
    return struct { a: @"%A", b: @"%B" };
}
pub fn @".a.b"(@"%a": anytype, @"%b": anytype) @"Record.a.b"(@TypeOf(@"%a"), @TypeOf(@"%b")) {
    return .{ .a = @"%a", .b = @"%b" };
}
pub fn @"Record.p.u"(@"%P": type, @"%U": type) type {
    return struct { p: @"%P", u: @"%U" };
}
pub fn @".p.u"(@"%p": anytype, @"%u": anytype) @"Record.p.u"(@TypeOf(@"%p"), @TypeOf(@"%u")) {
    return .{ .p = @"%p", .u = @"%u" };
}
pub fn @"Record.mode.n"(@"%Mode": type, @"%N": type) type {
    return struct { mode: @"%Mode", n: @"%N" };
}
pub fn @".mode.n"(@"%mode": anytype, @"%n": anytype) @"Record.mode.n"(@TypeOf(@"%mode"), @TypeOf(@"%n")) {
    return .{ .mode = @"%mode", .n = @"%n" };
}
pub fn @"Record.new.vec"(@"%New": type, @"%Vec": type) type {
    return struct { new: @"%New", vec: @"%Vec" };
}
pub fn @".new.vec"(@"%new": anytype, @"%vec": anytype) @"Record.new.vec"(@TypeOf(@"%new"), @TypeOf(@"%vec")) {
    return .{ .new = @"%new", .vec = @"%vec" };
}
pub fn @"Record.new.out"(@"%New": type, @"%Out": type) type {
    return struct { new: @"%New", vec: @"%Out" };
}
pub fn @".new.out"(@"%new": anytype, @"%out": anytype) @"Record.new.out"(@TypeOf(@"%new"), @TypeOf(@"%out")) {
    return .{ .new = @"%new", .out = @"%out" };
}
pub fn @"Record.slot.vec"(@"%Slot": type, @"%Vec": type) type {
    return struct { slot: @"%Slot", vec: @"%Vec" };
}
pub fn @".slot.vec"(@"%slot": anytype, @"%vec": anytype) @"Record.slot.vec"(@TypeOf(@"%slot"), @TypeOf(@"%vec")) {
    return .{ .slot = @"%slot", .vec = @"%vec" };
}
pub fn @"Record.span.vec"(@"%Span": type, @"%Vec": type) type {
    return struct { span: @"%Span", vec: @"%Vec" };
}
pub fn @".span.vec"(@"%span": anytype, @"%vec": anytype) @"Record.span.vec"(@TypeOf(@"%span"), @TypeOf(@"%vec")) {
    return .{ .span = @"%span", .vec = @"%vec" };
}
pub fn @"Record.new.slot.vec"(@"%New": type, @"%Slot": type, vec: type) type {
    return struct { new: @"%New", slot: @"%Slot", vec: vec };
}
pub fn @".new.slot.vec"(@"%new": anytype, @"%slot": anytype, @"%vec": anytype) @"Record.new.slot.vec"(@TypeOf(@"%new"), @TypeOf(@"%slot"), @TypeOf(@"%vec")) {
    return .{ .slot = @"%slot", .vec = @"%vec" };
}
pub fn @"Record.origin_rid.slot"(@"%Origin_rid": type, @"%Slot": type) type {
    return struct { origin_rid: @"%Origin_rid", slot: @"%Slot" };
}
pub fn @".origin_rid.slot"(@"%origin_rid": anytype, @"%slot": anytype) @"Record.origin_rid.slot"(@TypeOf(@"%origin_rid"), @TypeOf(@"%slot")) {
    return .{ .origin_rid = @"%origin_rid", .slot = @"%slot" };
}
pub fn @"Record.origin_rid.span"(@"%Origin_rid": type, @"%Span": type) type {
    return struct { origin_rid: @"%Origin_rid", span: @"%Span" };
}
pub fn @".origin_rid.span"(@"%origin_rid": anytype, @"%span": anytype) @"Record.origin_rid.span"(@TypeOf(@"%origin_rid"), @TypeOf(@"%span")) {
    return .{ .origin_rid = @"%origin_rid", .span = @"%span" };
}
pub fn @"|contained|overflowed"(@"%Contained": type, @"%Overflowed": type) type {
    return union(enum) { contained: @"%Contained", overflowed: @"%Overflowed" };
}
pub fn @"|absent|present"(@"%Absent": type, @"%Present": type) type {
    return union(enum) { absent: @"%Absent", present: @"%Present" };
}
pub fn @"|away_from_0|down|nearest_else_away_from_0|nearest_else_even|toward_0|up"(@"%Away_from_0": type, @"%Down": type, @"%Nearest_else_away_from_0": type, @"%Nearest_else_even": type, @"%Toward_0": type, @"%Up": type) type {
    return union(enum) { away_from_0: @"%Away_from_0", down: @"%Down", nearest_else_away_from_0: @"%Nearest_else_away_from_0", nearest_else_even: @"%Nearest_else_even", toward_0: @"%Toward_0", up: @"%Up" };
}

pub const P32 = struct {
    // zig does not have non-zero number types, yet.
    // This is quite wasteful
    positive: u32,
    pub const one = P32{ .positive = 1 };
    pub const max = P32{ .positive = std.math.maxInt(u32) };
    pub fn predecessor(@"%p": @This()) u32 {
        return @"%p".positive - 1;
    }
    // when dealing with memory, use `addOrOutOfMem` instead
    pub fn addClamp(@"%p": @This(), @"%increase": u32) P32 {
        return .{ .positive = @"%p".positive +| @"%increase" };
    }
    pub fn addOrOutOfMem(@"%p": @This(), @"%increase": u32) error{OutOfMemory}!P32 {
        return .{ .positive = std.math.add(u32, @"%p".positive, @"%increase") catch {
            return error.OutOfMemory;
        } };
    }
    pub fn mulClamp(@"%p": @This(), @"%increase": P32) P32 {
        return .{ .positive = @"%p".positive *| @"%increase".positive };
    }
};
pub const U32 = u32;
pub const I32 = i32;
pub const F32 = f32;
pub const Char = u21;
pub const Str = []const u8;
pub fn Fn(@"%In": type, @"%Out": type) type {
    return fn (@"%In") error{OutOfMemory}!@"%Out";
}
pub fn Opt(@"%Present": type) type {
    return @"|absent|present"(void, @"%Present");
}
pub const Round_mode = @"|away_from_0|down|nearest_else_away_from_0|nearest_else_even|toward_0|up"(void, void, void, void, void, void);
/// This wrapper is largely meaningless in zig. It exists to make it safe on the rust side
pub fn Origin(@"%Origin": type) type {
    return if (@bitSizeOf(@"%Origin") == 0) @"%Origin" else @compileError("Only zero-sized values should be used as origins, as they are stored within slots, spans, vecs etc. Try using e.g. `enum { myExampleVec }`");
}
pub fn Origin_rid(@"%Origin": type) type {
    return struct { origin_rid: @"%Origin" };
}
pub const SpanRaw = struct {
    start: u32,
    length: P32,
    pub fn endIndexUsize(@"%span": @This()) usize {
        return @as(usize, @"%span".start) + @as(usize, @"%span".length.predecessor());
    }
    // the fact that this can return an error and which isn't checked for earlier is a little sad
    pub fn endIndex(@"%span": @This()) error{OutOfMemory}!u32 {
        return std.math.add(u32, @"%span".start, @"%span".length.predecessor()) catch return error.OutOfMemory;
    }
};
pub fn Span(@"%Origin": type) type {
    return struct {
        start: Slot(@"%Origin"),
        length: P32,
        pub fn endIndexUsize(@"%span": @This()) usize {
            return @"%span".raw().endIndexUsize();
        }
        pub fn endIndex(@"%span": @This()) error{OutOfMemory}!u32 {
            return @"%span".raw().endIndex();
        }
        pub fn raw(@"%span": @This()) SpanRaw {
            return .{ .start = @"%span".start.index, .length = @"%span".length };
        }
    };
}
pub fn Slot(@"%Origin": type) type {
    return struct {
        origin: @"%Origin",
        index: u32,
        pub fn to_span(@"%slot": @This()) Span(@"%Origin") {
            return .{ .start = @"%slot", .length = P32.one };
        }
    };
}
/// Usage is only safe when
/// - each vec has a unique origin
/// - returned slots, spans, origin-rids are never mem-copied
/// - vacated spans are respected when accesssing elements
///
/// Additionally, when any of the given-out slots and spans are not returned,
/// be aware that the indexes they pointed to are now stale.
/// So: do not ignore them when they point into a persistent `Vec`
///
/// in general, if you really want to directly access .elements,
/// be extra aware of the ABA problem (e.g. a pointer to an element could point to a wrong, new element instead of invalid memory when its index was vacated and re-populated in between)
pub fn Vec(@"%Origin": type, @"%Element": type) type {
    return struct {
        origin: @"%Origin",
        elements: std.ArrayList(@"%Element"),
        vacant: std.ArrayList(SpanRaw),
        pub fn empty(@"%origin": Origin(@"%Origin")) @This() {
            return .{ .origin = @"%origin", .elements = std.ArrayList(@"%Element").empty, .vacant = std.ArrayList(SpanRaw).empty };
        }
        pub fn rid(@"%vec": @This(), @"%allocator": std.mem.Allocator) void {
            var @"%vec_mut" = @"%vec";
            @"%vec_mut".elements.deinit(@"%allocator");
            @"%vec_mut".vacant.deinit(@"%allocator");
        }
        pub fn vacantSlotCount(@"%vec": @This()) u32 {
            var @"%combined_length": u32 = 0;
            for (@"%vec".vacant.items) |@"%vacant"| {
                @"%combined_length" += @"%vacant".length.positive;
            }
            return @"%combined_length";
        }
        pub fn occupiedCount(@"%vec": @This()) usize {
            return @"%vec".elements.items.len - @"%vec".vacantSlotCount();
        }
        pub fn addIgnoringVacant(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%new_element": @"%Element") error{OutOfMemory}!Slot(@"%Origin") {
            const @"%new_slot" = Slot(@"%Origin"){ .origin = @"%vec".origin, .index = std.math.lossyCast(u32, @"%vec".elements.items.len) };
            try @"%vec".elements.append(@"%allocator", @"%new_element");
            return @"%new_slot";
        }
        pub fn add(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%new_element": @"%Element") error{OutOfMemory}!Slot(@"%Origin") {
            if (@"%vec".vacant.last()) |@"%vacant_span_ref"| {
                const @"%new_slot" = Slot(@"%Origin"){ .origin = @"%vec".origin, .index = @"%vacant_span_ref".start };
                @"%vec".elements.items[@"%vacant_span_ref".start] = @"%new_element";
                if (@"%vacant_span_ref".length.positive >= 2) {
                    @"%vacant_span_ref".length.positive -= 1;
                } else {
                    _ = @"%vec".vacant.pop();
                }
                return @"%new_slot";
            } else {
                return @"%vec".addIgnoringVacant(@"%allocator", @"%new_element");
            }
        }
        pub fn element(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%slot": Slot(@"%Origin")) error{OutOfMemory}!@"%Element" {
            const @"%accessed_element" = @"%vec".elements.items[@"%slot".index];
            try @"%vec".vacateSpan(@"%allocator", @"%slot".to_span());
            return @"%accessed_element";
        }
        pub fn elementUpdate(@"%vec": *@This(), @"%Out": type, @"%slot": Slot(@"%Origin"), @"%in": anytype, @"%update": fn (@TypeOf(@"%in")) @"Record.new.out"(@"%Element", @"%Out")) @"%Out" {
            var @"%element_mut" = @"%vec".elements.items[@"%slot".index];
            const @"%out" = @"%update"(@"%in", @"%element_mut".*);
            @"%element_mut" = @"%out".new;
            return @"%out".out;
        }
        // must be paired with `vacateSpan` (or a length shortening of elements) and not be accessed after.
        // A slightly harder-to-abuse function is spanSliceConsume
        pub fn spanSlice(@"%vec": *@This(), @"%span": Span(@"%Origin")) []@"%Element" {
            return @"%vec".elements.items[@"%span".start.index..(@"%span".endIndexUsize() + 1)];
        }
        pub fn spanSliceConsume(@"%vec": *@This(), @"%Out": type, @"%allocator": std.mem.Allocator, @"%span": Span(@"%Origin"), @"%in": anytype, @"%consume_slice": fn (@TypeOf(@"%in"), []@"%Element") @"%Out") error{OutOfMemory}!@"%Out" {
            const @"%out" = @"%consume_slice"(@"%in", @"%vec".spanSlice(@"%span"));
            try @"%vec".vacateSpan(@"%allocator", @"%span");
            return @"%out";
        }
        /// only use when the element values are safe to not handle or are handled immediately after
        fn vacateSpan(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%span_to_vacate": Span(@"%Origin")) error{OutOfMemory}!void {
            var @"%maybe_vacant_span_index_connecting_earlier": ?usize = null;
            var @"%maybe_vacant_span_index_connecting_later": ?usize = null;
            looking_for_connections: for (@"%vec".vacant.items, 0..) |@"%vacant_span", @"%vacant_span_index"| {
                if (@"%maybe_vacant_span_index_connecting_earlier" == null and @"%span_to_vacate".start.index == @"%vacant_span".endIndexUsize() + 1) {
                    @"%maybe_vacant_span_index_connecting_earlier" = @"%vacant_span_index";
                    if (@"%maybe_vacant_span_index_connecting_later") |_| {
                        break :looking_for_connections;
                    }
                } else if (@"%maybe_vacant_span_index_connecting_later" == null and @"%span_to_vacate".endIndexUsize() + 1 == @"%vacant_span".start) {
                    @"%maybe_vacant_span_index_connecting_later" = @"%vacant_span_index";
                    if (@"%maybe_vacant_span_index_connecting_earlier") |_| {
                        break :looking_for_connections;
                    }
                }
            }
            if (@"%maybe_vacant_span_index_connecting_earlier") |@"%vacant_span_index_connecting_earlier"| {
                var @"%vacantSpanConnectingEarlier" = &@"%vec".vacant.items[@"%vacant_span_index_connecting_earlier"];
                if (@"%maybe_vacant_span_index_connecting_later") |@"%vacant_span_index_connecting_later"| {
                    const @"%vacant_span_connecting_later" = @"%vec".vacant.swapRemove(@"%vacant_span_index_connecting_later");
                    @"%vacantSpanConnectingEarlier".length = try @"%vacantSpanConnectingEarlier".length.addOrOutOfMem((try @"%span_to_vacate".length.addOrOutOfMem(@"%vacant_span_connecting_later".length.positive)).positive);
                } else {
                    // maybeVacantSpanIndexConnectingLater == null
                    @"%vacantSpanConnectingEarlier".length = try @"%vacantSpanConnectingEarlier".length.addOrOutOfMem(@"%span_to_vacate".length.positive);
                }
            } else if (@"%maybe_vacant_span_index_connecting_later") |@"%vacant_span_index_connecting_later"| {
                // maybeVacantSpanIndexConnectingEarlier == null
                var @"%vacant_span_connecting_later" = &@"%vec".vacant.items[@"%vacant_span_index_connecting_later"];
                @"%vacant_span_connecting_later".* = SpanRaw{ .start = @"%span_to_vacate".start.index, .length = try @"%vacant_span_connecting_later".length.addOrOutOfMem(@"%span_to_vacate".length.positive) };
            } else {
                // maybeVacantSpanIndexConnectingEarlier == null and maybeVacantSpanIndexConnectingLater == null
                if (@"%span_to_vacate".endIndexUsize() + 1 == @"%vec".elements.items.len) {
                    @"%vec".elements.shrinkRetainingCapacity(std.math.sub(usize, @"%vec".elements.items.len, @"%span_to_vacate".length.positive) catch 0);
                } else {
                    try @"%vec".vacant.append(@"%allocator", @"%span_to_vacate".raw());
                }
            }
        }
        pub fn moveSpanToEnd(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%span": Span(@"%Origin")) error{OutOfMemory}!Span(@"%Origin") {
            if (@"%span".endIndexUsize() + 1 == @"%vec".elements.items.len) {
                return @"%span";
            }
            // span is not at the end already
            const @"%moved_span" = switch (try @"%vec".addSliceIgnoringVacant(@"%allocator", @"%vec".spanSlice(@"%span"))) {
                .absent => unreachable,
                .present => |moved_span| moved_span,
            };
            try @"%vec".vacateSpan(@"%allocator", @"%span");
            return @"%moved_span";
        }
        pub fn moveSpanToVacant(@"%vec": *@This(), @"%span": Span(@"%Origin")) Span(@"%Origin") {
            if (@"%span".endIndexUsize() + 1 < @"%vec".elements.items.len) {
                return @"%span";
            }
            // span is at the end of elements
            if (@"%vec".mark_length_positive_as_occupied(@"%span".length)) |@"%earlier_start_to_occupy_from"| {
                @"%vec".elements.replaceRangeAssumeCapacity(@"%earlier_start_to_occupy_from", @"%span".length.positive, @"%vec".spanSlice(@"%span"));
                @"%vec".elements.shrinkRetainingCapacity(@"%vec".elements.items.len - @"%span".length.positive);
                return Span(@"%Origin"){ .start = .{ .origin = @"%vec".origin, .index = @"%earlier_start_to_occupy_from" }, .length = @"%span".length };
            } else {
                return @"%span";
            }
        }
        fn mark_length_positive_as_occupied(@"%vec": *@This(), @"%length_to_occupy": P32) ?u32 {
            for (@"%vec".vacant.items, 0..) |*@"%vacant", @"%vacant_index"| {
                if (@"%vacant".length.positive > @"%length_to_occupy".positive) {
                    @"%vacant".length.positive -|= @"%length_to_occupy".positive;
                    return @"%vacant".start;
                } else if (@"%vacant".length.positive == @"%length_to_occupy".positive) {
                    return @"%vec".vacant.swapRemove(@"%vacant_index").start;
                }
            }
            return null;
        }
        pub fn addSliceIgnoringVacant(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%new_elements": []const @"%Element") error{OutOfMemory}!Opt(Span(@"%Origin")) {
            const @"%length_before_add" = @"%vec".elements.items.len;
            try @"%vec".elements.appendSlice(@"%allocator", @"%new_elements");
            return Opt(Span(@"%Origin")){ .present = .{
                .start = .{ .origin = @"%vec".origin, .index = try (std.math.cast(u32, @"%length_before_add") orelse error.OutOfMemory) },
                .length = P32{ .positive = try (std.math.cast(u32, @"%vec".elements.items.len - @"%length_before_add") orelse error.OutOfMemory) },
            } };
        }
        pub fn opt_span_add(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%opt_span": Opt(Span(@"%Origin")), @"%new_element": @"%Element") error{OutOfMemory}!Span(@"%Origin") {
            return switch (@"%opt_span") {
                .absent => (try @"%vec".addIgnoringVacant(@"%allocator", @"%new_element")).to_span(),
                .present => |@"%span"| @"%vec".span_add(@"%allocator", @"%span", @"%new_element"),
            };
        }
        pub fn span_add(@"%vec": *@This(), @"%allocator": std.mem.Allocator, @"%span": Span(@"%Origin"), @"%new_element": @"%Element") error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%vec".moveSpanToEnd(@"%allocator", @"%span");
            try @"%vec".elements.append(@"%allocator", @"%new_element");
            return Span(@"%Origin"){ .start = @"%moved_span".start, .length = try @"%moved_span".length.addOrOutOfMem(1) };
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
pub fn p32_dup(@"%n": P32) error{OutOfMemory}!@"Record.a.b"(P32, P32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn p32_add_clamp(@"%": @"Record.p.u"(P32, U32)) error{OutOfMemory}!P32 {
    return @"%".p.addClamp(@"%".u);
}
pub fn p32_mul_clamp(@"%": @"Record.a.b"(P32, P32)) error{OutOfMemory}!P32 {
    return @"%".a.mulClamp(@"%".b);
}

pub fn u32_rid(_: U32) error{OutOfMemory}!void {}
pub fn u32_dup(@"%n": U32) error{OutOfMemory}!@"Record.a.b"(U32, U32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn u32_add_clamp(@"%": @"Record.a.b"(U32, U32)) error{OutOfMemory}!U32 {
    return @"%".a +| @"%".b;
}
pub fn u32_mul_clamp(@"%": @"Record.a.b"(U32, U32)) error{OutOfMemory}!U32 {
    return @"%".a *| @"%".b;
}

pub fn i32_rid(_: I32) error{OutOfMemory}!void {}
pub fn i32_dup(@"%n": I32) error{OutOfMemory}!@"Record.a.b"(I32, I32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn i32_add_clamp(@"%": @"Record.a.b"(I32, I32)) error{OutOfMemory}!I32 {
    return @"%".a +| @"%".b;
}
pub fn i32_mul_clamp(@"%": @"Record.a.b"(I32, I32)) error{OutOfMemory}!I32 {
    return @"%".a *| @"%".b;
}
pub fn i32_negate_clamp(@"%n": I32) error{OutOfMemory}!I32 {
    return 0 -| @"%n";
}
pub fn i32_abs_to_u32(@"%n": I32) error{OutOfMemory}!U32 {
    return @abs(@"%n");
}

pub fn f32_rid(_: F32) error{OutOfMemory}!void {}
pub fn f32_dup(@"%n": F32) error{OutOfMemory}!@"Record.a.b"(F32, F32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn f32_negate(@"%n": F32) F32 {
    return -@"%n";
}
pub fn f32_abs(@"%n": F32) error{OutOfMemory}!F32 {
    return @abs(@"%n");
}
pub fn f32_round(@"%": @"Record.mode.n"(Round_mode, F32)) error{OutOfMemory}!F32 {
    return switch (@"%".mode) {
        .up => @ceil(@"%".n),
        .down => @floor(@"%".n),
        .toward_0 => @trunc(@"%".n),
        .away_from_0 => @ceil(@abs(@"%".n)) * std.math.sign(@"%".n),
        .nearest_else_away_from_0 => @round(@"%".n),
        .nearest_else_even => {
            // your move zig. Please add an intrinsic
            const @"%mod" = std.math.modf(@"%".n);
            return if (@"%mod".fpart == 0.0) @"%".n
                // @"%".n is on the midpoint
            else if (@abs(@"%mod".fpart) == 0.5)
                (
                    // @"%".n is on the midpoint
                    if (@mod(@"%mod".ipart, 2) == 1)
                        // is odd
                        //  11.5 ->  12
                        // -11.5 -> -12
                        @round(@"%".n)
                    else
                        // @"%".n is even
                        //  10.5 ->  10, not  11
                        // -10.5 -> -10, not -11
                        (@round(@"%".n) - std.math.sign(@"%".n)))
            else
                @round(@"%".n);
        },
    };
}
pub fn f32_to_i32_clamp(@"%": @"Record.mode.n"(Round_mode, F32)) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, f32_round(@"%"));
}
pub fn f32_add_clamp(@"%": @"Record.a.b"(F32, F32)) error{OutOfMemory}!F32 {
    const @"%sum" = @"%".a + @"%".b;
    return if (std.math.isNegativeInf(@"%sum")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%sum")) std.math.floatMax(f32) else @"%sum";
}
pub fn f32_mul_clamp(@"%": @"Record.a.b"(F32, F32)) error{OutOfMemory}!F32 {
    const @"%product" = @"%".a * @"%".b;
    return if (std.math.isNegativeInf(@"%product")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%product")) std.math.floatMax(f32) else @"%product";
}
pub fn f32_div_clamp(@"%": @"Record.a.b"(F32, F32)) error{OutOfMemory}!F32 {
    return if (@"%".b == 0) 0 else {
        const @"%div_result" = @"%".a / @"%".b;
        return if (std.math.isNegativeInf(@"%div_result")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%div_result")) std.math.floatMax(f32) else @"%div_result";
    };
}

pub fn char_rid(_: Char) error{OutOfMemory}!void {}
pub fn char_dup(@"%n": Char) error{OutOfMemory}!@"Record.a.b"(Char, Char) {
    return .{ .a = @"%n", .b = @"%n" };
}

pub fn str_rid(_: Str) error{OutOfMemory}!void {}
pub fn str_dup(@"%n": Str) error{OutOfMemory}!@"Record.a.b"(Str, Str) {
    return .{ .a = @"%n", .b = @"%n" };
}

pub fn fn_rid(@"%In": type, @"%Out": type, _: Fn(@"%In", @"%Out")) error{OutOfMemory}!void {}
pub fn fn_dup(@"%In": type, @"%Out": type, @"%function": Fn(@"%In", @"%Out")) error{OutOfMemory}!@"Record.a.b"(Fn(@"%In", @"%Out"), Fn(@"%In", @"%Out")) {
    return .{ .a = @"%function", .b = @"%function" };
}

pub fn origin_rid(@"%Origin": type, _: Origin(@"%Origin")) error{OutOfMemory}!void {}

pub fn origin_rid_rid(@"%Origin": type, _: Origin_rid(@"%Origin")) error{OutOfMemory}!void {}
pub fn origin_rid_dup(@"%Origin": type, @"%origin_rid_proof": Origin_rid(@"%Origin")) error{OutOfMemory}!@"Record.a.b"(Origin_rid(@"%Origin"), Origin_rid(@"%Origin")) {
    return .{ .a = @"%origin_rid_proof", .b = @"%origin_rid_proof" };
}

pub fn slot_rid(@"%Origin": type, _: @"Record.origin_rid.slot"(Origin_rid(@"%Origin"), Slot(@"%Origin"))) error{OutOfMemory}!void {}
pub fn slot_to_span(@"%Origin": type, @"%slot": Slot(@"%Origin")) Span(@"%Origin") {
    return @"%slot".to_span();
}

pub fn span_rid(@"%Origin": type, _: @"Record.origin_rid.span"(Origin_rid(@"%Origin"), Span(@"%Origin"))) error{OutOfMemory}!void {}

pub fn vec_empty(@"%Origin": type, @"%Element": type, @"%origin": Origin(@"%Origin")) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    return Vec(@"%Origin", @"%Element").empty(@"%origin");
}
pub fn vec_add(@"%Origin": type, @"%Element": type, @"%allocator": std.mem.Allocator, @"%": @"Record.new.vec"(@"%Element", Vec(@"%Origin", @"%Element"))) error{OutOfMemory}!@"Record.slot.vec"(Slot(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%slot" = try @"%".vec.add(@"%allocator", @"%".new);
    return .{ .vec = @"%".vec, .slot = @"%slot" };
}
pub fn vec_add_ignoring_vacant(@"%Origin": type, @"%Element": type, @"%allocator": std.mem.Allocator, @"%": @"Record.new.vec"(@"%Element", Vec(@"%Origin", @"%Element"))) error{OutOfMemory}!@"Record.slot.vec"(Slot(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%slot" = try @"%".vec.addIgnoringVacant(@"%allocator", @"%".new);
    return .{ .vec = @"%".vec, .slot = @"%slot" };
}
pub fn vec_rid(@"%Origin": type, @"%Element": type, @"%allocator": std.mem.Allocator, @"%vec": Vec(@"%Origin", @"%Element")) error{OutOfMemory}!void {
    @"%vec".rid(@"%allocator");
}
