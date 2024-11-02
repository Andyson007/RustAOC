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
  // cpy a d
  a = d;
  // cpy 9 c
  c = 9;
b:
  // cpy 282 b
  b = 282;
a:
  // inc d
  d++;
  // dec b
  b--;
  // jnz b -2
  if (b != 0) {
    goto a;
  }
  // dec c
  c--;
  // jnz c -5
  if (c != 0) {
    goto b;
  }
l:
  // cpy d a
  a = d;
k:
  // jnz 0 0
  // cpy a b
  b = a;
  // cpy 0 a
  a = 0;
// cpy 2 c
f:
  c = 2;
// jnz b 2
e:
  if (b != 0) {
    goto c;
  }
  // jnz 1 6
  goto d;
c:
  // dec b
  b--;
  // dec c
  c--;
  // jnz c -4
  goto e;
  // inc a
  a++;
  goto f;
d:
  // cpy 2 b
  b = 2;
// jnz c 2
j:
  if (c != 0) {
    goto h;
  }
  // jnz 1 4
  goto i;
h:
  // dec b
  b--;
  // dec c
  c--;
  // jnz 1 -4
  goto j;
i:
  // jnz 0 0
  // out b
  printf("%d", b);
  // jnz a -19
  if (a != 0) {
    goto k;
  }
  // jnz 1 -21
  goto l;
}
