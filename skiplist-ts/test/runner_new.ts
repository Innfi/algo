import assert from 'assert';

import { SkipList } from '../src/skiplist_new';

describe('skiplist_new', () => {
  it('span] initial state ', () => {
    const instance = new SkipList();

    assert.strictEqual(instance.root.span.every((v) => v === 0), true);
  });

  it('span] add a node and its span is one', () => {
    const instance = new SkipList();

    instance.insert(1);
    instance.insert(2);

    assert.strictEqual(instance.root.span[0], 1);
    assert.strictEqual(instance.root.forward[0]?.span, 1);
    assert.strictEqual(instance.root.forward[0]?.forward[0]?.span, 0);
  });

  it('span] tests when level expands', () => {
    const instance = new SkipList();

    let elem = 1;
    while (elem < 100) {
      instance.insert(elem);

      if (instance.root.forward[1]) break;

      elem++;
    }

    instance.display();
    // console.log(`forward[1]: ${instance.root.forward[1]?.span}`);

    assert.strictEqual(instance.root.forward[1]!.span[1] > 1, true);
  });
});