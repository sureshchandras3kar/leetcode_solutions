function spiralMatrixLayer(matrix) {
    if (!matrix || matrix.length === 0 || matrix[0].length === 0) {
        return [];
    }

    const result = [];
    const m = matrix.length;
    const n = matrix[0].length;
    const layers = Math.floor((Math.min(m, n) + 1) / 2);

    for (let layer = 0; layer < layers; layer++) {
        const top = layer;
        const bottom = m - 1 - layer;
        const left = layer;
        const right = n - 1 - layer;

        // Traverse right
        for (let col = left; col <= right; col++) {
            result.push(matrix[top][col]);
        }

        // Traverse down
        for (let row = top + 1; row <= bottom; row++) {
            result.push(matrix[row][right]);
        }

        // Traverse left (if there's a row remaining)
        if (top < bottom) {
            for (let col = right - 1; col >= left; col--) {
                result.push(matrix[bottom][col]);
            }
        }

        // Traverse up (if there's a column remaining)
        if (left < right) {
            for (let row = bottom - 1; row > top; row--) {
                result.push(matrix[row][left]);
            }
        }
    }

    return result;
}

const matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
const result = spiralMatrixLayer(matrix);
console.log(result);
