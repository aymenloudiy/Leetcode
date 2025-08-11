package hindex

import "sort"

func _(citations []int) int {
	sort.Slice(citations, func(i, j int) bool {
		return citations[i] > citations[j]
	})
    max:=0
	for index, element := range citations {
		if element <= index {
			return index
		}
        max+=1
	}
    return max
}