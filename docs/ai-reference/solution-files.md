# Solution Files Reference

## File naming

Pattern: `solutions/{Language}/{problem_slug}_{approach_slug}.{ext}`

### Language folders & extensions

| Language | Folder | Extension |
|----------|--------|-----------|
| Python | `solutions/Python/` | `.py` |
| C++ | `solutions/C++/` | `.cpp` |
| Java | `solutions/Java/` | `.java` |
| JavaScript | `solutions/JavaScript/` | `.js` |
| Rust | `solutions/Rust/` | `.rs` |
| Go | `solutions/Golang/` | `.go` |

### Problem slug

Snake_case of the problem title:
- "Two Sum" → `two_sum`
- "Group Anagrams" → `group_anagrams`
- "Trapping Rain Water" → `trapping_rain_water`
- "Product of Array Except Self" → `product_except_self`

### Approach slugs used in this project

| Slug | Description |
|------|-------------|
| `hash_map` | Hash map / dictionary approach |
| `hash_set` | Hash set approach |
| `two_pointer` | Two-pointer technique |
| `brute_force` | Nested loops, naive approach |
| `sort_key` | Sorting-based key |
| `char_count` | Character frequency array |
| `prefix_sum` | Prefix sum technique |
| `prefix_suffix` | Prefix + suffix arrays |
| `two_pass` | Two-pass O(1) space variant |
| `stack` | Stack-based approach |
| `prefix_max` | Prefix max arrays |
| `index_hash` | Use array index as hash |
| `cycle_sort` | Cycle sort placement |
| `max_heap` | Max heap / priority queue |
| `bucket_sort` | Bucket sort technique |
| `sort_two_pointer` | Sort then two pointers |
| `bitmask` | Bitmask instead of set |
| `boyer_moore` | Boyer-Moore voting |
| `xor` | XOR bit manipulation |
| `math` | Mathematical formula |

---

## Code style per language

All solution files follow the same structure:
1. Necessary imports/includes
2. One clean function (no class wrapper unless language requires it)
3. A `main` / `print` call demonstrating the function on 1–2 examples

### Python

```python
from typing import List
from collections import defaultdict

def groupAnagrams(strs: List[str]) -> List[List[str]]:
    groups = defaultdict(list)
    for s in strs:
        key = tuple(sorted(s))
        groups[key].append(s)
    return list(groups.values())

print(groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]))
# [['eat', 'tea', 'ate'], ['tan', 'nat'], ['bat']]
```

Rules:
- Type hints on function signature
- `from typing import List, Dict, Tuple, Optional` as needed
- `from collections import defaultdict, Counter, deque` as needed
- `print(...)` at bottom for demo

### C++

```cpp
#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>

std::vector<std::vector<std::string>> groupAnagrams(std::vector<std::string>& strs) {
    std::unordered_map<std::string, std::vector<std::string>> groups;
    for (const std::string& s : strs) {
        std::string key = s;
        std::sort(key.begin(), key.end());
        groups[key].push_back(s);
    }
    std::vector<std::vector<std::string>> result;
    for (auto& [k, v] : groups) result.push_back(v);
    return result;
}

int main() {
    std::vector<std::string> strs = {"eat", "tea", "tan", "ate", "nat", "bat"};
    auto result = groupAnagrams(strs);
    for (const auto& group : result) {
        for (const std::string& s : group) std::cout << s << " ";
        std::cout << "\n";
    }
    return 0;
}
```

Rules:
- Include only what's used
- Use `std::` prefix (no `using namespace std`)
- Const references for read-only params: `const std::string& s`
- `main()` returns `int`, ends with `return 0`

### Java

```java
import java.util.*;

public class Main {
    public static List<List<String>> groupAnagrams(String[] strs) {
        Map<String, List<String>> groups = new HashMap<>();
        for (String s : strs) {
            char[] arr = s.toCharArray();
            Arrays.sort(arr);
            String key = new String(arr);
            groups.computeIfAbsent(key, k -> new ArrayList<>()).add(s);
        }
        return new ArrayList<>(groups.values());
    }

    public static void main(String[] args) {
        String[] strs = {"eat", "tea", "tan", "ate", "nat", "bat"};
        System.out.println(groupAnagrams(strs));
    }
}
```

