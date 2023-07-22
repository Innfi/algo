export const LEVEL_MAX = 4;

type LevelType = DoubleNode | undefined;

export class DoubleNode {
  elem: number;
  prev: DoubleNode;
  next: DoubleNode;

  nextMilestone: DoubleNode;

  level: LevelType[];

  constructor(elem: number) {
    this.elem = elem;
    this.level = [];

    for(let i=0;i<LEVEL_MAX;i++) this.level.push(undefined);
  }
}

export class SkipList {
  root: DoubleNode;
  current: DoubleNode;
  len: number;

  constructor() {
    this.len = 0;
  }

  insertSimple(elem: number): void {
    const newNode = new DoubleNode(elem);

    if (!this.root) {
      this.root = newNode;
      this.current = newNode;
      this.len += 1;
      return;
    }

    this.current.next = newNode;
    newNode.prev = this.current;

    this.current = this.current.next;
    this.len += 1;
  }

  createLevelPreset(): void {
    console.log(`createLevelPreset] max: ${LEVEL_MAX}, elem len: ${this.len}`);

    for(let current = 0;current < LEVEL_MAX;current++) {
      this.createLevelByIndex(current);
      console.log('-----------------------------');
    }
  }

  createLevelByIndex(index: number) {
    const span = Math.floor(this.len / Math.pow(2, index+1));
    console.log(`createLevelByIndex] index: ${index}, span: ${span}`);

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
      console.log(`createLevelByIndex] currentLevelLen: ${currentLevelLen}, currentSpan: ${currentSpan}`);

      currentNode.level[index] = nextNode;
      console.log(`createLevelByIndex] index: ${index}, currentNode: ${currentNode.elem}, nextNode: ${nextNode.elem}`);

      currentNode = nextNode;
      currentSpan += span;
    }
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