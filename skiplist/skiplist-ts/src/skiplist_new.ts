export class Node {
  elem: number;
  forward: (Node | undefined)[];
  span: number[];

  constructor(elem: number, level: number) {
    this.elem = elem;

    this.forward = new Array(level+1).fill(undefined);
    this.span = new Array(level+1).fill(0);
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
    const rLevel = this.randomLevel();
    const newNode = new Node(elem, rLevel);

    for (let i = this.level; i >= 0;i--) {
      let spanSum = 0;
      while(current.forward[i] && current.forward[i]!.elem < elem) {
        spanSum += current.span[i];
        current = current.forward[i]!;
      }

      update[i] = current;

      if (i < MAX_LEVEL-1) newNode.span[i+1] = spanSum;
    }

    newNode.span[0] = 1;
    current = current.forward[0]!;

    if (current && current.elem === elem) return;

    if (rLevel > this.level) {
      for (let i = this.level+1; i<rLevel+1;i++) update[i] = this.root;

      this.level = rLevel;
    }

    let spanSum = 0;
    for (let i=0;i<=rLevel;i++) {
      const currentUpdate = update[i];

      newNode.forward[i] = update[i].forward[i];
      update[i].forward[i] = newNode;

      const oldSpan = currentUpdate.span[i];
      const newNodeSpan = newNode.span[i];

      spanSum += newNodeSpan;

      currentUpdate.span[i] = spanSum;
      const newSpan = oldSpan - spanSum;

      newNode.span[i] = newSpan < 0 ? 0 : newSpan;
    }
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

  deleteOne(elem: number): void {
    let current = this.root;

    let update = new Array(MAX_LEVEL+1).fill(undefined);
    for (let i = this.level; i >= 0;i--) {
      while(current.forward[i] && current.forward[i]!.elem < elem) {
        current = current.forward[i]!;
      }

      if (current.forward[i]!.elem !== elem) {
        update[i] = undefined;
        continue;
      }

      update[i] = current;
    }

    if (current.forward.every((node: Node|undefined) => !node)) return;

    for (let i=0;i<=this.level;i++) {
      if (!update[i]) continue;

      update[i].forward[i] = update[i].forward[i].forward[i];
    }
  }

  // display
  display() {
    for (let i=this.level;i >= 0;i--) {
      let current: Node | undefined = this.root.forward[i];
      console.log(`Level ${i}] `);
      while (current) {
        console.log(`${current.elem} `);
        current = current.forward[i];
      }
      console.log('\n');
    }
  }

  // rank
  rank(elem: number): number {
    let current = this.root;
    let rank = 0;

    while (current) {
      if (current.elem === elem) return rank;

      let i = MAX_LEVEL;
      for (i;i>=0;i--) {
        if (!current.forward[i]) continue;

        if (i === 0 && 
          current.elem < elem && 
          current.forward[0]!.elem > elem) return 0;

        if (elem >= current.forward[i]!.elem) {
          current = current.forward[i]!;
          rank += current.span[i];
          break;
        }
      }

      if (current === this.root) return 0;
    }

    return rank;
  }
}