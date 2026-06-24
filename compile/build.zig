const std = @import("std");

pub fn build(b: *std.Build) void {
    const native = b.resolveTargetQuery(.{});
    const exe_mod = b.addModule("index", .{ .root_source_file = b.path("index.zig"), .target = native });

    const test_step = b.step("test", "Run unit tests");
    const unit_tests = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("index.zig"),
            .target = native,
        }),
    });
    const run_unit_tests = b.addRunArtifact(unit_tests);
    test_step.dependOn(&run_unit_tests.step);

    // detected by ZLS and automatically enables Build-On-Save.
    // Note that as of the time of writing ZLS checking was non-functional
    // and waiting for build system protocol to land in 0.17.0
    const check = b.step("check", "Check if compiles");
    const exe_check = b.addExecutable(.{
        .name = "core",
        .root_module = exe_mod,
    });
    check.dependOn(&exe_check.step);
}
