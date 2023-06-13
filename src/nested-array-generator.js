var inorderTraversal = function* (arr) {
    for (const item of arr) {
      if (Array.isArray(item)) {
        yield* inorderTraversal(item);
      } else {
        yield item;
      }
    }
  };
  
  作者：__sgf__
  链接：https://leetcode.cn/problems/nested-array-generator/solutions/2289450/2649-qian-tao-shu-zu-sheng-cheng-qi-ti-m-a2xa/
  来源：力扣（LeetCode）
  著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。