pub trait HashNodeTrait {
	fn to_payload_key(&self) -> &String;
}

pub trait HashFuntionTrait {
	fn to_hash(&self, key: &str) -> String;
}

pub struct VirtualNode<N> {
	pub payload: N,
	pub hash: String,
}

impl<N> VirtualNode<N> where N: HashNodeTrait {
	pub fn new(new_payload: N) -> Self {
		Self { payload: new_payload, hash: String::new() }
	}
}

// pub struct NewHashRing<T, F> 
// where F: HashFunctionTrait {
// 	rings: Vec
// }