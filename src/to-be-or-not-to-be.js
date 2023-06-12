/**
 * @param {string} val
 * @return {Object}
 */
var expect = function (val) {
    return {
        toBe(right) {
            if (val === right) {
                return true;
            } else {
                throw new Error("Not Equal");
            }
        },
        notToBe(right) {
            if (val !== right) {
                return true;
            } else {
                throw new Error("Equal");
            }
        }
    }
};

/**
 * expect(5).toBe(5); // true
 * expect(5).notToBe(5); // throws "Equal"
 */