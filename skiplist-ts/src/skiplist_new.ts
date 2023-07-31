export class Node {
  elem: number;
  forward: (Node | undefined)[];

  constructor(elem: number, level: number) {
    this.elem = elem;

    this.forward = new Array(level=1).fill(undefined);
  }
}

const MAX_LEVEL = 8;
const LEVEL_PROB = 0.25;

export class SkipList {
  root: Node;
  level: number;

  constructor() {
    this.root = new Node(-1, MAX_LEVEL);
    this.level = 0;
  }

  // insert
  insert(elem: number): void {
    let current = this.root;

    let update = new Array(MAX_LEVEL+1).fill(undefined);

    for (let i = this.level; i >= 0;i--) {
      while(current.forward[i] && current.forward[i]!.elem < elem) {
        current = current.forward[i]!;
      }

      update[i] = current;
    }

    current = current.forward[0]!;

    if (current && current.elem === elem) return;

    const rLevel = this.randomLevel();

    if (rLevel > this.level) {
      for (let i = this.level+1; i<rLevel+1;i++) update[i] = this.root;

      this.level = rLevel;
    }

    const newNode = new Node(elem, rLevel);

    for (let i=0;i<=rLevel;i++) newNode.forward[i] = update[i].forward[i];
  }

  private randomLevel() {
    let r = Math.random()
    let level = 0;

    while(r < LEVEL_PROB && level < MAX_LEVEL) {
      level++;
      r = Math.random();
    }

    return level;
  }

  // display
  // display() {
  //   for (let i=this.level;i >= 0;i--) {
  //     let current = this.root;
  //   }
  // }
}