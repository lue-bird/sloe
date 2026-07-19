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

pub fn @".a.b"(@"%A": type, @"%B": type) type {
    return struct { a: @"%A", b: @"%B" };
}
pub fn @"record.a.b"(@"%a": anytype, @"%b": anytype) @".a.b"(@TypeOf(@"%a"), @TypeOf(@"%b")) {
    return .{ .a = @"%a", .b = @"%b" };
}
pub fn @".p.u"(@"%P": type, @"%U": type) type {
    return struct { p: @"%P", u: @"%U" };
}
pub fn @"record.p.u"(@"%p": anytype, @"%u": anytype) @".p.u"(@TypeOf(@"%p"), @TypeOf(@"%u")) {
    return .{ .p = @"%p", .u = @"%u" };
}
pub fn @".mode.n"(@"%Mode": type, @"%N": type) type {
    return struct { mode: @"%Mode", n: @"%N" };
}
pub fn @"record.mode.n"(@"%mode": anytype, @"%n": anytype) @".mode.n"(@TypeOf(@"%mode"), @TypeOf(@"%n")) {
    return .{ .mode = @"%mode", .n = @"%n" };
}
pub fn @".end.start"(@"%End": type, @"%Start": type) type {
    return struct { end: @"%End", start: @"%Start" };
}
pub fn @"record.end.start"(@"%end": anytype, @"%start": anytype) @".end.start"(@TypeOf(@"%end"), @TypeOf(@"%start")) {
    return .{ .end = @"%end", .start = @"%start" };
}
pub fn @".after.start"(@"%After": type, @"%Start": type) type {
    return struct { after: @"%After", start: @"%Start" };
}
pub fn @"record.after.start"(@"%after": anytype, @"%start": anytype) @".after.start"(@TypeOf(@"%after"), @TypeOf(@"%start")) {
    return .{ .after = @"%after", .start = @"%start" };
}
pub fn @".before.end"(@"%Before": type, @"%End": type) type {
    return struct { before: @"%Before", end: @"%End" };
}
pub fn @"record.before.end"(@"%before": anytype, @"%end": anytype) @".before.end"(@TypeOf(@"%before"), @TypeOf(@"%end")) {
    return .{ .before = @"%before", .end = @"%end" };
}
pub fn @".index.slot"(@"%Index": type, @"%Slot": type) type {
    return struct { index: @"%Index", slot: @"%Slot" };
}
pub fn @"record.slot.index"(@"%index": anytype, @"%slot": anytype) @".index.slot"(@TypeOf(@"%index"), @TypeOf(@"%slot")) {
    return .{ .index = @"%index", .slot = @"%slot" };
}
pub fn @".length.span"(@"%Length": type, @"%Span": type) type {
    return struct { length: @"%Length", span: @"%Span" };
}
pub fn @"record.length.span"(@"%length": anytype, @"%span": anytype) @".length.span"(@TypeOf(@"%length"), @TypeOf(@"%span")) {
    return .{ .length = @"%length", .span = @"%span" };
}
pub fn @".slice.span"(@"%Slice": type, @"%Span": type) type {
    return struct { slice: @"%Slice", span: @"%Span" };
}
pub fn @"record.slice.span"(@"%slice": anytype, @"%span": anytype) @".slice.span"(@TypeOf(@"%slice"), @TypeOf(@"%span")) {
    return .{ .slice = @"%slice", .span = @"%span" };
}
pub fn @".new.vec"(@"%New": type, @"%Vec": type) type {
    return struct { new: @"%New", vec: @"%Vec" };
}
pub fn @"record.new.vec"(@"%new": anytype, @"%vec": anytype) @".new.vec"(@TypeOf(@"%new"), @TypeOf(@"%vec")) {
    return .{ .new = @"%new", .vec = @"%vec" };
}
pub fn @".out.vec"(@"%Out": type, @"%Vec": type) type {
    return struct { out: @"%Out", vec: @"%Vec" };
}
pub fn @"record.out.vec"(@"%out": anytype, @"%vec": anytype) @".out.vec"(@TypeOf(@"%out"), @TypeOf(@"%vec")) {
    return .{ .out = @"%out", .vec = @"%vec" };
}
pub fn @".new.out"(@"%New": type, @"%Out": type) type {
    return struct { new: @"%New", vec: @"%Out" };
}
pub fn @"record.new.out"(@"%new": anytype, @"%out": anytype) @".new.out"(@TypeOf(@"%new"), @TypeOf(@"%out")) {
    return .{ .new = @"%new", .out = @"%out" };
}
pub fn @".element.in"(@"%Element": type, @"%In": type) type {
    return struct { element: @"%Element", vec: @"%In" };
}
pub fn @"record.element.in"(@"%element": anytype, @"%in": anytype) @".element.in"(@TypeOf(@"%element"), @TypeOf(@"%in")) {
    return .{ .element = @"%element", .in = @"%in" };
}
pub fn @".element.out"(@"%Element": type, @"%Out": type) type {
    return struct { element: @"%Element", out: @"%Out" };
}
pub fn @"record.element.out"(@"%element": anytype, @"%out": anytype) @".element.out"(@TypeOf(@"%element"), @TypeOf(@"%out")) {
    return .{ .element = @"%element", .out = @"%out" };
}
pub fn @".element.slot"(@"%Element": type, @"%Slot": type) type {
    return struct { element: @"%Element", slot: @"%Slot" };
}
pub fn @"record.element.slot"(@"%element": anytype, @"%slot": anytype) @".element.slot"(@TypeOf(@"%element"), @TypeOf(@"%slot")) {
    return .{ .element = @"%element", .slot = @"%slot" };
}
pub fn @".element.vec"(@"%Element": type, @"%Vec": type) type {
    return struct { element: @"%Element", vec: @"%Vec" };
}
pub fn @"record.element.vec"(@"%element": anytype, @"%vec": anytype) @".element.vec"(@TypeOf(@"%element"), @TypeOf(@"%vec")) {
    return .{ .element = @"%element", .vec = @"%vec" };
}
pub fn @".element.slot.vec"(@"%Element": type, @"%Slot": type, @"%Vec": type) type {
    return struct { element: @"%Element", slot: @"%Slot", vec: @"%Vec" };
}
pub fn @"record.element.slot.vec"(@"%element": anytype, @"%slot": anytype, @"%vec": anytype) @".element.slot.vec"(@TypeOf(@"%element"), @TypeOf(@"%slot"), @TypeOf(@"%vec")) {
    return .{ .element = @"%element", .slot = @"%slot", .vec = @"%vec" };
}
pub fn @".slot.vec"(@"%Slot": type, @"%Vec": type) type {
    return struct { slot: @"%Slot", vec: @"%Vec" };
}
pub fn @"record.slot.vec"(@"%slot": anytype, @"%vec": anytype) @".slot.vec"(@TypeOf(@"%slot"), @TypeOf(@"%vec")) {
    return .{ .slot = @"%slot", .vec = @"%vec" };
}
pub fn @".length.vec"(@"%Length": type, @"%Vec": type) type {
    return struct { length: @"%Length", vec: @"%Vec" };
}
pub fn @"record.length.vec"(@"%length": anytype, @"%vec": anytype) @".length.vec"(@TypeOf(@"%length"), @TypeOf(@"%vec")) {
    return .{ .length = @"%length", .vec = @"%vec" };
}
pub fn @".length.slice"(@"%Length": type, @"%Slice": type) type {
    return struct { length: @"%Length", slice: @"%Slice" };
}
pub fn @"record.length.slice"(@"%length": anytype, @"%slice": anytype) @".length.slice"(@TypeOf(@"%length"), @TypeOf(@"%slice")) {
    return .{ .length = @"%length", .slice = @"%slice" };
}
pub fn @".origin.slice"(@"%Origin": type, @"%Slice": type) type {
    return struct { origin: @"%Origin", slice: @"%Slice" };
}
pub fn @"record.origin.slice"(@"%origin": anytype, @"%slice": anytype) @".origin.slice"(@TypeOf(@"%origin"), @TypeOf(@"%slice")) {
    return .{ .origin = @"%origin", .slice = @"%slice" };
}
pub fn @".slot.state"(@"%Slot": type, @"%State": type) type {
    return struct { slot: @"%Slot", state: @"%State" };
}
pub fn @"record.slot.state"(@"%slot": anytype, @"%state": anytype) @".slot.state"(@TypeOf(@"%slot"), @TypeOf(@"%state")) {
    return .{ .slot = @"%slot", .state = @"%state" };
}
pub fn @".span.vec"(@"%Span": type, @"%Vec": type) type {
    return struct { span: @"%Span", vec: @"%Vec" };
}
pub fn @"record.span.vec"(@"%span": anytype, @"%vec": anytype) @".span.vec"(@TypeOf(@"%span"), @TypeOf(@"%vec")) {
    return .{ .span = @"%span", .vec = @"%vec" };
}
pub fn @".span.state.step"(@"%Span": type, @"%State": type, @"%Step": type) type {
    return struct { span: @"%Span", state: @"%State", step: @"%Step" };
}
pub fn @"record.span.state.step"(@"%span": anytype, @"%state": anytype, @"%step": anytype) @".span.state.step"(@TypeOf(@"%span"), @TypeOf(@"%state"), @TypeOf(@"%step")) {
    return .{ .span = @"%span", .state = @"%state", .step = @"%step" };
}
pub fn @".new.span.vec"(@"%New": type, @"%Span": type, @"%Vec": type) type {
    return struct { new: @"%New", span: @"%Span", vec: @"%Vec" };
}
pub fn @"record.new.span.vec"(@"%new": anytype, @"%span": anytype, @"%vec": anytype) @".new.span.vec"(@TypeOf(@"%new"), @TypeOf(@"%span"), @TypeOf(@"%vec")) {
    return .{ .new = @"%new", .span = @"%span", .vec = @"%vec" };
}
pub fn @".end.start.vec"(@"%End": type, @"%Start": type, @"%Vec": type) type {
    return struct { end: @"%End", start: @"%Start", vec: @"%Vec" };
}
pub fn @"record.end.start.vec"(@"%end": anytype, @"%start": anytype, @"%vec": anytype) @".end.start.vec"(@TypeOf(@"%end"), @TypeOf(@"%start"), @TypeOf(@"%vec")) {
    return .{ .end = @"%end", .start = @"%start", .vec = @"%vec" };
}
pub fn @".source.source_span.span.vec"(@"%Source": type, @"%Source_span": type, @"%Span": type, @"%Vec": type) type {
    return struct { source: @"%Source", source_span: @"%Source_span", span: @"%Span", vec: @"%Vec" };
}
pub fn @"record.source.source_span.span.vec"(@"%source": anytype, @"%source_span": anytype, @"%span": anytype, @"%vec": anytype) @".source.source_span.span.vec"(@TypeOf(@"%source"), @TypeOf(@"%source_span"), @TypeOf(@"%span"), @TypeOf(@"%vec")) {
    return .{ .source = @"%source", .source_span = @"%source_span", .span = @"%span", .vec = @"%vec" };
}
pub fn @".new.slot.vec"(@"%New": type, @"%Slot": type, @"%Vec": type) type {
    return struct { new: @"%New", slot: @"%Slot", vec: @"%Vec" };
}
pub fn @"record.new.slot.vec"(@"%new": anytype, @"%slot": anytype, @"%vec": anytype) @".new.slot.vec"(@TypeOf(@"%new"), @TypeOf(@"%slot"), @TypeOf(@"%vec")) {
    return .{ .new = @"%new", .slot = @"%slot", .vec = @"%vec" };
}
pub fn @".in.slot.update.vec"(@"%In": type, @"%Slot": type, @"%Update": type, @"%Vec": type) type {
    return struct { in: @"%In", slot: @"%Slot", update: @"%Update", vec: @"%Vec" };
}
pub fn @"record.in.slot.update.vec"(@"%in": anytype, @"%slot": anytype, @"%update": anytype, @"%vec": anytype) @".in.slot.update.vec"(@TypeOf(@"%in"), @TypeOf(@"%slot"), @TypeOf(@"%update"), @TypeOf(@"%vec")) {
    return .{ .in = @"%in", .slot = @"%slot", .update = @"%update", .vec = @"%vec" };
}
pub fn @"|empty"(@"%Empty": type) type {
    return union(enum) { empty: @"%Empty" };
}
pub fn @"|occupied"(@"%Occupied": type) type {
    return union(enum) { occupied: @"%Occupied" };
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
    // This is quite wasteful.
    // The one tiny performance benefit is that .predecessor() never underflows on valid P32s
    positive: u32,
    pub const one = P32{ .positive = 1 };
    pub const maxValue = P32{ .positive = std.math.maxInt(u32) };
    pub fn fromComptime(comptime @"%u32": u32) @This() {
        return if (@"%u32" == 0) @compileError("given unsigned integer is not positive") else .{ .positive = @"%u32" };
    }
    pub fn fromU32(@"%u32": U32) ?@This() {
        return if (@"%u32" == 0) null else .{ .positive = @"%u32" };
    }
    pub fn predecessor(@"%p": @This()) u32 {
        return @"%p".positive - 1;
    }
    // when dealing with memory, use `addOrOutOfMem` instead
    pub fn addClamp(@"%p": @This(), @"%increase": u32) P32 {
        return .{ .positive = @"%p".positive +| @"%increase" };
    }
    pub fn addOrOutOfMem(@"%p": @This(), @"%increase": u32) error{OutOfMemory}!P32 {
        return .{ .positive = try u32AddOrOutOfMem(@"%p".positive, @"%increase") };
    }
    pub fn mulClamp(@"%p": @This(), @"%increase": P32) P32 {
        return .{ .positive = @"%p".positive *| @"%increase".positive };
    }
    pub fn min(@"%a": @This(), @"%b": P32) P32 {
        return .{ .positive = @min(@"%a".positive, @"%b".positive) };
    }
    pub fn max(@"%a": @This(), @"%b": P32) P32 {
        return .{ .positive = @max(@"%a".positive, @"%b".positive) };
    }
};
pub const U32 = u32;
pub const I32 = i32;
pub const F32 = f32;
pub const Char = u21;
/// assumed to be valid (!) UTF-8.
/// This means using e.g. "\xff\xfe" might lead to UB
pub const Str = []const u8;
pub fn Fn(@"%In": type, @"%Out": type) type {
    return *const fn (@"%In") error{OutOfMemory}!@"%Out";
}
pub fn Opt(@"%Present": type) type {
    return @"|absent|present"(void, @"%Present");
}
pub const Round_mode = @"|away_from_0|down|nearest_else_away_from_0|nearest_else_even|toward_0|up"(void, void, void, void, void, void);

