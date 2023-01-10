

export class TrieNode {
  nodeMap: Map<string, TrieNode>;

  constructor() {
    this.nodeMap = new Map<string, TrieNode>();
  }

  setNode(key: string, newNode: TrieNode): void {
    this.nodeMap.set(key, newNode);
  }

  addNode(key: string): void {
    this.nodeMap.set(key, new TrieNode());
  }

  getNode(key: string): TrieNode {
    return this.nodeMap.get(key);
  }

  deleteNode(key: string): boolean {
    return this.nodeMap.delete(key);
  }

  keysLen(): number {
    return Array.from(this.nodeMap.keys()).length;
  }

  isEmpty(): boolean {
    return this.nodeMap.size <= 0;
  }
}

export class Trie {
  root: TrieNode;

  constructor() {
    this.root = new TrieNode();
  }

  insert(data: string): void {
    this.spread(data, this.root);
  }

  private spread(data: string, node: TrieNode): void {
    if (data.length <= 0) return;

    const prefix = data[0];
    const substring = data.substring(1);

    if (!node.nodeMap.has(prefix)) node.addNode(prefix);

    this.spread(substring, node.getNode(prefix));
  }

  exists(data: string): boolean {
    try {
      let currentNode = this.root;
      let currentData = data;
      
      while (currentData.length > 0) {
        currentNode = currentNode.getNode(currentData[0]);
        currentData = currentData.substring(1);
      }

      return true;

    } catch (err) {
      return false;
    }
  }

  getRelatives(data: string): string[] {
    try {
      const result: string[] = [];
      const targetNode = this.findStartNode(data);

      this.traverse(data, targetNode, result);

      return result;

    } catch {
      return [];
    }
  }

  private findStartNode(data: string): TrieNode {
    let targetNode = this.root;

    data.split('').forEach((v) => {
      targetNode = targetNode.getNode(v);
    });
    
    return targetNode;
  }

  traverse(prefix: string, node: TrieNode, result: string[]): void {
    if (node.isEmpty()) {
      result.push(prefix);
      return;
    }

    Array.from(node.nodeMap.keys()).forEach((member: string) => {
      this.traverse(prefix + member, node.getNode(member), result);
    });
  }

  delete(data: string): void {
    const targetNodes: { isBranch: boolean; node: TrieNode; }[] = [];

    let current = this.root;
    data.substring(0, data.length-1).split('').forEach((v) => {
      const node = current.getNode(v);
      const isBranch = node.keysLen() > 1;

      targetNodes.push({ isBranch, node });

      current = node;
    });

    const sub = data.substring(1).split('');

    for (let i=targetNodes.length-1;i > 0;i--) {
      const { isBranch, node } = targetNodes[i];

      node.deleteNode(sub[i]);

      if (isBranch) break;
    }
  }
}
