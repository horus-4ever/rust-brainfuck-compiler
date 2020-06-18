pub const _STACKSIZE: usize = 65_536;

pub const _DEFINES: &'static str = "
%define STACK_SIZE 65536
";

pub const _MESSAGES: &'static str = "
askinput_msg: db 'Input a char : '
askinput_msg_len equ $ - askinput_msg

error_msg: db 'An error occured at runtime...', 10
error_msg_len equ $ - error_msg

success_msg: db 10, 'Code successfully executed !', 10
success_msg_len equ $ - success_msg

newline: db 10
newlinelen equ $ - newline
";

pub const _PRINT_FUNCTION: &'static str = "
print:
    pusha           ; save the registers states
    mov edx, edx    ; msg length in edx
    mov ecx, edi    ; msg to write is in edi : copy address to ecx
    mov	ebx,1       ;file descriptor (stdout)
    mov	eax,4       ;system call number (sys_write)
    int	0x80        ;call kernel
    ; end
    popa
    ret
";

pub const _INPUT_FUNCTION: &'static str = "
input:
    pusha

    mov edi, newline
    mov edx, 1
    call print
    
    mov edi, askinput_msg
    mov edx, askinput_msg_len
    call print
    
    mov eax, 3
    mov ebx, 2
    mov ecx, input_byte
    mov edx, 2        ; n + 1 byte to read (cause of newline)
    int 80h
    
    mov edi, newline
    mov edx, 1
    call print
    
    popa
    
    mov al, byte[input_byte]
    mov byte[edi], al
    ret
";

pub const _ON_ERROR: &'static str = "
error:
    mov edi, error_msg
    mov edx, error_msg_len
    call print              ; show an error msg
    jmp exit               ; we must exit : it's an error
";

pub const _START_FUNCTION: &'static str = "
_start:
    call code
    
    mov edi, success_msg
    mov edx, success_msg_len
    call print                  ; print a success msg
exit:
    mov	eax,1       ;system call number (sys_exit)
    int	0x80        ;call kernel

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
    inc edi
    jmp .L1
.L1end:
    mov edi, esp

    mov edx, 1

    ; ==========
    ;   CODE
    ; ==========

";

pub const _CODE_END: &'static str = "
    leave
    ret
";


/*

*/


pub const _CHECK_ESI_INC: &'static str = "
    add edi, {number}
    cmp edi, [ebp-STACK_SIZE]  ; compare with ebp - output_length
    jg error                   ; if ebp > ebp-output_lenght, then print an error
";

pub const _CHECK_ESI_DEC: &'static str = "
    sub edi, {number}
    cmp edi, esp            ; compare with esp
    jl error               ; if edi <= esp, then print an error
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
    call print
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