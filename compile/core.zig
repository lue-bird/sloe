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
pub fn @"|done|going"(@"%Done": type, @"%Going": type) type {
    return union(enum) { done: @"%Done", going: @"%Going" };
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
    pub fn addAssumeNoOverflow(@"%p": @This(), @"%increase": u32) P32 {
        return .{ .positive = @"%p".positive + @"%increase" };
    }
    pub fn addClamp(@"%p": @This(), @"%increase": u32) P32 {
        return .{ .positive = @"%p".positive +| @"%increase" };
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
        return comptime @"%str": {
            const @"%utf8_view" = std.unicode.Utf8View.initComptime(@"%bytes");
            if (std.math.cast(u32, @"%bytes".len) == null) @compileError(std.fmt.comptimePrint("Str byte length must fit in a u32, is {}", .{@"%bytes".len}));
            break :@"%str" if (Str.fromUtf8View(@"%utf8_view")) |@"%str"| @"%str" else @compileError("Str must contain at least one codepoint");
        };
    }
    pub fn fromUtf8View(@"%utf8_view": std.unicode.Utf8View) ?Str {
        return if (@"%utf8_view".bytes.len >= 1)
            .{ .utf8 = @"%utf8_view" }
        else
            null;
    }
    pub fn utf8_byte_count_p32(@"%str": Str) P32 {
        return P32.fromU32(std.math.cast(u32, @"%str".utf8.bytes.len).?).?;
    }
    pub fn codepoint_count_p32(@"%str": Str) P32 {
        return P32.fromU32(std.math.cast(
            u32,
            std.unicode.utf8CountCodepoints(@"%str".utf8.bytes) catch unreachable,
        ).?).?;
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

fn strideOf(@"%Item": type) comptime_int {
    // at the time of writing, this is the same as
    // @sizeOf(@"%Item")
    // is there a nicer way? And is this always correct in the first place?
    return @sizeOf(@"%Item");
}
fn mathOrderToOrder(@"%order": std.math.Order) Order {
    return switch (@"%order") {
        .lt => .{ .less = {} },
        .eq => .{ .equal = {} },
        .gt => .{ .greater = {} },
    };
}

pub fn Array(@"%Item": type, @"%Record": type) type {
    return [
        switch (@typeInfo(@"%Record")) {
            .@"struct" => |@"%record_type_info"| @max(1, @"%record_type_info".field_names.len),
            else =>
            // no point in throwing a compile error since this cannot happen in sloe-generated code
            // and should not lead to valid (but useless) sloe code not compiling when converted to zig
            1,
        }
    ]@"%Item";
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
pub fn Origin_erased(@"%ValueErased": type) type {
    return struct {
        erased: @"%ValueErased",
        pub fn unerase(@"%Origin": type, @"%origin_erased": @This(), _: Origin(@"%Origin", void)) Origin_isolated(@"%Origin", @"%ValueErased") {
            return .{ .erased = @"%origin_erased".erased };
        }
    };
}
pub fn Origin_isolated(@"%Origin": type, @"%ValueErased": type) type {
    return struct {
        erased: @"%ValueErased",
        pub const origin = @"%Origin";
        pub fn erase(@"%origin_isolated": @This()) Origin_erased(@"%ValueErased") {
            return .{ .erased = @"%origin_isolated".erased };
        }
    };
}
pub fn Origin_uneraser(@"%Origin": type) type {
    return struct {
        pub const origin = @"%Origin";
    };
}
pub fn Slot(@"%Origin": type) type {
    return struct {
        index: u32,
        pub const origin = @"%Origin";
        pub fn toSpan(@"%slot": @This()) Span(
            @"%Origin",
        ) {
            return .{ .start = @"%slot", .length = P32.one };
        }
    };
}
pub fn Span(@"%Origin": type) type {
    return struct {
        // TODO replace with start: u32, const origin = @"%Origin":,
        start: Slot(@"%Origin"),
        length: P32,
        pub fn endIndex(@"%span": @This()) u32 {
            return @"%span".start.index + @"%span".length.predecessor();
        }
        pub fn splitStart(@"%span": @This()) struct {
            after: Opt(Span(@"%Origin")),
            start: Slot(@"%Origin"),
        } {
            return .{
                .start = @"%span".start,
                .after = if (P32.fromU32(@"%span".length.predecessor())) |@"%end_length"|
                    .{ .yes = .{
                        .start = .{ .index = @"%span".start.index + 1 },
                        .length = @"%end_length",
                    } }
                else
                    .{ .no = {} },
            };
        }
        pub fn splitEnd(@"%span": @This()) struct {
            before: Opt(Span(@"%Origin")),
            end: Slot(@"%Origin"),
        } {
            return .{
                .end = .{ .index = @"%span".endIndex() },
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
            after: Opt(Span(@"%Origin")),
            start: Span(@"%Origin"),
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
            before: Opt(Span(@"%Origin")),
            end: Span(@"%Origin"),
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
        pub fn step(
            @"%span": Span(@"%Origin"),
            @"%allocator": std.mem.Allocator,
            @"%direction": @"|down|up"(void, void),
            @"%initial_state": anytype,
            @"%step": Fn(Record(struct { slot: Slot(@"%Origin"), state: @TypeOf(@"%initial_state") }), @TypeOf(@"%initial_state")),
        ) error{OutOfMemory}!@TypeOf(@"%initial_state") {
            var @"%state" = @"%initial_state";
            switch (@"%direction") {
                .up => {
                    for (@"%span".start.index..(@"%span".start.index + @"%span".length.positive)) |@"%index"| {
                        @"%state" = try @"%step"(@"%allocator", .{
                            .state = @"%state",
                            .slot = .{ .index = @intCast(@"%index") },
                        });
                    }
                },
                .down => {
                    // dear zig, add for (range) in reverse
                    var @"%index": u32 = @"%span".start.index + @"%span".length.positive;
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
        pub fn step_while(
            @"%span": Span(@"%Origin"),
            @"%Done": type,
            @"%allocator": std.mem.Allocator,
            @"%direction": @"|down|up"(void, void),
            @"%initial_state": anytype,
            @"%step": Fn(
                Record(struct { slot: Slot(@"%Origin"), state: @TypeOf(@"%initial_state") }),
                @"|done|going"(@"%Done", @TypeOf(@"%initial_state")),
            ),
        ) error{OutOfMemory}!@"|done|going"(
            Record(struct { done: @"%Done", rest: Opt(Span(@"%Origin")) }),
            @TypeOf(@"%initial_state"),
        ) {
            var @"%state" = @"%initial_state";
            switch (@"%direction") {
                .up => {
                    for (@"%span".start.index..(@"%span".start.index + @"%span".length.positive)) |@"%index_usize"| {
                        const @"%index": u32 = @intCast(@"%index_usize");
                        switch (try @"%step"(@"%allocator", .{
                            .state = @"%state",
                            .slot = .{ .index = @"%index" },
                        })) {
                            .going => |@"%new_state"| {
                                @"%state" = @"%new_state";
                            },
                            .done => |@"%done"| {
                                return .{ .done = .{
                                    .done = @"%done",
                                    .rest = if (P32.fromU32(
                                        @"%span".start.index + @"%span".length.positive - 1 - @"%index",
                                    )) |@"%rest_length"| .{
                                        .yes = .{ .start = .{ .index = @"%index" + 1 }, .length = @"%rest_length" },
                                    } else .{ .no = {} },
                                } };
                            },
                        }
                    }
                },
                .down => {
                    // dear zig, add for (range) in reverse
                    var @"%index": u32 = @"%span".start.index + @"%span".length.positive;
                    while (@"%index" > @"%span".start.index) {
                        @"%index" -= 1;
                        switch (try @"%step"(@"%allocator", .{
                            .state = @"%state",
                            .slot = .{ .index = @"%index" },
                        })) {
                            .going => |@"%going"| {
                                @"%state" = @"%going";
                            },
                            .done => |@"%done"| {
                                return .{ .done = .{
                                    .done = @"%done",
                                    .rest = if (P32.fromU32(@"%index" - @"%span".start.index)) |@"%rest_length"| .{
                                        .yes = .{ .start = @"%span".start, .length = @"%rest_length" },
                                    } else .{ .no = {} },
                                } };
                            },
                        }
                    }
                },
            }
            return .{ .going = @"%state" };
        }
    };
}
/// slice whose actual items are undefined
pub fn Unset_slice(@"%Item": type) type {
    return struct {
        undefined_items: []@"%Item",

        pub fn allocateLength(
            @"%allocator": std.mem.Allocator,
            @"%length": u32,
        ) error{OutOfMemory}!@This() {
            return .{ .undefined_items = try @"%allocator".alloc(@"%Item", @"%length") };
        }
        pub fn capacity(@"%unset_slice": @This()) u32 {
            return @intCast(@"%unset_slice".undefined_items.len);
        }
        /// the given unset slice is invalid after
        pub fn castOrRidAndAllocate(
            @"%unset_slice": @This(),
            @"%NewItem": type,
            @"%allocator": std.mem.Allocator,
        ) error{OutOfMemory}!Unset_slice(@"%NewItem") {
            // alignment must match exactly sadly, required by Allocator.free.
            // SmpAllocator for example uses alignment for size classes.
            // The alternative would be carrying original allocation alignment
            // through all uses (Buf, Unset_slice, future collections)
            // which is a price I'm not willing to pay for a niche feature
            if (strideOf(@"%NewItem") == strideOf(@"%Item") and @alignOf(@"%NewItem") == @alignOf(@"%Item")) {
                return .{ .undefined_items = @as(
                    []@"%NewItem",
                    @ptrCast(@"%unset_slice".undefined_items),
                ) };
            } else {
                @"%unset_slice".rid(@"%allocator");
                return Unset_slice(@"%NewItem").allocateLength(@"%allocator", @"%unset_slice".capacity());
            }
        }
        pub fn rid(@"%unset_slice": @This(), @"%allocator": std.mem.Allocator) void {
            return @"%allocator".free(@"%unset_slice".undefined_items);
        }
    };
}
pub fn Buf_origin_erased(@"%Part": type, @"%Item": type) type {
    return struct { erased: Buf(Origin(Erased, @"%Part"), @"%Item") };
}
const Range = struct {
    start: u32,
    length: P32,
    pub fn splitStart(@"%range": @This()) struct { start: u32, after: ?Range } {
        return .{
            .start = @"%range".start,
            .after = if (P32.fromU32(@"%range".length.predecessor())) |@"%end_length"|
                .{
                    .start = @"%range".start + 1,
                    .length = @"%end_length",
                }
            else
                null,
        };
    }
};
/// Not thread-safe.
pub fn Buf(@"%Origin": type, @"%Item": type) type {
    return struct {
        /// Assumed to have an .items.len that fits into a u32.
        /// .items.capacity has no such constraint.
        /// if you want to directly access .items, be extra aware of
        ///   - considering .unset_masks
        ///   - the ABA problem
        ///     (e.g. an index to an item could point to a wrong, new item instead of invalid memory when its index was unset and re-occupied in between)
        items: std.ArrayList(@"%Item"),
        /// true means unset, false means set. Use .unsetBitSet().
        /// All bits at index >= .items.items.len should be set to false.
        /// we do not store its .bit_length as it is equal to .items.capacity.
        unset_masks: @TypeOf((std.bit_set.Dynamic{}).masks),
        /// invariants:
        /// - .none_unset if set_bit_set() is all false within 0..items.items.len
        /// - _ points to a valid index in items
        /// - _ points to the first false index in .unsetBitSet()
        first_unset_index: UnsetIndexOrNone,
        const origin = @"%Origin";

        pub const UnsetIndexOrNone = enum(u32) { none_unset = std.math.maxInt(u32), _ };

        // Contains the set bits until .items.items.len (not its capacity).
        // modifying the resulting value does not change the given Buf's .unset_masks
        pub fn unsetBitSet(@"%buf": @This()) std.bit_set.Dynamic {
            return std.bit_set.Dynamic{ .masks = @"%buf".unset_masks, .bit_length = @"%buf".items.items.len };
        }
        // modifying the resulting value does not change the given Buf's .unset_masks
        pub fn unsetBitSetUntilCapacity(@"%buf": @This()) std.bit_set.Dynamic {
            return std.bit_set.Dynamic{ .masks = @"%buf".unset_masks, .bit_length = @"%buf".items.capacity };
        }

        pub fn preAllocateAtLeast(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%min_pre_allocated_length": u32,
        ) error{OutOfMemory}!void {
            var @"%unset_bit_set" = @"%buf".unsetBitSetUntilCapacity();
            try @"%buf".items.ensureUnusedCapacity(@"%allocator", @"%min_pre_allocated_length");
            try @"%unset_bit_set".resize(@"%allocator", @"%buf".items.capacity, false);
            @"%buf".unset_masks = @"%unset_bit_set".masks;
        }
        pub fn preAllocationRid(@"%buf": *@This(), @"%allocator": std.mem.Allocator) error{OutOfMemory}!void {
            try @"%buf".items.shrinkToLen(@"%allocator");
            var @"%unset_bit_set" = @"%buf".unsetBitSetUntilCapacity();
            try @"%unset_bit_set".resize(@"%allocator", @"%buf".items.items.len, false);
            @"%buf".unset_masks = @"%unset_bit_set".masks;
        }
        pub fn unsetCount(@"%buf": @This()) u32 {
            return @intCast(@"%buf".unsetBitSet().count());
        }
        pub fn setCount(@"%buf": @This()) usize {
            return @"%buf".items.items.len - @"%buf".unsetCount();
        }
        pub fn add(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_item": @"%Item",
        ) error{OutOfMemory}!Slot(@"%Origin") {
            var @"%unset_bit_set" = @"%buf".unsetBitSetUntilCapacity();
            const @"%new_index": u32 = @intCast(@"%buf".items.items.len);
            const @"%new_item_ptr" = try @"%buf".items.addOne(@"%allocator");
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            @"%new_item_ptr".* = @"%new_item";
            try @"%unset_bit_set".resize(@"%allocator", @"%buf".items.capacity, false);
            @"%buf".unset_masks = @"%unset_bit_set".masks;
            return Slot(@"%Origin"){ .index = @"%new_index" };
        }
        pub fn insert(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_item": @"%Item",
        ) error{OutOfMemory}!Slot(@"%Origin") {
            switch (@"%buf".first_unset_index) {
                .none_unset => {
                    return @"%buf".add(@"%allocator", @"%new_item");
                },
                _ => |@"%first_unset_index_enum"| {
                    const @"%first_unset_index" = @intFromEnum(@"%first_unset_index_enum");
                    @"%buf".items.items[@"%first_unset_index"] = @"%new_item";
                    var @"%unset_bit_set" = @"%buf".unsetBitSet();
                    @"%unset_bit_set".unset(@"%first_unset_index");
                    const @"%set_slot" = Slot(@"%Origin"){ .index = @"%first_unset_index" };
                    @"%buf".first_unset_index = @"%buf".firstUnsetIndexStartSearchFrom(@"%first_unset_index" + 1);
                    return @"%set_slot";
                },
            }
        }
        fn firstUnsetIndexStartSearchFrom(@"%buf": @This(), @"%search_start_index": u32) UnsetIndexOrNone {
            const @"%unset_bit_set" = @"%buf".unsetBitSet();
            // skip first_unset_index bits rounded down to the mask
            const @"%unset_bit_mask_index_to_start_search" = @"%search_start_index" / @bitSizeOf(std.bit_set.Dynamic.MaskInt);
            var @"%unset_bit_set_after_unset_index": std.bit_set.Dynamic = .{
                .masks = @"%unset_bit_set".masks + @"%unset_bit_mask_index_to_start_search",
                .bit_length = @"%buf".items.items.len - (@"%unset_bit_mask_index_to_start_search" * @bitSizeOf(std.bit_set.Dynamic.MaskInt)),
            };
            return if (@"%unset_bit_set_after_unset_index".findFirstSet()) |@"%new_first_unset_index"|
                @enumFromInt(@as(u32, @intCast(@"%new_first_unset_index")))
            else
                .none_unset;
        }
        /// slot is invalid while resulting ptr is live
        pub fn item_ptr(@"%buf": @This(), @"%slot": Slot(@"%Origin")) *@"%Item" {
            return &@"%buf".items.items[@"%slot".index];
        }
        pub fn item(@"%buf": @This(), @"%slot": Slot(@"%Origin")) @"%Item" {
            return @"%buf".items.items[@"%slot".index];
        }
        pub fn remove(
            @"%buf": *@This(),
            @"%slot": Slot(@"%Origin"),
        ) @"%Item" {
            const @"%item" = @"%buf".item(@"%slot");
            @"%buf".unsetSpanRid(.{ .start = @"%slot".index, .length = P32.one });
            return @"%item";
        }
        pub fn spanRid(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%item_rid": Fn(@"%Item", void),
        ) error{OutOfMemory}!void {
            for (@"%buf".spanSlice(@"%span")) |@"%item"| {
                try @"%item_rid"(@"%allocator", @"%item");
            }
            @"%buf".unsetSpanRid(.{
                .start = @"%span".start.index,
                .length = @"%span".length,
            });
        }
        pub fn optSpanRid(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%item_rid": Fn(@"%Item", void),
        ) error{OutOfMemory}!void {
            switch (@"%opt_span") {
                .no => {},
                .yes => |@"%span"| {
                    return @"%buf".spanRid(@"%allocator", @"%span", @"%item_rid");
                },
            }
        }
        // The given span is invalid while the returned slice is live
        pub fn spanSlice(@"%buf": @This(), @"%span": Span(@"%Origin")) []@"%Item" {
            return @"%buf".items.items[@"%span".start.index..][0..@"%span".length.positive];
        }
        // The given span is invalid while the returned slice is live
        pub fn optSpanSlice(@"%buf": @This(), @"%opt_span": Opt(Span(@"%Origin"))) []@"%Item" {
            return switch (@"%opt_span") {
                .no => &.{},
                .yes => |@"%span"| @"%buf".spanSlice(@"%span"),
            };
        }
        /// The returned slice is only valid while buf.items.items is live
        pub fn removeSpan(
            @"%buf": *@This(),
            @"%span": Span(@"%Origin"),
        ) []@"%Item" {
            const @"%slice" = @"%buf".spanSlice(@"%span");
            @"%buf".unsetSpanRid(.{ .start = @"%span".start.index, .length = @"%span".length });
            return @"%slice";
        }
        /// The returned slice is only valid while buf.items.items is live
        pub fn removeOptSpan(
            @"%buf": *@This(),
            @"%opt_span": Opt(Span(@"%Origin")),
        ) []@"%Item" {
            switch (@"%opt_span") {
                .no => return []@"%Item",
                .yes => |@"%span"| {
                    return @"%buf".removeSpan(@"%span");
                },
            }
        }
        fn unsetSpanRid(
            @"%buf": *@This(),
            @"%span_to_unset": Range,
        ) void {
            if (@"%span_to_unset".start + @"%span_to_unset".length.positive < @as(u32, @intCast(@"%buf".items.items.len))) {
                var @"%unset_bit_set" = @"%buf".unsetBitSet();
                @"%unset_bit_set".setRangeValue(
                    .{
                        .start = @"%span_to_unset".start,
                        .end = @"%span_to_unset".start + @"%span_to_unset".length.positive,
                    },
                    true,
                );
                @"%buf".first_unset_index = @enumFromInt(@min(@intFromEnum(@"%buf".first_unset_index), @"%span_to_unset".start));
            } else {
                // span is at the end
                @"%buf".endUnsetSpanRid(@"%span_to_unset".length);
            }
        }
        fn endUnsetSpanRid(
            @"%buf": *@This(),
            @"%length_to_unset": P32,
        ) void {
            var @"%unset_bit_set" = @"%buf".unsetBitSet();
            // can be optimized a bit
            var @"%length_to_keep" = @"%buf".items.items.len - @"%length_to_unset".positive;
            while (@"%length_to_keep" >= 1) {
                @"%length_to_keep" -= 1;
                if (!@"%unset_bit_set".isSet(@"%length_to_keep")) {
                    @"%length_to_keep" += 1;
                    @"%buf".items.shrinkRetainingCapacity(@"%length_to_keep");
                    @"%unset_bit_set".setRangeValue(
                        .{
                            .start = @"%length_to_keep",
                            .end = @"%unset_bit_set".bit_length,
                        },
                        false,
                    );
                    if (@"%buf".first_unset_index == @as(UnsetIndexOrNone, @enumFromInt(@"%length_to_keep"))) {
                        @"%buf".first_unset_index = .none_unset;
                    }
                    return;
                }
            }
            @"%buf".clearRetainingCapacity();
        }
        /// invalidates all Slots and Spans into this Buf
        pub fn clearRetainingCapacity(@"%buf": *@This()) void {
            @"%buf".items.clearRetainingCapacity();
            var @"%unset_bit_set" = @"%buf".unsetBitSet();
            @"%unset_bit_set".unsetAll();
            @"%buf".first_unset_index = .none_unset;
        }
        pub fn spanMoveToEnd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            if (@"%span".start.index + @"%span".length.positive == @as(u32, @intCast(@"%buf".items.items.len))) {
                return @"%span";
            }
            // span is not at the end already
            try @"%buf".preAllocateAtLeast(@"%allocator", @"%span".length.positive);
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            @"%buf".items.appendSliceAssumeCapacity(@"%buf".spanSlice(@"%span"));
            @"%buf".unsetSpanRid(.{
                .start = @"%span".start.index,
                .length = @"%span".length,
            });
            return Span(@"%Origin"){
                .start = .{ .index = @intCast(@"%buf".items.items.len - @"%span".length.positive) },
                .length = @"%span".length,
            };
        }
        pub fn spanMoveToUnset(@"%buf": *@This(), @"%span": Span(@"%Origin")) Span(@"%Origin") {
            if (@"%span".start.index + @"%span".length.positive < @as(u32, @intCast(@"%buf".items.items.len))) {
                return @"%span";
            }
            // span is at the end of items
            if (@"%buf".markLengthPositiveAsSet(@"%span".length)) |@"%earlier_start_to_set_from"| {
                @"%buf".items.replaceRangeAssumeCapacity(
                    @"%earlier_start_to_set_from",
                    @"%span".length.positive,
                    @"%buf".spanSlice(@"%span"),
                );
                @"%buf".unsetSpanRid(.{ .start = @"%span".start.index, .length = @"%span".length });
                return Span(@"%Origin"){
                    .start = .{ .index = @"%earlier_start_to_set_from" },
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
            if (@"%start".start.index + @"%start".length.positive == @"%end".start.index) {
                return Span(@"%Origin"){ .start = @"%start".start, .length = @"%start".length.positive + @"%end".length.positive };
            } else {
                const @"%moved_start" = try @"%buf".spanMoveToEnd(@"%allocator", @"%start");
                _ = try @"%buf".spanMoveToEnd(@"%allocator", @"%end");
                return Span(@"%Origin"){
                    .start = @"%moved_start".start,
                    .length = @"%start".length.positive + @"%end".length.positive,
                };
            }
        }
        fn markLengthPositiveAsSet(@"%buf": *@This(), @"%length_to_set": P32) ?u32 {
            switch (@"%buf".first_unset_index) {
                .none_unset => {
                    return null;
                },
                _ => |@"%first_unset_index_enum"| {
                    const @"%first_unset_index" = @intFromEnum(@"%first_unset_index_enum");
                    // can be optimized
                    const @"%unset_bit_mask_index_to_start_search" = @"%first_unset_index" / @bitSizeOf(std.bit_set.Dynamic.MaskInt);
                    var @"%unset_iterator" = (std.bit_set.Dynamic{
                        .masks = @"%buf".unset_masks + @"%unset_bit_mask_index_to_start_search",
                        .bit_length = @"%buf".items.items.len - (@"%unset_bit_mask_index_to_start_search" * @bitSizeOf(std.bit_set.Dynamic.MaskInt)),
                    }).iterator(.{ .direction = .forward, .kind = .set });
                    _ = @"%unset_iterator".next().?;
                    var @"%unset_end_so_far" = @"%first_unset_index";
                    var @"%unset_length_so_far": u32 = 1;
                    while (@"%unset_iterator".next()) |@"%unset_index_usize"| {
                        const @"%unset_index": u32 = @intCast(@"%unset_index_usize");
                        if (@"%unset_end_so_far" + 1 > @"%unset_index") {
                            @"%unset_length_so_far" = 0;
                        } else {
                            @"%unset_length_so_far" += 1;
                            @"%unset_end_so_far" = @"%unset_index";
                            if (@"%unset_length_so_far" == @"%length_to_set".positive) {
                                const @"%found_start" = @as(u32, @intCast(@"%unset_index")) + 1 - @"%length_to_set".positive;
                                var @"%unset_bit_set" = @"%buf".unsetBitSet();
                                @"%unset_bit_set".setRangeValue(
                                    .{
                                        .start = @"%found_start",
                                        .end = @"%found_start" + @"%length_to_set".positive,
                                    },
                                    false,
                                );
                                if (@"%first_unset_index" == @"%found_start") {
                                    @"%buf".first_unset_index = @"%buf".firstUnsetIndexStartSearchFrom(@"%first_unset_index" + 1);
                                }
                                return @"%found_start";
                            }
                        }
                    }
                    return null;
                },
            }
        }
        // add insertSlice?
        pub fn addSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_items": []const @"%Item",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            if (@"%new_items".len >= 1) {
                try @"%buf".items.appendSlice(@"%allocator", @"%new_items");
                if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
                return .{ .yes = .{
                    .start = .{
                        .index = std.math.cast(
                            u32,
                            @"%buf".items.items.len - @"%new_items".len,
                        ).?,
                    },
                    .length = P32.fromU32(std.math.cast(u32, @"%new_items".len).?).?,
                } };
            } else return .{ .no = {} };
        }
        // add insertIterator?
        pub fn addIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_items": anytype,
            @"%next_item": fn (*@TypeOf(@"%new_items")) ?@"%Item",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            const @"%length_before_add" = std.math.cast(u32, @"%buf".items.items.len).?;
            var @"%new_items_iterator" = @"%new_items";
            while (@"%next_item"(&@"%new_items_iterator")) |@"%new_item"| {
                try @"%buf".items.append(@"%allocator", @"%new_item");
            }
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            return if (P32.fromU32(
                std.math.cast(u32, @"%buf".items.items.len).? - @"%length_before_add",
            )) |@"%new_length"|
                .{ .yes = .{
                    .start = .{
                        .index = @"%length_before_add",
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
            @"%new_items": Array(@"%Item", @"%Record"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%length_before_add" = @"%buf".items.items.len;
            try @"%buf".items.appendSlice(@"%allocator", &@"%new_items");
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            return .{
                .start = .{
                    .index = std.math.cast(u32, @"%length_before_add").?,
                },
                .length = P32.fromU32(
                    std.math.cast(u32, @"%buf".items.items.len - @"%length_before_add").?,
                ).?,
            };
        }
        pub fn optSpanAdd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_item": @"%Item",
        ) error{OutOfMemory}!Span(@"%Origin") {
            return switch (@"%opt_span") {
                .no => (try @"%buf".add(@"%allocator", @"%new_item")).toSpan(),
                .yes => |@"%span"| @"%buf".spanAdd(@"%allocator", @"%span", @"%new_item"),
            };
        }
        pub fn spanAdd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_item": @"%Item",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            try @"%buf".items.append(@"%allocator", @"%new_item");
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            return Span(@"%Origin"){
                .start = @"%moved_span".start,
                .length = @"%moved_span".length.addAssumeNoOverflow(1),
            };
        }
        pub fn optSpanAddSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_items": []const @"%Item",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            return switch (@"%opt_span") {
                .no => @"%buf".addSlice(@"%allocator", @"%new_items"),
                .yes => |@"%span"| .{ .yes = try @"%buf".spanAddSlice(@"%allocator", @"%span", @"%new_items") },
            };
        }
        pub fn spanAddSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_items": []const @"%Item",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            try @"%buf".items.appendSlice(@"%allocator", @"%new_items");
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            return Span(@"%Origin"){
                .start = @"%moved_span".start,
                .length = @"%moved_span".length.addAssumeNoOverflow(
                    std.math.cast(u32, @"%new_items".len).?,
                ),
            };
        }
        pub fn optSpanAddArray(
            @"%buf": *@This(),
            @"%Record": type,
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_items": Array(@"%Item", @"%Record"),
        ) error{OutOfMemory}!Span(@"%Origin") {
            return switch (@"%opt_span") {
                .no => @"%buf".addArray(@"%Record", @"%allocator", @"%new_items"),
                .yes => |@"%span"| try @"%buf".spanAddSlice(@"%allocator", @"%span", &@"%new_items"),
            };
        }
        pub fn optSpanAddIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin")),
            @"%new_items": anytype,
            @"%next_item": fn (*@TypeOf(@"%new_items")) ?@"%Item",
        ) error{OutOfMemory}!Opt(Span(@"%Origin")) {
            return switch (@"%opt_span") {
                .no => @"%buf".addIterator(@"%allocator", @"%new_items", @"%next_item"),
                .yes => |@"%span"| .{ .yes = try @"%buf".spanAddIterator(@"%allocator", @"%span", @"%new_items", @"%next_item") },
            };
        }
        pub fn spanAddIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin"),
            @"%new_items": anytype,
            @"%next_item": fn (*@TypeOf(@"%new_items")) ?@"%Item",
        ) error{OutOfMemory}!Span(@"%Origin") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            const @"%length_before_add" = @"%buf".items.items.len;
            var @"%new_items_iterator" = @"%new_items";
            while (@"%next_item"(&@"%new_items_iterator")) |@"%new_item"| {
                try @"%buf".items.append(@"%allocator", @"%new_item");
            }
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            const @"%new_length" = std.math.cast(u32, @"%buf".items.items.len - @"%length_before_add").?;
            return Span(@"%Origin"){
                .start = @"%moved_span".start,
                .length = @"%moved_span".length.addAssumeNoOverflow(@"%new_length"),
            };
        }
        pub fn spanReverse(@"%buf": @This(), @"%span": Span(@"%Origin")) Span(@"%Origin") {
            std.mem.reverse(@"%Item", @"%buf".spanSlice(@"%span"));
            return @"%span";
        }
        pub fn optSpanReverse(@"%buf": @This(), @"%opt_span": Opt(Span(@"%Origin"))) Opt(Span(@"%Origin")) {
            std.mem.reverse(@"%Item", @"%buf".optSpanSlice(@"%opt_span"));
            return @"%opt_span";
        }
        /// buf is invalid after
        pub fn intoUnsetSlice(
            @"%buf": @This(),
            @"%allocator": std.mem.Allocator,
        ) Unset_slice(@"%Item") {
            var @"%unset_bit_set" = @"%buf".unsetBitSetUntilCapacity();
            @"%unset_bit_set".deinit(@"%allocator");
            var @"%items" = @"%buf".items;
            @"%items".clearRetainingCapacity();
            return .{ .undefined_items = @"%items".allocatedSlice() };
        }
        /// buf is invalid after
        pub fn rid(@"%buf": @This(), @"%allocator": std.mem.Allocator) void {
            var @"%buf_mut" = @"%buf";
            @"%buf_mut".items.deinit(@"%allocator");
            var @"%unset_bit_set" = @"%buf_mut".unsetBitSetUntilCapacity();
            @"%unset_bit_set".deinit(@"%allocator");
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

pub fn p32_rid(_: P32) void {}
pub fn p32_dup(@"%n": P32) Record(struct { a: P32, b: P32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn p32_to_u32(@"%n": P32) U32 {
    return @"%n".positive;
}
pub fn p32_add_clamp(@"%": Record(struct { p: P32, u: U32 })) P32 {
    return @"%".p.addClamp(@"%".u);
}
pub fn p32_mul_clamp(@"%": Record(struct { a: P32, b: P32 })) P32 {
    return @"%".a.mulClamp(@"%".b);
}
pub fn p32_order(@"%": Record(struct { left: P32, right: P32 })) Order {
    return mathOrderToOrder(std.math.order(@"%".left.positive, @"%".right.positive));
}
pub fn p32_origin_isolate(
    @"%Origin": type,
    @"%n": P32,
) Origin_isolated(@"%Origin", P32) {
    return .{ .erased = @"%n" };
}

pub fn u32_rid(_: U32) void {}
pub fn u32_dup(@"%n": U32) Record(struct { a: U32, b: U32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn u32_to_i32_clamp(@"%n": U32) I32 {
    return std.math.lossyCast(i32, @"%n");
}
pub fn u32_round_to_nearest_f32_else_even(@"%n": U32) F32 {
    // see i32_round_to_nearest_f32_else_even for details
    return @floatFromInt(@"%n");
}
pub fn u32_successor_clamp(@"%n": U32) P32 {
    return .{ .positive = @"%n" +| 1 };
}
pub fn u32_add_clamp(@"%": Record(struct { a: U32, b: U32 })) U32 {
    return @"%".a +| @"%".b;
}
pub fn u32_add_i32_clamp(@"%": Record(struct { i: I32, u: U32 })) U32 {
    return std.math.lossyCast(u32, @as(i33, @"%".u) +| @as(i33, @"%".i));
}
pub fn u32_mul_clamp(@"%": Record(struct { a: U32, b: U32 })) U32 {
    return @"%".a *| @"%".b;
}
pub fn u32_pow_clamp(@"%": Record(struct { base: U32, exponent: P32 })) U32 {
    return std.math.powi(u32, @"%".base, @"%".exponent.positive) catch std.math.maxInt(u32);
}
pub fn u32_to_p32(@"%n": U32) Opt(P32) {
    return if (P32.fromU32(@"%n")) |@"%p32"| .{ .yes = @"%p32" } else .{ .no = {} };
}
pub fn u32_order(@"%": Record(struct { left: U32, right: U32 })) Order {
    return mathOrderToOrder(std.math.order(@"%".left, @"%".right));
}
pub fn u32_origin_isolate(
    @"%Origin": type,
    @"%n": U32,
) Origin_isolated(@"%Origin", U32) {
    return .{ .erased = @"%n" };
}

pub fn i32_rid(_: I32) void {}
pub fn i32_dup(@"%n": I32) Record(struct { a: I32, b: I32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn i32_to_u32(@"%i": I32) Opt(U32) {
    return if (std.math.cast(U32, @"%i")) |@"%u"| .{ .yes = @"%u" } else .{ .no = {} };
}
pub fn i32_round_to_nearest_f32_else_even(@"%n": I32) F32 {
    // - custom backend: explicit round ties to even
    //   https://codeberg.org/ziglang/zig/src/branch/master/lib/compiler_rt/float_from_int.zig#L508-L509
    // - in llvm, this compiles to sitofp which uses the default rounding mode
    //   @setRoundMode is set to strict by default which claims IEEE compliance
    //   IEEE specifies round ties to even (see §4.3.3, §7.4 in IEEE 754-2008)
    return @floatFromInt(@"%n");
}
pub fn i32_add_clamp(@"%": Record(struct { a: I32, b: I32 })) I32 {
    return @"%".a +| @"%".b;
}
pub fn i32_mul_clamp(@"%": Record(struct { a: I32, b: I32 })) I32 {
    return @"%".a *| @"%".b;
}
pub fn i32_pow_clamp(@"%": Record(struct { base: I32, exponent: P32 })) I32 {
    return std.math.powi(i32, @"%".base, std.math.lossyCast(i32, @"%".exponent.positive)) catch std.math.maxInt(i32);
}
pub fn i32_negate_clamp(@"%n": I32) I32 {
    return 0 -| @"%n";
}
pub fn i32_abs_to_u32(@"%n": I32) U32 {
    return @abs(@"%n");
}
pub fn i32_order(@"%": Record(struct { left: I32, right: I32 })) Order {
    return mathOrderToOrder(std.math.order(@"%".left, @"%".right));
}
pub fn i32_origin_isolate(
    @"%Origin": type,
    @"%n": I32,
) Origin_isolated(@"%Origin", I32) {
    return .{ .erased = @"%n" };
}

pub fn f32_rid(_: F32) void {}
pub fn f32_dup(@"%n": F32) Record(struct { a: F32, b: F32 }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn f32_pi(_: void) F32 {
    return std.math.pi;
}
pub fn f32_negate(@"%n": F32) F32 {
    return -@"%n";
}
pub fn f32_abs(@"%n": F32) F32 {
    return @abs(@"%n");
}
pub fn f32_ln(@"%n": F32) Opt(F32) {
    if (@"%n" <= 0) {
        return .{ .no = {} };
    } else {
        const @"%ln_result" = @log(@"%n");
        return if (std.math.isFinite(@"%ln_result")) .{ .yes = @"%ln_result" } else .{ .no = {} };
    }
}
pub fn f32_exp(@"%n": F32) F32 {
    return @min(@exp(@"%n"), std.math.floatMax(f32));
}
pub fn f32_sin(@"%n": F32) F32 {
    return @sin(@"%n");
}
pub fn f32_cos(@"%n": F32) F32 {
    return @cos(@"%n");
}
pub fn f32_tan(@"%n": F32) F32 {
    return @tan(@"%n");
}
pub fn f32_atan(@"%n": F32) F32 {
    return std.math.atan(@"%n");
}
pub fn f32_round_up(@"%n": F32) F32 {
    return @ceil(@"%n");
}
pub fn f32_round_down(@"%n": F32) F32 {
    return @floor(@"%n");
}
pub fn f32_round_toward_0(@"%n": F32) F32 {
    return @trunc(@"%n");
}
pub fn f32_round_away_from_0(@"%n": F32) F32 {
    return @ceil(@abs(@"%n")) * std.math.sign(@"%n");
}
pub fn f32_round_nearest_else_away_from_0(@"%n": F32) F32 {
    return @round(@"%n");
}
pub fn f32_round_nearest_else_even(@"%n": F32) F32 {
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
pub fn f32_round_up_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_up(@"%n"));
}
pub fn f32_round_down_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_down(@"%n"));
}
pub fn f32_round_toward_0_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_toward_0(@"%n"));
}
pub fn f32_round_away_from_0_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_away_from_0(@"%n"));
}
pub fn f32_round_nearest_else_away_from_0_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_nearest_else_away_from_0(@"%n"));
}
pub fn f32_round_nearest_else_even_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_nearest_else_even(@"%n"));
}
pub fn f32_add_clamp(@"%": Record(struct { a: F32, b: F32 })) F32 {
    const @"%sum" = @"%".a + @"%".b;
    return if (std.math.isNegativeInf(@"%sum")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%sum")) std.math.floatMax(f32) else @"%sum";
}
pub fn f32_mul_clamp(@"%": Record(struct { a: F32, b: F32 })) F32 {
    const @"%product" = @"%".a * @"%".b;
    return if (std.math.isNegativeInf(@"%product")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%product")) std.math.floatMax(f32) else @"%product";
}
pub fn f32_div_clamp(@"%": Record(struct { n: F32, by: F32 })) F32 {
    return if (@"%".by == 0.0) 0.0 else {
        const @"%div_result" = @"%".n / @"%".by;
        return if (std.math.isNegativeInf(@"%div_result")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%div_result")) std.math.floatMax(f32) else @"%div_result";
    };
}
pub fn f32_square_root(@"%n": F32) Opt(F32) {
    return if (@"%n" < 0.0) .{ .no = {} } else .{ .yes = @sqrt(@"%n") };
}
pub fn f32_pow_i32(@"%": Record(struct { base: F32, exponent: I32 })) Opt(F32) {
    const @"%power" = std.math.pow(f32, @"%".base, @floatFromInt(@"%".exponent));
    return if (std.math.isFinite(@"%power")) .{ .yes = @"%power" } else .{ .no = {} };
}
pub fn f32_pow(@"%": Record(struct { base: F32, exponent: F32 })) Opt(F32) {
    const @"%power" = std.math.pow(f32, @"%".base, @"%".exponent);
    return if (std.math.isFinite(@"%power")) .{ .yes = @"%power" } else .{ .no = {} };
}
pub fn f32_order(@"%": Record(struct { left: F32, right: F32 })) Order {
    return mathOrderToOrder(std.math.order(@"%".left, @"%".right));
}
pub fn f32_origin_isolate(
    @"%Origin": type,
    @"%n": F32,
) Origin_isolated(@"%Origin", F32) {
    return .{ .erased = @"%n" };
}

pub fn char_rid(_: Char) void {}
pub fn char_to_u32(@"%char": Char) U32 {
    return @"%char";
}
pub fn char_dup(@"%n": Char) Record(struct { a: Char, b: Char }) {
    return .{ .a = @"%n", .b = @"%n" };
}
pub fn char_origin_isolate(
    @"%Origin": type,
    @"%char": Char,
) Origin_isolated(@"%Origin", Char) {
    return .{ .erased = @"%char" };
}

pub fn str_rid(_: Str) void {}
pub fn str_dup(@"%str": Str) Record(struct { a: Str, b: Str }) {
    return .{ .a = @"%str", .b = @"%str" };
}
pub fn str_utf8_length(@"%str": Str) P32 {
    return @"%str".utf8_byte_count_p32();
}
pub fn str_char_count(@"%str": Str) P32 {
    return @"%str".codepoint_count_p32();
}
pub fn str_start(@"%str": Str) Record(struct { after: Opt(Str), start: Char }) {
    const @"%split" = @"%str".splitStart();
    return .{
        .start = @"%split".start,
        .after = if (Str.fromUtf8View(@"%split".after)) |@"%after"| .{ .yes = @"%after" } else .{ .no = {} },
    };
}
pub fn str_end(@"%str": Str) Record(struct { before: Opt(Str), end: Char }) {
    const @"%split" = @"%str".splitEnd();
    return .{
        .end = @"%split".end,
        .before = if (Str.fromUtf8View(@"%split".before)) |@"%before"| .{ .yes = @"%before" } else .{ .no = {} },
    };
}
pub fn str_origin_isolate(
    @"%Origin": type,
    @"%str": Str,
) Origin_isolated(@"%Origin", Str) {
    return .{ .erased = @"%str" };
}

pub fn fn_rid(@"%In": type, @"%Out": type, _: Fn(@"%In", @"%Out")) void {}
pub fn fn_dup(
    @"%In": type,
    @"%Out": type,
    @"%function": Fn(@"%In", @"%Out"),
) Record(struct { a: Fn(@"%In", @"%Out"), b: Fn(@"%In", @"%Out") }) {
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
pub fn fn_origin_isolate(
    @"%In": type,
    @"%Origin": type,
    @"%Out": type,
    @"%fn": Fn(@"%In", @"%Out"),
) Origin_isolated(@"%Origin", Fn(@"%In", @"%Out")) {
    return .{ .erased = @"%fn" };
}

pub fn choice_empty_to(
    @"%Result": type,
    @"%impossible": Choice,
) @"%Result" {
    return switch (@"%impossible") {};
}

pub fn opt_yes(@"%Yes": type, @"%yes": @"%Yes") Opt(@"%Yes") {
    return .{ .present = @"%yes" };
}

pub fn origin_rid(@"%Origin": type, @"%Part": type, _: Origin(@"%Origin", @"%Part")) void {}

pub fn origin_isolate_constant(
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%Constant": type,
    @"%constant": Fn(void, @"%Constant"),
) error{OutOfMemory}!Origin_isolated(@"%Origin", @"%Constant") {
    return .{ .erased = try @"%constant"(@"%allocator", {}) };
}
pub fn origin_isolated_merge(
    @"%A": type,
    @"%B": type,
    @"%Origin": type,
    @"%": Record(struct {
        a: Origin_isolated(@"%Origin", @"%A"),
        b: Origin_isolated(@"%Origin", @"%B"),
    }),
) Origin_isolated(@"%Origin", Record(struct { a: @"%A", b: @"%B" })) {
    return .{ .erased = .{ .a = @"%".a.erased, .b = @"%".b.erased } };
}
pub fn origin_isolated_map(
    @"%Erased": type,
    @"%NewErased": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        change: Fn(@"%Erased", @"%NewErased"),
        isolated: Origin_isolated(@"%Origin", @"%Erased"),
    }),
) error{OutOfMemory}!Origin_isolated(@"%Origin", @"%NewErased") {
    return .{ .erased = try @"%".change(@"%allocator", @"%".isolated.erased) };
}
pub fn origin_erase(
    @"%Origin": type,
    @"%ValueErased": type,
    @"%": Origin_isolated(@"%Origin", @"%ValueErased"),
) Origin_erased(@"%ValueErased") {
    return @"%".erase();
}
pub fn origin_erased_rid(
    @"%ValueErased": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        erased: Origin_erased(@"%ValueErased"),
        rid: Fn(@"%ValueErased", void),
    }),
) error{OutOfMemory}!void {
    return @"%".rid(@"%allocator", @"%".erased.erased);
}
pub fn origin_unerase(
    @"%Origin": type,
    @"%Value": type,
    @"%ValueErased": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        erased: Origin_erased(@"%ValueErased"),
        origin: Origin(@"%Origin", void),
        unerase: Fn(
            Record(struct {
                erased: @"%ValueErased",
                uneraser: Origin_uneraser(@"%Origin"),
            }),
            Record(struct {
                unerased: @"%Value",
                uneraser: Origin_uneraser(@"%Origin"),
            }),
        ),
    }),
) error{OutOfMemory}!@"%Value" {
    return (try @"%".unerase(@"%allocator", .{
        .uneraser = .{},
        .erased = @"%".erased.erased,
    })).unerased;
}

