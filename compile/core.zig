const std = @import("std");

fn RecordWithFieldNames(comptime @"%field_names": []const []const u8) type {
    return struct {
        fn WithFieldValueTypesOf(@"%type_tuple": type) type {
            return @Struct(
                std.lang.Type.ContainerLayout.auto,
                null,
                @"%field_names",
                @ptrCast(@typeInfo(@"%type_tuple").@"struct".field_types),
                &@splat(.{}),
            );
        }
        fn withFieldValuesOf(@"%field_values": anytype) @This().WithFieldValueTypesOf(@TypeOf(@"%field_values")) {
            var @"%record": @This().WithFieldValueTypesOf(@TypeOf(@"%field_values")) = undefined;
            inline for (@"%field_names") |@"%field_name"| {
                @field(@"%record", @"%field_name") = @field(@"%field_values", @"%field_name");
            }
            return @"%record";
        }
    };
}
/// be aware: different field order means different type
pub fn Record(@"%struct_type": type) type {
    // uncomment to verify field names are sorted
    // if (!std.sort.isSorted([]const u8, @typeInfo(@"%struct_type").@"struct".field_names, {}, struct {
    //     fn f(_: void, @"%a": []const u8, @"%b": []const u8) bool {
    //         return std.mem.order(u8, @"%a", @"%b") == .lt;
    //     }
    // }.f)) {
    //     @compileError("fields must be sorted");
    // }
    return RecordWithFieldNames(@typeInfo(@"%struct_type").@"struct".field_names).WithFieldValueTypesOf(@"%struct_type");
}
/// be aware: different field order means different type
pub fn record(@"%struct_value": anytype) Record(@TypeOf(@"%struct_value")) {
    return RecordWithFieldNames(@typeInfo(@TypeOf(@"%struct_value")).@"struct".field_names).withFieldValuesOf(@"%struct_value");
}

// If you're wondering about the strange names:
// - @"|variant_a|variant_b": since zig removed
//   support for proper anonymous union(enum)s,
//   this workaround is necessary to make zig believe they all belong to the same type
// - @"%Type" for type variables to not overlap with existing type names
// - @"%variable" for expression variables to not overlap with existing file-scope const/fn names.
// For the last 2, alternative naming schemes like `var_name` are much harder to
// properly disambiguate. E.g. what if a sloe name actually also starts with var-?
// Prefixing with % is inspired by LLVM IR. Could have just as well used $ or others.
//
// When writing core declarations, this is a little error prone. It is what it is

// TODO do the same for structural tagged unions as for records
pub fn @"|contained|overflowed"(@"%Contained": type, @"%Overflowed": type) type {
    return union(enum) { contained: @"%Contained", overflowed: @"%Overflowed" };
}
pub fn @"|no|yes"(@"%No": type, @"%Yes": type) type {
    return union(enum) { no: @"%No", yes: @"%Yes" };
}
pub fn @"|down|up"(@"%Down": type, @"%Up": type) type {
    return union(enum) { down: @"%Down", up: @"%Up" };
}
pub fn @"|equal|greater|less"(@"%Equal": type, @"%Greater": type, @"%Less": type) type {
    return union(enum) { equal: @"%Equal", greater: @"%Greater", less: @"%Less" };
}
/// would preferably be noreturn but it isn't allowed in parameters for some reason
pub const Choice = enum {};

