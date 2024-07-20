package main

import "fmt"

type TreeNode struct {
    value int
    left *TreeNode
    right *TreeNode
}

func (node *TreeNode) insert(value int) {
    if value <= node.value {
        if node.left == nil {
            node.left = &TreeNode{value: value}
        } else {
            node.left.insert(value)
        }
    }
}

func (node *TreeNode) inOrder() {
    if node == nil {
        return
    }
    node.left.inOrder()
    fmt.Print(node.value, " ") 
    node.right.inOrder()
}

func main() {
	root := &TreeNode{value: 10}
	values := []int{5, 15, 3, 7, 12, 18}

	for _, v := range values {
		root.insert(v)
	}

	fmt.Print("In-order traversal: ")
	root.inOrder()
	fmt.Println()
}
