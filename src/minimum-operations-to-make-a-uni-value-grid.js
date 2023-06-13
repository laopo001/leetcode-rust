/**
 * @param {number[][]} grid
 * @param {number} x
 * @return {number}
 */
var minOperations = function (grid, x) {

    if (grid.length === 0) { return -1 };
    let arr = [];
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            const element = grid[i][j];
            arr.push(element);
        }
    }
    arr.sort((a, b) => a - b);
    let min = arr[0];
    let arr2 = [];
    for (let i = 0; i < arr.length; i++) {
        const element = arr[i];
        arr2.push(element - min);
    }
    let arr3 = [];
    for (let i = 0; i < arr2.length; i++) {
        const element = arr2[i];
        if (element % x != 0) {
            return -1;
        } else {
            arr3.push(element / x);
        }
    }

    let s = 0;
    for (let i = 0; i < arr3.length; i++) {
        const element = arr3[i];
        s += element;
    }
    let avg = Math.floor(s / arr3.length);
    let res = 0;
    for (let i = 0; i < arr3.length; i++) {
        const element = arr3[i];
        res += Math.abs(element - avg)
    }
    return res;
};
// 296 ms, faster than 86.18%
var minOperations = (grid, x) => {
    // 行、列
    const [m, n] = [grid.length, grid[0].length];
    // 如果只有一个元素，返回0
    if (m === 1 && n === 1) return 0;
    // 将网格扁平化
    const nums = [];
    for (let i = 0; i < m; i++) {
        nums.push(...grid[i]);
    }
    // 升序排序
    nums.sort((a, b) => a - b);
    const numsLen = nums.length;
    // 中位数
    const num = nums[numsLen >> 1];
    let res = 0;
    for (let i = 0; i < numsLen; i++) {
        // 当前数和中位数的差值
        const gap = nums[i] - num;
        // 某个差值不是x的倍数，则不能完成操作
        if (gap % x) return -1;
        // 累加上步骤次数
        res += (gap > 0 ? gap : -gap) / x;
    }
    return res;
};


var res = minOperations(
    [[529, 529, 989], [989, 529, 345], [989, 805, 69]]
    , 92)
console.log(res);