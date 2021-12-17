// Runtime: 301 ms, faster than 12.27%

/**
 * @param {string} homepage
 */
 var BrowserHistory = function (homepage) {
    this.arr = [homepage];
    this.index = 0;
};

/** 
 * @param {string} url
 * @return {void}
 */
BrowserHistory.prototype.visit = function (url) {
    while (this.index + 1 < this.arr.length && this.arr.length != 0) {
        this.arr.pop();
    }
    this.arr.push(url)
    this.index = this.arr.length - 1;
};

/** 
 * @param {number} steps
 * @return {string}
 */
BrowserHistory.prototype.back = function (steps) {
    if (this.index - steps < 0) {
        this.index = 0;
    } else {
        this.index -= steps;
    }
    return this.arr[this.index];
};

/** 
 * @param {number} steps
 * @return {string}
 */
BrowserHistory.prototype.forward = function (steps) {
    if (this.index + steps >= this.arr.length) {
        this.index = this.arr.length - 1;
    } else {
        this.index += steps;
    }
    return this.arr[this.index];
};