pub fn Part_rest(@"%Part": type, @"%Rest": type) type {
    return Record(struct { part: @"%Part", rest: @"%Rest" });
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
/// assumed to be a valid utf8 codepoint. Should probably be a wrapper instead.
/// Would be nice if zig had one in std.unicode
pub const Char = u21;
pub const Str = struct {
    /// assumed to contain at least one codepoint. Using "" can lead to UB.
    /// Splitting this into a first u21 and the rest would be no safer because there are no
    /// checked utf8 char wrappers in zig
    utf8: std.unicode.Utf8View,

    pub fn fromComptime(comptime @"%bytes": []const u8) Str {
        return comptime str: {
            const @"%utf8_view" = std.unicode.Utf8View.initComptime(@"%bytes");
            break :str if (Str.fromUtf8View(@"%utf8_view")) |@"%str"| @"%str" else @compileError("Str must contain at least one codepoint");
        };
    }
    pub fn fromUtf8View(@"%utf8_view": std.unicode.Utf8View) ?Str {
        return if (@"%utf8_view".bytes.len >= 1)
            .{ .utf8 = @"%utf8_view" }
        else
            null;
    }
    pub fn utf8_byte_count_p32(@"%str": Str) error{OutOfMemory}!P32 {
        return P32.fromU32(std.math.cast(u32, @"%str".utf8.bytes.len) orelse return error.OutOfMemory).?;
    }
    pub fn codepoint_count_p32(@"%str": Str) error{OutOfMemory}!P32 {
        return P32.fromU32(std.math.cast(
            u32,
            std.unicode.utf8CountCodepoints(@"%str".utf8.bytes) catch unreachable,
        ) orelse return error.OutOfMemory).?;
    }
    pub fn splitStart(@"%str": Str) struct { start: Char, after: std.unicode.Utf8View } {
        var @"%codepoint_iterator" = @"%str".utf8.iterator();
        const @"%start_codepoint" = @"%codepoint_iterator".nextCodepoint().?;
        return .{
            .start = @"%start_codepoint",
            .after = std.unicode.Utf8View.initUnchecked(@"%str".utf8.bytes[@"%codepoint_iterator".i..]),
        };
    }
    pub fn splitEnd(@"%str": Str) struct { before: std.unicode.Utf8View, end: Char } {
        var @"%i" = @"%str".utf8.bytes.len;
        while (@"%i" >= 1) {
            @"%i" -= 1;
            if (std.unicode.utf8ByteSequenceLength(@"%str".utf8.bytes[@"%i"])) |_| {
                var @"%last_codepoint_iterator" = std.unicode.Utf8View.initUnchecked(@"%str".utf8.bytes[@"%i"..]).iterator();
                const @"%end_codepoint" = @"%last_codepoint_iterator".nextCodepoint().?;
                return .{
                    .end = @"%end_codepoint",
                    .before = std.unicode.Utf8View.initUnchecked(@"%str".utf8.bytes[0..@"%i"]),
                };
            } else |@"%not_start_byte"| {
                switch (@"%not_start_byte") {
                    error.Utf8InvalidStartByte => {},
                }
            }
        }
        unreachable;
    }
};
pub fn Fn(@"%In": type, @"%Out": type) type {
    return *const fn (@"%allocator": std.mem.Allocator, @"%In") error{OutOfMemory}!@"%Out";
}
pub const Order =
    @"|equal|greater|less"(void, void, void);
pub fn Opt(@"%Yes": type) type {
    return @"|no|yes"(void, @"%Yes");
}

fn u32AddOrOutOfMem(@"%a": u32, @"%b": u32) error{OutOfMemory}!u32 {
    const sum, const overflow = @addWithOverflow(@"%a", @"%b");
    return if (overflow != 0) error.OutOfMemory else sum;
}
fn usizeAddOrOutOfMem(@"%a": usize, @"%b": usize) error{OutOfMemory}!usize {
    const sum, const overflow = @addWithOverflow(@"%a", @"%b");
    return if (overflow != 0) error.OutOfMemory else sum;
}
fn strideOf(@"%Element": type) comptime_int {
    // at the time of writing, this is the same as
    // @sizeOf(@"%Element")
    // is there a nicer way? And is this always correct in the first place?
    return @sizeOf(@"%Element");
}
fn mathOrderToOrder(@"%order": std.math.Order) Order {
    return switch (@"%order") {
        .lt => .{ .less = {} },
        .eq => .{ .equal = {} },
        .gt => .{ .greater = {} },
    };
}

pub fn Array(@"%Element": type, @"%Record": type) type {
    return [
        switch (@typeInfo(@"%Record")) {
            .@"struct" => |@"%record_type_info"| @max(1, @"%record_type_info".field_names.len),
            else =>
            // no point in throwing a compile error since this cannot happen in sloe-generated code
            // and should not lead to valid (but useless) sloe code not compiling when converted to zig
            1,
        }
    ]@"%Element";
}
pub fn recordToArray(@"%record": anytype) Array(
    @typeInfo(@TypeOf(@"%record")).@"struct".field_types[0],
    @TypeOf(@"%record"),
) {
    // yeah this is all crazy
    const @"%record_struct_type_info" = @typeInfo(@TypeOf(@"%record")).@"struct";
    var @"%actual_array": [@"%record_struct_type_info".field_names.len]@typeInfo(@TypeOf(@"%record")).@"struct".field_types[0] = undefined;
    inline for (0..@"%record_struct_type_info".field_names.len) |@"%actual_array_index"| {
        @"%actual_array"[@"%actual_array_index"] = @field(@"%record", std.fmt.comptimePrint("e{}", .{@"%actual_array_index"}));
    }
    return @"%actual_array";
}
pub fn Origin(@"%Origin": type, @"%Part": type) type {
    return struct {
        pub const origin = @"%Origin";
        pub const part = @"%Part";
    };
}
pub const Erased = struct {};
pub fn Origin_erased(@"%Parts": type, @"%ValueErased": type) type {
    return struct {
        erased: @"%ValueErased",
        pub const parts = @"%Parts";
    };
}
pub fn Origin_eraser(@"%Origin": type, @"%Part": type) type {
    return struct {
        pub const origin = @"%Origin";
        pub const part = @"%Part";
    };
}
pub fn Origin_uneraser(@"%Origin": type, @"%Part": type) type {
    return struct {
        pub const origin = @"%Origin";
        pub const part = @"%Part";
    };
}
pub fn Slot_with_occupancy(@"%Origin": type, @"%Occupancy": type) type {
    return struct {
        index: u32,
        pub const origin = @"%Origin";
        pub const occupancy = @"%Occupancy";
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
        pub fn splitStart(@"%span": @This()) error{OutOfMemory}!struct {
            after: Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
            start: Slot_with_occupancy(@"%Origin", @"%Occupancy"),
        } {
            return .{
                .start = @"%span".start,
                .after = if (P32.fromU32(@"%span".length.predecessor())) |@"%end_length"|
                    .{ .yes = .{
                        .start = .{ .index = try u32AddOrOutOfMem(@"%span".start.index, 1) },
                        .length = @"%end_length",
                    } }
                else
                    .{ .no = {} },
            };
        }
        pub fn splitEnd(@"%span": @This()) error{OutOfMemory}!struct {
            before: Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
            end: Slot_with_occupancy(@"%Origin", @"%Occupancy"),
        } {
            return .{
                .end = .{ .index = try @"%span".endIndex() },
                .before = if (P32.fromU32(@"%span".length.predecessor())) |@"%start_length"|
                    .{ .yes = .{
                        .start = @"%span".start,
                        .length = @"%start_length",
                    } }
                else
                    .{ .no = {} },
            };
        }
        pub fn splitAfterLengthPositive(
            @"%span": @This(),
            @"%start_length_or_greater": P32,
        ) struct {
            after: Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
            start: Span_with_occupancy(@"%Origin", @"%Occupancy"),
        } {
            const @"%start_length" = P32.min(@"%start_length_or_greater", @"%span".length);
            return .{
                .start = .{ .start = @"%span".start, .length = @"%start_length" },
                .after = if (P32.fromU32(@"%span".length.positive - @"%start_length".positive)) |@"%after_length_positive"| .{
                    .yes = .{
                        .start = .{ .index = @"%span".start.index + @"%start_length".positive },
                        .length = @"%after_length_positive",
                    },
                } else .{ .no = {} },
            };
        }
        pub fn splitBeforeEndLengthPositive(
            @"%span": @This(),
            @"%end_length_or_greater": P32,
        ) struct {
            before: Opt(Span_with_occupancy(@"%Origin", @"%Occupancy")),
            end: Span_with_occupancy(@"%Origin", @"%Occupancy"),
        } {
            const @"%end_length" = P32.min(@"%end_length_or_greater", @"%span".length);
            const @"%before_length" = @"%span".length.positive - @"%end_length".positive;
            return .{
                .end = .{
                    .start = .{ .index = @"%span".start.index + @"%before_length" },
                    .length = @"%end_length",
                },
                .before = if (P32.fromU32(@"%before_length")) |@"%before_length_positive"| .{
                    .yes = .{
                        .start = @"%span".start,
                        .length = @"%before_length_positive",
                    },
                } else .{ .no = {} },
            };
        }
        pub fn fold(
            @"%span": Span(@"%Origin"),
            @"%allocator": std.mem.Allocator,
            @"%direction": @"|down|up"(void, void),
            @"%initial_state": anytype,
            @"%step": Fn(Record(struct { slot: Slot(@"%Origin"), state: @TypeOf(@"%initial_state") }), @TypeOf(@"%initial_state")),
        ) error{OutOfMemory}!@TypeOf(@"%initial_state") {
            var @"%state" = @"%initial_state";
            switch (@"%direction") {
                .up => {
                    for (@"%span".start.index..(try @"%span".length.addOrOutOfMem(@"%span".start.index)).positive) |@"%index"| {
                        @"%state" = try @"%step"(@"%allocator", .{
                            .state = @"%state",
                            .slot = .{ .index = @intCast(@"%index") },
                        });
                    }
                },
                .down => {
                    // dear zig, add for (range) in reverse
                    var @"%index": u32 = try u32AddOrOutOfMem(@"%span".start.index, @"%span".length.positive);
                    while (@"%index" > @"%span".start.index) {
                        @"%index" -= 1;
                        @"%state" = try @"%step"(@"%allocator", .{
                            .state = @"%state",
                            .slot = .{ .index = @"%index" },
                        });
                    }
                },
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
            return @intCast(@"%unset_slice".undefined_items.len);
        }
        /// the given unset slice is invalid after
        pub fn castOrRidAndAllocate(
            @"%unset_slice": @This(),
            @"%NewElement": type,
            @"%allocator": std.mem.Allocator,
        ) error{OutOfMemory}!Unset_slice(@"%NewElement") {
            // alignment must match exactly sadly, required by Allocator.free.
            // SmpAllocator for example uses alignment for size classes.
            // The alternative would be carrying original allocation alignment
            // through all uses (Buf, Unset_slice, future collections)
            // which is a price I'm not willing to pay for a niche feature
            if (strideOf(@"%NewElement") == strideOf(@"%Element") and @alignOf(@"%NewElement") == @alignOf(@"%Element")) {
                return .{ .undefined_items = @as(
                    []@"%NewElement",
                    @ptrCast(@"%unset_slice".undefined_items),
                ) };
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
/// - each buf has a unique origin
/// - returned slots, spans, origin-rids are never mem-copied
/// - vacated spans are respected when accesssing elements
///
/// Additionally, when any of the given-out slots and spans are not returned,
/// be aware that the indexes they pointed to are now stale.
/// So: do not ignore them when they point into a persistent `Buf`
///
/// in general, if you really want to directly access .elements,
/// be extra aware of the ABA problem (e.g. a pointer to an element could point to a wrong, new element instead of invalid memory when its index was vacated and re-populated in between)
pub fn Buf(@"%Origin": type, @"%Element": type) type {
    return struct {
        elements: std.ArrayList(@"%Element"),
        vacant: std.ArrayList(Unset_span(@"%Origin")),
        const origin = @"%Origin";

        pub fn preAllocateAtLeast(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%min_pre_allocated_length": u32,
        ) error{OutOfMemory}!void {
            return @"%buf".elements.ensureUnusedCapacity(@"%allocator", @"%min_pre_allocated_length");
        }
        pub fn preAllocationRid(@"%buf": *@This(), @"%allocator": std.mem.Allocator) void {
            return @"%buf".elements.shrinkAndFreePrecise(@"%allocator", @"%buf".elements.items.len);
        }
        pub fn vacantSlotCount(@"%buf": @This()) u32 {
            var @"%combined_length": u32 = 0;
            for (@"%buf".vacant.items) |@"%vacant"| {
                @"%combined_length" += @"%vacant".length.positive;
            }
            return @"%combined_length";
        }
        /// counts both occupied positions and unset ones referenced by `unset-slot` and `unset-span`s
        pub fn notVacantCount(@"%buf": @This()) usize {
            return @"%buf".elements.items.len - @"%buf".vacantSlotCount();
        }
        pub fn add(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Slot(@"%Origin") {
            const @"%new_slot" = Slot(@"%Origin"){
                .index = std.math.cast(u32, @"%buf".elements.items.len) orelse return error.OutOfMemory,
            };
            try @"%buf".elements.append(@"%allocator", @"%new_element");
            return @"%new_slot";
        }
        pub fn addUnset(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
        ) error{OutOfMemory}!Unset_slot(@"%Origin") {
            const @"%new_slot" = Unset_slot(@"%Origin"){
                .index = std.math.cast(u32, @"%buf".elements.items.len) orelse return error.OutOfMemory,
            };
            try @"%buf".elements.append(@"%allocator", undefined);
            return @"%new_slot";
        }
        pub fn addUnsetLength(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%length": U32,
        ) error{OutOfMemory}!Opt(Unset_span(@"%Origin")) {
            if (P32.fromU32(@"%length")) |@"%length_positive"| {
                const @"%span" = @"%buf".addUnsetLengthPositive(@"%allocator", @"%length_positive");
                return .{ .yes = @"%span" };
            } else {
                return .{ .no = {} };
            }
        }
        pub fn addUnsetLengthPositive(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%length": P32,
        ) error{OutOfMemory}!Unset_span(@"%Origin") {
            const @"%start" = std.math.cast(u32, @"%buf".elements.items.len) orelse return error.OutOfMemory;
            try @"%buf".elements.resize(@"%allocator", try u32AddOrOutOfMem(@"%buf".elements.items.len, @"%length".positive));
            return Unset_span(@"%Origin"){
                .start = .{ .index = @"%start" },
                .length = @"%length",
            };
        }
        pub fn insert(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Slot(@"%Origin") {
            const @"%unset_slot" = try @"%buf".insertUnset(@"%allocator");
            return @"%buf".set(@"%unset_slot", @"%new_element");
        }
        fn arrayListLast(T: type, array_list: std.ArrayList(T)) ?*T {
            return array_list.lastPtr();
        }
        pub fn insertUnset(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
        ) error{OutOfMemory}!Unset_slot(@"%Origin") {
            _ = arrayListLast(Unset_span(@"%Origin"), @"%buf".vacant);
            // I couldn't get .last() to return a pointer :/
            if (@"%buf".vacant.lastPtr()) |@"%vacant_span_ptr"| {
                const @"%vacant_span_start_end" = try @"%vacant_span_ptr".splitStart();
                switch (@"%vacant_span_start_end".after) {
                    .no => {
                        _ = @"%buf".vacant.pop();
                    },
                    .yes => |@"%new_shrunk_vacant_span"| {
                        @"%vacant_span_ptr".* = @"%new_shrunk_vacant_span";
                    },
                }
                return @"%vacant_span_start_end".start;
            } else {
                return @"%buf".addUnset(@"%allocator");
            }
        }
        /// slot is invalid while resulting ptr is live
        pub fn element(@"%buf": @This(), @"%slot": Slot(@"%Origin")) *@"%Element" {
            return &@"%buf".elements.items[@"%slot".index];
        }
        pub fn remove(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%slot": Slot(@"%Origin"),
        ) error{OutOfMemory}!@"%Element" {
            const @"%accessed" = @"%buf".unset(@"%slot");
            try @"%buf".slotRid(@"%allocator", @"%accessed".slot);
            return @"%accessed".element;
        }
        pub fn unset(
            @"%buf": @This(),
            @"%slot": Slot(@"%Origin"),
        ) struct { element: @"%Element", slot: Unset_slot(@"%Origin") } {
            const @"%accessed_element" = @"%buf".element(@"%slot").*;
            return .{
                .element = @"%accessed_element",
                .slot = .{ .index = @"%slot".index },
            };
        }
        pub fn set(
            @"%buf": @This(),
            @"%slot": Unset_slot(@"%Origin"),
            @"%new": @"%Element",
        ) Slot(@"%Origin") {
            @"%buf".elements.items[@"%slot".index] = @"%new";
            return .{ .index = @"%slot".index };
        }
        // The given span is invalid while the returned slice is live.
        pub fn spanSlice(@"%buf": @This(), @"%span": Span(@"%Origin")) []@"%Element" {
            return @"%buf".elements.items[@"%span".start.index..][0..@"%span".length.positive];
        }
        // The given span is invalid while the returned slice is live.
        pub fn optSpanSlice(@"%buf": @This(), @"%opt_span": Opt(Span(@"%Origin"))) []@"%Element" {
            return switch (@"%opt_span") {
                .no => &.{},
                .yes => |@"%span"| @"%buf".spanSlice(@"%span"),
            };
        }
        /// The returned slice is only valid while buf.elements.items is live.
        /// The returned unset span is only valid once all elements in the slice have been used
        fn spanElements(
            @"%buf": @This(),
            @"%span": Span(@"%Origin"),
        ) struct { slice: []@"%Element", span: Unset_span(@"%Origin") } {
            const @"%slice" = @"%buf".spanSlice(@"%span");
            return .{
                .slice = @"%slice",
                .span = .{
                    .start = .{ .index = @"%span".start.index },
                    .length = @"%span".length,
                },
            };
        }
        /// The returned slice is only valid while buf.elements.items is live
        pub fn optSpanElements(
            @"%buf": @This(),
            @"%opt_span": Opt(Span(@"%Origin")),
        ) struct { slice: []@"%Element", span: Opt(Unset_span(@"%Origin")) } {
            switch (@"%opt_span") {
                .no => return .{ .slice = []@"%Element", .span = .{ .no = {} } },
                .yes => |@"%span"| {
                    return @"%buf".spanElements(@"%span");
                },
            }
        }
        pub fn slotRid(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%slot": Unset_slot(@"%Origin"),
        ) error{OutOfMemory}!void {
            // can maybe be optimized
            return @"%buf".spanRid(@"%allocator", @"%slot".to_span());
        }
        pub fn optSpanRid(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span_to_vacate": Opt(Unset_span(@"%Origin")),
        ) error{OutOfMemory}!void {
            switch (@"%opt_span_to_vacate") {
                .no => {},
                .yes => |@"%span_to_vacate"| {
                    return @"%buf".spanRid(@"%allocator", @"%span_to_vacate");
                },
            }
        }
        pub fn spanRid(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span_to_vacate": Unset_span(@"%Origin"),
        ) error{OutOfMemory}!void {
            var @"%maybe_vacant_span_index_connecting_earlier": ?usize = null;
            var @"%maybe_vacant_span_index_connecting_later": ?usize = null;
            looking_for_connections: for (@"%buf".vacant.items, 0..) |@"%vacant_span", @"%vacant_span_index"| {
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
                var @"%vacant_span_connecting_earlier" = &@"%buf".vacant.items[@"%vacant_span_index_connecting_earlier"];
                if (@"%maybe_vacant_span_index_connecting_later") |@"%vacant_span_index_connecting_later"| {
                    const @"%vacant_span_connecting_later" = @"%buf".vacant.items[@"%vacant_span_index_connecting_later"];
                    @"%vacant_span_connecting_earlier".length = try @"%vacant_span_connecting_earlier".length.addOrOutOfMem(
                        (try @"%span_to_vacate".length.addOrOutOfMem(@"%vacant_span_connecting_later".length.positive)).positive,
                    );
                    _ = @"%buf".vacant.swapRemove(@"%vacant_span_index_connecting_later");
                } else {
                    // maybeVacantSpanIndexConnectingLater == null
                    if (@as(usize, @"%span_to_vacate".start.index) + @as(usize, @"%span_to_vacate".length.positive) == @"%buf".elements.items.len) {
                        @"%buf".elements.shrinkRetainingCapacity(
                            @"%buf".elements.items.len - @as(usize, @"%vacant_span_connecting_earlier".length.positive) - @as(usize, @"%span_to_vacate".length.positive),
                        );
                        _ = @"%buf".vacant.swapRemove(@"%vacant_span_index_connecting_earlier");
                    } else {
                        @"%vacant_span_connecting_earlier".length = try @"%vacant_span_connecting_earlier".length.addOrOutOfMem(@"%span_to_vacate".length.positive);
                    }
                }
            } else if (@"%maybe_vacant_span_index_connecting_later") |@"%vacant_span_index_connecting_later"| {
                // maybeVacantSpanIndexConnectingEarlier == null
                var @"%vacant_span_connecting_later" = &@"%buf".vacant.items[@"%vacant_span_index_connecting_later"];
                @"%vacant_span_connecting_later".* = Unset_span(@"%Origin"){
                    .start = @"%span_to_vacate".start,
                    .length = try @"%vacant_span_connecting_later".length.addOrOutOfMem(
                        @"%span_to_vacate".length.positive,
                    ),
                };
            } else {
                // maybeVacantSpanIndexConnectingEarlier == null and maybeVacantSpanIndexConnectingLater == null
                if (@as(usize, @"%span_to_vacate".start.index) + @as(usize, @"%span_to_vacate".length.positive) == @"%buf".elements.items.len) {
                    @"%buf".elements.shrinkRetainingCapacity(
                        std.math.sub(usize, @"%buf".elements.items.len, @"%span_to_vacate".length.positive) catch 0,
                    );
                } else {
                    try @"%buf".vacant.append(@"%allocator", @"%span_to_vacate");
                }
            }
        }
        pub fn spanMoveToEnd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            if (@as(usize, @"%span".start.index) + @as(usize, @"%span".length.positive) == @"%buf".elements.items.len) {
                return @"%span";
            }
            // span is not at the end already
            const @"%move_destination_start" = std.math.cast(u32, @"%buf".elements.items.len) orelse return error.OutOfMemory;
            try @"%buf".elements.ensureUnusedCapacity(@"%allocator", @"%span".length.positive);
            @"%buf".elements.appendSliceAssumeCapacity(@"%buf".spanSlice(@"%span"));
            try @"%buf".spanRid(@"%allocator", Unset_span(@"%Origin"){
                .start = .{ .index = @"%span".start.index },
                .length = @"%span".length,
            });
            return Span(@"%Origin"){
                .start = .{ .index = @"%move_destination_start" },
                .length = @"%span".length,
            };
        }
        pub fn spanMoveToVacant(@"%buf": *@This(), @"%span": Span(@"%Origin")) Span(@"%Origin") {
            if (@as(usize, @"%span".start.index) + @as(usize, @"%span".length.positive) < @"%buf".elements.items.len) {
                return @"%span";
            }
            // span is at the end of elements
            if (@"%buf".markLengthPositiveAsOccupied(@"%span".length)) |@"%earlier_start_to_occupy_from"| {
                @"%buf".elements.replaceRangeAssumeCapacity(
                    @"%earlier_start_to_occupy_from",
                    @"%span".length.positive,
                    @"%buf".spanSlice(@"%span"),
                );
                @"%buf".elements.shrinkRetainingCapacity(@"%buf".elements.items.len - @"%span".length.positive);
                return Span(@"%Origin"){
                    .start = .{ .index = @"%earlier_start_to_occupy_from" },
                    .length = @"%span".length,
                };
            } else {
                return @"%span";
            }
        }
        pub fn spanAddOwnSpan(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%start": Span(@"%Origin"),
            @"%end": Span(@"%Origin"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%combined_length" = @"%start".length.addOrOutOfMem(@"%end".length.positive);
            if (u32AddOrOutOfMem(@"%start".start.index, @"%start".length.positive) == @"%end".start.index) {
                return Span(@"%Origin"){ .start = @"%start".start, .length = @"%combined_length" };
            } else {
                const @"%moved_start" = try @"%buf".spanMoveToEnd(@"%allocator", @"%start");
                _ = try @"%buf".spanMoveToEnd(@"%allocator", @"%end");
                return Span(@"%Origin"){ .start = @"%moved_start".start, .length = @"%combined_length" };
            }
        }
        pub fn unsetSpanAddOwnSpan(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%start": Unset_span(@"%Origin"),
            @"%end": Unset_span(@"%Origin"),
        ) error{OutOfMemory}!Unset_span(@"%Origin") {
            const @"%combined_length" = @"%start".length.addOrOutOfMem(@"%end".length.positive);
            if (u32AddOrOutOfMem(@"%start".start.index, @"%start".length.positive) == @"%end".start.index) {
                return Unset_span(@"%Origin"){ .start = @"%start".start, .length = @"%combined_length" };
            } else {
                return @"%buf".addUnsetLengthPositive(@"%allocator", @"%combined_length");
            }
        }
        pub fn unsetSpanAdd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Unset_span(@"%Origin"),
            @"%length_increase": Unset_span(@"%Origin"),
        ) error{OutOfMemory}!Unset_span(@"%Origin") {
            const @"%combined_length" = @"%span".length.addOrOutOfMem(@"%length_increase");
            if (@as(usize, @"%span".start.index) + @as(usize, @"%span".length.positive) < @"%buf".elements.items.len) {
                try @"%buf".spanRid(@"%span");
                return @"%buf".addUnsetLengthPositive(@"%allocator", @"%combined_length");
            }
            // span is at the end of elements
            try @"%buf".elements.resize(@"%allocator", try u32AddOrOutOfMem(@"%buf".elements.items.len, @"%span".length.positive));
            return Unset_span(@"%Origin"){ .start = @"%span".start, .length = @"%combined_length" };
        }
        fn markLengthPositiveAsOccupied(@"%buf": *@This(), @"%length_to_occupy": P32) ?u32 {
            for (@"%buf".vacant.items, 0..) |*@"%vacant", @"%vacant_index"| {
                if (@"%vacant".length.positive > @"%length_to_occupy".positive) {
                    @"%vacant".length.positive -|= @"%length_to_occupy".positive;
                    return @"%vacant".start.index;
                } else if (@"%vacant".length.positive == @"%length_to_occupy".positive) {
                    return @"%buf".vacant.swapRemove(@"%vacant_index").start.index;
                }
            }
            return null;
        }
        // add insertSlice?
        pub fn addSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_elements": []const @"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            if (P32.fromU32(std.math.cast(u32, @"%new_elements".len) orelse return error.OutOfMemory)) |@"%new_length"| {
                const @"%length_before_add" = @"%buf".elements.items.len;
                try @"%buf".elements.appendSlice(@"%allocator", @"%new_elements");
                return .{ .yes = .{
                    .start = .{
                        .index = std.math.cast(u32, @"%length_before_add") orelse return error.OutOfMemory,
                    },
                    .length = @"%new_length",
                } };
            } else return .{ .no = {} };
        }
        // add insertIterator?
        pub fn addIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_elements": anytype,
            @"%next_element": fn (*@TypeOf(@"%new_elements")) ?@"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            const @"%length_before_add" = @"%buf".elements.items.len;
            var @"%new_elements_iterator" = @"%new_elements";
            while (@"%next_element"(&@"%new_elements_iterator")) |@"%new_element"| {
                try @"%buf".elements.append(@"%allocator", @"%new_element");
            }
            return if (P32.fromU32(
                std.math.cast(u32, @"%buf".elements.items.len - @"%length_before_add") orelse return error.OutOfMemory,
            )) |@"%new_length"|
                .{ .yes = .{
                    .start = .{
                        .index = std.math.cast(u32, @"%length_before_add") orelse return error.OutOfMemory,
                    },
                    .length = @"%new_length",
                } }
            else
                .{ .no = {} };
        }
        pub fn addArray(
            @"%buf": *@This(),
            @"%Record": type,
            @"%allocator": std.mem.Allocator,
            @"%new_elements": Array(@"%Element", @"%Record"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%length_before_add" = @"%buf".elements.items.len;
            try @"%buf".elements.appendSlice(@"%allocator", &@"%new_elements");
            return .{
                .start = .{
                    .index = std.math.cast(u32, @"%length_before_add") orelse return error.OutOfMemory,
                },
                .length = P32.fromU32(
                    std.math.cast(u32, @"%buf".elements.items.len - @"%length_before_add") orelse return error.OutOfMemory,
                ).?,
            };
        }
        pub fn optSpanAdd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            return switch (@"%opt_span") {
                .no => (try @"%buf".add(@"%allocator", @"%new_element")).to_span(),
                .yes => |@"%span"| @"%buf".spanAdd(@"%allocator", @"%span", @"%new_element"),
            };
        }
        pub fn spanAdd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_element": @"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            try @"%buf".elements.append(@"%allocator", @"%new_element");
            return Span(@"%Origin"){ .start = @"%moved_span".start, .length = try @"%moved_span".length.addOrOutOfMem(1) };
        }
        pub fn optSpanAddSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_elements": []const @"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            return switch (@"%opt_span") {
                .no => @"%buf".addSlice(@"%allocator", @"%new_elements"),
                .yes => |@"%span"| .{ .yes = try @"%buf".spanAddSlice(@"%allocator", @"%span", @"%new_elements") },
            };
        }
        pub fn spanAddSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_elements": []const @"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            try @"%buf".elements.appendSlice(@"%allocator", @"%new_elements");
            return Span(@"%Origin"){
                .start = @"%moved_span".start,
                .length = try @"%moved_span".length.addOrOutOfMem(
                    std.math.cast(u32, @"%new_elements".len) orelse return error.OutOfMemory,
                ),
            };
        }
        pub fn optSpanAddArray(
            @"%buf": *@This(),
            @"%Record": type,
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_elements": Array(@"%Element", @"%Record"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            return switch (@"%opt_span") {
                .no => @"%buf".addArray(@"%Record", @"%allocator", @"%new_elements"),
                .yes => |@"%span"| try @"%buf".spanAddSlice(@"%allocator", @"%span", &@"%new_elements"),
            };
        }
        pub fn optSpanAddIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_elements": anytype,
            @"%next_element": fn (*@TypeOf(@"%new_elements")) ?@"%Element",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            return switch (@"%opt_span") {
                .no => @"%buf".addIterator(@"%allocator", @"%new_elements", @"%next_element"),
                .yes => |@"%span"| .{ .yes = try @"%buf".spanAddIterator(@"%allocator", @"%span", @"%new_elements", @"%next_element") },
            };
        }
        pub fn spanAddIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_elements": anytype,
            @"%next_element": fn (*@TypeOf(@"%new_elements")) ?@"%Element",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            const @"%length_before_add" = @"%buf".elements.items.len;
            var @"%new_elements_iterator" = @"%new_elements";
            while (@"%next_element"(&@"%new_elements_iterator")) |@"%new_element"| {
                try @"%buf".elements.append(@"%allocator", @"%new_element");
            }
            const @"%new_length" = std.math.cast(u32, @"%buf".elements.items.len - @"%length_before_add") orelse return error.OutOfMemory;
            return Span(@"%Origin"){
                .start = @"%moved_span".start,
                .length = try @"%moved_span".length.addOrOutOfMem(@"%new_length"),
            };
        }
        pub fn spanReverse(@"%buf": @This(), @"%span": Span(@"%Origin")) Span(@"%Origin") {
            std.mem.reverse(@"%Element", @"%buf".spanSlice(@"%span"));
            return @"%span";
        }
        pub fn optSpanReverse(@"%buf": @This(), @"%opt_span": Opt(Span(@"%Origin"))) Opt(Span(@"%Origin")) {
            std.mem.reverse(@"%Element", @"%buf".optSpanSlice(@"%opt_span"));
            return @"%opt_span";
        }
        /// buf is invalid after
        pub fn intoUnsetSlice(
            @"%buf": @This(),
            @"%allocator": std.mem.Allocator,
        ) Unset_slice(@"%Element") {
            var @"%vacant" = @"%buf".vacant;
            @"%vacant".deinit(@"%allocator");
            var @"%elements" = @"%buf".elements;
            @"%elements".clearRetainingCapacity();
            return .{ .undefined_items = @"%elements".unusedCapacitySlice() };
        }
        /// buf is invalid after
        pub fn rid(@"%buf": @This(), @"%allocator": std.mem.Allocator) void {
            var @"%buf_mut" = @"%buf";
            @"%buf_mut".elements.deinit(@"%allocator");
            @"%buf_mut".vacant.deinit(@"%allocator");
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
pub fn p32_dup(@"%n": P32) error{OutOfMemory}!Record(struct { a: P32, b: P32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn p32_to_u32(@"%n": P32) error{OutOfMemory}!U32 {
    return @"%n".positive;
}
pub fn p32_add_clamp(@"%": Record(struct { p: P32, u: U32 })) error{OutOfMemory}!P32 {
    return @"%".p.addClamp(@"%".u);
}
pub fn p32_mul_clamp(@"%": Record(struct { a: P32, b: P32 })) error{OutOfMemory}!P32 {
    return @"%".a.mulClamp(@"%".b);
}
pub fn p32_order(@"%": Record(struct { left: P32, right: P32 })) error{OutOfMemory}!Order {
    return mathOrderToOrder(std.math.order(@"%".left.positive, @"%".right.positive));
}

pub fn u32_rid(_: U32) error{OutOfMemory}!void {}
pub fn u32_dup(@"%n": U32) error{OutOfMemory}!Record(struct { a: U32, b: U32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn u32_to_i32_clamp(@"%n": U32) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, @"%n");
}
pub fn u32_round_to_nearest_f32_else_even(@"%n": U32) error{OutOfMemory}!F32 {
    // see i32_round_to_nearest_f32_else_even for details
    return @floatFromInt(@"%n");
}
pub fn u32_successor_clamp(@"%n": U32) error{OutOfMemory}!P32 {
    return .{ .positive = @"%n" +| 1 };
}
pub fn u32_add_clamp(@"%": Record(struct { a: U32, b: U32 })) error{OutOfMemory}!U32 {
    return @"%".a +| @"%".b;
}
pub fn u32_add_i32_clamp(@"%": Record(struct { i: I32, u: U32 })) error{OutOfMemory}!U32 {
    return std.math.lossyCast(u32, @as(i33, @"%".u) +| @as(i33, @"%".i));
}
pub fn u32_mul_clamp(@"%": Record(struct { a: U32, b: U32 })) error{OutOfMemory}!U32 {
    return @"%".a *| @"%".b;
}
pub fn u32_pow_clamp(@"%": Record(struct { base: U32, exponent: P32 })) error{OutOfMemory}!U32 {
    return std.math.powi(u32, @"%".base, @"%".exponent.positive) catch std.math.maxInt(u32);
}
pub fn u32_to_p32(@"%n": U32) error{OutOfMemory}!Opt(P32) {
    return if (P32.fromU32(@"%n")) |@"%p32"| .{ .yes = @"%p32" } else .{ .no = {} };
}
pub fn u32_order(@"%": Record(struct { left: U32, right: U32 })) error{OutOfMemory}!Order {
    return mathOrderToOrder(std.math.order(@"%".left, @"%".right));
}

pub fn i32_rid(_: I32) error{OutOfMemory}!void {}
pub fn i32_dup(@"%n": I32) error{OutOfMemory}!Record(struct { a: I32, b: I32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn i32_to_u32(@"%i": I32) error{OutOfMemory}!Opt(U32) {
    return if (std.math.cast(U32, @"%i")) |@"%u"| .{ .yes = @"%u" } else .{ .no = {} };
}
pub fn i32_round_to_nearest_f32_else_even(@"%n": I32) error{OutOfMemory}!F32 {
    // - custom backend: explicit round ties to even
    //   https://codeberg.org/ziglang/zig/src/branch/master/lib/compiler_rt/float_from_int.zig#L508-L509
    // - in llvm, this compiles to sitofp which uses the default rounding mode
    //   @setRoundMode is set to strict by default which claims IEEE compliance
    //   IEEE specifies round ties to even (see §4.3.3, §7.4 in IEEE 754-2008)
    return @floatFromInt(@"%n");
}
pub fn i32_add_clamp(@"%": Record(struct { a: I32, b: I32 })) error{OutOfMemory}!I32 {
    return @"%".a +| @"%".b;
}
pub fn i32_mul_clamp(@"%": Record(struct { a: I32, b: I32 })) error{OutOfMemory}!I32 {
    return @"%".a *| @"%".b;
}
pub fn i32_pow_clamp(@"%": Record(struct { base: I32, exponent: P32 })) error{OutOfMemory}!I32 {
    return std.math.powi(i32, @"%".base, std.math.lossyCast(i32, @"%".exponent.positive)) catch std.math.maxInt(i32);
}
pub fn i32_negate_clamp(@"%n": I32) error{OutOfMemory}!I32 {
    return 0 -| @"%n";
}
pub fn i32_abs_to_u32(@"%n": I32) error{OutOfMemory}!U32 {
    return @abs(@"%n");
}
pub fn i32_order(@"%": Record(struct { left: I32, right: I32 })) error{OutOfMemory}!Order {
    return mathOrderToOrder(std.math.order(@"%".left, @"%".right));
}

pub fn f32_rid(_: F32) error{OutOfMemory}!void {}
pub fn f32_dup(@"%n": F32) error{OutOfMemory}!Record(struct { a: F32, b: F32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn f32_pi(_: void) error{OutOfMemory}!F32 {
    return std.math.pi;
}
pub fn f32_negate(@"%n": F32) error{OutOfMemory}!F32 {
    return -@"%n";
}
pub fn f32_abs(@"%n": F32) error{OutOfMemory}!F32 {
    return @abs(@"%n");
}
pub fn f32_ln(@"%n": F32) error{OutOfMemory}!Opt(F32) {
    if (@"%n" <= 0) {
        return .{ .no = {} };
    } else {
        const @"%ln_result" = @log(@"%n");
        return if (std.math.isFinite(@"%ln_result")) .{ .yes = @"%ln_result" } else .{ .no = {} };
    }
}
pub fn f32_exp(@"%n": F32) error{OutOfMemory}!F32 {
    return @min(@exp(@"%n"), std.math.floatMax(f32));
}
pub fn f32_sin(@"%n": F32) error{OutOfMemory}!F32 {
    return @sin(@"%n");
}
pub fn f32_cos(@"%n": F32) error{OutOfMemory}!F32 {
    return @cos(@"%n");
}
pub fn f32_tan(@"%n": F32) error{OutOfMemory}!F32 {
    return @tan(@"%n");
}
pub fn f32_atan(@"%n": F32) error{OutOfMemory}!F32 {
    return std.math.atan(@"%n");
}
pub fn f32_round_up(@"%n": F32) error{OutOfMemory}!F32 {
    return @ceil(@"%n");
}
pub fn f32_round_down(@"%n": F32) error{OutOfMemory}!F32 {
    return @floor(@"%n");
}
pub fn f32_round_toward_0(@"%n": F32) error{OutOfMemory}!F32 {
    return @trunc(@"%n");
}
pub fn f32_round_away_from_0(@"%n": F32) error{OutOfMemory}!F32 {
    return @ceil(@abs(@"%n")) * std.math.sign(@"%n");
}
pub fn f32_round_nearest_else_away_from_0(@"%n": F32) error{OutOfMemory}!F32 {
    return @round(@"%n");
}
pub fn f32_round_nearest_else_even(@"%n": F32) error{OutOfMemory}!F32 {
    // your move zig. Please add an intrinsic
    const @"%mod" = std.math.modf(@"%n");
    return if (@"%mod".fpart == 0.0) @"%n" else if (@abs(@"%mod".fpart) == 0.5)
        (
            // @"%n" is on the midpoint
            if (@mod(@"%mod".ipart, 2) == 1)
                // is odd
                //  11.5 ->  12
                // -11.5 -> -12
                @round(@"%n")
            else
                // @"%n" is even
                //  10.5 ->  10, not  11
                // -10.5 -> -10, not -11
                (@round(@"%n") - std.math.sign(@"%n")))
    else
        @round(@"%n");
}
pub fn f32_round_up_to_i32_clamp(@"%n": F32) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, try f32_round_up(@"%n"));
}
pub fn f32_round_down_to_i32_clamp(@"%n": F32) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, try f32_round_down(@"%n"));
}
pub fn f32_round_toward_0_to_i32_clamp(@"%n": F32) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, try f32_round_toward_0(@"%n"));
}
pub fn f32_round_away_from_0_to_i32_clamp(@"%n": F32) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, try f32_round_away_from_0(@"%n"));
}
pub fn f32_round_nearest_else_away_from_0_to_i32_clamp(@"%n": F32) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, try f32_round_nearest_else_away_from_0(@"%n"));
}
pub fn f32_round_nearest_else_even_to_i32_clamp(@"%n": F32) error{OutOfMemory}!I32 {
    return std.math.lossyCast(i32, try f32_round_nearest_else_even(@"%n"));
}
pub fn f32_add_clamp(@"%": Record(struct { a: F32, b: F32 })) error{OutOfMemory}!F32 {
    const @"%sum" = @"%".a + @"%".b;
    return if (std.math.isNegativeInf(@"%sum")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%sum")) std.math.floatMax(f32) else @"%sum";
}
pub fn f32_mul_clamp(@"%": Record(struct { a: F32, b: F32 })) error{OutOfMemory}!F32 {
    const @"%product" = @"%".a * @"%".b;
    return if (std.math.isNegativeInf(@"%product")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%product")) std.math.floatMax(f32) else @"%product";
}
pub fn f32_div_clamp(@"%": Record(struct { n: F32, by: F32 })) error{OutOfMemory}!F32 {
    return if (@"%".by == 0.0) 0.0 else {
        const @"%div_result" = @"%".n / @"%".by;
        return if (std.math.isNegativeInf(@"%div_result")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%div_result")) std.math.floatMax(f32) else @"%div_result";
    };
}
pub fn f32_pow_i32(@"%": Record(struct { base: F32, exponent: I32 })) error{OutOfMemory}!Opt(F32) {
    const @"%power" = std.math.pow(f32, @"%".base, @floatFromInt(@"%".exponent));
    return if (std.math.isFinite(@"%power")) .{ .yes = @"%power" } else .{ .no = {} };
}
pub fn f32_pow(@"%": Record(struct { base: F32, exponent: F32 })) error{OutOfMemory}!Opt(F32) {
    const @"%power" = std.math.pow(f32, @"%".base, @"%".exponent);
    return if (std.math.isFinite(@"%power")) .{ .yes = @"%power" } else .{ .no = {} };
}
pub fn f32_order(@"%": Record(struct { left: F32, right: F32 })) error{OutOfMemory}!Order {
    return mathOrderToOrder(std.math.order(@"%".left, @"%".right));
}

pub fn char_rid(_: Char) error{OutOfMemory}!void {}
pub fn char_to_u32(@"%char": Char) error{OutOfMemory}!U32 {
    return @"%char";
}
pub fn char_dup(@"%n": Char) error{OutOfMemory}!Record(struct { a: Char, b: Char }) {
    return .{ .a = @"%n", .b = @"%n" };
}

pub fn str_rid(_: Str) error{OutOfMemory}!void {}
pub fn str_dup(@"%str": Str) error{OutOfMemory}!Record(struct { a: Str, b: Str }) {
    return .{ .a = @"%str", .b = @"%str" };
}
pub fn str_utf8_length(@"%str": Str) error{OutOfMemory}!P32 {
    return @"%str".utf8_byte_count_p32();
}
pub fn str_char_count(@"%str": Str) error{OutOfMemory}!P32 {
    return @"%str".codepoint_count_p32();
}
pub fn str_start(@"%str": Str) error{OutOfMemory}!Record(struct { after: Opt(Str), start: Char }) {
    const @"%split" = @"%str".splitStart();
    return .{
        .start = @"%split".start,
        .after = if (Str.fromUtf8View(@"%split".after)) |@"%after"| .{ .yes = @"%after" } else .{ .no = {} },
    };
}
pub fn str_end(@"%str": Str) error{OutOfMemory}!Record(struct { before: Opt(Str), end: Char }) {
    const @"%split" = @"%str".splitEnd();
    return .{
        .end = @"%split".end,
        .before = if (Str.fromUtf8View(@"%split".before)) |@"%before"| .{ .yes = @"%before" } else .{ .no = {} },
    };
}

pub fn fn_rid(@"%In": type, @"%Out": type, _: Fn(@"%In", @"%Out")) error{OutOfMemory}!void {}
pub fn fn_dup(
    @"%In": type,
    @"%Out": type,
    @"%function": Fn(@"%In", @"%Out"),
) error{OutOfMemory}!Record(struct { a: Fn(@"%In", @"%Out"), b: Fn(@"%In", @"%Out") }) {
    return .{ .a = @"%function", .b = @"%function" };
}
pub inline fn call(
    @"%In": type,
    @"%Out": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { @"fn": Fn(@"%In", @"%Out"), in: @"%In" }),
) error{OutOfMemory}!@"%Out" {
    return @"%".@"fn"(@"%allocator", @"%".in);
}

pub fn choice_empty_to(
    @"%Result": type,
    @"%impossible": Choice,
) error{OutOfMemory}!@"%Result" {
    return switch (@"%impossible") {};
}

pub fn opt_yes(@"%Yes": type, @"%yes": @"%Yes") Opt(@"%Yes") {
    return .{ .present = @"%yes" };
}

pub fn origin_rid(@"%Origin": type, @"%Part": type, _: Origin(@"%Origin", @"%Part")) error{OutOfMemory}!void {}
pub fn origin_add(
    @"%PartName": type,
    @"%PartOrigin": type,
    @"%RestName": type,
    @"%RestOrigin": type,
    @"%": Record(struct {
        part: Origin(@"%PartOrigin", @"%PartName"),
        rest: Origin(@"%RestOrigin", @"%RestName"),
    }),
) error{OutOfMemory}!Origin(
    Record(struct { part: @"%PartOrigin", rest: @"%RestOrigin" }),
    Record(struct { part: @"%PartName", rest: @"%RestName" }),
) {
    return .{
        .origin = .{ .rest = @"%".rest.origin, .part = @"%".part.origin },
        .part = .{ .rest = @"%".rest.part, .part = @"%".part.part },
    };
}
pub fn origin_part(
    @"%Origin": type,
    @"%Part": type,
    @"%Rest": type,
    _: Origin(@"%Origin", Record(struct { part: @"%Part", rest: @"%Rest" })),
) error{OutOfMemory}!Record(struct {
    part: Origin(@"%Origin", @"%Part"),
    rest: Origin(@"%Origin", @"%Rest"),
}) {
    return .{
        .part = .{},
        .rest = .{},
    };
}

pub fn origin_erase(
    @"%Origin": type,
    @"%Parts": type,
    @"%Value": type,
    @"%ValueErased": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        value: @"%Value",
        erase: Fn(
            Record(struct {
                value: @"%Value",
                eraser: Origin_eraser(@"%Origin", @"%Parts"),
            }),
            @"%ValueErased",
        ),
    }),
) error{OutOfMemory}!Origin_erased(@"%Parts", @"%ValueErased") {
    return .{ .erased = try @"%".erase(@"%allocator", .{ .value = @"%".value, .eraser = .{} }) };
}
pub fn origin_unerase(
    @"%Origin": type,
    @"%Parts": type,
    @"%Value": type,
    @"%ValueErased": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        erased: Origin_erased(@"%Parts", @"%ValueErased"),
        origin: Origin(@"%Origin", @"%Parts"),
        unerase: Fn(
            Record(struct {
                erased: @"%ValueErased",
                uneraser: Origin_uneraser(@"%Origin", @"%Parts"),
            }),
            @"%Value",
        ),
        value_rid: Fn(@"%Value", void),
    }),
) error{OutOfMemory}!@"%Value" {
    return try @"%".unerase(@"%allocator", .{
        .uneraser = .{},
        .erased = @"%".erased.erased,
    });
}

