use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
extern crate rand;
use std::collections::BTreeSet;

pub fn c_gen_message_h(all_messages: &BTreeSet<String>) -> String {
    ///////////////////////////////////
    let all_messageslengths = || {
        all_messages
            .iter()
            .map(|msg| {
                format!(
                    "{{ {msgid},sizeof ({msgid}_real), \"{msgid}\" }}",
                    msgid = build_msg_id(msg)
                )
            })
            .collect::<Vec<String>>()
            .join(",\n\t")
    };
    let toret = format!(
        "#ifndef MESSAGES_H
#define MESSAGES_H
#define SIZE_SPRINTF_DEST_BUFF 200
#include \"MessageTypes.h\"
#include \"CustomRealMsg.h\"
// MESSAGES_LENGTHS
static struct ONE_MESSANGE_LENGTH {{
	MESSAGETYPES amsg;
	int msg_len;
	const char * msg_name;
}} MESSAGES_LOOKUP[] = {{
  {msglengths}
}};

typedef struct privMSG privMSG;

typedef enum {{
	NO_VALUE, OK, ERR
}} RESULT;

struct privMSG {{
	RESULT result;
	MESSAGETYPES thetype;
	void * a_real_message;
}};

typedef privMSG INMSG; // forward from FSM into _prod
typedef privMSG OUTMSG; //used in passing back from _PROD to FSM

typedef struct {{
	char *errortext;
	OUTMSG msgout;
}} OUTMESSAGE;
#endif /* MESSAGES_H */
",
        msglengths = all_messageslengths()
    );
    return toret;
}
