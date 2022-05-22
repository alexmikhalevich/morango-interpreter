# Morango interpreter

This project is an interpreter for the toy stack-based language Morango.

## Quick start

1. Build project: `cargo build`
1. Execute enterpreter: `cargo run -- -f <test file>`

You can run tests by executing `cargo test`.

## Supported instructions

- `LOAD_VAL <value>`: pushes `<value>` to the stack;
- `WRITE_VAR <var name>`: pops value from the stack and saves it to the variable `<var name>`;
- `READ_VAR <var name>`: pushes the variable `<var name>` value to the stack;
- `ADD`: pops two values from the stack and pushes their sum;
- `MULTIPLY`: pops two values from the stack and pushes their product;
- `DUP`: pops value from the stack and pushes two same values (duplicates the last value on the stack);
- `POP`: pops value from the stack;
- `TEST_EQ`: pops two values from the stack, pushes `1` if values are equal and `0` otherwise;
- `TEST_GT`: pops two values from the stack, pushes `1` if the first poped value is greater than the second, `0` otherwise;
- `TEST_LT`: pops two values from the stack, pushes `1` if the first poped value is less than the second, `0` otherwise;
- `&<label name>`: declares a label `<label name>`;
- `GOTO &<label name>`: pops value from the stack, if the poped value is `1` - moves the instruction pointer to the label `<label name>`;
- `RETURN_VALUE`: pops value from the stack and exits the program returning the poped value.

## Examples

This repo contains two source files in the `test-sources` subdirectory. These files contain Morango programs that you can use for experiments.

### test.mor

```
x = 1
y = 2
return (x + 1) * y
```

### test2.mor

```
x = 20
y = 0
for i = 0 to 10:
 x += 1
 for j = 1 to 3:
  y += 1
return x * y
```
