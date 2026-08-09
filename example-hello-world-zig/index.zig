const std = @import("std");
const sloe = @import("sloe.zig");

pub fn main(init: std.process.Init) !void {
    const ResultOrigin = enum { origin };
    const result_origin = ResultOrigin.origin;
    const greeting = try sloe.greet(ResultOrigin, init.gpa, .{
        .result_origin = result_origin,
        .name = sloe.Str.fromComptime("world"),
    });
    var greeting_string_buffer: [100]u8 = undefined;
    var writer = std.Io.File.stdout().writer(
        init.io,
        &greeting_string_buffer,
    );
    for (greeting.buf.spanSlice(greeting.span)) |greeting_char| {
        try writer.interface.print("{u}", .{greeting_char});
    }
    try writer.flush();
}
