public class remove_nth_node_from_end_of_list_two_pass {

    static class ListNode {
        int val;
        ListNode next;

        ListNode() {}

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    /**
     * Remove the nth node from the end of a linked list using two passes.
     *
     * Approach:
     * 1. First pass: count the total number of nodes
     * 2. Second pass: find and skip the nth node from the end
     *
     * Time: O(L) + O(L-n) = O(L)
     * Space: O(1)
     */
    public static ListNode removeNthNode(ListNode head, int n) {
        // First pass: count the total nodes
        int length = 0;
        ListNode current = head;
        while (current != null) {
            length++;
            current = current.next;
        }

        // Edge case: removing the head node
        if (length == n) {
            return head.next;
        }

        // Second pass: find the node before the one to remove
        current = head;
        for (int i = 0; i < length - n - 1; i++) {
            current = current.next;
        }

        // Remove the nth node
        current.next = current.next.next;
        return head;
    }

    // Helper function to create a linked list from an array
    static ListNode createLinkedList(int[] values) {
        if (values.length == 0)
            return null;
        ListNode head = new ListNode(values[0]);
        ListNode current = head;
        for (int i = 1; i < values.length; i++) {
            current.next = new ListNode(values[i]);
            current = current.next;
        }
        return head;
    }

    // Helper function to convert linked list to string for easy printing
    static String linkedListToString(ListNode head) {
        StringBuilder result = new StringBuilder("[");
        ListNode current = head;
        while (current != null) {
            result.append(current.val);
            current = current.next;
            if (current != null) {
                result.append(", ");
            }
        }
        result.append("]");
        return result.toString();
    }

    // Test cases
    public static void main(String[] args) {
        // Test 1: [1, 2, 3, 4, 5], n=2
        ListNode head1 = createLinkedList(new int[]{1, 2, 3, 4, 5});
        ListNode result1 = removeNthNode(head1, 2);
        System.out.println(linkedListToString(result1));  // [1, 2, 3, 5]

        // Test 2: [1], n=1
        ListNode head2 = createLinkedList(new int[]{1});
        ListNode result2 = removeNthNode(head2, 1);
        System.out.println(linkedListToString(result2));  // []

        // Test 3: [1, 2], n=2
        ListNode head3 = createLinkedList(new int[]{1, 2});
        ListNode result3 = removeNthNode(head3, 2);
        System.out.println(linkedListToString(result3));  // [2]

        // Test 4: [1, 2], n=1
        ListNode head4 = createLinkedList(new int[]{1, 2});
        ListNode result4 = removeNthNode(head4, 1);
        System.out.println(linkedListToString(result4));  // [1]
    }
}
