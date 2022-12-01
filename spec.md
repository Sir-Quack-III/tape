# Tape

## Brainfuck, but *slightly* more usable

Tape compiles to brainfuck

```
    {0,='A'};
    {1,='B'};
    {2,='C'};
    {3,=0};
    pr!();
```

`{}`:
    - Curly brackets modify the cell stated in the first argument. The second argument is the instruction.
    - Example:
        `{3,=42}`: Modifies the 3rd cell to equal 42