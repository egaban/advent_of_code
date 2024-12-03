package main

import (
	"fmt"
	"log"
	"math"

	"github.com/egaban/advent_of_code/pkg/common"
)

func isIncreasing(report []int) bool {
	for i := 1; i < len(report); i++ {
		if report[i-1] >= report[i] {
			return false
		}
	}

	return true
}

func isDecreasing(report []int) bool {
	for i := 1; i < len(report); i++ {
		if report[i-1] <= report[i] {
			return false
		}
	}

	return true

}

func isSafe(report []int) bool {
	for i := 1; i < len(report); i++ {
		diff := math.Abs(float64(report[i] - report[i-1]))
		if diff < 1 || diff > 3 {
			return false
		}
	}
	return true
}

func removeIndex(original []int, index int) []int {
	result := make([]int, 0, len(original)-1)

	result = append(result, original[:index]...)
	result = append(result, original[index+1:]...)

	return result
}

func main() {
	lines, err := common.ReadInputLines("day02/input.txt")
	if err != nil {
		log.Fatalln(err)
	}

	total := 0
	for _, line := range lines {
		report := common.StringToInts(line)
		if (isIncreasing(report) || isDecreasing(report)) && isSafe(report) {
			total++
		}
	}

	fmt.Printf("Part 1 = %d\n", total)

	total = 0
	for _, line := range lines {
		report := common.StringToInts(line)

		for i := range report {
			newReport := removeIndex(report, i)

			if (isIncreasing(newReport) || isDecreasing(newReport)) && isSafe(newReport) {
				total++
				break
			}
		}

	}
	fmt.Printf("Part 2 = %d\n", total)
}
