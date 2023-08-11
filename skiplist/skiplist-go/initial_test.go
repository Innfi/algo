package skiplist_test

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type Node struct {
	elem     int
	forward  []*Node
	span     []int
	levelMax int
}

func (node *Node) InitNode() {
	node.levelMax = 4

	node.elem = -1
	node.forward = make([]*Node, node.levelMax)
	node.span = make([]int, node.levelMax)
}

func TestNode(t *testing.T) {
	node := Node{}
	node.InitNode()

	assert.Equal(t, node.elem, -1)
	assert.Equal(t, len(node.forward), node.levelMax)
	assert.Equal(t, len(node.span), node.levelMax)
}
