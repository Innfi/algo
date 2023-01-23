import { DoubleLink, DoubleNode } from "../link/double_link";

export class LRUCache extends DoubleLink {
	readonly limit: number;

	constructor(limit: number = 10) {
		super();
		this.limit = limit;
	}

	insert(data: string): void {
		if (!this.root) {
			super.insert(data);
			return;
		}

		const temp = this.root;

		this.root = new DoubleNode(data);
		this.root.next = temp;
		temp.prev = this.root;

		if (this.nodeLen() > this.limit) this.deleteTail();
	}

	private deleteTail(): void {
		let current = this.root;

		while(current) {
			if (!current.next) break;

			current = current.next;
		}

	  if (current.prev) current.prev.next = undefined;
		current = undefined;
	}

	get(data: string): string {
		const targetNode = this.find(data);

		if(!targetNode) return undefined;

		const targetData = targetNode.data;
		this.moveToRoot(data);
		
		return targetData;
	}

	private find(data: string): DoubleNode {
		let current = this.root;

		while(current) {
			if (current.data === data) return current;

			current = current.next;
		}

		return undefined;
	}

	private moveToRoot(data: string): void {
		this.deleteNode(data);

		this.insert(data);
	}

	nodeLen(): number {
		let current = this.root;
		let len = 0;

		while(current) {
			len += 1;
			current = current.next;
		}

		return len;
	}
}