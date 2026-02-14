This Rust crate implements IDNA
[per the WHATWG URL Standard](https://url.spec.whatwg.org/#idna).

It also exposes the underlying algorithms from [*Unicode IDNA Compatibility Processing*
(Unicode Technical Standard #46)](http://www.unicode.org/reports/tr46/)
and [Punycode (RFC 3492)](https://tools.ietf.org/html/rfc3492).

Quoting from [UTS #46’s introduction](http://www.unicode.org/reports/tr46/#Introduction):

> Initially, domain names were restricted to ASCII characters.
> A system was introduced in 2003 for internationalized domain names (IDN).
> This system is called Internationalizing Domain Names for Applications,
> or IDNA2003 for short.
> This mechanism supports IDNs by means of a client software transformation
> into a format known as Punycode.
> A revision of IDNA was approved in 2010 (IDNA2008).
> This revision has a number of incompatibilities with IDNA2003.
>
> The incompatibilities force implementers of client software,
> such as browsers and emailers,
> to face difficult choices during the transition period
> as registries shift from IDNA2003 to IDNA2008.
> This document specifies a mechanism
> that minimizes the impact of this transition for client software,
> allowing client software to access domains that are valid under either system.