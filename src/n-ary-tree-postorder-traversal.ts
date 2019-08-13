/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */
/**
 * @param {Node} root
 * @return {number[]}
 */
var postorder = function (root) {
    let arr = [];
    if (root == null) return arr;
    function run(root) {
        for (let i = 0; i < root.children.length; i++) {
            const item = root.children[i];
            run(item);

        }
        arr.push(root.val)
    }
    run(root);
    return arr;
};