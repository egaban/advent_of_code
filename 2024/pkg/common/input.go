package common

import (
	"bufio"
	"os"
	"path/filepath"
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
