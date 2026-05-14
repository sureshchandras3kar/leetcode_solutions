/**
 * Simplify an absolute path using string parsing.
 *
 * Time: O(n) where n is the length of the path
 * Space: O(n) for the result string
 *
 * @param {string} path - The absolute file path to simplify
 * @returns {string} - The simplified path
 */
function simplifyPathParsing(path) {
    let canonical = "/";
    let i = 0;

    while (i < path.length) {
        // Skip slashes
        while (i < path.length && path[i] === '/') {
            i++;
        }

        if (i >= path.length) {
            break;
        }

        // Extract the next component
        let j = i;
        while (j < path.length && path[j] !== '/') {
            j++;
        }

        const component = path.substring(i, j);

        if (component === '..') {
            // Go up one level
            const lastSlash = canonical.lastIndexOf('/');
            if (lastSlash > 0) {  // Don't go above root
                canonical = canonical.substring(0, lastSlash);
            }
        } else if (component !== '.') {
            // Add the component to canonical path
            if (canonical !== '/') {
                canonical += '/';
            }
            canonical += component;
        }

        i = j;
    }

    return canonical;
}

// Test cases
console.log(simplifyPathParsing("/a/./b/../../c/"));  // "/c"
console.log(simplifyPathParsing("/a/../../b/../c//.//"));  // "/c"
console.log(simplifyPathParsing("/a//b////c/d//././/.."));  // "/a/b/c"
console.log(simplifyPathParsing("/"));  // "/"
console.log(simplifyPathParsing("/../"));  // "/"
