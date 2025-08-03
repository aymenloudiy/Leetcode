package main
func removeElement(nums []int, val int) int {
    k := 0
    for index,element := range nums{
        if element != val {
            nums[k] = nums[index]
            k++
        }
    }
    return k
}