## Enumerations in TypeDown

Enumerations in TypeDown provide a way to create ordered lists with automatically generated numbering and hierarchical structure.

::: info

- Each enumeration item must start on a new line
- Numbering is handled automatically - you must not specify the numbers
  - The actual numbers displayed are generated during rendering
- Use consistent indentation for nested levels (typically 4 spaces)
  - Currently hardcoded at 4
- Individual enumeration items can have an attached label using curly braces:
  - `+ item {label}`

:::


## Basic Enumerations

An enumeration in TypeDown is created using the plus (`+`) character followed by a space and the enumeration item content.

```
+ First item in the enumeration
+ Second item in the enumeration
+ Third item in the enumeration
```

### Rendered Output:

1. First item in the enumeration
2. Second item in the enumeration
3. Third item in the enumeration

## Nested Enumerations

Enumerations can be nested by indenting child items with four spaces or a tab.

```
+ Main item 1
    + Sub-item 1.1
    + Sub-item 1.2
+ Main item 2
    + Sub-item 2.1
        + Sub-sub-item 2.1.1
```

### Rendered Output:

1. Main item 1
   1. Sub-item 1.1
   2. Sub-item 1.2
2. Main item 2
   1. Sub-item 2.1
      1. Sub-sub-item 2.1.1

## Enumeration with Labels

TypeDown allows attaching labels to enumeration items using curly braces.

```
+ Important step {critical}
+ Regular step
+ Final step {completion}
```

Labels may in the future be used for styling, filtering, or identifying specific items in your document.
At the moment they allow you to reference list items in your documentation.
