
pub struct SingleNode {
	name: String,
}

impl SingleNode {
	pub fn new(input: &str) -> Self {
		Self {
			name: input.into()
		}
	}

	pub fn dummy_function(&self) -> String {
		self.name.clone()
	}
}

pub struct SingleLinkedList {
	
}

impl SingleLinkedList {
	pub fn get_root(&self) -> SingleNode {
		SingleNode::new("dummy")
	}
}