public class SimplifyPathParsing {
    public static String simplifyPath(String path) {
        /**
         * Simplify an absolute path using string parsing.
         *
         * Time: O(n) where n is the length of the path
         * Space: O(n) for the result string
         */
        StringBuilder canonical = new StringBuilder("/");
        int i = 0;

        while (i < path.length()) {
            // Skip slashes
            while (i < path.length() && path.charAt(i) == '/') {
                i++;
            }

            if (i >= path.length()) {
                break;
            }

            // Extract the next component
            int j = i;
            while (j < path.length() && path.charAt(j) != '/') {
                j++;
            }

            String component = path.substring(i, j);

            if (component.equals("..")) {
                // Go up one level
                int len = canonical.length();
                if (len > 1) {  // Don't go above root
                    canonical.setLength(canonical.lastIndexOf("/"));
                }
            } else if (!component.equals(".")) {
                // Add the component to canonical path
                if (!canonical.toString().equals("/")) {
                    canonical.append("/");
                }
                canonical.append(component);
            }

            i = j;
        }

        return canonical.toString();
    }

    public static void main(String[] args) {
        System.out.println(simplifyPath("/a/./b/../../c/"));  // "/c"
        System.out.println(simplifyPath("/a/../../b/../c//.//"));  // "/c"
        System.out.println(simplifyPath("/a//b////c/d//././/.."));  // "/a/b/c"
        System.out.println(simplifyPath("/"));  // "/"
        System.out.println(simplifyPath("/../"));  // "/"
    }
}
