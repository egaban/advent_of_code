package main

import (
	"fmt"
	"math"
	"sort"

	"github.com/egaban/advent_of_code/pkg/common"
)

func intFrequency(ints []int) map[int]int {
	result := make(map[int]int)

	for _, num := range ints {
		result[num]++
	}

	return result
}

func main() {
	lines, err := common.ReadInputLines("day01/input.txt")

	if err != nil {
		fmt.Println(err)
	}

	list1 := make([]int, 0, len(lines))
	list2 := make([]int, 0, len(lines))

	for _, line := range lines {
		numbers := common.StringToInts(line)
		list1 = append(list1, numbers[0])
		list2 = append(list2, numbers[1])
	}

	sort.Ints(list1)
	sort.Ints(list2)

	totalDiff := 0
	for i := range list1 {
		totalDiff += int(math.Abs(float64(list1[i] - list2[i])))
	}

	fmt.Printf("Part 1 = %d\n", totalDiff)

	similarity := 0
	frequencies := intFrequency(list2)

	for _, num := range list1 {
		similarity += num * frequencies[num]
	}

	fmt.Printf("Part 2 = %d\n", similarity)
}
