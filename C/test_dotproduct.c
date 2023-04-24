// gcc -o test_dotproduct  test_dotproduct.c

#include <stdio.h>
#include <time.h>
#include <stdlib.h>

struct vector {
   double *data;
   int len;
};

int dotProduct(struct vector x, struct vector y,double *dot) {    

    if (x.len != y.len){
        return -1;
    }

    *dot = 0.0;

    for (int i=0;i<x.len;i++){ 
        *dot += x.data[i] * y.data[i];
    }

    return 0;
}




int main()
{
    struct vector x,y;
    double dot;
    const int N=10000000;
    
    srand(time(NULL));
    
    
    x.len=N;
    x.data=(double*)malloc(x.len*sizeof(double));
    for (int i=0;i<x.len;i++)
    	x.data[i]= (double)rand()/(double)RAND_MAX;
    	
    y.len=N;
    y.data=(double*)malloc(y.len*sizeof(double));
    for (int i=0;i<y.len;i++)
    	y.data[i]= (double)rand()/(double)RAND_MAX;
    	
    if(dotProduct(x,y,&dot)<0)
    	printf("Dot product failed\n");
    else
    	printf("Dot product of x and y is: %f\n",dot);
    	
    free(x.data);
    free(y.data);	
    
   
}
