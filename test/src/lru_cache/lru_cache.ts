import { DoubleLink, DoubleNode } from "../link/double_link";

export class LRUCache extends DoubleLink {
	private len: number;
	readonly limit: number;

	constructor(limit: number = 10) {
		super();
		this.limit = limit;
		this.len = 0;
	}

	insert(data: string): void {
		if (!this.root) {
			super.insert(data);
			this.len += 1;
			return;
		}

		const temp = this.root;

		this.root = new DoubleNode(data);
		this.root.next = temp;
		temp.prev = this.root;

		this.len += 1;

		if (this.nodeLen() > this.limit) this.deleteTail();
	}

	private deleteTail(): void {
		this.tail.prev.next = undefined;
		this.tail = this.tail.prev;
		this.len -= 1;
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
		return this.len;
	}
}