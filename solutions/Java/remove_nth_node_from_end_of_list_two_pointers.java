public class remove_nth_node_from_end_of_list_two_pointers {

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
     * Remove the nth node from the end of a linked list using two pointers in one
     * pass.
     *
     * Approach:
     * 1. Create a dummy node pointing to head (handles edge case of removing head)
     * 2. Use fast and slow pointers with a gap of n nodes between them
     * 3. Move both pointers until fast reaches the end
     * 4. Skip the target node by adjusting the slow pointer
     *
     * Time: O(L) single pass
     * Space: O(1)
     */
    public static ListNode removeNthNode(ListNode head, int n) {
        // Create a dummy node to handle edge case of removing the head
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode slow = dummy;
        ListNode fast = dummy;

        // Move fast pointer n+1 steps ahead
        for (int i = 0; i <= n; i++) {
            if (fast == null)
                return head;
            fast = fast.next;
        }

        // Move both pointers until fast reaches the end
        while (fast != null) {
            slow = slow.next;
            fast = fast.next;
        }

        // Remove the nth node
        slow.next = slow.next.next;
        return dummy.next;
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
