package main

import (
	"fmt"
	"log"
	"sort"
	"strings"

	"github.com/egaban/advent_of_code/pkg/common"
)

type Instance struct {
	rules     map[int][]int
	orderings [][]int
}

func parse(lines []string) Instance {
	result := Instance{
		rules:     make(map[int][]int),
		orderings: nil,
	}

	i := 0
	for i < len(lines) {
		if lines[i] == "" {
			i++
			break
		}

		split := strings.Split(lines[i], "|")
		rule := common.StringsToInts(split)

		rule_for_x := result.rules[rule[0]]
		result.rules[rule[0]] = append(rule_for_x, rule[1])

		i++
	}

	result.orderings = make([][]int, 0, len(lines)-i)
	for i < len(lines) {
		split := strings.Split(lines[i], ",")
		ordering := common.StringsToInts(split)
		result.orderings = append(result.orderings, ordering)
		i++
	}

	return result
}

func checkOrderings(instance Instance) ([][]int, [][]int) {
	valids := make([][]int, 0, len(instance.orderings))
	invalids := make([][]int, 0, len(instance.orderings))

	for _, ordering := range instance.orderings {
		valid := true
		printed := make([]int, 0, len(ordering))
		for _, page := range ordering {
			rules := instance.rules[page]

			if common.ContainsAny(rules, printed) {
				valid = false
				break
			}

			printed = append(printed, page)
		}

		if valid {
			valids = append(valids, ordering)
		} else {
			invalids = append(invalids, ordering)
		}
	}

	return valids, invalids
}

func part1Answer(orderings [][]int) int {
	result := 0
	for _, ordering := range orderings {
		result += ordering[len(ordering)/2]
	}
	return result
}

func part2Answer(instance Instance, orderings [][]int) int {
	result := 0
	for _, ordering := range orderings {
		sort.Slice(ordering,
			func(i, j int) bool {
				x, y := ordering[i], ordering[j]
				return common.ContainsAny([]int{y}, instance.rules[x])
			})
		result += ordering[len(ordering)/2]
	}
	return result
}

func main() {
	lines, err := common.ReadInputLines("day05/input.txt")

	if err != nil {
		log.Fatalln(err)
	}

	instance := parse(lines)
	valids, invalids := checkOrderings(instance)

	fmt.Printf("Part 1 = %d\n", part1Answer(valids))
	fmt.Printf("Part 2 = %d\n", part2Answer(instance, invalids))
}