pub fn slot_index(
    @"%Origin": type,
    @"%slot": Slot(@"%Origin"),
) Record(struct { index: U32, slot: Slot(@"%Origin") }) {
    return .{ .slot = @"%slot", .index = @"%slot".index };
}
pub fn slot_to_span(@"%Origin": type, @"%slot": Slot(@"%Origin")) Span(@"%Origin") {
    return @"%slot".toSpan();
}
pub fn slot_origin_isolate(
    @"%Origin": type,
    @"%Part": type,
    @"%slot": Slot(Origin(@"%Origin", @"%Part")),
) Origin_isolated(@"%Origin", Slot(Origin(Erased, @"%Part"))) {
    return .{ .erased = .{ .index = @"%slot".index } };
}
pub fn slot_origin_unerase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    slot: Slot(Origin(Erased, @"%Part")),
    uneraser: Origin_uneraser(@"%Origin"),
})) Record(struct {
    slot: Slot(Origin(@"%Origin", @"%Part")),
    uneraser: Origin_uneraser(@"%Origin"),
}) {
    return .{
        .slot = .{ .index = @"%".slot.index },
        .uneraser = @"%".uneraser,
    };
}

pub fn span_length(
    @"%Origin": type,
    @"%span": Span(@"%Origin"),
) Record(struct { length: P32, span: Span(@"%Origin") }) {
    return .{ .span = @"%span", .length = @"%span".length };
}
pub fn opt_span_length(
    @"%Origin": type,
    @"%opt_span": Opt(Span(@"%Origin")),
) Record(struct { length: U32, span: Opt(Span(@"%Origin")) }) {
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
) Record(struct { after: Opt(Span(@"%Origin")), start: Slot(@"%Origin") }) {
    return record(@"%span".splitStart());
}
pub fn span_end(
    @"%Origin": type,
    @"%span": Span(@"%Origin"),
) Record(struct { before: Opt(Span(@"%Origin")), end: Slot(@"%Origin") }) {
    return record(@"%span".splitEnd());
}
pub fn span_start_of_length_positive(
    @"%Origin": type,
    @"%": Record(struct { length: P32, span: Span(@"%Origin") }),
) Record(struct {
    after: Opt(Span(@"%Origin")),
    start: Span(@"%Origin"),
}) {
    return record(@"%".span.splitAfterLengthPositive(@"%".length));
}
pub fn span_end_of_length_positive(
    @"%Origin": type,
    @"%": Record(struct { length: P32, span: Span(@"%Origin") }),
) Record(struct {
    before: Opt(Span(@"%Origin")),
    end: Span(@"%Origin"),
}) {
    return record(@"%".span.splitBeforeEndLengthPositive(@"%".length));
}
pub fn opt_span_step(
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
        .yes => |@"%span"| @"%span".step(@"%allocator", @"%".direction, @"%".state, @"%".step),
    };
}
pub fn span_step(
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
    return @"%".span.step(@"%allocator", @"%".direction, @"%".state, @"%".step);
}
pub fn done(@"%Done": type, @"%Going": type, @"%done": @"%Done") @"|done|going"(@"%Done", @"%Going") {
    return .{ .done = @"%done" };
}
pub fn going(@"%Done": type, @"%Going": type, @"%going": @"%Going") @"|done|going"(@"%Done", @"%Going") {
    return .{ .going = @"%going" };
}
pub fn opt_span_step_while(
    @"%Done": type,
    @"%Going": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        direction: @"|down|up"(void, void),
        span: Opt(Span(@"%Origin")),
        state: @"%Going",
        step: Fn(
            Record(struct { slot: Slot(@"%Origin"), state: @"%Going" }),
            @"|done|going"(@"%Done", @"%Going"),
        ),
    }),
) error{OutOfMemory}!@"|done|going"(
    Record(struct { done: @"%Done", rest: Opt(Span(@"%Origin")) }),
    @"%Going",
) {
    return switch (@"%".span) {
        .no => .{ .going = @"%".state },
        .yes => |@"%span"| @"%span".step_while(@"%Done", @"%allocator", @"%".direction, @"%".state, @"%".step),
    };
}
pub fn span_step_while(
    @"%Done": type,
    @"%Going": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        direction: @"|down|up"(void, void),
        span: Span(@"%Origin"),
        state: @"%Going",
        step: Fn(
            Record(struct { slot: Slot(@"%Origin"), state: @"%Going" }),
            @"|done|going"(@"%Done", @"%Going"),
        ),
    }),
) error{OutOfMemory}!@"|done|going"(
    Record(struct { done: @"%Done", rest: Opt(Span(@"%Origin")) }),
    @"%Going",
) {
    return @"%".span.step_while(@"%Done", @"%allocator", @"%".direction, @"%".state, @"%".step);
}
pub fn span_origin_isolate(
    @"%Origin": type,
    @"%Part": type,
    @"%span": Span(Origin(@"%Origin", @"%Part")),
) Origin_isolated(@"%Origin", Span(Origin(Erased, @"%Part"))) {
    return .{ .erased = .{ .start = .{ .index = @"%span".start.index }, .length = @"%span".length } };
}
pub fn opt_span_origin_isolate(
    @"%Origin": type,
    @"%Part": type,
    @"%opt_span": Opt(Span(Origin(@"%Origin", @"%Part"))),
) Origin_isolated(@"%Origin", Opt(Span(Origin(Erased, @"%Part")))) {
    return .{ .erased = switch (@"%opt_span") {
        .no => .{ .no = {} },
        .yes => |@"%span"| .{ .yes = .{
            .start = .{ .index = @"%span".start.index },
            .length = @"%span".length,
        } },
    } };
}
pub fn span_origin_unerase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    span: Span(Origin(Erased, @"%Part")),
    uneraser: Origin_uneraser(@"%Origin"),
})) Record(struct {
    span: Span(Origin(@"%Origin", @"%Part")),
    uneraser: Origin_uneraser(@"%Origin"),
}) {
    return .{
        .span = .{ .start = .{ .index = @"%".span.start.index }, .length = @"%".span.length },
        .uneraser = @"%".uneraser,
    };
}
pub fn opt_span_origin_unerase(@"%Origin": type, @"%Part": type, @"%": Record(struct {
    span: Opt(Span(Origin(Erased, @"%Part"))),
    uneraser: Origin_uneraser(@"%Origin"),
})) Record(struct {
    span: Opt(Span(Origin(@"%Origin", @"%Part"))),
    uneraser: Origin_uneraser(@"%Origin"),
}) {
    return .{
        .uneraser = @"%".uneraser,
        .span = switch (@"%".span) {
            .no => .{ .no = {} },
            .yes => |@"%span"| .{ .yes = .{
                .start = .{ .index = @"%span".start.index },
                .length = @"%span".length,
            } },
        },
    };
}

