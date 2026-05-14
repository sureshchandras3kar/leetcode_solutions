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

public class partition_list_single_rearrange {
    /**
     * Partition linked list into two groups: values < x and values >= x.
     * Rearranges nodes in a single pass without creating separate lists.
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(1) excluding the output list
     */
    public static ListNode partitionListSingleRearrange(ListNode head, int x) {
        // Create a dummy node to simplify edge cases
        ListNode dummy = new ListNode(0);
        dummy.next = head;

        // Find the node just before the partition point
        ListNode prev = dummy;
        ListNode current = head;

        while (current != null && current.val < x) {
            prev = current;
            current = current.next;
        }

        // Now prev is the last node with value < x (or dummy if all nodes >= x)
        // current is the first node with value >= x (or null if all nodes < x)

        // If all nodes are less than x or list is empty, return as is
        if (current == null) {
            return dummy.next;
        }

        // Now we need to insert nodes with value < x before the partition point
        ListNode partition_prev = prev;  // Last node of < x group
        ListNode run = current;  // Start from the partition point

        while (run != null) {
            if (run.val < x) {
                // Remove run from its position
                current.next = run.next;

                // Insert run before the partition point
                run.next = partition_prev.next;
                partition_prev.next = run;

                // Move partition_prev forward
                partition_prev = run;

                // Current stays the same to check the next node
                run = current.next;
            } else {
                // Move forward
                current = run;
                run = run.next;
            }
        }

        return dummy.next;
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
        ListNode result = partitionListSingleRearrange(head, 3);
        System.out.println(linkedListToList(result));  // [1, 2, 2, 4, 3, 5]

        head = createLinkedList(new int[]{2, 1});
        result = partitionListSingleRearrange(head, 2);
        System.out.println(linkedListToList(result));  // [1, 2]

        head = createLinkedList(new int[]{5, 3, 1, 4, 2});
        result = partitionListSingleRearrange(head, 3);
        System.out.println(linkedListToList(result));  // [1, 2, 5, 3, 4]
    }
}
