import java.util.ArrayList;
import java.util.List;

class ListNode {
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

class RotateListBreakPoint {
    /**
     * Rotate a linked list to the right by k positions using the break point approach.
     *
     * Approach:
     * 1. Calculate the effective rotation: k = k % length
     * 2. Find the break point: the node at position (length - k - 1)
     * 3. Perform rotation by breaking the list at the break point
     *
     * Time: O(n) - single pass to find length, single pass to find break point
     * Space: O(1) - only using pointers
     */
    public static ListNode rotateListBreakPoint(ListNode head, int k) {
        if (head == null || head.next == null || k == 0) {
            return head;
        }

        // Step 1: Find the length of the list
        int length = 0;
        ListNode current = head;
        while (current != null) {
            length++;
            current = current.next;
        }

        // Step 2: Normalize k (handle cases where k > length)
        k = k % length;
        if (k == 0) {
            return head;
        }

        // Step 3: Find the break point (node before rotation point)
        // We need to find the (length - k - 1)th node
        current = head;
        for (int i = 0; i < length - k - 1; i++) {
            current = current.next;
        }

        // Step 4: Perform rotation
        // The new head is current.next
        ListNode newHead = current.next;
        current.next = null;

        // Find the tail of the new list and connect it back to the old head
        ListNode tail = newHead;
        while (tail.next != null) {
            tail = tail.next;
        }
        tail.next = head;

        return newHead;
    }

    // Helper function to create a linked list from an array
    public static ListNode createLinkedList(int[] values) {
        if (values.length == 0) return null;
        ListNode head = new ListNode(values[0]);
        ListNode current = head;
        for (int i = 1; i < values.length; i++) {
            current.next = new ListNode(values[i]);
            current = current.next;
        }
        return head;
    }

    // Helper function to convert linked list to list for easy printing
    public static List<Integer> linkedListToList(ListNode head) {
        List<Integer> result = new ArrayList<>();
        ListNode current = head;
        while (current != null) {
            result.add(current.val);
            current = current.next;
        }
        return result;
    }

    // Test cases
    public static void main(String[] args) {
        // Test 1: [1, 2, 3, 4, 5], k=2
        ListNode head1 = createLinkedList(new int[]{1, 2, 3, 4, 5});
        ListNode result1 = rotateListBreakPoint(head1, 2);
        System.out.println(linkedListToList(result1));  // [4, 5, 1, 2, 3]

        // Test 2: [0, 1, 2], k=4
        ListNode head2 = createLinkedList(new int[]{0, 1, 2});
        ListNode result2 = rotateListBreakPoint(head2, 4);
        System.out.println(linkedListToList(result2));  // [2, 0, 1]

        // Test 3: [1], k=1
        ListNode head3 = createLinkedList(new int[]{1});
        ListNode result3 = rotateListBreakPoint(head3, 1);
        System.out.println(linkedListToList(result3));  // [1]

        // Test 4: [1, 2], k=2
        ListNode head4 = createLinkedList(new int[]{1, 2});
        ListNode result4 = rotateListBreakPoint(head4, 2);
        System.out.println(linkedListToList(result4));  // [1, 2]

        // Test 5: [1, 2, 3, 4, 5], k=0
        ListNode head5 = createLinkedList(new int[]{1, 2, 3, 4, 5});
        ListNode result5 = rotateListBreakPoint(head5, 0);
        System.out.println(linkedListToList(result5));  // [1, 2, 3, 4, 5]
    }
}
