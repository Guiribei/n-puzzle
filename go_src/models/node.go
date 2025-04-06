package models

type Node struct {
	PuzzleConfiguration [][]int
	HeuristicValue int
	Depth int
	WasSeen bool
	Parent *Node
}

func NewNode(puzzleConfiguration [][]int) *Node {
	return &Node{
		PuzzleConfiguration: puzzleConfiguration,
	}
}