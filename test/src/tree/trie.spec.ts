import { Trie, TrieNode } from "./trie";

describe('trie', () => {
  it('trie node: save node by key', () => {
    const node = new TrieNode();
    
    node.setNode('temp', new TrieNode());

    expect(node.nodeMap.has('temp')).toBeTruthy();
  });

  it('trie node: isEmpty', () => {
    const node = new TrieNode();

    expect(node.isEmpty()).toBeTruthy();

    node.setNode('temp', new TrieNode());

    expect(node.isEmpty()).toBeFalsy();
  });

  it('insert initial node', () => {
    const instance = new Trie();

    const input = 'aabb';
    instance.insert(input);

    expect(instance.exists(input)).toBeTruthy();
  });

  it('trie.exists: check subset', () => {
    const instance = new Trie();

    instance.insert('abcd');

    expect(instance.exists('abc')).toBeTruthy();
  });

  it('trie.exists: check superset', () => {
    const instance = new Trie();

    const input = 'aabb';
    instance.insert(input);

    expect(instance.exists('aabbcc')).toBeFalsy();
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
    instance.insert('aacc');

    expect(instance.exists('aabb')).toBeTruthy();
    expect(instance.exists('aacc')).toBeTruthy();
  });
  
  it('insert a single letter', () => {
    const instance = new Trie();

    instance.insert('a');
    instance.insert('b');

    expect(instance.root.nodeMap.has('a')).toBeTruthy();

    expect(instance.exists('a')).toBeTruthy();
    expect(instance.exists('b')).toBeTruthy();
  });

  it('getRelatives', () => {

  });
});