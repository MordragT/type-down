# Heading

The Heading element defines a heading for a section of text.
Heading levels are indicated by the number of consecutive equal signs ('=') at the beginning of the line,
followed by a space.
There are six heading levels (H1-H6), with a single '=' representing H1 and '=====' representing H6.

::: info
Labels can be attached to Headings.
:::

## Example

```
= This is an H1 Heading {label}

== This is an H2 Heading

This is some text content following the H2 Heading.

=== This is an H3 Heading

Another paragraph of text referencing the first heading @label.
```
