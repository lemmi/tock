#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <console.h>
#include <timer.h>

char buf[100];

tock_timer_t t;

static void getnstr_cb(int result,
                       int _y __attribute__ ((unused)),
                       int _z __attribute__ ((unused)),
                       void* ud __attribute__ ((unused))) {
  printf("got %c\n", buf[0]);
}

static void timer_cb(int result,
                       int _y __attribute__ ((unused)),
                       int _z __attribute__ ((unused)),
                       void* ud __attribute__ ((unused))) {
  // printf("TIMEOUT\n");
  command(DRIVER_NUM_CONSOLE, 3, 0, 0);
}

int main(void) {

  getnstr_async(buf, 30, getnstr_cb, NULL);

  timer_in(5000, timer_cb, NULL, &t);


  // delay_ms(1000);

  // command(DRIVER_NUM_CONSOLE, 3, 0, 0);


  // // Repeatedly read a character from the console
  // // and print a message to report it.
  // while (1) {
  //   int c = getch();

  //   if (c == TOCK_FAIL) {
  //     printf("\ngetch() failed!\n");
  //   } else {
  //     printf("Got character: '%c'\n", (char) c);
  //   }
  // }
}
