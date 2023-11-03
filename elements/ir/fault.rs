internal {
	float prob
}
fn init(prob) {
	prob = 0.9
}

fn req() {
	Match(randomf(0,1) < prob) {
		true => {
			send(rpc_req, NET)
		}
		false => {
			send(err("fault injected"), APP) 
		}
	} 
}

fn resp() {
    send(rpc_resp, APP)	
}