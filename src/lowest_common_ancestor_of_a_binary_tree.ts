/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */

/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function (root, p, q) {
    let res;
    function run(root, p, q) {
        if (root == null || res != null) {
            return 0;
        }
        let t = 0;
        if (root === p) {
            t = 1;
        }
        if (root === q) {
            t = 1;
        }
        let left = run(root.left, p, q);
        let right = run(root.right, p, q);
        if (t + left + right === 2 && res == null) {
            res = root;
        }
        return t + left + right;
    }
    run(root, p, q);
    return res;
};

