section .data
    align 16                        
    LINE_LENGTH equ 14
    input_file db "input.txt", 0
    error_msg db "Could not open input.txt", 10, 0  ; Added newline for better output

section .bss
    align 16                         
    input resb 1024
    container1 resd 256
    container2 resd 256
    seen resb 1024
    part1_result resd 1
    part2_result resd 1

section .text
    extern _printf, _fopen, _fread, _fclose, _abs, _sort
    global _start

_start:
    ; Set up stack frame with proper alignment
    push ebp
    mov ebp, esp
    and esp, -16                     

    ; Initialize registers
    xor eax, eax                     
    xor ebx, ebx
    xor ecx, ecx
    xor edx, edx

    ; Call main
    call main

    ; Restore stack frame
    mov esp, ebp
    pop ebp

    ; Exit program safely
    mov eax, 1
    xor ebx, ebx                     
    int 0x80

main:
    push ebp
    mov ebp, esp
    sub esp, 16                     

    ; Open input file
    push dword input_file
    call _fopen
    add esp, 4
    test eax, eax
    jz .file_error

    ; Store file handle safely
    mov [ebp-4], eax                 

    ; Read input file
    push dword [ebp-4]               
    push dword 1024                  
    push dword input                 
    push dword [ebp-4]             
    call _fread
    add esp, 16

    ; Close file immediately after use
    push dword [ebp-4]
    call _fclose
    add esp, 4

    ; Clear sensitive data
    mov dword [ebp-4], 0            

    ; Initialize result storage
    mov dword [part1_result], 0
    mov dword [part2_result], 0

    ; Call part1 with aligned stack
    sub esp, 12                      
    push dword input
    call part1
    add esp, 16                      
    mov [part1_result], eax

    ; Call part2 with aligned stack
    sub esp, 12                      
    push dword input
    call part2
    add esp, 16                      
    mov [part2_result], eax

    ; Print results with format string
    push dword [part1_result]
    call _printf
    add esp, 4
    push dword [part2_result]
    call _printf
    add esp, 4

    ; Clean up and return
    mov esp, ebp
    pop ebp
    ret

.file_error:
    ; Handle file error
    push error_msg
    call _printf
    add esp, 4
    
    ; Return with error code
    mov eax, 1                       
    mov esp, ebp
    pop ebp
    ret

parse_input_fast:
    ; Implementation of parse_input_fast
    ; ...
    ret

part1:
    ; Implementation of part1
    ; ...
    ret

part2:
    ; Implementation of part2
    ; ...
    ret

 