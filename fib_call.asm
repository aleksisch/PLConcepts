MOVN ax, 3

CALL label_start_fib;
MOV ax, dx
END




deflabel_start_fib:
JZE ax, label_fib_end;
SUBN ax, 1
JZE ax, label_fib_end;

PUSH ax
CALL label_start_fib;
POP ax
PUSH dx

SUBN ax, 1
CALL label_start_fib;

POP cx
ADD dx, cx

RET

deflabel_fib_end:
MOVN dx, 1
RET

