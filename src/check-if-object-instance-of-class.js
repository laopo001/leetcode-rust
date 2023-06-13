/**
 * @param {any} obj
 * @param {any} classFunction
 * @return {boolean}
 */
var checkIfInstanceOf = function (obj, classFunction) {
    if (classFunction == null) {
        return false
    }
    while (obj != null) {
        if (Object.getPrototypeOf(obj) === classFunction.prototype) {
            return true;
        }
        obj = Object.getPrototypeOf(obj);
    }
    return false;
};

/**
 * checkIfInstanceOf(new Date(), Date); // true
 */

// 请你编写一个函数，检查给定的值是否是给定类或超类的实例。

// 可以传递给函数的数据类型没有限制。例如，值或类可能是  undefined 。

