#ifndef MESSAGES_H
#define MESSAGES_H
#define SIZE_SPRINTF_DEST_BUFF 200
#include "MessageTypes.h"
#include "CustomRealMsg.h"
// MESSAGES_LENGTHS
static struct ONE_MESSANGE_LENGTH {
	MESSAGETYPES amsg;
	int msg_len;
	const char * msg_name;
} MESSAGES_LOOKUP[] = {
  { Msg_24,sizeof (Msg_24_real), "Msg_24" },
	{ Msg_42,sizeof (Msg_42_real), "Msg_42" },
	{ Msg_43,sizeof (Msg_43_real), "Msg_43" },
	{ Msg_blablabla,sizeof (Msg_blablabla_real), "Msg_blablabla" },
	{ Msg_bye,sizeof (Msg_bye_real), "Msg_bye" },
	{ Msg_bye_bob,sizeof (Msg_bye_bob_real), "Msg_bye_bob" },
	{ Msg_callin,sizeof (Msg_callin_real), "Msg_callin" },
	{ Msg_dontdisturb,sizeof (Msg_dontdisturb_real), "Msg_dontdisturb" },
	{ Msg_hello_alice,sizeof (Msg_hello_alice_real), "Msg_hello_alice" },
	{ Msg_hello_bob,sizeof (Msg_hello_bob_real), "Msg_hello_bob" },
	{ Msg_hm,sizeof (Msg_hm_real), "Msg_hm" },
	{ Msg_how_are_you,sizeof (Msg_how_are_you_real), "Msg_how_are_you" },
	{ Msg_no,sizeof (Msg_no_real), "Msg_no" },
	{ Msg_no_message_to_send,sizeof (Msg_no_message_to_send_real), "Msg_no_message_to_send" },
	{ Msg_nooutput,sizeof (Msg_nooutput_real), "Msg_nooutput" },
	{ Msg_or,sizeof (Msg_or_real), "Msg_or" },
	{ Msg_other,sizeof (Msg_other_real), "Msg_other" },
	{ Msg_ping,sizeof (Msg_ping_real), "Msg_ping" },
	{ Msg_pong,sizeof (Msg_pong_real), "Msg_pong" },
	{ Msg_questionwas,sizeof (Msg_questionwas_real), "Msg_questionwas" },
	{ Msg_rich,sizeof (Msg_rich_real), "Msg_rich" },
	{ Msg_sorry,sizeof (Msg_sorry_real), "Msg_sorry" },
	{ Msg_tired,sizeof (Msg_tired_real), "Msg_tired" },
	{ Msg_well,sizeof (Msg_well_real), "Msg_well" },
	{ Msg_what,sizeof (Msg_what_real), "Msg_what" },
	{ Msg_yes,sizeof (Msg_yes_real), "Msg_yes" }
};

typedef struct privMSG privMSG;

typedef enum {
	NO_VALUE, OK, ERR
} RESULT;

struct privMSG {
	RESULT result;
	MESSAGETYPES thetype;
	void * a_real_message;
};

typedef privMSG INMSG; // forward from FSM into _prod
typedef privMSG OUTMSG; //used in passing back from _PROD to FSM

typedef struct {
	char *errortext;
	OUTMSG msgout;
} OUTMESSAGE;
#endif /* MESSAGES_H */
