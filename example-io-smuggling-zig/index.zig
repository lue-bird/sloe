const std = @import("std");
const sloe = @import("sloe.zig");

const IoState = struct { io: std.Io, writer: std.Io.File.Writer };

pub fn main(init: std.process.Init) !void {
    var writer_buffer: [128]u8 = undefined;
    const writer = std.Io.File.stdout().writer(
        init.io,
        &writer_buffer,
    );
    _ = try sloe.print_random_quote(IoState, init.arena.allocator(), .{
        .io = IoState{ .io = init.io, .writer = writer },
        .stdout_flush = stdout_flush,
        .stdout_write_char_span = stdout_write_char_span,
        .http_get = http_get,
    });
}
fn stdout_flush(_: std.mem.Allocator, io: IoState) error{OutOfMemory}!IoState {
    var writer = io.writer;
    writer.flush() catch {};
    return .{ .io = io.io, .writer = writer };
}
fn stdout_write_char_span(_: std.mem.Allocator, in: sloe.Record(struct {
    char_span: sloe.Origin_erased(sloe.Opt_char_span_erased),
    io: IoState,
})) error{OutOfMemory}!sloe.Record(struct {
    char_span: sloe.Origin_erased(sloe.Opt_char_span_erased),
    io: IoState,
}) {
    var writer = in.io.writer;
    for (in.char_span.erased.chars.erased.optSpanSlice(in.char_span.erased.span)) |char| {
        writer.interface.print("{u}", .{char}) catch {};
    }
    return .{
        .char_span = in.char_span,
        .io = .{ .io = in.io.io, .writer = writer },
    };
}
fn http_get(allocator: std.mem.Allocator, in: sloe.Record(struct {
    io: IoState,
    url: sloe.Str,
})) error{OutOfMemory}!sloe.Record(struct {
    io: IoState,
    response: sloe.Origin_erased(sloe.Opt_char_span_erased),
}) {
    var chars = sloe.buf_empty(sloe.Char, sloe.Erased, void, sloe.Origin(sloe.Erased, void){});
    if (http_get_request(in.io.io, allocator, in.url.utf8.bytes)) |http_response_body| {
        const span = try chars.addIterator(
            allocator,
            http_response_body.iterator(),
            std.unicode.Utf8Iterator.nextCodepoint,
        );
        return .{
            .io = in.io,
            .response = .{
                .erased = .{ .chars = .{ .erased = chars }, .span = span },
            },
        };
    } else |err| {
        std.debug.print("{}", .{err});
        return .{
            .io = in.io,
            .response = .{
                .erased = .{ .chars = .{ .erased = chars }, .span = .{ .no = {} } },
            },
        };
    }
}
fn http_get_request(io: std.Io, allocator: std.mem.Allocator, url: []const u8) !std.unicode.Utf8View {
    var http_client = std.http.Client{ .io = io, .allocator = allocator };
    var response_writer = std.Io.Writer.Allocating.init(allocator);
    _ = try http_client.fetch(.{
        .location = .{ .url = url },
        .response_writer = &response_writer.writer,
    });
    const response_body = try response_writer.toOwnedSlice();
    http_client.deinit();
    return std.unicode.Utf8View.init(response_body);
}
