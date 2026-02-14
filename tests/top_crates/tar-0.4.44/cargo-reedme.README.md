A library for reading and writing TAR archives

This library provides utilities necessary to manage [TAR archives][1]
abstracted over a reader or writer. Great strides are taken to ensure that
an archive is never required to be fully resident in memory, and all objects
provide largely a streaming interface to read bytes from.

[1]: http://en.wikipedia.org/wiki/Tar_%28computing%29