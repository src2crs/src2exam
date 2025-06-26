package task_1

import "fmt"

func ExampleNumberList_task() {
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

func ExampleNumberList_extra() {
	fmt.Println(NumberList(2))
	fmt.Println(NumberList(10))
	fmt.Println(NumberList(3))
	fmt.Println(NumberList(-4))

	// Output:
	// [1 2]
	// [1 2 3 4 5 6 7 8 9 10]
	// [1 2 3]
	// []
}
