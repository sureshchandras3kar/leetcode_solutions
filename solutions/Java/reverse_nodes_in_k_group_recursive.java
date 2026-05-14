import java.util.*;

class ListNode {
    int val;
    ListNode next;

    ListNode(int val) {
        this.val = val;
        this.next = null;
    }
}

public class ReverseNodesInKGroupRecursive {
    /**
     * Reverse nodes in k-group recursively.
     *
     * Approach: Use recursion to handle each group independently.
     * - Check if there are k nodes remaining
     * - Reverse the first k nodes
     * - Recursively process the rest of the list
     * - Connect the reversed group to the result of the recursive call
     *
     * Time: O(n) - visit each node once
     * Space: O(n/k) - recursion depth is proportional to number of groups
     */

    public static ListNode reverseKGroup(ListNode head, int k) {
        // Find the k-th node
        ListNode kth = head;
        for (int i = 0; i < k - 1 && kth != null; i++) {
            kth = kth.next;
        }

        // If no k-th node exists, cannot reverse, return original
        if (kth == null) {
            return head;
        }

        // Save the next group's head (after k-th node)
        ListNode nextGroupHead = kth.next;

        // Reverse from head to kth
        ListNode prev = nextGroupHead;
        ListNode curr = head;

        while (curr != nextGroupHead) {
            ListNode nextTemp = curr.next;
            curr.next = prev;
            prev = curr;
            curr = nextTemp;
        }

        // head is now the tail of the reversed group
        // prev is the new head (which was kth)
        // Recursively process the remaining list and connect
        head.next = reverseKGroup(nextGroupHead, k);

        return prev;  // Return the new head (which was kth)
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
