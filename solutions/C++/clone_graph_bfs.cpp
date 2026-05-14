class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node(int _val = 0) { val = _val; }
};

class Solution {
public:
    Node* cloneGraphBFS(Node* node) {
        if (!node) return nullptr;
        
        unordered_map<int, Node*> visited;
        queue<Node*> q;
        q.push(node);
        visited[node->val] = new Node(node->val);
        
        while (!q.empty()) {
            Node* curr = q.front();
            q.pop();
            
            for (Node* neighbor : curr->neighbors) {
                if (!visited.count(neighbor->val)) {
                    visited[neighbor->val] = new Node(neighbor->val);
                    q.push(neighbor);
                }
                visited[curr->val]->neighbors.push_back(visited[neighbor->val]);
            }
        }
        
        return visited[node->val];
    }
};
