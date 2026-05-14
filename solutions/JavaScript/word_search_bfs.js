function exist(board, word) {
    function bfs(startRow, startCol) {
        const queue = [[startRow, startCol, 0, new Set()]];
        while (queue.length) {
            const [row, col, index, visited] = queue.shift();
            if (index === word.length) return true;
            const dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];
            for (const [dr, dc] of dirs) {
                const nr = row + dr, nc = col + dc;
                const key = nr * 100 + nc;
                if (nr >= 0 && nr < board.length && nc >= 0 && nc < board[0].length &&
                    board[nr][nc] === word[index] && !visited.has(key)) {
                    const newVis = new Set(visited);
                    newVis.add(key);
                    queue.push([nr, nc, index + 1, newVis]);
                }
            }
        }
        return false;
    }
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[0].length; j++) {
            if (bfs(i, j)) return true;
        }
    }
    return false;
}
