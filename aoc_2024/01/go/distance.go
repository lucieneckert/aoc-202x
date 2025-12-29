package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"math"
	"os"
	"strconv"
	"strings"
)

var part = flag.Int("part", 0, "part of the problem to solve")

func max(nums []int) *int {
	var out *int
	for _, num := range nums {
		if out == nil || num > *out {
			out = &num
		}
	}

	return out
}

func sort(nums []int) []int {
	maxNum := max(nums)
	if maxNum == nil {
		return nums
	}

	// Do a funny counting sort to get the numbers in order:
	counts := make([]int, *maxNum+1)
	for _, num := range nums {
		counts[num] += 1
	}

	// Then, slot them in the right index in the initial slice:
	i := 0
	for num, count := range counts {
		for range count {
			nums[i] = num
			i += 1
		}
	}

	return nums
}

func parse(r io.Reader) ([]int, []int, error) {
	scanner := bufio.NewScanner(r)
	var left, right []int
	for scanner.Scan() {
		nums := strings.Fields(scanner.Text())
		if len(nums) != 2 {
			return nil, nil, fmt.Errorf("expected 2 numbers per line, got %v", nums)
		}
		l, err := strconv.Atoi(nums[0])
		if err != nil {
			return nil, nil, fmt.Errorf("could not parse %q as int", nums[0])
		}
		r, err := strconv.Atoi(nums[1])
		if err != nil {
			return nil, nil, fmt.Errorf("could not parse %q as int", nums[1])
		}
		left = append(left, l)
		right = append(right, r)
	}
	return left, right, nil
}

func partOne(left, right []int) int64 {
	sort(left)
	sort(right)

	var diffSum int64
	for idx := range len(left) {
		// fmt.Printf("a: %v, b: %v\n", left[idx], right[idx])
		diffSum += int64(math.Abs(float64(left[idx] - right[idx])))
	}

	return diffSum
}

func partTwo(left, right []int) int64 {
	rightCounts := map[int]int{}
	for _, num := range right {
		rightCounts[num] += 1
	}

	similaritySum := 0
	for _, num := range left {
		similaritySum += num * rightCounts[num]
	}

	return int64(similaritySum)

}

func main() {
	flag.Parse()

	left, right, err := parse(os.Stdin)
	if err != nil {
		fmt.Printf("error parsing input: %v\n", err)
		os.Exit(1)
	}

	var out int64
	switch *part {
	case 1:
		out = partOne(left, right)
	case 2:
		out = partTwo(left, right)
	default:
		fmt.Printf("invalid part %v supplied\n", *part)
		os.Exit(1)
	}

	fmt.Printf("Output: %v\n", out)
}
