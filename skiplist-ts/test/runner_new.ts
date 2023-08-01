import assert from 'assert';

import { SkipList } from '../src/skiplist_new';

describe('skiplist_new', () => {
  it('basic insert / display', () => {
    const instance = new SkipList();

    instance.insert(3);
    instance.insert(6);
    instance.insert(7);
    instance.insert(9);
    instance.insert(12);
    instance.insert(19);
    instance.insert(17);
    instance.insert(26);
    instance.insert(21);
    instance.insert(25);

    assert.strictEqual(instance.root.elem, -1);

    instance.display();

    instance.deleteOne(21);

    instance.display();
  });
});