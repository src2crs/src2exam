package task_1

// ASSIGNMENT: Complete the following function.

// NumberList returns a slice containing all integers from 1 to n.
// If n <= 0, the slice will be empty.
func NumberList(n int) []int {
	if n <= 0 {
		return []int{}
	}
	result := make([]int, n)
	for i := 0; i < n; i++ {
		result[i] = i + 1
	}
	return result
}
