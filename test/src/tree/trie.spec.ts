import { Trie, TrieNode } from "./trie";

describe('trie', () => {
  it('trie node: save node by key', () => {
    const node = new TrieNode();
    
    node.set('temp', undefined);

    expect(Object.keys(node.nodeMap).includes('temp')).toBeTruthy();
  });

  it('trie node: isEmpty', () => {
    const node = new TrieNode();

    expect(node.isEmpty()).toBeTruthy();

    node.set('temp', undefined);

    expect(node.isEmpty()).toBeFalsy();
  });

  it('insert initial node', () => {
    const instance = new Trie();

    const input = 'aabb';
    instance.insert(input);

    expect(instance.exists(input)).toBeTruthy();
  });

  it('insert a stranger', () => {
    const instance = new Trie();

    instance.insert('aabb');
    instance.insert('dbca');

    expect(instance.exists('aabb')).toBeTruthy();
    expect(instance.exists('dbca')).toBeTruthy();
  });

  it('insert a sibling', () => {
    const instance = new Trie();

    instance.insert('aabb');
    // instance.insert('aacc');

    expect(instance.exists('aabb')).toBeTruthy();
    //expect(instance.exists('aacc')).toBeTruthy();
  });
  
});