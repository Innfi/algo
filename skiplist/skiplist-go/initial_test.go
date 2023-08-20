package skiplist_test

import (
	"testing"

	"math/rand"
	"time"

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

func NewNode() *Node {
	newNode := Node{}
	newNode.InitNode()

	return &newNode
}

type SkipList struct {
	root  *Node
	level int
}

func (skipList *SkipList) Init() {
	skipList.root = NewNode()
	skipList.level = 0
}

func (skipList *SkipList) Insert(elem int) {
	// current := skipList.root
	// update := make([]*Node, skipList.root.levelMax)
}

func TestNode(t *testing.T) {
	node := Node{}
	node.InitNode()

	assert.Equal(t, node.elem, -1)
	assert.Equal(t, len(node.forward), node.levelMax)
	assert.Equal(t, len(node.span), node.levelMax)
}

func TestSkipListInitial(t *testing.T) {
	instance := SkipList{}
	instance.Init()

	root := instance.root

	assert.Equal(t, root.elem, -1)
	assert.Equal(t, len(root.forward), root.levelMax)
}

func TestRandom(t *testing.T) {
	rand.Seed(time.Now().UnixNano())

	for i := 0; i < 1000; i++ {
		result := rand.Intn(8)

		assert.Equal(t, result >= 0, true)
		assert.Equal(t, result < 8, true)
	}
}
