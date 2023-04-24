#include <string.h>
#include <stdio.h>

// gcc -o test_strcopy  -fno-stack-protector  test_strcopy.c   

//
 void foo(char *bar)
{
   float My_Float = 10.5; 
   char  c[28];     

  
   printf("My Float value = %f\n", My_Float);


   strcpy(c, bar);

   // Will print 96.031372
   printf("My Float value = %f\n", My_Float);
}

int main(int argc, char **argv)
{
   foo("my string is too long !!!!! \x10\x10\xc0\x42");
   return 0;
}
