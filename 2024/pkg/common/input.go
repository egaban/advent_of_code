package common

import (
	"bufio"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func ReadInputLines(inputpath string) ([]string, error) {
	fullpath := filepath.Join("inputs/", inputpath)
	file, err := os.Open(fullpath)

	if err != nil {
		return nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}

func ReadInput(inputpath string) (string, error) {
	fullpath := filepath.Join("inputs", inputpath)
	content, err := os.ReadFile(fullpath)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

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
