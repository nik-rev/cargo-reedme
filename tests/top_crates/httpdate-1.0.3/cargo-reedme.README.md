Date and time utils for HTTP.

Multiple HTTP header fields store timestamps.
For example a response created on May 15, 2015 may contain the header
`Date: Fri, 15 May 2015 15:34:21 GMT`. Since the timestamp does not
contain any timezone or leap second information it is equvivalent to
writing 1431696861 Unix time. Rust’s `SystemTime` is used to store
these timestamps.

This crate provides two public functions:

* `parse_http_date` to parse a HTTP datetime string to a system time
* `fmt_http_date` to format a system time to a IMF-fixdate

In addition it exposes the `HttpDate` type that can be used to parse
and format timestamps. Convert a sytem time to `HttpDate` and vice versa.
The `HttpDate` (8 bytes) is smaller than `SystemTime` (16 bytes) and
using the display impl avoids a temporary allocation.