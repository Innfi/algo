package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
only leaf node can have data value, any node above leaf have keys for index
each node can have multiple keys, limited by the decree
when the number of keys reach the decree, divide keys into two subnodes
each nodes on the same level are sorted, and connected as single linked list

TODO
--------------------------------------------------------------------------------
define node structure
insert, update, delete node while maintaining constraints
assert the search process

DONE
--------------------------------------------------------------------------------

*/

func TestInit(t *testing.T) {
	assert.Equal(t, 1, 1)
}
