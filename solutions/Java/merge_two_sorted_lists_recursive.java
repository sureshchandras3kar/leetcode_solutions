public class merge_two_sorted_lists_recursive {

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
     * Merge two sorted linked lists recursively.
     *
     * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
     * Space Complexity: O(m + n) due to the recursion call stack
     */
    public static ListNode mergeTwoSortedListsRecursive(ListNode list1, ListNode list2) {
        // Base cases: if one list is empty, return the other
        if (list1 == null) {
            return list2;
        }
        if (list2 == null) {
            return list1;
        }

        // Compare the values and recursively merge
        if (list1.val <= list2.val) {
            list1.next = mergeTwoSortedListsRecursive(list1.next, list2);
            return list1;
        } else {
            list2.next = mergeTwoSortedListsRecursive(list1, list2.next);
            return list2;
        }
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
        ListNode result = mergeTwoSortedListsRecursive(list1, list2);
        printLinkedList(result);  // [1, 1, 2, 3, 4, 4]

        list1 = createLinkedList(new int[]{});
        list2 = createLinkedList(new int[]{});
        result = mergeTwoSortedListsRecursive(list1, list2);
        printLinkedList(result);  // []

        list1 = createLinkedList(new int[]{});
        list2 = createLinkedList(new int[]{0});
        result = mergeTwoSortedListsRecursive(list1, list2);
        printLinkedList(result);  // [0]
    }
}
