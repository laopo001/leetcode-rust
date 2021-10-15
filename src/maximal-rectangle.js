/**
 * @param {character[][]} matrix
 * @return {number}
 */





const largestRectangleArea = (heights) => {
    let maxArea = 0
    const stack = []
    heights = [0, ...heights, 0]
    for (let i = 0; i < heights.length; i++) {
        while (heights[i] < heights[stack[stack.length - 1]]) { // 当前bar比栈顶bar矮
            const stackTopIndex = stack.pop() // 栈顶元素出栈，并保存栈顶bar的索引
            maxArea = Math.max(               // 计算面积，并挑战最大面积
                maxArea,                        // 计算出栈的bar形成的长方形面积
                heights[stackTopIndex] * (i - stack[stack.length - 1] - 1)
            )
        }
        stack.push(i)                       // 当前bar比栈顶bar高了，入栈
    }
    return maxArea
}


var maximalRectangle = function (matrix) {

    let m = matrix.length;
    if (m === 0) {
        return 0;
    }
    let n = matrix[0].length;
    let h = [];
    for (let i = 0; i < m; i++) {

        for (let j = 0; j < n; j++) {
            if (h[j] == null) {
                h[j] = []
            }
            h[j][i] = 0;
            if (matrix[i][j] == '1') {
                if (j - 1 == -1) {
                    h[j][i] = 1
                } else {
                    h[j][i] = h[j - 1][i] + 1;
                }

            }
        }
    }
    console.log(h);
    let res = -Infinity;
    for (let i = 0; i < h.length; i++) {
        res = Math.max(largestRectangleArea(h[i]), res)
    }
    return res;
};

let res = maximalRectangle([["1", "0", "1", "0", "0"], ["1", "0", "1", "1", "1"], ["1", "1", "1", "1", "1"], ["1", "0", "0", "1", "0"]])
console.log(res);