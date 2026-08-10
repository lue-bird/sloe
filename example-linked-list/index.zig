const std = @import("std");
const sloe = @import("sloe.zig");

pub fn main(init: std.process.Init) !void {
    const example_result = try sloe.example(init.arena.allocator(), {});
    std.debug.print("{}\n", .{example_result});
}
