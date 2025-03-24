# Headings in TypeDown

Headings are essential elements in TypeDown that help organize your document into sections and subsections.
They provide structure and make your content easier to navigate.

## Syntax

In TypeDown, headings are created using equal signs (`=`) at the beginning of a line:

- `= Heading 1` - Creates an H1 heading (highest level)
- `== Heading 2` - Creates an H2 heading
- `=== Heading 3` - Creates an H3 heading
- `==== Heading 4` - Creates an H4 heading
- `===== Heading 5` - Creates an H5 heading
- `====== Heading 6` - Creates an H6 heading

The number of equal signs determines the heading level, with more equal signs creating lower-level headings.

## Labels and References

You can attach labels to headings using curly braces after the heading text:

```
= Introduction {intro}
```

These labels can then be referenced elsewhere in your document using the `@` symbol:

```
See @intro for more information.
```

::: info
Labels make it easy to create cross-references within your document that will update automatically if you reorganize your content.
:::

## Examples

```
= TypeDown Documentation {doc-main}

== Getting Started

This section covers the basics of TypeDown syntax.

=== Headings

As you can see, TypeDown uses = symbols for headings.

== Advanced Features

Please refer to @doc-main for a complete overview.
```
