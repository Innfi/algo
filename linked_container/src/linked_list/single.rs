// pub struct SingleNode {
// 	pub name: String,
// 	pub next: Option<Box<SingleNode>>,
// }

// impl SingleNode {
// 	pub fn new(input: String) -> Self {
// 		Self {
// 			name: input,
// 			next: None,
// 		}
// 	}
// }

// pub struct SingleLinkedList<'a> {
// 	root: Option<Box<SingleNode>>,
// 	current: Option<&'a Box<SingleNode>>,
// }

// impl<'a> SingleLinkedList<'a> {
// 	pub fn new() -> Self {
// 		Self {
// 			root: None,
// 			current: None,
// 		}
// 	}

// 	pub fn get_root(&self) -> Option<&Box<SingleNode>> {
// 		return self.root.as_ref();
// 	}

// 	pub fn insert(&mut self, input: String) -> Result<(), &'static str> {
// 		if self.root.is_none() {
// 			self.root = Some(Box::<SingleNode>::new(
// 				SingleNode::new(input)
// 			));
// 			self.current = self.root;

// 			return Ok(());
// 		}

// 		Ok(())
// 	}
// }