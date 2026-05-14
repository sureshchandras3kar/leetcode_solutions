class MinStack:
    """Min Stack using two stacks (dual stack approach).

    Maintains a main stack for all values and a separate min stack
    that tracks the minimum value at each level.

    Time: O(1) per operation
    Space: O(n)
    """

    def __init__(self):
        self.main_stack = []
        self.min_stack = []

    def push(self, val: int) -> None:
        self.main_stack.append(val)
        # Push to min_stack the minimum of val and current min
        if not self.min_stack:
            self.min_stack.append(val)
        else:
            self.min_stack.append(min(val, self.min_stack[-1]))

    def pop(self) -> None:
        self.main_stack.pop()
        self.min_stack.pop()

    def top(self) -> int:
        return self.main_stack[-1]

    def getMin(self) -> int:
        return self.min_stack[-1]


# Example usage
stack = MinStack()
stack.push(-2)
stack.push(0)
stack.push(-3)
print(f"Min: {stack.getMin()}")  # -3
stack.pop()
print(f"Top: {stack.top()}")     # 0
print(f"Min: {stack.getMin()}")  # -2
