from typing import List


def simplify_path_stack(path: str) -> str:
    """
    Simplify an absolute path using a stack approach.

    Time: O(n) where n is the length of the path
    Space: O(n) for the stack storing directory names
    """
    # Split path by '/' and filter empty strings
    parts = [part for part in path.split('/') if part and part != '.']

    stack = []
    for part in parts:
        if part == '..':
            # Go up one directory if possible
            if stack:
                stack.pop()
        else:
            # Add directory name to stack
            stack.append(part)

    # Reconstruct the path
    return '/' + '/'.join(stack)


if __name__ == "__main__":
    # Test cases
    print(simplify_path_stack("/a/./b/../../c/"))  # "/c"
    print(simplify_path_stack("/a/../../b/../c//.//"))  # "/c"
    print(simplify_path_stack("/a//b////c/d//././/.."))  # "/a/b/c"
    print(simplify_path_stack("/"))  # "/"
    print(simplify_path_stack("/../"))  # "/"
