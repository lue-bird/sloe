const std = @import("std");
const sloe = @import("sloe.zig");

pub fn main(init: std.process.Init) !void {
    const ResultOrigin = enum { origin };
    const result_origin = sloe.record(.{ .origin = ResultOrigin.origin, .part = sloe.record(.{ .origin = {} }) });
    const greeting = try sloe.greet(@TypeOf(result_origin), init.gpa, .{
        .name = sloe.Str.fromComptime("world"),
        .buf = .empty(result_origin),
    });
    var greeting_string_buffer: [32]u8 = undefined;
    var writer = std.Io.File.stdout().writer(
        init.io,
        &greeting_string_buffer,
    );
    for (greeting.buf.spanSlice(greeting.span)) |greeting_char| {
        try writer.interface.print("{u}", .{greeting_char});
    }
    try writer.flush();
    greeting.buf.rid(init.gpa);
}