pub fn origin_eraser_part(
    @"%Origin": type,
    @"%Part": type,
    @"%Rest": type,
    _: Origin_eraser(@"%Origin", Record(struct { part: @"%Part", rest: @"%Rest" })),
) error{OutOfMemory}!Record(struct {
    part: Origin_eraser(@"%Origin", @"%Part"),
    rest: Origin_eraser(@"%Origin", @"%Rest"),
}) {
    return .{ .part = .{}, .rest = .{} };
}
pub fn origin_uneraser_part(
    @"%Origin": type,
    @"%Part": type,
    @"%Rest": type,
    _: Origin_uneraser(@"%Origin", Record(struct { part: @"%Part", rest: @"%Rest" })),
) error{OutOfMemory}!Record(struct {
    part: Origin_uneraser(@"%Origin", @"%Part"),
    rest: Origin_uneraser(@"%Origin", @"%Rest"),
}) {
    return .{
        .part = .{},
        .rest = .{},
    };
}

pub fn slot_index(
    @"%Origin": type,
    @"%slot": Slot(@"%Origin"),
) error{OutOfMemory}!Record(struct { index: U32, slot: Slot(@"%Origin") }) {
    return .{ .slot = @"%slot", .index = @"%slot".index };
}
pub fn slot_to_span(@"%Origin": type, @"%slot": Slot(@"%Origin")) error{OutOfMemory}!Span(@"%Origin") {
    return @"%slot".to_span();
}
pub fn slot_origin_erase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    slot: Slot(Origin(@"%Origin", @"%Part")),
    eraser: Origin_eraser(@"%Origin", @"%Part"),
})) error{OutOfMemory}!Record(struct {
    slot: Slot(Origin(Erased, @"%Part")),
    eraser: Origin_eraser(@"%Origin", @"%Part"),
}) {
    return .{
        .slot = .{ .index = @"%".slot.index },
        .eraser = @"%".eraser,
    };
}
pub fn slot_origin_unerase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    slot: Slot(Origin(Erased, @"%Part")),
    uneraser: Origin_uneraser(@"%Origin", @"%Part"),
})) error{OutOfMemory}!Record(struct {
    slot: Slot(Origin(@"%Origin", @"%Part")),
    uneraser: Origin_uneraser(@"%Origin", @"%Part"),
}) {
    return .{
        .slot = .{ .index = @"%".slot.index },
        .uneraser = @"%".uneraser,
    };
}

