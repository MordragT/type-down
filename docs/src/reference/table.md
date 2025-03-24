# Tables in TypeDown

Tables in TypeDown provide a structured way to organize and present information in a grid format with rows and columns.
TypeDown supports tables with the ability to include various content types in cells, including text and list items.

::: info

- Tables are created using the pipe character (`|`) to separate columns
- Each row must be on a new line
- All rows in a table must have the same number of cells
- Table cells can contain plain text, list items, or ordered items
- Table rows can have an attached label using curly braces

:::

## Basic Tables

A table in TypeDown is created using pipe characters (`|`) to separate columns.

```
| Header 1 | Header 2 | Header 3 |
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |
```

### Rendered Output:

| Header 1 | Header 2 | Header 3 |
|:--------:|:--------:|:--------:|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |

## Tables with Mixed Content

Tables can contain different types of content in cells, including plain text and list items.

```
| Header 1 | Header 2 | Header 3 |
| - This is a list item | + This is an ordered item | Text content |
| Row 2, Cell 1 | Row 2, Cell 2 | Row 2, Cell 3 |
```

### Rendered Output:

| Header 1 | Header 2 | Header 3 |
|:--------:|:--------:|:--------:|
| - This is a list item | 1. This is an ordered item | Text content |
| Row 2, Cell 1 | Row 2, Cell 2 | Row 2, Cell 3 |

## Table Rows

Each row in a table can contain different types of cell content:

- Plain text
- List items (starting with `-`)
- Ordered items (starting with `+`)

Table rows can also have labels attached using curly braces, which may be used for referencing specific rows in your documentation.
