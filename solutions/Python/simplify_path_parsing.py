from typing import List


def simplify_path_parsing(path: str) -> str:
    """
    Simplify an absolute path using string parsing.

    Time: O(n) where n is the length of the path
    Space: O(n) for the result string
    """
    # Start with root
    canonical = "/"
    i = 0

    while i < len(path):
        # Skip slashes
        while i < len(path) and path[i] == '/':
            i += 1

        if i >= len(path):
            break

        # Extract the next component
        j = i
        while j < len(path) and path[j] != '/':
            j += 1

        component = path[i:j]

        if component == '..':
            # Go up one level (remove last component from canonical)
            # Find the last '/' and remove everything after it
            last_slash = canonical.rfind('/')
            if last_slash > 0:  # Don't go above root
                canonical = canonical[:last_slash]
        elif component != '.':
            # Add the component to canonical path
            if canonical != '/':
                canonical += '/'
            canonical += component
        # If component == '.', we skip it (stay in current directory)

        i = j

    return canonical


if __name__ == "__main__":
    # Test cases
    print(simplify_path_parsing("/a/./b/../../c/"))  # "/c"
    print(simplify_path_parsing("/a/../../b/../c//.//"))  # "/c"
    print(simplify_path_parsing("/a//b////c/d//././/.."))  # "/a/b/c"
    print(simplify_path_parsing("/"))  # "/"
    print(simplify_path_parsing("/../"))  # "/"
