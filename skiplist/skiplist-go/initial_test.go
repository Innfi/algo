package skiplist_test

import (
	"testing"

	"math/rand"
	"time"

	"github.com/stretchr/testify/assert"
)

const MAX_LEVEL int = 8
const LEVEL_PROB float32 = 0.25

type Node struct {
	elem    int
	forward []*Node
	span    []int
}

func (node *Node) InitNode() {
	node.elem = -1
	node.forward = make([]*Node, MAX_LEVEL)
	node.span = make([]int, MAX_LEVEL)
}

func NewNode(elem int) *Node {
	newNode := Node{
		elem: elem,
	}
	newNode.InitNode()

	return &newNode
}

type SkipList struct {
	root  *Node
	level int
}

func (skipList *SkipList) Init() {
	skipList.root = NewNode(-1)
	skipList.level = 0

	rand.Seed(time.Now().UnixNano())
}

func (skipList *SkipList) Insert(elem int) {
	current := skipList.root
	update := make([]*Node, MAX_LEVEL)

	rLevel := skipList.randomLevel()
	newNode := NewNode(elem)

	for i := skipList.level; i >= 0; i-- {
		spanSum := 0
		for current.forward[i] != nil && current.forward[i].elem < elem {
			spanSum += current.span[i]
			current = current.forward[i]
		}

		update[i] = current

		if i < MAX_LEVEL-1 {
			newNode.span[i+1] = spanSum
		}
	}

	newNode.span[0] = 1
	current = current.forward[0]

	if current != nil && current.elem == elem {
		return
	}

	if rLevel > skipList.level {
		for i := skipList.level + 1; i < rLevel+1; i++ {
			update[i] = skipList.root
		}

		skipList.level = rLevel
	}

	spanSum := 0
	for i := 0; i <= rLevel; i++ {
		currentUpdate := update[i]

		newNode.forward[i] = update[i].forward[i]
		update[i].forward[i] = newNode

		oldSpan := currentUpdate.span[i]
		newNodeSpan := newNode.span[i]

		spanSum += newNodeSpan

		currentUpdate.span[i] = spanSum
		newSpan := oldSpan - spanSum

		if newSpan < 0 {
			newNode.span[i] = 0
		} else {
			newNode.span[i] = newSpan
		}
	}
}

func (skipList *SkipList) randomLevel() int {
	r := rand.Float32()
	level := 0

	for r < LEVEL_PROB && level < MAX_LEVEL {
		level++
		r = rand.Float32()
	}

	return level
}

func TestNode(t *testing.T) {
	node := Node{}
	node.InitNode()

	assert.Equal(t, node.elem, -1)
	assert.Equal(t, len(node.forward), MAX_LEVEL)
	assert.Equal(t, len(node.span), MAX_LEVEL)
}

func TestSkipListInitial(t *testing.T) {
	instance := SkipList{}
	instance.Init()

	root := instance.root

	assert.Equal(t, root.elem, -1)
	assert.Equal(t, len(root.forward), MAX_LEVEL)
}

func TestRandom(t *testing.T) {
	rand.Seed(time.Now().UnixNano())

	for i := 0; i < 1000; i++ {
		result := rand.Intn(8)

		assert.Equal(t, result >= 0, true)
		assert.Equal(t, result < 8, true)
	}
}

func TestMakeArray(t *testing.T) {
	lists := make([]*Node, 8)

	assert.Equal(t, len(lists), 8)
	assert.Equal(t, lists[0] == nil, true)
}
