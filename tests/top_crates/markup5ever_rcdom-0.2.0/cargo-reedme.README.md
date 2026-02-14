A simple reference-counted DOM.

This is sufficient as a static parse tree, but don’t build a
web browser using it. :)

A DOM is a [tree structure] with ordered children that can be represented in an XML-like
format. For example, the following graph

```text
div
 +- "text node"
 +- span
```
in HTML would be serialized as

```html
<div>text node<span></span></div>
```

See the [document object model article on wikipedia][dom wiki] for more information.

This implementation stores the information associated with each node once, and then hands out
refs to children. The nodes themselves are reference-counted to avoid copying - you can create
a new ref and then a node will outlive the document. Nodes own their children, but only have
weak references to their parents.

[tree structure]: https://en.wikipedia.org/wiki/Tree_(data_structure)
[dom wiki]: https://en.wikipedia.org/wiki/Document_Object_Model