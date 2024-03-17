MOVN ax, 5
MOVN bx, 1
MOVN cx, 1

deflabel_start_fib:
JZE ax, label_end_fib;
MOV dx, bx
ADD bx, cx
MOV cx, dx
SUBN ax, 1
JMP label_start_fib;

deflabel_end_fib:

PRINT ax
END
