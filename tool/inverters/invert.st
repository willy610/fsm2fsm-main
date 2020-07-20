| transitions_as_string transitions_as_rows transitions_as_rows_and_cols 
mirror_trans final_csv |

transitions_as_rows:=#(
'INIT,Open,1,OpenOk,Process'
'INIT,Open,2,OpenFail,FINAL'
'Process,Add,1,AddOK,Process'
'Process,Add,2,RetryAdd,Process'
'Process,Add,3,AddFail,FINAL'
'Process,Close,1,CloseOk,FINAL'
'Process,Close,2,CloseFail,FINAL').

transitions_as_rows_and_cols := transitions_as_rows collect:[:row|row splitOn: $,].
mirror_trans := Set new.
transitions_as_rows_and_cols do: [ :aRow|
	(aRow at:1) ='INIT' 
	ifTrue:[
		|event|
		event := aRow at:2.
		mirror_trans add:{'INIT'.'callin'.'NOGUARD_YET'. event. 'INIT','_',event}
		].
	(aRow at:5) ='FINAL' 
	ifTrue:[
		|from event out|
		from  := aRow at:1.
		event := aRow at:2.
		out   := aRow at:4.
		mirror_trans add:{from,'_',event.out.'NOGUARD_YET'. '-'.'FINAL'}
		].
	
	].
transitions_as_rows_and_cols do: [ :left|
	|l_from l_event l_next l_out|
	l_from  := left at:1.
	l_event := left at:2.
	l_out   := left at:4.
	l_next  := left at:5.
	transitions_as_rows_and_cols do: [ :right|
		|r_from r_event|
		r_from  := right at:1.
		r_event := right at:2.
		l_next = r_from 
		ifTrue:[
			mirror_trans add:{l_from,'_',l_event . l_out.'NOGUARD_YET' . r_event.r_from,'_',r_event}
			]
		].
	].

final_csv := Array new: (mirror_trans size).
mirror_trans doWithIndex: [:aRow :i|
	|from event out next|
	from  := aRow at:1.
	event := aRow at:2.
	out   := aRow at:4.
	next  := aRow at:5.
	final_csv at: i put: (from,',',event,',',(from),'_',(i printString),',',out,',',next).
	].
final_csv 

