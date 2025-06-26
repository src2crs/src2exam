package task_1

// ASSIGNMENT: Complete the following function.

// NumberList returns a slice containing all integers from 1 to n.
// If n <= 0, the slice will be empty.
func NumberList(n int) []int {
	result := []int{}
	//begin:solution
	for i := 1; i <= n; i++ {
		result = append(result, i)
	}
	//end:solution
	return result
}
