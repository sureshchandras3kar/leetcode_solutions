/**
 * Simplify an absolute path using string parsing.
 *
 * Time: O(n) where n is the length of the path
 * Space: O(n) for the result string
 */
pub fn simplify_path(path: String) -> String {
    let mut canonical = String::from("/");
    let mut i = 0;
    let chars: Vec<char> = path.chars().collect();

    while i < chars.len() {
        // Skip slashes
        while i < chars.len() && chars[i] == '/' {
            i += 1;
        }

        if i >= chars.len() {
            break;
        }

        // Extract the next component
        let mut j = i;
        while j < chars.len() && chars[j] != '/' {
            j += 1;
        }

        let component: String = chars[i..j].iter().collect();

        if component == ".." {
            // Go up one level
            if canonical.len() > 1 {
                if let Some(last_slash) = canonical[..canonical.len() - 1].rfind('/') {
                    canonical.truncate(last_slash);
                }
            }
        } else if component != "." {
            // Add the component to canonical path
            if canonical != "/" {
                canonical.push('/');
            }
            canonical.push_str(&component);
        }

        i = j;
    }

    canonical
}

fn main() {
    println!("{}", simplify_path("/a/./b/../../c/".to_string()));  // "/c"
    println!("{}", simplify_path("/a/../../b/../c//.//".to_string()));  // "/c"
    println!("{}", simplify_path("/a//b////c/d//././/..".to_string()));  // "/a/b/c"
    println!("{}", simplify_path("/".to_string()));  // "/"
    println!("{}", simplify_path("/../".to_string()));  // "/"
}
