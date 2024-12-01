section .data
    LINE_LENGTH equ 14
    input_file db "input.txt", 0
    error_msg db "Could not open input.txt", 0

section .bss
    input resb 1024
    container1 resd 256
    container2 resd 256
    seen resb 1024
    part1_result resq 1
    part2_result resq 1

section .text
    extern printf, fopen, fread, fclose, abs, sort
    global _start

_start:
    ; Set up stack frame
    push rbp
    mov rbp, rsp

    ; Call main
    call main

    ; Exit program
    mov rax, 60         ; sys_exit
    xor rdi, rdi        ; status: 0
    syscall

main:
    ; Open input file
    mov rdi, input_file
    call fopen
    test rax, rax
    jz .file_error

    ; Read input file
    mov rsi, rax
    mov rdi, input
    mov rdx, 1024
    call fread
    call fclose

    ; Call part1
    mov rdi, input
    call part1
    mov [part1_result], rax

    ; Call part2
    mov rdi, input
    call part2
    mov [part2_result], rax

    ; Print results
    mov rdi, part1_result
    call printf
    mov rdi, part2_result
    call printf

    ret

.file_error:
    ; Handle file error
    mov rdi, error_msg
    call printf
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

 