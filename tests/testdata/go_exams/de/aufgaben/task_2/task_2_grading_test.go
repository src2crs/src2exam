package task_2

import "fmt"

func ExampleFactorial_task() {
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

func ExampleFactorial_extra() {
	fmt.Println(Factorial(3))
	fmt.Println(Factorial(10))
	fmt.Println(Factorial(4))
	fmt.Println(Factorial(-5))

	// Output:
	// 6
	// 3628800
	// 24
	// -1
}
