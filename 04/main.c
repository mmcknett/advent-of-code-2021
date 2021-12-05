#include <stdio.h>

int main() {
  unsigned int x[50];
  unsigned int* y = x;
  while(scanf("%d,", (y++)) > 0);

  for(y = x; *y < 50; ++y) {
    printf("%u ", *y);
  }

  return 0;
}
