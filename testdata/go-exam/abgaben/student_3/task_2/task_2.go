package task_2

// ASSIGNMENT: Complete the following function.
// ADDITIONAL REQUIREMENT: The function must be recursive.

// Factorial returns the factorial of n for n >= 0.
// For n < 0, the function returns -1.
func Factorial(n int) int {
	return n * Factorial(n-1)
}
