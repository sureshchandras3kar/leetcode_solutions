public class merge_two_sorted_lists_iterative {

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
     * Merge two sorted linked lists iteratively.
     *
     * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
     * Space Complexity: O(1) excluding the output list
     */
    public static ListNode mergeTwoSortedListsIterative(ListNode list1, ListNode list2) {
        // Create a dummy node to simplify the code
        ListNode dummy = new ListNode(0);
        ListNode current = dummy;

        // Traverse both lists and append the smaller node
        while (list1 != null && list2 != null) {
            if (list1.val <= list2.val) {
                current.next = list1;
                list1 = list1.next;
            } else {
                current.next = list2;
                list2 = list2.next;
            }
            current = current.next;
        }

        // Append the remaining nodes
        if (list1 != null) {
            current.next = list1;
        } else {
            current.next = list2;
        }

        return dummy.next;
    }

    // Helper function to create a linked list from an array
    static ListNode createLinkedList(int[] values) {
        if (values.length == 0) {
            return null;
        }
        ListNode head = new ListNode(values[0]);
        ListNode current = head;
        for (int i = 1; i < values.length; i++) {
            current.next = new ListNode(values[i]);
            current = current.next;
        }
        return head;
    }

    // Helper function to print linked list
    static void printLinkedList(ListNode node) {
        System.out.print("[");
        boolean first = true;
        while (node != null) {
            if (!first) System.out.print(", ");
            System.out.print(node.val);
            first = false;
            node = node.next;
        }
        System.out.println("]");
    }

    // Test cases
    public static void main(String[] args) {
        ListNode list1 = createLinkedList(new int[]{1, 2, 4});
        ListNode list2 = createLinkedList(new int[]{1, 3, 4});
        ListNode result = mergeTwoSortedListsIterative(list1, list2);
        printLinkedList(result);  // [1, 1, 2, 3, 4, 4]

        list1 = createLinkedList(new int[]{});
        list2 = createLinkedList(new int[]{});
        result = mergeTwoSortedListsIterative(list1, list2);
        printLinkedList(result);  // []

        list1 = createLinkedList(new int[]{});
        list2 = createLinkedList(new int[]{0});
        result = mergeTwoSortedListsIterative(list1, list2);
        printLinkedList(result);  // [0]
    }
}
