#include <iostream>
#include <string>
#include <vector>
#include <sstream>

std::string simplifyPathStack(std::string path) {
    // Split path by '/' and filter empty strings and '.'
    std::vector<std::string> stack;
    std::stringstream ss(path);
    std::string component;

    while (std::getline(ss, component, '/')) {
        if (component.empty() || component == ".") {
            // Skip empty strings and current directory references
            continue;
        } else if (component == "..") {
            // Go up one directory if possible
            if (!stack.empty()) {
                stack.pop_back();
            }
        } else {
            // Add directory name to stack
            stack.push_back(component);
        }
    }

    // Reconstruct the path
    std::string result = "/";
    for (int i = 0; i < (int)stack.size(); i++) {
        if (i > 0) result += "/";
        result += stack[i];
    }

    return result;
}

int main() {
    std::cout << simplifyPathStack("/a/./b/../../c/") << std::endl;  // "/c"
    std::cout << simplifyPathStack("/a/../../b/../c//.//") << std::endl;  // "/c"
    std::cout << simplifyPathStack("/a//b////c/d//././/..") << std::endl;  // "/a/b/c"
    std::cout << simplifyPathStack("/") << std::endl;  // "/"
    std::cout << simplifyPathStack("/../") << std::endl;  // "/"
    return 0;
}
