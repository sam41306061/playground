package main

import (
	"fmt"
	"strings"
)

func getInput() string {
	return `s..##.......
	#...#...#..
	.#....#..#.
	..#.#...#.#
	.#...##..#.
	..#.##.....
	.#.#.#....#
	.#........#
	#.##...#...
	#...##....#
	.#..#...#.`
}

type Thing = int

const (
	Tree Thing = iota
	Snow
)

func main() {
	treeCount := 0
	for r, line := range strings.Split(getInput(), "\n") {
		if string(line[r*3%len(line)]) == "#" {
			treeCount += 1
		}
	}
	fmt.Printf("treecount %v\n", treeCount)
}
