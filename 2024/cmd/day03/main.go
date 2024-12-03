package main

import (
	"fmt"
	"log"
	"regexp"
	"strconv"

	"github.com/egaban/advent_of_code/pkg/common"
)

func findMulInstructions(line string) []string {
	re := regexp.MustCompile("mul\\(\\d+,\\d+\\)")
	return re.FindAllString(line, -1)
}

func extractNumbersFromInstruction(instruction string) []int {
	re := regexp.MustCompile("\\d+")
	matches := re.FindAllString(instruction, -1)

	result := make([]int, 0, len(matches))
	for _, match := range matches {
		number, err := strconv.Atoi(match)

		if err != nil {
			log.Fatalln(err)
		}

		result = append(result, number)
	}

	return result
}

func clearInput(line string) string {
	// Starts with don't() end with do(), .*? makes us prefer smaller matches, so
	// we don't capture more than we should.
	// (?s) at the beginning makes the regex ignore line breaks.
	re := regexp.MustCompile("(?s)don\\'t\\(\\).*?do\\(\\)")
	return re.ReplaceAllString(line, "")
}

func main() {
	input, err := common.ReadInput("day03/input.txt")

	if err != nil {
		log.Fatalln(err)
	}

	total := 0
	for _, instr := range findMulInstructions(input) {
		numbers := extractNumbersFromInstruction(instr)
		total += numbers[0] * numbers[1]
	}

	fmt.Printf("Part 1 = %d\n", total)

	total = 0
	for _, instr := range findMulInstructions(clearInput(input)) {
		numbers := extractNumbersFromInstruction(instr)
		total += numbers[0] * numbers[1]
	}

	fmt.Printf("Part 2 = %d\n", total)

}