Rules:
- Always `public class Main`
- `public static` methods (no instance needed)
- `import java.util.*` covers most needs
- `main` signature: `public static void main(String[] args)`

### JavaScript

```javascript
function groupAnagrams(strs) {
    const groups = new Map();
    for (const s of strs) {
        const key = s.split('').sort().join('');
        if (!groups.has(key)) groups.set(key, []);
        groups.get(key).push(s);
    }
    return Array.from(groups.values());
}

console.log(groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]));
```

Rules:
- No TypeScript, no ES module syntax (`import`/`export`)
- `const`/`let`, no `var`
- `console.log(...)` at bottom for demo
- Arrow functions acceptable for callbacks

### Rust

```rust
use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort_unstable();
        groups.entry(key).or_insert_with(Vec::new).push(s);
    }
    groups.into_values().collect()
}

fn main() {
    let strs = vec![
        "eat".to_string(), "tea".to_string(), "tan".to_string(),
        "ate".to_string(), "nat".to_string(), "bat".to_string(),
    ];
    println!("{:?}", group_anagrams(strs));
}
```

Rules:
- `use std::collections::HashMap` (or HashSet, BTreeMap, etc.)
- Snake_case for function and variable names
- `fn main()` at bottom
- `println!("{:?}", result)` for debug output
- No `impl Solution` struct — just plain functions
- Handle `usize` underflow with `saturating_sub` when needed
- Use `.to_string()` for &str → String conversion

### Go

```go
package main

import (
    "fmt"
    "sort"
    "strings"
)

func groupAnagrams(strs []string) [][]string {
    groups := make(map[string][]string)
    for _, s := range strs {
        runes := []rune(s)
        sort.Slice(runes, func(i, j int) bool { return runes[i] < runes[j] })
        key := string(runes)
        groups[key] = append(groups[key], s)
    }
    result := make([][]string, 0, len(groups))
    for _, v := range groups {
        result = append(result, v)
    }
    return result
}

func main() {
    strs := []string{"eat", "tea", "tan", "ate", "nat", "bat"}
    fmt.Println(groupAnagrams(strs))
}
```

Rules:
- `package main` at top (always)
- Explicit imports, only used packages
- No `math.Abs` / `math.Max` for integers — use explicit if/else comparison
- `fmt.Println(...)` for demo output
- CamelCase for exported functions, camelCase for local vars

---

## Importing in MDX

```typescript
// One import per language per approach
import pyHashMap     from '@solutions/Python/two_sum_hash_map.py?raw';
import cppHashMap    from '@solutions/C++/two_sum_hash_map.cpp?raw';
import javaHashMap   from '@solutions/Java/two_sum_hash_map.java?raw';
import jsHashMap     from '@solutions/JavaScript/two_sum_hash_map.js?raw';
import rustHashMap   from '@solutions/Rust/two_sum_hash_map.rs?raw';
import goHashMap     from '@solutions/Golang/two_sum_hash_map.go?raw';
```

Using in Tabs:
```typescript
<Tabs>
  <TabItem label="Python">
    <Code code={pyHashMap} lang="python" title="two_sum_hash_map.py" />
  </TabItem>
  <TabItem label="C++">
    <Code code={cppHashMap} lang="cpp" title="two_sum_hash_map.cpp" />
  </TabItem>
  <TabItem label="Java">
    <Code code={javaHashMap} lang="java" title="two_sum_hash_map.java" />
  </TabItem>
  <TabItem label="JavaScript">
    <Code code={jsHashMap} lang="javascript" title="two_sum_hash_map.js" />
  </TabItem>
  <TabItem label="Rust">
    <Code code={rustHashMap} lang="rust" title="two_sum_hash_map.rs" />
  </TabItem>
  <TabItem label="Go">
    <Code code={goHashMap} lang="go" title="two_sum_hash_map.go" />
  </TabItem>
</Tabs>
```

Tab label order is always: Python, C++, Java, JavaScript, Rust, Go.
