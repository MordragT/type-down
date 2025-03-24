# Terms in TypeDown

Terms in TypeDown allow you to create dictionary-like definitions with clear visual separation between the term and its description.
TypeDown supports term lists with a simple, consistent syntax that makes definitions easy to write and read.

## Basic Terms

A term in TypeDown is created using the greater-than symbol (`>`) followed by the term name, a colon, and the description.

```
> Cat: A small domestic mammal with soft fur. Cats are popular pets known for their independence and hunting skills.
> Dog: A domesticated carnivorous mammal that is often kept as a companion animal. Dogs have been bred in many varieties for various purposes such as hunting, herding, and companionship.
```

### Rendered Output:

**Cat**

  A small domestic mammal with soft fur. Cats are popular pets known for their independence and hunting skills.

**Dog**

  A domesticated carnivorous mammal that is often kept as a companion animal. Dogs have been bred in many varieties for various purposes such as hunting, herding, and companionship.

## Term Structure

Each term consists of two main parts:
1. The term name (before the colon)
2. The description (after the colon)

```
> Term name: Term description and explanation.
```

## Term with Labels

TypeDown allows attaching labels to terms using curly braces, similar to other elements.

```
> Semiconductor: A material with electrical conductivity between a conductor and an insulator. {electronics}
> Algorithm: A step-by-step procedure for solving a problem or accomplishing a task. {computing}

...

Modern technology relies heavily on both @electronics and @computing.
Devices like smartphones utilize @electronics components such as semiconductors for their hardware,
while running complex @computing algorithms to process data and provide functionality to users.
```

Labels are used for referencing terms in the remaining document, allowing you to create connections between different parts of your content.
