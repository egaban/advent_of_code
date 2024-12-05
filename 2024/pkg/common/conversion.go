package common

import (
	"log"
	"strconv"
	"strings"
)

func StringToInts(s string) []int {
	fields := strings.Fields(s)

	nums := make([]int, 0, len(fields))

	for _, field := range fields {
		if num, err := strconv.Atoi(field); err == nil {
			nums = append(nums, num)
		} else {
			log.Fatalf("Failed to parse integer %s", field)
		}
	}

	return nums
}

func StringsToInts(strings []string) []int {
	result := make([]int, 0, len(strings))

	for _, element := range strings {
		int_element, err := strconv.Atoi(element)
		if err != nil {
			log.Fatalln(err)
		}
		result = append(result, int_element)
	}

	return result
}
