# PLConcepts
Homework for Programming languages concepts

## Build
```
cargo build
```

## Launch
Assume that you're in folder with executables (by default it will be `target/debug`)
To create binary:
```
./assembly --input-file <asm.txt> --output-file a.out
```
To run binary:
```
./main --input-file a.out
```
To disassembly binary (prints to `stdout`):
```
./disasm --input-file a.out
```
Be careful, there's almost no compile time checks and clear errors.

## Asm description
We have such registers:
- `ax`, `bx`, `cx`, `dx` - general purpose registers
- `sp` - stack pointer
- `ip` - it exists, but user can't access it

Asm instructions:
- `MOV r1, r2` moves content of `r2` to `r1`
- `MOVN r1, num` moves `num` to `r1`
- `PRINT r1` prints
- `CALL label_name` - jmp to label with name `name` and push current `ip` to stack (a.k.a shift `sp`)
- `PUSH r1` - push register onto stack
- `POP r1` - pop stack value to register
- `RET` - `POP` + `JMP`
- `JMP` - replace `ip` content
- `JZE`/`JZNE` - conditional jumps
- `END` - end of program
- `ADD r1, r2`/`ADDN r1, num` <=> `r1 = r1 + r2/num`
- `SUB r1, r2`/`SUBN r1, num` <=> `r1 = r1 - r2/num`


## Examples
`fib_call.asm` - Calculate `n`-th Fibonacci number using calls
`fib_loop.asm` - Calculate `n`-th Fibonacci number using loop
