#include <iostream>
#include <string>

std::string simplifyPathParsing(std::string path) {
    std::string canonical = "/";
    int i = 0;

    while (i < (int)path.length()) {
        // Skip slashes
        while (i < (int)path.length() && path[i] == '/') {
            i++;
        }

        if (i >= (int)path.length()) {
            break;
        }

        // Extract the next component
        int j = i;
        while (j < (int)path.length() && path[j] != '/') {
            j++;
        }

        std::string component = path.substr(i, j - i);

        if (component == "..") {
            // Go up one level
            int lastSlash = canonical.rfind('/');
            if (lastSlash > 0) {  // Don't go above root
                canonical = canonical.substr(0, lastSlash);
            }
        } else if (component != ".") {
            // Add the component to canonical path
            if (canonical != "/") {
                canonical += "/";
            }
            canonical += component;
        }

        i = j;
    }

    return canonical;
}

int main() {
    std::cout << simplifyPathParsing("/a/./b/../../c/") << std::endl;  // "/c"
    std::cout << simplifyPathParsing("/a/../../b/../c//.//") << std::endl;  // "/c"
    std::cout << simplifyPathParsing("/a//b////c/d//././/..") << std::endl;  // "/a/b/c"
    std::cout << simplifyPathParsing("/") << std::endl;  // "/"
    std::cout << simplifyPathParsing("/../") << std::endl;  // "/"
    return 0;
}
