#include <iostream>
#include <vector>

class Node {
public:
    bool val;
    bool isLeaf;
    Node* topLeft;
    Node* topRight;
    Node* bottomLeft;
    Node* bottomRight;
    
    Node(bool val, bool isLeaf) : val(val), isLeaf(isLeaf), topLeft(nullptr), topRight(nullptr), bottomLeft(nullptr), bottomRight(nullptr) {}
};

Node* construct(std::vector<std::vector<int>>& grid, int top, int left, int size) {
    bool all_same = true;
    int val = grid[top][left];
    
    for (int i = top; i < top + size && all_same; i++) {
        for (int j = left; j < left + size && all_same; j++) {
            if (grid[i][j] != val) all_same = false;
        }
    }
    
    if (all_same) return new Node(val == 1, true);
    
    int half = size / 2;
    Node* node = new Node(1, false);
    node->topLeft = construct(grid, top, left, half);
    node->topRight = construct(grid, top, left + half, half);
    node->bottomLeft = construct(grid, top + half, left, half);
    node->bottomRight = construct(grid, top + half, left + half, half);
    return node;
}

Node* construct(std::vector<std::vector<int>>& grid) {
    return construct(grid, 0, 0, grid.size());
}

int main() {
    std::vector<std::vector<int>> grid = {{1, 1}, {1, 0}};
    Node* root = construct(grid);
    std::cout << root->val << " " << root->isLeaf << std::endl;
    return 0;
}
