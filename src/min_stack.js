/**
 * initialize your data structure here.
 */
// 156 ms, faster than 42.31%
var MinStack = function () {
    this.list = [];
};

/** 
 * @param {number} val
 * @return {void}
 */
MinStack.prototype.push = function (val) {
    this.list.push(val);
};

/**
 * @return {void}
 */
MinStack.prototype.pop = function () {
    this.list.pop();
};

/**
 * @return {number}
 */
MinStack.prototype.top = function () {
    return this.list[this.list.length - 1];
};

/**
 * @return {number}
 */
MinStack.prototype.getMin = function () {
    let minVal = Infinity;
    for (let i = 0; i < this.list.length; i++) {
        const element = this.list[i];
        if (element < minVal) {
            minVal = element;
        }
    }
    return minVal;
};

/**
 * Your MinStack object will be instantiated and called as such:
 * var obj = new MinStack()
 * obj.push(val)
 * obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.getMin()
 */