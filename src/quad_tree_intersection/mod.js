/**
 * // Definition for a QuadTree node.
 * function Node(val,isLeaf,topLeft,topRight,bottomLeft,bottomRight) {
 *    this.val = val;
 *    this.isLeaf = isLeaf;
 *    this.topLeft = topLeft;
 *    this.topRight = topRight;
 *    this.bottomLeft = bottomLeft;
 *    this.bottomRight = bottomRight;
 * };
 */
function run(quadTree1, quadTree2) {
    run(quadTree1.topLeft, quadTree2.topLeft);
    run(quadTree1.topRight, quadTree2.topRight);
    run(quadTree1.bottomLeft, quadTree2.bottomLeft);
    run(quadTree1.bottomRight, quadTree2.bottomRight);
    if (quadTree1 != null && quadTree1.val) {
        return quadTree1;
    }
    return quadTree2;
}

/**
 * @param {Node} quadTree1
 * @param {Node} quadTree2
 * @return {Node}
 */
var intersect = function (quadTree1, quadTree2) {
    return new Node(quadTree1.val && quadTree2.val, false,
        run(quadTree1.topLeft, quadTree2.topLeft),
        run(quadTree1.topRight, quadTree2.topRight),
        run(quadTree1.bottomLeft, quadTree2.bottomLeft),
        run(quadTree1.bottomRight, quadTree2.bottomRight),
    );
};