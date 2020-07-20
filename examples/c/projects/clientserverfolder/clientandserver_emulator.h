//#include "Wire_IO.h"

#ifndef MAIN_EMULATOR_H
#define MAIN_EMULATOR_H
#include "MsgFactory.h"

int init_client_main(MSGFACTORY *glob_mess_fact);
int client_main();
int client_wire_write_and_read_UDP(unsigned char *wire_out, int outlen, unsigned char **wiren_in,size_t *wire_read_len);
int server_read_and_write_UDP(unsigned char *wire_out, int outlen, unsigned char **wiren_in,size_t *wire_read_len);
int init_server_main(MSGFACTORY *glob_mess_fact);

#endif /* MAIN_EMULATOR_H */
