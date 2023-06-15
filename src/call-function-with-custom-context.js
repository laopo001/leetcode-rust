/**
 * @param {Object} context
 * @param {any[]} args
 * @return {any}
 */
Function.prototype.callPolyfill = function (context, ...args) {
    let ____fn_name____ = Symbol('callPolyfill');
    context[____fn_name____] = this;
    // let res = context[____fn_name____](...args);
    let res = eval('context[____fn_name____](' + args.map((x, i) => 'args[' + i + ']').join(',') + ')');
    // console.log(context, args, 'context[____fn_name____](' + args.map((x, i) => 'args[' + i + ']').join(',') + ')');
    delete context[____fn_name____];
    return res;
  };

/**
 * function increment() { this.count++; return this.count; }
 * increment.callPolyfill({count: 1}); // 2
 */