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

class RotateListCircular {
    /**
     * Rotate a linked list to the right by k positions using the circular approach.
     *
     * Approach:
     * 1. Calculate the effective rotation: k = k % length
     * 2. Create a circular list by connecting the tail to the head
     * 3. Find the new head position: walk (length - k) steps from the original head
     * 4. Break the circle at the appropriate point
     *
     * Time: O(n) - single pass to find length and establish circle, walk (length - k) steps
     * Space: O(1) - only using pointers
     */
    public static ListNode rotateListCircular(ListNode head, int k) {
        if (head == null || head.next == null || k == 0) {
            return head;
        }

        // Step 1: Find the length of the list and the tail
        int length = 0;
        ListNode tail = head;
        while (tail.next != null) {
            length++;
            tail = tail.next;
        }
        length++;  // Add 1 for the tail node itself

        // Step 2: Normalize k
        k = k % length;
        if (k == 0) {
            return head;
        }

        // Step 3: Create a circular list
        tail.next = head;

        // Step 4: Find the new head position
        // After rotation by k, the new head is at position (length - k)
        // We need to walk (length - k) steps from the original head
        int stepsToWalk = length - k;
        ListNode current = head;
        for (int i = 0; i < stepsToWalk - 1; i++) {
            current = current.next;
        }

        // Step 5: Break the circle
        ListNode newHead = current.next;
        current.next = null;

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
        ListNode result1 = rotateListCircular(head1, 2);
        System.out.println(linkedListToList(result1));  // [4, 5, 1, 2, 3]

        // Test 2: [0, 1, 2], k=4
        ListNode head2 = createLinkedList(new int[]{0, 1, 2});
        ListNode result2 = rotateListCircular(head2, 4);
        System.out.println(linkedListToList(result2));  // [2, 0, 1]

        // Test 3: [1], k=1
        ListNode head3 = createLinkedList(new int[]{1});
        ListNode result3 = rotateListCircular(head3, 1);
        System.out.println(linkedListToList(result3));  // [1]

        // Test 4: [1, 2], k=2
        ListNode head4 = createLinkedList(new int[]{1, 2});
        ListNode result4 = rotateListCircular(head4, 2);
        System.out.println(linkedListToList(result4));  // [1, 2]

        // Test 5: [1, 2, 3, 4, 5], k=0
        ListNode head5 = createLinkedList(new int[]{1, 2, 3, 4, 5});
        ListNode result5 = rotateListCircular(head5, 0);
        System.out.println(linkedListToList(result5));  // [1, 2, 3, 4, 5]
    }
}
