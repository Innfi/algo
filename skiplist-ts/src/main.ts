export const LEVEL_MAX = 5;

type LevelType = DoubleNode | undefined;

export class DoubleNode {
  elem: number;
  prev: DoubleNode;
  next: DoubleNode;

  level: LevelType[];

  constructor(elem: number) {
    this.elem = elem;
    this.level = [];

    for(let i=0;i<LEVEL_MAX;i++) this.level.push(undefined);
  }
}

export interface FindResult {
  err: 'match' | 'next' | 'fail';
  targetNode: DoubleNode | undefined;
}

export class SkipList {
  root: DoubleNode;
  current: DoubleNode;
  len: number;

  constructor() {
    this.len = 0;
  }

  // pushBack
  pushBack(elem: number): number {
    const newNode = new DoubleNode(elem);

    if (!this.root) {
      this.root = newNode;
      this.current = newNode;
      this.len += 1;
      return this.len;
    }

    this.current.next = newNode;
    newNode.prev = this.current;

    this.current = this.current.next;
    this.len += 1;

    return this.len;
  }

  // insert
  insert(elem: number): number {
    const findResult = this.find(elem);
    if (findResult.err === 'fail') return this.pushBack(elem);
    if (findResult.err === 'match') return this.len;

    const targetNode = findResult.targetNode!;
    const nextNode = targetNode.next;
    const newNode = new DoubleNode(elem);

    targetNode.next = newNode;
    newNode.prev = targetNode;

    newNode.next = nextNode;
    nextNode.prev = newNode;

    this.len += 1;

    return this.len;
  }

  // createLevelPreset
  createLevelPreset(): void {
    for(let current = 0;current < LEVEL_MAX;current++) {
      this.createLevelByIndex(current);
    }
  }

  private createLevelByIndex(index: number) {
    const span = Math.floor(this.len / Math.pow(2, index+1));

    let currentNode = this.root;
    let nextNode = this.root;
    let currentLevelLen = 0;
    let currentSpan = span;

    while (true) {
      if (currentSpan >= this.len) break;

      while (currentLevelLen < currentSpan) {
        nextNode = nextNode.next;
        currentLevelLen += 1;
      }

      currentNode.level[index] = nextNode;

      currentNode = nextNode;
      currentSpan += span;
    }
  }

  // find
  find(elem: number): FindResult {
    let current = this.root;

    while(true) {
      if (!current) return { err: 'fail', targetNode: undefined };
      if (current.elem > elem) return { err: 'fail', targetNode: undefined };
      if (current.elem === elem) break;

      let index = 0;
      for (index;index<LEVEL_MAX;index++) {
        if (!current.level[index]) continue;
        if (current.level[index]!.elem > elem) continue;

        current = current.level[index]!;

        break;
      }

      if (current.elem < elem && elem < current.next.elem) {
        return { err: 'next', targetNode: current };
      }

      if (index >= LEVEL_MAX) current = current.next;
    }

    return { err: 'match', targetNode: current };
  }

  // deleteElem
  deleteElem(elem: number): number {
    if (this.root.elem === elem) return this.deleteRoot();

    let current = this.root;

    for (let i=0;i<current.level.length;i++) {
      this.tryDeleteLevel(current.level[i], i, elem);
    }

    while(current.next) {
      if (current.next.elem === elem) break;

      current = current.next;
    }

    if (!current.next) return this.len;

    current.next = current.next.next;
    this.len -= 1;   

    return this.len;
  }

  private deleteRoot(): number {
    const root = this.root;
    const newRoot = root.next;

    for (let i=0;i<root.level.length;i++) newRoot.level[i] = root.level[i];

    this.root = newRoot;

    return this.len-1;
  }

  private tryDeleteLevel(level: LevelType, index: number, elem: number): void {
    if (!level) return;

    let current = level!;
    while(current.level[index]) {
      if (current.level[index]!.elem === elem) break;

      current = current.level[index]!;
    }

    if (!current.level[index]) return;

    current.level[index] = current.level[index]!.level[index];
    return;
  }
}