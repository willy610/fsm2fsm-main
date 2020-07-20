transitions = [
'INIT,Open,1,OpenOk,Process',
'INIT,Open,2,OpenFail,FINAL',
'Process,Add,1,AddOK,Process',
'Process,Add,2,RetryAdd,Process',
'Process,Add,3,AddFail,FINAL',
'Process,Close,1,CloseOk,FINAL',
'Process,Close,2,CloseFail,FINAL']
transitions.collect!{|row|row.split(',')}
mirror_trans =[]
transitions.each {|from,event,guard,out,nxt|
  if from == 'INIT'
    mirror_trans << ['INIT','callin','NOGUARD_YET',event,'INIT'+'_'+event]
	end
  if nxt == 'FINAL'
    mirror_trans << [from+'_'+event,out,'NOGUARD_YET','-','FINAL']
  end
}
transitions.each {|l_from,l_event,l_guard,l_out,l_nxt|
  transitions.each {|r_from,r_event,r_guard,r_out,r_nxt|
    if l_nxt == r_from
      mirror_trans << [l_from+'_'+l_event,l_out,'NOGUARD_YET',
        r_event,r_from+'_'+r_event]
    end
    }
  }
mirror_trans.uniq!
guard_value = 0
mirror_trans.collect!{|from,event,_,out,nxt|
  guard_value += 1
  [from,event,from+'_'+event+'_'+guard_value.to_s,out,nxt]}
mirror_trans.each{|from,event,guard,out,nxt|
	puts [from,event,guard,out,nxt].join(',')}
