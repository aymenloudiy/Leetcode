package main

func _(nums []int) int {
	count := 0
	curr := nums[0]
	for _, element := range nums {
		if element == curr {
			count += 1
		} else {
			if count == 0 {
				curr = element
				count = 1
			} else {
				count--
			}
		}
	}
	return curr
}