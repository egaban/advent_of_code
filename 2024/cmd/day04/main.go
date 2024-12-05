package main

import (
	"fmt"
	"log"

	"github.com/egaban/advent_of_code/pkg/common"
)

func toRuneMatrix(input []string) [][]rune {
	result := make([][]rune, 0, len(input))
	for _, line := range input {
		result = append(result, []rune(line))
	}
	return result
}

func matchesInDirection(matrix [][]rune, words []string, start, direction [2]int) bool {
	for _, word := range words {
		word_runes := []rune(word)
		matched := true
		for i := range len(word) {
			x := start[0] + i*direction[0]
			y := start[1] + i*direction[1]

			if x < 0 || x >= len(matrix) {
				matched = false
				break
			}

			if y < 0 || y >= len(matrix[i]) {
				matched = false
				break
			}

			if matrix[x][y] != word_runes[i] {
				matched = false
				break
			}
		}

		if matched {
			return true
		}
	}
	return false
}

func numberOfMatches(matrix [][]rune) int {
	words := []string{"XMAS"}
	directions := [][2]int{
		{-1, -1},
		{-1, 0},
		{-1, 1},
		{0, -1},
		{0, 1},
		{1, -1},
		{1, 0},
		{1, 1},
	}

	result := 0
	for i := range len(matrix) {
		for j := range len(matrix[i]) {
			for k := range len(directions) {
				if matchesInDirection(matrix, words, [2]int{i, j}, directions[k]) {
					result++
				}
			}

		}
	}

	return result
}

// Naive, but will do the job
func part2(matrix [][]rune) int {
	result := 0
	for i := range len(matrix) {
		for j := range len(matrix[i]) {
			if matchesInDirection(matrix, []string{"MAS", "SAM"}, [2]int{i - 1, j - 1}, [2]int{1, 1}) &&
				matchesInDirection(matrix, []string{"MAS", "SAM"}, [2]int{i - 1, j + 1}, [2]int{1, -1}) {
				result += 1
			}
		}
	}
	return result
}

func main() {
	input, err := common.ReadInputLines("day04/input.txt")

	if err != nil {
		log.Fatalln(err)
	}

	matrix := toRuneMatrix(input)

	part1 := numberOfMatches(matrix)
	fmt.Printf("Part 1 = %d\n", part1)

	part2Result := part2(matrix)
	fmt.Printf("Part 2 = %d\n", part2Result)
}
