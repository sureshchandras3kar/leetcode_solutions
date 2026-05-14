/**
 * Simplify an absolute path using a stack approach.
 *
 * Time: O(n) where n is the length of the path
 * Space: O(n) for the stack storing directory names
 *
 * @param {string} path - The absolute file path to simplify
 * @returns {string} - The simplified path
 */
function simplifyPathStack(path) {
    // Split path by '/' and filter empty strings and '.'
    const components = path.split('/').filter(c => c && c !== '.');
    const stack = [];

    for (const component of components) {
        if (component === '..') {
            // Go up one directory if possible
            if (stack.length > 0) {
                stack.pop();
            }
        } else {
            // Add directory name to stack
            stack.push(component);
        }
    }

    // Reconstruct the path
    return '/' + stack.join('/');
}

// Test cases
console.log(simplifyPathStack("/a/./b/../../c/"));  // "/c"
console.log(simplifyPathStack("/a/../../b/../c//.//"));  // "/c"
console.log(simplifyPathStack("/a//b////c/d//././/.."));  // "/a/b/c"
console.log(simplifyPathStack("/"));  // "/"
console.log(simplifyPathStack("/../"));  // "/"
