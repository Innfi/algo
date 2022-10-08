#[cfg(test)]
mod tests {
	use linked_container::linked_list::*;

	#[test]
	fn create_manager_instance() {
		let instance = SingleLinkedList {};

		let root = instance.get_root();

		assert_eq!(root.dummy_function().len() > 0, true)
	}
}