pub fn unset_slot_index(
    @"%Origin": type,
    @"%slot": Unset_slot(@"%Origin"),
) error{OutOfMemory}!Record(struct { index: U32, slot: Unset_slot(@"%Origin") }) {
    return .{ .slot = @"%slot", .index = @"%slot".index };
}
pub fn unset_slot_to_span(@"%Origin": type, @"%slot": Unset_slot(@"%Origin")) error{OutOfMemory}!Span(@"%Origin") {
    return @"%slot".to_span();
}

pub fn span_length(
    @"%Origin": type,
    @"%span": Span(@"%Origin"),
) error{OutOfMemory}!Record(struct { length: P32, span: Span(@"%Origin") }) {
    return .{ .span = @"%span", .length = @"%span".length };
}
pub fn opt_span_length(
    @"%Origin": type,
    @"%opt_span": Opt(Span(@"%Origin")),
) error{OutOfMemory}!Record(struct { length: U32, span: Opt(Span(@"%Origin")) }) {
    return .{
        .span = @"%opt_span",
        .length = switch (@"%opt_span") {
            .no => 0,
            .yes => |@"%span"| @"%span".length.positive,
        },
    };
}
pub fn span_start(
    @"%Origin": type,
    @"%span": Span(@"%Origin"),
) error{OutOfMemory}!Record(struct { after: Opt(Span(@"%Origin")), start: Slot(@"%Origin") }) {
    return record(try @"%span".splitStart());
}
pub fn span_end(
    @"%Origin": type,
    @"%span": Span(@"%Origin"),
) error{OutOfMemory}!Record(struct { before: Opt(Span(@"%Origin")), end: Slot(@"%Origin") }) {
    return record(try @"%span".splitEnd());
}
pub fn span_start_of_length_positive(
    @"%Origin": type,
    @"%": Record(struct { length: P32, span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct {
    after: Opt(Span(@"%Origin")),
    start: Span(@"%Origin"),
}) {
    return record(@"%".span.splitAfterLengthPositive(@"%".length));
}
pub fn span_end_of_length_positive(
    @"%Origin": type,
    @"%": Record(struct { length: P32, span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct {
    before: Opt(Span(@"%Origin")),
    end: Span(@"%Origin"),
}) {
    return record(@"%".span.splitBeforeEndLengthPositive(@"%".length));
}
pub fn opt_span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        direction: @"|down|up"(void, void),
        span: Opt(Span(@"%Origin")),
        state: @"%State",
        step: Fn(Record(struct { slot: Slot(@"%Origin"), state: @"%State" }), @"%State"),
    }),
) error{OutOfMemory}!@"%State" {
    return switch (@"%".span) {
        .no => @"%".state,
        .yes => |@"%span"| @"%span".fold(@"%allocator", @"%".direction, @"%".state, @"%".step),
    };
}
pub fn span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        direction: @"|down|up"(void, void),
        span: Span(@"%Origin"),
        state: @"%State",
        step: Fn(Record(struct { slot: Slot(@"%Origin"), state: @"%State" }), @"%State"),
    }),
) error{OutOfMemory}!@"%State" {
    return @"%".span.fold(@"%allocator", @"%".direction, @"%".state, @"%".step);
}
pub fn span_origin_erase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    span: Span(Origin(@"%Origin", @"%Part")),
    eraser: Origin_eraser(@"%Origin", @"%Part"),
})) error{OutOfMemory}!Record(struct {
    span: Span(Origin(Erased, @"%Part")),
    eraser: Origin_eraser(@"%Origin", @"%Part"),
}) {
    return .{
        .span = .{ .start = .{ .index = @"%".span.start.index }, .length = @"%".span.length },
        .eraser = @"%".eraser,
    };
}
pub fn opt_span_origin_erase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    span: Opt(Span(Origin(@"%Origin", @"%Part"))),
    eraser: Origin_eraser(@"%Origin", @"%Part"),
})) error{OutOfMemory}!Record(struct {
    span: Opt(Span(Origin(Erased, @"%Part"))),
    eraser: Origin_eraser(@"%Origin", @"%Part"),
}) {
    return .{
        .eraser = @"%".eraser,
        .span = switch (@"%".span) {
            .absent => .{ .absent = {} },
            .present => |@"%span"| .{ .present = .{
                .start = .{ .index = @"%span".start.index },
                .length = @"%span".length,
            } },
        },
    };
}
pub fn span_origin_unerase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    span: Span(Origin(Erased, @"%Part")),
    uneraser: Origin_uneraser(@"%Origin", @"%Part"),
})) error{OutOfMemory}!Record(struct {
    span: Span(Origin(@"%Origin", @"%Part")),
    uneraser: Origin_uneraser(@"%Origin", @"%Part"),
}) {
    return .{
        .span = .{ .start = .{ .index = @"%".span.start.index }, .length = @"%".span.length },
        .uneraser = @"%".uneraser,
    };
}
pub fn opt_span_origin_unerase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    span: Opt(Span(Origin(Erased, @"%Part"))),
    uneraser: Origin_uneraser(@"%Origin", @"%Part"),
})) error{OutOfMemory}!Record(struct {
    span: Opt(Span(Origin(@"%Origin", @"%Part"))),
    uneraser: Origin_uneraser(@"%Origin", @"%Part"),
}) {
    return .{
        .uneraser = @"%".uneraser,
        .span = switch (@"%".span) {
            .absent => .{ .absent = {} },
            .present => |@"%span"| .{ .present = .{
                .start = .{ .index = @"%span".start.index },
                .length = @"%span".length,
            } },
        },
    };
}

