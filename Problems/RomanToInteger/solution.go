package main

func _(s string) int {
	numeralsMap := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}
	result := 0
	for i := range s {
		if  i != len(s)-1 && numeralsMap[s[i]] < numeralsMap[s[i+1]]  {
			continue
		}
		if i != 0 && numeralsMap[s[i]] > numeralsMap[s[i-1]]  {
			result += numeralsMap[s[i]] - numeralsMap[s[i-1]]
		} else {
			result += numeralsMap[s[i]]
		}

	}
	return result
}