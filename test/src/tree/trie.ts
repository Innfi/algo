

export class TrieNode {
  nodeMap: Map<string, TrieNode>;

  constructor() {
    this.nodeMap = new Map<string, TrieNode>();
  }

  setNode(key: string, newNode: TrieNode) {
    this.nodeMap.set(key, newNode);
  }

  getNode(key: string): TrieNode {
    return this.nodeMap.get(key);
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

  spread(data: string, node: TrieNode): void {
    if (data.length <= 0) return;

    const prefix = data[0];
    const substring = data.substring(1);

    if (!node.nodeMap.has(prefix)) {
      node.setNode(prefix, new TrieNode());
    }

    this.spread(substring, node.getNode(prefix));
  }

  exists(data: string): boolean {
    try {
      let currentData = data;
      let currentNode = this.root;

      while (currentData.length > 0) {
        currentNode = currentNode.getNode(currentData[0]);
        currentData = currentData.substring(1);
      }

      return true;

    } catch (err) {
      return false;
    }
  }
}
