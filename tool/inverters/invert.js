let transitions = [
	'INIT,Open,1,OpenOk,Process',
	'INIT,Open,2,OpenFail,FINAL',
	'Process,Add,1,AddOK,Process',
	'Process,Add,2,RetryAdd,Process',
	'Process,Add,3,AddFail,FINAL',
	'Process,Close,1,CloseOk,FINAL',
	'Process,Close,2,CloseFail,FINAL'];

let mirror_trans = new Set;

transitions.forEach((arow, index) => {
	let [from, event, guard, out, nxt] = arow.split(',');
	if (from === "INIT")
		mirror_trans.add(['INIT', 'callin', 'NOGUARD_YET', event, 'INIT' + '_' + event].join(','));
	if (nxt === "FINAL")
		mirror_trans.add([from + '_' + event, out, 'NOGUARD_YET', '-', 'FINAL'].join(','));
});
transitions.forEach(arow_left => {
	let [l_from, l_event, l_guard, l_out, l_nxt] = arow_left.split(',');
	transitions.forEach(arow_right => {
		let [r_from, r_event, r_guard, r_out, r_nxt] = arow_right.split(',');
		if (l_nxt === r_from)
			mirror_trans.add([l_from + '_' + l_event, l_out, 'NOGUARD_YET', r_event, r_from + '_' + r_event].join(','));
	});
});
let final_csv = [...mirror_trans].map((arow, index) => {
	let [from, event, guard, out, nxt] = arow.split(',');
	return [from, event, from + '_' + event + '_' + (index + 1), out, nxt].join(',');
}).join('\n');

console.log(final_csv);
