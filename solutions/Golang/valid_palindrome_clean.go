import "unicode"

func isPalindrome(s string) bool {
	var cleaned []rune
	for _, c := range s {
		if unicode.IsLetter(c) || unicode.IsDigit(c) {
			cleaned = append(cleaned, unicode.ToLower(c))
		}
	}
	n := len(cleaned)
	for i := 0; i < n/2; i++ {
		if cleaned[i] != cleaned[n-1-i] {
			return false
		}
	}
	return true
}
