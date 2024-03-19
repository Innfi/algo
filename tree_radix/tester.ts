import assert from 'assert';

class RadixNode {
  nodes: { [key: string]: RadixNode }

  constructor() {
    this.nodes = {};
  }
}

class RadixTree {
  root: { [key: string]: RadixNode };

  constructor() {
    this.root = {};
  }

  insert(input: string): void {
    for (const key of Object.keys(this.root)) {
      const subset = this.findSubset(input, key);
      if (subset.length <= 0) continue;
     
      const remainFromInput = input.substring(0, subset.length);
      const remainFromKey = input.substring(0, subset.length);

      const previousNode = this.root[key];

      this.root[key] = undefined;

      this.root[subset] = new RadixNode();
      this.root[subset].nodes[remainFromInput] = new RadixNode();
      this.root[subset].nodes[remainFromKey] = previousNode;

      return;
    }

    this.root[input] = new RadixNode();
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

    assert.deepStrictEqual(node.nodes, {});
  });

  it ('insert first string', () => {
    const instance = new RadixTree();

    const input = 'first';
    instance.insert(input);

    assert.strictEqual(instance.root[input] != undefined, true)
  });

  it ('insert second string', () => {
    const instance = new RadixTree();

    const first = 'first';
    const second = 'second';
    instance.insert(first);
    instance.insert(second);

    assert.strictEqual(instance.root[first] != undefined, true);
    assert.strictEqual(instance.root[second] != undefined, true);
  });

  it ('javascript substring', () => {
    assert.strictEqual('first'.substring(0, 3), 'fir');
  });

  // it ('insert first and firm', () => {
  //   const instance = new RadixTree();

  //   const input: string[] = ['first', 'firm'];
  //   input.forEach((v) => instance.insert(v));

  //   assert.strictEqual(input.every((v) => instance.findExactMatch(v)), true);
  // });
});
