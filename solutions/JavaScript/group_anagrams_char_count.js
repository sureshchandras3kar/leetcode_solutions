function groupAnagrams(strs) {
    const groups = new Map();
    for (const s of strs) {
        const count = new Array(26).fill(0);
        for (const c of s) count[c.charCodeAt(0) - 97]++;
        const key = count.join(',');
        if (!groups.has(key)) groups.set(key, []);
        groups.get(key).push(s);
    }
    return Array.from(groups.values());
}

console.log(groupAnagrams(["eat","tea","tan","ate","nat","bat"]));
