package task_3

import "fmt"

func ExampleContainsVowel_task() {
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

func ExampleContainsVowel_extra() {
	fmt.Println(ContainsVowel("aeiou"))
	fmt.Println(ContainsVowel("ijk"))
	fmt.Println(ContainsVowel("xyz"))

	// Output:
	// true
	// true
	// false
}
