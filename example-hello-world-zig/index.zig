const std = @import("std");
const sloe = @import("sloe.zig");

pub fn main(init: std.process.Init) !void {
    const ResultOrigin = enum {};
    const result_origin = sloe.Origin(ResultOrigin, void){};
    const greeting = try sloe.greet(@TypeOf(result_origin), init.gpa, .{
        .name = sloe.Str.fromComptime("world"),
        .buf = sloe.buf_empty(sloe.Char, ResultOrigin, void, result_origin),
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
