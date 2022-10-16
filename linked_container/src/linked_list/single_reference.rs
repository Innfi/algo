// https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676

enum Link<T> {
	None,
	Tail { item: T },
	Link { item: T, next: Box<Link<T>> }
}

impl<T> Link<T> where T:Copy {
	pub fn push(&mut self, x: T) {
		match self {
			Self::None => self.to_tail(x),
			Self::Tail { .. } => self.to_link(x),
			Self::Link { next, .. } => next.push(x)
		};
	}

	fn to_tail(&mut self, it: T) {
		*self = match self {
			Self::None => Self::Tail { item: it },
			Self::Link { item:_, next:_ } => Self::Tail { item: it },
			_ => panic!("cannot convert to tail")
		}
	}

	fn to_link(&mut self, x: T) {
		*self = match self {
			Self::Tail { item } => {
				Self::Link { item: *item, next: Box::new(Self::Tail { item: x }) }
			},
			_ => panic!("connot convert to link")
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		match self {
			Self::None => None,
			Self::Tail { item } => {
				let item = *item;
				self.to_none();
				
				Some(item)
			},
			Self::Link { item, next } => {
				let mut n = Box::new(Self::None);
				let item = *item;
				std::mem::swap(next, &mut n);
				self.to_next(*n);

				Some(item)
			}
		}
	}

	fn to_none(&mut self) {
		*self = std::mem::replace(self, Link::None);
	}

	fn to_next(&mut self, next: Link<T>) {
		*self = next;
	}
}
