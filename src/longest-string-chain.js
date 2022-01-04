function isLike(short, long) {
    if (short.length + 1 != long.length) {
        return false;
    }
    let c = 0;
    for (let i = 0; i < short.length; i++) {
        if (short[i] !== long[i] && c === 0) {
            c++;
            i--;
            continue;
        }
        if (short[i] !== long[i + 1] && c === 1) {
            return false;
        }
    }
    return true;
}

function run(nodes, parent, c) {
    let arr = []
    for (let i = 0; i < nodes.length; i++) {
        if (parent[nodes[i]]) {
            arr.push(...parent[nodes[i]])
        }
    }
    if (arr.length === 0) {
        return c;
    }
    return run(arr, parent, c + 1)
}

/**
 * @param {string[]} words
 * @return {number}
 */
var longestStrChain = function (words) {
    let map = {};
    let map2 = {};
    let max = 0;
    let visited = {};
    for (let i = 0; i < words.length; i++) {
        const word = words[i];
        visited[word] = false;
        max = Math.max(max, word.length);
        map[word.length] = map[word.length] ? (map[word.length].push(word), map[word.length]) : [word]
    }
    // console.log(map);
    let parent = {};
    for (let i = 1; i <= max; i++) {
        if (map[i + 1] == null || map[i] == null) {
            continue;
        }
        for (let j = 0; j < map[i].length; j++) {
            for (let k = 0; k < map[i + 1].length; k++) {
                if (isLike(map[i][j], map[i + 1][k])) {
                    if (parent[map[i][j]]) {
                        parent[map[i][j]].push(map[i + 1][k]);
                    } else {
                        parent[map[i][j]] = [map[i + 1][k]]
                    }
                    // parent[map[i][j]] = map[i + 1][k];
                    visited[map[i + 1][k]] = true;
                }
            }
        }
    }
    // console.log(visited, parent);
    let roots = [];
    for (let key in visited) {
        if (visited[key] === false) {
            roots.push(key);
        }
    }
    return run(roots, parent, 1)
};

/**
 * @param {string[]} words
 * @return {number}
 */
var longestStrChain = function (words) {
    let dp = {};
    let res = 0;
    words.sort((a, b) => a.length - b.length);
    for (let i = 0; i < words.length; i++) {
        const word = words[i];
        for (let j = 0; j < word.length; j++) {
            let left = word.slice(0, j);
            let right = word.slice(j + 1, word.length);
            let newWord = left + right;
            dp[word] = Math.max(dp[word] || 0, dp[newWord] ? dp[newWord] + 1 : 1);
            res = Math.max(res, dp[word]);
        }
    }
    return res;
};