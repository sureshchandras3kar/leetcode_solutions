import java.util.HashMap;
import java.util.Map;

class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
    /**
     * Remove all nodes with duplicate values from sorted linked list using hash set.
     *
     * Approach: Count frequency of each value, then rebuild list with unique values only.
     * Time: O(n) — two passes through the list
     * Space: O(n) — hash map stores frequencies
     */
    public ListNode deleteDuplicates(ListNode head) {
        if (head == null) return null;

        // Count frequencies
        Map<Integer, Integer> freq = new HashMap<>();
        ListNode current = head;
        while (current != null) {
            freq.put(current.val, freq.getOrDefault(current.val, 0) + 1);
            current = current.next;
        }

        // Build result list with unique values only
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode prev = dummy;
        current = head;

        while (current != null) {
            if (freq.get(current.val) == 1) {
                // Keep unique node
                prev = current;
                current = current.next;
            } else {
                // Skip duplicate node
                prev.next = current.next;
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
