var TimeLimitedCache = function () {
    this.map = {}
    this.timeout_map = {}
};

/** 
 * @param {number} key
 * @param {number} value
 * @param {number} time until expiration in ms
 * @return {boolean} if un-expired key already existed
 */
TimeLimitedCache.prototype.set = function (key, value, duration) {
    this.map[key] = value;
    if (this.timeout_map[key] > -1) {
        clearTimeout(this.timeout_map[key]);
        let timer = setTimeout(() => {
            this.timeout_map[key] = -1;
        }, duration);
        this.timeout_map[key] = timer;
        return true;
    } else {
        let timer = setTimeout(() => {
            this.timeout_map[key] = -1;
        }, duration);
        console.log(timer);
        this.timeout_map[key] = timer;
        return false;
    }
};

/** 
 * @param {number} key
 * @return {number} value associated with key
 */
TimeLimitedCache.prototype.get = function (key) {
    if (this.timeout_map[key] > -1) {
        return this.map[key];
    } else {
        return -1;
    }
};

/** 
 * @return {number} count of non-expired keys
 */
TimeLimitedCache.prototype.count = function () {
    let count = 0;
    for(let key in this.timeout_map) {
        if (this.timeout_map[key] > -1) {
            count++;
        }
    }
    return count;
};


// 编写一个类，它允许获取和设置键-值对，并且每个键都有一个 过期时间 。

// 该类有三个公共方法：

// set(key, value, duration) ：接收参数为整型键 key 、整型值 value 和以毫秒为单位的持续时间 duration 。一旦 duration 到期后，这个键就无法访问。如果相同的未过期键已经存在，该方法将返回 true ，否则返回 false 。如果该键已经存在，则它的值和持续时间都应该被覆盖。

// get(key) ：如果存在一个未过期的键，它应该返回这个键相关的值。否则返回 -1 。

// count() ：返回未过期键的总数。



// ["TimeLimitedCache", "set", "set", "get", "get", "get", "count"]
// [[], [1, 42, 50], [1, 50, 100], [1], [1], [1], []]
// [0,  0,           40,           50, 120, 200, 250]
// 输出
// [null,false,false,50,-1,-1,0]
// 预期结果
// [null,false,true,50,50,-1,0]