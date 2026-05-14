import java.util.*;

class ListNode {
    int val;
    ListNode next;

    ListNode() {
    }

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }
}

public class partition_list_two_heads {
    /**
     * Partition linked list into two groups: values < x and values >= x.
     * Uses two separate list heads and merges them at the end.
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(1) excluding the output list
     */
    public static ListNode partitionListTwoHeads(ListNode head, int x) {
        // Create dummy nodes for two lists
        ListNode smaller_dummy = new ListNode(0);
        ListNode larger_dummy = new ListNode(0);

        // Pointers to build the two lists
        ListNode smaller_ptr = smaller_dummy;
        ListNode larger_ptr = larger_dummy;

        // Partition nodes into two lists
        ListNode current = head;
        while (current != null) {
            if (current.val < x) {
                smaller_ptr.next = current;
                smaller_ptr = smaller_ptr.next;
            } else {
                larger_ptr.next = current;
                larger_ptr = larger_ptr.next;
            }
            current = current.next;
        }

        // Close the larger list to avoid cycles
        larger_ptr.next = null;

        // Connect the two lists
        smaller_ptr.next = larger_dummy.next;

        return smaller_dummy.next;
    }

    // Helper function to create a linked list from an array
    public static ListNode createLinkedList(int[] values) {
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

    // Helper function to convert linked list to list for easy testing
    public static List<Integer> linkedListToList(ListNode node) {
        List<Integer> result = new ArrayList<>();
        while (node != null) {
            result.add(node.val);
            node = node.next;
        }
        return result;
    }

    // Test cases
    public static void main(String[] args) {
        ListNode head = createLinkedList(new int[]{1, 4, 3, 2, 5, 2});
        ListNode result = partitionListTwoHeads(head, 3);
        System.out.println(linkedListToList(result));  // [1, 2, 2, 4, 3, 5]

        head = createLinkedList(new int[]{2, 1});
        result = partitionListTwoHeads(head, 2);
        System.out.println(linkedListToList(result));  // [1, 2]

        head = createLinkedList(new int[]{5, 3, 1, 4, 2});
        result = partitionListTwoHeads(head, 3);
        System.out.println(linkedListToList(result));  // [1, 2, 5, 3, 4]
    }
}
