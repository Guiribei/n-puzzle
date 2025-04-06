package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/0xEDU/n-puzzle/go_src/models"
	//"github.com/0xEDU/n-puzzle/go_src/heuristics"
)

func isComment(line string) bool {
	trimmed := strings.TrimLeft(line, " \t")
	return strings.HasPrefix(trimmed, "#") || trimmed == ""
}

func stripComments(line string) string {
	if i := strings.Index(line, "#"); i != -1 {
		return line[:i]
	}
	return line
}

func main() {

	args := os.Args[1:]
	argsLen := len(args)

	var heuristicFunction func(models.Node, models.Node) int

	switch argsLen{
	case 1:
		heuristicFunction = heuristics.ManhattanDistance
	case 2:
		fmt.Println("Tem heuristica")
	default:
		fmt.Println("Usage: ./n-puzzle <file_path> [manhattan/conflict/gaschnig]")
		return
	}

	file, err := os.Open(args[0])
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lineNum := 0
	var puzzleSize int
	var puzzleConfiguration [][]int

	for scanner.Scan() {
		line := scanner.Text()
		if isComment(line){
			continue
		}
		cleanLine := stripComments(line)
		cleanLine = strings.TrimSpace(cleanLine)

		if lineNum == 0 {
			value, err := strconv.Atoi(cleanLine)
			if err != nil {
				fmt.Printf("Invalid puzzle configuration: %v\n", err)
				return
			}
			puzzleSize = value
			lineNum++
			continue
		}
		
		var numbersLine []int
		splitLine := strings.Fields(cleanLine)
		if len(splitLine) != puzzleSize {
			fmt.Printf("Invalid puzzle configuration: %v\n", err)
			return
		}
		for i := range puzzleSize {
			number, err := strconv.Atoi(splitLine[i])
			if err != nil {
				fmt.Printf("Invalid puzzle configuration: %v\n", err)
				return
			}
			numbersLine = append(numbersLine, number)
		}
		puzzleConfiguration = append(puzzleConfiguration, numbersLine)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}

	firstNode := models.NewNode(puzzleConfiguration)
	// AStar(heuristicFunction, firstNode)

}