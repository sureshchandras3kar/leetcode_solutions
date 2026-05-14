package main

import "container/list"

func exist(board [][]byte, word string) bool {
    for i := 0; i < len(board); i++ {
        for j := 0; j < len(board[0]); j++ {
            if bfs(board, word, i, j) { return true }
        }
    }
    return false
}

func bfs(board [][]byte, word string, startRow, startCol int) bool {
    queue := list.New()
    visited := make(map[string]bool)
    queue.PushBack([3]interface{}{startRow, startCol, 0})

    for queue.Len() > 0 {
        elem := queue.Front()
        queue.Remove(elem)
        state := elem.Value.([3]interface{})
        row, col, index := state[0].(int), state[1].(int), state[2].(int)

        if index == len(word) { return true }

        dirs := [4][2]int{{0,1}, {0,-1}, {1,0}, {-1,0}}
        for _, d := range dirs {
            nr, nc := row+d[0], col+d[1]
            if nr >= 0 && nr < len(board) && nc >= 0 && nc < len(board[0]) &&
               board[nr][nc] == word[index] {
                queue.PushBack([3]interface{}{nr, nc, index + 1})
            }
        }
    }
    return false
}
