use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::RowToOutmsgGroup;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::RowToStateEventGroup;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Fsm {
    pub fn gen_spec(&mut self, mach_name: &String) -> String {
        let grp_state_inmsg: BTreeMap<(String, String), Vec<RowToStateEventGroup>> =
            self.gen_state_event_group();
        let grp_on_outmsg: BTreeMap<String, Vec<RowToOutmsgGroup>> = self.gen_outmsg_group();
        /*******************************************/
        let build_all_state_event = || -> String {
            let mut all_state_event: Vec<String> = Vec::new();
            for ((state, inmsg), rows) in grp_state_inmsg {
                all_state_event.push(format!(
                    "<tr>
<td rowspan=\"{rowspan}\"><b>{state}<br>{inmsg}</b></td>
<td>Always</td>
<td>Check In Message attributes...<br>
Check Business Object attributes...
</td>
<td>(Might) Fail=><br><b>ERR(\"In Pre Guard\",\"{state}_{inmsg}\")</b></td>
</tr>
",
                    state = state,
                    inmsg = inmsg,
                    rowspan = 2 + rows.len()
                ));
                for arow in rows.iter() {
                    all_state_event.push(format!(
                        "<tr>
<td>When</td>
<td><i>Inmessage.xyz > ...<br>
    Businessobject.abc < ... </i>
      </90>
</td>
<td>OK=><br><b>{synt_guard_result}</b></td>
</tr>
",
                        synt_guard_result = arow.synt_guard_result
                    ));
                }

                all_state_event.push(format!(
                    "<tr>
<td>Otherwise</td>
<td></td>
<td>Fail=><br><b>ERR(\"In Post Guard\", \"{state}_{inmsg}\")</b></td>
</tr>
",
                    state = state,
                    inmsg = inmsg,
                ));
            }
            all_state_event.join("\n")
        };
        /*******************************************/
        let build_all_prod = || -> String {
            let mut all_all_prod: Vec<String> = Vec::new();
            for (outmsg, rows) in grp_on_outmsg {
                all_all_prod.push(format!(
                    "<tr>
<td rowspan=\"{rowspan}\">{outmsg}</td>
<td>Always</td>
<td><i>new outmessage<br>
Set content...<br>
Get from inmessage...<br>
Update object...</i>
</td>
<td>Might FAIL<br>
<b>ERR(\"Pre Prod Fail\",\"{outmsg}\")</b></td>
</tr>
        ",
                    rowspan = 2 + rows.len(),
                    outmsg = outmsg
                ));
                for arow in rows.iter() {
                    all_all_prod.push(format!(
"<tr>
<td><B>{synt_guard_result}</B></td>
<td><i>
Set content...<br>
Get from inmessage...<br>
Update object...</i>
</td>
<td>Ok=><br><b>{outmsg}<br>OR<br>Err(\"In Prod Cond\", \"{outmsg}\", \"{synt_guard_result}\")</b></td>
</tr>
        ",
        synt_guard_result=arow.synt_guard_result,
        outmsg=outmsg));
                }
                all_all_prod.push(format!(
                    "<tr>
            <td>Otherwise</td>
            <td><i>Rollback Object updates?</i></td>
            <td>Fail=><b>ERR(\"Post Prod Fail\", \"{outmsg}\")</b></td>
          </tr>
      ",
                    outmsg = outmsg
                ));
            }
            all_all_prod.join("\n")
        };
        /*******************************************/
        let build_all_inmessages = || -> String {
            let mut all_mess: Vec<String> = Vec::new();
            let mut allin: BTreeSet<String> = BTreeSet::new();
            for a_trans in &self.the_normalized_rows {
                allin.insert(a_trans[1].to_lowercase()); // inmess
            }
            all_mess.push(
                "<table><tr><th>InMessage</th><th>Attribute</th><th>Comment</th></tr>".to_string(),
            );
            for x in allin {
                all_mess.push(format!("<tr><td>{x}</td></tr>", x = x));
                all_mess.push("<tr><td></td><td><i>attr1</i></td><td>...</td></tr>".to_string());
            }
            all_mess.push("</table>".to_string());
            all_mess.join("\n")
        };
        /*******************************************/
        let build_all_outmessages = || -> String {
            let mut all_mess: Vec<String> = Vec::new();
            let mut allout: BTreeSet<String> = BTreeSet::new();
            for a_trans in &self.the_normalized_rows {
                allout.insert(a_trans[3].to_lowercase()); // outmess
            }
            all_mess.push(
                "<table><tr><th>OutMessage</th><th>Attribute</th><th>Comment</th></tr>".to_string(),
            );
            for x in allout {
                all_mess.push(format!("<tr><td>{x}</td></tr>", x = x));
                all_mess.push("<tr><td></td><td><i>attr1</i></td><td>...</td></tr>".to_string());
            }
            all_mess.push("</table>".to_string());
            all_mess.join("\n")
        };
        /*******************************************/
        let ett = format!(
            "
<body>
<head>
<style>
body {{
background-color: white;
}}
table,
th,
td {{
border: 0.5px solid black;
font-family: 'Courier New', Courier, monospace;
font-size: 13px;
color: black;
}}
th,
td {{
padding: 3px;
vertical-align: top;
}}
</style>
</head>
<h2>Machine Name: {mach_name}</h2>

<h3>Business Object</h3>
<table>
<tr><th>Attribute</th><th>Kind</th><th>Comment</th></tr>
<tr><td><i>attr1</i></td><td>...</td><td>...</td></tr>
</table>
<h3>In Messages</h3>
{inmess}
<h3>Out Messages</h3>
{outmess}
<h3>Guard functions</h3>
<table>
<tr>
  <th>State and<br>In-Message</th>
  <th>&nbsp;</th>
  <th>Guard Validation</th>
  <th>Outcome</th>
</tr>
{all_state_event}
</table>",
            mach_name = mach_name,
            inmess = build_all_inmessages(),
            outmess = build_all_outmessages(),
            all_state_event = build_all_state_event()
        );
        let tva = format!(
            "
  <h3>Outmessage Production and Customer Object Updates</h3>
  <table>
  <tr>
    <th>Outmessages</th>
    <th>Condition from <br>Guard Function</th>
    <th>Build Outmessage and <br>Update Business Object</th>
    <th>Outcome</th>
</tr>
{all_prod}
</table>
  </body>",
            all_prod = build_all_prod()
        );
        vec![ett.to_string(), tva.to_string()].concat()
    }
}
