function searchMatrix(matrix, target) {
    if (!matrix || !matrix.length) return false;

    const m = matrix.length, n = matrix[0].length;
    let left = 0, right = m * n - 1;

    while (left <= right) {
        const mid = Math.floor((left + right) / 2);
        const mid_value = matrix[Math.floor(mid / n)][mid % n];

        if (mid_value === target) return true;
        else if (mid_value < target) left = mid + 1;
        else right = mid - 1;
    }

    return false;
}
