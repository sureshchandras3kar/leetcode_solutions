class ListNode {
    int val;
    ListNode next;
    ListNode(int val) {
        this.val = val;
    }
}

class Solution {
    public ListNode sortList(ListNode head) {
        if (head == null) return head;
        
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        
        int length = 0;
        ListNode curr = head;
        while (curr != null) {
            length++;
            curr = curr.next;
        }
        
        for (int size = 1; size < length; size *= 2) {
            ListNode prev = dummy;
            curr = dummy.next;
            
            while (curr != null) {
                ListNode l1 = curr;
                ListNode l1_tail = l1;
                int l1_len = 0;
                while (l1_len < size && l1_tail != null) {
                    l1_tail = l1_tail.next;
                    l1_len++;
                }
                
                ListNode l2 = l1_tail;
                int l2_len = 0;
                while (l2_len < size && l2 != null) {
                    l2 = l2.next;
                    l2_len++;
                }
                
                l1_tail = l1;
                for (int i = 1; i < l1_len; i++) {
                    if (l1_tail != null) l1_tail = l1_tail.next;
                }
                if (l1_tail != null) l1_tail.next = null;
                
                prev.next = merge(l1, l2);
                while (prev.next != null) prev = prev.next;
                curr = l2;
            }
        }
        
        return dummy.next;
    }
    
    private ListNode merge(ListNode l1, ListNode l2) {
        ListNode dummy = new ListNode(0);
        ListNode curr = dummy;
        while (l1 != null && l2 != null) {
            if (l1.val <= l2.val) {
                curr.next = l1;
                l1 = l1.next;
            } else {
                curr.next = l2;
                l2 = l2.next;
            }
            curr = curr.next;
        }
        curr.next = l1 != null ? l1 : l2;
        return dummy.next;
    }
}

public class SortList_Iterative {
    public static void main(String[] args) {
        ListNode head = new ListNode(4);
        head.next = new ListNode(2);
        head.next.next = new ListNode(1);
        head.next.next.next = new ListNode(3);
        
        Solution sol = new Solution();
        ListNode result = sol.sortList(head);
        while (result != null) {
            System.out.print(result.val + " ");
            result = result.next;
        }
        System.out.println();
    }
}