pub fn unset_span_rid(@"%Origin": type, _: Unset_span(@"%Origin")) error{OutOfMemory}!void {}
pub fn unset_span_length(
    @"%Origin": type,
    @"%span": Unset_span(@"%Origin"),
) error{OutOfMemory}!Record(struct { length: P32, span: Unset_span(@"%Origin") }) {
    return .{ .span = @"%span", .length = @"%span".length };
}
pub fn opt_unset_span_length(
    @"%Origin": type,
    @"%opt_span": Opt(Unset_span(@"%Origin")),
) error{OutOfMemory}!Record(struct { length: U32, span: Opt(Unset_span(@"%Origin")) }) {
    return .{
        .span = @"%opt_span",
        .length = switch (@"%opt_span") {
            .no => 0,
            .yes => |@"%span"| @"%span".length.positive,
        },
    };
}
pub fn unset_span_start(@"%Origin": type, @"%span": Unset_span(@"%Origin")) error{OutOfMemory}!Record(struct {
    after: Opt(Unset_span(@"%Origin")),
    start: Unset_slot(@"%Origin"),
}) {
    return record(try @"%span".splitStart());
}
pub fn unset_span_end(@"%Origin": type, @"%span": Unset_span(@"%Origin")) error{OutOfMemory}!Record(struct {
    before: Opt(Unset_span(@"%Origin")),
    end: Unset_slot(@"%Origin"),
}) {
    return record(try @"%span".splitEnd());
}
pub fn unset_span_start_of_length_positive(
    @"%Origin": type,
    @"%": Record(struct { length: P32, span: Unset_span(@"%Origin") }),
) error{OutOfMemory}!Record(struct {
    after: Opt(Unset_span(@"%Origin")),
    start: Unset_span(@"%Origin"),
}) {
    return @"%".span.splitAfterLengthPositive(@"%".length);
}
pub fn unset_span_end_of_length_positive(
    @"%Origin": type,
    @"%": Record(struct { length: P32, span: Unset_span(@"%Origin") }),
) error{OutOfMemory}!Record(struct {
    before: Opt(Unset_span(@"%Origin")),
    end: Unset_span(@"%Origin"),
}) {
    return @"%".span.splitBeforeEndLengthPositive(@"%".length);
}
pub fn opt_unset_span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        direction: @"|down|up"(void, void),
        span: Opt(Unset_span(@"%Origin")),
        state: @"%State",
        step: Fn(Record(struct { slot: Unset_slot(@"%Origin"), state: @"%State" }), @"%State"),
    }),
) error{OutOfMemory}!@"%State" {
    return switch (@"%".span) {
        .no => @"%".state,
        .yes => |@"%span"| @"%span".fold(@"%allocator", @"%".direction, @"%".state, @"%".step),
    };
}
pub fn unset_span_fold(
    @"%Origin": type,
    @"%State": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        direction: @"|down|up"(void, void),
        span: Unset_span(@"%Origin"),
        state: @"%State",
        step: Fn(Record(struct { slot: Unset_slot(@"%Origin"), state: @"%State" }), @"%State"),
    }),
) error{OutOfMemory}!@"%State" {
    return @"%".span.fold(@"%allocator", @"%".direction, @"%".state, @"%".step);
}

