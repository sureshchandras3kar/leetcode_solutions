def is_anagram(s: str, t: str) -> bool:
    return sorted(s) == sorted(t)


print(is_anagram("anagram", "nagaram"))  # True
print(is_anagram("rat", "car"))          # False
