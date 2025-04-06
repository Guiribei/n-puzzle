package heuristics

import (
	"github.com/0xEDU/n-puzzle/go_src/models"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func ManhattanDistance(current models.Node, goal models.Node) int {
	distance := 0
	puzzleSize := len(current.PuzzleConfiguration)
	goalPositions := make(map[int]struct{i, j int})

	for i := range puzzleSize {
		for j := range puzzleSize {
			tile := current.PuzzleConfiguration[i][j]
			goalPositions[tile] = struct{i, j int} {i, j}
		}
	}

	for i := range puzzleSize {
		for j := range puzzleSize {
			tile := current.PuzzleConfiguration[i][j]
			if tile != 0 {
				goalPair := goalPositions[tile]
				distance += abs(i-goalPair.i) + abs(j-goalPair.j)
			}
		}
	}

	return distance
}