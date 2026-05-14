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
 * Reverse a portion of a linked list from position left to right (1-indexed).
 *
 * Time Complexity: O(n) - single pass through the list
 * Space Complexity: O(1) - only pointer variables
 *
 * Strategy:
 * 1. Create a dummy node to handle edge case of reversing from head
 * 2. Advance prev pointer to position left-1
 * 3. Perform (right - left) reversals by moving nodes to the front of the sublist
 * 4. Return dummy.next
 */
class Solution {
    public ListNode reverseBetween(ListNode head, int left, int right) {
        if (left == right) {
            return head;
        }

        // Create dummy node pointing to head
        ListNode dummy = new ListNode(0, head);
        ListNode prev = dummy;

        // Advance prev to position left-1
        for (int i = 0; i < left - 1; i++) {
            prev = prev.next;
        }

        // curr is the first node to reverse (at position left)
        ListNode curr = prev.next;

        // Perform (right - left) reversals
        for (int i = 0; i < right - left; i++) {
            // nextTemp is the node we want to move to the front
            ListNode nextTemp = curr.next;
            // Bypass nextTemp by connecting curr to nextTemp.next
            curr.next = nextTemp.next;
            // Move nextTemp to the front of the sublist
            nextTemp.next = prev.next;
            prev.next = nextTemp;
        }

        return dummy.next;
    }
}

// Test harness
public class reverse_linked_list_ii_iterative {
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
