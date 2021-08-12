// https://www.codeproject.com/Articles/15971/Using-Inline-Assembly-in-C-C

#include <stdio.h>

int main() {
	int no = 100, val ;
	asm ("movl %1, %%ebx;"
		 "inc %%ebx;"
		 "inc %%ebx;"
		 "inc %%ebx;"
		 "movl %%ebx, %0;"
		 : "=r" (val)      /* output */
		 : "r"  (no)       /* input */
		 : "%ebx"          /* clobbered register */
	);

	printf("val:%d\n", val);

	return 0 ;
}

