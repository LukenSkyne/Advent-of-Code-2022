package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Day01")

	input, _ := os.ReadFile("./input.txt")
	stashes := strings.Split(string(input), "\n\n")

	var sums []int

	for _, stash := range stashes {
		items := strings.Split(stash, "\n")
		sum := 0

		for _, item := range items {
			val, _ := strconv.Atoi(item)
			sum += val
		}

		sums = append(sums, sum)
	}

	sort.Sort(sort.Reverse(sort.IntSlice(sums)))
	fmt.Println("Part 1:", sums[0])
}
