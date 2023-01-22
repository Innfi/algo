import { DoubleLink, DoubleNode } from "../link/double_link";

export class LRUCache extends DoubleLink {
	readonly limit: number;

	constructor(limit?: number) {
		super();
		this.limit = limit;
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

		const temp = this.root;

		this.root = new DoubleNode(data);
		this.root.next = temp;
		temp.prev = this.root;
	}
}