pub fn array_rid(
    @"%Element": type,
    @"%Record": type,
    _: Array(@"%Element", @"%Record"),
) error{OutOfMemory}!void {}

pub fn buf_empty(
    @"%Element": type,
    @"%Origin": type,
    @"%Part": type,
    _: Origin(@"%Origin", @"%Part"),
) error{OutOfMemory}!Buf(Origin(@"%Origin", @"%Part"), @"%Element") {
    return .{
        .elements = std.ArrayList(@"%Element").empty,
        .vacant = std.ArrayList(Unset_span(Origin(@"%Origin", @"%Part"))).empty,
    };
}
pub fn buf_reuse(
    @"%Element": type,
    @"%Origin": type,
    @"%Part": type,
    @"%": Record(struct { origin: Origin(@"%Origin", @"%Part"), slice: Unset_slice(@"%Element") }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Element") {
    var elements = std.ArrayList(@"%Element").fromOwnedSlice(@"%".slice.undefined_items);
    elements.clearRetainingCapacity();
    return .{
        .elements = elements,
        .vacant = std.ArrayList(Unset_span(@"%Origin")).empty,
    };
}
pub fn buf_pre_allocate_at_least(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Element", @"%Origin"), length: u32 }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Element") {
    var @"%buf" = @"%".buf;
    try @"%buf".preAllocateAtLeast(@"%allocator", @"%".length);
    return @"%buf";
}
pub fn buf_pre_allocation_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Buf(@"%Element", @"%Origin"),
) error{OutOfMemory}!Buf(@"%Origin", @"%Element") {
    var @"%buf" = @"%".buf;
    try @"%buf".preAllocationRid(@"%allocator");
    return @"%buf";
}
pub fn buf_insert(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), new: @"%Element" }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Slot(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%slot" = try @"%buf".insert(@"%allocator", @"%".new);
    return .{ .buf = @"%buf", .slot = @"%slot" };
}
pub fn buf_add(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), new: @"%Element" }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Slot(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%slot" = try @"%buf".add(@"%allocator", @"%".new);
    return .{ .buf = @"%buf", .slot = @"%slot" };
}
pub fn buf_insert_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%buf": Buf(@"%Origin", @"%Element"),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Unset_slot(@"%Origin") }) {
    var @"%buf_ptr" = &@"%buf";
    const @"%slot" = try @"%buf_ptr".insertUnset(@"%allocator");
    return .{ .buf = @"%buf_ptr", .slot = @"%slot" };
}
pub fn buf_add_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%buf": Buf(@"%Origin", @"%Element"),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Unset_slot(@"%Origin") }) {
    var @"%buf_ptr" = &@"%buf";
    const @"%slot" = try @"%buf_ptr".addUnset(@"%allocator");
    return .{ .buf = @"%buf_ptr", .slot = @"%slot" };
}
pub fn buf_add_unset_length(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), length: U32 }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Unset_span(@"%Origin")) }) {
    var @"%buf" = @"%".buf;
    const @"%unset_span" = try @"%buf".addUnsetLength(@"%allocator", @"%".length);
    return .{ .buf = @"%buf", .span = @"%unset_span" };
}
pub fn buf_add_unset_length_positive(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), length: P32 }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Unset_span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%unset_span" = try @"%buf".addUnsetLengthPositive(@"%allocator", @"%".length);
    return .{ .buf = @"%buf", .span = @"%unset_span" };
}
pub fn buf_add_array(
    @"%Element": type,
    @"%Origin": type,
    @"%Record": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        new: Array(@"%Element", @"%Record"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%new_span" = try @"%buf".addArray(@"%Record", @"%allocator", @"%".new);
    return .{
        .span = @"%new_span",
        .buf = @"%buf",
    };
}
pub fn buf_remove(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Slot(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), element: @"%Element" }) {
    var @"%buf" = @"%".buf;
    const @"%element" = try @"%buf".remove(@"%allocator", @"%".slot);
    return .{ .buf = @"%buf", .element = @"%element" };
}
pub fn buf_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Slot(@"%Origin") }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Element"),
    element: @"%Element",
    slot: Unset_slot(@"%Origin"),
}) {
    const @"%element" = @"%".buf.unset(@"%allocator", @"%".slot);
    return .{ .buf = @"%".buf, .element = @"%element".element, .slot = @"%element".slot };
}
pub fn buf_set(@"%Element": type, @"%Origin": type, @"%": Record(struct {
    buf: Buf(@"%Origin", @"%Element"),
    new: @"%Element",
    slot: Unset_slot(@"%Origin"),
})) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Slot(@"%Origin") }) {
    const @"%slot" = @"%".buf.set(@"%".slot, @"%".new);
    return .{ .buf = @"%".buf, .slot = @"%slot" };
}
pub fn buf_opt_span_add(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        new: @"%Element",
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAdd(@"%allocator", @"%".span, @"%".new);
    return .{ .span = @"%combined_span", .buf = @"%buf" };
}
pub fn buf_span_add(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        new: @"%Element",
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAdd(@"%allocator", @"%".span, @"%".new);
    return .{ .span = @"%combined_span", .buf = @"%buf" };
}
pub fn buf_char_opt_span_add_str(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: Str, span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAddIterator(
        @"%allocator",
        @"%".span,
        @"%".new.utf8.iterator(),
        std.unicode.Utf8Iterator.nextCodepoint,
    );
    return .{ .span = @"%combined_span".yes, .buf = @"%buf" };
}
pub fn buf_char_span_add_str(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: Str, span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddIterator(
        @"%allocator",
        @"%".span,
        @"%".new.utf8.iterator(),
        std.unicode.Utf8Iterator.nextCodepoint,
    );
    return .{ .span = @"%combined_span", .buf = @"%buf" };
}
// is there a more correct way?
const u32_max_print_len = std.fmt.count("{}", .{std.math.maxInt(U32)});
pub fn buf_char_span_add_u32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: U32, span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buffer": [u32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    return buf_char_span_add_str(@"%Origin", @"%allocator", .{
        .buf = @"%".buf,
        .span = @"%".span,
        .new = Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    });
}
pub fn buf_char_opt_span_add_u32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: U32, span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buffer": [u32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    const @"%combined" = try buf_char_opt_span_add_str(@"%Origin", @"%allocator", .{
        .buf = @"%".buf,
        .span = @"%".span,
        .new = Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    });
    return .{ .buf = @"%combined".buf, .span = @"%combined".span };
}
// is there a more correct way?
const i32_max_print_len = @max(
    std.fmt.count("{}", .{std.math.minInt(I32)}),
    std.fmt.count("{}", .{std.math.maxInt(I32)}),
);
pub fn buf_char_span_add_i32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: I32, span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buffer": [i32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    return buf_char_span_add_str(@"%Origin", @"%allocator", .{
        .buf = @"%".buf,
        .span = @"%".span,
        .new = Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    });
}
pub fn buf_char_opt_span_add_i32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: I32, span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buffer": [i32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%".new, 10, std.fmt.Case.lower, .{});
    const @"%combined" = try buf_char_opt_span_add_str(Char, @"%Origin", @"%allocator", .{
        .buf = @"%".buf,
        .span = @"%".span,
        .new = Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    });
    return .{ .buf = @"%combined".buf, .span = @"%combined".span.yes };
}
const f32_max_decimal_print_len =
    std.fmt.float.bufferSize(std.fmt.float.Mode.decimal, F32);
