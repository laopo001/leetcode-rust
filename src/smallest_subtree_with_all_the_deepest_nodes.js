/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

function TreeNode(val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
}

/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
// Runtime: 84 ms, faster than 49.09%
var subtreeWithAllDeepest = function (root) {
    if (root == null) {
        return root;
    }
    let deepest = [];
    function deep(nodes) {
        let arr = [];
        for (let i = 0; i < nodes.length; i++) {
            const node = nodes[i];
            if (node.left != null) {
                node.left.parent = node;
                arr.push(node.left);
            }
            if (node.right != null) {
                node.right.parent = node;
                arr.push(node.right);
            }
        }
        if (arr.length === 0) {
            deepest = nodes;
            return;
        }
        deep(arr);
    }
    deep([root]);
    if (deepest.length === 1) {
        return deepest[0]
    }
    let res;
    function up(nodesSet) {
        let set = new Set();
        for (let x of nodesSet) {
            set.add(x.parent);
        }
        if (set.size === 1) {
            res = Array.from(set)[0];
            return;
        }
        up(set);
    }
    up(Set.from(deepest))
    return res;
};