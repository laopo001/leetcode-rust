/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */
/**
 * @param {Node} root
 * @return {number[][]}
 */
var levelOrder = function (root) {
    let arr = [];
    let res = [];
    if (root == null) return arr;
    arr = [root];
    res = [[root.val]];
    function run(arr) {
        let temp = [];
        for (let i = 0; i < arr.length; i++) {
            const node = arr[i];
            node.children.forEach(c => {
                if (c != null) {
                    temp.push(c);
                }
            });
        }
        if (temp.length == 0) { return; }
        res.push(temp.map(n => n.val));
        run(temp);
    }
    run(arr);
    return res;
};