pub fn buf_char_span_add_f32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: F32, span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buffer": [f32_max_decimal_print_len]u8 = undefined;
    const @"%used_buffer_slice" = std.fmt.float.render(
        &@"%buffer",
        @"%".new,
        .{ .mode = .decimal, .precision = null },
    ) catch unreachable;
    return buf_char_span_add_str(@"%Origin", @"%allocator", .{
        .buf = @"%".buf,
        .span = @"%".span,
        .new = Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%used_buffer_slice")).?,
    });
}
pub fn buf_char_opt_span_add_f32(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", Char), new: F32, span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", Char), span: Span(@"%Origin") }) {
    var @"%buffer": [f32_max_decimal_print_len]u8 = undefined;
    const @"%used_buffer_slice" = std.fmt.float.render(
        &@"%buffer",
        @"%".new,
        .{ .mode = .decimal, .precision = null },
    ) catch unreachable;
    const @"%combined" = try buf_char_opt_span_add_str(@"%Origin", @"%allocator", .{
        .buf = @"%".buf,
        .span = @"%".span,
        .new = Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%used_buffer_slice")).?,
    });
    return .{ .buf = @"%combined".buf, .span = @"%combined".span.yes };
}
pub fn buf_span_add_array(
    @"%Element": type,
    @"%Origin": type,
    @"%Record": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        new: Array(@"%Element", @"%Record"),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddSlice(@"%allocator", @"%".span, @"%".new);
    return .{
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_array(
    @"%Element": type,
    @"%Origin": type,
    @"%Record": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        new: Array(@"%Element", @"%Record"),
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAddArray(@"%Record", @"%allocator", @"%".span, @"%".new);
    return .{
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_span_add_buf_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        source: Buf(@"%SourceOrigin", @"%Element"),
        source_span: Opt(Span(@"%SourceOrigin")),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Element"),
    source: Buf(@"%SourceOrigin", @"%Element"),
    source_span: Opt(Unset_span(@"%SourceOrigin")),
    span: Span(@"%Origin"),
}) {
    const @"%sourced" = @"%".source.optSpanElements(@"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddSlice(@"%allocator", @"%".span, @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_span_add_buf_span(
    @"%Element": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        source: Buf(@"%SourceOrigin", @"%Element"),
        source_span: Span(@"%SourceOrigin"),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Element"),
    source: Buf(@"%SourceOrigin", @"%Element"),
    source_span: Unset_span(@"%SourceOrigin"),
    span: Span(@"%Origin"),
}) {
    const @"%sourced" = @"%".source.spanElements(@"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddSlice(@"%allocator", @"%".span, @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_buf_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        source: Buf(@"%SourceOrigin", @"%Element"),
        source_span: Opt(Span(@"%SourceOrigin")),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Element"),
    source: Buf(@"%SourceOrigin", @"%Element"),
    source_span: Opt(Unset_span(@"%SourceOrigin")),
    span: Opt(Span(@"%Origin")),
}) {
    const @"%sourced" = @"%".source.spanElements(@"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAddSlice(@"%allocator", @"%".span, @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_buf_span(
    @"%Element": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        source: Buf(@"%SourceOrigin", @"%Element"),
        source_span: Span(@"%SourceOrigin"),
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Element"),
    source: Buf(@"%SourceOrigin", @"%Element"),
    source_span: Unset_span(@"%SourceOrigin"),
    span: Span(@"%Origin"),
}) {
    // is there a better way?
    const @"%sourced" = @"%".source.spanElements(@"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%span_combined_with_start" = try @"%buf".optSpanAdd(@"%allocator", @"%".span, @"%sourced".slice[0]);
    const @"%combined_span" = try @"%buf".optSpanAddSlice(@"%allocator", @"%span_combined_with_start", @"%sourced".slice);
    return .{
        .source = @"%".source,
        .source_span = @"%sourced".span,
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Span(@"%Origin"),
        start: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddOwnSpan(@"%allocator", @"%".start, @"%".end);
    return .{
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Opt(Span(@"%Origin")),
        start: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    switch (@"%".start) {
        .no => return .{ .buf = @"%".buf, .span = @"%".end },
        .yes => |@"%start"| {
            var @"%buf" = @"%".buf;
            const @"%combined_span" = try @"%buf".spanAddOwnSpan(@"%allocator", @"%start", @"%".end);
            return .{
                .span = @"%combined_span",
                .buf = @"%buf",
            };
        },
    }
}
pub fn buf_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Span(@"%Origin"),
        start: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    switch (@"%".end) {
        .no => return .{ .buf = @"%".buf, .span = @"%".start },
        .yes => |@"%end"| {
            var @"%buf" = @"%".buf;
            const @"%combined_span" = try @"%buf".spanAddOwnSpan(@"%allocator", @"%".start, @"%end");
            return .{
                .span = @"%combined_span",
                .buf = @"%buf",
            };
        },
    }
}
pub fn buf_opt_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Span(@"%Origin"),
        start: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    switch (@"%".start) {
        .no => return .{ .buf = @"%".buf, .span = @"%".end },
        .yes => |@"%start"| {
            switch (@"%".end) {
                .no => return .{ .buf = @"%".buf, .span = .{ .yes = @"%start" } },
                .yes => |@"%end"| {
                    var @"%buf" = @"%".buf;
                    const @"%combined_span" = try @"%buf".spanAddOwnSpan(@"%allocator", @"%start", @"%end");
                    return .{
                        .span = .{ .yes = @"%combined_span" },
                        .buf = @"%buf",
                    };
                },
            }
        },
    }
}
pub fn buf_unset_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Unset_span(@"%Origin"),
        start: Unset_span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Unset_span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".unsetSpanAddOwnSpan(@"%allocator", @"%".start, @"%".end);
    return .{
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_unset_span_add_own_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Opt(Unset_span(@"%Origin")),
        start: Unset_span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Unset_span(@"%Origin") }) {
    switch (@"%".start) {
        .no => return .{ .buf = @"%".buf, .span = @"%".end },
        .yes => |@"%start"| {
            const @"%combined_unset_span" = try @"%".buf.unsetSpanAddOwnSpan(@"%allocator", @"%start", @"%".end);
            return .{
                .span = @"%combined_unset_span",
                .buf = @"%".buf,
            };
        },
    }
}
pub fn buf_unset_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Unset_span(@"%Origin"),
        start: Opt(Unset_span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Unset_span(@"%Origin") }) {
    switch (@"%".end) {
        .no => return .{ .buf = @"%".buf, .span = @"%".start },
        .yes => |@"%end"| {
            const @"%combined_unset_span" = try @"%".buf.unsetSpanAddOwnSpan(@"%allocator", @"%".start, @"%end");
            return .{
                .span = @"%combined_unset_span",
                .buf = @"%".buf,
            };
        },
    }
}
pub fn buf_opt_unset_span_add_own_opt_span(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Element"),
        end: Unset_span(@"%Origin"),
        start: Opt(Unset_span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Unset_span(@"%Origin") }) {
    switch (@"%".start) {
        .no => return .{ .buf = @"%".buf, .span = @"%".end },
        .yes => |@"%start"| {
            switch (@"%".end) {
                .no => return .{ .buf = @"%".buf, .span = .{ .yes = @"%start" } },
                .yes => |@"%end"| {
                    const @"%combined_unset_span" = try @"%".buf.unsetSpanAddOwnSpan(@"%allocator", @"%start", @"%end");
                    return .{
                        .span = .{ .yes = @"%combined_unset_span" },
                        .buf = @"%".buf,
                    };
                },
            }
        },
    }
}
pub fn buf_span_move_to_vacant(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    const @"%moved_span" = @"%".buf.spanMoveToVacant(@"%allocator", @"%".span);
    return .{ .buf = @"%".buf, .span = @"%moved_span" };
}
pub fn buf_opt_span_move_to_vacant(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Span(@"%Origin")) }) {
    switch (@"%".span) {
        .no => return .{ .buf = @"%".buf, .span = .{ .no = {} } },
        .yes => |@"%span"| {
            const @"%moved_span" = @"%".buf.spanMoveToVacant(@"%allocator", @"%span");
            return .{ .buf = @"%".buf, .span = .{ .yes = @"%moved_span" } };
        },
    }
}
pub fn buf_span_move_to_end(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    const @"%moved_span" = @"%".buf.spanMoveToEnd(@"%allocator", @"%".span);
    return .{ .buf = @"%".buf, .span = @"%moved_span" };
}
pub fn buf_opt_span_move_to_end(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Span(@"%Origin")) }) {
    switch (@"%".span) {
        .no => return .{ .buf = @"%".buf, .span = .{ .no = {} } },
        .yes => |@"%span"| {
            const @"%moved_span" = @"%".buf.spanMoveToEnd(@"%allocator", @"%span");
            return .{ .buf = @"%".buf, .span = .{ .preent = @"%moved_span" } };
        },
    }
}
pub fn buf_span_reverse(
    @"%Element": type,
    @"%Origin": type,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Span(@"%Origin") }) {
    const @"%reversed_span" = @"%".buf.spanReverse(@"%".span);
    return .{ .buf = @"%".buf, .span = @"%reversed_span" };
}
pub fn buf_opt_span_reverse(
    @"%Element": type,
    @"%Origin": type,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Span(@"%Origin")) }) {
    const @"%reversed_span" = @"%".buf.optSpanReverse(@"%".span);
    return .{ .buf = @"%".buf, .span = @"%reversed_span" };
}
pub fn buf_slot_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), slot: Unset_slot(@"%Origin") }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Element") {
    @"%".buf.slotRid(@"%allocator", @"%".span);
    return @"%".buf;
}
pub fn buf_span_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Unset_span(@"%Origin") }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Element") {
    @"%".buf.spanRid(@"%allocator", @"%".span);
    return @"%".buf;
}
pub fn buf_opt_span_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Element"), span: Opt(Unset_span(@"%Origin")) }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Element") {
    @"%".buf.optSpanRid(@"%allocator", @"%".span);
    return @"%".buf;
}
pub fn buf_to_unset(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%buf": Buf(@"%Origin", @"%Element"),
) error{OutOfMemory}!Unset_slice(@"%Element") {
    @"%buf".intoUnsetSlice(@"%allocator");
}
pub fn buf_rid(
    @"%Element": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%buf": Buf(@"%Origin", @"%Element"),
) error{OutOfMemory}!void {
    @"%buf".rid(@"%allocator");
}
pub fn buf_origin_erase(
    @"%Element": type,
    @"%Origin": type,
    @"%Part": type,
    @"%": Record(struct {
        buf: Buf(Origin(@"%Origin", @"%Part"), @"%Element"),
        eraser: Origin_eraser(@"%Origin", @"%Part"),
    }),
) error{OutOfMemory}!Buf(Origin(Erased, @"%Part"), @"%Element") {
    return .{
        .vacant = std.ArrayList(Unset_span(Origin(Erased, @"%Part"))){
            .capacity = @"%".buf.vacant.capacity,
            .items = @ptrCast(@"%".buf.vacant.items),
        },
        .elements = @"%".buf.elements,
    };
}
pub fn buf_origin_unerase(
    @"%Element": type,
    @"%Origin": type,
    @"%Part": type,
    @"%": Record(struct {
        buf: Buf(Origin(Erased, @"%Part"), @"%Element"),
        uneraser: Origin_uneraser(@"%Origin", @"%Part"),
    }),
) error{OutOfMemory}!Buf(Origin(@"%Origin", @"%Part"), @"%Element") {
    return .{
        .vacant = std.ArrayList(Unset_span(Origin(@"%Origin", @"%Part"))){
            .capacity = @"%".buf.vacant.capacity,
            .items = @ptrCast(@"%".buf.vacant.items),
        },
        .elements = @"%".buf.elements,
    };
}
/// Assumes no Unset_slot or Unset_span still points into the given buf
pub fn buf_origin_erase_with_elements(
    @"%Element": type,
    @"%ElementErased": type,
    @"%Origin": type,
    @"%Part": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(Origin(@"%Origin", @"%Part"), @"%Element"),
        eraser: Origin_eraser(@"%Origin", @"%Part"),
        element_erase: Fn(
            Record(struct {
                element: @"%Element",
                eraser: Origin_eraser(@"%Origin", @"%Part"),
            }),
            Record(struct {
                element: @"%ElementErased",
                eraser: Origin_eraser(@"%Origin", @"%Part"),
            }),
        ),
    }),
) error{OutOfMemory}!Buf(Origin(Erased, @"%Part"), @"%ElementErased") {
    const elements_erased: std.ArrayList(@"%ElementErased") = elements_erased: {
        if (comptime can_reuse: {
            break :can_reuse (@sizeOf(@"%Element") == @sizeOf(@"%ElementErased")) and
                (@alignOf(@"%Element") == @alignOf(@"%ElementErased"));
        }) {
            for (@"%".buf.elements.items) |*element| {
                element.* = @bitCast((try @"%".element_erase(@"%allocator", .{
                    .element = element.*,
                    .eraser = @"%".eraser,
                })).element);
            }
            break :elements_erased .{
                .capacity = @"%".buf.elements.capacity,
                .items = @ptrCast(@"%".buf.elements.items),
            };
        } else {
            var @"%elements_erased" = std.ArrayList(@"%ElementErased").empty;
            try @"%elements_erased".resize(
                @"%allocator",
                @"%".buf.elements.items.len,
            );
            for (@"%".buf.elements.items, 0..) |@"%element", @"%i"| {
                @"%elements_erased".items[@"%i"] = (try @"%".element_erase(@"%allocator", .{
                    .element = @"%element",
                    .eraser = @"%".eraser,
                })).element;
            }
            var @"%elements" = @"%".buf.elements;
            @"%elements".deinit(@"%allocator");
            break :elements_erased @"%elements_erased";
        }
    };
    return .{
        .vacant = std.ArrayList(Unset_span(Origin(Erased, @"%Part"))){
            .capacity = @"%".buf.vacant.capacity,
            .items = @ptrCast(@"%".buf.vacant.items),
        },
        .elements = elements_erased,
    };
}
pub fn buf_origin_unerase_with_elements(
    @"%Element": type,
    @"%ElementErased": type,
    @"%Origin": type,
    @"%Part": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(Origin(Erased, @"%Part"), @"%ElementErased"),
        uneraser: Origin_uneraser(@"%Origin", @"%Part"),
        element_unerase: Fn(
            Record(struct {
                element: @"%ElementErased",
                uneraser: Origin_uneraser(@"%Origin", @"%Part"),
            }),
            Record(struct {
                element: @"%Element",
                uneraser: Origin_uneraser(@"%Origin", @"%Part"),
            }),
        ),
    }),
) error{OutOfMemory}!Buf(Origin(@"%Origin", @"%Part"), @"%Element") {
    const @"%elements_erased": std.ArrayList(@"%Element") = elements_erased: {
        if (comptime can_reuse: {
            break :can_reuse (@sizeOf(@"%Element") == @sizeOf(@"%ElementErased")) and
                (@alignOf(@"%Element") == @alignOf(@"%ElementErased"));
        }) {
            for (@"%".buf.elements.items) |*element| {
                element.* = @bitCast((try @"%".element_unerase(@"%allocator", .{
                    .element = element.*,
                    .uneraser = @"%".uneraser,
                })).element);
            }
            break :elements_erased .{
                .capacity = @"%".buf.elements.capacity,
                .items = @ptrCast(@"%".buf.elements.items),
            };
        } else {
            var @"%elements_erased" = std.ArrayList(@"%Element").empty;
            try @"%elements_erased".resize(
                @"%allocator",
                @"%".buf.elements.items.len,
            );
            for (@"%".buf.elements.items, 0..) |@"%element", @"%i"| {
                @"%elements_erased".items[@"%i"] = (try @"%".element_unerase(@"%allocator", .{
                    .element = @"%element",
                    .uneraser = @"%".uneraser,
                })).element;
            }
            var @"%elements" = @"%".buf.elements;
            @"%elements".deinit(@"%allocator");
            break :elements_erased @"%elements_erased";
        }
    };
    return .{
        .vacant = std.ArrayList(Unset_span(Origin(@"%Origin", @"%Part"))){
            .capacity = @"%".buf.vacant.capacity,
            .items = @ptrCast(@"%".buf.vacant.items),
        },
        .elements = @"%elements_erased",
    };
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
) error{OutOfMemory}!Record(struct { length: U32, span: Unset_slice(@"%Element") }) {
    return .{ .length = @"%unset_slice".length, .slice = @"%unset_slice" };
}
pub fn unset_slice_cast_or_rid_and_allocate(
    @"%Element": type,
    @"%NewElement": type,
    @"%allocator": std.mem.Allocator,
    @"%unset_slice": Unset_slice(@"%Element"),
) error{OutOfMemory}!Unset_slice(@"%NewElement") {
    return @"%unset_slice".castOrRidAndAllocate(@"%NewElement", @"%allocator");
}
pub fn unset_slice_rid(
    @"%Element": type,
    @"%allocator": std.mem.Allocator,
    @"%unset_slice": Unset_slice(@"%Element"),
) error{OutOfMemory}!Unset_slice(@"%Element") {
    return @"%unset_slice".rid(@"%allocator");
}
