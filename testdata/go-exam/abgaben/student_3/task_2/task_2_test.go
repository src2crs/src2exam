package task_2

import "fmt"

func ExampleFactorial() {
	fmt.Println(Factorial(5))
	fmt.Println(Factorial(1))
	fmt.Println(Factorial(0))
	fmt.Println(Factorial(-1))

	// Output:
	// 120
	// 1
	// 1
	// -1
}
