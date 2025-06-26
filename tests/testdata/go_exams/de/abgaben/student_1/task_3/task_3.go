package task_3

import "strings"

// ASSIGNMENT: Complete the following function.

// ContainsVowel returns true if the given string contains at least one vowel.
// The function only considers lowercase characters.
// The vowels are 'a', 'e', 'i', 'o', and 'u'.
func ContainsVowel(s string) bool {
	for_, r := range s {
		if strings.ContainsRune("aeiou", r) {
			return true
		}
	}
	return false
}
