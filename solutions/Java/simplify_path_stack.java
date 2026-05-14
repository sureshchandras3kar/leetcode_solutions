import java.util.*;

public class SimplifyPathStack {
    public static String simplifyPath(String path) {
        /**
         * Simplify an absolute path using a stack approach.
         *
         * Time: O(n) where n is the length of the path
         * Space: O(n) for the stack storing directory names
         */
        Stack<String> stack = new Stack<>();
        String[] components = path.split("/");

        for (String component : components) {
            if (component.isEmpty() || component.equals(".")) {
                // Skip empty strings and current directory references
                continue;
            } else if (component.equals("..")) {
                // Go up one directory if possible
                if (!stack.isEmpty()) {
                    stack.pop();
                }
            } else {
                // Add directory name to stack
                stack.push(component);
            }
        }

        // Reconstruct the path
        StringBuilder result = new StringBuilder("/");
        for (int i = 0; i < stack.size(); i++) {
            if (i > 0) result.append("/");
            result.append(stack.get(i));
        }

        return result.toString();
    }

    public static void main(String[] args) {
        System.out.println(simplifyPath("/a/./b/../../c/"));  // "/c"
        System.out.println(simplifyPath("/a/../../b/../c//.//"));  // "/c"
        System.out.println(simplifyPath("/a//b////c/d//././/.."));  // "/a/b/c"
        System.out.println(simplifyPath("/"));  // "/"
        System.out.println(simplifyPath("/../"));  // "/"
    }
}
