

export class Node {
  public next: Node;

  public data: string;

  constructor(input: string = '') {
    this.next = undefined;
    this.data = input;
  }
}

export class SingleLink {
  public root: Node;
  public tail: Node;

  constructor() {
    this.root = undefined;
    this.tail = undefined;
  }

  public insert(input: string = ''): void {
    if (!this.root) {
      this.root = new Node(input);
      this.tail = this.root;
      return;
    }

    const newNode = new Node(input);

    this.tail.next = newNode;
    this.tail = newNode;
  }
}