
// Definition for a Node.
function Node(val, neighbors) {
    this.val = val === undefined ? 0 : val;
    this.neighbors = neighbors === undefined ? [] : neighbors;
};




/**
 * @param {Node} node
 * @return {Node}
 */
// 76 ms, faster than 78.70%
var cloneGraph = function (node) {
    let map = [];
    var run = function (node) {
        // console.log(node.val)
        if (node == null) {
            return node;
        }
        let clone = new Node(node.val, []);
        if (map[node.val]) {
            return map[node.val];
        }
        map[node.val] = clone;


        for (let i = 0; i < node.neighbors.length; i++) {
            const child = node.neighbors[i];
            clone.neighbors.push(run(child));
        }
        return clone;
    }

    return run(node);
};