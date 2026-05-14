class MinStack:
    """Min Stack using single stack with (value, min) pairs.

    Each element in the stack is a tuple of (value, min_at_this_level).
    The min_at_this_level is the minimum value from the bottom up to
    and including the current element.

    Time: O(1) per operation
    Space: O(n)
    """

    def __init__(self):
        self.stack = []

    def push(self, val: int) -> None:
        if not self.stack:
            # First element: value is the minimum
            self.stack.append((val, val))
        else:
            # Store both value and the minimum up to this point
            current_min = min(val, self.stack[-1][1])
            self.stack.append((val, current_min))

    def pop(self) -> None:
        self.stack.pop()

    def top(self) -> int:
        return self.stack[-1][0]

    def getMin(self) -> int:
        return self.stack[-1][1]


# Example usage
stack = MinStack()
stack.push(-2)
stack.push(0)
stack.push(-3)
print(f"Min: {stack.getMin()}")  # -3
stack.pop()
print(f"Top: {stack.top()}")     # 0
print(f"Min: {stack.getMin()}")  # -2
