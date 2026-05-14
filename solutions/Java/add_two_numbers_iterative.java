import java.util.ArrayList;
import java.util.List;

class ListNode {
    int val;
    ListNode next;
    ListNode(int x) {
        val = x;
    }
}

public class add_two_numbers_iterative {
    /**
     * Add two numbers represented by linked lists in reverse order.
     *
     * Time Complexity: O(max(m, n)) where m and n are the lengths of the lists
     * Space Complexity: O(max(m, n)) for the output list
     */
    public static ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode dummy = new ListNode(0);
        ListNode current = dummy;
        int carry = 0;

        while (l1 != null || l2 != null || carry != 0) {
            int val1 = l1 != null ? l1.val : 0;
            int val2 = l2 != null ? l2.val : 0;

            int total = val1 + val2 + carry;
            carry = total / 10;
            int digit = total % 10;

            current.next = new ListNode(digit);
            current = current.next;

            l1 = l1 != null ? l1.next : null;
            l2 = l2 != null ? l2.next : null;
        }

        return dummy.next;
    }

    // Helper function to create linked list from array
    private static ListNode createLinkedList(int[] arr) {
        if (arr.length == 0) return null;
        ListNode head = new ListNode(arr[0]);
        ListNode current = head;
        for (int i = 1; i < arr.length; i++) {
            current.next = new ListNode(arr[i]);
            current = current.next;
        }
        return head;
    }

    // Helper function to convert linked list to list for printing
    private static List<Integer> linkedListToList(ListNode head) {
        List<Integer> result = new ArrayList<>();
        ListNode current = head;
        while (current != null) {
            result.add(current.val);
            current = current.next;
        }
        return result;
    }

    public static void main(String[] args) {
        // Test case 1: [2,4,3] + [5,6,4] = [7,0,8] (342 + 465 = 807)
        ListNode l1 = createLinkedList(new int[]{2, 4, 3});
        ListNode l2 = createLinkedList(new int[]{5, 6, 4});
        ListNode result = addTwoNumbers(l1, l2);
        System.out.println("Test 1: " + linkedListToList(result));  // [7, 0, 8]

        // Test case 2: [0] + [0] = [0]
        l1 = createLinkedList(new int[]{0});
        l2 = createLinkedList(new int[]{0});
        result = addTwoNumbers(l1, l2);
        System.out.println("Test 2: " + linkedListToList(result));  // [0]

        // Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
        l1 = createLinkedList(new int[]{9, 9, 9, 9, 9, 9, 9});
        l2 = createLinkedList(new int[]{9, 9, 9, 9});
        result = addTwoNumbers(l1, l2);
        System.out.println("Test 3: " + linkedListToList(result));  // [8, 9, 9, 9, 0, 0, 0, 1]
    }
}
