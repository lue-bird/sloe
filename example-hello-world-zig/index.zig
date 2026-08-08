const std = @import("std");
const sloe = @import("sloe.zig");

fn main() !void {
    const greeting = try sloe.greet(sloe.Str.fromComptime("world"));
    std.debug.print("{s}", .{greeting});
}
