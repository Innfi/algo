

export class TrieNode {
  nodeMap: { [key: string]: TrieNode };

  constructor() {
    this.nodeMap = {};
  }

  set(key: string, newNode: TrieNode) {
    this.nodeMap[key] = newNode;
  }

  isEmpty(): boolean {
    return Object.keys(this.nodeMap).length <= 0;
  }
}

export class Trie {
  root: TrieNode;

  constructor() {
    this.root = new TrieNode();
  }

  insert(data: string): void {
    const prefix = data[0];

    if (this.root.isEmpty() || !Object.keys(this.root.nodeMap).includes(prefix)) {
      const substring = data.substring(1);

      const newNode = new TrieNode();
      this.root.set(prefix, newNode);

      this.spread(substring, newNode);
      return;
    }

    //const keys = Object.keys(this.root.nodeMap);
  }

  spread(data: string, node: TrieNode): void {
    if (data.length <= 1) return;

    const prefix = data[0];
    const substring = data.substring(1);

    if (node.isEmpty() || !Object.keys(node.nodeMap).includes(prefix)) {
      
      const newNode = new TrieNode();
      node.set(prefix, newNode);

      this.spread(substring, newNode);
    }

  }

  exists(data: string): boolean {
    try {
      let currentData = data;
      let currentNode = this.root;

      while(currentData.length > 1) {
        const prefix = currentData[0];
        
        if(!currentNode.nodeMap[prefix]) return false;

        currentNode = currentNode.nodeMap[prefix];
        currentData = currentData.substring(1);
      }

      return true;

    } catch (err) {
      return false;
    }
  }
}
