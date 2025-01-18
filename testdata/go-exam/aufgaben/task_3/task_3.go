package task_3

// ASSIGNMENT: Complete the following function.

// ContainsVowel returns true if the given string contains at least one vowel.
// The function only considers lowercase characters.
// The vowels are 'a', 'e', 'i', 'o', and 'u'.
func ContainsVowel(s string) bool {
	//begin:solution
	for _, r := range s {
		switch r {
		case 'a', 'e', 'i', 'o', 'u':
			return true
		}
	}
	return false
	//end:solution
}
