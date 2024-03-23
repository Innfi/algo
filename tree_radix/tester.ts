import assert from 'assert';

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

  print(): void {
    this.nodes.forEach((v) => {
      console.log(`${this.key} -> ${v.key}`);
      v.print();
    });
  }
}

class RadixTree {
  root: RadixNode[];

  constructor() {
    this.root = [];
  }

  insert(input: string): void {
    for (const node of this.root) {
      const subset = this.findSubset(input, node.key);

      if (subset.length <= 0) continue;

      this.insertRecursive(node, input, subset);
      return;
    }

    this.root.push(new RadixNode(input));
  }

  private insertRecursive(node: RadixNode, input: string, subset: string): void {
    // assume node.key.len > subset.len

    const remainFromInput = input.substring(subset.length, input.length);
    const remainFromKey = node.key.substring(subset.length, node.key.length);

    node.key = subset;

    if (remainFromKey.length > 0) {
      const intermediate = new RadixNode(remainFromKey);
      intermediate.nodes = node.nodes;

      node.nodes = [intermediate];
      node.nodes.push(new RadixNode(remainFromInput));

      return;
    } 

    for (const subnode of node.nodes) {
      const innerSubset = this.findSubset(remainFromInput, subnode.key);

      if (innerSubset.length <= 0) continue;

      this.insertRecursive(subnode, remainFromInput, innerSubset);
      return;
    }
  }

  private findSubset(input: string, key: string): string {
    let common = "";
    let len = input.length < key.length ? input.length : key.length;
    for (let i=0;i<len;i++) {
      if (input[i] !== key[i]) break;

      common += input[i];
    }

    return common;
  }

  print(): void {
    this.root.forEach((v) => v.print());
  }

  findExactMatch(input: string): boolean {
    // FIXME: maybe root should be RadixNode?
    const tempNode = new RadixNode('');
    tempNode.nodes = this.root;

    return this.findExactMatchRecursive(input, tempNode);
  }

  private findExactMatchRecursive(input: string, node: RadixNode): boolean {
    const current = node.nodes.find((v) => {
      return this.findSubset(input, v.key).length > 0;
    });

    if (!current) return false;

    const remain = input.substring(this.findSubset(input, current.key).length, input.length);
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

    assert.strictEqual(instance.root.findIndex((v) => v.key === input) >= 0, true);
  });

  it ('first and firm', () => {
    const instance = new RadixTree;

    ['first', 'firm'].forEach((v) => instance.insert(v));

    const subset = 'fir';
    const remainFirst = 'st';
    const remainSecond = 'm';

    const commonNode = instance.root.find((v) => v.key === subset);
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

    const node1 = instance.root.find((v) => v.key === subset1);
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