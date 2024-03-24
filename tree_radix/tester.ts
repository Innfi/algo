import assert from 'assert';

const toPrefix = (input: string, key: string): string => {
  let common = "";
  let len = input.length < key.length ? input.length : key.length;
  for (let i=0;i<len;i++) {
    if (input[i] !== key[i]) break;

    common += input[i];
  }

  return common;
}

const removePrefix = (input: string, prefix: string): string => {
  return input.substring(prefix.length, input.length);
};


class RadixNode {
  public key: string;
  public nodes: RadixNode[];

  constructor(key: string) {
    this.key = key;
    this.nodes = [];
  }

  findNode(key: string): RadixNode {
    return this.nodes.find((v) => v.key === key);
  }

  findNodeByPrefixMatch(key: string): RadixNode {
    return this.nodes.find((v) => v.key[0] === key[0]);
  }

  addNode(newNode: RadixNode): void {
    this.nodes.push(newNode);
  }

  print(): void {
    this.nodes.forEach((v) => {
      console.log(`${this.key} -> ${v.key}`);
      v.print();
    });
  }
}

class RadixTree {
  root: RadixNode;

  constructor() {
    this.root = new RadixNode('');
  }

  insert(input: string): void {
    const nodeByPrefix: RadixNode = this.root.findNodeByPrefixMatch(input);
    if (!nodeByPrefix) {
      this.root.addNode(new RadixNode(input));
      return;
    }

    this.insertRecursive(nodeByPrefix, input);
  }

  private insertRecursive(node: RadixNode, input: string): void {
    // assume node.key.len > subset.len

    const prefix = toPrefix(input, node.key);
    const remainFromInput = removePrefix(input, prefix);
    const remainFromKey = removePrefix(node.key, prefix);

    node.key = prefix;

    if (remainFromKey.length > 0) {
      const newChild = new RadixNode(remainFromKey);
      newChild.nodes = node.nodes;

      node.nodes = [newChild];
      node.nodes.push(new RadixNode(remainFromInput));

      return;
    } 

    const nodeByPrefix: RadixNode = node.findNodeByPrefixMatch(remainFromInput);
    if (!nodeByPrefix) return;

    this.insertRecursive(nodeByPrefix, remainFromInput);
  }

  print(): void {
    this.root.print();
  }

  findExactMatch(input: string): boolean {
    return this.findExactMatchRecursive(input, this.root);
  }

  private findExactMatchRecursive(input: string, node: RadixNode): boolean {
    const current = node.findNodeByPrefixMatch(input);
    if (!current) return false;

    const remain = removePrefix(input, toPrefix(input, current.key));
    if (remain.length <= 0) return true;

    return this.findExactMatchRecursive(remain, current);
  }
}

describe('test', () => {
  it ('radix node: initial state', () => {
    const key = 'initial';
    const node = new RadixNode(key);

    assert.strictEqual(node.key, key);
    assert.deepStrictEqual(node.nodes, []);
  });

  it ('first', () => {
    const instance = new RadixTree();

    const input = 'first';
    instance.insert(input);

    assert.strictEqual(instance.root.nodes.findIndex((v) => v.key === input) >= 0, true);
  });

  it ('first and firm', () => {
    const instance = new RadixTree;

    ['first', 'firm'].forEach((v) => instance.insert(v));

    const subset = 'fir';
    const remainFirst = 'st';
    const remainSecond = 'm';

    const commonNode = instance.root.nodes.find((v) => v.key === subset);
    assert.strictEqual(commonNode.key, subset);

    assert.strictEqual(commonNode.findNode(remainFirst) !== undefined, true);
    assert.strictEqual(commonNode.findNode(remainSecond) !== undefined, true);
  });

  it ('first, firm, and final', () => {
    const instance = new RadixTree();

    [
      'first', 
      'firm', 
      'final',
    ].forEach((v) => instance.insert(v));

    const subset1 = 'fi';
    const subset2 = 'r';
    const subset3 = 'nal';

    const node1 = instance.root.nodes.find((v) => v.key === subset1);
    assert.strictEqual(node1 !== undefined, true);

    const node2 = node1.findNode(subset2);
    assert.strictEqual(node2 !== undefined, true);
    assert.strictEqual(node2.findNode('st') !== undefined, true);
    assert.strictEqual(node2.findNode('m') !== undefined, true);

    const node3 = node1.findNode(subset3);
    assert.strictEqual(node3 !== undefined, true);
  });

  it ('first, firm, final, and finite', () => {
    const instance = new RadixTree();

    [
      'first', 
      'firm', 
      'final',
      'finite',
    ].forEach((v) => instance.insert(v));

    instance.print();
  });

  it ('radix tree: findExactMatch', () => {
    const instance = new RadixTree();

    [
      'first', 
      'firm', 
      'final',
      'finite',
    ].forEach((v) => instance.insert(v));

    assert.strictEqual(instance.findExactMatch('finite'), true);
    assert.strictEqual(instance.findExactMatch('fortnight'), false);
    assert.strictEqual(instance.findExactMatch('f'), true);
  });
});