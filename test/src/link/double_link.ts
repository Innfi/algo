

export class DoubleNode {
  public prev: DoubleNode;
  public next: DoubleNode;

  public data: string;

  constructor(data: string = '') {
    this.prev = undefined;
    this.next = undefined;

    this.data = data;
  }
}

export class DoubleLink {
  public root: DoubleNode;
  public tail: DoubleNode;

  constructor() {
    this.root = undefined;
  }

  insert(data: string): void {
    if (!this.root) {
      this.root = new DoubleNode(data);
      this.tail = this.root;
      return;
    }

    const newNode = new DoubleNode(data);
    this.tail.next = newNode;
    newNode.prev = this.tail;

    this.tail = this.tail.next;
  }

  deleteNode(data: string): void {
    if (this.root.data === data) {
      this.root.next.prev = undefined;
      this.root = this.root.next;
      return;
    }

    let targetNode = this.root;
    
    while(targetNode) {
      if (targetNode.data !== data) {
        targetNode = targetNode.next;
        continue;
      }

      if (targetNode.prev) targetNode.prev.next = targetNode.next;
      if (targetNode.next) targetNode.next.prev = targetNode.prev;

      return;
    }
  }
}