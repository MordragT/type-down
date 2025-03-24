# Inline Elements in TypeDown

Inline elements in TypeDown allow you to format and enhance text within paragraphs and other block-level structures.
TypeDown supports a variety of inline formatting options including emphasis, strong text, links, and more.

::: info

- Inline elements don't create new blocks but modify existing text
- Most inline formats use symmetric markers (like `* ... *` for strong text)
- Some inline elements can be nested within others
- Special characters can be escaped with backslash `\`

:::

## Basic Formatting

TypeDown provides several options for basic text formatting to emphasize and structure your content.

```
Normal text with *strong emphasis* and /italic text/ and ~strikeout~ content.
```

### Rendered Output:
Normal text with **strong emphasis** and *italic text* and ~~strikeout~~ content.

## Mathematical Notation

Mathematical expressions can be included inline using dollar signs.

```
The formula $E = mc^2$ demonstrates mass-energy equivalence.
```

### Rendered Output:
The formula $E = mc^2$ demonstrates mass-energy equivalence.

## Subscript and Superscript

TypeDown supports both subscript and superscript notation.

```
Water is H_2O and the equation is 2^10 = 1024.
```

### Rendered Output:
Water is H₂O and the equation is 2¹⁰ = 1024.

## Links and References

You can include hyperlinks and reference other parts of your document.

```
Visit <https://example.com> for more information.
See the @introduction section for context.
```

### Rendered Output:
Visit [example.com](https://example.com) for more information.
See the introduction section for context.

## Raw Content and Escaping

Raw inline content and escape sequences allow you to include special characters.

```
Use `code` for technical terms.
Use \* to show an asterisk without formatting.
```

### Rendered Output:
Use `code` for technical terms.
Use * to show an asterisk without formatting.

## Labels and Quotes

Add labels to elements or include quoted text in your document.

```
This is "quoted text" in a sentence.
Important paragraph {key-concept}.
```
