# CLI

TypeDown comes with a command-line interface that provides various tools for working with TypeDown (TYD) documents. The general syntax for TypeDown's CLI is:

```
tyd <COMMAND>
```

## Available Commands

TypeDown provides the following commands:

- **check**: Validates a TypeDown document for errors
- **format**: Formats a TypeDown document according to style guidelines
- **compile**: Converts a TypeDown document to another format
- **help**: Displays help information for commands

## Check Command

The check command validates your TypeDown document for syntax errors and other issues:

```
tyd check <PATH>
```

Where `<PATH>` is the path to the TypeDown document you want to validate.

## Format Command

The format command automatically formats your TypeDown document:

```
tyd format <PATH>
```

Where `<PATH>` is the path to the TypeDown document you want to format.

## Compile Command

The compile command converts your TypeDown document to other formats:

```
tyd compile <FORMAT> <INPUT> [OUTPUT]
```

### Arguments:

- `<FORMAT>`: The output format to compile to. Available options:
  - **html**: HTML format (default)
  - **pdf**: PDF format
  - **docx**: Microsoft Word DOCX format
  - **json**: JSON format (Pandoc intermediate representation)

- `<INPUT>`: Path to the input TypeDown document

- `[OUTPUT]`: Optional path for the output file. If not specified, the compiled output will be sent to stdout.

## Global Options

All commands support these options:

- `-h, --help`: Display help information
- `-V, --version`: Display version information (available at the top level)
