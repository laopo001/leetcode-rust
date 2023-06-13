/**
 * @param {number[]} arr
 * @return {boolean}
 */

//68 ms, faster than 93.25%
var uniqueOccurrences = function (arr) {
    let map = {};
    for (let i = 0; i < arr.length; i++) {
      if (map[arr[i]] == null) {
        map[arr[i]] = 1;
      } else {
        map[arr[i]]++;
      }
    }
    let map2 = {};
    for (let k in map) {
      if (map2[map[k]] == null) {
        map2[map[k]] = 1;
      } else {
        return false;
      }
    }
    return true;
  };