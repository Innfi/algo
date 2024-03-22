import assert from 'assert';

class RadixNode {
  nodes: Map<string, RadixNode>;

  constructor() {
    this.nodes = new Map();
  }
}

class RadixTree {
  root: Map<string, RadixNode>;

  constructor() {
    this.root = new Map();
  }

  insert(input: string): void {
    let current = this.root;

    for (const key of [...current.keys()]) {
      const subset = this.findSubset(input, key);

      if (subset.length <= 0) continue;
     
      const remainFromInput = input.substring(subset.length, input.length);
      const remainFromKey = key.substring(subset.length, key.length);

      const previousNode = current.get(key);
      current.delete(key);

      current.set(subset, new RadixNode());
      current.get(subset).nodes.set(remainFromInput, new RadixNode());
      current.get(subset).nodes.set(remainFromKey, previousNode);

      return;
    }

    // this.root[input] = new RadixNode();
    this.root.set(input, new RadixNode());
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

  findExactMatch(key: string): boolean {
    return false;
  }
}

describe('test', () => {
  it ('radix node: initial state', () => {
    const node = new RadixNode();

    assert.deepStrictEqual(node.nodes != undefined, true);
  });

  it ('insert first string', () => {
    const instance = new RadixTree();
    const input = 'first';

    instance.insert(input);
  
    assert.strictEqual(instance.root.has(input), true);
  });

  it ('insert second string', () => {
    const instance = new RadixTree();

    const first = 'first';
    const second = 'second';

    instance.insert(first);
    instance.insert(second);

    assert.strictEqual(instance.root.has(first), true);
    assert.strictEqual(instance.root.has(second), true);
  });

  it ('javascript substring', () => {
    assert.strictEqual('first'.substring(0, 3), 'fir');
  });

  it ('insert first and firm', () => {
    const instance = new RadixTree();
    const input: string[] = ['first', 'firm'];
  
    input.forEach((v) => instance.insert(v));
    assert.strictEqual(instance.root.has('fir'), true);

    const child = instance.root.get('fir');
    assert.strictEqual(child.nodes.has('m'), true);
    assert.strictEqual(child.nodes.has('st'), true);
  });

  it ('first, firm and final', () => {
    const instance = new RadixTree();

    ['first', 'firm', 'final'].forEach((v) => instance.insert(v));

    assert.strictEqual(instance.root.has('fi'), true);

    const intermediate = instance.root.get('fi');
    assert.strictEqual(intermediate.nodes.has('nal'), true);
    assert.strictEqual(intermediate.nodes.has('r'), true);

    const last = intermediate.nodes.get('r');
    assert.strictEqual(last.nodes.has('m'), true);
    assert.strictEqual(last.nodes.has('st'), true);
  });

  it ('first, firm, final, and finite', () => {
    const instance = new RadixTree();

    ['first', 'firm', 'final', 'finite'].forEach((v) => instance.insert(v));
  });
});

