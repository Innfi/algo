import assert from 'assert';

import { SkipList } from '../src/main';

/*
TODO
--------------------------------------------------------------------------------
add a milstone in the middle
add more milestones with a binary fashion
change elements of the list
change milestones according to pivots


DONE
--------------------------------------------------------------------------------
start with a simple double linked list
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

  it('insert] set milestone', () => {
    const instance = new SkipList();

    instance.insertBulk([1,2,3,4,5,6,7,8,9]);

    const root = instance.root;

    assert.strictEqual(root.nextMilestone.elem, 5);
  });
});