pub fn array_rid(
    @"%Item": type,
    @"%Record": type,
    _: Array(@"%Item", @"%Record"),
) void {}

pub fn buf_empty(
    @"%Item": type,
    @"%Origin": type,
    @"%Part": type,
    _: Origin(@"%Origin", @"%Part"),
) Buf(Origin(@"%Origin", @"%Part"), @"%Item") {
    return .{
        .items = std.ArrayList(@"%Item").empty,
        .unset_masks = (std.bit_set.Dynamic{}).masks,
        .first_unset_index = .none_unset,
    };
}
pub fn buf_reuse(
    @"%Item": type,
    @"%Origin": type,
    @"%Part": type,
    @"%": Record(struct { origin: Origin(@"%Origin", @"%Part"), slice: Unset_slice(@"%Item") }),
) Buf(@"%Origin", @"%Item") {
    var items = std.ArrayList(@"%Item").fromOwnedSlice(@"%".slice.undefined_items);
    items.clearRetainingCapacity();
    return .{
        .items = items,
        .unset_masks = (std.bit_set.Dynamic{}).masks,
        .first_unset_index = .none_unset,
    };
}
pub fn buf_pre_allocate_at_least(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Item", @"%Origin"), length: u32 }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Item") {
    var @"%buf" = @"%".buf;
    try @"%buf".preAllocateAtLeast(@"%allocator", @"%".length);
    return @"%buf";
}
pub fn buf_pre_allocation_rid(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Buf(@"%Item", @"%Origin"),
) error{OutOfMemory}!Buf(@"%Origin", @"%Item") {
    var @"%buf" = @"%".buf;
    try @"%buf".preAllocationRid(@"%allocator");
    return @"%buf";
}
pub fn buf_insert(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), new: @"%Item" }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), slot: Slot(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%slot" = try @"%buf".insert(@"%allocator", @"%".new);
    return .{ .buf = @"%buf", .slot = @"%slot" };
}
pub fn buf_add(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), new: @"%Item" }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), slot: Slot(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%slot" = try @"%buf".add(@"%allocator", @"%".new);
    return .{ .buf = @"%buf", .slot = @"%slot" };
}
pub fn buf_add_array(
    @"%Item": type,
    @"%Origin": type,
    @"%Record": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        new: Array(@"%Item", @"%Record"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%new_span" = try @"%buf".addArray(@"%Record", @"%allocator", @"%".new);
    return .{
        .span = @"%new_span",
        .buf = @"%buf",
    };
}
pub fn buf_remove(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), slot: Slot(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), item: @"%Item" }) {
    var @"%buf" = @"%".buf;
    const @"%item" = try @"%buf".remove(@"%allocator", @"%".slot);
    return .{ .buf = @"%buf", .item = @"%item" };
}
pub fn buf_item_step(
    @"%In": type,
    @"%Item": type,
    @"%Origin": type,
    @"%Out": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        slot: Slot(@"%Origin"),
        in: @"%In",
        step: Fn(
            Record(struct { in: @"%In", item: @"%Item" }),
            Record(struct { item: @"%Item", out: @"%Out" }),
        ),
    }),
) error{OutOfMemory}!@"%Out" {
    const @"%item_ptr" = @"%".buf.item_ptr(@"%".slot);
    const @"%stepped" = try @"%".step(@"%allocator", .{ .in = @"%".in, .item = @"%item_ptr".* });
    @"%item_ptr".* = @"%stepped".item;
    return @"%stepped".out;
}
pub fn buf_span_rid(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        item_rid: Fn(@"%Item", void),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Item") {
    var @"%buf" = @"%".buf;
    try @"%buf".spanRid(@"%allocator", @"%".span, @"%".item_rid);
    return @"%buf";
}
pub fn buf_opt_span_rid(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        item_rid: Fn(@"%Item", void),
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Buf(@"%Origin", @"%Item") {
    var @"%buf" = @"%".buf;
    try @"%buf".optSpanRid(@"%allocator", @"%".span, @"%".item_rid);
    return @"%buf";
}
pub fn buf_opt_span_add(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        new: @"%Item",
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAdd(@"%allocator", @"%".span, @"%".new);
    return .{ .span = @"%combined_span", .buf = @"%buf" };
}
pub fn buf_span_add(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        new: @"%Item",
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
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
    @"%Item": type,
    @"%Origin": type,
    @"%Record": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        new: Array(@"%Item", @"%Record"),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddSlice(@"%allocator", @"%".span, @"%".new);
    return .{
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_array(
    @"%Item": type,
    @"%Origin": type,
    @"%Record": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        new: Array(@"%Item", @"%Record"),
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAddArray(@"%Record", @"%allocator", @"%".span, @"%".new);
    return .{
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_span_add_buf_opt_span(
    @"%Item": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        source: Buf(@"%SourceOrigin", @"%Item"),
        source_span: Opt(Span(@"%SourceOrigin")),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Item"),
    source: Buf(@"%SourceOrigin", @"%Item"),
    span: Span(@"%Origin"),
}) {
    var @"%source" = @"%".source;
    const @"%source_slice" = try @"%source".removeOptSpan(@"%allocator", @"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddSlice(@"%allocator", @"%".span, @"%source_slice");
    return .{
        .source = @"%source",
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_span_add_buf_span(
    @"%Item": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        source: Buf(@"%SourceOrigin", @"%Item"),
        source_span: Span(@"%SourceOrigin"),
        span: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Item"),
    source: Buf(@"%SourceOrigin", @"%Item"),
    span: Span(@"%Origin"),
}) {
    var @"%source" = @"%".source;
    const @"%source_slice" = @"%source".removeSpan(@"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddSlice(@"%allocator", @"%".span, @"%source_slice");
    return .{
        .source = @"%source",
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_buf_opt_span(
    @"%Item": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        source: Buf(@"%SourceOrigin", @"%Item"),
        source_span: Opt(Span(@"%SourceOrigin")),
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Item"),
    source: Buf(@"%SourceOrigin", @"%Item"),
    span: Opt(Span(@"%Origin")),
}) {
    var @"%source" = @"%".source;
    const @"%source_slice" = try @"%source".removeOptSpan(@"%allocator", @"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAddSlice(@"%allocator", @"%".span, @"%source_slice");
    return .{
        .source = @"%source",
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_buf_span(
    @"%Item": type,
    @"%Origin": type,
    @"%SourceOrigin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        source: Buf(@"%SourceOrigin", @"%Item"),
        source_span: Span(@"%SourceOrigin"),
        span: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(@"%Origin", @"%Item"),
    source: Buf(@"%SourceOrigin", @"%Item"),
    span: Span(@"%Origin"),
}) {
    var @"%source" = @"%".source;
    const @"%source_slice" = @"%source".removeSpan(@"%".source_span);
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".optSpanAddSlice(@"%allocator", @"%".span, @"%source_slice");
    return .{
        .source = @"%source",
        .span = @"%combined_span".yes,
        .buf = @"%buf",
    };
}
pub fn buf_span_add_own_span(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        end: Span(@"%Origin"),
        start: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    var @"%buf" = @"%".buf;
    const @"%combined_span" = try @"%buf".spanAddOwnSpan(@"%allocator", @"%".start, @"%".end);
    return .{
        .span = @"%combined_span",
        .buf = @"%buf",
    };
}
pub fn buf_opt_span_add_own_span(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        end: Opt(Span(@"%Origin")),
        start: Span(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
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
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        end: Span(@"%Origin"),
        start: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
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
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(@"%Origin", @"%Item"),
        end: Span(@"%Origin"),
        start: Opt(Span(@"%Origin")),
    }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
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
pub fn buf_span_move_to_unset(
    @"%Item": type,
    @"%Origin": type,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }),
) Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    const @"%moved_span" = @"%".buf.spanMoveToUnset(@"%".span);
    return .{ .buf = @"%".buf, .span = @"%moved_span" };
}
pub fn buf_opt_span_move_to_unset(
    @"%Item": type,
    @"%Origin": type,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Opt(Span(@"%Origin")) }),
) Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Opt(Span(@"%Origin")) }) {
    switch (@"%".span) {
        .no => return .{ .buf = @"%".buf, .span = .{ .no = {} } },
        .yes => |@"%span"| {
            const @"%moved_span" = @"%".buf.spanMoveToUnset(@"%span");
            return .{ .buf = @"%".buf, .span = .{ .yes = @"%moved_span" } };
        },
    }
}
pub fn buf_span_move_to_end(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    const @"%moved_span" = @"%".buf.spanMoveToEnd(@"%allocator", @"%".span);
    return .{ .buf = @"%".buf, .span = @"%moved_span" };
}
pub fn buf_opt_span_move_to_end(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Opt(Span(@"%Origin")) }),
) error{OutOfMemory}!Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Opt(Span(@"%Origin")) }) {
    switch (@"%".span) {
        .no => return .{ .buf = @"%".buf, .span = .{ .no = {} } },
        .yes => |@"%span"| {
            const @"%moved_span" = @"%".buf.spanMoveToEnd(@"%allocator", @"%span");
            return .{ .buf = @"%".buf, .span = .{ .preent = @"%moved_span" } };
        },
    }
}
pub fn buf_span_reverse(
    @"%Item": type,
    @"%Origin": type,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }),
) Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Span(@"%Origin") }) {
    const @"%reversed_span" = @"%".buf.spanReverse(@"%".span);
    return .{ .buf = @"%".buf, .span = @"%reversed_span" };
}
pub fn buf_opt_span_reverse(
    @"%Item": type,
    @"%Origin": type,
    @"%": Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Opt(Span(@"%Origin")) }),
) Record(struct { buf: Buf(@"%Origin", @"%Item"), span: Opt(Span(@"%Origin")) }) {
    const @"%reversed_span" = @"%".buf.optSpanReverse(@"%".span);
    return .{ .buf = @"%".buf, .span = @"%reversed_span" };
}
pub fn buf_to_unset(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%buf": Buf(@"%Origin", @"%Item"),
) Unset_slice(@"%Item") {
    return @"%buf".intoUnsetSlice(@"%allocator");
}
pub fn buf_rid(
    @"%Item": type,
    @"%Origin": type,
    @"%allocator": std.mem.Allocator,
    @"%buf": Buf(@"%Origin", @"%Item"),
) void {
    @"%buf".rid(@"%allocator");
}
pub fn buf_origin_isolate(
    @"%Item": type,
    @"%ItemErased": type,
    @"%Origin": type,
    @"%Part": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf(Origin(@"%Origin", @"%Part"), @"%Item"),
        item_isolate: Fn(@"%Item", Origin_isolated(@"%Origin", @"%ItemErased")),
    }),
) error{OutOfMemory}!Origin_isolated(@"%Origin", Buf_origin_erased(@"%Part", @"%ItemErased")) {
    const items_erased: std.ArrayList(@"%ItemErased") = @"%items_erased": {
        if (comptime @"%can_reuse": {
            break :@"%can_reuse" (@sizeOf(@"%Item") == @sizeOf(@"%ItemErased")) and
                (@alignOf(@"%Item") == @alignOf(@"%ItemErased"));
        }) {
            for (@"%".buf.items.items) |*item| {
                item.* = @bitCast((try @"%".item_isolate(@"%allocator", item.*)).erased);
            }
            break :@"%items_erased" .{
                .pointer_stability = @"%".buf.items.pointer_stability,
                .capacity = @"%".buf.items.capacity,
                .items = @ptrCast(@"%".buf.items.items),
            };
        } else {
            var @"%items_erased" = std.ArrayList(@"%ItemErased").empty;
            try @"%items_erased".resize(
                @"%allocator",
                @"%".buf.items.items.len,
            );
            for (@"%".buf.items.items, 0..) |@"%item", @"%i"| {
                @"%items_erased".items[@"%i"] = (try @"%".item_isolate(@"%allocator", @"%item")).erased;
            }
            var @"%items" = @"%".buf.items;
            @"%items".deinit(@"%allocator");
            break :@"%items_erased" @"%items_erased";
        }
    };
    return .{ .erased = .{ .erased = .{
        .unset_masks = @"%".buf.unset_masks,
        .first_unset_index = @"%".buf.first_unset_index,
        .items = items_erased,
    } } };
}
pub fn buf_origin_unerase_keep_items(
    @"%Item": type,
    @"%Origin": type,
    @"%Part": type,
    @"%": Record(struct {
        buf: Buf_origin_erased(@"%Part", @"%Item"),
        uneraser: Origin_uneraser(@"%Origin"),
    }),
) Record(struct {
    buf: Buf(Origin(@"%Origin", @"%Part"), @"%Item"),
    uneraser: Origin_uneraser(@"%Origin"),
}) {
    return .{
        .buf = .{
            .items = @"%".buf.erased.items,
            .unset_masks = @"%".buf.erased.unset_masks,
            .first_unset_index = @"%".buf.erased.first_unset_index,
        },
        .uneraser = @"%".uneraser,
    };
}
pub fn buf_origin_unerase(
    @"%Item": type,
    @"%ItemErased": type,
    @"%Origin": type,
    @"%Part": type,
    @"%allocator": std.mem.Allocator,
    @"%": Record(struct {
        buf: Buf_origin_erased(@"%Part", @"%ItemErased"),
        item_unerase: Fn(
            Record(struct {
                item: @"%ItemErased",
                uneraser: Origin_uneraser(@"%Origin"),
            }),
            Record(struct {
                item: @"%Item",
                uneraser: Origin_uneraser(@"%Origin"),
            }),
        ),
        uneraser: Origin_uneraser(@"%Origin"),
    }),
) error{OutOfMemory}!Record(struct {
    buf: Buf(Origin(@"%Origin", @"%Part"), @"%Item"),
    uneraser: Origin_uneraser(@"%Origin"),
}) {
    const @"%items_erased": std.ArrayList(@"%Item") = @"%items_erased": {
        if (comptime @"%can_reuse": {
            break :@"%can_reuse" (@sizeOf(@"%Item") == @sizeOf(@"%ItemErased")) and
                (@alignOf(@"%Item") == @alignOf(@"%ItemErased"));
        }) {
            for (@"%".buf.erased.items.items) |*item| {
                item.* = @bitCast((try @"%".item_unerase(@"%allocator", .{
                    .item = item.*,
                    .uneraser = @"%".uneraser,
                })).item);
            }
            break :@"%items_erased" .{
                .pointer_stability = @"%".buf.erased.items.pointer_stability,
                .capacity = @"%".buf.erased.items.capacity,
                .items = @ptrCast(@"%".buf.erased.items.items),
            };
        } else {
            var @"%items_erased" = std.ArrayList(@"%Item").empty;
            try @"%items_erased".resize(
                @"%allocator",
                @"%".buf.erased.items.items.len,
            );
            for (@"%".buf.erased.items.items, 0..) |@"%item", @"%i"| {
                @"%items_erased".items[@"%i"] = (try @"%".item_unerase(@"%allocator", .{
                    .item = @"%item",
                    .uneraser = @"%".uneraser,
                })).item;
            }
            var @"%items" = @"%".buf.erased.items;
            @"%items".deinit(@"%allocator");
            break :@"%items_erased" @"%items_erased";
        }
    };
    return .{
        .buf = .{
            .items = @"%items_erased",
            .unset_masks = @"%".buf.erased.unset_masks,
            .first_unset_index = @"%".buf.erased.first_unset_index,
        },
        .uneraser = @"%".uneraser,
    };
}

pub fn unset_slice_allocate_length(
    @"%Item": type,
    @"%allocator": std.mem.Allocator,
    @"%length": U32,
) error{OutOfMemory}!Unset_slice(@"%Item") {
    return Unset_slice(@"%Item").allocateLength(@"%allocator", @"%length");
}
pub fn unset_slice_cast_or_rid_and_allocate(
    @"%Item": type,
    @"%NewItem": type,
    @"%allocator": std.mem.Allocator,
    @"%unset_slice": Unset_slice(@"%Item"),
) error{OutOfMemory}!Unset_slice(@"%NewItem") {
    return @"%unset_slice".castOrRidAndAllocate(@"%NewItem", @"%allocator");
}
pub fn unset_slice_rid(
    @"%Item": type,
    @"%allocator": std.mem.Allocator,
    @"%unset_slice": Unset_slice(@"%Item"),
) Unset_slice(@"%Item") {
    return @"%unset_slice".rid(@"%allocator");
}
