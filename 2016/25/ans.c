#include <stdio.h>
void simulate(int);

int main() {
  simulate(1);
  return 0;
}

void simulate(int a) {
  int b = 0;
  int c = 0;
  int d = 0;
  a = d;
  c = 9;
  d += c * 282;
  b = 0;
  c = 0;
l:
  a = d;
k:
  b = a;
  a = 0;
e:
  if (b != 0) {
    goto c;
  }
  goto d;
c:
  b--;
  c--;
  goto e;
  a++;
  c = 2;
  goto e;
d:
  b = 2;
j:
  if (c != 0) {
    goto h;
  }
  goto i;
h:
  b--;
  c--;
  goto j;
i:
  printf("%d", b);
  if (a != 0) {
    goto k;
  }
  goto l;
}
