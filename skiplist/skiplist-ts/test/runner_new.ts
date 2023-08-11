import assert from 'assert';

import { SkipList } from '../src/skiplist_new';

describe('skiplist_new', () => {
  // it('span] initial state ', () => {
  //   const instance = new SkipList();

  //   assert.strictEqual(instance.root.span.every((v) => v === 0), true);
  // });

  // it('span] add a node and its span is one', () => {
  //   const instance = new SkipList();

  //   instance.insert(1);
  //   instance.insert(2);

  //   assert.strictEqual(instance.root.span[0], 1);
  //   assert.strictEqual(instance.root.forward[0]?.span[0], 1);
  //   assert.strictEqual(instance.root.forward[0]?.forward[0]?.span[0], 0);
  // });

  // it('span] simple level', () => {
  //   const instance = new SkipList();

  //   for (let i=1;i<=10;i++) instance.insert(i);

  //   let current = instance.root;

  //   while (current.forward[0] !== undefined) {
  //     assert.strictEqual(current.span[0], 1)
  //     current = current.forward[0]!;
  //   }
  // });

  // it ('rank] initial', () => {
  //   const instance = new SkipList();

  //   assert.strictEqual(instance.root !== undefined, true);
  //   assert.strictEqual(instance.rank(5), 0);
  // });

  // it('rank] returns 0 for element not exist', () => {
  //   const instance = new SkipList();

  //   instance.insert(1);
  //   instance.insert(3);
  //   instance.insert(5);
  //   instance.insert(8);

  //   assert.strictEqual(instance.rank(7), 0);
  // });

  it ('rank] simple rank', () => {
    const instance = new SkipList();

    instance.insert(2);
    instance.insert(4);
    instance.insert(6);
    instance.insert(12);
    instance.insert(10);

    assert.strictEqual(instance.root.span[0], 1);
    assert.strictEqual(instance.root.forward[0]!.span[0], 1);
    assert.strictEqual(instance.root.forward[0]!.forward[0]!.span[0], 1);

    assert.strictEqual(instance.root.span[1], 4);
  });
});