class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node(int _val = 0) { val = _val; }
};

class Solution {
public:
    Node* cloneGraphDFS(Node* node) {
        if (!node) return nullptr;
        unordered_map<int, Node*> visited;
        return dfs(node, visited);
    }
    
private:
    Node* dfs(Node* node, unordered_map<int, Node*>& visited) {
        if (visited.count(node->val)) return visited[node->val];
        
        Node* cloned = new Node(node->val);
        visited[node->val] = cloned;
        
        for (Node* neighbor : node->neighbors) {
            cloned->neighbors.push_back(dfs(neighbor, visited));
        }
        
        return cloned;
    }
};
