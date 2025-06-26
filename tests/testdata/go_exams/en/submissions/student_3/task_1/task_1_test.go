package task_1

import "fmt"

func ExampleNumberList() {
	fmt.Println(NumberList(5))
	fmt.Println(NumberList(1))
	fmt.Println(NumberList(0))
	fmt.Println(NumberList(-1))

	// Output:
	// [1 2 3 4 5]
	// [1]
	// []
	// []
}
