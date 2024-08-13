const std = @import("std");

const FileError = error {
    NotFound,
    PermissionDenied,
};
