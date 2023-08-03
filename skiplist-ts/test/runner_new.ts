import assert from 'assert';

import { SkipList } from '../src/skiplist_new';

describe('skiplist_new', () => {
  // it('basic insert / display', () => {
  //   const instance = new SkipList();

  //   instance.insert(3);
  //   instance.insert(6);
  //   instance.insert(7);
  //   instance.insert(9);
  //   instance.insert(12);
  //   instance.insert(19);
  //   instance.insert(17);
  //   instance.insert(26);
  //   instance.insert(21);
  //   instance.insert(29);
  //   instance.insert(28);
  //   instance.insert(22);
  //   instance.insert(23);
  //   instance.insert(24);
  //   instance.insert(35);
  //   instance.insert(55);

  //   assert.strictEqual(instance.root.elem, -1);

  //   instance.display();
  //   instance.deleteOne(21);
  //   instance.display();
  // });

  it('span] initial state ', () => {
    const instance = new SkipList();

    assert.strictEqual(instance.rootSpan.every((v) => v === 0), true);
  });

  it('span] add a node and its span is one', () => {
    const instance = new SkipList();

    instance.insert(1);
    instance.insert(2);

    assert.strictEqual(instance.rootSpan[0], 1);
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

    assert.strictEqual(instance.root.forward[1]!.span > 1, true);
  });
});