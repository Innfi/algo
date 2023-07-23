import assert from 'assert';

import { SkipList } from '../src/main';

/*
TODO
--------------------------------------------------------------------------------
add node to skiplist
delete node from skiplist

DONE
--------------------------------------------------------------------------------
start with a simple double linked list
add more milestones with a binary fashion
*/

describe('skiplist', () => {
  it('initial condition: root is empty', () => {
    const instance = new SkipList();

    assert.strictEqual(instance.root === undefined, true);
  });

  it('insert] first node', () => {
    const instance = new SkipList();

    instance.insert(1);

    assert.strictEqual(instance.root.elem, 1);
  });

  it('insert] second node', () => {
    const instance = new SkipList();

    instance.insert(1);
    instance.insert(2);

    assert.strictEqual(instance.root.next.elem, 2);
    assert.strictEqual(instance.root.next.prev.elem, 1);
  });

  it('insert] third node', () => {
    const instance = new SkipList();

    instance.insert(1);
    instance.insert(2);
    instance.insert(3);

    assert.strictEqual(instance.root.elem, 1);
    assert.strictEqual(instance.root.next.next.elem, 3);
    assert.strictEqual(instance.root.next.next.prev.elem, 2);
  });
});

const randomSeed = (): number => {
  return Math.floor(Math.random() * 10)+1;
};

const createPreset = (len: number): SkipList => {
  const instance = new SkipList();
  let seed = 1;

  for(let i=0;i<len;i++) {
    seed += randomSeed();

    instance.insertSimple(seed);
  }

  instance.createLevelPreset();

  return instance;
};

describe('skiplist: preset', () => {
  it('check basic property of the linked list', () => {
    const instance = new SkipList();
    const len = 10;
    let seed = 1;

    for(let i=0;i<len;i++) {
      seed += randomSeed();

      instance.insertSimple(seed);
    }

    let root = instance.root;
    while(root.next != undefined) {
      assert.strictEqual(root.elem < root.next.elem, true);
      root = root.next;
    }
  });
  
  it('search node via levels', () => {
    const instance: SkipList = createPreset(100);

    const findResult = instance.find(50);

    assert.strictEqual(findResult !== undefined, true);
  });
});