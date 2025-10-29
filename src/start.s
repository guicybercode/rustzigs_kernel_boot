.section .text
.global _start
_start:
    mov %rdi, %rsp
    sub $0x1000, %rsp
    call kernel_main
    hlt
