import java.util.*;

class ListNode {
    int val;
    ListNode next;

    ListNode(int val) {
        this.val = val;
        this.next = null;
    }
}

public class ReverseNodesInKGroupIterative {
    /**
     * Reverse nodes in k-group iteratively.
     *
     * Approach: Reverse each group of k nodes in-place using iteration.
     * - Find the k-th node to determine the group boundary
     * - Reverse the group from current to k-th node
     * - Link the reversed group back to the previous group
     * - Move to the next group and repeat
     *
     * Time: O(n) - visit each node once
     * Space: O(1) - only pointers, no extra structures
     */

    private static ListNode getKthNode(ListNode node, int k) {
        while (node != null && k > 1) {
            node = node.next;
            k--;
        }
        return node;
    }

    public static ListNode reverseKGroup(ListNode head, int k) {
        // Dummy node to simplify logic
        ListNode dummy = new ListNode(0);
        dummy.next = head;

        ListNode prevGroup = dummy;

        while (true) {
            // Check if there are at least k nodes remaining
            ListNode kthNode = getKthNode(prevGroup, k + 1);
            if (kthNode == null) {
                break;
            }

            // Mark the start and end of the current group
            ListNode groupStart = prevGroup.next;
            ListNode groupEnd = kthNode;

            // Save the next group's start
            ListNode nextGroupStart = groupEnd.next;

            // Reverse the current group
            ListNode prev = nextGroupStart;
            ListNode curr = groupStart;

            while (curr != nextGroupStart) {
                ListNode nextTemp = curr.next;
                curr.next = prev;
                prev = curr;
                curr = nextTemp;
            }

            // Link the previous group to the reversed current group
            prevGroup.next = groupEnd;
            prevGroup = groupStart;
        }

        return dummy.next;
    }

    // Helper functions for testing
    private static ListNode createList(int[] values) {
        if (values.length == 0) return null;
        ListNode head = new ListNode(values[0]);
        ListNode current = head;
        for (int i = 1; i < values.length; i++) {
            current.next = new ListNode(values[i]);
            current = current.next;
        }
        return head;
    }

    private static void printList(ListNode head) {
        System.out.print("[");
        while (head != null) {
            System.out.print(head.val);
            if (head.next != null) System.out.print(", ");
            head = head.next;
        }
        System.out.println("]");
    }

    // Test
    public static void main(String[] args) {
        ListNode head1 = createList(new int[]{1, 2, 3, 4, 5});
        printList(reverseKGroup(head1, 2));  // [2, 1, 4, 3, 5]

        ListNode head2 = createList(new int[]{1, 2, 3, 4, 5});
        printList(reverseKGroup(head2, 3));  // [3, 2, 1, 4, 5]

        ListNode head3 = createList(new int[]{1});
        printList(reverseKGroup(head3, 1));  // [1]
    }
}
