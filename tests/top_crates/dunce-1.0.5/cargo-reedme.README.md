Filesystem paths in Windows are a total mess. This crate normalizes paths to the most
compatible (but still correct) format, so that you don’t have to worry about the mess.

In Windows the regular/legacy paths (`C:\foo`) are supported by all programs, but have
lots of bizarre restrictions for backwards compatibility with MS-DOS.

And there are Windows NT UNC paths (`\\?\C:\foo`), which are more robust and with fewer
gotchas, but are rarely supported by Windows programs. Even Microsoft’s own!

This crate converts paths to legacy format whenever possible, but leaves UNC paths as-is
when they can’t be unambiguously expressed in a simpler way. This allows legacy programs
to access all paths they can possibly access, and UNC-aware programs to access all paths.

On non-Windows platforms these functions leave paths unmodified, so it’s safe to use them
unconditionally for all platforms.

Parsing is based on <https://msdn.microsoft.com/en-us/library/windows/desktop/aa365247(v=vs.85).aspx>

[Project homepage](https://lib.rs/crates/dunce).