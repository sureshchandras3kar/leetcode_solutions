#include <iostream>
#include <vector>
#include <queue>

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

Node* construct(std::vector<std::vector<int>>& grid) {
    int n = grid.size();
    struct Task { int top, left, size; Node* parent; };
    std::queue<Task> q;
    q.push({0, 0, n, nullptr});
    Node* root = nullptr;
    
    while (!q.empty()) {
        Task task = q.front();
        q.pop();
        
        bool all_same = true;
        int val = grid[task.top][task.left];
        for (int i = task.top; i < task.top + task.size && all_same; i++) {
            for (int j = task.left; j < task.left + task.size && all_same; j++) {
                if (grid[i][j] != val) all_same = false;
            }
        }
        
        Node* node = new Node(val == 1, all_same);
        if (root == nullptr) root = node;
        
        if (!all_same) {
            int half = task.size / 2;
            q.push({task.top, task.left, half, node});
            q.push({task.top, task.left + half, half, node});
            q.push({task.top + half, task.left, half, node});
            q.push({task.top + half, task.left + half, half, node});
        }
    }
    return root;
}

int main() {
    std::vector<std::vector<int>> grid = {{1, 1}, {1, 0}};
    Node* root = construct(grid);
    std::cout << root->val << " " << root->isLeaf << std::endl;
    return 0;
}
