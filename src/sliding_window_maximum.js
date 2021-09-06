/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */

class Dequeue {
    list = [];
    max() {
        return this.list[this.list.length - 1];
    }
    push(x) {
        while (this.list[0] < x) {
            this.list.shift()
        }
        this.list.unshift(x);
    }
    shift(x) {
        if (this.list[this.list.length - 1] === x) {
            this.list.pop();
        }
    }
}

// 维护一个单调队列，这里的队列是双端队列
class Dequeue {
    constructor() {
        this.list = [];
    }

    push(val) {
        // console.log(val)
        // 保证数据从队头到队尾递减
        while (this.list[this.list.length - 1] < val) {
            this.list.pop(); // 其余数据于本题无用，丢弃
        }
        this.list.push(val); // 插入到适当位置
    }

    // 队头出队
    shift(val) {
        if (this.list[0] === val) { // 按题意出队，不满足当前是最大值的情况下不出队
            // 这里的js实现shift()理论上复杂度应该是O(k), 就不去真实实现一个O(1)出队的队列了，意思到位即可
            this.list.shift();
        }
    }

    // peek查看队列首端的最大值
    max() {
        return this.list[0];
    }
}

//1040 ms, faster than 43.92%
var maxSlidingWindow = function (nums, k) {
    let res = [];
    let q = new Dequeue();
    for (let i = 0; i < nums.length; i++) {
        if (i < k - 1) {
            q.push(nums[i])
        } else {
            q.push(nums[i])
            res.push(q.max());
            q.shift(nums[i - k + 1])
        }
    }
    return res;
};

maxSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3)