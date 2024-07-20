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
