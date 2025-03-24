# Code in TypeDown

TypeDown supports embedded code scripting capabilities, allowing you to include dynamic elements and logic within your documents.
TypeDown's scripting syntax provides a flexible way to enhance your documents with programmatic functionality.

::: info

- Code statements begin with the hash (`#`) character
- Comments begin with the percent (`%`) character
- Different data types are supported (strings, integers, floats, booleans)
- Function calls use standard syntax: `#functionName(arguments)`
- Variables can be created using `#let` expressions

:::

## Basic Code Syntax

A code statement in TypeDown is created using the hash (`#`) character followed by your code expression.

```
#let greeting = "Hello World"
#print(greeting)
```

### Comments

Comments can be added to your code using the percent (`%`) character. Comments are ignored during execution.

```
#let x = 10  % This is a comment explaining the code
% This entire line is a comment
```

## Data Types

TypeDown scripting supports various data types for use in your code:

```
#let myString = "This is a string"         % String type
#let myInteger = 42                        % Integer type
#let myFloat = 3.14159                     % Float type
#let myBoolean = true                      % Boolean type (true or false)
#let myList = List(1, 2, 3, 4)             % List type
#let myMap = Map(name: "peter", age: 32)   % Map type with `name` of type String and `age` of type Integer
```

## Function Calls

You can call functions using standard syntax:

```
#figure(caption: "My Caption")[ Figure body content ]
#underline("Underline this text")
```

## Variable Assignment

Variables are declared and assigned using the `let` keyword:

```
#let username = "user123"
#let itemCount = 5
#let totalPrice = calculatePrice(itemCount)
```

Multiple variables can be declared like so:

```
#let username = "user123"; itemCount = 5; totalPrice = calculatePrice(itemCount);
```

## Content Blocks

Multiline markup content can be enclosed in square brackets:

```
#renderTemplate([
    This is a multiline
    content block that will be
    processed by the renderTemplate function
])
```
