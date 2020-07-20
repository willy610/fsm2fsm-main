#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "clientandserver_emulator.h"

MSGFACTORY shared_global_message_factory;
MSGFACTORY *glob_mess_fact;


int main(int argc, char** argv)
{
  glob_mess_fact = in_MSGFACTORY(& shared_global_message_factory);
  int seed = 0;
  if (argc > 2)
  { //main --seed value (--seed 610)
    sscanf(argv[2], "%i", &seed);
  }
  fprintf(stderr, "seed=%i\n", seed);
  srand(seed);
  init_server_main(glob_mess_fact);
  init_client_main(glob_mess_fact);

  return    client_main();
}

