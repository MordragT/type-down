# Raw Blocks in TypeDown

Raw blocks in TypeDown allow you to include preformatted code snippets with optional syntax highlighting.
These blocks preserve all whitespace and formatting, making them ideal for displaying code examples,
configuration files, or any content that should be rendered exactly as written.

::: info

- Raw blocks are enclosed within triple backticks (```)
- You can specify an optional language identifier for syntax highlighting
- The content inside is treated as verbatim text and not processed by TypeDown
- Raw blocks can have an attached label using curly braces:
  - ` ``` {label}`

:::

## Basic Raw Blocks

A raw block in TypeDown is created using triple backticks (```) at the beginning and end of your content.

````
```
This is a basic raw block.
All whitespace    and line breaks
are preserved exactly as written.
```
````

### Rendered Output:

```
This is a basic raw block.
All whitespace    and line breaks
are preserved exactly as written.
```

## Language-Specific Raw Blocks

You can specify a language identifier after the opening triple backticks to enable syntax highlighting.

````
```rust
fn main() {
  println!("Hello, world!");
}
```
````

### Rendered Output:

```rust
fn main() {
  println!("Hello, world!");
}
```

## Raw Blocks with Labels

::: danger

Labels in Raw Blocks are currently not implemented

:::

TypeDown allows attaching labels to raw blocks using curly braces.

````
```json {config-file}
{
  "name": "project",
  "version": "1.0.0"
}
```
````

Labels may be used for referencing specific code blocks in your documentation or for additional styling purposes.
