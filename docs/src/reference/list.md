# Lists in TypeDown

Lists in TypeDown provide a flexible way to organize and present information in a hierarchical structure.
TypeDown supports bulleted lists with the ability to nest items, attach labels, and create complex structures.

::: info

- Each list item must start on a new line
- Use consistent indentation for nested levels (typically 4 spaces)
  - Currently hardcoded at 4
- Individual list items can have an attached label using curly braces:
  - `- item {label}`

:::

## Basic Lists

A list in TypeDown is created using the dash (`-`) character followed by a space and the list item content.

```
- First item
- Second item
- Third item
```

### Rendered Output:

- First item
- Second item
- Third item

## Nested Lists

Lists can be nested by indenting child items with four spaces or a tab.

```
- Parent item
    - Child item 1
    - Child item 2
        - Grandchild item
    - Child item 3
- Another parent item
```

### Rendered Output:

- Parent item
    - Child item 1
    - Child item 2
        - Grandchild item
    - Child item 3
- Another parent item

## List with Labels

TypeDown allows attaching labels to list items using curly braces.

```
- Important task {critical}
- Regular task
- Optional task {low-priority}
```

Labels may in the future be used for styling, filtering, or identifying specific items in your document.
At the moment they allow you to reference list items in your documentation.
