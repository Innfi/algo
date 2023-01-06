

export class SingleNode {
  public next: SingleNode;

  public data: string;

  constructor(input: string = '') {
    this.next = undefined;
    this.data = input;
  }
}

export class SingleLink {
  public root: SingleNode;
  public tail: SingleNode;

  constructor() {
    this.root = undefined;
    this.tail = undefined;
  }

  public insert(input: string = ''): void {
    if (!this.root) {
      this.root = new SingleNode(input);
      this.tail = this.root;
      return;
    }

    const newNode = new SingleNode(input);

    this.tail.next = newNode;
    this.tail = newNode;
  }

  public deleteOne(input: string): void {
    let prevNode = this.root;

    if (prevNode.data === input) {
      this.root = prevNode.next;
      return;
    }

    while (prevNode.next !== undefined) {
      if (prevNode.next.data === input) {
        prevNode.next = prevNode.next.next;
        return;
      }

      prevNode = prevNode.next;
    }

    prevNode = undefined;
  }
}