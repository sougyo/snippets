section .data
string db "hello, world", 0x0A
string_length equ $ - string
section .text
global _start

_start:
mov eax, 4
mov ebx, 1
mov ecx, string
mov edx, string_length
int 0x80
mov eax, 1
mov ebx, 0
int 0x80
