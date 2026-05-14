from collections import deque

def ladderLength(beginWord, endWord, wordList):
    """Bidirectional BFS for more efficient search."""
    wordSet = set(wordList)
    if endWord not in wordSet:
        return 0
    
    begin_queue, end_queue = deque([beginWord]), deque([endWord])
    begin_visited, end_visited = {beginWord}, {endWord}
    length = 1
    
    while begin_queue or end_queue:
        length += expand(begin_queue, begin_visited, end_visited, wordSet)
        if begin_visited & end_visited:
            return length
        length += expand(end_queue, end_visited, begin_visited, wordSet)
        if begin_visited & end_visited:
            return length
    
    return 0

def expand(queue, visited, other_visited, wordSet):
    if not queue:
        return 0
    
    for _ in range(len(queue)):
        word = queue.popleft()
        for i in range(len(word)):
            for c in 'abcdefghijklmnopqrstuvwxyz':
                if c != word[i]:
                    new_word = word[:i] + c + word[i+1:]
                    if new_word in other_visited:
                        return 1
                    if new_word in wordSet and new_word not in visited:
                        visited.add(new_word)
                        queue.append(new_word)
    return 0
