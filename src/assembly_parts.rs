pub const _STACKSIZE: usize = 65_536;

pub const _DEFINES: &'static str = "
%define STACK_SIZE 65536
";

pub const _MESSAGES: &'static str = "
askinput_msg: db 'Input a char : '
askinput_msg_len equ $ - askinput_msg

error_msg: db 'An error occured at runtime...', 10
error_msg_len equ $ - error_msg

error_pos: db 'Error occured at pos (row, column) : '
error_pos_len equ $ - error_pos

error_overflow_msg: db 'Runtime error : The stack pointer exceeded the stack size => |>|', 10
error_overflow_msg_len equ $ - error_overflow_msg

error_underflow_msg: db 'Runtime error : The stack pointer went under the stack => |<|', 10
error_underflow_msg_len equ $ - error_underflow_msg

success_msg: db 10, 'Code successfully executed !', 10
success_msg_len equ $ - success_msg

newline: db 10
newlinelen equ $ - newline

space: db 32
spacelen equ $ - space

format: db '%.*s'
format_len equ  - format
";

pub const _PRINT_FUNCTION: &'static str = "
extern putchar
extern printf
print:
    pusha
    push ebp
    mov ebp, esp
    push edi
    push edx
    push format
    call printf
    leave
    popa
    ret
";

pub const _PRINT_NUMBER: &'static str = "
print_number:
    pusha
    push ebp
    mov ebp, esp
    test eax, eax
    jnz .L1
    push '0'
.L1:
    test eax, eax
    jz .L2
    xor edx, edx
    mov eax, eax
    mov ecx, 10
    div ecx
    add edx, '0'
    push edx
    jmp .L1
.L2:
    cmp esp, ebp
    je .end
    call putchar
    pop edx
    jmp .L2
.end:
    leave
    popa
    ret
";

pub const _INPUT_FUNCTION: &'static str = "
extern getchar
input:
    pusha

    mov edi, newline
    mov edx, 1
    call print
    
    mov edi, askinput_msg
    mov edx, askinput_msg_len
    call print
    
    call getchar
    mov byte[edi], al
.L1:
    cmp al, 10
    je .L1end
    call getchar
    jmp .L1
.L1end:
    mov edi, newline
    mov edx, 1
    call print
    
    popa
    ret
";

pub const _ON_ERROR: &'static str = "
overflow_error:
    mov edi, error_overflow_msg
    mov edx, error_overflow_msg_len
    jmp error
underflow_error:
    mov edi, error_underflow_msg
    mov edx, error_underflow_msg_len
    jmp error
error:
    call print              ; show an error msg
    mov edi, error_pos
    mov edx, error_pos_len
    call print
    mov eax, ecx
    mov ebx, ecx
    and ebx, 0xFFFF
    shr eax, 16
    call print_number
    mov edi, space
    mov edx, 1
    call print
    mov eax, ebx
    call print_number
    mov edi, newline
    mov edx, 1
    call print
    mov eax, 1      ; return value : error
    leave
    ret
";

pub const _START_FUNCTION: &'static str = "
global main
main:
    call code

    test eax, eax
    jnz exit
    mov edi, success_msg
    mov edx, success_msg_len
    call print                  ; print a success msg
exit:
    ret

code:
    push ebp
    mov ebp, esp
    
    sub esp, STACK_SIZE         ; esp : stack part
    mov edi, esp

    xor eax, eax                ; fill the stack with null bytes
.L1:
    cmp edi, ebp
    jge .L1end
    stosb
    jmp .L1
.L1end:
    mov edi, esp

    mov edx, 1

    ; ==========
    ;   CODE
    ; ==========

";

pub const _CODE_END: &'static str = "
    xor eax, eax
    leave
    ret
";


/*

*/


pub const _CHECK_ESI_INC: &'static str = "
    mov ecx, {position}
    add edi, {number}
    cmp edi, ebp  ; compare with ebp - output_length
    jg overflow_error                   ; if ebp > ebp-output_lenght, then print an error
";

pub const _CHECK_ESI_DEC: &'static str = "
    mov ecx, {position}
    sub edi, {number}
    cmp edi, esp          ; compare with esp
    jl underflow_error               ; if edi <= esp, then print an error
";

pub const _LOOP_BEGIN: &'static str = "
.loop{level}:
    cmp byte[edi], 0
    je .loopend{level}
";

pub const _LOOP_END: &'static str = "
    cmp byte[edi], 0
    jne .loop{loopname}
.loopend{name}:
";

pub const _ON_PRINT: &'static str = "
    xor eax, eax
    mov al, byte[edi]
    push eax
    call putchar
    pop eax
";

pub const _ON_INPUT: &'static str = "
    call input
";

pub const _INC_MEMORY: &'static str = "
    add byte[edi], {number}
";

pub const _DEC_MEMORY: &'static str = "
    sub byte[edi], {number}
";