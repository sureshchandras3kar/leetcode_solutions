/**
 * Simplify an absolute path using a stack approach.
 *
 * Time: O(n) where n is the length of the path
 * Space: O(n) for the stack storing directory names
 */
pub fn simplify_path(path: String) -> String {
    let mut stack: Vec<String> = Vec::new();

    for component in path.split('/') {
        if component.is_empty() || component == "." {
            // Skip empty strings and current directory references
            continue;
        } else if component == ".." {
            // Go up one directory if possible
            stack.pop();
        } else {
            // Add directory name to stack
            stack.push(component.to_string());
        }
    }

    // Reconstruct the path
    "/".to_string() + &stack.join("/")
}

fn main() {
    println!("{}", simplify_path("/a/./b/../../c/".to_string()));  // "/c"
    println!("{}", simplify_path("/a/../../b/../c//.//".to_string()));  // "/c"
    println!("{}", simplify_path("/a//b////c/d//././/..".to_string()));  // "/a/b/c"
    println!("{}", simplify_path("/".to_string()));  // "/"
    println!("{}", simplify_path("/../".to_string()));  // "/"
}
