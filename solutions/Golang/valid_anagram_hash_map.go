func isAnagram(s string, t string) bool {
    if len(s) != len(t) {
        return false
    }
    count := map[rune]int{}
    for _, ch := range s {
        count[ch]++
    }
    for _, ch := range t {
        if _, ok := count[ch]; !ok {
            return false
        }
        count[ch]--
        if count[ch] == 0 {
            delete(count, ch)
        }
    }
    return len(count) == 0
}
