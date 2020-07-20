# FSM2FSM <!-- omit in toc -->

## How to generate a collaborative FSM from a given FSM <!-- omit in toc -->

- [Background](#background)
- [Why FSM](#why-fsm)
- [Intended Audience](#intended-audience)
- [What's in the package](#whats-in-the-package)
- [Two Key Takeaways](#two-key-takeaways)
- [Main objective](#main-objective)
- [Concepts and Formats](#concepts-and-formats)
  - [Program source format](#program-source-format)
  - [Offline formats](#offline-formats)
  - [Online formats](#online-formats)
- [Usage overview](#usage-overview)
- [Inverting a FSM](#inverting-a-fsm)
  - [Algoritm Outlined](#algoritm-outlined)
  - [Ruby](#ruby)
  - [SQL](#sql)
  - [RUST](#rust)
  - [Other languages](#other-languages)
- [Software Pattern](#software-pattern)
  - [Program Source format of a FSM](#program-source-format-of-a-fsm)
- [Build the tool](#build-the-tool)
- [Examples of FSM](#examples-of-fsm)
- [Usage](#usage)
  - [show --astty](#show---astty)
  - [show --asmatrix](#show---asmatrix)
  - [show --groupby](#show---groupby)
  - [graph --asbox](#graph---asbox)
  - [genspectext](#genspectext)
  - [chat](#chat)
  - [genc](#genc)
  - [genrust](#genrust)
- [Code generation for a FSM](#code-generation-for-a-fsm)
- [Target platforms and tools](#target-platforms-and-tools)
  - [Rust](#rust-1)
  - [C](#c)
- [Using examples](#using-examples)
  - [Destination of generated sources](#destination-of-generated-sources)
    - [C](#c-1)
    - [Rust](#rust-2)
  - [Differens between new and update at source generation](#differens-between-new-and-update-at-source-generation)
  - [C - Code generation](#c---code-generation)
    - [C clientandserver executable](#c-clientandserver-executable)
    - [C stdinserver executable](#c-stdinserver-executable)
    - [C udp executables](#c-udp-executables)
  - [Rust - Code generation](#rust---code-generation)
  - [Build all executables](#build-all-executables)
    - [Rust clientandserver executable](#rust-clientandserver-executable)
    - [Rust stdinserver executable](#rust-stdinserver-executable)
    - [Rust udp executables](#rust-udp-executables)
  - [Own source modifications (C example)](#own-source-modifications-c-example)
- [How to continue](#how-to-continue)
  - [Dismiss (for a while?)](#dismiss-for-a-while)
  - [Enhance the cruisecontrol](#enhance-the-cruisecontrol)
  - [Eventlogger sketches](#eventlogger-sketches)
  - [Introduce timers](#introduce-timers)
  - [IETF RFC reform](#ietf-rfc-reform)

## Background

> Alice and Bob are related. Bob is Alice's niece. One of them, Alice is retired, rich and childless. She appreciates, well expect, phone calls from her younger siblings. She wants to distribute inheritance plots according to active social competence.  

>Alice is generally unsure as older can be. Especially in connection with telephone calls. Is it really a nephew calling or a jerk? But she has established a safe way to talk, chat, over the phone. Through a formalized speech act.  

>She has build a Finite State Machine with state, inmessage and outmessage. By putting her finger in the current state, she knows what to expect from a credible nephew and what she can choose to answer at any moment. Then she changes state. Safe and secure for an old lady.  

>At one point she described this FSM to Bob over the phone; one transition at a time. Bob listened dutifully but only realized later that he could act like Alice himself. A state machine that helped him respond correctly to Alice's claims.  

>And then he came to the realization that he could twist or mirror her FSM to get his own correct FSM so that they could form two collaborative FSM. He went back to his recorded phone calls and start working.

This document is a description of how Bob went about it

!['./overview01.png'](./mdfigures/overview01.png)
<p style="text-align: center;">- Fig 1 - Overview -</p>

## Why FSM

FSM (Finite State Machine) is a strong pattern when designing soft- and hardware. There are a plethora of aspects, theories and practices around this and with it a variety of values.  
For example a lot of communication protocols are outlined using FSM. In my opinion the usage is poor and unprofessional.

Introduction to FSM can be found in

[en.wikipedia.org/wiki/Finite-state_machine](https://en.wikipedia.org/wiki/Finite-state_machine)

An variant of FSM is the SDL which is covered in

[https://en.wikipedia.org/wiki/Specification_and_Description_Language](https://en.wikipedia.org/wiki/Specification_and_Description_Language)

Many protocols describing a client/server relation as one - and only one - helicopter FSM. To be useful there should be one FSM for each partner.

I think it would be more fruitful to describe the behavior for one part, the server, in a more rigourous FSM. Behavior is here a combination of inmessages, outmesages, conditions and states. 

>**The other part, the client, can then be generated to a counterpart FSM by the algoritm presented herein.**

Such a client could be used just to verify the server, or the client could be wrapped up as an API to a remote server or be a true client application. Or set up the sever as a test for an API implementation!

The mechanism to construct a counterpart FSM has no name because it is a new function. You may call it inverting, mirroring, complementing, counter-parting, mate, part...

## Intended Audience

* Communication Protocol designers and implementers
* Rust programmers
* C programmers
* Dialogue modeller
* Business Process modeller
* ...

## What's in the package

* Basic concepts
* Inverting a FSM
* The Pattern **FSMClass + GuardFunctions + OutMessageProducer + BusinessObject**
* Different Views of a FSM
* The Tool. [Build it here](#Build-the-tool)
* Generating your own FSM (Rust or C)

## Two Key Takeaways

I think this material seems very chaotic with a lot of figures, unclear writings, poor language and no pedagogical presentation. But in any case try to understand two important and hopefully fruitful things.

1. A Finite State Machine with transitions of kind `StateNow,Inmessage,OutMessage,NextState` can be transformed, inverted into a counterpart FSM so, together, they constitute a kind of client and server. Out of one specification! That's what this tool is about! (The tool is though useful towards just a FSM without inverting).  
[More on inverting here](#Inverting-an-FSM)

2. The FSM is used to maintain the attributes for a Business Object of some kind. Not primarily maintaining the state! Business Object could be a Mac-address and lease time in DHCP. Or minimum speed for a Cruise Control in a car. Or order information in a sales order. When software engineering the FSM into the pattern the FSM itself is in fact stateless!  
[More on pattern here](#software-pattern)

    1. FSM State and the Business Object are owned by the Invoker of the FSM  
    2. The FSM Box is actually built on four classes/objects. A state/Event dispatch FSMClass, GuardFunctions,  OutMessage Producer and a MessageFactory. As they are stateless they could have been built as plain functions!
    3. At each transition the Invoker feeds, lends out, the Box with InMessage, mutable State and mutable Business Object and gets an OutMessage back. Done.

## Main objective

* Short introduction
* Introduce some new formalism around FSM
* Offer a tool showing different views of a FSM
* Describe the inverting algorithm [here](#Inverting-an-FSM)
* Emulate a chat between a server and a client [here](#chat)
* Tool build
* Tool usage [here](#usage)
* Code generation for Rust and C [here](#Code-generation)

## Concepts and Formats

A FSM is traditionally defined by a set of Transitions. Each transition contains  

`FromState, Event, NextState`  

In order to reduce the number of messages and also to be able to take care of bad messages a Guard function is applied on a state/event value. (The `guard` is for historical reason from UML but perhaps selector, alternative, variant, conditions or similar would be better.) Each transition then looks like  

`FromState, Event, Guard Condition_1, NextState_1`  
`FromState, Event, Guard Condition_2, NextState_2`  

As we are deeply intrested in the inverting opportunity we must be able to specify the output too. We got, after renaming event to inmessage the columns

`FromState, InMessage, GuardCondition, OutMessage, NextState`  

### Program source format

A very simple FSM machine in source format  - accepting `Open,More...,Close` - might look like

```text
INIT,Open,Guard_ONE,Out1,Process
Process,More,Guard_TWO,Out2,Process
Process,Close,Guard_THREE,Out3,FINAL
```

Note the mandatory INIT and FINAL state.

The tool could be used to display the FSM more tasty.

```text
here > thetool testdata/server1.txt show --astty

+--State--+-InMessage-+-UserGuardValue-+-OutMessage-+-NextState-+
|INIT     |Open       |Cond_OK         |OK          |Process    |
|Process  |Close      |Cond_OK         |Ok          |FINAL      |
|         |More       |Cond_OK         |OK          |Process    |
+---------+-----------+----------------+------------+-----------+
```

And the `inverted` FSM becoms:

```text
here > thetool testdata/server1.txt show --astty -i

+-----State-----+-InMessage-+---UserGuardValue---+-OutMessage-+---NextState---+
|INIT           |Callin     |INIT_Callin_4       |Open        |INIT_Open      |
|INIT_Open      |OK         |INIT_Open_OK_5      |Close       |Process_Close  |
|               |           |INIT_Open_OK_6      |More        |Process_More   |
|Process_Close  |Ok         |Process_Close_Ok_3  |nooutput    |FINAL          |
|Process_More   |OK         |Process_More_OK_1   |More        |Process_More   |
|               |           |Process_More_OK_2   |Close       |Process_Close  |
+---------------+-----------+--------------------+------------+---------------+
```

### Offline formats

There are many advantages to representing a FSM in 1NF (First normal Form from Relational Database. Close to a CSV-file where all rows has the same column names). You can process data with traditional SQL but also with known patterns - filter, sort, select, group by, etc - in common programming languages.

!['./mdfigures/datamodell02.png'](./mdfigures/datamodell02.png)
<p style="text-align: center;">- Fig 2 - Datamodell -</p>

You can extend this database with whatever you need. Business Object, Source and Destination for Messages if you work with multiple FSM. You can add timers. 

Or add a relation between OutMessages and guard-condition with an attribute like specification of which fields in the OutMessages should be produced depending on guard-condition. Look into the usage `genspectext`

Note. The transition is actually an SQL View. Quite easy to export as CSV.

### Online formats

The machines as input to the tool are defined as CSV-files with the implicit columns shown as

`State,InMessage,UserGuardValue,OutMessage,NextState`

## Usage overview

Given a certain FSM source file you can:

* **show**  
  Different text displays of a FSM. astty, asmatrix, ascsv. [Example](#show---astty)
* **graph**  
  More classic graphic views of a FSM. asbox, ascircle. 
  [Example](#graph---asbox)
* **genspectext**  
  Will generate an HTML page which can be dropped into a browser and copy pasted into a word processor. It holds sections for documenting relationships between In Message, Guard Functions, Out Message and Business Object. [Example](#genspectext)
* **chat**  
  Will invoke a chat between a client and server. You will be  involved as making the Guard Condition Choice. [Try it](#chat)
* **genc**
* **genrust**  
  Will generate source for FSM, GuardFunctions and OutMessagProducer. For Rust or C

## Inverting a FSM

>**Note**. You don't need to utilize or even care of the inverting facility. Just use the components for the server. The client and server are two separate compenents at build time and at run time.


Shortly. Join rows from the FSM transitions on itself and select distinct
from the result where Next_State = State.

Out of that join do produce a transition row for the Inverter taking a smart combination of columns from left and right!

Add some transitions for the initial and final states in the Inverter. In the outlines below the final state is 'FINAL' and the initial state is 'INIT'


### Algoritm Outlined

When joining two transaction (as Left and as Right) with 
  `Left.NextState = Right.State` we can see

1. The server will send `Left.OutMessage` and go to `Left.NextState`
2. The server will later accept `Right.InMessage` in this `Right.State`, 
   which is the same as `Left.NextState`!

The invert to be generated must therefor have a transition looking like

`state_N,Left.OutMessage,'-',Right.InMessage,state_M`

How to assign namnes to these states?

We can look into the border construction for `INIT` and `FINAL` states
and use the same pattern for all.

INIT: ('callin' is used for start the inverted machine)

For all transitions with `State = 'INIT'` add to the invert transitions

 `'INIT','callin',,event,'INIT'_event`
  
FINAL:
For all transitions with Next_State = 'FINAL' add to the invert transitions

`'FINAL'_event,out,,'FINAL'`

From that one can use the pattern (from SQL)

```SQL
SELECT DISTINCT Concat(left.state, '_', left.inmessage)    AS State, 
                 left.outmessage                           AS Event, 
                 '-'                                       AS 'Guard Value', 
                 right.inmessage                           AS Output, 
                 Concat(right.state, '_', right.inmessage) AS 'NextState'
```

### Ruby

Here we have a working rouby source for inverting a FSM. You can copy and paste into a file and let ruby interpret it.

```ruby
transitions = [
'INIT,Open,1,OpenOk,Process',
'Process,Add,1,AddOK,Process',
'Process,Add,2,RetryAdd,Process',
'Process,Add,3,AddFail,FINAL',
'Process,Close,1,CloseOk,FINAL',
'Process,Close,2,CloseFail,FINAL'
]
transitions.collect!{|row|row.split(',')}
mirror_trans =[]
transitions.each {|from,event,guard,out,nxt|
  if from == 'INIT'
    mirror_trans << ['INIT','callin','',event,'INIT'+'_'+event]
  end
  if nxt == 'FINAL'
    mirror_trans << ['FINAL'+'_'+event,out,'','-','FINAL']
  end
}
transitions.each {|l_from,l_event,l_guard,l_out,l_nxt|
  transitions.each {|r_from,r_event,r_guard,r_out,r_nxt|
    if l_nxt == r_from
      mirror_trans << [l_from+'_'+l_event,l_out,'',
        r_event,r_from+'_'+r_event]
    end
    }
  }
mirror_trans.uniq!
guard_value = 0
mirror_trans.collect!{|from,event,guard,out,nxt|
  guard_value += 1
  [from,event,guard_value,out,nxt]}
mirror_trans.each{|from,event,guard,out,nxt|
  puts from+','+event+','+guard.to_s+','+out+','+nxt}
```

### SQL

Having FMS's in an SQL database is fruitful for many reasons. Versioning,checks, extensions etc

```sql
SELECT DISTINCT 'INIT'                     AS State,
                'callin'                   AS Event, 
                '-'                        AS 'Guard Value', 
                inmessage                  AS Output, 
                Concat('INIT', '_', inmessage) AS 'NextState' 
FROM   transition 
WHERE  state = 'INIT' 
       AND fsm = 'adderserver' 
UNION 
(SELECT DISTINCT Concat(state, '_', inmessage) AS State, 
                 outmessage                AS Event, 
                 '-'                       AS 'Guard Value', 
                 '-'                       AS Output, 
                 'FINAL'                   AS 'NextState' 
 FROM   transition 
 WHERE  nextstate = 'FINAL' 
        AND fsm = 'adderserver') 
UNION 
(SELECT DISTINCT Concat(first.state, '_', first.inmessage)   AS State, 
                 first.outmessage                            AS Event, 
                 '-'                                         AS 'Guard Value', 
                 second.inmessage                            AS Output, 
                 Concat(second.state, '_', second.inmessage) AS 'NextState' 
 FROM   transition AS first 
        LEFT JOIN transition AS second 
               ON first.nextstate = second.state 
 WHERE  first.fsm = 'adderserver' 
        AND second.fsm = 'adderserver')  
```

### RUST

The method for creating the Inverted FSM could be found
in the source fms.rs and method is mirror_direct()

It's almost as compact as the ruby above.

### Other languages

Look into inverters/ and you will find the above source but also for Javascript, Smalltalk and Mathematica.

## Software Pattern

### Program Source format of a FSM

* Invoker, driver
* Business Object
* FSM State
* FSMClass + GuardFunctions + ProdOutMessages
* Message factory

All sources in the `Box` below are generated with the tool for a certain FSM machine. Acutally every machine will be produced the same way with the same pattern but with different content. The invokers can be used on any machine.

!['./mdfigures/components03.png'](./mdfigures/components03.png)
<p style="text-align: center;">- Fig 3 - Components -</p>

>**Note**. As there is a 1-1 exclusive mapping between state/inmessage in the FSM and the dedicated guardfunction this guardfunction could be inlined as source in the FSM. For performance reason. But it might be harder to maintain though.

## Build the tool

On the distribution there is a Rust project which can be rebuilt directly.

```#
> export THEDISTR=<...>/fsm2fsm
> #export THEDISTR=/Users/willy/MYRUST/fsm2fsm
> cd $THEDISTR/tool
> cargo clean
> cargo build
```

Ensure it was built

```
> alias thetool=$THEDISTR/tool/target/debug/tool
> thetool -h
```

## Examples of FSM

You find them on `tool/testdata`

* dhcpserver.txt (proper)
* adderserver.txt (open,add,add_again,close)
* alice.txt (famous)
* alice1.txt (demo start)
* touchevents.txt (imported example on one-two finger move and scale)
* bufferserver.txt (zlib's deflate method)
* bgp1771server.txt (outline only)
* cruisecontroll.txt (embryo)

## Usage

```
> thetool -h
```

Gives

```
<fsm> [-i] [options] show ( --astty | --ascsv | --asmatrix )
<fsm> [-i] [options] show --groupby --keys key1,.. --cols col1,...
<fsm> [-i] [options] genrust (--new | --update ) --destproject <directory>
<fsm> [-i] [options] genc (--new | --update ) --destproject <directory>
<fsm> [-i] [options] genspectext
<fsm> [-i] [options] calc
<fsm> [-i] [options] chat
<fsm> [-i] [options] graph [ --layoutnumber n ] ( --asbox | --ascircle )
<fsm> [-i] chat
<fsm> a finite state machine source file
--invert,-i       Will work on first inverting the 
--help,-h         Show help
--astty           Simple more pretty form of transactions
--ascsv           Raw inputformat
--asmatrix        Tabulated in event and state
--groupby         Grouped by --keys, -k showing specified --cols, -c
--destproject     Will generate source and drivers for a 
    Server and Client fsm talking to each other
--new             Will generate all required sources
--update          Will generate the sources for the FSM object only
--layoutnumber,-l Use a certain optional layoutnumber got from calc command 
```

### show --astty

```
> thetool adderserver.txt show --astty
```

Gives a compact formatted text output.

( A banal server accepting a Open, zero or more Add and a final Close )

```
+--State--+-InMessage-+-UserGuardValue-+-OutMessage-+-NextState-+
|INIT     |Open       |1               |OpenOk      |Process    |
|         |           |2               |OpenFail    |FINAL      |
|Process  |Add        |1               |AddOK       |Process    |
|         |           |2               |RetryAdd    |Process    |
|         |           |3               |AddFail     |FINAL      |
|         |Close      |1               |CloseOk     |FINAL      |
|         |           |2               |CloseFail   |FINAL      |
+---------+-----------+----------------+------------+-----------+
```

The machine for a generated client (-i) looks like

```
> thetool adderserver.txt show --astty -i
```

```
+-----State-----+-InMessage-+------UserGuardValue-------+-OutMessage-+---NextState---+
|INIT           |Callin     |INIT_Callin_9              |Open        |INIT_Open      |
|INIT_Open      |OpenFail   |INIT_Open_OpenFail_11      |nooutput    |FINAL          |
|               |OpenOk     |INIT_Open_OpenOk_3         |Close       |Process_Close  |
|               |           |INIT_Open_OpenOk_8         |Add         |Process_Add    |
|Process_Add    |AddFail    |Process_Add_AddFail_7      |nooutput    |FINAL          |
|               |AddOK      |Process_Add_AddOK_10       |Add         |Process_Add    |
|               |           |Process_Add_AddOK_6        |Close       |Process_Close  |
|               |RetryAdd   |Process_Add_RetryAdd_4     |Close       |Process_Close  |
|               |           |Process_Add_RetryAdd_5     |Add         |Process_Add    |
|Process_Close  |CloseFail  |Process_Close_CloseFail_2  |nooutput    |FINAL          |
|               |CloseOk    |Process_Close_CloseOk_1    |nooutput    |FINAL          |
+---------------+-----------+---------------------------+------------+---------------+
```

### show --asmatrix

Another text form is a kind of matrix with one row for each fromstate and one column for each tostate.

```
> thetool adderserver.txt show --asmatrix

+------------+----------+-----------+
|tostate ->  |          |           |
|            |-Process--|---FINAL---|
|fromstate -v|          |           |
+------------+----------+-----------+
|INIT        |Open      |Open       |
|            | 1        | 2         |
|            |  OpenOk  |  OpenFail |
+------------+----------+-----------+
|Process     |Add       |Add        |
|            | 1        | 3         |
|            |  AddOK   |  AddFail  |
|            |Add       |Close      |
|            | 2        | 1         |
|            |  RetryAdd|  CloseOk  |
|            |          |Close      |
|            |          | 2         |
|            |          |  CloseFail|
+------------+----------+-----------+
```

### show --groupby

This option let you view a FSM grouped on what columns you are interested in. If you like to see under what circumstances OutMessages are to be produced to type

```
> thetool dhcpserver.txt show --groupby --keys OutMessage --cols 0,1,2

+OutMessage++State------------+InMessage---------+UserGuardValue---+
|ACK       ||WAITREQUEST      |DECLINE           |Sorry            |
|          ||WAITREQUEST      |REQUEST           |Enjoy            |
|          ||WAITREQUEST_REUSE|DECLINE           |AcceptThat       |
|          ||WAITREQUEST_REUSE|RELEASE           |OkReleased       |
|          ||WAITREQUEST_REUSE|REQUEST           |OkReUse          |
|INFORM    ||IPAllocated      |INFORM            |OK               |
|NAK       ||INIT             |DISCOVER          |Error            |
|          ||INIT             |DISCOVER_TRY_REUSE|OldNotIPAvailable|
|OFFER     ||INIT             |DISCOVER          |NewIPAvailable   |
|          ||INIT             |DISCOVER_TRY_REUSE|OldIPAvailable   |
|          ||IPAllocated      |DISCOVER_TRY_REUSE|Free             |
|          ||IPAllocated      |RELEASE           |NowReleased      |
|nooutput  ||INIT             |DISCOVER          |NoIPAvailable    |
|          ||IPAllocated      |tmo               |Tmo2             |
|          ||WAITREQUEST      |tmo               |Tmo1             |
|          ||WAITREQUEST_REUSE|tmo               |Tmo3             |
+----------++-----------------+------------------+-----------------+
```

Keys and columns could be either columnindex or named.

### graph --asbox

You can get a view of a FSM as a more traditional graph by

```
> thetool dhcpserver.txt graph --asbox >dhcpserver.svg
```

Drop the (standard) output onto any web browser and you get

!['./mdfigures/dhcpserver04.png'](./mdfigures/dhcpserver04.png)
<p style="text-align: center;">- Fig 4 - DHCP server -</p>

If there are too many crossig lines try optimise via

```
> thetool dhcpserver.txt calc
```
You get `Optimum layout at number '1798345'`

Use the iter value as value for --layoutnumber

```
> thetool dhcpserver.txt graph --layoutnumber 1798345 --asbox >dhcpserver.svg
```

### genspectext

In early phases when outlines of messages content is about to be drawn but also when a final public specification is to be produced a framework is needed.
There is a possibility to get a basis for a structured text document. 

Sections are

1. Business Object
2. In Messages
3. Out Messages
4. Guard functions
5. Outmessage Production and Customer Object Updates

Run this function, drop the html file into a browser, copy and paste into your favorite word editor or import the html file direct if possible.

Could be a killer! Give it a try.

```
> thetool dhcpserver.txt genspectext >adderserver.html
```

Here is a partial screen scrape of that

!['./mdfigures/adderserver05.png'](./mdfigures/adderserver05.png)
<p style="text-align: center;">- Fig 5 - Example genspectext -</p>

### chat

This function will invoke a chat between a client and a server. You will be asked for making the guard condition decisions. Suitable for basic scenarios and use cases.

```
> thetool alice.txt chat
FSM Name=Client, State Now=INIT, In Event=Callin
 Choose a Condition number
[
    "0->INIT_Callin_19, Outevent=Hello_Alice, Nextstate=INIT_Hello_Alice",
    "1->INIT_Callin_4, Outevent=Ping, Nextstate=INIT_Ping",
]
0
FSM Name=Server, State Now=INIT, In Event=Hello_Alice
 Choose a Condition number
[
    "0->Alert, Outevent=Hello_Bob, Nextstate=StartUp",
    "1->Tired, Outevent=DontDisturb, Nextstate=Final",
]
0
FSM Name=Client, State Now=INIT_Hello_Alice, In Event=Hello_Bob
 Choose a Condition number
[
    "0->INIT_Hello_Alice_Hello_Bob_12, Outevent=How_Are_You, Nextstate=StartUp_How_Are_You",
]
0
FSM Name=Server, State Now=StartUp, In Event=How_Are_You
 Choose a Condition number
[
    "0->Good, Outevent=Rich, Nextstate=KeyExchage",
    "1->Tired, Outevent=Tired, Nextstate=Final",
]
1
Server went into final state
```

### genc

The tool will generate a set of files for the actual FSM into a folder named `generatedsources`. Note that the present make files assumes the folder name for the generated files is `generatedsources`. 

```
> thetool dhcpserver.txt genc [--what] (--new | --update) --destproject <projfolder>
```

For more information look at [here](#destination-of-generated-sources)

### genrust

The tool will generate a set of files and folders for the actual FSM into the rustproject `src`

```
> thetool dhcpserver.txt genrust [--what] (--new | --update) --destproject <projfolder>
```

For more information look at [here](#destination-of-generated-sources)

## Code generation for a FSM

The aim for code generation is to create sources for all (inner) components of a FSM. The outer parts, the invoker(s), are not generated but are the same for any machine you generate from. You can than try different scenarios for the same machine.


The distribution folder holds two folders:

* `tool` with sources for the tool
* `examples` with examples and source for C and Rust and scripts for build and execute FSM's.

The projects are ready made projects in Rust and C. The source is at `examples`. There are 5 different kind of executables (invokers)

  1. A main holding a clientemulator and a serveremulator. Just ping-pong.
  2. A main reading stdin as In Message and driving a client FSM.
  3. A main reading stdin as In Message and driving a server FSM.
  4. A main server listening on UDP connections. The server is a multisession, single-threaded one. Each message is shorter than max UDP size.
  5. One or more clients connecting to the UDP server. 

The stdin client or server could be useful for test/verification. Note that they are reading InMessages from a files called stdinCLIENT.txt and stdinSERVER.txt as the debuggers have problem supporting traditional stdin.

You can make a quick start by using VS Code and open a window on `examples/rust/projects` or `examples/c/projects/ANYfolder`. Build and debug existing Rust or C examples. Or look into the make files and Cargo.toml if you are not familiar with VS Code.

Source for both server and client are always generated.

## Target platforms and tools

The generated examples on examples has been build as follows

### Rust

| OS  | Edit | Build | Debug | Run |
| --- | ---- | ----- | ----- | --- |
| macOS | VS Code | VS Code task (cargo build)| VS Code launch Plug-in | Terminal |
| Windows | VS Code | VS Code (cargo build)| - | PowerShell |
| Raspberry Pie | TextEdit | cargo build | - | shell |

### C

| OS  | Edit | Build | Debug | Run |
| --- | ---- | ----- | ----- | --- |
| macOS | VS Code | VS Code task (make) | VS Code launch (cppdbg) | Terminal |
| Windows | VS Code | VS Code task (cl)| VS Code launch (cppvsdbg) | PowerShell |
| Raspberry Pie | TextEdit | make | - | shell |

1. You can always look into the make* files (for C) to find out how to adjust your favorite IDE on Unix and also into the .vscode tasks and launch for Windows. (The makefiles does not have full dependency rules so adjust or always do a clean before build.)
2. And of course look into the Cargo.toml and the hierarchy of mod.rs, *.rs and *-folder. 
3. On windows you must start the VS Code using 'Powershell for VS Code...'

## Using examples

>**Note**. You can work on the `examples` immediately. It's generated with the alice FSM. You don't even need to copy them. Just build, debug and run.

Copy from `examples`. So assign a path to the `examples` folder via

```
> export FROMEXAMPLES=/Users/<...>/fsm2fsm/examples/
> # export FROMEXAMPLES=/Users/willy/MYRUST/fsm2fsm/examples/
```

We use the `testdata/alice.txt` as an example for all targets.

### Destination of generated sources

When generating source files for machine note the following.

You can use the suboption `--what` which will cause files to listed but not written.

#### C

The files will be written into the folder `--destproject` x/`generatedsources`

```
> thetool testdata/alice.txt genc --what --new --destproject x
```

will give a list

```
File to be written= x/generatedsources/Messages.h
File to be written= x/generatedsources/CustomRealMsg.h
File to be written= x/generatedsources/MessageTypes.h
File to be written= x/generatedsources/C_GuardConditions.h
File to be written= x/generatedsources/S_GuardConditions.h
File to be written= x/generatedsources/C_GI.h
File to be written= x/generatedsources/S_GI.h
File to be written= x/generatedsources/C_GI.c
File to be written= x/generatedsources/S_GI.c
File to be written= x/generatedsources/C_PI.c
File to be written= x/generatedsources/S_PI.c
File to be written= x/generatedsources/C_PI.h
File to be written= x/generatedsources/S_PI.h
File to be written= x/generatedsources/C_FSM.h
File to be written= x/generatedsources/S_FSM.h
File to be written= x/generatedsources/C_FSM.c
File to be written= x/generatedsources/S_FSM.c
File to be written= x/generatedsources/MsgFactory.c
File to be written= x/generatedsources/MsgFactory.h
```

#### Rust

The files will be written into the folder `--destproject` x/`src`

```
> thetool testdata/alice.txt genrust --what --new --destproject x
```

will give a list (note that several foldesr are used)

```
File to be written= x/src/shared_folder/realmessages.rs
File to be written= x/src/client_folder/messagesets.rs
File to be written= x/src/server_folder/messagesets.rs
File to be written= x/src/client_folder/msgfactoryclass.rs
File to be written= x/src/server_folder/msgfactoryclass.rs
File to be written= x/src/client_folder/fsmclass.rs
File to be written= x/src/server_folder/fsmclass.rs
File to be written= x/src/client_folder/guardclass.rs
File to be written= x/src/server_folder/guardclass.rs
File to be written= x/src/client_folder/prodmsgclass.rs
File to be written= x/src/server_folder/prodmsgclass.rs
File to be written= x/src/client_folder/guardconditions.rs
File to be written= x/src/server_folder/guardconditions.rs
File to be written= x/src/client_folder/prodconditions.rs
File to be written= x/src/server_folder/prodconditions.rs
```


### Differens between new and update at source generation

When you update the set of transitions for a FSM you can regenerate some parts of the sources inside the FSM box. Use `--update`

1. the FSMClass is regenerated
2. the set of Messages and MessageTypes are regenerated
3. the set of GuardConditions is regenerated

Other sources as below generated at `--new` is not affected.

1. C : CustomRealMsg, GI (Guard), PI (Prodmsg), MsgFactory.
2. Rust : guardclass, prodmsgclass, msgfactoryclass, realmessages.

They are probably edited to conform to you real conditions and message content. Be ware that when rebuild you for sure get a lot of compiler errors. Depending on what changes you made to the machine

Elaborate on different changes in the machine like new name on states, message, new or deleted transitions, new tostate, fewer or more guardconditions.

I have tried to make all `match-switch` in sources full domain without any defaults in order to ensure proper covering.

Here we go for C

```
> thetool testdata/alice.txt genc --what --update --destproject x
```

```
File to be written= x/generatedsources/Messages.h
File to be written= x/generatedsources/MessageTypes.h
File to be written= x/generatedsources/C_GuardConditions.h
File to be written= x/generatedsources/S_GuardConditions.h
File to be written= x/generatedsources/C_FSM.h
File to be written= x/generatedsources/S_FSM.h
File to be written= x/generatedsources/C_FSM.c
File to be written= x/generatedsources/S_FSM.c
```

and for Rust

```
> thetool testdata/alice.txt genrust --what --update --destproject x
```

```
File to be written= x/src/client_folder/messagesets.rs
File to be written= x/src/server_folder/messagesets.rs
File to be written= x/src/client_folder/fsmclass.rs
File to be written= x/src/server_folder/fsmclass.rs
File to be written= x/src/client_folder/guardconditions.rs
File to be written= x/src/server_folder/guardconditions.rs
File to be written= x/src/client_folder/prodconditions.rs
File to be written= x/src/server_folder/prodconditions.rs
```

### C - Code generation

Suppose you want a project in folder real `MYPROJ`. Assign a path to that project and create the folder. First assign a dirpath to where you build the tool.

```
> export DESTPROJ=/Users/<...>/MYPROJ
> #export DESTPROJ=/Users/willy/MYRUST/MYPROJ
> mkdir $DESTPROJ
```

and copy all files from the distribution

```
> cp -R $FROMEXAMPLES/c/projects/* $DESTPROJ
```

It will look like

```
> ls -1p
clientserverfolder/
generatedsources/
stdinclientfolder/
stdinserverfolder/
udpclientfolder/
udpserverfolder/
```

If you are intrested just in one project like stdinserverfolder just do

```
> cp -R $FROMEXAMPLES/c/projects/generatedsources/* $DESTPROJ/generatedsources
> cp -R $FROMEXAMPLES/c/projects/stdinserverfolder/* $DESTPROJ/stdinserverfolder
```

When you generate source for a FSM the only folder affected is `generatedsources` inside the DESTPROJ.

To generate the sources to the FSM box for testdata/alice.txt you do

```#
> cd /Users/<...>/fsm2fsm/tool
> #cd /Users/willy/MYRUST/fsm2fsm/tool
> alias thetool=$THEDISTR/tool/target/debug/tool
```

```
> thetool testdata/alice.txt genc -d --what $DESTPROJ --new
```

#### C clientandserver executable

```
> cd $DESTPROJ/clientserverfolder
> make -f makeclientserver clean
> make -f makeclientserver
```

Now execute the simple emulator ping pong for server and client

```
> ./clientandserver --seed 610
```

You get

```
seed=610
Client >>>>>      Msg_ping      >>> Server
Client <<<<<      Msg_pong      <<< Server
Client >>>>>  Msg_how_are_you   >>> Server
Client <<<<<      Msg_rich      <<< Server
Client >>>>>       Msg_42       >>> Server
Client <<<<<       Msg_43       <<< Server
Client >>>>>       Msg_hm       >>> Server
Client <<<<<       Msg_or       <<< Server
Client >>>>>       Msg_no       >>> Server
Client <<<<<      Msg_what      <<< Server
Client >>>>>     Msg_other      >>> Server
Client <<<<<      Msg_bye       <<< Server
Server is in finalstate
Client is in finalstate
```

#### C stdinserver executable

Now generate and run the stdin programs. Note the generatedsources are the same in all examples.

```
> cd $DESTPROJ/stdinfolder
> make -f makestdinserver clean
> make -f makestdinserver
```

and run

```
./stdinserver --seed 610
```

You get

```
Server >>>>>  Msg_hello_alice   >>> Server
Server <<<<<   Msg_hello_bob    <<< Server
Server >>>>>  Msg_how_are_you   >>> Server
Server <<<<<     Msg_tired      <<< Server
The server went into final state. No more In Messages are allowed.
```

#### C udp executables

Here we build a server and a client which will interchange messages on UDP. The server is of kind multi-session but single-threaded.

```
> cd $DESTPROJ/udpfolder
> make -f makeudpserver clean
> make -f makeudpserver
> make -f makeudpclient clean
> make -f makeudpclient
```

Now start the server first and then one or more clients. One console window for the server and one for each client.

```
> ./udpserver --seed 610
```

```
> ./udpclient --seed 610
Client >>>>>      Msg_ping      >>> Server
Client <<<<<      Msg_pong      <<< Server
Client >>>>>  Msg_how_are_you   >>> Server
Client <<<<<     Msg_tired      <<< Server
Client is now in finalstate
```

In the server console we get

```
New session on (Port:27074)
Client (Port:27074) >>>>>      Msg_ping      >>> Server
Client (Port:27074) <<<<<      Msg_pong      <<< Server
Client (Port:27074) >>>>>  Msg_how_are_you   >>> Server
Client (Port:27074) <<<<<     Msg_tired      <<< Server
Session (Port:27074) went into finalstate
```

### Rust - Code generation

Suppose you want a project in folder real `MYPROJ`. Assign a path to that project and create the folder. First assign a dirpath to where you build the tool.

```
> export DESTPROJ=/Users/<...>/MYPROJ
> #export DESTPROJ=/Users/willy/MYRUST/MYPROJ
> mkdir $DESTPROJ
```

and copy all files from the distribution

```
> cp -R $FROMEXAMPLES/rust/projects/* $DESTPROJ
```

It will look like. It holds one Cargo.toml for all builds. Browse that one in order to understand the file structure.

```
> ls -1p
Cargo.lock
Cargo.toml
src/
stdinCLIENT.txt
stdinSERVER.txt
target/
```

 and the /src holds four *.rs file implementing each main()

```
>ls -1p src
client_folder/
clientandserver.rs
clientandserver_folder/
server_folder/
shared_folder/
stdinclient.rs
stdinserver.rs
udpclient.rs
udpserver.rs
```

When you generate source for a FSM the only folder affected are `client_folder` and `server_folder` inside the DESTPROJ.

To generate the sources to the FSM box for testdata/alice.txt you do

```#
> cd /Users/<...>/fsm2fsm/tool
> #cd $THEDISTR
> alias thetool=$THEDISTR/tool/target/debug/tool
```
and gen for rust
```
> thetool tool/testdata/alice.txt genrust -d $DESTPROJ --new
```

### Build all executables

```
> cd $DESTPROJ/clientserverfolder
> cargo clean
> cargo build
```

#### Rust clientandserver executable

Now execute the simple emulator ping pong for server and client

```
> ./target/debug/clientandserver --seed 2
```

You get

```
Client >>>>>>     Msg_ping,siiiiiie     >>>>>>> Server
Client <<<<<<     Msg_pong,shhhhhhs     <<<<<<< Server
Client >>>>>> Msg_how_are_you,seeeeees  >>>>>>> Server
Client <<<<<<    Msg_tired,slllllls     <<<<<<< Server
Server_main_Object:: fsm went into final state
Client_main_Object:: fsm went into final state
```

#### Rust stdinserver executable

You can edit the file `stdinSERVER.txt` which holds messages to be sent to the client. And try different `--seed` values!

```
> ./target/debug/stdinserver --seed 0
```

You get

```
Stdin  >>>>>>         Msg_hello_alice,Nice you answered          >>>>>>> FSM
Stdout <<<<<<               Msg_hello_bob,sffffffN               <<<<<<< FSM
Stdin  >>>>>>           Msg_how_are_you,mummel mummel            >>>>>>> FSM
Stdout <<<<<<                 Msg_rich,sjjjjjjm                  <<<<<<< FSM
Stdin  >>>>>>           Msg_24,OKj o owijefowij efoj?            >>>>>>> FSM
Stdout <<<<<<              Msg_questionwas,siiiiiiO              <<<<<<< FSM
Stdin  >>>>>>           Msg_24,OKj o owijefowij efoj?            >>>>>>> FSM
Stdout <<<<<<              Msg_questionwas,siiiiiiO              <<<<<<< FSM
Stdin  >>>>>>           Msg_24,OKj o owijefowij efoj?            >>>>>>> FSM
Stdout <<<<<<              Msg_questionwas,siiiiiiO              <<<<<<< FSM
Stdin  >>>>>>           Msg_24,OKj o owijefowij efoj?            >>>>>>> FSM
Stdout <<<<<<              Msg_questionwas,siiiiiiO              <<<<<<< FSM
Stdin  >>>>>>           Msg_24,OKj o owijefowij efoj?            >>>>>>> FSM
Stdout <<<<<<              Msg_questionwas,siiiiiiO              <<<<<<< FSM
Stdin  >>>>>>           Msg_42,OKj o owijefowij efoj?            >>>>>>> FSM
Stdout <<<<<<                  Msg_43,saaaaaaO                   <<<<<<< FSM
Stdin  >>>>>>         Msg_well,iwellwellwellwellwellwell         >>>>>>> FSM
Stdout <<<<<<                Msg_bye_bob,sddddddi                <<<<<<< FSM
stdinserver:: fsm went into final state
```

#### Rust udp executables

We have built a server and a client which will interchange messages on UDP. The server is of kind multi-session but single-threaded. If you have built the C version you can of cource intermix C and Rust udp executables.

Now first start the server and then one or more clients. One console window for the server and one for each client.

```
> ./target/debug/udpserver --seed 1232
```

```
> ./target/debug/udpclient --seed 610
Client >>>>>      Msg_ping      >>> Server
Client <<<<<      Msg_pong      <<< Server
Client >>>>>  Msg_how_are_you   >>> Server
Client <<<<<     Msg_tired      <<< Server
Client is now in finalstate
```

### Own source modifications (C example)

1. Define a machine with transitions or use one existing.
2. Generate the specifiction template `genspectext`
3. Use the project stdinsever as driver.
4. Edit the first message in stdinSERVER.txt. Use some basic JSON as format.
5. Edit `CustomRealmsg.h` and define a computational structure.
6. Edit `MsgFactory.c` (wire2comp_)
7. Edit S_BusinessObject.h to take care of inmessage.
8. Edit S_GI.c the guardfunctions to further checks.
9. Edit S_PI.c and take care of updates to BusinessObject.
10. Creating an outmessage. Edit `CustomRealmsg.h` and `MsgFactory.c` (wire2comp)
11. First turn is done!
12. Continue with next inmessage

## How to continue


### Dismiss (for a while?)

### Enhance the cruisecontrol

Chat the cruisecontroller.

When making dialogues, in MVC patterns, you can map controls to inmessages. The FSM will tell which controls should be enabled in each state. Use the FSM to creta runtime enabling of controllers. At higer level where you have a FSM managing several views you tell which views should be visibal or enabled at each state.

### Eventlogger sketches

1. Build an eventlogger for different devices which will collect events to a server managing one file for each device.
2. Offer an API to the client with parameters like id=unitnam,data and dataleegth.
3. The API is an invoker to the Client FSM Box. The business object is the id and the data to be collected in one or more UDP records.
4. Both client and server must have time out on recvfrom. Use `select` with some time out on the `filedescriptors` for the sockets. Gently add a time-out message into the FSM components.
5. The server will collect each record and append that to the proper file.
6. The server must accept a message from an operator to backup one logfile to other back ups in order to keep the logifile small.
7. Start with FSM testdata/adderserver.txt and look into the udp* projects.

### Introduce timers

Timers are covered at all above. There are several requirements on this. Timeout, keepalive, ownership, mutual, absolute or delta, etc

Enhance the database with diffrent timers. Make skeletons and injections/generators.

### IETF RFC reform

There are many RFC using FSM in order to define behavior etc. In the case of client/server there is a mix of message content definitions and message content checks.

So, pick your favorite RFC, outline a basic FSM, or split into several FSM (several Business Objects?), and use the tool with `--genspectext` to get a word document and try to copy portion from the RFC into your document.

