function generateParenthesis(n) {
    const result = [];

    function backtrack(current, left, right) {
        if (current.length === 2 * n) {
            result.push(current);
            return;
        }
        if (left < n) backtrack(current + "(", left + 1, right);
        if (right < left) backtrack(current + ")", left, right + 1);
    }

    backtrack("", 0, 0);
    return result;
}
