
export class DoubleNode {
  elem: number;
  prev: DoubleNode;
  next: DoubleNode;

  nextMilestone: DoubleNode;

  constructor(elem: number) {
    this.elem = elem;
  }
}

export class SkipList {
  root: DoubleNode;
  len: number;

  constructor() {
    this.len = 0;
  }

  insert(elem: number): void {
    const newNode = new DoubleNode(elem);

    if (!this.root) {
      this.root = newNode;
      this.len += 1;
      return;
    }

    let current = this.root;
    while (current.next) {
      current = current.next;
    }

    current.next = newNode;
    newNode.prev = current;
    this.len += 1;
  }

  insertBulk(elems: number[]): void {
    elems.forEach((elem: number) => { this.insert(elem) });

    this.linkMilestone(Math.floor(this.len/2));
  }

  private linkMilestone(offset: number): void {
    let currentIndex = 0;
    let currentNode = this.root;

    while (currentIndex < offset-1) {
      currentNode = currentNode.next;
      currentIndex += 1;
    }

    this.root.nextMilestone = currentNode;
  }
}