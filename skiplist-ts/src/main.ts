
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

  insert(elem: number): void {
    const newNode = new DoubleNode(elem);

    if (!this.root) {
      this.root = newNode;
      return;
    }

    let current = this.root;
    while (current.next) {
      current = current.next;
    }

    current.next = newNode;
    newNode.prev = current;
  }

  insertBulk(elems: number[]): void {
    elems.forEach((elem: number) => { this.insert(elem) });
  }
}