class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
    /**
     * Remove all nodes with duplicate values from sorted linked list in single pass.
     *
     * Approach: Use a dummy node and prev pointer. When we find duplicates, skip all
     * nodes with that value by advancing current and updating prev.next.
     * Time: O(n) — single pass through the list
     * Space: O(1) — only pointer variables
     */
    public ListNode deleteDuplicates(ListNode head) {
        if (head == null) return null;

        // Create dummy node to handle edge case where head is duplicate
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode prev = dummy;
        ListNode current = head;

        while (current != null) {
            // Check if current node is the start of a duplicate group
            if (current.next != null && current.val == current.next.val) {
                // Skip all nodes with the same value
                int value = current.val;
                while (current != null && current.val == value) {
                    current = current.next;
                }
                // Link prev to the first non-duplicate node
                prev.next = current;
            } else {
                // Current node is unique, keep it
                prev = current;
                current = current.next;
            }
        }

        return dummy.next;
    }

    // Helper function to create linked list from array
    public static ListNode createList(int[] arr) {
        if (arr.length == 0) return null;
        ListNode head = new ListNode(arr[0]);
        ListNode current = head;
        for (int i = 1; i < arr.length; i++) {
            current.next = new ListNode(arr[i]);
            current = current.next;
        }
        return head;
    }

    // Helper function to print linked list
    public static void printList(ListNode head) {
        while (head != null) {
            System.out.print(head.val + " -> ");
            head = head.next;
        }
        System.out.println("null");
    }

    public static void main(String[] args) {
        Solution sol = new Solution();

        // Test case 1: [1,2,3,3,4,4,5]
        ListNode head1 = createList(new int[]{1, 2, 3, 3, 4, 4, 5});
        System.out.print("Test 1: ");
        printList(sol.deleteDuplicates(head1));  // 1 -> 2 -> 5 -> null

        // Test case 2: [1,1,1,2,3]
        ListNode head2 = createList(new int[]{1, 1, 1, 2, 3});
        System.out.print("Test 2: ");
        printList(sol.deleteDuplicates(head2));  // 2 -> 3 -> null

        // Test case 3: [1,1]
        ListNode head3 = createList(new int[]{1, 1});
        System.out.print("Test 3: ");
        printList(sol.deleteDuplicates(head3));  // null
    }
}
