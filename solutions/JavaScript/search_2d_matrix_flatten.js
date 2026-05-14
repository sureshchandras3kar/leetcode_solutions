function searchMatrix(matrix, target) {
    if (!matrix || !matrix.length) return false;

    for (let row of matrix) {
        if (row.includes(target)) return true;
    }
    return false;
}
