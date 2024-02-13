# Palindrome Numbers

## Problem Statement

Given an integer `x`, return true if `x` is a palindrome number. Palindrome numbers are those that read the same forwards and backwards. For example, 121 is a palindrome, while 123 is not.

## Constraints

- `-2^31 <= x <= 2^31 - 1`

## Approach 1: Without Using String

### Flowchart

``` mermaid
graph LR;
    Start[Start] --> CheckNegativeNum{num >= 0}
    CheckNegativeNum -- Yes --> Initialize[Initialize variables]
    CheckNegativeNum -- No --> ReturnFalse[Return False]
    Initialize --> Loop[Start Loop]
    Loop --> |"num > 0"| ExtractDigit[Extract rightmost digit of num]
    Loop --> |"num == 0"| EndLoop[End Loop]
    ExtractDigit --> CalculateReverse[Calculate reverse]
    CalculateReverse --> UpdateReverse[Update reverse]
    CalculateReverse --> UpdateNum[Update num]
    UpdateReverse --> Loop
    UpdateNum --> Loop
    EndLoop --> |"reverse == original"| ReturnTrue[Return True]
    EndLoop --> |"reverse != original"| ReturnFalse
    ReturnTrue --> End[End]
    ReturnFalse --> End

    classDef startEnd fill:#9f9,stroke:#333,stroke-width:2px;
    class Start,End startEnd;
```

### Pseudocode

```plaintext
function reverseInteger(num):
    // Initialize variables
    reverse = 0
    original = num
    
    // Iterate over each digit of the number
    for each digit in num:
        // Extract the last digit of the number
        digit = num % 10
        // Update the reverse by adding the extracted digit
        reverse = reverse * 10 + digit
        // Remove the last digit from the number
        num = num // 10
    
    // Return the reversed integer
    return reverse
```

### Explanation

#### Example 1: num = 121

- The number `num` is not negative, so the condition `if num < 0:` is not met.
- We initialize `reverse` to 0 and `original` to 121 (the original value of `num`).
- The `while` loop starts since `num > 0`.
  - In the first iteration:
  - `digit` is 121 % 10 = 1.
    - `reverse` becomes 0 * 10 + 1 = 1.
    - `num` becomes 121 // 10 = 12.
  - In the second iteration:
    - `digit` is 12 % 10 = 2.
    - `reverse` becomes 1 * 10 + 2 = 12.
    - `num` becomes 12 // 10 = 1.
  - In the third iteration:
    - `digit` is 1 % 10 = 1.
    - `reverse` becomes 12 * 10 + 1 = 121.
    - `num` becomes 1 // 10 = 0.
- The loop ends since `num` is no longer greater than 0.
- Finally, the function returns `reverse == original`, which is 121 == 121, so it returns True.

#### Example 2: num = -121

- Since `num` is negative, the function immediately returns False.

#### Example 3: num = 10

- `num` is not negative.
- In the first iteration of the loop, `num` becomes 1 after removing the last digit.
- The loop ends, and the function returns `reverse == original`, which is 1 == 10, so it returns False.

### Solution Code Block

=== "Python"
    ``` Python title="Without Using String" linenums="1"
    --8<-- 'src\Python\palindrome_number_without_string.py'
    ```

=== "C++"
    ``` c++ title="Without Using Strings" linenums="1"
    --8<-- 'src\C++\palindrome_number_without_string.cpp'
    ```

=== "Rust"
    ``` rust title="Without Using String" linenums="1"
    --8<-- 'src\Rust\palindrome_number_without_string.rs'
    ```

=== "Go"
    ``` Go title="Without Using String" linenums="1"
    --8<-- 'src\Golang\palindrome_number_without_string.go'
    ```

=== "Java"
    ``` Java title="Without Using String" linenums="1"
    --8<-- 'src\Java\palindrome_number_without_string.java'
    ```

=== "JavaScript"
    ``` JavaScript title="Without Using String" linenums="1"
    --8<-- 'src\JavaScript\palindrome_number_without_string.js'
    ```

[TryIt](https://repl.it/languages){ .md-button target="_blank" }

## Approach 2: Using String

### Flowchart

``` mermaid
graph LR;
    Start[Start] --> ConvertToStr[Convert integer to string]
    ConvertToStr --> ReverseString[Reverse the string]
    ReverseString --> CompareStrings[Compare original and reversed strings]
    CompareStrings --> |"strings are equal"| ReturnTrue[Return True]
    CompareStrings --> |"strings are not equal"| ReturnFalse[Return False]
    ReturnTrue --> End[End]
    ReturnFalse --> End

    classDef startEnd fill:#9f9,stroke:#333,stroke-width:2px;
    class Start,End startEnd;
```

### Pseudocode

```plaintext
function isPalindrome(num):
    // Convert the integer to a string
    str_num = str(num)
    // Reverse the string
    reversed_str = str_num[::-1]
    // Check if the original string is equal to the reversed string
    if str_num == reversed_str:
        return True
    else:
        return False
```

### Explanation

#### Example 1: num = 121

- We convert the integer 121 to the string "121".
- Reversing the string "121" gives us "121".
- Since the original string is equal to the reversed string, the function returns True.

#### Example 2: num = -121

- Converting a negative number to a string includes the negative sign, so "-121" is the string representation.
- Reversing the string "-121" gives us "121-", which is not equal to the original string.
- Therefore, the function returns False.

#### Example 3: num = 10

- Converting the integer 10 to a string gives us "10".
- Reversing the string "10" gives us "01", which is not equal to the original string.
- Thus, the function returns False.

### Solution Code Block

=== "Python"
    ``` Python title="With Using String" linenums="1"
    --8<-- 'https://github.com/sureshchandras3kar/leetcode_solutions/blob/main/src/Python/palindrome_number_with_string.py'
    ```

=== "C++"
    ``` c++ title="With Using String" linenums="1"
    --8<-- 'src\C++\palindrome_number_with_string.cpp'
    ```

=== "Rust"
    ``` rust title="With Using String" linenums="1"
    --8<-- 'src\Rust\palindrome_number_with_string.rs'
    ```

=== "Go"
    ``` Go title="With Using String" linenums="1"
    --8<-- 'src\Golang\palindrome_number_with_string.go'
    ```

=== "Java"
    ``` Java title="With Using String" linenums="1"
    --8<-- 'src\Java\palindrome_number_with_string.java'
    ```

=== "JavaScript"
    ``` JavaScript title="With Using String" linenums="1"
    --8<-- 'src\JavaScript\palindrome_number_with_string.js'
    ```
[TryIt](https://repl.it/languages){ .md-button target="_blank" }
