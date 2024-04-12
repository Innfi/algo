package ref

type Node struct {
	leaf     bool
	keys     []int
	pointers []*Node
	parent   *Node
	next     *Node // for leaf nodes
}

type BTree struct {
	root *Node
}

func NewBTree() *BTree {
	return &BTree{}
}

func (t *BTree) Insert(key int) {
	if t.root == nil {
		t.root = &Node{leaf: true, keys: []int{key}}
		return
	}

	leaf := t.findLeafNode(key)
	index, found := findKeyInNode(leaf, key)
	if found {
		return // key already exists
	}

	leaf.keys = append(leaf.keys[:index], append([]int{key}, leaf.keys[index:]...)...)
	if len(leaf.keys) <= 3 {
		return
	}

	t.splitLeafNode(leaf)
}

func (t *BTree) findLeafNode(key int) *Node {
	node := t.root
	for !node.leaf {
		for i := 0; i < len(node.keys); i++ {
			if key < node.keys[i] {
				node = node.pointers[i]
				break
			}

			if i == len(node.keys)-1 {
				node = node.pointers[i+1]
				break
			}
		}
	}
	return node
}

func findKeyInNode(node *Node, key int) (int, bool) {
	for i, k := range node.keys {
		if k == key {
			return i, true
		} else if k > key {
			return i, false
		}
	}
	return len(node.keys), false
}

func (t *BTree) splitLeafNode(leaf *Node) {
	mid := len(leaf.keys) / 2
	newNode := &Node{leaf: true, keys: leaf.keys[mid:], next: leaf.next}
	leaf.keys = leaf.keys[:mid]
	leaf.next = newNode

	if leaf == t.root {
		t.root = &Node{keys: []int{newNode.keys[0]}, pointers: []*Node{leaf, newNode}}
		return
	}

	parent := leaf.parent
	index, _ := findKeyInNode(parent, leaf.keys[0])
	parent.keys = append(parent.keys[:index], append([]int{newNode.keys[0]}, parent.keys[index:]...)...)
	parent.pointers = append(parent.pointers[:index+1], append([]*Node{newNode}, parent.pointers[index+1:]...)...)
	newNode.parent = parent

	if len(parent.keys) > 3 {
		t.splitInternalNode(parent)
	}
}

func (t *BTree) splitInternalNode(node *Node) {
	mid := len(node.keys) / 2
	newNode := &Node{keys: node.keys[mid+1:], pointers: node.pointers[mid+1:]}
	node.keys = node.keys[:mid]
	node.pointers = node.pointers[:mid+1]

	if node == t.root {
		t.root = &Node{keys: []int{node.keys[mid]}, pointers: []*Node{node, newNode}}
		return
	}

	parent := node.parent
	index, _ := findKeyInNode(parent, node.keys[0])
	parent.keys = append(parent.keys[:index], append([]int{node.keys[mid]}, parent.keys[index:]...)...)
	parent.pointers = append(parent.pointers[:index+1], append([]*Node{newNode}, parent.pointers[index+1:]...)...)
	newNode.parent = parent

	if len(parent.keys) > 3 {
		t.splitInternalNode(parent)
	}
}

func tester() {
	tree := NewBTree()
	for _, v := range []int{10, 20, 30, 40, 50, 60, 70, 80, 90} {
		tree.Insert(v)
	}
}
