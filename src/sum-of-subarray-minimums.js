// class MinDequeue {
//     constructor() {
//         this.list = [];
//     }

//     push(val) {
//         // console.log(val)
//         // 保证数据从队头到队尾递减
//         while (this.list[this.list.length - 1] > val) {
//             this.list.pop(); // 其余数据于本题无用，丢弃
//         }
//         this.list.push(val); // 插入到适当位置
//     }

//     // 队头出队
//     shift(val) {
//         if (this.list[0] === val) { // 按题意出队，不满足当前是最大值的情况下不出队
//             // 这里的js实现shift()理论上复杂度应该是O(k), 就不去真实实现一个O(1)出队的队列了，意思到位即可
//             this.list.shift();
//         }
//     }

//     // peek查看队列首端的最大值
//     min() {
//         return this.list[0];
//     }
// }

/**
 * @param {number[]} arr
 * @return {number}
 */
var sumSubarrayMins = function (arr) {
    let mod = 1000000007;
    let res = 0;
    for (let i = 0; i < arr.length; i++) {
        // const x = arr[i];
        let min = Infinity;
        for (let j = i; j < arr.length; j++) {
            if (min > arr[j]) {
                min = arr[j]
            }
            res = (res + min) % mod
        }
    }
    return res;
};

var sumSubarrayMins = function (A) {
    let mod = 1000000007;
    let sum = 0;
    for (let i = 0; i < A.length; i++) {
        let left = i - 1, right = i + 1;
        while (left >= 0 && A[left] >= A[i]) left--;
        while (right < A.length && A[right] > A[i]) right++;
        sum = (sum + (right - i) * (i - left) * A[i] % mod) % mod;
        // console.log(sum);
    }
    return sum;
};


var res = sumSubarrayMins([3, 1, 2, 4]);
console.log(res);

