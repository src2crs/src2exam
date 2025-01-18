package task_3

import "fmt"

func ExampleContainsVowel() {
	fmt.Println(ContainsVowel("hello"))
	fmt.Println(ContainsVowel("world"))
	fmt.Println(ContainsVowel("bcd"))
	fmt.Println(ContainsVowel(""))

	// Output:
	// true
	// true
	// false
	// false
}
