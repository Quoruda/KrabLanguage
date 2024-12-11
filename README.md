# 🦀 Krab

Krab is a minimalist programming language designed as a personal challenge. It is an interpreter written in Rust, inspired by Rust's mascot 🦀. Simplicity, flexibility, and minimalism are the guiding principles!

## Current Features 🛠️

Krab supports the following features:

- **Loops**: Use `while` to execute instructions as long as a condition is true.
- **Conditionals**: Create conditional blocks with `if` and `else`.
- **Dynamic variables**: No typing needed! A single variable can hold an integer, a string, or a float.
- **Simple operations**: Perform basic calculations (`a + b`, but not yet `a + b + c`).
- **Comments**: Add annotations using `#your comment#`.
- **Error handling (internal)**: Errors are managed by the interpreter, but users cannot yet generate or catch errors (`try-catch` or equivalent is planned).
- **Interactive interpreter**: Execute instructions one by one with `krab.sh`.
- **File execution**: Provide a `.kb` file as input to execute its content.

## How to Use Krab 🚀

1. **Interactive execution**:  
   Launch the interactive mode with:
   ```bash
   ./krab.sh
   ```
   Example:
   ```
   >>> i = 0;
   >>> if i > 5 { i = 100 } else { i = 8 };
   ```

   Output:
   ```
   [{"i": Integer(8)}]
   ```

2. **File execution**:  
   Provide a file as an argument:
   ```bash
   ./krab.sh path/to/file.kb
   ```

## Code Examples 🎯

Here are a few examples of what you can do with Krab:

### Variable Declaration
```krab
# Declare a variable #
i = 0;
message = "Hello, Krab!";
```

### Conditionals
```krab
# Using conditionals #
if i > 5 {
    r = "i is greater";
} else {
    r = "i is smaller";
};
```

### Loops
```krab
# A simple loop #
i = 0;
while i < 5 {
    i = i + 1;
};
```

### Nested Loops
```krab
# Nested loops #
i = 0;
while i < 10 {
    j = 0;
    while j < 10 {
        j = j + 1;
    };
    i = i + 1;
};
```

### Comments
```krab
# This is a comment #
# This is another comment 
spanning multiple lines #
```

## Upcoming Features 🚧

- [ ] Support for predefined functions and user-defined functions.
- [ ] Support for complex operation chains like `a + b + c`.
- [ ] `else if` for richer conditional structures.
- [ ] User error handling (`try-catch` or equivalent).

## License 📄

This project is licensed under the MIT License. You are free to use, modify, and distribute this software as long as the original license is included.

For more details, see the [LICENSE](LICENSE.md) file.