fn u32AddOrOutOfMem(a: u32, b: u32) error{OutOfMemory}!u32 {
    const sum, const overflow = @addWithOverflow(a, b);
    return if (overflow != 0) error.OutOfMemory else sum;
}
fn usizeAddOrOutOfMem(a: usize, b: usize) error{OutOfMemory}!usize {
    const sum, const overflow = @addWithOverflow(a, b);
    return if (overflow != 0) error.OutOfMemory else sum;
}

/// This wrapper is largely meaningless in zig. It exists to make it safe on the rust side.
/// I have tried to patch in some mechanisms to avoid having multiple origins with the same name in a scope
/// but due to the (reasonable) lack of comptime mutable variables/mutable pointers it can't be done
pub fn Origin(@"%Origin": type) type {
    const @"%is_valid" = switch (@typeInfo(@"%Origin")) {
        .@"enum" => |enum_info| (enum_info.field_names.len == 1 and @bitSizeOf(@"%Origin") == 0),
        else => false,
    };
    if (!@"%is_valid") @compileError(std.fmt.comptimePrint(
        "Only zero-sized enum values should be used as origins, as they are stored within slots, spans, vecs etc. and should be safe to copy and free (found bit size {} for origin type {}). Easiest is to just put `enum {{ origin }}`",
        .{ @bitSizeOf(@"%Origin"), @"%Origin" },
    ));
    return @"%Origin";
}
pub fn Slot_with_occupancy(@"%Origin": type, @"%Occupancy": type) type {
    return struct {
        origin: @"%Origin",
        index: u32,
        const occupancy = @"%Occupancy";
        pub fn to_span(@"%slot": @This()) Span_with_occupancy(@"%Origin", @"%Occupancy") {
            return .{ .start = @"%slot", .length = P32.one };
        }
    };
}
pub fn Slot(@"%Origin": type) type {
    return Slot_with_occupancy(@"%Origin", OccupancySet);
}
pub fn Unset_slot(@"%Origin": type) type {
    return Slot_with_occupancy(@"%Origin", OccupancyUnset);
}
pub const OccupancySet = enum {};
pub const OccupancyUnset = enum {};
pub fn Span_with_occupancy(@"%Origin": type, @"%Occupancy": type) type {
    return struct {
        start: Slot_with_occupancy(@"%Origin", @"%Occupancy"),
        length: P32,
        pub fn endIndexUsize(@"%span": @This()) usize {
            return @as(usize, @"%span".start.index) + @as(usize, @"%span".length.predecessor());
        }
        pub fn endIndex(@"%span": @This()) error{OutOfMemory}!u32 {
            return u32AddOrOutOfMem(@"%span".start.index, @"%span".length.predecessor());
        }
        pub fn splitStart(@"%span": @This()) error{OutOfMemory}!@".end.start"(
            Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
            Slot_with_occupancy(@"%Origin", @"%Occupancy"),
        ) {
            return .{
                .start = @"%span".start,
                .end = if (P32.fromU32(@"%span".length.predecessor())) |@"%end_length"|
                    .{ .present = .{
                        .start = .{
                            .origin = @"%span".start.origin,
                            .index = try u32AddOrOutOfMem(@"%span".start.index, 1),
                        },
                        .length = @"%end_length",
                    } }
                else
                    .{ .absent = {} },
            };
        }
        pub fn splitEnd(@"%span": @This()) error{OutOfMemory}!@".end.start"(
            Slot_with_occupancy(@"%Origin", @"%Occupancy"),
            Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
        ) {
            return .{
                .end = .{ .origin = @"%span".start.origin, .index = try @"%span".endIndex() },
                .start = if (P32.fromU32(@"%span".length.predecessor())) |@"%start_length"|
                    .{ .present = .{
                        .start = @"%span".start,
                        .length = @"%start_length",
                    } }
                else
                    .{ .absent = {} },
            };
        }
        pub fn splitAfterLengthPositive(
            @"%span": @This(),
            @"%start_length_or_greater": P32,
        ) @".after.start"(
            Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
            Span_with_occupancy(@"%Origin", @"%Occupancy"),
        ) {
            const @"%start_length" = P32.min(@"%start_length_or_greater", @"%span".length);
            return .{
                .start = .{ .start = @"%span".start, .length = @"%start_length" },
                .after = if (P32.fromU32(@"%span".length.positive - @"%start_length".positive)) |@"%after_length_positive"| .{
                    .present = .{
                        .start = .{
                            .origin = @"%span".start.origin,
                            .index = @"%span".start.index + @"%start_length".positive,
                        },
                        .length = @"%after_length_positive",
                    },
                } else .{ .absent = {} },
            };
        }
        pub fn splitBeforeEndLengthPositive(
            @"%span": @This(),
            @"%end_length_or_greater": P32,
        ) @".before.end"(
            Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
            Span_with_occupancy(@"%Origin", @"%Occupancy"),
        ) {
            const @"%end_length" = P32.min(@"%end_length_or_greater", @"%span".length);
            const @"%before_length" = @"%span".length.positive - @"%end_length".positive;
            return .{
                .end = .{
                    .start = .{
                        .origin = @"%span".start.origin,
                        .index = @"%span".start.index + @"%before_length",
                    },
                    .length = @"%end_length",
                },
                .before = if (P32.fromU32(@"%before_length")) |@"%before_length_positive"| .{
                    .present = .{
                        .start = @"%span".start,
                        .length = @"%before_length_positive",
                    },
                } else .{ .absent = {} },
            };
        }
        pub fn fold(
            @"%span": Span(@"%Origin"),
            @"%initial_state": anytype,
            @"%step": Fn(@".slot.state"(Slot(@"%Origin"), @TypeOf(@"%initial_state")), @TypeOf(@"%initial_state")),
        ) error{OutOfMemory}!@TypeOf(@"%initial_state") {
            var @"%state" = @"%initial_state";
            for (@"%span".start.index..(try @"%span".length.addOrOutOfMem(@"%span".start.index)).positive) |index| {
                @"%state" = try @"%step"(.{
                    .state = @"%state",
                    .slot = .{ .origin = @"%span".start.origin, .index = std.math.lossyCast(u32, index) },
                });
            }
            return @"%state";
        }
    };
}
pub fn Span(@"%Origin": type) type {
    return Span_with_occupancy(@"%Origin", OccupancySet);
}
pub fn Unset_span(@"%Origin": type) type {
    return Span_with_occupancy(@"%Origin", OccupancyUnset);
}
/// slice whose actual items are undefined
pub fn Unset_slice(@"%Element": type) type {
    return struct {
        undefined_items: []@"%Element",

        pub fn allocateLength(
            @"%allocator": std.mem.Allocator,
            @"%length": u32,
        ) error{OutOfMemory}!@This() {
            return .{ .undefined_items = try @"%allocator".alloc(@"%Element", @"%length") };
        }
        pub fn length(@"%unset_slice": @This()) u32 {
            return std.math.lossyCast(u32, @"%unset_slice".undefined_items.len);
        }
        /// the given unset slice is invalid after
        pub fn transmuteOrRidAndAllocate(
            @"%unset_slice": @This(),
            @"%NewElement": type,
            @"%allocator": std.mem.Allocator,
        ) error{OutOfMemory}!Unset_slice(@"%NewElement") {
            if (@sizeOf(@"%NewElement") == @sizeOf(@"%Element")) {
                return .{ .undefined_items = @as([]@"%NewElement", @ptrCast(@"%unset_slice".undefined_items)) };
            } else {
                @"%unset_slice".rid(@"%allocator");
                return Unset_slice(@"%NewElement").allocateLength(@"%allocator", @"%unset_slice".length());
            }
        }
        pub fn rid(@"%unset_slice": @This(), @"%allocator": std.mem.Allocator) void {
            return @"%allocator".free(@"%unset_slice".undefined_items);
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
        origin: Origin(@"%Origin"),
        elements: std.ArrayList(@"%Element"),
        vacant: std.ArrayList(Unset_span(@"%Origin")),

        pub fn empty(@"%origin": Origin(@"%Origin")) @This() {
            return .{
                .origin = @"%origin",
                .elements = std.ArrayList(@"%Element").empty,
                .vacant = std.ArrayList(Unset_span(@"%Origin")).empty,
            };
        }
        pub fn reuse(@"%origin": Origin(@"%Origin"), @"%unset_slice": Unset_slice(@"%Element")) @This() {
            var elements = std.ArrayList(@"%Element").fromOwnedSlice(@"%unset_slice".undefined_items);
            elements.clearRetainingCapacity();
            return .{
                .origin = @"%origin",
                .elements = elements,
                .vacant = std.ArrayList(Unset_span(@"%Origin")).empty,
            };
        }
        pub fn preAllocateAtLeast(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%min_pre_allocated_length": u32,
        ) error{OutOfMemory}!void {
            return @"%vec".elements.ensureUnusedCapacity(@"%allocator", @"%min_pre_allocated_length");
        }
        pub fn preAllocationRid(@"%vec": *@This(), @"%allocator": std.mem.Allocator) void {
            return @"%vec".elements.shrinkAndFreePrecise(@"%allocator", @"%vec".elements.items.len);
        }
        pub fn vacantSlotCount(@"%vec": @This()) u32 {
            var @"%combined_length": u32 = 0;
            for (@"%vec".vacant.items) |@"%vacant"| {
                @"%combined_length" += @"%vacant".length.positive;
            }
            return @"%combined_length";
        }
        /// counts both occupied positions and unset ones referenced by `unset-slot` and `unset-span`s
        pub fn notVacantCount(@"%vec": @This()) usize {
            return @"%vec".elements.items.len - @"%vec".vacantSlotCount();
        }
        pub fn add(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Slot(@"%Origin") {
            const @"%new_slot" = Slot(@"%Origin"){
                .origin = @"%vec".origin,
                .index = std.math.lossyCast(u32, @"%vec".elements.items.len),
            };
            try @"%vec".elements.append(@"%allocator", @"%new_element");
            return @"%new_slot";
        }
        pub fn addUnset(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
        ) error{OutOfMemory}!Unset_slot(@"%Origin") {
            const @"%new_slot" = Unset_slot(@"%Origin"){
                .origin = @"%vec".origin,
                .index = std.math.lossyCast(u32, @"%vec".elements.items.len),
            };
            try @"%vec".elements.append(@"%allocator", undefined);
            return @"%new_slot";
        }
        pub fn addUnsetLength(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%length": U32,
        ) error{OutOfMemory}!Opt(Unset_span(@"%Origin")) {
            if (P32.fromU32(@"%length")) |@"%length_positive"| {
                const @"%span" = @"%vec".addUnsetLengthPositive(@"%allocator", @"%length_positive");
                return .{ .present = @"%span" };
            } else {
                return .{ .absent = {} };
            }
        }
        pub fn addUnsetLengthPositive(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%length": P32,
        ) error{OutOfMemory}!Unset_span(@"%Origin") {
            const @"%start" = @"%vec".elements.items.len;
            try @"%vec".elements.resize(@"%allocator", try u32AddOrOutOfMem(@"%vec".elements.items.len, @"%length".positive));
            return Unset_span(@"%Origin"){
                .start = .{ .origin = @"%vec".origin, .index = try (std.math.cast(u32, @"%start") orelse error.OutOfMemory) },
                .length = @"%length",
            };
        }
        pub fn insert(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Slot(@"%Origin") {
            const @"%unset_slot" = try @"%vec".insertUnset(@"%allocator");
            return @"%vec".set(@"%unset_slot", @"%new_element");
        }
        pub fn insertUnset(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
        ) error{OutOfMemory}!Unset_slot(@"%Origin") {
            if (@"%vec".vacant.last()) |@"%vacant_span_ref"| {
                const @"%vacant_span_start_end" = try @"%vacant_span_ref".splitStart();
                switch (@"%vacant_span_start_end".end) {
                    .absent => {
                        _ = @"%vec".vacant.pop();
                    },
                    .present => |@"%new_shrunk_vacant_span"| {
                        @"%vacant_span_ref".* = @"%new_shrunk_vacant_span";
                    },
                }
                return @"%vacant_span_start_end".start;
            } else {
                return @"%vec".addUnset(@"%allocator");
            }
        }
        /// slot is invalid while resulting ptr is live
        pub fn element(@"%vec": @This(), @"%slot": Slot(@"%Origin")) *@"%Element" {
            return &@"%vec".elements.items[@"%slot".index];
        }
        pub fn remove(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%slot": Slot(@"%Origin"),
        ) error{OutOfMemory}!@"%Element" {
            const @"%accessed" = @"%vec".unset(@"%slot");
            try @"%vec".slotRid(@"%allocator", @"%accessed".slot);
            return @"%accessed".element;
        }
        pub fn unset(
            @"%vec": @This(),
            @"%slot": Slot(@"%Origin"),
        ) struct { element: @"%Element", slot: Unset_slot(@"%Origin") } {
            const @"%accessed_element" = @"%vec".element(@"%slot").*;
            return .{
                .element = @"%accessed_element",
                .slot = .{ .origin = @"%slot".origin, .index = @"%slot".index },
            };
        }
        pub fn set(
            @"%vec": @This(),
            @"%slot": Unset_slot(@"%Origin"),
            @"%new": @"%Element",
        ) Slot(@"%Origin") {
            @"%vec".elements.items[@"%slot".index] = @"%new";
            return .{ .origin = @"%slot".origin, .index = @"%slot".index };
        }
        // The given span is invalid while the returned slice is live.
        pub fn spanSlice(@"%vec": @This(), @"%span": Span(@"%Origin")) []@"%Element" {
            return @"%vec".elements.items[@"%span".start.index..][0..@"%span".length.positive];
        }
        // The given span is invalid while the returned slice is live.
        pub fn optSpanSlice(@"%vec": @This(), @"%opt_span": Opt(Span(@"%Origin"))) []@"%Element" {
            return switch (@"%opt_span") {
                .absent => &.{},
                .present => |@"%span"| @"%vec".spanSlice(@"%span"),
            };
        }
        /// The returned slice is only valid while vec.elements.items is live.
        /// The returned unset span is only valid once all elements in the slice have been used
        fn spanElements(
            @"%vec": @This(),
            @"%span": Span(@"%Origin"),
        ) struct { slice: []@"%Element", span: Unset_span(@"%Origin") } {
            const @"%slice" = @"%vec".spanSlice(@"%span");
            return .{
                .slice = @"%slice",
                .span = .{
                    .start = .{
                        .origin = @"%span".start.origin,
                        .index = @"%span".start.index,
                    },
                    .length = @"%span".length,
                },
            };
        }
        /// The returned slice is only valid while vec.elements.items is live
        pub fn optSpanElements(
            @"%vec": @This(),
            @"%opt_span": Opt(Span(@"%Origin")),
        ) struct { slice: []@"%Element", span: Opt(Unset_span(@"%Origin")) } {
            switch (@"%opt_span") {
                .absent => return .{ .slice = []@"%Element", .span = .{ .absent = {} } },
                .present => |@"%span"| {
                    return @"%vec".spanElements(@"%span");
                },
            }
        }
        pub fn slotRid(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%slot": Unset_slot(@"%Origin"),
        ) error{OutOfMemory}!void {
            // can maybe be optimized
            return @"%vec".spanRid(@"%allocator", @"%slot".to_span());
        }
        pub fn optSpanRid(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span_to_vacate": Opt(Unset_span(@"%Origin")),
        ) error{OutOfMemory}!void {
            switch (@"%opt_span_to_vacate") {
                .absent => {},
                .present => |@"%span_to_vacate"| {
                    return @"%vec".spanRid(@"%allocator", @"%span_to_vacate");
                },
            }
        }
        pub fn spanRid(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span_to_vacate": Unset_span(@"%Origin"),
        ) error{OutOfMemory}!void {
            var @"%maybe_vacant_span_index_connecting_earlier": ?usize = null;
            var @"%maybe_vacant_span_index_connecting_later": ?usize = null;
            looking_for_connections: for (@"%vec".vacant.items, 0..) |@"%vacant_span", @"%vacant_span_index"| {
                if (@"%maybe_vacant_span_index_connecting_earlier" == null and @"%span_to_vacate".start.index == (@as(usize, @"%vacant_span".start.index) + @as(usize, @"%vacant_span".length.positive))) {
                    @"%maybe_vacant_span_index_connecting_earlier" = @"%vacant_span_index";
                    if (@"%maybe_vacant_span_index_connecting_later") |_| {
                        break :looking_for_connections;
                    }
                } else if (@"%maybe_vacant_span_index_connecting_later" == null and (@as(usize, @"%span_to_vacate".start.index) + @as(usize, @"%span_to_vacate".length.positive)) == @"%vacant_span".start.index) {
                    @"%maybe_vacant_span_index_connecting_later" = @"%vacant_span_index";
                    if (@"%maybe_vacant_span_index_connecting_earlier") |_| {
                        break :looking_for_connections;
                    }
                }
            }
            if (@"%maybe_vacant_span_index_connecting_earlier") |@"%vacant_span_index_connecting_earlier"| {
                var @"%vacant_span_connecting_earlier" = &@"%vec".vacant.items[@"%vacant_span_index_connecting_earlier"];
                if (@"%maybe_vacant_span_index_connecting_later") |@"%vacant_span_index_connecting_later"| {
                    const @"%vacant_span_connecting_later" = @"%vec".vacant.items[@"%vacant_span_index_connecting_later"];
                    @"%vacant_span_connecting_earlier".length = try @"%vacant_span_connecting_earlier".length.addOrOutOfMem(
                        (try @"%span_to_vacate".length.addOrOutOfMem(@"%vacant_span_connecting_later".length.positive)).positive,
                    );
                    _ = @"%vec".vacant.swapRemove(@"%vacant_span_index_connecting_later");
                } else {
                    // maybeVacantSpanIndexConnectingLater == null
                    if (@as(usize, @"%span_to_vacate".start.index) + @as(usize, @"%span_to_vacate".length.positive) == @"%vec".elements.items.len) {
                        @"%vec".elements.shrinkRetainingCapacity(
                            @"%vec".elements.items.len - @as(usize, @"%vacant_span_connecting_earlier".length.positive) - @as(usize, @"%span_to_vacate".length.positive),
                        );
                        _ = @"%vec".vacant.swapRemove(@"%vacant_span_index_connecting_earlier");
                    } else {
                        @"%vacant_span_connecting_earlier".length = try @"%vacant_span_connecting_earlier".length.addOrOutOfMem(@"%span_to_vacate".length.positive);
                    }
                }
            } else if (@"%maybe_vacant_span_index_connecting_later") |@"%vacant_span_index_connecting_later"| {
                // maybeVacantSpanIndexConnectingEarlier == null
                var @"%vacant_span_connecting_later" = &@"%vec".vacant.items[@"%vacant_span_index_connecting_later"];
                @"%vacant_span_connecting_later".* = Unset_span(@"%Origin"){
                    .start = @"%span_to_vacate".start,
                    .length = try @"%vacant_span_connecting_later".length.addOrOutOfMem(
                        @"%span_to_vacate".length.positive,
                    ),
                };
            } else {
                // maybeVacantSpanIndexConnectingEarlier == null and maybeVacantSpanIndexConnectingLater == null
                if (@as(usize, @"%span_to_vacate".start.index) + @as(usize, @"%span_to_vacate".length.positive) == @"%vec".elements.items.len) {
                    @"%vec".elements.shrinkRetainingCapacity(
                        std.math.sub(usize, @"%vec".elements.items.len, @"%span_to_vacate".length.positive) catch 0,
                    );
                } else {
                    try @"%vec".vacant.append(@"%allocator", @"%span_to_vacate");
                }
            }
        }
        pub fn spanMoveToEnd(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            if (@as(usize, @"%span".start.index) + @as(usize, @"%span".length.positive) == @"%vec".elements.items.len) {
                return @"%span";
            }
            // span is not at the end already
            const @"%move_destination_start" = try (std.math.cast(u32, @"%vec".elements.items.len) orelse error.OutOfMemory);
            try @"%vec".elements.ensureUnusedCapacity(@"%allocator", @"%span".length.positive);
            @"%vec".elements.appendSliceAssumeCapacity(@"%vec".spanSlice(@"%span"));
            try @"%vec".spanRid(@"%allocator", Unset_span(@"%Origin"){
                .start = .{ .origin = @"%vec".origin, .index = @"%span".start.index },
                .length = @"%span".length,
            });
            return Span(@"%Origin"){
                .start = .{ .origin = @"%vec".origin, .index = @"%move_destination_start" },
                .length = @"%span".length,
            };
        }
        pub fn spanMoveToVacant(@"%vec": *@This(), @"%span": Span(@"%Origin")) Span(@"%Origin") {
            if (@as(usize, @"%span".start.index) + @as(usize, @"%span".length.positive) < @"%vec".elements.items.len) {
                return @"%span";
            }
            // span is at the end of elements
            if (@"%vec".markLengthPositiveAsOccupied(@"%span".length)) |@"%earlier_start_to_occupy_from"| {
                @"%vec".elements.replaceRangeAssumeCapacity(
                    @"%earlier_start_to_occupy_from",
                    @"%span".length.positive,
                    @"%vec".spanSlice(@"%span"),
                );
                @"%vec".elements.shrinkRetainingCapacity(@"%vec".elements.items.len - @"%span".length.positive);
                return Span(@"%Origin"){
                    .start = .{ .origin = @"%vec".origin, .index = @"%earlier_start_to_occupy_from" },
                    .length = @"%span".length,
                };
            } else {
                return @"%span";
            }
        }
        pub fn spanAddOwnSpan(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%start": Span(@"%Origin"),
            @"%end": Span(@"%Origin"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%combined_length" = @"%start".length.addOrOutOfMem(@"%end".length.positive);
            if (u32AddOrOutOfMem(@"%start".start.index, @"%start".length.positive) == @"%end".start.index) {
                return Span(@"%Origin"){ .start = @"%start".start, .length = @"%combined_length" };
            } else {
                const @"%moved_start" = try @"%vec".spanMoveToEnd(@"%allocator", @"%start");
                _ = try @"%vec".spanMoveToEnd(@"%allocator", @"%end");
                return Span(@"%Origin"){ .start = @"%moved_start".start, .length = @"%combined_length" };
            }
        }
        pub fn unsetSpanAddOwnSpan(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%start": Unset_span(@"%Origin"),
            @"%end": Unset_span(@"%Origin"),
        ) error{OutOfMemory}!Unset_span(@"%Origin") {
            const @"%combined_length" = @"%start".length.addOrOutOfMem(@"%end".length.positive);
            if (u32AddOrOutOfMem(@"%start".start.index, @"%start".length.positive) == @"%end".start.index) {
                return Unset_span(@"%Origin"){ .start = @"%start".start, .length = @"%combined_length" };
            } else {
                return @"%vec".addUnsetLengthPositive(@"%allocator", @"%combined_length");
            }
        }
        pub fn unsetSpanAdd(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Unset_span(@"%Origin"),
            @"%length_increase": Unset_span(@"%Origin"),
        ) error{OutOfMemory}!Unset_span(@"%Origin") {
            const @"%combined_length" = @"%span".length.addOrOutOfMem(@"%length_increase");
            if (@as(usize, @"%span".start.index) + @as(usize, @"%span".length.positive) < @"%vec".elements.items.len) {
                try @"%vec".spanRid(@"%span");
                return @"%vec".addUnsetLengthPositive(@"%allocator", @"%combined_length");
            }
            // span is at the end of elements
            try @"%vec".elements.resize(@"%allocator", try u32AddOrOutOfMem(@"%vec".elements.items.len, @"%span".length.positive));
            return Unset_span(@"%Origin"){ .start = @"%span".start, .length = @"%combined_length" };
        }
        fn markLengthPositiveAsOccupied(@"%vec": *@This(), @"%length_to_occupy": P32) ?u32 {
            for (@"%vec".vacant.items, 0..) |*@"%vacant", @"%vacant_index"| {
                if (@"%vacant".length.positive > @"%length_to_occupy".positive) {
                    @"%vacant".length.positive -|= @"%length_to_occupy".positive;
                    return @"%vacant".start.index;
                } else if (@"%vacant".length.positive == @"%length_to_occupy".positive) {
                    return @"%vec".vacant.swapRemove(@"%vacant_index").start.index;
                }
            }
            return null;
        }
        // add insertSlice?
        pub fn addSlice(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_elements": []const @"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            if (P32.fromU32(try (std.math.cast(u32, @"%new_elements".len) orelse error.OutOfMemory))) |@"%new_length"| {
                const @"%length_before_add" = @"%vec".elements.items.len;
                try @"%vec".elements.appendSlice(@"%allocator", @"%new_elements");
                return .{ .present = .{
                    .start = .{
                        .origin = @"%vec".origin,
                        .index = try (std.math.cast(u32, @"%length_before_add") orelse error.OutOfMemory),
                    },
                    .length = @"%new_length",
                } };
            } else return .{ .absent = {} };
        }
        // add insertIterator?
        pub fn addIterator(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_elements": anytype,
            @"%next_element": fn (*@TypeOf(@"%new_elements")) ?@"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            const @"%length_before_add" = @"%vec".elements.items.len;
            var @"%new_elements_iterator" = @"%new_elements";
            while (@"%next_element"(&@"%new_elements_iterator")) |@"%new_element"| {
                try @"%vec".elements.append(@"%allocator", @"%new_element");
            }
            return if (P32.fromU32(try (std.math.cast(u32, @"%vec".elements.items.len - @"%length_before_add") orelse error.OutOfMemory))) |@"%new_length"|
                .{ .present = .{
                    .start = .{
                        .origin = @"%vec".origin,
                        .index = try (std.math.cast(u32, @"%length_before_add") orelse error.OutOfMemory),
                    },
                    .length = @"%new_length",
                } }
            else
                .{ .absent = {} };
        }
        pub fn optSpanAdd(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            return switch (@"%opt_span") {
                .absent => (try @"%vec".add(@"%allocator", @"%new_element")).to_span(),
                .present => |@"%span"| @"%vec".spanAdd(@"%allocator", @"%span", @"%new_element"),
            };
        }
        pub fn spanAdd(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%vec".spanMoveToEnd(@"%allocator", @"%span");
            try @"%vec".elements.append(@"%allocator", @"%new_element");
            return Span(@"%Origin"){ .start = @"%moved_span".start, .length = try @"%moved_span".length.addOrOutOfMem(1) };
        }
        pub fn optSpanAddSlice(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_elements": []const @"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            return switch (@"%opt_span") {
                .absent => @"%vec".addSlice(@"%allocator", @"%new_elements"),
                .present => |@"%span"| .{ .present = try @"%vec".spanAddSlice(@"%allocator", @"%span", @"%new_elements") },
            };
        }
        pub fn spanAddSlice(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_elements": []const @"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%vec".spanMoveToEnd(@"%allocator", @"%span");
            try @"%vec".elements.appendSlice(@"%allocator", @"%new_elements");
            return Span(@"%Origin"){
                .start = @"%moved_span".start,
                .length = try @"%moved_span".length.addOrOutOfMem(@"%new_elements".len),
            };
        }
        pub fn optSpanAddIterator(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_elements": anytype,
            @"%next_element": fn (*@TypeOf(@"%new_elements")) ?@"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            return switch (@"%opt_span") {
                .absent => @"%vec".addIterator(@"%allocator", @"%new_elements", @"%next_element"),
                .present => |@"%span"| .{ .present = try @"%vec".spanAddIterator(@"%allocator", @"%span", @"%new_elements", @"%next_element") },
            };
        }
        pub fn spanAddIterator(
            @"%vec": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_elements": anytype,
            @"%next_element": fn (*@TypeOf(@"%new_elements")) ?@"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%vec".spanMoveToEnd(@"%allocator", @"%span");
            const @"%length_before_add" = @"%vec".elements.items.len;
            var @"%new_elements_iterator" = @"%new_elements";
            while (@"%next_element"(&@"%new_elements_iterator")) |@"%new_element"| {
                try @"%vec".elements.append(@"%allocator", @"%new_element");
            }
            const @"%new_length" = try (std.math.cast(u32, @"%vec".elements.items.len - @"%length_before_add") orelse error.OutOfMemory);
            return Span(@"%Origin"){
                .start = @"%moved_span".start,
                .length = try @"%moved_span".length.addOrOutOfMem(@"%new_length"),
            };
        }
        pub fn spanReverse(@"%vec": @This(), @"%span": Span(@"%Origin")) Span(@"%Origin") {
            std.mem.reverse(@"%Element", @"%vec".spanSlice(@"%span"));
            return @"%span";
        }
        pub fn optSpanReverse(@"%vec": @This(), @"%opt_span": Opt(Span(@"%Origin"))) Opt(Span(@"%Origin")) {
            std.mem.reverse(@"%Element", @"%vec".optSpanSlice(@"%opt_span"));
            return @"%opt_span";
        }
        /// vec is invalid after
        pub fn intoUnsetSlice(
            @"%vec": @This(),
            @"%allocator": std.mem.Allocator,
        ) Unset_slice(@"%Element") {
            var @"%vacant" = @"%vec".vacant;
            @"%vacant".deinit(@"%allocator");
            var @"%elements" = @"%vec".elements;
            @"%elements".clearRetainingCapacity();
            return .{ .undefined_items = @"%elements".unusedCapacitySlice() };
        }
        /// vec is invalid after
        pub fn rid(@"%vec": @This(), @"%allocator": std.mem.Allocator) void {
            var @"%vec_mut" = @"%vec";
            @"%vec_mut".elements.deinit(@"%allocator");
            @"%vec_mut".vacant.deinit(@"%allocator");
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
pub fn p32_dup(@"%n": P32) error{OutOfMemory}!@".a.b"(P32, P32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn p32_add_clamp(@"%": @".p.u"(P32, U32)) error{OutOfMemory}!P32 {
    return @"%".p.addClamp(@"%".u);
}
pub fn p32_mul_clamp(@"%": @".a.b"(P32, P32)) error{OutOfMemory}!P32 {
    return @"%".a.mulClamp(@"%".b);
}

pub fn u32_rid(_: U32) error{OutOfMemory}!void {}
pub fn u32_dup(@"%n": U32) error{OutOfMemory}!@".a.b"(U32, U32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn u32_add_clamp(@"%": @".a.b"(U32, U32)) error{OutOfMemory}!U32 {
    return @"%".a +| @"%".b;
}
pub fn u32_mul_clamp(@"%": @".a.b"(U32, U32)) error{OutOfMemory}!U32 {
    return @"%".a *| @"%".b;
}
pub fn u32_to_p32(@"%n": U32) error{OutOfMemory}!Opt(P32) {
    return if (P32.fromU32(@"%n")) |@"%p32"| .{ .present = @"%p32" } else .{ .absent = {} };
}

pub fn i32_rid(_: I32) error{OutOfMemory}!void {}
pub fn i32_dup(@"%n": I32) error{OutOfMemory}!@".a.b"(I32, I32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn i32_add_clamp(@"%": @".a.b"(I32, I32)) error{OutOfMemory}!I32 {
    return @"%".a +| @"%".b;
}
pub fn i32_mul_clamp(@"%": @".a.b"(I32, I32)) error{OutOfMemory}!I32 {
    return @"%".a *| @"%".b;
}
pub fn i32_negate_clamp(@"%n": I32) error{OutOfMemory}!I32 {
    return 0 -| @"%n";
}
pub fn i32_abs_to_u32(@"%n": I32) error{OutOfMemory}!U32 {
    return @abs(@"%n");
}

pub fn f32_rid(_: F32) error{OutOfMemory}!void {}
pub fn f32_dup(@"%n": F32) error{OutOfMemory}!@".a.b"(F32, F32) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn f32_negate(@"%n": F32) F32 {
    return -@"%n";
}
pub fn f32_abs(@"%n": F32) error{OutOfMemory}!F32 {
    return @abs(@"%n");
}
pub fn f32_round(@"%": @".mode.n"(Round_mode, F32)) error{OutOfMemory}!F32 {
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
pub fn f32_to_i32_clamp(@"%": @".mode.n"(Round_mode, F32)) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, f32_round(@"%"));
}
pub fn f32_add_clamp(@"%": @".a.b"(F32, F32)) error{OutOfMemory}!F32 {
    const @"%sum" = @"%".a + @"%".b;
    return if (std.math.isNegativeInf(@"%sum")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%sum")) std.math.floatMax(f32) else @"%sum";
}
pub fn f32_mul_clamp(@"%": @".a.b"(F32, F32)) error{OutOfMemory}!F32 {
    const @"%product" = @"%".a * @"%".b;
    return if (std.math.isNegativeInf(@"%product")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%product")) std.math.floatMax(f32) else @"%product";
}
pub fn f32_div_clamp(@"%": @".a.b"(F32, F32)) error{OutOfMemory}!F32 {
    return if (@"%".b == 0) 0 else {
        const @"%div_result" = @"%".a / @"%".b;
        return if (std.math.isNegativeInf(@"%div_result")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%div_result")) std.math.floatMax(f32) else @"%div_result";
    };
}

pub fn char_rid(_: Char) error{OutOfMemory}!void {}
pub fn char_dup(@"%n": Char) error{OutOfMemory}!@".a.b"(Char, Char) {
    return .{ .a = @"%n", .b = @"%n" };
}

pub fn str_rid(_: Str) error{OutOfMemory}!void {}
pub fn str_dup(@"%n": Str) error{OutOfMemory}!@".a.b"(Str, Str) {
    return .{ .a = @"%n", .b = @"%n" };
}

pub fn fn_rid(@"%In": type, @"%Out": type, _: Fn(@"%In", @"%Out")) error{OutOfMemory}!void {}
pub fn fn_dup(@"%In": type, @"%Out": type, @"%function": Fn(@"%In", @"%Out")) error{OutOfMemory}!@".a.b"(Fn(@"%In", @"%Out"), Fn(@"%In", @"%Out")) {
    return .{ .a = @"%function", .b = @"%function" };
}

pub fn origin_rid(@"%Origin": type, _: Origin(@"%Origin")) error{OutOfMemory}!void {}

pub fn slot_index(@"%Origin": type, @"%slot": Slot(@"%Origin")) error{OutOfMemory}!@".index.slot"(U32, Slot(@"%Origin")) {
    return .{ .slot = @"%slot", .index = @"%slot".index };
}
pub fn slot_to_span(@"%Origin": type, @"%slot": Slot(@"%Origin")) error{OutOfMemory}!Span(@"%Origin") {
    return @"%slot".to_span();
}

pub fn unset_slot_index(@"%Origin": type, @"%slot": Unset_slot(@"%Origin")) error{OutOfMemory}!@".index.slot"(U32, Unset_slot(@"%Origin")) {
    return .{ .slot = @"%slot", .index = @"%slot".index };
}
pub fn unset_slot_to_span(@"%Origin": type, @"%slot": Unset_slot(@"%Origin")) error{OutOfMemory}!Span(@"%Origin") {
    return @"%slot".to_span();
}

pub fn span_length(@"%Origin": type, @"%span": Span(@"%Origin")) error{OutOfMemory}!@".length.span"(P32, Span(@"%Origin")) {
    return .{ .span = @"%span", .length = @"%span".length };
}
pub fn opt_span_length(
    @"%Origin": type,
    @"%opt_span": Opt(Span(@"%Origin")),
) error{OutOfMemory}!@".length.span"(U32, Opt(Span(@"%Origin"))) {
    return .{
        .span = @"%opt_span",
        .length = switch (@"%opt_span") {
            .absent => 0,
            .present => |@"%span"| @"%span".length.positive,
        },
    };
}
pub fn span_start(@"%Origin": type, @"%span": Span(@"%Origin")) error{OutOfMemory}!@".end.start"(
    Opt(Span(@"%Origin")),
    Slot(@"%Origin"),
) {
    return @"%span".splitStart();
}
pub fn span_end(@"%Origin": type, @"%span": Span(@"%Origin")) error{OutOfMemory}!@".end.start"(
    Slot(@"%Origin"),
    Opt(Span(@"%Origin")),
) {
    return @"%span".splitEnd();
}
pub fn span_start_of_length_positive(
    @"%Origin": type,
    @"%": @".length.span"(P32, Span(@"%Origin")),
) error{OutOfMemory}!@".after.start"(
    Opt(Span(@"%Origin")),
    Span(@"%Origin"),
) {
    return @"%".span.splitAfterLengthPositive(@"%".length);
}
pub fn span_end_of_length_positive(
    @"%Origin": type,
    @"%": @".length.span"(P32, Span(@"%Origin")),
) error{OutOfMemory}!@".before.end"(
    Opt(Span(@"%Origin")),
    Span(@"%Origin"),
) {
    return @"%".span.splitBeforeEndLengthPositive(@"%".length);
}
pub fn opt_span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%": @".span.state.step"(
        Opt(Span(@"%Origin")),
        @"%State",
        Fn(@".slot.state"(Slot(@"%Origin"), @"%State"), @"%State"),
    ),
) error{OutOfMemory}!@"%State" {
    return switch (@"%".span) {
        .absent => @"%".state,
        .present => |@"%span"| @"%span".fold(@"%".state, @"%".step),
    };
}
pub fn span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%": @".span.state.step"(
        Span(@"%Origin"),
        @"%State",
        Fn(@".slot.state"(Slot(@"%Origin"), @"%State"), @"%State"),
    ),
) error{OutOfMemory}!@"%State" {
    return @"%".span.fold(@"%".state, @"%".step);
}

pub fn unset_span_length(
    @"%Origin": type,
    @"%span": Unset_span(@"%Origin"),
) error{OutOfMemory}!@".length.span"(P32, Unset_span(@"%Origin")) {
    return .{ .span = @"%span", .length = @"%span".length };
}
pub fn opt_unset_span_length(
    @"%Origin": type,
    @"%opt_span": Opt(Unset_span(@"%Origin")),
) error{OutOfMemory}!@".length.span"(U32, Opt(Unset_span(@"%Origin"))) {
    return .{
        .span = @"%opt_span",
        .length = switch (@"%opt_span") {
            .absent => 0,
            .present => |@"%span"| @"%span".length.positive,
        },
    };
}
pub fn unset_span_start(@"%Origin": type, @"%span": Unset_span(@"%Origin")) error{OutOfMemory}!@".end.start"(
    Opt(Unset_span(@"%Origin")),
    Unset_slot(@"%Origin"),
) {
    return @"%span".splitStart();
}
pub fn unset_span_end(@"%Origin": type, @"%span": Unset_span(@"%Origin")) error{OutOfMemory}!@".end.start"(
    Unset_slot(@"%Origin"),
    Opt(Unset_span(@"%Origin")),
) {
    return @"%span".splitEnd();
}
pub fn unset_span_start_of_length_positive(
    @"%Origin": type,
    @"%": @".length.span"(P32, Unset_span(@"%Origin")),
) error{OutOfMemory}!@".after.start"(
    Opt(Unset_span(@"%Origin")),
    Unset_span(@"%Origin"),
) {
    return @"%".span.splitAfterLengthPositive(@"%".length);
}
pub fn unset_span_end_of_length_positive(
    @"%Origin": type,
    @"%": @".length.span"(P32, Unset_span(@"%Origin")),
) error{OutOfMemory}!@".before.end"(
    Opt(Unset_span(@"%Origin")),
    Unset_span(@"%Origin"),
) {
    return @"%".span.splitBeforeEndLengthPositive(@"%".length);
}
pub fn opt_unset_span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%": @".span.state.step"(
        Opt(Unset_span(@"%Origin")),
        @"%State",
        Fn(@".slot.state"(Unset_slot(@"%Origin"), @"%State"), @"%State"),
    ),
) error{OutOfMemory}!@"%State" {
    return switch (@"%".span) {
        .absent => @"%".state,
        .present => |@"%span"| @"%span".fold(@"%".state, @"%".step),
    };
}
pub fn unset_span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%": @".span.state.step"(
        Unset_span(@"%Origin"),
        @"%State",
        Fn(@".slot.state"(Unset_slot(@"%Origin"), @"%State"), @"%State"),
    ),
) error{OutOfMemory}!@"%State" {
    return @"%".span.fold(@"%".state, @"%".step);
}

pub fn vec_empty(
    @"%Element": type,
    @"%Origin": type,
    @"%origin": Origin(@"%Origin"),
) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    return Vec(@"%Origin", @"%Element").empty(@"%origin");
}
pub fn vec_reuse(
    @"%Element": type,
    @"%Origin": type,
    @"%": @".origin.slice"(Origin(@"%Origin"), Unset_slice(@"%Element")),
) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    return Vec(@"%Origin", @"%Element").reuse(@"%".origin, @"%".slice);
}
pub fn vec_pre_allocate_at_least(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".length.vec"(u32, Vec(@"%Element", @"%Origin")),
) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    var @"%vec" = @"%".vec;
    try @"%vec".preAllocateAtLeast(@"%allocator", @"%".length);
    return @"%vec";
}
pub fn vec_pre_allocation_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Vec(@"%Element", @"%Origin"),
) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    var @"%vec" = @"%".vec;
    try @"%vec".preAllocationRid(@"%allocator");
    return @"%vec";
}
pub fn vec_insert(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.vec"(@"%Element", Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".slot.vec"(Slot(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%slot" = try @"%".vec.insert(@"%allocator", @"%".new);
    return .{ .vec = @"%".vec, .slot = @"%slot" };
}
pub fn vec_add(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.vec"(@"%Element", Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".slot.vec"(Slot(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%slot" = try @"%".vec.add(@"%allocator", @"%".new);
    return .{ .vec = @"%".vec, .slot = @"%slot" };
}
pub fn vec_insert_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%vec": Vec(@"%Origin", @"%Element"),
) error{OutOfMemory}!@".slot.vec"(Unset_slot(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%slot" = try @"%vec".insertUnset(@"%allocator");
    return .{ .vec = @"%vec", .slot = @"%slot" };
}
pub fn vec_add_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%vec": Vec(@"%Origin", @"%Element"),
) error{OutOfMemory}!@".slot.vec"(Unset_slot(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%slot" = try @"%vec".addUnset(@"%allocator");
    return .{ .vec = @"%vec", .slot = @"%slot" };
}
pub fn vec_add_unset_length(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".length.vec"(U32, Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Opt(Unset_span(@"%Origin")), Vec(@"%Origin", @"%Element")) {
    const @"%unset_span" = try @"%".vec.addUnsetLength(@"%allocator", @"%".length);
    return .{ .vec = @"%".vec, .span = @"%unset_span" };
}
pub fn vec_add_unset_length_positive(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".length.vec"(P32, Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Unset_span(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%unset_span" = try @"%".vec.addUnsetLengthPositive(@"%allocator", @"%".length);
    return .{ .vec = @"%".vec, .span = @"%unset_span" };
}
pub fn vec_remove(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".slot.vec"(Slot(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".element.vec"(@"%Element", Vec(@"%Origin", @"%Element")) {
    const @"%element" = @"%".vec.remove(@"%allocator", @"%".slot);
    return .{ .vec = @"%".vec, .element = @"%element" };
}
pub fn vec_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".slot.vec"(Slot(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".element.slot.vec"(@"%Element", Unset_slot(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%element" = @"%".vec.unset(@"%allocator", @"%".slot);
    return .{ .vec = @"%".vec, .element = @"%element".element, .slot = @"%element".slot };
}
pub fn vec_set(@"%Element": type, @"%Origin": type, @"%": @".new.slot.vec"(@"%Element", Slot(@"%Origin"), Vec(@"%Origin", @"%Element"))) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    @"%".vec.set(@"%".slot, @"%".new);
    return @"%".vec;
}
pub fn vec_opt_span_add(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(@"%Element", Opt(Span(@"%Origin")), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    var @"%vec" = @"%".vec;
    const @"%combined_span" = @"%vec".optSpanAdd(@"%allocator", @"%".span, @"%".new);
    return .{ .span = @"%combined_span", .vec = @"%vec" };
}
pub fn vec_span_add(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(@"%Element", Span(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    var @"%vec" = @"%".vec;
    const @"%combined_span" = @"%vec".spanAdd(@"%allocator", @"%".span, @"%".new);
    return .{ .span = @"%combined_span", .vec = @"%vec" };
}
pub fn vec_char_opt_span_add_str(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(Str, Opt(Span(@"%Origin")), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Opt(Span(@"%Origin")), Vec(@"%Origin", Char)) {
    var @"%vec" = @"%".vec;
    const @"%combined_span" = try @"%vec".optSpanAddIterator(
        @"%allocator",
        @"%".span,
        std.unicode.Utf8View.initUnchecked(@"%".new).iterator(),
        std.unicode.Utf8Iterator.nextCodepoint,
    );
    return .{ .span = @"%combined_span", .vec = @"%vec" };
}
pub fn vec_char_span_add_str(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(Str, Span(@"%Origin"), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", Char)) {
    var @"%vec" = @"%".vec;
    const @"%combined_span" = try @"%vec".spanAddIterator(
        @"%allocator",
        @"%".span,
        std.unicode.Utf8View.initUnchecked(@"%".new).iterator(),
        std.unicode.Utf8Iterator.nextCodepoint,
    );
    return .{ .span = @"%combined_span", .vec = @"%vec" };
}
// is there a more correct way?
const u32_max_print_len = std.fmt.count("{}", .{std.math.maxInt(U32)});
pub fn vec_char_span_add_u32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(U32, Span(@"%Origin"), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", Char)) {
    var @"%buffer": [u32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    return vec_char_span_add_str(@"%Origin", @"%allocator", .{
        .vec = @"%".vec,
        .span = @"%".span,
        .new = @"%buffer"[0..@"%buffer_exclusive_end"],
    });
}
pub fn vec_char_opt_span_add_u32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(U32, Opt(Span(@"%Origin")), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", Char)) {
    var @"%buffer": [u32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    const @"%combined" = try vec_char_opt_span_add_str(@"%Origin", @"%allocator", .{
        .vec = @"%".vec,
        .span = @"%".span,
        .new = @"%buffer"[0..@"%buffer_exclusive_end"],
    });
    return .{ .vec = @"%combined".vec, .span = @"%combined".span.present };
}
// is there a more correct way?
const i32_max_print_len = @max(
    std.fmt.count("{}", .{std.math.minInt(I32)}),
    std.fmt.count("{}", .{std.math.maxInt(I32)}),
);
pub fn vec_char_span_add_i32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(I32, Span(@"%Origin"), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", Char)) {
    var @"%buffer": [i32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    return vec_char_span_add_str(@"%Origin", @"%allocator", .{
        .vec = @"%".vec,
        .span = @"%".span,
        .new = @"%buffer"[0..@"%buffer_exclusive_end"],
    });
}
pub fn vec_char_opt_span_add_i32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(I32, Opt(Span(@"%Origin")), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", Char)) {
    var @"%buffer": [i32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    const @"%combined" = try vec_char_opt_span_add_str(Char, @"%Origin", @"%allocator", .{
        .vec = @"%".vec,
        .span = @"%".span,
        .new = @"%buffer"[0..@"%buffer_exclusive_end"],
    });
    return .{ .vec = @"%combined".vec, .span = @"%combined".span.present };
}
const f32_max_decimal_print_len =
    std.fmt.float.bufferSize(std.fmt.float.Mode.decimal, F32);
pub fn vec_char_span_add_f32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(F32, Span(@"%Origin"), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", Char)) {
    var @"%buffer": [f32_max_decimal_print_len]u8 = undefined;
    const @"%used_buffer_slice" = std.fmt.float.render(
        &@"%buffer",
        @"%".new,
        .{ .mode = .decimal, .precision = null },
    ) catch unreachable;
    return vec_char_span_add_str(@"%Origin", @"%allocator", .{
        .vec = @"%".vec,
        .span = @"%".span,
        .new = @"%used_buffer_slice",
    });
}
pub fn vec_char_opt_span_add_f32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".new.span.vec"(F32, Opt(Span(@"%Origin")), Vec(@"%Origin", Char)),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", Char)) {
    var @"%buffer": [f32_max_decimal_print_len]u8 = undefined;
    const @"%used_buffer_slice" = std.fmt.float.render(
        &@"%buffer",
        @"%".new,
        .{ .mode = .decimal, .precision = null },
    ) catch unreachable;
    const @"%combined" = try vec_char_opt_span_add_str(@"%Origin", @"%allocator", .{
        .vec = @"%".vec,
        .span = @"%".span,
        .new = @"%used_buffer_slice",
    });
    return .{ .vec = @"%combined".vec, .span = @"%combined".span.present };
}
pub fn vec_span_add_vec_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".source.source_span.span.vec"(
        Vec(@"%Origin", @"%Element"),
        Opt(Span(@"%Origin")),
        Span(@"%Origin"),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".source.source_span.span.vec"(
    Vec(@"%Origin", @"%Element"),
    Opt(Unset_span(@"%Origin")),
    Span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    const @"%sourced" = @"%".source.optSpanElements(@"%".source_span);
    const @"%combined_span" = try @"%".vec.spanAddSlice(@"%allocator", @"%".span, @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .vec = @"%".vec,
    };
}
pub fn vec_span_add_vec_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".source.source_span.span.vec"(
        Vec(@"%Origin", @"%Element"),
        Span(@"%Origin"),
        Span(@"%Origin"),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".source.source_span.span.vec"(
    Vec(@"%Origin", @"%Element"),
    Unset_span(@"%Origin"),
    Span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    const @"%sourced" = @"%".source.spanElements(@"%".source_span);
    const @"%combined_span" = try @"%".vec.spanAddSlice(@"%allocator", @"%".span, @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .vec = @"%".vec,
    };
}
pub fn vec_opt_span_add_vec_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".source.source_span.span.vec"(
        Vec(@"%Origin", @"%Element"),
        Opt(Span(@"%Origin")),
        Span(@"%Origin"),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".source.source_span.span.vec"(
    Vec(@"%Origin", @"%Element"),
    Opt(Unset_span(@"%Origin")),
    Opt(Span(@"%Origin")),
    Vec(@"%Origin", @"%Element"),
) {
    const @"%sourced" = @"%".source.spanElements(@"%".source_span);
    const @"%combined_span" = try @"%".vec.optSpanAddSlice(@"%allocator", @"%".span, @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .vec = @"%".vec,
    };
}
pub fn vec_opt_span_add_vec_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".source.source_span.span.vec"(
        Vec(@"%Origin", @"%Element"),
        Span(@"%Origin"),
        Opt(Span(@"%Origin")),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".source.source_span.span.vec"(
    Vec(@"%Origin", @"%Element"),
    Unset_span(@"%Origin"),
    Span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    // is there a better way?
    const @"%sourced" = @"%".source.spanElements(@"%".source_span);
    const @"%span_combined_with_start" = try @"%".vec.optSpanAdd(@"%allocator", @"%".span, @"%sourced".slice[0]);
    const @"%combined_span" = try @"%".vec.optSpanAddSlice(@"%allocator", @"%span_combined_with_start", @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .vec = @"%".vec,
    };
}
pub fn vec_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Span(@"%Origin"),
        Span(@"%Origin"),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    const @"%combined_span" = try @"%".vec.spanAddOwnSpan(@"%allocator", @"%".start, @"%".end);
    return .{
        .span = @"%combined_span",
        .vec = @"%".vec,
    };
}
pub fn vec_opt_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Opt(Span(@"%Origin")),
        Span(@"%Origin"),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    switch (@"%".start) {
        .absent => return .{ .vec = @"%".vec, .span = @"%".end },
        .present => |@"%start"| {
            const @"%combined_span" = try @"%".vec.spanAddOwnSpan(@"%allocator", @"%start", @"%".end);
            return .{
                .span = @"%combined_span",
                .vec = @"%".vec,
            };
        },
    }
}
pub fn vec_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Span(@"%Origin"),
        Opt(Span(@"%Origin")),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    switch (@"%".end) {
        .absent => return .{ .vec = @"%".vec, .span = @"%".start },
        .present => |@"%end"| {
            const @"%combined_span" = try @"%".vec.spanAddOwnSpan(@"%allocator", @"%".start, @"%end");
            return .{
                .span = @"%combined_span",
                .vec = @"%".vec,
            };
        },
    }
}
pub fn vec_opt_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Span(@"%Origin"),
        Opt(Span(@"%Origin")),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    switch (@"%".start) {
        .absent => return .{ .vec = @"%".vec, .span = @"%".end },
        .present => |@"%start"| {
            switch (@"%".end) {
                .absent => return .{ .vec = @"%".vec, .span = .{ .present = @"%start" } },
                .present => |@"%end"| {
                    const @"%combined_span" = try @"%".vec.spanAddOwnSpan(@"%allocator", @"%start", @"%end");
                    return .{
                        .span = .{ .present = @"%combined_span" },
                        .vec = @"%".vec,
                    };
                },
            }
        },
    }
}
pub fn vec_unset_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Unset_span(@"%Origin"),
        Unset_span(@"%Origin"),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Unset_span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    const @"%combined_span" = try @"%".vec.unsetSpanAddOwnSpan(@"%allocator", @"%".start, @"%".end);
    return .{
        .span = @"%combined_span",
        .vec = @"%".vec,
    };
}
pub fn vec_opt_unset_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Opt(Unset_span(@"%Origin")),
        Unset_span(@"%Origin"),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Unset_span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    switch (@"%".start) {
        .absent => return .{ .vec = @"%".vec, .span = @"%".end },
        .present => |@"%start"| {
            const @"%combined_unset_span" = try @"%".vec.unsetSpanAddOwnSpan(@"%allocator", @"%start", @"%".end);
            return .{
                .span = @"%combined_unset_span",
                .vec = @"%".vec,
            };
        },
    }
}
pub fn vec_unset_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Unset_span(@"%Origin"),
        Opt(Unset_span(@"%Origin")),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Unset_span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    switch (@"%".end) {
        .absent => return .{ .vec = @"%".vec, .span = @"%".start },
        .present => |@"%end"| {
            const @"%combined_unset_span" = try @"%".vec.unsetSpanAddOwnSpan(@"%allocator", @"%".start, @"%end");
            return .{
                .span = @"%combined_unset_span",
                .vec = @"%".vec,
            };
        },
    }
}
pub fn vec_opt_unset_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".end.start.vec"(
        Unset_span(@"%Origin"),
        Opt(Unset_span(@"%Origin")),
        Vec(@"%Origin", @"%Element"),
    ),
) error{OutOfMemory}!@".span.vec"(
    Unset_span(@"%Origin"),
    Vec(@"%Origin", @"%Element"),
) {
    switch (@"%".start) {
        .absent => return .{ .vec = @"%".vec, .span = @"%".end },
        .present => |@"%start"| {
            switch (@"%".end) {
                .absent => return .{ .vec = @"%".vec, .span = .{ .present = @"%start" } },
                .present => |@"%end"| {
                    const @"%combined_unset_span" = try @"%".vec.unsetSpanAddOwnSpan(@"%allocator", @"%start", @"%end");
                    return .{
                        .span = .{ .present = @"%combined_unset_span" },
                        .vec = @"%".vec,
                    };
                },
            }
        },
    }
}
pub fn vec_span_move_to_vacant(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%moved_span" = @"%".vec.spanMoveToVacant(@"%allocator", @"%".span);
    return .{ .vec = @"%".vec, .span = @"%moved_span" };
}
pub fn vec_opt_span_move_to_vacant(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".span.vec"(Opt(Span(@"%Origin")), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Opt(Span(@"%Origin")), Vec(@"%Origin", @"%Element")) {
    switch (@"%".span) {
        .absent => return .{ .vec = @"%".vec, .span = .{ .absent = {} } },
        .present => |@"%span"| {
            const @"%moved_span" = @"%".vec.spanMoveToVacant(@"%allocator", @"%span");
            return .{ .vec = @"%".vec, .span = .{ .present = @"%moved_span" } };
        },
    }
}
pub fn vec_span_move_to_end(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%moved_span" = @"%".vec.spanMoveToEnd(@"%allocator", @"%".span);
    return .{ .vec = @"%".vec, .span = @"%moved_span" };
}
pub fn vec_opt_span_move_to_end(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".span.vec"(Opt(Span(@"%Origin")), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Opt(Span(@"%Origin")), Vec(@"%Origin", @"%Element")) {
    switch (@"%".span) {
        .absent => return .{ .vec = @"%".vec, .span = .{ .absent = {} } },
        .present => |@"%span"| {
            const @"%moved_span" = @"%".vec.spanMoveToEnd(@"%allocator", @"%span");
            return .{ .vec = @"%".vec, .span = .{ .preent = @"%moved_span" } };
        },
    }
}
pub fn vec_span_reverse(
    @"%Element": type,
    @"%Origin": type,
    @"%": @".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Span(@"%Origin"), Vec(@"%Origin", @"%Element")) {
    const @"%reversed_span" = @"%".vec.spanReverse(@"%".span);
    return .{ .vec = @"%".vec, .span = @"%reversed_span" };
}
pub fn vec_opt_span_reverse(
    @"%Element": type,
    @"%Origin": type,
    @"%": @".span.vec"(Opt(Span(@"%Origin")), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!@".span.vec"(Opt(Span(@"%Origin")), Vec(@"%Origin", @"%Element")) {
    const @"%reversed_span" = @"%".vec.optSpanReverse(@"%".span);
    return .{ .vec = @"%".vec, .span = @"%reversed_span" };
}
pub fn vec_slot_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".slot.vec"(Unset_slot(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    @"%".vec.slotRid(@"%allocator", @"%".span);
    return @"%".vec;
}
pub fn vec_span_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".span.vec"(Unset_span(@"%Origin"), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    @"%".vec.spanRid(@"%allocator", @"%".span);
    return @"%".vec;
}
pub fn vec_opt_span_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": @".span.vec"(Opt(Unset_span(@"%Origin")), Vec(@"%Origin", @"%Element")),
) error{OutOfMemory}!Vec(@"%Origin", @"%Element") {
    @"%".vec.optSpanRid(@"%allocator", @"%".span);
    return @"%".vec;
}
pub fn vec_to_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%vec": Vec(@"%Origin", @"%Element"),
) error{OutOfMemory}!Unset_slice(@"%Element") {
    @"%vec".intoUnsetSlice(@"%allocator");
}
pub fn vec_rid(@"%Element": type, @"%Origin": type, @"%allocator": std.mem.Allocator, @"%vec": Vec(@"%Origin", @"%Element")) error{OutOfMemory}!void {
    @"%vec".rid(@"%allocator");
}

pub fn unset_slice_allocate_length(
    @"%Element": type,
    @"%allocator": std.mem.Allocator,
    @"%length": U32,
) error{OutOfMemory}!Unset_slice(@"%Element") {
    return Unset_slice(@"%Element").allocateLength(@"%allocator", @"%length");
}
pub fn unset_slice_length(
    @"%Element": type,
    @"%unset_slice": Unset_slice(@"%Element"),
) error{OutOfMemory}!@".length.slice"(U32, Unset_slice(@"%Element")) {
    return .{ .length = @"%unset_slice".length, .slice = @"%unset_slice" };
}
pub fn unset_slice_transmute_or_rid_and_allocate(
    @"%Element": type,
    @"%NewElement": type,
    @"%allocator": std.mem.Allocator,
    @"%unset_slice": Unset_slice(@"%Element"),
) error{OutOfMemory}!Unset_slice(@"%NewElement") {
    return @"%unset_slice".transmuteOrRidAndAllocate(@"%NewElement", @"%allocator");
}
pub fn unset_slice_rid(
    @"%Element": type,
    @"%allocator": std.mem.Allocator,
    @"%unset_slice": Unset_slice(@"%Element"),
) error{OutOfMemory}!Unset_slice(@"%Element") {
    return @"%unset_slice".rid(@"%allocator");
}
