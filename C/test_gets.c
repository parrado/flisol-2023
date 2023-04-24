// gcc -o test_gets  -fno-stack-protector  test_gets.c

#include <stdio.h>


int main()
{
    char buf[24];
    printf("Please enter your name and press <Enter>\n");
    gets(buf);
    printf("%s", buf);
    return 0;
}
