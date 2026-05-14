import java.util.ArrayList;
import java.util.List;

class ListNode {
    int val;
    ListNode next;

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Reverse a portion of a linked list from position left to right using recursion.
 *
 * Time Complexity: O(n) - traverse to left, then reverse to right
 * Space Complexity: O(n) - recursion call stack
 */
class Solution {
    // Global variable to track the successor node
    private ListNode successor = null;

    /**
     * Reverse the first n nodes of a linked list.
     *
     * The global 'successor' variable tracks the node after the nth node,
     * which becomes the tail of the reversed sublist.
     */
    private ListNode reverseN(ListNode head, int n) {
        if (n == 1) {
            // Base case: mark successor as the node after the first node
            successor = head.next;
            return head;
        }

        // Recursively reverse the rest
        ListNode newHead = reverseN(head.next, n - 1);

        // At this point, head.next points to the (n-1)th node
        // We want to reverse: head.next.next = head
        head.next.next = head;

        // Connect head to successor (the node after position n)
        head.next = successor;

        return newHead;
    }

    /**
     * Reverse a portion of a linked list from position left to right.
     *
     * Strategy:
     * 1. If left == 1, call reverseN() to reverse the first 'right' nodes
     * 2. Otherwise, recursively process the next node with adjusted positions
     */
    public ListNode reverseBetween(ListNode head, int left, int right) {
        if (left == 1) {
            return reverseN(head, right);
        } else {
            // Recursively process the rest of the list
            head.next = reverseBetween(head.next, left - 1, right - 1);
            return head;
        }
    }
}

// Test harness
public class reverse_linked_list_ii_recursive {
    // Helper function to create a linked list from an array
    static ListNode createList(int[] values) {
        if (values.length == 0) return null;
        ListNode head = new ListNode(values[0]);
        ListNode current = head;
        for (int i = 1; i < values.length; i++) {
            current.next = new ListNode(values[i]);
            current = current.next;
        }
        return head;
    }

    // Helper function to convert linked list to array for comparison
    static List<Integer> listToArray(ListNode head) {
        List<Integer> result = new ArrayList<>();
        while (head != null) {
            result.add(head.val);
            head = head.next;
        }
        return result;
    }

    // Helper function to print a linked list
    static void printList(ListNode head) {
        List<Integer> values = listToArray(head);
        System.out.println(values);
    }

    public static void main(String[] args) {
        Solution solution = new Solution();

        // Test 1: Reverse middle portion
        ListNode head1 = createList(new int[]{1, 2, 3, 4, 5});
        ListNode result1 = solution.reverseBetween(head1, 2, 4);
        System.out.print("Test 1: ");
        printList(result1);  // [1, 4, 3, 2, 5]

        // Test 2: Single node (no reversal)
        ListNode head2 = createList(new int[]{5});
        ListNode result2 = solution.reverseBetween(head2, 1, 1);
        System.out.print("Test 2: ");
        printList(result2);  // [5]

        // Test 3: Reverse entire list
        ListNode head3 = createList(new int[]{1, 2});
        ListNode result3 = solution.reverseBetween(head3, 1, 2);
        System.out.print("Test 3: ");
        printList(result3);  // [2, 1]

        // Test 4: Reverse from start
        ListNode head4 = createList(new int[]{1, 2, 3, 4, 5});
        ListNode result4 = solution.reverseBetween(head4, 1, 3);
        System.out.print("Test 4: ");
        printList(result4);  // [3, 2, 1, 4, 5]

        // Test 5: Reverse at end
        ListNode head5 = createList(new int[]{1, 2, 3, 4, 5});
        ListNode result5 = solution.reverseBetween(head5, 3, 5);
        System.out.print("Test 5: ");
        printList(result5);  // [1, 2, 5, 4, 3]
    }
}
